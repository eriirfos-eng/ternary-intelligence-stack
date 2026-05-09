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
