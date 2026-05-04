use candle_core::{Result, Tensor};

/// Straight-Through Estimator (STE) for ternary quantization.
/// 
/// ## Mathematical Proof
/// - **Manifold Stability**: Verified in `whitepaper/ternlang-whitepaper.tex`, Section §3.1 (Ternary Gradient Flow).
/// - **Threshold Convergence**: Derived in DOI 10.17605/OSF.IO/TZ7DC.
pub fn ternarize_ste(w: &Tensor, threshold: f32) -> Result<Tensor> {
    let dtype = w.dtype();
    
    // Mask for positive: w > threshold
    let pos_mask = w.gt(threshold)?;
    // Mask for negative: w < -threshold
    let neg_mask = w.lt(-threshold)?;

    // Quantized = 1.0 * pos_mask - 1.0 * neg_mask
    let quantized = (pos_mask.to_dtype(dtype)? - neg_mask.to_dtype(dtype)?)?;

    // y = w + (quantized - w).detach()
    w + (quantized - w)?.detach()
}
