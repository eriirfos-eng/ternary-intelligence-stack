// Per-layer post-training ternary quantization (PTQ).
//
// Algorithm (BitNet b1.58 scheme):
//   1. Compute per-layer scale α = mean(|W|)
//   2. Normalise: W_norm = W / α
//   3. Map to trits:
//        W_norm >  0.5  →  +1 (Affirm)
//        W_norm < -0.5  →  -1 (Reject)
//        otherwise      →   0 (Tend)
//
// The scale α is stored alongside the ternary weights so the original
// magnitude can be approximately reconstructed: W ≈ α × W_t

use ternlang_core::trit::Trit;

/// The result of quantizing one weight tensor.
#[derive(Debug, Clone)]
pub struct PerLayerQuant {
    /// Ternary weights in row-major order.
    pub trits: Vec<Trit>,
    /// Per-layer scale factor α = mean(|W|).
    /// Multiply by this to approximate the original magnitudes.
    pub scale: f32,
    /// Shape: [rows, cols] for 2D tensors; arbitrary for higher ranks.
    pub shape: Vec<usize>,
    /// Human-readable layer name (e.g. "model.layers.0.self_attn.q_proj.weight").
    pub name: String,
    /// Fraction of trits that are Tend (zero) — 0.0 … 1.0.
    pub sparsity: f64,
}

impl PerLayerQuant {
    /// Quantize `weights` (flat, row-major) to balanced ternary.
    ///
    /// `name`  — layer identifier for diagnostics.
    /// `shape` — logical shape of the tensor (product must equal `weights.len()`).
    pub fn quantize(weights: &[f32], name: impl Into<String>, shape: Vec<usize>) -> Self {
        assert_eq!(
            shape.iter().product::<usize>(),
            weights.len(),
            "shape product must match weights length"
        );

        // Step 1: scale
        let scale = {
            let sum: f32 = weights.iter().map(|w| w.abs()).sum();
            if sum == 0.0 { 1.0 } else { sum / weights.len() as f32 }
        };

        // Step 2 + 3: normalise and threshold
        let threshold = 0.5_f32;
        let trits: Vec<Trit> = weights.iter().map(|&w| {
            let n = w / scale;
            if n > threshold {
                Trit::Affirm
            } else if n < -threshold {
                Trit::Reject
            } else {
                Trit::Tend
            }
        }).collect();

        let zeros = trits.iter().filter(|&&t| t == Trit::Tend).count();
        let sparsity = zeros as f64 / trits.len() as f64;

        Self { trits, scale, shape, name: name.into(), sparsity }
    }

    /// Approximate reconstruction: W_approx = α × W_t (as f32).
    /// Useful for measuring quantization error.
    pub fn reconstruct(&self) -> Vec<f32> {
        self.trits.iter().map(|&t| match t {
            Trit::Affirm =>  self.scale,
            Trit::Reject => -self.scale,
            Trit::Tend   =>  0.0,
        }).collect()
    }

    /// Mean squared error between original weights and ternary reconstruction.
    pub fn mse(&self, original: &[f32]) -> f32 {
        assert_eq!(original.len(), self.trits.len());
        let recon = self.reconstruct();
        let sum_sq: f32 = original.iter().zip(recon.iter())
            .map(|(o, r)| (o - r).powi(2))
            .sum();
        sum_sq / original.len() as f32
    }
}

/// Quantize all layers of a model in parallel (requires `parallel` feature).
#[cfg(feature = "parallel")]
pub fn quantize_layers_parallel(
    layers: Vec<(String, Vec<f32>, Vec<usize>)>,
) -> Vec<PerLayerQuant> {
    use rayon::prelude::*;
    layers.into_par_iter()
        .map(|(name, weights, shape)| PerLayerQuant::quantize(&weights, name, shape))
        .collect()
}

/// Single-threaded fallback.
pub fn quantize_layers(
    layers: Vec<(String, Vec<f32>, Vec<usize>)>,
) -> Vec<PerLayerQuant> {
    layers.into_iter()
        .map(|(name, weights, shape)| PerLayerQuant::quantize(&weights, name, shape))
        .collect()
}
