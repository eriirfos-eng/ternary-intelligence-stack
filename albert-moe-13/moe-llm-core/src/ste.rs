/// Straight-Through Estimator (STE) for ternary neural network training.
///
/// ## The Problem
///
/// Ternary networks quantize weights to {-1, 0, +1} using a threshold function:
///
/// ```text
/// q(w) = +1  if w >  τ
/// q(w) =  0  if |w| ≤ τ
/// q(w) = -1  if w < -τ
/// ```
///
/// This step function has zero gradient almost everywhere and undefined gradient
/// at the threshold boundaries — making standard backpropagation impossible.
///
/// ## The STE Solution
///
/// The Straight-Through Estimator (Hinton 2012, Bengio et al. 2013) approximates
/// the gradient by passing it straight through the quantization boundary unchanged,
/// as if quantization were the identity function during the backward pass:
///
/// ```text
/// ∂L/∂w ≈ ∂L/∂q(w)   when |w| ≤ 1 (clipped STE)
/// ∂L/∂w = 0            when |w| > 1
/// ```
///
/// This allows the continuous (F32) shadow weights to accumulate gradient signal
/// across training, while the forward pass uses the discrete ternary projection.
///
/// ## Integration with Albert Training
///
/// Albert uses a **phased training approach**:
///
/// 1. **Phase 1 (current)**: Train in F32 to establish a stable loss landscape and
///    allow the EvolutionManager to grow the architecture to its target depth.
///    L1 regularisation (λ=1e-5) pushes weights toward zero, pre-conditioning the
///    weight distribution for clean ternary projection.
///
/// 2. **Phase 2 (QAT)**: Apply `ste_forward` during the forward pass to simulate
///    ternary quantization while keeping F32 shadow weights for gradient accumulation.
///    This is Quantization-Aware Training with the STE backward approximation.
///
/// 3. **Phase 3 (export)**: Call `quantize_model` binary to pack F32 weights into
///    the `.trit` binary format using `pack_tensor`.
///
/// The two-phase approach mirrors successful methodologies in BitNet b1.58 and
/// other ternary training pipelines, where direct QAT from random initialisation
/// is numerically unstable. Pre-training in F32 first is the standard practice.

use candle_core::{Result, Tensor};

/// Compute the BitNet-style ternary threshold for a weight tensor.
///
/// τ = 0.5 × mean(|W|)
///
/// This is the standard absolute-mean threshold from "BitNet: Scaling 1-bit
/// Transformers for Large Language Models" (Wang et al. 2023).
pub fn bitnet_threshold(weights: &Tensor) -> Result<f64> {
    let abs_mean = weights.abs()?.mean_all()?.to_scalar::<f32>()?;
    Ok(0.5 * abs_mean as f64)
}

/// Project a weight tensor onto the ternary manifold {-1, 0, +1}.
///
/// Uses the BitNet threshold τ = 0.5 × mean(|W|).
/// Values in (-τ, +τ) → 0 (hold / sparse)
/// Values ≥ τ → +1 (affirm)
/// Values ≤ -τ → -1 (reject)
pub fn ternary_project(weights: &Tensor) -> Result<Tensor> {
    let threshold = bitnet_threshold(weights)?;
    let t = Tensor::new(threshold as f32, weights.device())?
        .broadcast_as(weights.shape())?;
    let neg_t = Tensor::new(-(threshold as f32), weights.device())?
        .broadcast_as(weights.shape())?;

    // +1 where w > τ, -1 where w < -τ, 0 otherwise
    let pos_mask = weights.gt(&t)?.to_dtype(candle_core::DType::F32)?;
    let neg_mask = weights.lt(&neg_t)?.to_dtype(candle_core::DType::F32)?;
    (pos_mask - neg_mask)
}

/// Straight-Through Estimator forward pass for a weight tensor.
///
/// Returns a tensor that:
/// - **Forward pass**: evaluates to `ternary_project(weights)` — discrete {-1, 0, +1}
/// - **Backward pass**: gradient flows as if `∂q/∂w = 1` (straight through)
///
/// This is achieved via the identity trick:
/// ```text
/// ste(w) = w + stop_gradient(q(w) - w)
///        = w + (q(w) - w)_detached
/// ```
/// In the forward pass: w + (q(w) - w) = q(w)  ✓
/// In the backward pass: ∂/∂w [w + const] = 1   ✓ (straight through)
///
/// # Usage in training
///
/// ```rust,ignore
/// // During QAT phase — replace direct weight use with STE projection:
/// let w_ternary = ste_forward(&attention.weight)?;
/// let output = input.matmul(&w_ternary.t()?)?;
/// ```
pub fn ste_forward(weights: &Tensor) -> Result<Tensor> {
    let quantized = ternary_project(weights)?;
    let quantized_detached = quantized.detach();
    let weights_detached = weights.detach();
    // ste = weights + stop_gradient(quantized - weights)
    weights + (quantized_detached - weights_detached)?
}

/// Compute weight sparsity: fraction of weights in the zero (hold) state.
///
/// At threshold τ, weights in (-τ, +τ) → 0. For a normally distributed weight
/// matrix calibrated to Albert's L1-regularised distribution, expected sparsity
/// is approximately 32% at stable convergence — consistent with the ternary
/// manifold's geometric properties.
pub fn compute_sparsity(weights: &Tensor) -> Result<f64> {
    let threshold = bitnet_threshold(weights)?;
    let t = Tensor::new(threshold as f32, weights.device())?
        .broadcast_as(weights.shape())?;
    // |w| <= threshold → zero zone
    let in_zero_zone = weights.abs()?.le(&t)?.to_dtype(candle_core::DType::F32)?;
    let zero_count = in_zero_zone.sum_all()?.to_scalar::<f32>()?;
    let total = weights.elem_count() as f64;
    Ok(zero_count as f64 / total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{Device, Tensor};

    #[test]
    fn ternary_project_correct() {
        let device = Device::Cpu;
        // weights: [-2, -0.01, 0.0, 0.01, 2] — threshold ≈ 0.5 * mean(|w|) ≈ 0.506
        let w = Tensor::new(&[-2.0f32, -0.01, 0.0, 0.01, 2.0], &device).unwrap();
        let q = ternary_project(&w).unwrap();
        let v = q.to_vec1::<f32>().unwrap();
        assert_eq!(v[0], -1.0, "large negative → -1");
        assert_eq!(v[4],  1.0, "large positive → +1");
        assert_eq!(v[1],  0.0, "near-zero → 0 (hold)");
        assert_eq!(v[2],  0.0, "zero → 0 (hold)");
        assert_eq!(v[3],  0.0, "near-zero → 0 (hold)");
    }

    #[test]
    fn ste_forward_matches_quantized() {
        let device = Device::Cpu;
        let w = Tensor::new(&[-1.5f32, 0.0, 0.05, 1.5], &device).unwrap();
        let ste = ste_forward(&w).unwrap();
        let q   = ternary_project(&w).unwrap();
        // Forward values must match ternary projection
        let ste_v = ste.to_vec1::<f32>().unwrap();
        let q_v   = q.to_vec1::<f32>().unwrap();
        for (s, p) in ste_v.iter().zip(q_v.iter()) {
            assert!((s - p).abs() < 1e-6, "STE forward ≠ quantized: {} ≠ {}", s, p);
        }
    }
}
