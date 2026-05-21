//! # Zero-Skip Advantage Proof
//! 
//! Proving that Ternary zero-skipping (HOLD state) outperforms optimized INT8.

use anyhow::Result;
use std::time::Instant;
use std::hint::black_box;
use rand::Rng;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Benchmark Config
const DIM: usize = 1024 * 1024 * 128; // 128 Million elements
const STEPS: usize = 10;

/// --- [1] INT8 OPTIMIZED (DENSE) ---
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
// SAFETY: AVX2 guaranteed by `#[target_feature]`. Loop bound `i + 31 < len` keeps all
// `[offset, offset+8)` accesses (j ∈ 0..4) within both slices. Stack `res[8]` always valid.
pub unsafe fn dot_int8_dense(weights: &[i8], inputs: &[f32]) -> f32 {
    let mut sum = _mm256_setzero_ps();
    let mut i = 0;
    while i + 31 < weights.len() {
        for j in 0..4 {
            let offset = i + j * 8;
            let w_int = _mm_loadl_epi64(weights.as_ptr().add(offset) as *const __m128i);
            let w_f = _mm256_cvtepi32_ps(_mm256_cvtepi8_epi32(w_int));
            let in_f = _mm256_loadu_ps(inputs.as_ptr().add(offset));
            sum = _mm256_fmadd_ps(w_f, in_f, sum);
        }
        i += 32;
    }
    let mut res = [0.0f32; 8];
    _mm256_storeu_ps(res.as_mut_ptr(), sum);
    res.iter().sum()
}

/// --- [2] TERNARY ZERO-SKIP (BLOCK-BASED) ---
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
// SAFETY: AVX2 guaranteed by `#[target_feature]`. `_mm256_loadu_si256` reads 32 bytes at
// `weights.ptr+i`: valid because `i + 31 < len`. Inner offsets satisfy the same bound as
// `dot_int8_dense`. Stack `res[8]` always valid.
pub unsafe fn dot_ternary_skip(weights: &[i8], inputs: &[f32]) -> f32 {
    let mut sum = _mm256_setzero_ps();
    let mut i = 0;
    let zero_vec = _mm256_setzero_si256();
    
    while i + 31 < weights.len() {
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
    let mut res = [0.0f32; 8];
    _mm256_storeu_ps(res.as_mut_ptr(), sum);
    res.iter().sum()
}

fn run_test(sparsity: f32) -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let mut weights = vec![0i8; DIM];
    let inputs = vec![0.5f32; DIM];
    
    // Randomize blocks to be either all-zero or dense
    let num_blocks = DIM / 32;
    for i in 0..num_blocks {
        if rng.r#gen::<f32>() > sparsity {
            for j in 0..32 {
                weights[i * 32 + j] = 1;
            }
        }
    }

    // SAFETY: Both kernel functions require AVX2 (guaranteed by `#[target_feature]`) and
    // matching slice lengths (`weights` and `inputs` are both `DIM` elements).
    unsafe {
        let mut t1 = 0.0;
        let mut t2 = 0.0;

        // Force evaluate
        let r1 = black_box(dot_int8_dense(&weights, &inputs));
        let r2 = black_box(dot_ternary_skip(&weights, &inputs));
        assert!((r1 - r2).abs() < 1.0);

        let start = Instant::now();
        for _ in 0..STEPS { 
            t1 += black_box(dot_int8_dense(black_box(&weights), black_box(&inputs))); 
        }
        let int8_dur = start.elapsed().as_nanos() as f64 / STEPS as f64;

        let start = Instant::now();
        for _ in 0..STEPS { 
            t2 += black_box(dot_ternary_skip(black_box(&weights), black_box(&inputs))); 
        }
        let tern_dur = start.elapsed().as_nanos() as f64 / STEPS as f64;

        if t1 == t2 && t1 == 0.0 { println!("Warning: Possible elision"); }

        (int8_dur / 1_000_000.0, tern_dur / 1_000_000.0)
    }
}

fn main() -> Result<()> {
    println!("--- ZERO-SKIP ADVANTAGE PROOF ---");
    println!("Workload: 128M elements | Block Size: 32");
    
    let levels = [0.0, 0.25, 0.5, 0.75, 0.9];
    
    println!("\n| Sparsity | INT8 Latency (ms) | Ternary Latency (ms) | Speedup |");
    println!("|----------|-------------------|----------------------|---------|");

    for &s in levels.iter() {
        let (int8_lat, tern_lat) = run_test(s);
        println!("| {:.0}% | {:.4} | {:.4} | {:.2}x |", 
            s * 100.0, int8_lat, tern_lat, int8_lat / tern_lat);
    }

    Ok(())
}
