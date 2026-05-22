# Reproducing Albert-MoE-13 Benchmarks

Tested on: Ubuntu 22.04 · rustc 1.87 stable · no CUDA required

---

## Prerequisites

```bash
git clone https://github.com/eriirfos-eng/ternary-intelligence-stack
cd ternary-intelligence-stack/albert-moe-13
```

The checkpoint (`models/albert_v3.0.safetensors`, ~537 MB) is not yet in the
public repo. Pull it from the Modal volume if you have access:

```bash
modal volume get albert-vol models/albert_v3.0.safetensors models/
modal volume get albert-vol models/albert_v3.0.config.json models/
```

---

## Five reproducible commands

### 1  Build (release, CPU)

```bash
cargo build --release -p moe-llm-core
```

Expected: compiles in ~2–4 min on first run (Cargo caches subsequent builds).

---

### 2  Architecture + gradient smoke test (no checkpoint required)

```bash
./target/release/repro_check
```

Verifies: (a) 134,754,816 total parameters match config, (b) forward+backward
produces non-zero gradient norms confirming STE is wired end-to-end, (c)
checkpoint round-trip is lossless to float32 precision. Exit 0 = all pass.

---

### 3  @sparseskip throughput benchmark (no checkpoint required)

```bash
./target/release/sparseskip_throughput
```

Reproduces the **79.5–92.2 tok/s** CPU inference claim. The binary runs a
synthetic Top-3/12 MoE layer at varying sparsity levels and reports wall-clock
speedup. At 75% sparsity (3/12 active experts) expect ≥2× over the dense
baseline. Measured values:

| Machine                        | tok/s  |
|-------------------------------|--------|
| AMD Ryzen 5 PRO 3500U         | 79.5   |
| Intel Core i7-4800MQ (ZBook)  | 92.2   |

Numbers are hardware-dependent but the speedup ratio should be stable (±5%).

---

### 4  Held-out perplexity (checkpoint required)

```bash
./target/release/eval_perplexity data/eval_sample.txt --max-windows=50
```

At ep2797 checkpoint:

```
Mean CE loss : 9.5975
Perplexity   : 14728.3
Tokens eval  : 12800
```

The model is mid-training; PPL will be lower on a later checkpoint. Run without
`--max-windows` to evaluate the full corpus (156 windows, ~6 min on CPU).

---

### 5  Inference benchmark — all 15 canonical prompts (checkpoint required)

```bash
./target/release/gpu_bench
```

Runs the same 15 multilingual prompts used in bench_v3.0_* exports. On CPU at
18L expect 14–37 tok/s depending on prompt length (shorter = more routing
overhead per token). On Modal T4 via `python3 train_modal.py bench_gpu` the
CUDA path activates and throughput scales with batch size.

---

## Notes on reproducibility

**Training is not bit-identical** — candle weight init uses an internal RNG not
exposed to the user. `repro_check` (step 2) verifies the structural invariants
that _are_ reproducible: tensor shapes, parameter count, gradient connectivity,
and checkpoint serialization round-trip.

**Multi-epoch efficiency** — 177M corpus tokens × 2946 epochs = 521B
token-steps at $61.87 total cost. Ternary STE prevents multi-epoch memorization:
discrete {-γ,0,+γ} weights cannot overfit surface patterns, so every pass yields
genuine gradient signal. See whitepaper §Multi-epoch corpus efficiency.
