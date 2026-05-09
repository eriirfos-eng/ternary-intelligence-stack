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

/// Orange expert output contribution factor.
/// At 0.4: orange experts contribute 40% of their normal output to the final mix.
pub const ORANGE_SCALE: f32 = 0.4;

/// Gate logit modifier magnitude (added to green, subtracted from red).
/// Calibrated relative to typical post-temperature logit spread (~0.01–0.1).
pub const LOGIT_STRENGTH: f32 = 0.4;

/// Warmup steps before traffic light activates.
/// Allows EMA to converge before applying corrections.
const WARMUP_STEPS: usize = 50;

/// Target utilization per expert at uniform Top-3/12 routing.
/// f_i = fraction of tokens that *selected* each expert → 3/12 = 0.25 (not 1/12).
/// 1/12 would apply if we tracked routing weights; we track selection frequency.
const TOP_K: f32 = 3.0;
const TARGET: f32 = TOP_K / 12.0; // 0.25
/// Below 80% of target → green (underloaded).
const GREEN_THRESH: f32 = TARGET * 0.80;
/// Above 140% of target → red (overloaded).
const RED_THRESH: f32 = TARGET * 1.40;
/// EMA smoothing factor: ~1/alpha steps effective window (~20 steps).
const EMA_ALPHA: f32 = 0.05;

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
}

impl TrafficLight {
    pub fn new(num_experts: usize) -> Self {
        let target = TOP_K / num_experts as f32;
        Self {
            ema: vec![target; num_experts],
            states: vec![Trit::Orange; num_experts],
            num_experts,
            step: 0,
        }
    }

    /// Update EMA from observed normalized routing weights, recompute trit states.
    /// `observed` is the per-expert combined_weight fraction (sums to ~1.0).
    pub fn update(&mut self, observed: &[f32]) {
        self.step += 1;
        if self.step < WARMUP_STEPS {
            return; // all Orange during warmup — don't interfere with initial routing
        }
        for i in 0..self.num_experts {
            let obs = observed.get(i).copied().unwrap_or(0.0);
            self.ema[i] = EMA_ALPHA * obs + (1.0 - EMA_ALPHA) * self.ema[i];
            self.states[i] = if self.ema[i] < GREEN_THRESH {
                Trit::Green
            } else if self.ema[i] > RED_THRESH {
                Trit::Red
            } else {
                Trit::Orange
            };
        }
    }

    pub fn is_warmup(&self) -> bool {
        self.step < WARMUP_STEPS
    }

    /// Gate logit modifier vector [num_experts] — broadcast-add to gate logits.
    pub fn logit_modifiers(&self) -> Vec<f32> {
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
