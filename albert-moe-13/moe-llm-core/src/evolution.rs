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

/// Fibonacci depth milestones (F_n ≥ 3). Albert climbs this sequence without a coded ceiling.
const FIB_TARGETS: &[usize] = &[3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610];

pub struct EvolutionManager {
    pub loss_history:         VecDeque<f32>,
    pub plateau_threshold:    f32,
    pub mastery_threshold:    f32,
    pub cooldown_remaining:   usize,
    /// Surgery suppressed when loss is above this — model is still near the random baseline.
    /// ln(32 000) ≈ 10.37; set to 9.8 to require real learning before growing.
    pub min_loss_for_plateau: f32,
    /// Index into FIB_TARGETS — the milestone we're currently climbing toward.
    pub fib_index:            usize,
    /// FIB_TARGETS[fib_index] — kept as a pub field for train_bible.rs compatibility.
    pub max_layers:           usize,
}

impl EvolutionManager {
    pub fn new() -> Self {
        Self {
            loss_history:         VecDeque::with_capacity(64),
            plateau_threshold:    0.02,
            mastery_threshold:    4.5,
            cooldown_remaining:   0,
            min_loss_for_plateau: 9.8,
            fib_index:            0,
            max_layers:           FIB_TARGETS[0],
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
    /// Mastery fires immediately — no history required.
    /// Plateau requires a full Fibonacci-length window and loss below random baseline.
    pub fn should_evolve(&self, current_layers: usize) -> bool {
        if current_layers >= self.max_layers { return false; }

        let latest = match self.loss_history.back() {
            Some(&v) => v,
            None     => return false,
        };

        // Mastery: fires at any depth the moment the model outgrows its current capacity.
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
            if latest > self.min_loss_for_plateau {
                println!("[evolution] Plateau detected (Δ{:.4}) but loss {:.4} > {:.4} — \
                    not past random baseline. Surgery suppressed.",
                    diff.abs(), latest, self.min_loss_for_plateau);
                return false;
            }
            println!("--- FIBONACCI PLATEAU TRIGGERED (Δ{:.4} over {} epochs, \
                next milestone: F{}={}L) ---",
                diff.abs(), history_len, self.fib_index + 1, self.max_layers);
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
}

impl Default for EvolutionManager {
    fn default() -> Self { Self::new() }
}
