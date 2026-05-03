//! # Ternary SIMD Kernels
//! 
//! Hardware-accelerated AVX2 implementations for ternary math.

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Standard Optimized INT8 Dense Dot Product (Baseline)
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn dot_int8_dense_avx2(weights: &[i8], inputs: &[f32]) -> f32 {
    let len = weights.len().min(inputs.len());
    let mut sum = _mm256_setzero_ps();
    let mut i = 0;
    while i + 31 < len {
        for j in 0..4 {
            let offset = i + j * 8;
            let w_int = _mm_loadl_epi64(weights.as_ptr().add(offset) as *const __m128i);
            let w_f = _mm256_cvtepi32_ps(_mm256_cvtepi8_epi32(w_int));
            let in_f = _mm256_loadu_ps(inputs.as_ptr().add(offset));
            sum = _mm256_fmadd_ps(w_f, in_f, sum);
        }
        i += 32;
    }
    let mut result_arr = [0.0f32; 8];
    _mm256_storeu_ps(result_arr.as_mut_ptr(), sum);
    result_arr.iter().sum()
}

/// Ternary Dot Product with Block-Level Zero-Skipping
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn ternary_dot_product_skip_avx2(weights: &[i8], inputs: &[f32]) -> f32 {
    let len = weights.len().min(inputs.len());
    let mut sum = _mm256_setzero_ps();
    let zero_vec = _mm256_setzero_si256();
    let mut i = 0;
    while i + 31 < len {
        let w_block = _mm256_loadu_si256(weights.as_ptr().add(i) as *const __m256i);
        let mask = _mm256_cmpeq_epi8(w_block, zero_vec);
        let movemask = _mm256_movemask_epi8(mask) as u32;
        
        if movemask == 0xFFFFFFFF {
            i += 32;
            continue;
        }

        for j in 0..4 {
            let offset = i + j * 8;
            let w_int = _mm_loadl_epi64(weights.as_ptr().add(offset) as *const __m128i);
            let w_f = _mm256_cvtepi32_ps(_mm256_cvtepi8_epi32(w_int));
            let in_f = _mm256_loadu_ps(inputs.as_ptr().add(offset));
            sum = _mm256_fmadd_ps(w_f, in_f, sum);
        }
        i += 32;
    }
    let mut result_arr = [0.0f32; 8];
    _mm256_storeu_ps(result_arr.as_mut_ptr(), sum);
    result_arr.iter().sum()
}

/// Legacy wrapper for backward compatibility
pub unsafe fn ternary_dot_product_avx2(weights: &[i8], inputs: &[f32]) -> f32 {
    ternary_dot_product_skip_avx2(weights, inputs)
}

pub fn ternary_dot_product_scalar(weights: &[i8], inputs: &[f32]) -> f32 {
    weights.iter().zip(inputs.iter())
           .filter(|(&w, _)| w != 0)
           .map(|(&w, &i)| (w as f32) * i)
           .sum()
}
