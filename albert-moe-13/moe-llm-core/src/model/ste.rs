use candle_core::{Result, Tensor};

/// Straight-Through Estimator (STE) for ternary quantization with scaling.
/// 
/// Refined for stability in deep networks by incorporating a scaling factor
/// that preserves the variance of the activation flow.
pub fn ternarize_ste(w: &Tensor, threshold: f32) -> Result<Tensor> {
    let dtype = w.dtype();
    
    // Mask for positive: w > threshold
    let pos_mask = w.gt(threshold)?;
    // Mask for negative: w < -threshold
    let neg_mask = w.lt(-threshold)?;

    // Quantized = 1.0 * pos_mask - 1.0 * neg_mask
    let quantized = (pos_mask.to_dtype(dtype)? - neg_mask.to_dtype(dtype)?)?;

    // Scaling factor (gamma): mean absolute value of the weights
    // This helps with "Static Weights" and "STE Instability" by maintaining magnitude.
    let gamma = w.abs()?.mean_all()?;
    let quantized = quantized.broadcast_mul(&gamma)?;

    // y = w + (quantized - w).detach()
    w + (quantized - w)?.detach()
}
