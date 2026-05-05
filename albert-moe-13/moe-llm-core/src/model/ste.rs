use candle_core::{Result, Tensor};

/// Full ternarize with gamma computed from weights. Call sparingly.
pub fn ternarize_ste(w: &Tensor, threshold: f32) -> Result<Tensor> {
    let gamma = w.abs()?.mean_all()?;
    ternarize_ste_with_gamma(w, threshold, &gamma)
}

/// Ternarize using a pre-computed gamma scalar. Hot path — no mean_all().
pub fn ternarize_ste_with_gamma(w: &Tensor, threshold: f32, gamma: &Tensor) -> Result<Tensor> {
    let dtype = w.dtype();
    let pos_mask = w.gt(threshold)?;
    let neg_mask = w.lt(-threshold)?;
    let quantized = pos_mask.to_dtype(dtype)?.broadcast_sub(&neg_mask.to_dtype(dtype)?)?;
    let quantized = quantized.broadcast_mul(gamma)?;
    let diff = quantized.broadcast_sub(w)?;
    Ok(w.broadcast_add(&diff.detach())?)
}
