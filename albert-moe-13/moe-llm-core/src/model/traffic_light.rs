// Ternary Traffic Light Routing — per-expert execution budget controller.
//
// Combines Lisa Scharler's triunity routing sketch with the traffic light
// load-balancing insight: each expert is assigned a trit execution state
// each forward pass based on rolling EMA utilization, then:
//
//   Green  (+1): underloaded — gate logit boosted, full output contribution
//   Orange ( 0): on-target  — gate logit unchanged, output scaled by ORANGE_SCALE
//   Red    (-1): overloaded — gate logit suppressed, @sparseskip handles skip
//
// The logit modifier is applied before routing (hard immediate correction).
// The output scale is applied after routing (controls gradient contribution).
// Together they form a ternary execution budget: full / partial / skip.
//
// This complements the differentiable LB loss (which shapes gate weights over
// time) with a hard non-differentiable correction that fires every step.
//
// Anti-stagnation burst (v2):
// When EMA-based routing stays all-Orange for STAGNATION_STEPS consecutive
// steps, the system has found a Nash equilibrium — all experts look equal,
// the gate can't differentiate, gradients through the gate vanish. The burst
// forcibly assigns Green/Red to disjoint expert subsets for BURST_DURATION
// steps, shifting routing and gradient flow toward forced-active experts.
// After the burst, natural EMA routing resumes with real utilization deltas.
// Rotation uses burst_count × 7 mod N (7 coprime to 12) for full coverage.

/// Orange expert output contribution factor.
/// At 0.4: orange experts contribute 40% of their normal output to the final mix.
pub const ORANGE_SCALE: f32 = 0.4;

/// Gate logit modifier magnitude (added to green, subtracted from red).
/// Calibrated relative to typical post-temperature logit spread (~0.01–0.1).
pub const LOGIT_STRENGTH: f32 = 0.4;

/// Warmup steps before traffic light activates.
const WARMUP_STEPS: usize = 50;

/// Target utilization per expert at uniform Top-3/12 routing.
/// f_i = fraction of tokens that *selected* each expert → 3/12 = 0.25 (not 1/12).
const TOP_K: f32 = 3.0;
const TARGET: f32 = TOP_K / 12.0; // 0.25
/// Below 80% of target → green (underloaded).
const GREEN_THRESH: f32 = TARGET * 0.80;
/// Above 140% of target → red (overloaded).
const RED_THRESH: f32 = TARGET * 1.40;
/// EMA smoothing factor: ~1/alpha steps effective window (~20 steps).
const EMA_ALPHA: f32 = 0.05;

/// Consecutive all-Orange steps before anti-stagnation burst fires.
const STAGNATION_STEPS: usize = 100;
/// Steps of forced Green/Red diversity per burst.
const BURST_DURATION: usize = 30;
/// Experts forced Green per burst.
const BURST_GREEN: usize = 3;
/// Experts forced Red per burst (non-overlapping with Green).
const BURST_RED: usize = 3;
/// Gap between Green and Red blocks in the rotation (experts left Orange between them).
const BURST_GAP: usize = 2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Trit {
    Green,   // +1
    Orange,  //  0
    Red,     // -1
}

impl Trit {
    pub fn logit_modifier(self) -> f32 {
        match self {
            Trit::Green  =>  LOGIT_STRENGTH,
            Trit::Orange =>  0.0,
            Trit::Red    => -LOGIT_STRENGTH,
        }
    }

    pub fn output_scale(self) -> f32 {
        match self {
            Trit::Green  => 1.0,
            Trit::Orange => ORANGE_SCALE,
            Trit::Red    => 1.0, // red experts are @sparseskip'd — scale irrelevant
        }
    }

    pub fn char(self) -> char {
        match self { Trit::Green => 'G', Trit::Orange => 'O', Trit::Red => 'R' }
    }
}

pub struct TrafficLight {
    ema: Vec<f32>,
    pub states: Vec<Trit>,
    num_experts: usize,
    step: usize,
    /// Consecutive all-Orange steps since last burst or non-Orange natural state.
    orange_streak: usize,
    /// Steps remaining in the current forced burst (0 = no active burst).
    burst_remaining: usize,
    /// How many bursts have fired — drives the rotation offset.
    burst_count: usize,
    /// TTL warmup freeze: remaining update() calls during which logit modifiers are suppressed.
    /// Set externally by train_bible on gradient-norm burst detection. Self-expiring countdown.
    freeze_remaining: usize,
}

impl TrafficLight {
    pub fn new(num_experts: usize) -> Self {
        let target = TOP_K / num_experts as f32;
        Self {
            ema: vec![target; num_experts],
            states: vec![Trit::Orange; num_experts],
            num_experts,
            step: 0,
            orange_streak: 0,
            burst_remaining: 0,
            burst_count: 0,
            freeze_remaining: 0,
        }
    }

    /// Update EMA from observed routing, recompute trit states.
    /// Handles warmup, normal EMA-based routing, and anti-stagnation bursts.
    pub fn update(&mut self, observed: &[f32]) {
        self.step += 1;
        if self.step < WARMUP_STEPS {
            return; // all Orange during warmup — don't interfere with initial routing
        }

        // Always update EMA regardless of burst state — keeps statistics honest.
        for i in 0..self.num_experts {
            let obs = observed.get(i).copied().unwrap_or(0.0);
            self.ema[i] = EMA_ALPHA * obs + (1.0 - EMA_ALPHA) * self.ema[i];
        }

        // If a burst is active, hold forced assignments and count down.
        if self.burst_remaining > 0 {
            self.burst_remaining -= 1;
            self.states = Self::burst_assignments(self.num_experts, self.burst_count);
            return;
        }

        // Normal EMA-based state assignment.
        let mut all_orange = true;
        for i in 0..self.num_experts {
            self.states[i] = if self.ema[i] < GREEN_THRESH {
                all_orange = false;
                Trit::Green
            } else if self.ema[i] > RED_THRESH {
                all_orange = false;
                Trit::Red
            } else {
                Trit::Orange
            };
        }

        // Track stagnation and fire burst if threshold reached.
        if all_orange {
            self.orange_streak += 1;
            if self.orange_streak >= STAGNATION_STEPS {
                self.burst_count += 1;
                self.burst_remaining = BURST_DURATION;
                self.orange_streak = 0;
                // Directly imprint EMA so G/R states persist ~20 steps after burst ends.
                // Without this, gradual EMA convergence during the burst isn't enough to
                // move values past the thresholds before the burst expires.
                self.imprint_burst_ema();
                self.states = Self::burst_assignments(self.num_experts, self.burst_count);
            }
        } else {
            self.orange_streak = 0;
        }

        // Decrement warmup freeze countdown — self-expiring, no external reset needed.
        if self.freeze_remaining > 0 {
            self.freeze_remaining -= 1;
        }
    }

    /// Directly set EMA values for burst experts so trit differentiation persists
    /// after the burst window closes. Green experts → 0.4× GREEN_THRESH (stays Green
    /// ~20 steps post-burst at α=0.05). Red experts → 1.6× RED_THRESH (stays Red ~18 steps).
    fn imprint_burst_ema(&mut self) {
        let base = (self.burst_count.wrapping_mul(7)) % self.num_experts;
        for i in 0..BURST_GREEN.min(self.num_experts) {
            let idx = (base + i) % self.num_experts;
            self.ema[idx] = GREEN_THRESH * 0.4;
        }
        for i in 0..BURST_RED.min(self.num_experts) {
            let idx = (base + BURST_GREEN + BURST_GAP + i) % self.num_experts;
            self.ema[idx] = RED_THRESH * 1.6;
        }
    }

    /// Generate forced burst trit assignments for a given burst index.
    /// Rotates by 7 per burst (coprime to 12) for full expert coverage over cycles.
    /// Layout: [BURST_GREEN Green] [BURST_GAP Orange] [BURST_RED Red] [rest Orange]
    fn burst_assignments(num_experts: usize, burst_count: usize) -> Vec<Trit> {
        let mut states = vec![Trit::Orange; num_experts];
        let base = (burst_count.wrapping_mul(7)) % num_experts;
        for i in 0..BURST_GREEN.min(num_experts) {
            states[(base + i) % num_experts] = Trit::Green;
        }
        for i in 0..BURST_RED.min(num_experts) {
            states[(base + BURST_GREEN + BURST_GAP + i) % num_experts] = Trit::Red;
        }
        states
    }

    pub fn is_warmup(&self) -> bool {
        self.step < WARMUP_STEPS
    }

    /// True during an active anti-stagnation burst.
    pub fn is_burst(&self) -> bool {
        self.burst_remaining > 0
    }

    /// Orange streak and burst count for telemetry.
    pub fn stagnation_state(&self) -> (usize, usize) {
        (self.orange_streak, self.burst_count)
    }

    /// Activate warmup freeze: suppress logit modifiers for `steps` update() calls.
    /// Idempotent — takes the max if a freeze is already active.
    pub fn freeze(&mut self, steps: usize) {
        self.freeze_remaining = steps.max(self.freeze_remaining);
    }

    pub fn is_frozen(&self) -> bool {
        self.freeze_remaining > 0
    }

    pub fn freeze_steps_remaining(&self) -> usize {
        self.freeze_remaining
    }

    /// Gate logit modifier vector [num_experts] — broadcast-add to gate logits.
    /// Returns zeros during warmup freeze so the gate can learn without TTL correction.
    pub fn logit_modifiers(&self) -> Vec<f32> {
        if self.freeze_remaining > 0 {
            return vec![0.0; self.num_experts];
        }
        self.states.iter().map(|s| s.logit_modifier()).collect()
    }

    /// Per-expert output scale [num_experts].
    pub fn output_scales(&self) -> Vec<f32> {
        self.states.iter().map(|s| s.output_scale()).collect()
    }

    /// One character per expert: G / O / R
    pub fn state_string(&self) -> String {
        self.states.iter().map(|s| s.char()).collect()
    }

    /// (green_count, orange_count, red_count)
    pub fn counts(&self) -> (usize, usize, usize) {
        let g = self.states.iter().filter(|&&s| s == Trit::Green).count();
        let r = self.states.iter().filter(|&&s| s == Trit::Red).count();
        (g, self.num_experts - g - r, r)
    }
}
