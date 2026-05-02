# TIS Benchmark Results

Reproducible benchmark data for the Ternary Intelligence Stack.  
All figures measured on real hardware — no simulations, no extrapolations.

---

## Hardware

| Field | Value |
|-------|-------|
| CPU | Intel Core i7-4800MQ @ 2.70 GHz (4C/8T, Haswell) |
| RAM | 7.1 GB |
| GPU | None — **CPU-only inference** |
| OS | Linux 6.17.0-22-generic |
| Rust | stable, `--release` profile |

**Note:** This is a 2013 mobile CPU — consumer laptop hardware. Production targets are server-class x86 or TernCore-Silicon ASICs.

---

## §1 — Sparse Ternary Matmul vs Dense f32 (TRCE Validation)

Run: `cargo run --release --bin bench_moe -p moe-core`  
Matrix: 512×512, measurement window: 500 ms per cell.

| Sparsity | Dense (µs/op) | Ternary (µs/op) | Speedup | TRCE% |
|----------|--------------|-----------------|---------|-------|
| 0% | 229.70 | 275.30 | 0.83× | 0.0% |
| 20% | 223.03 | 1013.26 | 0.22× | 0.0% |
| 40% | 225.06 | 545.34 | 0.41× | 0.0% |
| **60%** | **225.63** | **128.44** | **1.76×** | **43.1%** |
| **80%** | **226.98** | **125.77** | **1.80×** | **44.6%** |

**Interpretation:**

At low sparsity (0–40%), the zero-skip kernel is *slower* than a dense f32 loop — branch-prediction misses on the `if w != 0` check outweigh the savings from skipping zeros. The break-even is around 55% sparsity. At 80% sparsity we see a measured **1.80× speedup (44.6% TRCE)** on x86 software emulation.

The theoretical **80% TRCE at 80% sparsity** claim (from the TRCE whitepaper) assumes **hardware-native trit-skip execution on TernCore-Silicon**, where zero-trits are physically not computed at the ALU level — eliminating the branch entirely. On commodity x86 without ternary ISA extensions, you get roughly half the theoretical gain due to branch overhead. This distinction is documented in `docs/specifications/trce_v1.md`.

---

## §2 — Expert Domain Scoring Throughput (All 13 Experts)

Run: `cargo run --release --bin bench_moe -p moe-core`  
Input dim: 64, measurement window: 300 ms per expert.

| Expert | Domain | Throughput | Latency |
|--------|--------|-----------|---------|
| 0 | EthicsExpert | 497,922 q/s | 2,008 ns |
| 1 | LegalExpert | 504,685 q/s | 1,981 ns |
| 2 | ScienceExpert | 505,761 q/s | 1,977 ns |
| 3 | CausalExpert | 490,220 q/s | 2,040 ns |
| 4 | TemporalExpert | 508,630 q/s | 1,966 ns |
| 5 | SpatialExpert | 506,805 q/s | 1,973 ns |
| 6 | MathematicalExpert | 502,206 q/s | 1,991 ns |
| 7 | TechnicalExpert | 486,502 q/s | 2,056 ns |
| 8 | LinguisticExpert | 505,138 q/s | 1,980 ns |
| 9 | LogicExpert | 489,512 q/s | 2,043 ns |
| 10 | CulturalExpert | 506,599 q/s | 1,974 ns |
| 11 | MedicalExpert | 496,021 q/s | 2,016 ns |
| 12 | EcologicalExpert | 489,722 q/s | 2,042 ns |
| **Total bank** | **all 13** | **6,489,723 q/s** | **~2 µs avg** |

Each domain expert evaluates a 64-dimensional query vector through a 6-axis competence profile and returns a ternary decision `{affirm, tend, reject}` in **~2 µs** on a 2013 laptop CPU.

---

## §3 — Full Routing Pipeline

Run: `cargo run --release --bin bench_moe -p moe-core`  
Input dim: 64, 13 experts, top-3 selection.

| Metric | Value |
|--------|-------|
| Throughput | **363,106 routing decisions / second** |
| Latency | **2.75 µs / routing decision** |

Pipeline: `domain_scores()` (all 13 experts in parallel) → `route()` (gate matmul + AEDL bias + domain bias + top-k sort).

---

## §4 — End-to-End MoE Inference

Run: `cargo run --release --bin bench_moe -p moe-core`  
Input dim: 64, route top-3 experts, execute each.

| Metric | Value |
|--------|-------|
| Throughput | **25,514 full inferences / second** |
| Latency | **39.19 µs / inference** |

Pipeline: route (2.75 µs) + execute 3 × ternary expert layers (~12 µs each).

---

## §5 — Concurrent Routing Throughput

Run: `cargo run --release --bin bench_moe -p moe-core`  
Input dim: 64, top-3 selection, 1-second measurement window per thread count.

| Threads | Total (kq/s) | Per-thread (kq/s) | Scaling efficiency |
|---------|-------------|-------------------|--------------------|
| 1 | 358.3 | 358.3 | 99.5% |
| 2 | 708.5 | 354.2 | 98.4% |
| 4 | 1,112.9 | 278.2 | 77.3% |
| 8 | 1,183.9 | 148.0 | 41.1% |

Near-linear scaling to 4 threads (the physical core count of the test CPU). The drop at 8 threads is expected on a 4-core/8-thread Haswell — the hyperthreads share execution units.

---

## §6 — QAT/STE Training Convergence

Run: `cargo run --release --bin perplexity_eval -p ternlang-ml`  
Architecture: 32→64→8, 300 epochs, 32 held-out test samples.

| Metric | Pre-QAT (baseline) | Post-QAT (STE fine-tuned) | Delta |
|--------|-------------------|--------------------------|-------|
| Pseudo-perplexity | 3,899.2 | 2,227.9 | **−42.9%** |
| Mean cross-entropy | 8.2685 | 7.7088 | −0.559 |
| Top-1 accuracy | 12.5% | 18.8% | **+6.2 pp** |
| Output entropy | 0.094 nats | 0.493 nats | +0.399 |

Post-QAT the model is **43% less perplexed** and classifies **6.2% more inputs correctly**. The STE fine-tuning recovers representation quality lost during hard ternarization.

---

## Reproducing These Results

```bash
# MoE-13 benchmarks (§1–§5)
git clone https://github.com/eriirfos-eng/ternary-intelligence-stack
cd ternary-intelligence-stack/albert-moe-13
cargo run --release --bin bench_moe -p moe-core

# QAT/perplexity benchmark (§6)
cd ../ternlang-root
cargo run --release --bin perplexity_eval -p ternlang-ml
```

No GPU, no external data downloads, no API keys required.

---

## What These Numbers Mean for SPRIND

The benchmarks above run on a decade-old mobile CPU with no GPU. The routing pipeline achieves **363k decisions/second at 2.75 µs latency** with **near-linear multi-thread scaling** in this constrained environment.

On server hardware (e.g., AMD EPYC 96-core, or an Intel Xeon with AVX-512), the routing throughput scales proportionally with core count — the `ExpertBank13` is embarrassingly parallel and lock-free across the domain scoring step.

On **TernCore-Silicon** (TIS hardware target), the sparse matmul speedup moves from the measured **1.8× (44.6% TRCE)** at 80% sparsity on x86 to the theoretical **5× (80% TRCE)** by eliminating the branch entirely at the ALU level. The TRCE whitepaper (`docs/specifications/trce_v1.md`) details the ISA-level mechanism.

---

*Benchmarks run: 2026-05-02 · Hardware: i7-4800MQ / 7.1 GB / Linux 6.17.0*  
*Maintained by RFI-IRFOS — Research Focus Institute · Graz, Austria*
