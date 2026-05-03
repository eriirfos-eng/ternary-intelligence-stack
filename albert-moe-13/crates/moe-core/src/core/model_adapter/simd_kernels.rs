//! # Ternary SIMD Kernels
//! 
//! Hardware-accelerated AVX2 implementations for ternary math.
//! Exploit zero-skip (HOLD state) inherently via hardware FMA pipelining.

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Performs a highly optimized ternary dot product using AVX2.
/// Zeros are naturally skipped by the SIMD multiplication without branch divergence.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn ternary_dot_product_avx2(weights: &[i8], inputs: &[f32]) -> f32 {
    let len = weights.len().min(inputs.len());
    let mut sum = _mm256_setzero_ps();
    
    let mut i = 0;
    while i + 31 < len {
        // Load 32 ternary weights (i8)
        let w_int = _mm256_loadu_si256(weights.as_ptr().add(i) as *const __m256i);
        
        // Convert i8 to f32.
        let w_i16_0 = _mm256_cvtepi8_epi16(_mm256_castsi256_si128(w_int));
        let w_i16_1 = _mm256_cvtepi8_epi16(_mm256_extracti128_si256(w_int, 1));
        
        let w_i32_0 = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(w_i16_0));
        let w_i32_1 = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(w_i16_0, 1));
        let w_i32_2 = _mm256_cvtepi16_epi32(_mm256_castsi256_si128(w_i16_1));
        let w_i32_3 = _mm256_cvtepi16_epi32(_mm256_extracti128_si256(w_i16_1, 1));
        
        let w_f_0 = _mm256_cvtepi32_ps(w_i32_0);
        let w_f_1 = _mm256_cvtepi32_ps(w_i32_1);
        let w_f_2 = _mm256_cvtepi32_ps(w_i32_2);
        let w_f_3 = _mm256_cvtepi32_ps(w_i32_3);

        // Load 32 inputs (f32)
        let in_0 = _mm256_loadu_ps(inputs.as_ptr().add(i));
        let in_1 = _mm256_loadu_ps(inputs.as_ptr().add(i + 8));
        let in_2 = _mm256_loadu_ps(inputs.as_ptr().add(i + 16));
        let in_3 = _mm256_loadu_ps(inputs.as_ptr().add(i + 24));
        
        // FMA (Fused Multiply-Add). Zeros cost 0 compute penalty.
        sum = _mm256_fmadd_ps(w_f_0, in_0, sum);
        sum = _mm256_fmadd_ps(w_f_1, in_1, sum);
        sum = _mm256_fmadd_ps(w_f_2, in_2, sum);
        sum = _mm256_fmadd_ps(w_f_3, in_3, sum);

        i += 32;
    }
    
    // Horizontal sum
    let mut result_arr = [0.0f32; 8];
    _mm256_storeu_ps(result_arr.as_mut_ptr(), sum);
    let mut total = result_arr.iter().sum::<f32>();
    
    // Remainder scalar execution
    while i < len {
        total += (weights[i] as f32) * inputs[i];
        i += 1;
    }
    
    total
}

/// Fallback scalar implementation
pub fn ternary_dot_product_scalar(weights: &[i8], inputs: &[f32]) -> f32 {
    weights.iter().zip(inputs.iter())
           .filter(|(&w, _)| w != 0) // Software zero-skip
           .map(|(&w, &i)| (w as f32) * i)
           .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avx2_vs_scalar() {
        let weights = vec![1, 0, -1, 0, 1, 1, -1, 0; 8]; // 64 length
        let inputs = vec![0.5; 64];

        let scalar_res = ternary_dot_product_scalar(&weights, &inputs);
        #[cfg(target_arch = "x86_64")]
        {
            if std::is_x86_feature_detected!("avx2") {
                let avx_res = unsafe { ternary_dot_product_avx2(&weights, &inputs) };
                assert!((scalar_res - avx_res).abs() < 1e-5);
            }
        }
    }
}
