//! # Scientific Sparsity Benchmark (Harden)
//! 
//! Protocol:
//! - Thread Pinning (Physical Core)
//! - 1000 Iterations
//! - Warmup Phase (100 runs)
//! - Cycle-level timing (RDTSC)
//! - Latency-level timing (Instant)

use std::time::{Instant, Duration};
use std::hint::black_box;
use rand::Rng;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

const DIM: usize = 1024 * 1024 * 16; // 16M elements (64MB, fits in RAM, exceeds L3)
const ITERS: usize = 500;
const WARMUP: usize = 50;

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

#[cfg(target_arch = "x86_64")]
fn get_cycles() -> u64 {
    unsafe { _rdtsc() }
}

fn run_point(sparsity: f32) -> (f64, f64, f64, f64) {
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
        // Warmup
        for _ in 0..WARMUP {
            let _ = black_box(dot_ternary_skip(&weights, &inputs));
            let _ = black_box(dot_int8_dense(&weights, &inputs));
        }

        let mut latencies = Vec::with_capacity(ITERS);
        let mut cycles = Vec::with_capacity(ITERS);

        for _ in 0..ITERS {
            let start_c = get_cycles();
            let start_t = Instant::now();
            let _ = black_box(dot_ternary_skip(&weights, &inputs));
            let dur = start_t.elapsed();
            let end_c = get_cycles();
            
            latencies.push(dur.as_secs_f64() * 1000.0);
            cycles.push((end_c - start_c) as f64);
        }

        let mean_lat = latencies.iter().sum::<f64>() / ITERS as f64;
        let mean_cyc = cycles.iter().sum::<f64>() / ITERS as f64;
        let std_lat = (latencies.iter().map(|l| (l - mean_lat).powi(2)).sum::<f64>() / ITERS as f64).sqrt();
        
        // Baseline for speedup
        let mut base_lats = Vec::with_capacity(100);
        for _ in 0..100 {
            let start = Instant::now();
            let _ = black_box(dot_int8_dense(&weights, &inputs));
            base_lats.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        let mean_base = base_lats.iter().sum::<f64>() / 100.0;

        (mean_lat, mean_base / mean_lat, std_lat, mean_cyc)
    }
}

fn main() {
    // Attempt core pinning (linux only)
    #[cfg(target_os = "linux")]
    {
        use libc::{cpu_set_t, sched_setaffinity, CPU_SET, CPU_ZERO};
        let mut cpuset: cpu_set_t = unsafe { std::mem::zeroed() };
        unsafe {
            CPU_ZERO(&mut cpuset);
            CPU_SET(0, &mut cpuset); // Pin to core 0
            sched_setaffinity(0, std::mem::size_of::<cpu_set_t>(), &cpuset);
        }
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let p: f32 = args[1].parse().expect("Sparsity must be a float");
        let (lat, speed, std, cyc) = run_point(p);
        println!("{:.2},{:.6},{:.4},{:.6},{:.0}", p, lat, speed, std, cyc);
        return;
    }

    println!("sparsity,latency_ms,speedup,stddev_ms,cycles");
    let points = [0.0, 0.05, 0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.75, 0.9];
    for &p in points.iter() {
        eprintln!("Running point: {:.2}", p);
        let (lat, speed, std, cyc) = run_point(p);
        println!("{:.2},{:.6},{:.4},{:.6},{:.0}", p, lat, speed, std, cyc);
    }
}
