# @sparseskip — Benchmark Methodology & Measured Results

**Patent Pending A50296/2026 | RFI-IRFOS | 2026-05-09**

---

## What @sparseskip Does

In Albert's Top-3 / 12-expert Mixture-of-Experts architecture, each input token is routed to exactly 3 experts. The remaining 9 experts receive a combined routing weight of exactly zero.

`@sparseskip` exploits this: when `combined_weight.max_all() == 0.0`, the entire expert MLP is skipped — not masked, not zeroed-out, **not executed**. The forward pass for that expert does not run.

At single-token decode (the typical autoregressive inference step), this skips 9 of 12 expert MLPs — 75% of the expert network.

The same primitive is expressed as `TSPARSE_MATMUL` in the BET VM instruction set, enabling element-level sparse multiply at the opcode level (zero weights: no multiply, no memory access).

---

## Benchmark Setup

**Binary:** `sparseskip_throughput` (in `moe-llm-core`)
**Hardware:** x86 CPU (laptop-class, no GPU)
**Model config:** 12 experts, hidden=256, inner=1024 (matching Albert MoE-13)
**Method:** 200 warmup iterations + 2000 timed iterations per sparsity level
**Correctness:** Dense and sparse outputs verified to match within 1e-4 at every sparsity level

Each "expert" runs a two-layer MLP (256→1024→256) with ReLU. Weights are pseudo-random (fixed seed — reproducible). The benchmark isolates routing overhead, which is exactly what @sparseskip targets.

**Date run:** 2026-05-09
**Reproduce:**
```
cargo run --release --bin sparseskip_throughput -p moe-llm-core
```

---

## Results

| Sparsity | Experts skipped | Dense (ms) | Sparse (ms) | Speedup | Scenario |
|---|---|---|---|---|---|
| 0% | 0/12 | 26,638 | 30,953 | 0.86× | All experts active (branch overhead) |
| 25% | 3/12 | 26,420 | 18,907 | 1.40× | |
| 50% | 6/12 | 26,866 | 14,390 | 1.87× | |
| **75%** | **9/12** | **29,508** | **6,443** | **4.58×** | **← Typical Top-3 decode** |
| 83% | 10/12 | 24,364 | 3,882 | 6.28× | |
| 92% | 11/12 | 32,435 | 2,347 | 13.82× | |
| 99% | 11/12 | 33,246 | 3,342 | 9.95× | Cache effects at near-max |

**At the typical Top-3 inference operating point (75% sparsity, 9/12 experts skipped): 4.58× speedup over dense execution on x86 CPU.**

---

## The 83 tok/s Figure

The 83 tokens/second sustained decode figure (measured 2026-05-07) reflects the full inference stack combining three contributions:

1. **@sparseskip expert routing** — 9/12 experts not executing per decode step (4.58× contribution at 75% sparsity)
2. **KV-cache** — single query token attends over cached K/V, eliminating recomputation of past context
3. **Pre-ternarized weight cache** — `prepare_inference()` quantizes weights once at model load; forward passes operate on the cached ternary representation

These three combine multiplicatively in the inference loop. The benchmark above isolates contribution #1.

**Hardware context:** ThinkPad-class laptop CPU, 8 threads (Rayon), Ubuntu 24.04, Rust release build. No GPU. No INT8 kernel. No framework acceleration.

---

## Honest Scope

This benchmark measures expert-level skip on x86 with emulated ternary weights (STE). It does not measure:

- Element-level weight sparsity within active experts (a separate, larger optimization — work in progress)
- Native ternary hardware (the theoretical 122× figure in the whitepaper is for ASIC with native trit arithmetic — not this benchmark)
- GPU performance (we haven't benchmarked @sparseskip on GPU)

The 4.58× at 75% sparsity is the honest, reproducible x86 number. It is a floor — element-level @sparseskip within active expert weights is the next frontier and would multiply this further on both x86 and native hardware.

---

## Patent Reference

**A50296/2026** covers the `@sparseskip` annotation and `TSPARSE_MATMUL` opcode as a method for zero-overhead sparse execution in ternary neural network inference. The expert-level skip demonstrated here is one application. Element-level and layer-level skip are additional claims within the same filing.
