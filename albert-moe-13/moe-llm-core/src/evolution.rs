// EvolutionManager — Generational Fibonacci cycling for Net2Net layer surgery
//
// Albert's architectural growth follows the Fibonacci sequence — the same mathematical
// structure underlying leaf phyllotaxis, nautilus spirals, and orbital mechanics.
// Each depth milestone earns proportionally longer patience before the next surgery fires:
// a 21-layer model waits 21 epochs; a 144-layer model waits 144.
//
// GENERATIONAL CYCLING — modelled after natural seasons
// -------------------------------------------------------
// Growth is not linear accumulation. It is a spiral: same shape at every scale,
// always higher. Each generation runs a full Fibonacci arc of ARC_LENGTH surgeries
// (Spring → Summer → Autumn → Winter), then DIES and REBIRTHS from a higher base
// with a tighter standard. The model must prove DEEPER stability to earn each new spring.
//
//   Gen 1 (base F₄=13): windows 13→21→34→55→89→144, threshold 0.020
//         ↓ GENERATION_COMPLETE — death + rebirth
//   Gen 2 (base F₅=21): windows 21→34→55→89→144→233, threshold 0.015
//         ↓ GENERATION_COMPLETE
//   Gen 3 (base F₆=34): windows 34→55→89→144→233→377, threshold 0.011
//   ...
//
// fib_index governs window/cooldown and cycles within each generation's arc.
// max_layers is a SEPARATE forward-only ceiling so generational resets never
// cap growth below current depth.
//
// Two surgery triggers:
//   Mastery  — loss drops below mastery_threshold (capacity catastrophically exhausted).
//              Fires immediately; no history window or stability required.
//   Plateau  — loss delta < plateau_threshold over the full Fibonacci window
//              AND MYCELIUM hot-layer stable ≥ mycelium_stability_threshold epochs.
//
// Generation timeout (stuck-generation fallback):
//   After GENERATION_TIMEOUT_MULTIPLIER × final_arc_window epochs without surgery,
//   the generation force-advances (no layer added) to prevent eternal autumn.
//
// Fibonacci sequence extended to 24 terms. No architectural ceiling — the hardware
// is the only limit.

use std::collections::VecDeque;
use std::fs;

/// Fibonacci depth milestones. Extended to 24 terms — no coded ceiling.
const FIB_TARGETS: &[usize] = &[
    3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610,
    987, 1597, 2584, 4181, 6765, 10946, 17711, 28657,
    46368, 75025, 121393, 196418,
];

/// Number of surgeries per generation before the cycle resets.
const ARC_LENGTH: usize = 6;

/// plateau_threshold floor — below this, optimizer noise dominates at current batch geometry.
const MIN_PLATEAU_THRESHOLD: f32 = 0.008;

/// Multiplicative decay applied to plateau_threshold at each generation boundary.
const THRESHOLD_DECAY: f32 = 0.75;

/// After this multiple of the final arc window elapses with no surgery, force-advance the
/// generation without adding a layer (prevents eternal autumn).
const GENERATION_TIMEOUT_MULTIPLIER: usize = 3;

pub struct EvolutionManager {
    pub loss_history:                 VecDeque<f32>,
    pub plateau_threshold:            f32,
    pub mastery_threshold:            f32,
    pub cooldown_remaining:           usize,
    /// Minimum consecutive epochs the MYCELIUM hot-layer must remain stable before
    /// plateau surgery is allowed to fire.
    pub mycelium_stability_threshold: usize,
    /// Position in the current generation's arc — determines history_len and surgery_cooldown.
    /// Resets to gen_base_fib_index at each generation boundary.
    pub fib_index:                    usize,
    /// Forward-only layer ceiling. Advances to the next Fibonacci value above the
    /// current layer count after each surgery. Never decreases on generational reset.
    pub max_layers:                   usize,
    /// Consecutive epochs where routing entropy was near-uniform (≥95% of max entropy).
    near_uniform_streak:              usize,
    /// Set when near_uniform_streak reaches 34; consumed by consume_symmetry_break().
    symmetry_break_pending:           bool,

    // ── Generational state ────────────────────────────────────────────────────
    /// 1-indexed current generation number.
    pub generation:             usize,
    /// fib_index at the start of the current generation's arc.
    gen_base_fib_index:         usize,
    /// Surgeries completed in this generation (0..ARC_LENGTH).
    gen_step:                   usize,
    /// Epochs elapsed since this generation started.
    gen_epochs:                 usize,
    /// Epochs elapsed since the last surgery fired (for timeout detection).
    gen_epochs_no_surgery:      usize,
    /// plateau_threshold at the start of this generation (for GENERATION_COMPLETE log).
    gen_start_threshold:        f32,
    /// First loss recorded this generation (for GENERATION_COMPLETE log).
    gen_start_loss:             Option<f32>,
}

impl EvolutionManager {
    pub fn new() -> Self {
        Self {
            loss_history:                 VecDeque::with_capacity(256),
            plateau_threshold:            0.020,
            mastery_threshold:            4.5,
            cooldown_remaining:           0,
            mycelium_stability_threshold: 5,
            fib_index:                    0,
            max_layers:                   FIB_TARGETS[0],
            near_uniform_streak:          0,
            symmetry_break_pending:       false,
            generation:                   1,
            gen_base_fib_index:           0,
            gen_step:                     0,
            gen_epochs:                   0,
            gen_epochs_no_surgery:        0,
            gen_start_threshold:          0.020,
            gen_start_loss:               None,
        }
    }

    /// Calibrate Fibonacci position to the current model depth after loading a checkpoint.
    /// Call once after EvolutionManager::new() when resuming from an existing model.
    pub fn calibrate(&mut self, current_layers: usize) {
        let idx = FIB_TARGETS.iter()
            .position(|&f| f > current_layers)
            .unwrap_or(FIB_TARGETS.len() - 1);
        self.fib_index          = idx;
        self.gen_base_fib_index = idx;
        self.max_layers         = FIB_TARGETS[idx];
        self.gen_start_threshold = self.plateau_threshold;
        println!(
            "[evolution] Calibrated to {}L — ceiling F{}={}L, window={} epochs \
             gen={} step={}/{} threshold={:.4}",
            current_layers, idx + 1, self.max_layers, self.history_len(),
            self.generation, self.gen_step, ARC_LENGTH, self.plateau_threshold,
        );
    }

    /// Plateau detection window — equals FIB_TARGETS[fib_index].
    pub fn history_len(&self) -> usize {
        FIB_TARGETS[self.fib_index.min(FIB_TARGETS.len() - 1)]
    }

    /// Post-surgery cooldown — equals FIB_TARGETS[fib_index].
    pub fn surgery_cooldown(&self) -> usize {
        FIB_TARGETS[self.fib_index.min(FIB_TARGETS.len() - 1)]
    }

    /// Authoritative plateau-gate state for live telemetry — no side effects.
    /// Mirrors the quarter-means math in `should_evolve()` so the dashboard can
    /// render the REAL gate every epoch instead of a divergent client-side
    /// estimate. Returns (smoothed_delta, threshold, window, filled).
    /// `smoothed_delta` is NaN until the window is full.
    pub fn gate_telemetry(&self) -> (f32, f32, usize, usize) {
        let window = self.history_len();
        let filled = self.loss_history.len().min(window);
        let delta = if self.loss_history.len() >= window && window > 0 {
            let quarter = (self.loss_history.len() / 4).max(1);
            let first_mean: f32 = self.loss_history.iter().take(quarter).sum::<f32>()
                / quarter as f32;
            let last_mean: f32 = self.loss_history.iter().rev().take(quarter).sum::<f32>()
                / quarter as f32;
            first_mean - last_mean
        } else {
            f32::NAN
        };
        (delta, self.plateau_threshold, window, filled)
    }

    /// Record the average loss for a completed epoch and decrement cooldown.
    pub fn add_loss(&mut self, loss: f32) {
        let cap = self.history_len();
        if self.loss_history.len() >= cap {
            self.loss_history.pop_front();
        }
        self.loss_history.push_back(loss);
        if self.cooldown_remaining > 0 { self.cooldown_remaining -= 1; }
        self.gen_epochs           += 1;
        self.gen_epochs_no_surgery += 1;
        if self.gen_start_loss.is_none() {
            self.gen_start_loss = Some(loss);
        }
    }

    /// Check whether the current generation has timed out (no surgery for too long).
    /// If so, force-advance the generation WITHOUT adding a layer — prevents eternal autumn.
    /// Call once per epoch, before should_evolve.
    pub fn check_generation_timeout(&mut self) -> bool {
        let final_arc_idx = (self.gen_base_fib_index + ARC_LENGTH - 1)
            .min(FIB_TARGETS.len() - 1);
        let timeout = GENERATION_TIMEOUT_MULTIPLIER * FIB_TARGETS[final_arc_idx];
        if self.gen_epochs_no_surgery >= timeout {
            let final_loss = self.loss_history.back().copied().unwrap_or(0.0);
            println!(
                "[evolution] GENERATION_TIMEOUT gen={} step={}/{} \
                 ({} epochs without surgery, timeout={}) — forced generation advance, no layer added.",
                self.generation, self.gen_step, ARC_LENGTH,
                self.gen_epochs_no_surgery, timeout,
            );
            self.advance_generation(final_loss, None);
            return true;
        }
        false
    }

    /// Returns true if the architecture should grow by one layer.
    ///
    /// Mastery fires immediately at any depth.
    /// Plateau fires when: (1) full Fibonacci window filled, (2) smoothed loss delta within
    /// threshold (quarter-window means, noise-resistant), AND (3) MYCELIUM hot-layer stable
    /// ≥ mycelium_stability_threshold epochs.
    pub fn should_evolve(&self, current_layers: usize, mycelium_stable_epochs: usize) -> bool {
        if current_layers >= self.max_layers { return false; }

        let latest = match self.loss_history.back() {
            Some(&v) => v,
            None     => return false,
        };

        // Mastery: capacity gap is catastrophic — fire immediately.
        if latest < self.mastery_threshold {
            println!(
                "--- MASTERY EVOLUTION TRIGGERED (loss {:.4} < {:.4}, \
                 next ceiling: F{}={}L) ---",
                latest, self.mastery_threshold,
                self.fib_index + 1, self.max_layers,
            );
            return true;
        }

        let history_len = self.history_len();
        if self.loss_history.len() < history_len { return false; }

        if self.cooldown_remaining > 0 {
            println!(
                "[evolution] Fibonacci cooldown ({} epochs remaining, \
                 gen={} step={}/{}) — skipping",
                self.cooldown_remaining, self.generation, self.gen_step, ARC_LENGTH,
            );
            return false;
        }

        // Robust plateau: compare the mean of the earliest quarter of the window against
        // the mean of the most recent quarter. Single-point first-vs-last was too sensitive
        // to epoch-to-epoch oscillation (~0.030 nats) relative to the plateau threshold
        // (~0.015 after gen-2 decay), causing the gate to stall indefinitely on a true
        // structural plateau. Quarter-means smooth the noise while still detecting genuine
        // descent (model actually improving → first_mean << last_mean → diff > threshold).
        let quarter = (self.loss_history.len() / 4).max(1);
        let first_mean: f32 = self.loss_history.iter().take(quarter).sum::<f32>()
            / quarter as f32;
        let last_mean: f32 = self.loss_history.iter().rev().take(quarter).sum::<f32>()
            / quarter as f32;
        // diff > 0 → improvement (older was worse); diff < 0 → model got worse (also plateau).
        let diff = first_mean - last_mean;

        if diff < self.plateau_threshold {
            if mycelium_stable_epochs < self.mycelium_stability_threshold {
                println!(
                    "[evolution] Plateau detected (smoothed Δ{:.4} over {} epochs, \
                     early_mean={:.4} late_mean={:.4}) but MYCELIUM \
                     hot-layer only stable {}/{} epochs — routing hierarchy not yet \
                     crystallised. Surgery suppressed.",
                    diff, history_len, first_mean, last_mean,
                    mycelium_stable_epochs, self.mycelium_stability_threshold,
                );
                return false;
            }
            println!(
                "--- FIBONACCI PLATEAU TRIGGERED (smoothed Δ{:.4} over {} epochs, \
                 early_mean={:.4} late_mean={:.4}, threshold={:.4}, \
                 MYCELIUM stable {} epochs, gen={} step={}/{}, \
                 next ceiling: F{}={}L) ---",
                diff, history_len, first_mean, last_mean, self.plateau_threshold,
                mycelium_stable_epochs,
                self.generation, self.gen_step, ARC_LENGTH,
                self.fib_index + 1, self.max_layers,
            );
            return true;
        }

        false
    }

    /// Advance to the next position in the generational arc. Call after every successful surgery.
    /// `new_layers` is the layer count AFTER the surgery (used to update max_layers).
    pub fn promote_fib_target(&mut self, new_layers: usize) {
        let final_loss = self.loss_history.back().copied().unwrap_or(0.0);

        self.gen_step             += 1;
        self.gen_epochs_no_surgery = 0;

        // Advance fib_index within the current generation's arc.
        let arc_end = (self.gen_base_fib_index + ARC_LENGTH).min(FIB_TARGETS.len() - 1);
        if self.fib_index + 1 < arc_end {
            self.fib_index += 1;
        }

        // max_layers advances to the next Fibonacci ceiling above the new layer count.
        // This is always forward-only — never decreases on generational reset.
        self.max_layers = FIB_TARGETS.iter()
            .find(|&&f| f > new_layers)
            .copied()
            .unwrap_or(usize::MAX);

        if self.gen_step >= ARC_LENGTH {
            // Generation complete — death and rebirth.
            self.advance_generation(final_loss, Some(new_layers));
        } else {
            println!(
                "[evolution] Gen {} step {}/{} → window={} epochs, ceiling={}L, threshold={:.4}",
                self.generation, self.gen_step, ARC_LENGTH,
                self.history_len(), self.max_layers, self.plateau_threshold,
            );
        }
    }

    /// Complete the current generation and begin the next.
    /// Called by promote_fib_target (arc complete) and check_generation_timeout (forced).
    fn advance_generation(&mut self, final_loss: f32, layer_count: Option<usize>) {
        // Build window list for the archaeological log.
        let windows: Vec<String> = (0..ARC_LENGTH)
            .map(|i| {
                FIB_TARGETS[(self.gen_base_fib_index + i).min(FIB_TARGETS.len() - 1)]
                    .to_string()
            })
            .collect();

        let new_threshold = (self.plateau_threshold * THRESHOLD_DECAY)
            .max(MIN_PLATEAU_THRESHOLD);

        // One-line archaeological record — readable at Gen 10 looking back.
        println!(
            "GENERATION_COMPLETE gen={} surgeries={} epochs={} \
             windows=[{}] threshold={:.3}→{:.3} final_loss={:.4}{}",
            self.generation,
            self.gen_step,
            self.gen_epochs,
            windows.join(","),
            self.gen_start_threshold,
            new_threshold,
            final_loss,
            layer_count
                .map(|l| format!(" layer_count={}L", l))
                .unwrap_or_default(),
        );

        // ── Rebirth ──────────────────────────────────────────────────────────
        self.generation          += 1;
        self.gen_base_fib_index  += 1;
        self.fib_index            = self.gen_base_fib_index.min(FIB_TARGETS.len() - 1);
        self.gen_step             = 0;
        self.gen_epochs           = 0;
        self.gen_epochs_no_surgery = 0;
        self.gen_start_loss       = None;
        self.gen_start_threshold  = new_threshold;
        self.plateau_threshold    = new_threshold;

        println!(
            "[evolution] ↻ Rebirth — Gen {} begins: base=F{}={}, \
             window={} epochs, ceiling={}L, threshold={:.4}",
            self.generation,
            self.gen_base_fib_index + 1,
            FIB_TARGETS[self.gen_base_fib_index.min(FIB_TARGETS.len() - 1)],
            self.history_len(),
            self.max_layers,
            self.plateau_threshold,
        );
    }

    /// Reset loss history and start the Fibonacci-scaled post-surgery cooldown.
    pub fn reset_history(&mut self) {
        self.loss_history.clear();
        self.cooldown_remaining = self.surgery_cooldown();
    }

    /// Record epoch routing entropy. Near-uniform means entropy ≥ 95% of ln(num_experts).
    /// After 34 consecutive near-uniform epochs, schedules an autonomous symmetry break.
    pub fn record_routing_entropy(&mut self, entropy: f32, num_experts: usize) {
        let max_entropy = (num_experts as f32).ln();
        if max_entropy <= 0.0 { return; }
        if entropy / max_entropy >= 0.95 {
            self.near_uniform_streak += 1;
            if self.near_uniform_streak >= 34 && !self.symmetry_break_pending {
                self.symmetry_break_pending = true;
                println!(
                    "[evolution] near-uniform routing for {} consecutive epochs \
                     (entropy {:.3}/{:.3}) — symmetry break scheduled for next restart",
                    self.near_uniform_streak, entropy, max_entropy,
                );
            }
        } else {
            self.near_uniform_streak = 0;
        }
    }

    /// Returns true if an autonomous symmetry break was scheduled, clearing the flag.
    pub fn consume_symmetry_break(&mut self) -> bool {
        if self.symmetry_break_pending {
            self.symmetry_break_pending = false;
            self.near_uniform_streak    = 0;
            println!("[evolution] consuming symmetry-break flag — gate reset will fire this run");
            true
        } else {
            false
        }
    }

    /// Persist state to a sidecar file. Call after every checkpoint save and surgery.
    pub fn save_state(&self, path: &str) {
        let mut content = format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{:.6}\n",
            self.fib_index,
            self.cooldown_remaining,
            self.generation,
            self.gen_base_fib_index,
            self.gen_step,
            self.gen_epochs,
            self.gen_epochs_no_surgery,
            self.plateau_threshold,
        );
        // Persist loss_history so it survives restarts — without this the plateau ring
        // resets to empty on every Modal restart, forcing a 144-epoch refill before the
        // gate can ever fire (and if Modal restarts more often than that, it never fires).
        if !self.loss_history.is_empty() {
            let hist: Vec<String> = self.loss_history.iter()
                .map(|v| format!("{:.6}", v))
                .collect();
            content.push_str(&format!("h:{}\n", hist.join(",")));
        }
        if let Err(e) = fs::write(path, &content) {
            eprintln!("[evolution] Warning: could not save state to {}: {}", path, e);
        }
    }

    /// Restore state from a sidecar file. Backward-compatible with the old 2-line format.
    /// Returns true on success; caller falls back to calibrate() values if false.
    pub fn load_state(&mut self, path: &str) -> bool {
        let content = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let mut lines = content.lines();

        macro_rules! next_parsed {
            ($t:ty) => {
                match lines.next().and_then(|s| s.trim().parse::<$t>().ok()) {
                    Some(v) => v,
                    None    => return false,
                }
            };
        }

        let fib_index:   usize = next_parsed!(usize);
        let cooldown:    usize = next_parsed!(usize);

        if fib_index >= FIB_TARGETS.len() { return false; }
        self.fib_index          = fib_index;
        self.cooldown_remaining = cooldown;
        self.max_layers         = FIB_TARGETS[fib_index];

        // Extended fields — optional (old checkpoints only have 2 lines).
        if let Some(gen_val) = lines.next().and_then(|s| s.trim().parse::<usize>().ok()) {
            self.generation = gen_val;
        }
        if let Some(base) = lines.next().and_then(|s| s.trim().parse::<usize>().ok()) {
            self.gen_base_fib_index = base.min(FIB_TARGETS.len() - 1);
        }
        if let Some(step) = lines.next().and_then(|s| s.trim().parse::<usize>().ok()) {
            self.gen_step = step;
        }
        if let Some(ep) = lines.next().and_then(|s| s.trim().parse::<usize>().ok()) {
            self.gen_epochs = ep;
        }
        if let Some(no_surg) = lines.next().and_then(|s| s.trim().parse::<usize>().ok()) {
            self.gen_epochs_no_surgery = no_surg;
        }
        if let Some(thr) = lines.next().and_then(|s| s.trim().parse::<f32>().ok()) {
            self.plateau_threshold   = thr;
            self.gen_start_threshold = thr;
        }

        // Restore loss_history from the optional h: line — allows the plateau gate to
        // resume mid-window across restarts instead of starting from scratch each time.
        if let Some(hist_line) = lines.next() {
            let trimmed = hist_line.trim();
            if let Some(hist_str) = trimmed.strip_prefix("h:") {
                self.loss_history.clear();
                for tok in hist_str.split(',') {
                    if let Ok(v) = tok.trim().parse::<f32>() {
                        self.loss_history.push_back(v);
                    }
                }
            }
        }

        println!(
            "[evolution] Restored — F{}={}L, window={} epochs, cooldown={}, \
             gen={} step={}/{}, threshold={:.4}",
            self.fib_index + 1, self.max_layers, self.history_len(),
            self.cooldown_remaining, self.generation, self.gen_step, ARC_LENGTH,
            self.plateau_threshold,
        );
        true
    }
}

impl Default for EvolutionManager {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn em_at(layers: usize) -> EvolutionManager {
        let mut em = EvolutionManager::new();
        em.calibrate(layers);
        em
    }

    fn fill_flat(em: &mut EvolutionManager, loss: f32) {
        for _ in 0..em.history_len() { em.add_loss(loss); }
    }

    #[test]
    fn plateau_fires_when_mycelium_stable_enough() {
        let mut em = em_at(12);
        fill_flat(&mut em, 10.27);
        assert!(em.should_evolve(12, 5));
    }

    #[test]
    fn plateau_suppressed_when_mycelium_not_yet_stable() {
        let mut em = em_at(12);
        fill_flat(&mut em, 10.27);
        assert!(!em.should_evolve(12, 4));
    }

    #[test]
    fn no_plateau_no_fire_regardless_of_stability() {
        let mut em = em_at(12);
        for i in 0..em.history_len() { em.add_loss(10.50 - i as f32 * 0.02); }
        assert!(!em.should_evolve(12, 10));
    }

    #[test]
    fn mastery_fires_without_mycelium_stability() {
        let mut em = em_at(12);
        em.add_loss(4.0);
        assert!(em.should_evolve(12, 0));
    }

    #[test]
    fn cooldown_blocks_surgery() {
        let mut em = em_at(12);
        fill_flat(&mut em, 10.27);
        em.reset_history();
        assert!(!em.should_evolve(12, 10));
    }

    #[test]
    fn already_at_max_layers_never_fires() {
        let mut em = em_at(12);
        fill_flat(&mut em, 10.27);
        assert!(!em.should_evolve(13, 10));
    }

    #[test]
    fn generation_advances_after_arc_length_surgeries() {
        let mut em = em_at(12);
        assert_eq!(em.generation, 1);
        for i in 0..ARC_LENGTH {
            fill_flat(&mut em, 10.0 - i as f32 * 0.1);
            em.promote_fib_target(13 + i);
            em.reset_history();
        }
        assert_eq!(em.generation, 2);
        assert_eq!(em.gen_step, 0);
    }

    #[test]
    fn plateau_threshold_tightens_each_generation() {
        let mut em = em_at(12);
        let t0 = em.plateau_threshold;
        for i in 0..ARC_LENGTH {
            fill_flat(&mut em, 10.0 - i as f32 * 0.1);
            em.promote_fib_target(13 + i);
            em.reset_history();
        }
        assert!(em.plateau_threshold < t0);
        assert!(em.plateau_threshold >= MIN_PLATEAU_THRESHOLD);
    }

    #[test]
    fn threshold_floors_at_minimum() {
        let mut em = em_at(12);
        // Run many generations — threshold must never fall below floor.
        for g in 0..20 {
            for i in 0..ARC_LENGTH {
                fill_flat(&mut em, 9.0 - (g * ARC_LENGTH + i) as f32 * 0.01);
                em.promote_fib_target(13 + g * ARC_LENGTH + i);
                em.reset_history();
            }
        }
        assert!(em.plateau_threshold >= MIN_PLATEAU_THRESHOLD);
    }

    #[test]
    fn max_layers_never_decreases_on_generational_reset() {
        let mut em = em_at(12);
        let mut last_max = em.max_layers;
        for i in 0..ARC_LENGTH + 2 {
            fill_flat(&mut em, 10.0 - i as f32 * 0.1);
            em.promote_fib_target(13 + i);
            em.reset_history();
            assert!(em.max_layers >= last_max,
                "max_layers decreased on gen boundary: {} < {}", em.max_layers, last_max);
            last_max = em.max_layers;
        }
    }

    #[test]
    fn generation_timeout_advances_without_surgery() {
        let mut em = em_at(12);
        em.add_loss(10.5); // start gen_start_loss
        let final_arc_idx = (em.gen_base_fib_index + ARC_LENGTH - 1).min(FIB_TARGETS.len() - 1);
        let timeout = GENERATION_TIMEOUT_MULTIPLIER * FIB_TARGETS[final_arc_idx];
        // Simulate timeout epochs without surgery
        em.gen_epochs_no_surgery = timeout;
        let gen_before = em.generation;
        let fired = em.check_generation_timeout();
        assert!(fired);
        assert_eq!(em.generation, gen_before + 1);
        assert_eq!(em.gen_step, 0);
    }

    #[test]
    fn save_load_roundtrip() {
        let mut em = em_at(17);
        em.add_loss(9.84);
        em.promote_fib_target(18);
        em.reset_history();

        let path = "/tmp/evo_test_state.txt";
        em.save_state(path);

        let mut em2 = EvolutionManager::new();
        assert!(em2.load_state(path));
        assert_eq!(em2.fib_index,          em.fib_index);
        assert_eq!(em2.generation,         em.generation);
        assert_eq!(em2.gen_base_fib_index, em.gen_base_fib_index);
        assert_eq!(em2.gen_step,           em.gen_step);
        assert!((em2.plateau_threshold - em.plateau_threshold).abs() < 1e-5);
    }
}
