// EvolutionManager: mastery + plateau triggers for Net2Net layer surgery — whitepaper §11.2
use std::collections::VecDeque;

/// Orchestrates automatic topological growth of the Albert MoE architecture.
///
/// Monitors per-epoch average loss and triggers a Net2Net safe-copy surgery
/// (layer duplication) when either of two conditions is met:
///
/// - **Mastery**: loss drops below `mastery_threshold` (model has outgrown current depth)
/// - **Plateau**: loss delta over the last `history_len` epochs is below `plateau_threshold`
///   (model is stuck and needs more capacity)
///
/// A `surgery_cooldown` prevents cascade expansions by suppressing evolution checks
/// for a fixed number of epochs after each surgery. The divergence trigger has been
/// deliberately omitted: post-surgery loss elevation is a normal and temporary artefact
/// of weight initialisation and should not be penalised with further expansion.
pub struct EvolutionManager {
    pub loss_history:          VecDeque<f32>,
    pub history_len:           usize,
    pub plateau_threshold:     f32,
    pub mastery_threshold:     f32,
    pub max_layers:            usize,
    pub surgery_cooldown:      usize,
    pub cooldown_remaining:    usize,
    /// Plateau surgery only fires when the model has loss below this threshold.
    /// Prevents growing a dead/collapsed model (which never helps).
    pub min_loss_for_plateau:  f32,
}

impl EvolutionManager {
    pub fn new() -> Self {
        Self {
            loss_history:         VecDeque::with_capacity(10),
            history_len:          10,
            plateau_threshold:    0.02,
            mastery_threshold:    4.5,
            max_layers:           12,
            surgery_cooldown:     20,
            cooldown_remaining:   0,
            min_loss_for_plateau: 8.0,   // don't grow if model is still near the random baseline
        }
    }

    /// Record the average loss for a completed epoch.
    /// Automatically decrements the surgery cooldown counter.
    pub fn add_loss(&mut self, loss: f32) {
        if self.loss_history.len() >= self.history_len {
            self.loss_history.pop_front();
        }
        self.loss_history.push_back(loss);
        if self.cooldown_remaining > 0 {
            self.cooldown_remaining -= 1;
        }
    }

    /// Returns `true` if the architecture should grow by one layer.
    ///
    /// Conditions (checked in order):
    /// 1. Already at `max_layers` → never evolve.
    /// 2. Insufficient history → wait.
    /// 3. Surgery cooldown active → suppress and log.
    /// 4. Mastery trigger: `latest_loss < mastery_threshold`.
    /// 5. Plateau trigger: `|first - latest| < plateau_threshold` over `history_len` epochs.
    pub fn should_evolve(&self, current_layers: usize) -> bool {
        if current_layers >= self.max_layers { return false; }
        if self.loss_history.len() < self.history_len { return false; }
        if self.cooldown_remaining > 0 {
            println!("[evolution] cooldown active ({} epochs remaining) — skipping",
                self.cooldown_remaining);
            return false;
        }

        let latest = *self.loss_history.back().unwrap();
        // Mastery trigger: model has outgrown current depth (§11.2).
        if latest < self.mastery_threshold {
            println!("--- MASTERY EVOLUTION TRIGGERED (loss {:.4} < {:.4}) ---",
                latest, self.mastery_threshold);
            return true;
        }

        let first = *self.loss_history.front().unwrap();
        let diff = first - latest;
        // Plateau trigger: loss delta below threshold over history window (§11.2).
        // Guard: only fire when the model is actually learning (loss below the random baseline).
        // Growing a dead model (loss ≈ ln(vocab)) never helps — it just compounds the damage.
        if diff.abs() < self.plateau_threshold {
            if latest > self.min_loss_for_plateau {
                println!("[evolution] Plateau detected but loss {:.4} > {:.4} threshold — \
                    model not yet learning. Surgery suppressed until loss improves.",
                    latest, self.min_loss_for_plateau);
                return false;
            }
            println!("--- PLATEAU EVOLUTION TRIGGERED (Δ {:.4} < {:.4} over {} epochs) ---",
                diff.abs(), self.plateau_threshold, self.history_len);
            return true;
        }

        false
    }

    /// Reset loss history and start the post-surgery cooldown.
    /// Called immediately after every successful layer surgery.
    pub fn reset_history(&mut self) {
        self.loss_history.clear();
        self.cooldown_remaining = self.surgery_cooldown;
    }
}

impl Default for EvolutionManager {
    fn default() -> Self { Self::new() }
}
