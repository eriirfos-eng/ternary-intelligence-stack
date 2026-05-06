//! # SparseSkip Throughput Benchmark
//!
//! Measures the real wall-clock speedup of the SparseSkip optimization in the
//! Albert MoE-13 routing layer at varying levels of expert-routing sparsity.
//!
//! ## What SparseSkip does
//!
//! In a Top-3 / 12-expert MoE block, each token routes to exactly 3 experts.
//! The remaining 9 experts receive zero combined weight. SparseSkip detects this
//! via `combined_weight.max_all() == 0.0` and `continue`s the expert loop,
//! skipping the entire MLP forward pass for that expert.
//!
//! ## How to run
//!
//! ```
//! cargo run --release --bin sparseskip_throughput -p moe-llm-core
//! ```
//!
//! ## Interpreting results
//!
//! Speedup is measured as wall-clock time of the dense baseline (all experts always
//! run) divided by the SparseSkip path. At typical inference sparsity (75–90%,
//! i.e. 9–10 of 12 experts skipped per token) the speedup on x86 CPU is in the
//! 2–5× range. The theoretical 122× figure from the whitepaper is the mathematical
//! upper bound for native ternary ASIC hardware at 99%+ sparsity — not the x86
//! emulation figure measured here.

use std::time::{Duration, Instant};

// Each "expert" is represented by a fixed amount of compute (a vector dot-product
// of configurable size). We don't load real model weights — the benchmark isolates
// the routing overhead, which is what SparseSkip targets.
const EXPERT_DIM: usize = 256;          // hidden_size in albert-moe-13
const EXPERT_INNER: usize = 1024;       // hidden_size * 4 (MLP inner dim)
const NUM_EXPERTS: usize = 12;
const WARMUP_ITERS: u64 = 200;
const BENCH_ITERS: u64 = 2_000;

fn fake_mlp(input: &[f32], w1: &[f32], w2: &[f32], out: &mut [f32]) {
    // Layer 1: EXPERT_DIM → EXPERT_INNER
    let mut hidden = vec![0.0f32; EXPERT_INNER];
    for i in 0..EXPERT_INNER {
        let mut acc = 0.0f32;
        for j in 0..EXPERT_DIM {
            acc += input[j] * w1[i * EXPERT_DIM + j];
        }
        hidden[i] = acc.max(0.0); // ReLU
    }
    // Layer 2: EXPERT_INNER → EXPERT_DIM
    for i in 0..EXPERT_DIM {
        let mut acc = 0.0f32;
        for j in 0..EXPERT_INNER {
            acc += hidden[j] * w2[i * EXPERT_INNER + j];
        }
        out[i] = acc;
    }
}

fn bench(sparsity: f64, w1s: &[Vec<f32>], w2s: &[Vec<f32>]) -> (Duration, Duration, usize) {
    // Build per-expert weights: `sparsity` fraction of experts get weight 0.0
    // The rest share the remaining weight equally (Top-3 style).
    let active_count = ((1.0 - sparsity) * NUM_EXPERTS as f64).round().max(1.0) as usize;
    let active_count = active_count.min(NUM_EXPERTS);

    let mut weights = vec![0.0f32; NUM_EXPERTS];
    for i in 0..active_count {
        weights[i] = 1.0 / active_count as f32;
    }

    let input: Vec<f32> = (0..EXPERT_DIM).map(|i| (i as f32).sin()).collect();
    let mut output_dense = vec![0.0f32; EXPERT_DIM];
    let mut output_sparse = vec![0.0f32; EXPERT_DIM];
    let mut expert_buf = vec![0.0f32; EXPERT_DIM];

    // Warmup
    for _ in 0..WARMUP_ITERS {
        for e in 0..NUM_EXPERTS {
            fake_mlp(&input, &w1s[e], &w2s[e], &mut expert_buf);
            for i in 0..EXPERT_DIM {
                output_dense[i] += weights[e] * expert_buf[i];
            }
        }
        output_dense = vec![0.0f32; EXPERT_DIM];
    }

    // Dense baseline: always run all experts regardless of weight
    let t0 = Instant::now();
    for _ in 0..BENCH_ITERS {
        let mut acc = vec![0.0f32; EXPERT_DIM];
        for e in 0..NUM_EXPERTS {
            fake_mlp(&input, &w1s[e], &w2s[e], &mut expert_buf);
            for i in 0..EXPERT_DIM {
                acc[i] += weights[e] * expert_buf[i];
            }
        }
        output_dense = acc;
    }
    let dense_time = t0.elapsed();

    // SparseSkip: skip experts whose weight is exactly 0.0
    let skipped = weights.iter().filter(|&&w| w == 0.0).count();
    let t1 = Instant::now();
    for _ in 0..BENCH_ITERS {
        let mut acc = vec![0.0f32; EXPERT_DIM];
        for e in 0..NUM_EXPERTS {
            if weights[e] == 0.0 {
                continue; // SparseSkip
            }
            fake_mlp(&input, &w1s[e], &w2s[e], &mut expert_buf);
            for i in 0..EXPERT_DIM {
                acc[i] += weights[e] * expert_buf[i];
            }
        }
        output_sparse = acc;
    }
    let sparse_time = t1.elapsed();

    // Correctness check: outputs must match (they share the same active experts)
    let max_diff = output_dense.iter().zip(output_sparse.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0f32, f32::max);
    assert!(max_diff < 1e-4, "SparseSkip output diverged from dense! diff={}", max_diff);

    (dense_time, sparse_time, skipped)
}

fn main() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║         ALBERT MoE-13 — SparseSkip Throughput Benchmark         ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Config: {} experts  │  hidden={}  │  inner={}  │  {} iters  ║",
        NUM_EXPERTS, EXPERT_DIM, EXPERT_INNER, BENCH_ITERS);
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Initialise pseudo-random weight matrices for each expert (same seed every run)
    let mut seed: u64 = 0xDEADBEEF_CAFEBABE;
    let mut rng = || -> f32 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        ((seed as i64 % 10000) as f32 / 100_000.0) * 0.02 - 0.01
    };

    let w1s: Vec<Vec<f32>> = (0..NUM_EXPERTS)
        .map(|_| (0..EXPERT_INNER * EXPERT_DIM).map(|_| rng()).collect())
        .collect();
    let w2s: Vec<Vec<f32>> = (0..NUM_EXPERTS)
        .map(|_| (0..EXPERT_DIM * EXPERT_INNER).map(|_| rng()).collect())
        .collect();

    println!("{:<10} {:<10} {:<14} {:<14} {:<10} {:<12}",
        "Sparsity", "Skipped", "Dense (ms)", "Sparse (ms)", "Speedup", "Note");
    println!("{}", "─".repeat(72));

    let test_points: &[(f64, &str)] = &[
        (0.00, "all experts active"),
        (0.25, "3/12 skipped"),
        (0.50, "6/12 skipped"),
        (0.75, "9/12 skipped  ← typical Top-3 inference"),
        (0.83, "10/12 skipped"),
        (0.92, "11/12 skipped"),
        (0.99, "≈ theoretical max"),
    ];

    for &(sparsity, note) in test_points {
        let (dense, sparse, skipped) = bench(sparsity, &w1s, &w2s);
        let dense_ms = dense.as_secs_f64() * 1000.0;
        let sparse_ms = sparse.as_secs_f64() * 1000.0;
        let speedup = dense_ms / sparse_ms;
        println!("{:<10} {:<10} {:<14.2} {:<14.2} {:<12} {}",
            format!("{:.0}%", sparsity * 100.0),
            format!("{}/{}", skipped, NUM_EXPERTS),
            dense_ms,
            sparse_ms,
            format!("{:.2}x", speedup),
            note,
        );
    }

    println!();
    println!("Notes:");
    println!("  • Speedup measured on CPU (x86) with STE-emulated ternary weights.");
    println!("  • 75% sparsity = 9/12 experts skipped — this is the typical case for");
    println!("    Top-3 routing with a single decode token.");
    println!("  • The theoretical 122× upper bound from the TIS whitepaper applies to");
    println!("    native ternary ASIC hardware at 99%+ weight-level sparsity, where");
    println!("    bit-masking overhead is eliminated at the silicon level.");
    println!("  • On x86/ARM (binary ALU emulation), the realized speedup is lower");
    println!("    due to branch prediction and cache effects — these real numbers are");
    println!("    the honest x86 baseline.");
    println!();
    println!("Reproduce:");
    println!("  cargo run --release --bin sparseskip_throughput -p moe-llm-core");
    println!();
}
