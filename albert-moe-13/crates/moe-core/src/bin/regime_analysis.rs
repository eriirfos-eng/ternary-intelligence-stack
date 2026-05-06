//! # Sparsity Regime Analysis
//! 
//! High-resolution measurement and statistical analysis of Zero-Skip regimes.

use anyhow::Result;
use std::time::Instant;
use std::hint::black_box;
use rand::Rng;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

const DIM: usize = 1024 * 1024 * 128; // 128M elements for stability
const ITERS: usize = 10; // 10 runs per point
const WORK_STEPS: usize = 5; // internal steps per run

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
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

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
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

fn run_stats(sparsity: f32) -> (f64, f64, f64) {
    let mut rng = rand::thread_rng();
    let mut weights = vec![0i8; DIM];
    let inputs = vec![0.5f32; DIM];
    
    let num_blocks = DIM / 32;
    for i in 0..num_blocks {
        if rng.r#gen::<f32>() > sparsity {
            for j in 0..32 {
                weights[i * 32 + j] = 1;
            }
        }
    }

    let mut latencies = Vec::new();
    let mut int8_baseline = 0.0;

    unsafe {
        // Baseline INT8 (Mean of 10)
        for _ in 0..ITERS {
            let start = Instant::now();
            for _ in 0..WORK_STEPS { let _ = black_box(dot_int8_dense(&weights, &inputs)); }
            int8_baseline += start.elapsed().as_secs_f64() / WORK_STEPS as f64;
        }
        int8_baseline /= ITERS as f64;

        // Ternary Skip
        for _ in 0..ITERS {
            let start = Instant::now();
            for _ in 0..WORK_STEPS { let _ = black_box(dot_ternary_skip(&weights, &inputs)); }
            latencies.push(start.elapsed().as_secs_f64() / WORK_STEPS as f64);
        }
    }

    let mean = latencies.iter().sum::<f64>() / ITERS as f64;
    let variance = latencies.iter().map(|l| (l - mean).powi(2)).sum::<f64>() / ITERS as f64;
    let std_dev = variance.sqrt();

    (mean * 1000.0, (int8_baseline * 1000.0) / (mean * 1000.0), std_dev * 1000.0)
}

fn main() -> Result<()> {
    println!("--- SPARSITY REGIME ANALYSIS ---");
    let levels = [0.0, 0.05, 0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.75, 0.9];
    
    println!("\n| Sparsity | Latency (ms) | Speedup | StdDev (ms) |");
    println!("|----------|--------------|---------|-------------|");

    for &s in levels.iter() {
        let (mean, speedup, stddev) = run_stats(s);
        println!("| {:.0}% | {:.4} | {:.2}x | {:.4} |", 
            s * 100.0, mean, speedup, stddev);
    }

    Ok(())
}
