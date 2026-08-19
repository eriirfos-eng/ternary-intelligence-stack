// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Phase 12B: Quantization-Aware Training (QAT) with Straight-Through Estimator (STE)
//
// STE rule:  d(quantize(w))/dw = 1  when |w| <= clip_threshold, else 0.
// Latent f32 weights are maintained throughout training; quantized weights are
// used only during the forward pass. The backward pass treats the step function
// as identity within the clip window — this is the "straight-through" estimate.

use crate::{TernaryMLP, TritMatrix, bitnet_threshold, quantize};
use ternlang_core::Trit;

// ─── Configuration ────────────────────────────────────────────────────────────

pub struct QatConfig {
    /// Learning rate for SGD on latent weights.
    pub lr: f32,
    /// Number of full passes over the training set.
    pub epochs: usize,
    /// STE clip threshold: gradients are zeroed for |w_latent| > clip.
    /// A value of 1.0 lets gradients flow anywhere, 0.5 restricts to the
    /// "undecided" region near the quantization boundary.
    pub clip_threshold: f32,
    /// Print loss every N epochs (0 = silent).
    pub log_every: usize,
}

impl Default for QatConfig {
    fn default() -> Self {
        Self {
            lr: 0.01,
            epochs: 100,
            clip_threshold: 1.0,
            log_every: 10,
        }
    }
}

// ─── Training result ─────────────────────────────────────────────────────────

pub struct QatResult {
    pub final_loss: f32,
    pub epochs_run: usize,
    /// Fraction of latent weights that are currently in the |w| <= clip zone.
    pub active_gradient_fraction: f32,
}

// ─── STE Trainer ─────────────────────────────────────────────────────────────

/// Maintains latent f32 shadow weights for a 2-layer MLP.
/// During each training step:
///   1. Quantize latent weights → ternary {-1, 0, +1}
///   2. Forward pass (f32 arithmetic on quantized weights)
///   3. MSE loss + backprop through STE
///   4. SGD update on latent weights
pub struct SteTrainer {
    pub w1_latent: Vec<f32>,   // shape [in_features × hidden_size]
    pub w2_latent: Vec<f32>,   // shape [hidden_size × out_features]
    pub in_features:  usize,
    pub hidden_size:  usize,
    pub out_features: usize,
    pub config: QatConfig,
}

impl SteTrainer {
    /// Initialise from an existing TernaryMLP's quantized weights.
    /// The ternary {-1,0,+1} values become the initial latent floats.
    pub fn from_mlp(mlp: &TernaryMLP, config: QatConfig) -> Self {
        let w1_latent = mlp.w1.to_i8_vec().iter().map(|&v| v as f32).collect();
        let w2_latent = mlp.w2.to_i8_vec().iter().map(|&v| v as f32).collect();
        Self {
            w1_latent,
            w2_latent,
            in_features:  mlp.in_features,
            hidden_size:  mlp.hidden_size,
            out_features: mlp.out_features,
            config,
        }
    }

    /// Initialise from raw f32 weights (quantization happens at first step).
    pub fn from_f32(
        in_features: usize,
        hidden_size: usize,
        out_features: usize,
        w1_f32: Vec<f32>,
        w2_f32: Vec<f32>,
        config: QatConfig,
    ) -> Self {
        assert_eq!(w1_f32.len(), in_features * hidden_size);
        assert_eq!(w2_f32.len(), hidden_size * out_features);
        Self { w1_latent: w1_f32, w2_latent: w2_f32, in_features, hidden_size, out_features, config }
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn quantize_latent(latent: &[f32]) -> Vec<f32> {
        let tau = bitnet_threshold(latent);
        quantize(latent, tau).iter().map(|&t| match t {
            Trit::Affirm =>  1.0,
            Trit::Reject => -1.0,
            Trit::Tend   =>  0.0,
        }).collect()
    }

    /// STE mask: 1.0 where |w_latent| <= clip, else 0.0.
    fn ste_mask(latent: &[f32], clip: f32) -> Vec<f32> {
        latent.iter().map(|&w| if w.abs() <= clip { 1.0 } else { 0.0 }).collect()
    }

    /// Matrix multiply A [m×k] × B [k×n] → C [m×n] (f32, row-major).
    fn matmul(a: &[f32], b: &[f32], m: usize, k: usize, n: usize) -> Vec<f32> {
        let mut c = vec![0.0f32; m * n];
        for i in 0..m {
            for j in 0..n {
                let mut acc = 0.0f32;
                for p in 0..k {
                    acc += a[i * k + p] * b[p * n + j];
                }
                c[i * n + j] = acc;
            }
        }
        c
    }

    /// Transpose a matrix [rows×cols] → [cols×rows].
    fn transpose(a: &[f32], rows: usize, cols: usize) -> Vec<f32> {
        let mut out = vec![0.0f32; rows * cols];
        for r in 0..rows {
            for c in 0..cols {
                out[c * rows + r] = a[r * cols + c];
            }
        }
        out
    }

    // ── Forward + backward ────────────────────────────────────────────────────

    /// One SGD step on a single sample.
    ///
    /// `input`  — flat f32 row vector, length `in_features`
    /// `target` — flat f32 row vector, length `out_features`
    ///
    /// Returns the MSE loss for this sample.
    pub fn train_step(&mut self, input: &[f32], target: &[f32]) -> f32 {
        let (inf, hs, outf) = (self.in_features, self.hidden_size, self.out_features);

        // Quantize latent → float {-1,0,+1}
        let w1_q = Self::quantize_latent(&self.w1_latent);
        let w2_q = Self::quantize_latent(&self.w2_latent);

        // ── Forward ──────────────────────────────────────────────────────────
        // hidden = input [1×in] × w1_q [in×hs] → [1×hs]
        let hidden = Self::matmul(input, &w1_q, 1, inf, hs);

        // Trit activation (sign) on hidden
        let hidden_act: Vec<f32> = hidden.iter().map(|&h| {
            if h > 0.0 { 1.0 } else if h < 0.0 { -1.0 } else { 0.0 }
        }).collect();

        // output = hidden_act [1×hs] × w2_q [hs×out] → [1×out]
        let output = Self::matmul(&hidden_act, &w2_q, 1, hs, outf);

        // ── Loss (MSE) ───────────────────────────────────────────────────────
        let loss: f32 = output.iter().zip(target.iter())
            .map(|(o, t)| (o - t).powi(2))
            .sum::<f32>() / outf as f32;

        // ── Backward (STE) ───────────────────────────────────────────────────
        // d_loss / d_output = 2*(output - target) / out_features
        let d_output: Vec<f32> = output.iter().zip(target.iter())
            .map(|(o, t)| 2.0 * (o - t) / outf as f32)
            .collect();

        // Layer 2 weight gradient:
        //   d_loss/d_w2_q  = hidden_act^T [hs×1] × d_output [1×out] → [hs×out]
        //   d_loss/d_w2_latent = d_loss/d_w2_q ⊙ ste_mask(w2_latent)
        let hidden_act_t = Self::transpose(&hidden_act, 1, hs);
        let d_w2_q = Self::matmul(&hidden_act_t, &d_output, hs, 1, outf);
        let ste2 = Self::ste_mask(&self.w2_latent, self.config.clip_threshold);
        let d_w2: Vec<f32> = d_w2_q.iter().zip(ste2.iter()).map(|(g, m)| g * m).collect();

        // Propagate gradient back through w2 to hidden_act:
        //   d_loss/d_hidden_act = d_output [1×out] × w2_q^T [out×hs] → [1×hs]
        let w2_q_t = Self::transpose(&w2_q, hs, outf);
        let d_hidden_act = Self::matmul(&d_output, &w2_q_t, 1, outf, hs);

        // Trit activation derivative (straight-through: 1 for non-zero, 0 for 0)
        // We approximate: pass gradient through for hidden != 0.
        let d_hidden: Vec<f32> = d_hidden_act.iter().zip(hidden.iter())
            .map(|(g, h)| if *h != 0.0 { *g } else { 0.0 })
            .collect();

        // Layer 1 weight gradient:
        //   d_loss/d_w1_q  = input^T [in×1] × d_hidden [1×hs] → [in×hs]
        //   d_loss/d_w1_latent = d_loss/d_w1_q ⊙ ste_mask(w1_latent)
        let input_t = Self::transpose(input, 1, inf);
        let d_w1_q = Self::matmul(&input_t, &d_hidden, inf, 1, hs);
        let ste1 = Self::ste_mask(&self.w1_latent, self.config.clip_threshold);
        let d_w1: Vec<f32> = d_w1_q.iter().zip(ste1.iter()).map(|(g, m)| g * m).collect();

        // ── SGD update ───────────────────────────────────────────────────────
        let lr = self.config.lr;
        for (w, g) in self.w1_latent.iter_mut().zip(d_w1.iter()) {
            *w -= lr * g;
        }
        for (w, g) in self.w2_latent.iter_mut().zip(d_w2.iter()) {
            *w -= lr * g;
        }

        loss
    }

    /// Run the full training loop over a dataset.
    ///
    /// `samples` — slice of (input, target) pairs
    pub fn train(&mut self, samples: &[(Vec<f32>, Vec<f32>)]) -> QatResult {
        let mut final_loss = 0.0f32;

        for epoch in 0..self.config.epochs {
            let mut epoch_loss = 0.0f32;
            for (input, target) in samples.iter() {
                epoch_loss += self.train_step(input, target);
            }
            epoch_loss /= samples.len() as f32;
            final_loss = epoch_loss;

            if self.config.log_every > 0 && (epoch + 1) % self.config.log_every == 0 {
                println!("[QAT/STE] epoch {:>4}/{} | loss: {:.6}", epoch + 1, self.config.epochs, epoch_loss);
            }
        }

        let active = self.w1_latent.iter().chain(self.w2_latent.iter())
            .filter(|&&w| w.abs() <= self.config.clip_threshold)
            .count();
        let total = self.w1_latent.len() + self.w2_latent.len();
        let active_gradient_fraction = active as f32 / total as f32;

        QatResult {
            final_loss,
            epochs_run: self.config.epochs,
            active_gradient_fraction,
        }
    }

    /// Finalise training: quantize latent weights and return a TernaryMLP.
    pub fn finalize(&self) -> TernaryMLP {
        let tau1 = bitnet_threshold(&self.w1_latent);
        let tau2 = bitnet_threshold(&self.w2_latent);
        let w1 = TritMatrix::from_f32(self.in_features, self.hidden_size, &self.w1_latent, tau1);
        let w2 = TritMatrix::from_f32(self.hidden_size, self.out_features, &self.w2_latent, tau2);
        TernaryMLP::new(w1, w2)
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg(n: usize, seed: u64) -> Vec<f32> {
        let mut s = seed;
        (0..n).map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((s >> 33) as f32) / (u32::MAX as f32) * 2.0 - 1.0
        }).collect()
    }

    #[test]
    fn ste_trainer_reduces_loss() {
        let (inf, hs, outf) = (8, 16, 4);
        let w1 = lcg(inf * hs, 0xdead);
        let w2 = lcg(hs * outf, 0xbeef);
        let config = QatConfig { lr: 0.05, epochs: 50, clip_threshold: 1.0, log_every: 0 };
        let mut trainer = SteTrainer::from_f32(inf, hs, outf, w1, w2, config);

        // XOR-like classification target: output[0] = 1.0, rest = 0.0
        let samples: Vec<(Vec<f32>, Vec<f32>)> = (0..8).map(|i| {
            let input = lcg(inf, i as u64 * 17 + 3);
            let target = vec![1.0, -1.0, 0.0, 0.0];
            (input, target)
        }).collect();

        let initial_loss = {
            let mut l = 0.0f32;
            for (input, target) in &samples {
                let w1_q = SteTrainer::quantize_latent(&trainer.w1_latent);
                let w2_q = SteTrainer::quantize_latent(&trainer.w2_latent);
                let hidden = SteTrainer::matmul(input, &w1_q, 1, inf, hs);
                let hidden_act: Vec<f32> = hidden.iter().map(|&h|
                    if h > 0.0 { 1.0 } else if h < 0.0 { -1.0 } else { 0.0 }
                ).collect();
                let output = SteTrainer::matmul(&hidden_act, &w2_q, 1, hs, outf);
                l += output.iter().zip(target.iter()).map(|(o, t)| (o-t).powi(2)).sum::<f32>() / outf as f32;
            }
            l / samples.len() as f32
        };

        let result = trainer.train(&samples);
        println!("[test] initial_loss={:.4} final_loss={:.4}", initial_loss, result.final_loss);
        assert!(result.final_loss <= initial_loss, "QAT training must not increase loss");
        assert!(result.active_gradient_fraction > 0.0, "Some gradients must flow through STE");
    }

    #[test]
    fn finalize_produces_valid_mlp() {
        let (inf, hs, outf) = (4, 8, 2);
        let w1 = lcg(inf * hs, 0xfeed);
        let w2 = lcg(hs * outf, 0xcafe);
        let config = QatConfig { lr: 0.01, epochs: 5, clip_threshold: 1.0, log_every: 0 };
        let mut trainer = SteTrainer::from_f32(inf, hs, outf, w1, w2, config);

        let samples = vec![
            (lcg(inf, 1), vec![1.0, -1.0]),
            (lcg(inf, 2), vec![-1.0, 1.0]),
        ];
        trainer.train(&samples);

        let mlp = trainer.finalize();
        assert_eq!(mlp.in_features, inf);
        assert_eq!(mlp.hidden_size, hs);
        assert_eq!(mlp.out_features, outf);

        // Forward pass smoke test
        let input = TritMatrix::from_f32(1, inf, &lcg(inf, 99), Trit::Affirm);
        let (output, _, _) = mlp.forward(&input);
        assert_eq!(output.rows, 1);
        assert_eq!(output.cols, outf);
    }
}
