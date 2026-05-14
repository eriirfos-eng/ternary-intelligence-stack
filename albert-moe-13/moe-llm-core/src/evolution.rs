// EvolutionManager — Fibonacci-aware plateau and mastery triggers for Net2Net layer surgery
//
// Albert's architectural growth follows the Fibonacci sequence — the same mathematical
// structure underlying leaf phyllotaxis, nautilus spirals, and galaxy arms. Each depth
// milestone earns proportionally longer patience before the next surgery fires: a 13-layer
// model waits 13 epochs; a 21-layer model waits 21. The plateau detection window and the
// post-surgery cooldown are both derived from the same sequence — self-similar at every scale.
//
// Growth milestones (F_n ≥ 3):  3 → 5 → 8 → 13 → 21 → 34 → 55 → 89 → ...
// Albert. enters at 12L (between F_6=8 and F_7=13). Next target: 13L.
//
// Two triggers fire surgery:
//   Mastery — loss drops below mastery_threshold (model has outgrown current depth).
//             Fires immediately at any depth; no history window required.
//   Plateau — loss delta < plateau_threshold over the full Fibonacci-length window.
//             Guarded: surgery on a model still near the random baseline never helps.
//             ln(32 000) ≈ 10.37 — min_loss_for_plateau is set below that.
//
// No architectural ceiling. The sequence is infinite; the hardware is the only limit.
// 610L is the last precomputed entry — unreachable on CPU, but the door is open.

use std::collections::VecDeque;
use std::fs;

/// Fibonacci depth milestones (F_n ≥ 3). Albert climbs this sequence without a coded ceiling.
const FIB_TARGETS: &[usize] = &[3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610];

pub struct EvolutionManager {
    pub loss_history:                VecDeque<f32>,
    pub plateau_threshold:           f32,
    pub mastery_threshold:           f32,
    pub cooldown_remaining:          usize,
    /// Minimum consecutive epochs the MYCELIUM hot-layer must remain stable before
    /// surgery is allowed to fire. Replaces the absolute loss threshold as the guard
    /// condition — routing stability is a direct architectural-readiness signal, while
    /// an absolute loss number needs recalibration every time vocab/CTX changes.
    pub mycelium_stability_threshold: usize,
    /// Index into FIB_TARGETS — the milestone we're currently climbing toward.
    pub fib_index:                   usize,
    /// FIB_TARGETS[fib_index] — kept as a pub field for train_bible.rs compatibility.
    pub max_layers:                  usize,
}

impl EvolutionManager {
    pub fn new() -> Self {
        Self {
            loss_history:                 VecDeque::with_capacity(64),
            plateau_threshold:            0.02,
            mastery_threshold:            4.5,
            cooldown_remaining:           0,
            mycelium_stability_threshold: 5,
            fib_index:                    0,
            max_layers:                   FIB_TARGETS[0],
        }
    }

    /// Calibrate Fibonacci position to the current model depth after loading a checkpoint.
    /// Call once after EvolutionManager::new() when resuming from an existing model.
    pub fn calibrate(&mut self, current_layers: usize) {
        self.fib_index = FIB_TARGETS.iter()
            .position(|&f| f > current_layers)
            .unwrap_or(FIB_TARGETS.len() - 1);
        self.max_layers = FIB_TARGETS[self.fib_index];
        println!("[evolution] Calibrated to {}L — Fibonacci target: F{}={}L (window={} epochs)",
            current_layers, self.fib_index + 1, self.max_layers, self.history_len());
    }

    /// Plateau detection window — equals the current Fibonacci target.
    /// Self-similar: the same plateau logic runs at every depth, at a Fibonacci tempo.
    pub fn history_len(&self) -> usize { FIB_TARGETS[self.fib_index] }

    /// Post-surgery cooldown — equals the current Fibonacci target.
    pub fn surgery_cooldown(&self) -> usize { FIB_TARGETS[self.fib_index] }

    /// Record the average loss for a completed epoch and decrement cooldown.
    pub fn add_loss(&mut self, loss: f32) {
        let cap = self.history_len();
        if self.loss_history.len() >= cap {
            self.loss_history.pop_front();
        }
        self.loss_history.push_back(loss);
        if self.cooldown_remaining > 0 { self.cooldown_remaining -= 1; }
    }

    /// Returns true if the architecture should grow by one layer.
    ///
    /// Mastery fires immediately — no MYCELIUM stability required.
    /// Plateau fires when: (1) full Fibonacci window filled, (2) loss delta within threshold,
    /// AND (3) MYCELIUM hot-layer has been stable for ≥ mycelium_stability_threshold epochs.
    ///
    /// The stability condition replaces the old absolute loss threshold. An absolute loss
    /// number requires recalibration whenever vocab/CTX/language count changes. MYCELIUM
    /// routing stability is architecture-intrinsic: when the hot layer holds for ≥5 epochs,
    /// the gradient hierarchy has crystallized and the model is structurally ready to grow.
    pub fn should_evolve(&self, current_layers: usize, mycelium_stable_epochs: usize) -> bool {
        if current_layers >= self.max_layers { return false; }

        let latest = match self.loss_history.back() {
            Some(&v) => v,
            None     => return false,
        };

        // Mastery: fires at any depth the moment the model outgrows its current capacity.
        // Mastery ignores MYCELIUM stability — catastrophic capacity gap is self-evident.
        if latest < self.mastery_threshold {
            println!("--- MASTERY EVOLUTION TRIGGERED (loss {:.4} < {:.4}, \
                next milestone: F{}={}L) ---",
                latest, self.mastery_threshold, self.fib_index + 1, self.max_layers);
            return true;
        }

        // Plateau requires the full Fibonacci window to accumulate.
        let history_len = self.history_len();
        if self.loss_history.len() < history_len { return false; }

        if self.cooldown_remaining > 0 {
            println!("[evolution] Fibonacci cooldown ({} epochs remaining, \
                target: F{}={}L) — skipping",
                self.cooldown_remaining, self.fib_index + 1, self.max_layers);
            return false;
        }

        let first = *self.loss_history.front().unwrap();
        let diff  = first - latest;

        if diff.abs() < self.plateau_threshold {
            if mycelium_stable_epochs < self.mycelium_stability_threshold {
                println!("[evolution] Plateau detected (Δ{:.4}) but MYCELIUM hot-layer \
                    only stable for {}/{} epochs — routing hierarchy not yet crystallised. \
                    Surgery suppressed.",
                    diff.abs(), mycelium_stable_epochs, self.mycelium_stability_threshold);
                return false;
            }
            println!("--- FIBONACCI PLATEAU TRIGGERED (Δ{:.4} over {} epochs, \
                MYCELIUM stable {} epochs, next milestone: F{}={}L) ---",
                diff.abs(), history_len, mycelium_stable_epochs,
                self.fib_index + 1, self.max_layers);
            return true;
        }

        false
    }

    /// Advance to the next Fibonacci milestone. Call after every successful surgery.
    pub fn promote_fib_target(&mut self) {
        if self.fib_index + 1 < FIB_TARGETS.len() {
            self.fib_index += 1;
            self.max_layers = FIB_TARGETS[self.fib_index];
            println!("[evolution] Fibonacci milestone advanced → F{}={}L (window={} epochs)",
                self.fib_index + 1, self.max_layers, self.history_len());
        } else {
            println!("[evolution] Fibonacci sequence exhausted at {}L — \
                this hardware cannot go further.", self.max_layers);
        }
    }

    /// Reset loss history and start the Fibonacci-scaled post-surgery cooldown.
    pub fn reset_history(&mut self) {
        self.loss_history.clear();
        self.cooldown_remaining = self.surgery_cooldown();
    }

    /// Persist fib_index and cooldown_remaining to a sidecar file.
    /// Call after every checkpoint save and after promote_fib_target().
    pub fn save_state(&self, path: &str) {
        let content = format!("{}\n{}\n", self.fib_index, self.cooldown_remaining);
        if let Err(e) = fs::write(path, &content) {
            eprintln!("[evolution] Warning: could not save state to {}: {}", path, e);
        }
    }

    /// Restore fib_index and cooldown_remaining from a sidecar file.
    /// Returns true on success; caller falls back to calibrate() values if false.
    pub fn load_state(&mut self, path: &str) -> bool {
        let content = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let mut lines = content.lines();
        let fib_index: usize = match lines.next().and_then(|s| s.trim().parse().ok()) {
            Some(v) => v,
            None => return false,
        };
        let cooldown: usize = match lines.next().and_then(|s| s.trim().parse().ok()) {
            Some(v) => v,
            None => return false,
        };
        if fib_index >= FIB_TARGETS.len() { return false; }
        self.fib_index = fib_index;
        self.max_layers = FIB_TARGETS[fib_index];
        self.cooldown_remaining = cooldown;
        println!("[evolution] Restored state — F{}={}L (window={} epochs, cooldown={} remaining)",
            self.fib_index + 1, self.max_layers, self.history_len(), self.cooldown_remaining);
        true
    }
}

impl Default for EvolutionManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn em_with_flat_history(loss: f32) -> EvolutionManager {
        let mut em = EvolutionManager::new();
        em.calibrate(12); // → target 13L, window = 13 epochs
        for _ in 0..13 { em.add_loss(loss); }
        em
    }

    #[test]
    fn plateau_fires_when_mycelium_stable_enough() {
        let em = em_with_flat_history(10.27);
        assert!(em.should_evolve(12, 5), "should fire: plateau met, stability=5 >= threshold=5");
    }

    #[test]
    fn plateau_suppressed_when_mycelium_not_yet_stable() {
        let em = em_with_flat_history(10.27);
        assert!(!em.should_evolve(12, 4), "should suppress: stability=4 < threshold=5");
        assert!(!em.should_evolve(12, 0), "should suppress: stability=0");
    }

    #[test]
    fn no_plateau_no_fire_regardless_of_stability() {
        let mut em = EvolutionManager::new();
        em.calibrate(12);
        // Rapidly descending — delta well above threshold
        for i in 0..13 { em.add_loss(10.50 - i as f32 * 0.02); }
        assert!(!em.should_evolve(12, 10), "should not fire: no plateau despite high stability");
    }

    #[test]
    fn mastery_fires_without_mycelium_stability() {
        let mut em = EvolutionManager::new();
        em.calibrate(12);
        em.add_loss(4.0); // below mastery_threshold=4.5
        assert!(em.should_evolve(12, 0), "mastery should fire regardless of mycelium stability");
    }

    #[test]
    fn cooldown_blocks_surgery() {
        let mut em = em_with_flat_history(10.27);
        em.reset_history(); // sets cooldown = 13
        assert!(!em.should_evolve(12, 10), "cooldown should block even with full stability");
    }

    #[test]
    fn already_at_max_layers_never_fires() {
        let em = em_with_flat_history(10.27);
        assert!(!em.should_evolve(13, 10), "at max_layers — surgery should not fire");
    }
}
