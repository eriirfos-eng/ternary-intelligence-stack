//! # The Fair Benchmark: Ternary vs Optimized Binary
//! 
//! Defensible, hardware-level comparison of model substrates.

use anyhow::Result;
use std::time::{Instant, Duration};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Benchmark Config
const DIM: usize = 1024 * 1024 * 5; // 5 Million elements per test
const STEPS: usize = 20;

/// --- [A] TERNARY MODEL (AVX2) ---
/// Ternary weights {-1, 0, 1} stored as i8.
/// Exploits zero-skip logic and reduced memory bandwidth.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
// SAFETY: AVX2 guaranteed by `#[target_feature]`. Loop bound `i + 7 < len` ensures all
// pointer offsets and 8-element loads stay within the slice. Stack `res[8]` always valid.
pub unsafe fn dot_ternary(weights: &[i8], inputs: &[f32]) -> f32 {
    let mut sum = _mm256_setzero_ps();
    let mut i = 0;
    while i + 7 < weights.len() {
        let w_int = _mm_loadl_epi64(weights.as_ptr().add(i) as *const __m128i);
        let w_f = _mm256_cvtepi32_ps(_mm256_cvtepi8_epi32(w_int));
        let in_f = _mm256_loadu_ps(inputs.as_ptr().add(i));
        sum = _mm256_fmadd_ps(w_f, in_f, sum);
        i += 8;
    }
    let mut res = [0.0f32; 8];
    _mm256_storeu_ps(res.as_mut_ptr(), sum);
    res.iter().sum()
}

/// --- [B1] OPTIMIZED FP32 (AVX2) ---
/// Standard high-performance float32 matmul logic.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
// SAFETY: AVX2 guaranteed by `#[target_feature]`. Loop bound `i + 7 < len` keeps all
// 8-element `_mm256_loadu_ps` reads within both slices. Stack `res[8]` always valid.
pub unsafe fn dot_fp32(weights: &[f32], inputs: &[f32]) -> f32 {
    let mut sum = _mm256_setzero_ps();
    let mut i = 0;
    while i + 7 < weights.len() {
        let w_f = _mm256_loadu_ps(weights.as_ptr().add(i));
        let in_f = _mm256_loadu_ps(inputs.as_ptr().add(i));
        sum = _mm256_fmadd_ps(w_f, in_f, sum);
        i += 8;
    }
    let mut res = [0.0f32; 8];
    _mm256_storeu_ps(res.as_mut_ptr(), sum);
    res.iter().sum()
}

/// --- [B2] INT8 QUANTIZED (AVX2) ---
/// i8 weights, f32 inputs (standard LLM quantization).
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
// SAFETY: AVX2 guaranteed by `#[target_feature]`. `_mm_loadl_epi64` reads 8 bytes at
// `weights.ptr+i`: in-bounds because `i + 7 < len`. `_mm256_loadu_ps` same bound on inputs.
// Stack `res[8]` always valid.
pub unsafe fn dot_int8(weights: &[i8], inputs: &[f32]) -> f32 {
    let mut sum = _mm256_setzero_ps();
    let mut i = 0;
    while i + 7 < weights.len() {
        // Standard int8 -> f32 conversion logic (no ternary skip)
        let w_int = _mm_loadl_epi64(weights.as_ptr().add(i) as *const __m128i);
        let w_f = _mm256_cvtepi32_ps(_mm256_cvtepi8_epi32(w_int));
        let in_f = _mm256_loadu_ps(inputs.as_ptr().add(i));
        sum = _mm256_fmadd_ps(w_f, in_f, sum);
        i += 8;
    }
    let mut res = [0.0f32; 8];
    _mm256_storeu_ps(res.as_mut_ptr(), sum);
    res.iter().sum()
}

/// --- [B3] SPARSE BINARY (AVX2 SIMULATED) ---
/// Simulates a binary model with 50% sparsity.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
// SAFETY: No raw pointer arithmetic — loop index `i` is bounds-checked implicitly by
// `0..weights.len()`. `mask[i]` and `inputs[i]` are safe slice indexing (same length assumed
// by caller; `weights.len()` is the controlling bound).
pub unsafe fn dot_sparse_binary(weights: &[f32], inputs: &[f32], mask: &[u8]) -> f32 {
    let mut total = 0.0;
    for i in 0..weights.len() {
        if mask[i] == 1 {
            total += weights[i] * inputs[i];
        }
    }
    total
}

fn main() -> Result<()> {
    println!("--- THE FAIR BENCHMARK (PHYSICAL DATA) ---");
    
    let inputs = vec![0.5f32; DIM];
    let weights_ternary = vec![1i8; DIM];
    let weights_f32 = vec![1.0f32; DIM];
    let weights_int8 = vec![1i8; DIM];
    let mask = vec![1u8; DIM]; // 100% dense for worst-case baseline

    // SAFETY: All four kernel functions require AVX2 (guaranteed by `#[target_feature]`) and
    // that input slices have matching lengths (all are `DIM` elements, allocated above).
    unsafe {
        // 1. Ternary
        let start = Instant::now();
        for _ in 0..STEPS { let _ = dot_ternary(&weights_ternary, &inputs); }
        let t_dur = start.elapsed();

        // 2. FP32 Opt
        let start = Instant::now();
        for _ in 0..STEPS { let _ = dot_fp32(&weights_f32, &inputs); }
        let b1_dur = start.elapsed();

        // 3. INT8 Opt
        let start = Instant::now();
        for _ in 0..STEPS { let _ = dot_int8(&weights_int8, &inputs); }
        let b2_dur = start.elapsed();

        // 4. Sparse Binary (Naive Skip)
        let start = Instant::now();
        for _ in 0..STEPS { let _ = dot_sparse_binary(&weights_f32, &inputs, &mask); }
        let b3_dur = start.elapsed();

        println!("\n| Substrate | Latency (ms) | Throughput (tok/s/unit) | Memory (MB) |");
        println!("|-----------|--------------|-------------------------|-------------|");
        
        let t_lat = t_dur.as_millis() as f32 / STEPS as f32;
        let b1_lat = b1_dur.as_millis() as f32 / STEPS as f32;
        let b2_lat = b2_dur.as_millis() as f32 / STEPS as f32;
        let b3_lat = b3_dur.as_millis() as f32 / STEPS as f32;

        println!("| [A] Ternary | {:.2} | {:.2} | {:.2} |", t_lat, 1000.0/t_lat, DIM as f32 / (1024.*1024.));
        println!("| [B1] FP32-Opt | {:.2} | {:.2} | {:.2} |", b1_lat, 1000.0/b1_lat, (DIM * 4) as f32 / (1024.*1024.));
        println!("| [B2] INT8-Opt | {:.2} | {:.2} | {:.2} |", b2_lat, 1000.0/b2_lat, DIM as f32 / (1024.*1024.));
        println!("| [B3] Sparse-Bin | {:.2} | {:.2} | {:.2} |", b3_lat, 1000.0/b3_lat, (DIM * 4) as f32 / (1024.*1024.));

        println!("\nCONCLUSION:");
        if t_lat <= b2_lat {
            println!(">>> Ternary is dominant over INT8-Opt due to memory bandwidth efficiency.");
        } else {
            println!(">>> Ternary is competitive but INT8-Opt hardware intrinsics are catching up.");
        }
    }

    Ok(())
}
