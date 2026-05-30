// Mycelium Module — expert lifecycle management and rhizomorphic growth signals
//
// Complements EvolutionManager with per-expert, per-layer telemetry across epochs.
// Implements two mycelium properties that the base EvolutionManager lacks:
//
//   Self-repair:          Detect experts that have been consistently Red (dead) for
//                         N epochs. Generate ResurrectionJobs — copy the most active
//                         neighbour's weights into the dead expert with small Gaussian
//                         noise. Executed by perform_resurrection() in train_bible.
//
//   Rhizomorphic signal:  Track gradient pressure per layer. Report the hottest layer
//                         so future surgery decisions can grow width (more experts) in
//                         the overloaded layer rather than blindly adding depth.
//
// The module operates at EPOCH granularity — record_epoch() is called once per epoch
// with the last observed TLIGHT state strings and per-layer gradient norm averages.
//
// This is a pure analytics / recommendation layer. It reads telemetry and produces
// ResurrectionJobs. All weight modification is delegated to train_bible.

use std::collections::VecDeque;

/// Per-expert activation level, mapped from traffic-light trit state.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ExpertState {
    Green,
    Orange,
    Red,
}

impl ExpertState {
    fn from_char(c: char) -> Self {
        match c {
            'G' => ExpertState::Green,
            'R' => ExpertState::Red,
            _ => ExpertState::Orange,
        }
    }
}

/// A recommendation to resurrect a dead expert from a healthy neighbour.
/// Execution: copy `blocks.{layer}.moe.experts.{seed_expert}.*` weights to
/// `blocks.{layer}.moe.experts.{dead_expert}.*` and add Gaussian noise σ=noise_sigma.
#[derive(Debug)]
pub struct ResurrectionJob {
    pub layer:       usize,
    pub dead_expert: usize,
    pub seed_expert: usize,
    pub noise_sigma: f32,
}

/// Per-epoch mycelium telemetry report.
#[derive(Debug)]
pub struct MyceliumReport {
    /// Expert resurrection jobs — dead experts to reinitialise from healthy neighbours.
    pub resurrections:         Vec<ResurrectionJob>,
    /// Layer index with the highest sustained gradient pressure.
    pub hottest_layer:         usize,
    /// Layer index with the lowest sustained gradient pressure.
    pub coldest_layer:         usize,
    /// Number of experts consistently dead this epoch.
    pub dead_expert_count:     usize,
    /// Number of experts consistently blooming (G) this epoch.
    pub blooming_expert_count: usize,
    /// Per-layer average gradient pressure (last history window).
    pub layer_pressure:        Vec<f32>,
}

pub struct MyceliumModule {
    // activation_history[layer][expert] = epoch-level state history
    activation_history: Vec<Vec<VecDeque<ExpertState>>>,
    // gradient_pressure[layer] = per-epoch average grad norm history
    gradient_pressure:  Vec<VecDeque<f32>>,
    num_layers:         usize,
    num_experts:        usize,
    /// Epochs of history to retain.
    history_len:        usize,
    /// Consecutive Red epochs before an expert is declared dead.
    dead_threshold:     usize,
    /// Consecutive Green epochs before an expert is declared blooming.
    bloom_threshold:    usize,
    /// Noise applied when seeding a resurrected expert from its neighbour.
    resurrection_noise: f32,
}

impl MyceliumModule {
    pub fn new(num_layers: usize, num_experts: usize) -> Self {
        Self {
            activation_history: vec![
                vec![VecDeque::with_capacity(20); num_experts];
                num_layers
            ],
            gradient_pressure: vec![VecDeque::with_capacity(20); num_layers],
            num_layers,
            num_experts,
            history_len:        20,
            dead_threshold:     8,
            bloom_threshold:    5,
            resurrection_noise: 0.02,
        }
    }

    /// Record one epoch of telemetry.
    ///
    /// `tlight_states` — one state string per layer ("GOOGOORROOOO" format).
    ///   Missing layers are ignored gracefully.
    /// `layer_grad_norms` — average per-layer gradient norm for this epoch.
    pub fn record_epoch(&mut self, tlight_states: &[String], layer_grad_norms: &[f32]) {
        for (layer, state_str) in tlight_states.iter().enumerate() {
            if layer >= self.num_layers { break; }
            for (expert, ch) in state_str.chars().enumerate() {
                if expert >= self.num_experts { break; }
                let hist = &mut self.activation_history[layer][expert];
                if hist.len() >= self.history_len { hist.pop_front(); }
                hist.push_back(ExpertState::from_char(ch));
            }
        }
        for (layer, &norm) in layer_grad_norms.iter().enumerate() {
            if layer >= self.num_layers { break; }
            let hist = &mut self.gradient_pressure[layer];
            if hist.len() >= self.history_len { hist.pop_front(); }
            hist.push_back(norm);
        }
    }

    /// Lower dead_threshold and raise resurrection noise during LB-off mode.
    /// Without LB, experts can go Red fast — catch them earlier.
    pub fn set_lb_off_mode(&mut self, lb_off: bool) {
        self.dead_threshold     = if lb_off { 3  } else { 8    };
        self.resurrection_noise = if lb_off { 0.05 } else { 0.02 };
    }

    /// Called by EvolutionManager immediately after Net2Net layer surgery.
    /// Expands internal bookkeeping to match the new architecture.
    pub fn on_layer_added(&mut self) {
        self.num_layers += 1;
        self.activation_history.push(vec![VecDeque::with_capacity(20); self.num_experts]);
        self.gradient_pressure.push(VecDeque::with_capacity(20));
    }

    /// Average gradient pressure for a layer over its history window.
    fn avg_pressure(&self, layer: usize) -> f32 {
        let hist = &self.gradient_pressure[layer];
        if hist.is_empty() { return 0.0; }
        hist.iter().sum::<f32>() / hist.len() as f32
    }

    /// The layer under the most sustained gradient pressure.
    pub fn hottest_layer(&self) -> usize {
        (0..self.num_layers)
            .max_by(|&a, &b| self.avg_pressure(a)
                .partial_cmp(&self.avg_pressure(b))
                .unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(self.num_layers.saturating_sub(1))
    }

    /// The layer under the least sustained gradient pressure.
    pub fn coldest_layer(&self) -> usize {
        (0..self.num_layers)
            .min_by(|&a, &b| self.avg_pressure(a)
                .partial_cmp(&self.avg_pressure(b))
                .unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0)
    }

    /// Experts that have been consistently Red for dead_threshold epochs.
    fn dead_experts(&self) -> Vec<(usize, usize)> {
        let mut dead = Vec::new();
        for layer in 0..self.num_layers {
            for expert in 0..self.num_experts {
                let hist = &self.activation_history[layer][expert];
                if hist.len() < self.dead_threshold { continue; }
                let all_red = hist.iter()
                    .rev()
                    .take(self.dead_threshold)
                    .all(|&s| s == ExpertState::Red);
                if all_red { dead.push((layer, expert)); }
            }
        }
        dead
    }

    /// Experts that have been consistently Green for bloom_threshold epochs.
    fn blooming_experts(&self) -> Vec<(usize, usize)> {
        let mut blooming = Vec::new();
        for layer in 0..self.num_layers {
            for expert in 0..self.num_experts {
                let hist = &self.activation_history[layer][expert];
                if hist.len() < self.bloom_threshold { continue; }
                let all_green = hist.iter()
                    .rev()
                    .take(self.bloom_threshold)
                    .all(|&s| s == ExpertState::Green);
                if all_green { blooming.push((layer, expert)); }
            }
        }
        blooming
    }

    /// The most consistently active (Green) expert in a given layer — used as
    /// the seed for resurrecting a dead expert in the same layer.
    fn most_active_expert(&self, layer: usize) -> Option<usize> {
        (0..self.num_experts).max_by_key(|&e| {
            self.activation_history[layer][e]
                .iter()
                .filter(|&&s| s == ExpertState::Green)
                .count()
        })
    }

    /// Generate ResurrectionJobs for all currently dead experts.
    fn generate_resurrections(&self) -> Vec<ResurrectionJob> {
        self.dead_experts()
            .into_iter()
            .filter_map(|(layer, dead_expert)| {
                let seed = self.most_active_expert(layer)?;
                if seed == dead_expert { return None; }
                Some(ResurrectionJob {
                    layer,
                    dead_expert,
                    seed_expert: seed,
                    noise_sigma: self.resurrection_noise,
                })
            })
            .collect()
    }

    /// Produce a full epoch report: resurrections, pressure map, bloom/dead counts.
    pub fn generate_report(&self) -> MyceliumReport {
        let resurrections = self.generate_resurrections();
        let dead_expert_count = resurrections.len();
        let blooming = self.blooming_experts();
        let layer_pressure: Vec<f32> = (0..self.num_layers).map(|l| self.avg_pressure(l)).collect();
        MyceliumReport {
            resurrections,
            hottest_layer:         self.hottest_layer(),
            coldest_layer:         self.coldest_layer(),
            dead_expert_count,
            blooming_expert_count: blooming.len(),
            layer_pressure,
        }
    }
}

impl Default for MyceliumModule {
    fn default() -> Self { Self::new(3, 12) }
}

// ─────────────────────────────────────────────────────────────────────────────
// Flux-based expert health — two-axis ternary liveness.
//
// The dead-expert detector above keys on a SINGLE axis: sustained TLIGHT-Red
// (routing/gradient pressure). That misses a real failure mode under LB-off +
// ternary STE: an expert still ROUTED (~uniform share) whose MLP weights have
// been driven into the ternary-zero state. It is "weight-dead" yet never goes
// TLIGHT-Red, so resurrection never fires. (weight-dead != TLIGHT-dead.)
//
// Nature defends FLUX, not connectivity: mycelium reinforces hyphae carrying
// nutrient throughput and withdraws from idle-but-attached ones. So we measure
// two independent ternary axes per expert and compose them:
//
//   substance in {-1 starved, 0 thin, +1 dense}    — mean |weight| of the expert
//   flow      in {-1 unrouted, 0 trickle, +1 busy}  — routing mass to the expert
//
//   healthy   <=> substance = +1  AND  flow = +1
//   vestigial <=> substance = -1  AND  flow >= 0   (routed but starved — the blind spot)
//   dormant   <=> substance = -1  AND  flow = -1   (idle on both axes — viable seed-bank reserve)
//
// OBSERVATIONAL ONLY. classify_flux is pure; the caller emits a FLUX telemetry
// line. No weights are touched and no resurrection is triggered from it — by
// design, so genuinely dormant experts re-germinate on their own when a niche
// reappears rather than being force-revived.

/// One ternary axis bucket. `to_trit()` yields -1 / 0 / +1.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Trit { Low, Mid, High }

impl Trit {
    pub fn to_trit(self) -> i8 {
        match self { Trit::Low => -1, Trit::Mid => 0, Trit::High => 1 }
    }
    /// Bucket a raw value against the population median:
    ///   Low  : value <  0.25 x median  (far below the typical expert — starved/unrouted)
    ///   High : value >= 0.75 x median  (at or near the typical expert — substantial)
    ///   Mid  : in between
    /// Median-relative so a globally balanced regime (all experts similar)
    /// produces all-High and zero false positives.
    fn bucket(v: f32, median: f32) -> Trit {
        if median <= 1e-9 { return Trit::Mid; }
        let r = v / median;
        if r < 0.25 { Trit::Low } else if r >= 0.75 { Trit::High } else { Trit::Mid }
    }
}

/// Derived two-axis health classification for a single expert.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ExpertHealth {
    Healthy,      // dense + busy
    Vestigial,    // starved but still routed — the TLIGHT blind spot
    Dormant,      // starved AND unrouted — seed-bank reserve
    Transitional, // anything in between
}

/// Per-epoch flux report. Pure data; format with `log_line()`.
#[derive(Debug)]
pub struct FluxReport {
    pub substance: Vec<Trit>,
    pub flow:      Vec<Trit>,
    pub health:    Vec<ExpertHealth>,
    pub vestigial: Vec<usize>,
    pub dormant:   Vec<usize>,
}

impl FluxReport {
    /// Greppable one-liner for the training log / dashboard parser.
    pub fn log_line(&self, epoch: usize) -> String {
        let sub: Vec<String> = self.substance.iter().map(|t| t.to_trit().to_string()).collect();
        let flw: Vec<String> = self.flow.iter().map(|t| t.to_trit().to_string()).collect();
        let vidx: Vec<String> = self.vestigial.iter().map(|i| i.to_string()).collect();
        format!(
            "FLUX epoch={} vestigial={} dormant={} SUB={} FLOW={} VIDX=[{}]",
            epoch, self.vestigial.len(), self.dormant.len(),
            sub.join(","), flw.join(","), vidx.join(",")
        )
    }
}

fn median(xs: &[f32]) -> f32 {
    if xs.is_empty() { return 0.0; }
    let mut s: Vec<f32> = xs.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = s.len();
    if n % 2 == 1 { s[n / 2] } else { 0.5 * (s[n / 2 - 1] + s[n / 2]) }
}

/// Classify each expert on two independent ternary axes and compose into a health
/// state. `weight_mass[e]` = mean |weight| of expert e's MLP (substance axis);
/// `routing_mass[e]` = epoch-average routing share of expert e (flow axis). If the
/// slices differ in length the shorter is used.
pub fn classify_flux(weight_mass: &[f32], routing_mass: &[f32]) -> FluxReport {
    let n = weight_mass.len().min(routing_mass.len());
    let w = &weight_mass[..n];
    let r = &routing_mass[..n];
    let w_med = median(w);
    let r_med = median(r);

    let substance: Vec<Trit> = w.iter().map(|&v| Trit::bucket(v, w_med)).collect();
    let flow:      Vec<Trit> = r.iter().map(|&v| Trit::bucket(v, r_med)).collect();

    let mut health    = Vec::with_capacity(n);
    let mut vestigial = Vec::new();
    let mut dormant   = Vec::new();
    for e in 0..n {
        let h = match (substance[e], flow[e]) {
            (Trit::High, Trit::High) => ExpertHealth::Healthy,
            (Trit::Low,  Trit::Low)  => { dormant.push(e);   ExpertHealth::Dormant }
            (Trit::Low,  _)          => { vestigial.push(e); ExpertHealth::Vestigial }
            _                        => ExpertHealth::Transitional,
        };
        health.push(h);
    }
    FluxReport { substance, flow, health, vestigial, dormant }
}

#[cfg(test)]
mod flux_tests {
    use super::*;

    #[test]
    fn vestigial_is_routed_but_weight_starved() {
        // experts 0,1 healthy; 2 vestigial (routed, ~zero weight); 3 dormant (idle on both).
        let weight = [1.00, 0.90, 0.0005, 0.0004];
        let route  = [0.30, 0.30, 0.30,   0.001];
        let rep = classify_flux(&weight, &route);
        assert_eq!(rep.health[0], ExpertHealth::Healthy);
        assert_eq!(rep.health[2], ExpertHealth::Vestigial, "routed + weight-starved must be vestigial");
        assert_eq!(rep.health[3], ExpertHealth::Dormant,   "idle on both axes must be dormant");
        assert!(rep.vestigial.contains(&2));
        assert!(rep.dormant.contains(&3));
    }

    #[test]
    fn balanced_regime_has_no_false_positives() {
        // lb-on style: all experts similar weight + near-uniform routing → nobody flagged.
        let weight = [0.80, 0.85, 0.90, 1.00, 0.95, 0.88];
        let route  = [0.16, 0.17, 0.17, 0.16, 0.17, 0.17];
        let rep = classify_flux(&weight, &route);
        assert!(rep.vestigial.is_empty(), "no vestigial in a balanced regime");
        assert!(rep.dormant.is_empty(),   "no dormant in a balanced regime");
    }
}
