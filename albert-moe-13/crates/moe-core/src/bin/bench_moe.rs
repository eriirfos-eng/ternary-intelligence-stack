// MoE-13 Benchmark Suite — RFI-IRFOS Ternary Intelligence Stack
//
// Covers:
//   1. Sparse ternary matmul vs dense f32 — sparsity sweep (TRCE validation)
//   2. Expert domain scoring throughput — all 13 experts
//   3. Full routing pipeline latency — gate + domain bias + top-k selection
//   4. End-to-end expert execution — route + compute on selected experts
//   5. Concurrent routing throughput — multi-threaded query fan-out
//
// Run: cargo run --release --bin bench_moe -p moe-core

use moe_core::core::mock_layer::ternary_matmul_kernel;
use moe_core::core::routing::{ExpertBank13, MoERouter13};
use std::hint::black_box;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

// ── helpers ──────────────────────────────────────────────────────────────────

fn lcg(n: usize, seed: u64) -> Vec<f32> {
    let mut s = seed;
    (0..n)
        .map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((s >> 33) as f32) / (u32::MAX as f32) * 2.0 - 1.0
        })
        .collect()
}

fn lcg_i8(n: usize, sparsity: f32, seed: u64) -> Vec<i8> {
    let mut s = seed;
    (0..n)
        .map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let f = ((s >> 33) as f32) / (u32::MAX as f32);
            if f < sparsity {
                0i8
            } else if f < sparsity + (1.0 - sparsity) / 2.0 {
                1i8
            } else {
                -1i8
            }
        })
        .collect()
}

/// Run `f` for at least `min_duration`, return (iterations, elapsed).
fn bench_loop<F: FnMut()>(mut f: F, min_duration: Duration) -> (u64, Duration) {
    let start = Instant::now();
    let mut iters = 0u64;
    while start.elapsed() < min_duration {
        f();
        iters += 1;
    }
    (iters, start.elapsed())
}

fn throughput(iters: u64, elapsed: Duration) -> f64 {
    iters as f64 / elapsed.as_secs_f64()
}

fn ns_per_iter(iters: u64, elapsed: Duration) -> f64 {
    elapsed.as_nanos() as f64 / iters as f64
}

// ── section 1: sparse matmul vs dense ────────────────────────────────────────

fn bench_sparse_vs_dense() {
    const DIM: usize = 512;
    const WARMUP_ITERS: u64 = 200;
    let min_dur = Duration::from_millis(500);

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!(  "║  §1  Sparse Ternary Matmul vs Dense f32  —  TRCE Validation    ║");
    println!(  "╠══════════════════════════════════════════════════════════════════╣");
    println!("  Matrix: {}×{}    measurement window: 500 ms/cell", DIM, DIM);
    println!();
    println!("  {:>9} | {:>14} | {:>14} | {:>12} | {:>8}",
             "Sparsity", "Dense (µs/op)", "Ternary (µs/op)", "Speedup", "TRCE%");
    println!("  {:->9}-+-{:->14}-+-{:->14}-+-{:->12}-+-{:->8}",
             "", "", "", "", "");

    for &sparsity in &[0.0f32, 0.2, 0.4, 0.6, 0.8] {
        let input = lcg(DIM, 0xbeef);
        let tern_weights = lcg_i8(DIM * DIM, sparsity, 0xdead);
        let f32_weights: Vec<f32> = tern_weights.iter().map(|&w| w as f32).collect();

        // Dense f32 matmul (no zero-skip) — black_box prevents elision
        let mut out_dense = vec![0.0f32; DIM];
        let (d_iters, d_elapsed) = bench_loop(
            || {
                let inp = black_box(&input);
                let fw  = black_box(&f32_weights);
                for i in 0..DIM {
                    let mut s = 0.0f32;
                    let row = i * DIM;
                    for j in 0..DIM {
                        s += fw[row + j] * inp[j];
                    }
                    out_dense[i] = s;
                }
                black_box(&out_dense);
            },
            min_dur,
        );
        let _ = WARMUP_ITERS;

        // Ternary zero-skip kernel
        let mut out_tern = vec![0.0f32; DIM];
        let (t_iters, t_elapsed) = bench_loop(
            || {
                out_tern.iter_mut().for_each(|v| *v = 0.0);
                ternary_matmul_kernel(
                    black_box(&tern_weights),
                    black_box(&input),
                    &mut out_tern, 1.0, DIM, DIM,
                );
                black_box(&out_tern);
            },
            min_dur,
        );

        let dense_us = (d_elapsed.as_nanos() as f64 / d_iters as f64) / 1000.0;
        let tern_us  = (t_elapsed.as_nanos() as f64 / t_iters as f64) / 1000.0;
        let speedup  = dense_us / tern_us;
        let trce_pct = (1.0 - 1.0 / speedup) * 100.0;

        println!("  {:>8.0}% | {:>14.2} | {:>14.2} | {:>11.2}× | {:>7.1}%",
                 sparsity * 100.0, dense_us, tern_us, speedup, trce_pct.max(0.0));
    }
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

// ── section 2: expert domain scoring ─────────────────────────────────────────

fn bench_expert_scoring() {
    const DIM: usize = 64;
    let min_dur = Duration::from_millis(300);

    let bank = ExpertBank13::new(DIM, DIM);

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!(  "║  §2  Expert Domain Scoring Throughput  —  All 13 Experts       ║");
    println!(  "╠══════════════════════════════════════════════════════════════════╣");

    let experts_names = [
        "EthicsExpert", "LegalExpert",      "ScienceExpert",  "CausalExpert",
        "TemporalExpert", "SpatialExpert",  "MathExpert",     "TechnicalExpert",
        "LinguisticExpert", "LogicExpert",  "CulturalExpert", "MedicalExpert",
        "EcologicalExpert",
    ];

    let mut total_qps = 0.0f64;

    for (idx, name) in experts_names.iter().enumerate() {
        let query = lcg(DIM, idx as u64 * 7 + 1);
        let (iters, elapsed) = bench_loop(
            || { bank.domain_scores(&query); },
            min_dur,
        );
        let qps = throughput(iters, elapsed);
        let ns  = ns_per_iter(iters, elapsed);
        total_qps += qps;
        println!("  Expert {:>2} {:>18}  {:>10.0} q/s  {:>8.1} ns/eval",
                 idx, name, qps, ns);
    }

    println!("  ─────────────────────────────────────────────────────────────");
    println!("  Total bank throughput (sum):  {:>10.0} q/s", total_qps);
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

// ── section 3: full routing pipeline ─────────────────────────────────────────

fn bench_routing_pipeline() {
    const DIM: usize = 64;
    const TOP_K: usize = 3;
    let min_dur = Duration::from_millis(500);

    let bank   = ExpertBank13::new(DIM, DIM);
    let router = MoERouter13::new(DIM);
    let query  = lcg(DIM, 0xcafe);

    let (iters, elapsed) = bench_loop(
        || {
            let domain_bias = bank.domain_scores(&query);
            let _selected = router.route(&query, TOP_K, &domain_bias);
        },
        min_dur,
    );

    let qps = throughput(iters, elapsed);
    let us  = ns_per_iter(iters, elapsed) / 1000.0;

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!(  "║  §3  Full Routing Pipeline  —  domain_scores() + route(top-3) ║");
    println!(  "╠══════════════════════════════════════════════════════════════════╣");
    println!("  Input dim:    {}    experts: 13    top-k: {}", DIM, TOP_K);
    println!("  Throughput:   {:.0} routing decisions / second", qps);
    println!("  Latency:      {:.2} µs / routing decision", us);
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

// ── section 4: end-to-end expert execution ────────────────────────────────────

fn bench_e2e_execution() {
    const DIM: usize = 64;
    const TOP_K: usize = 3;
    let min_dur = Duration::from_millis(500);

    let bank   = ExpertBank13::new(DIM, DIM);
    let router = MoERouter13::new(DIM);
    let query  = lcg(DIM, 0xf00d);

    let (iters, elapsed) = bench_loop(
        || {
            let domain_bias = bank.domain_scores(&query);
            let selected    = router.route(&query, TOP_K, &domain_bias);
            for (expert_idx, _weight) in &selected {
                let _ = bank.execute_expert(*expert_idx, &query);
            }
        },
        min_dur,
    );

    let qps = throughput(iters, elapsed);
    let us  = ns_per_iter(iters, elapsed) / 1000.0;

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!(  "║  §4  End-to-End Inference  —  Route + Execute top-3 experts   ║");
    println!(  "╠══════════════════════════════════════════════════════════════════╣");
    println!("  Input dim:    {}    experts: 13    executed: {}", DIM, TOP_K);
    println!("  Throughput:   {:.0} inferences / second", qps);
    println!("  Latency:      {:.2} µs / full inference", us);
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

// ── section 5: concurrent routing throughput ──────────────────────────────────

fn bench_concurrent_routing() {
    const DIM: usize = 64;
    const TOP_K: usize = 3;
    const DURATION_MS: u64 = 1000;

    let bank_arc   = Arc::new(ExpertBank13::new(DIM, DIM));
    let router_arc = Arc::new(MoERouter13::new(DIM));

    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!(  "║  §5  Concurrent Routing Throughput  —  Multi-thread Scaling   ║");
    println!(  "╠══════════════════════════════════════════════════════════════════╣");
    println!("  {:>7} | {:>14} | {:>14} | {:>10}",
             "Threads", "Total (kq/s)", "Per-thread (kq/s)", "Efficiency");
    println!("  {:->7}-+-{:->14}-+-{:->14}-+-{:->10}", "", "", "", "");

    let single_kqps = {
        let bank   = Arc::clone(&bank_arc);
        let router = Arc::clone(&router_arc);
        let query  = lcg(DIM, 42);
        let dur    = Duration::from_millis(DURATION_MS);
        let (iters, elapsed) = bench_loop(
            || {
                let bias = bank.domain_scores(&query);
                let _ = router.route(&query, TOP_K, &bias);
            },
            dur,
        );
        throughput(iters, elapsed) / 1000.0
    };

    for &n_threads in &[1usize, 2, 4, 8] {
        let bank_t   = Arc::clone(&bank_arc);
        let router_t = Arc::clone(&router_arc);
        let dur      = Duration::from_millis(DURATION_MS);

        let handles: Vec<_> = (0..n_threads)
            .map(|t| {
                let b = Arc::clone(&bank_t);
                let r = Arc::clone(&router_t);
                let query = lcg(DIM, t as u64 * 13 + 1);
                thread::spawn(move || {
                    let start = Instant::now();
                    let mut iters = 0u64;
                    while start.elapsed() < dur {
                        let bias = b.domain_scores(&query);
                        let _ = r.route(&query, TOP_K, &bias);
                        iters += 1;
                    }
                    iters
                })
            })
            .collect();

        let total_iters: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
        let total_kqps   = total_iters as f64 / (DURATION_MS as f64 / 1000.0) / 1000.0;
        let per_thr_kqps = total_kqps / n_threads as f64;
        let efficiency   = (per_thr_kqps / single_kqps) * 100.0;

        println!("  {:>7} | {:>14.1} | {:>14.1} | {:>9.1}%",
                 n_threads, total_kqps, per_thr_kqps, efficiency);
    }

    println!("╚══════════════════════════════════════════════════════════════════╝");
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║         RFI-IRFOS TIS — MoE-13 Benchmark Suite v1.0           ║");
    println!("║         Ternary Intelligence Stack · albert-moe-13             ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!("  Build profile:  release");

    // Print hardware info from env if available, else generic label
    let cpu = std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("model name"))
                .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string())
        })
        .unwrap_or_else(|| "unknown CPU".to_string());
    let mem_kb: u64 = std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("MemTotal"))
                .and_then(|l| l.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        })
        .unwrap_or(0);
    println!("  CPU:  {}", cpu);
    println!("  RAM:  {:.1} GB", mem_kb as f64 / 1_048_576.0);
    println!("  GPU:  none (CPU-only inference)");

    bench_sparse_vs_dense();
    bench_expert_scoring();
    bench_routing_pipeline();
    bench_e2e_execution();
    bench_concurrent_routing();

    println!("\n[DONE] All benchmarks complete.");
}
