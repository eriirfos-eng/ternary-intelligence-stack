//! # Final Sparsity Curve Benchmark
//! 
//! High-resolution measurement of Zero-Skip performance scaling.

use anyhow::Result;
use std::time::Instant;
use std::hint::black_box;
use rand::Rng;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

const DIM: usize = 1024 * 1024 * 128; // 128M elements
const STEPS: usize = 20;

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

fn run_point(sparsity: f32) -> (f64, f64) {
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

    unsafe {
        let mut t1 = 0.0;
        let mut t2 = 0.0;

        let start = Instant::now();
        for _ in 0..STEPS { t1 += black_box(dot_int8_dense(black_box(&weights), black_box(&inputs))); }
        let int8_dur = start.elapsed().as_nanos() as f64 / STEPS as f64;

        let start = Instant::now();
        for _ in 0..STEPS { t2 += black_box(dot_ternary_skip(black_box(&weights), black_box(&inputs))); }
        let tern_dur = start.elapsed().as_nanos() as f64 / STEPS as f64;

        // Prevent elision by side effect
        if t1 == t2 && t1 == 0.0 { println!("DANGER: Elision detected."); }

        (int8_dur / 1_000_000.0, tern_dur / 1_000_000.0)
    }
}

fn main() -> Result<()> {
    println!("--- FINAL SPARSITY CURVE (HIGH RES) ---");
    let levels = [0.0, 0.1, 0.25, 0.4, 0.5, 0.6, 0.75, 0.9];
    
    println!("\n| Sparsity | INT8 Latency (ms) | Ternary Latency (ms) | Speedup |");
    println!("|----------|-------------------|----------------------|---------|");

    for &s in levels.iter() {
        let (int8_lat, tern_lat) = run_point(s);
        println!("| {:.0}% | {:.4} | {:.4} | {:.2}x |", 
            s * 100.0, int8_lat, tern_lat, int8_lat / tern_lat);
    }

    Ok(())
}
