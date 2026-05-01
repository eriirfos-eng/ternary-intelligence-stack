// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
//! # Ternary Mapper
//! 
//! Logic for mapping high-precision floating point weights into the discrete 
//! balanced ternary state space {-1, 0, +1}.

use anyhow::Result;

/// Structural representation of a ternary-mapped weight layer.
pub struct TernaryLayer {
    pub weights: Vec<i8>, // Packed as trits
    pub scale: f32,       // Layer-wise scaling factor to preserve signal amplitude
    pub sparsity: f32,    // Percentage of zero-state weights
}

pub struct TernaryMapper {
    /// Quantization threshold. Weights |w| < threshold map to 0.
    pub threshold: f32,
}

impl TernaryMapper {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }

    /// Concrete Ternary Mapping
    /// 
    /// Maps values:
    /// > +threshold -> +1
    /// < -threshold -> -1
    /// otherwise -> 0
    /// 
    /// Returns:
    /// - ternary vector (i8: -1, 0, 1)
    /// - alpha scaling factor (mean of absolute values of non-zero weights)
    pub fn ternarize(&self, weights: &[f32]) -> (Vec<i8>, f32) {
        let mut ternary_vec = Vec::with_capacity(weights.len());
        let mut sum_abs = 0.0;
        let mut non_zero_count = 0;

        for &w in weights {
            let t = if w > self.threshold {
                1
            } else if w < -self.threshold {
                -1
            } else {
                0
            };

            if t != 0 {
                sum_abs += w.abs();
                non_zero_count += 1;
            }
            ternary_vec.push(t);
        }

        let alpha = if non_zero_count > 0 {
            sum_abs / non_zero_count as f32
        } else {
            0.0
        };

        (ternary_vec, alpha)
    }

    /// Maps a buffer of float weights into a TernaryLayer.
    pub fn map_weights(&self, float_weights: &[f32]) -> Result<TernaryLayer> {
        let (weights, scale) = self.ternarize(float_weights);
        let zero_count = weights.iter().filter(|&&w| w == 0).count();
        let sparsity = zero_count as f32 / weights.len() as f32;

        Ok(TernaryLayer {
            weights,
            scale,
            sparsity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternarization_basic() {
        let mapper = TernaryMapper::new(0.5);
        let weights = [0.9, -0.8, 0.1, 0.0, -0.05, 0.7];
        
        let (ternary, alpha) = mapper.ternarize(&weights);
        
        // Expected mapping: [1, -1, 0, 0, 0, 1]
        assert_eq!(ternary, vec![1, -1, 0, 0, 0, 1]);
        
        // Expected alpha: (0.9 + 0.8 + 0.7) / 3 = 2.4 / 3 = 0.8
        assert!((alpha - 0.8).abs() < 1e-6);
        
        // Compression stats
        let zero_count = ternary.iter().filter(|&&w| w == 0).count();
        let sparsity = zero_count as f32 / weights.len() as f32;
        assert_eq!(zero_count, 3);
        assert_eq!(sparsity, 0.5);
    }
}
