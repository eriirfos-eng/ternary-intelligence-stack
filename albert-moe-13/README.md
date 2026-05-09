# Albert MoE-13

**Ternary-native Mixture-of-Experts language model** — weights constrained to {-1, 0, +1}, trained from scratch on CPU with real-time dashboard telemetry.

Part of the [Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack) | RFI-IRFOS, Graz · Patent Pending A50296/2026

---

## Run the Benchmark (one line)

```bash
curl -fsSL https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/albert-moe-13/bench/install.sh | sh
```

Downloads the model, runs three suites (inference speed · @sparseskip analysis · perplexity), exports `albert_bench_results.csv`. Linux and macOS. No GPU required.

**First published result — HP ZBook 15 (i7-4800MQ, 2013, CPU only):** `84.4 tok/s · 11.8 ms/tok · 75% expert skip rate`

---

## What it is

Albert MoE-13 is a research model exploring whether ternary weight quantization can match or exceed float32 performance at a fraction of the inference cost. Ternary matmuls reduce to integer additions — no multiplies — making inference 2–5× cheaper per parameter on standard hardware and far cheaper on future ternary silicon.

The architecture combines:
- **Straight-Through Estimation (STE)** for end-to-end ternary training
- **Mixture-of-Experts (MoE)** routing with 12 domain experts and Top-3 selection
- **@sparseskip** — 9 of 12 experts are skipped per decode step at the routing level, compounding weight-level sparsity savings *(patent pending A50296/2026)*
- **Ternary Traffic Light Routing (TTL)** — per-expert trit execution budget (Green / Orange / Red) based on rolling EMA utilization, with anti-stagnation burst mechanism
- **Auto-evolutionary expansion** — the model grows its own depth via Net2Net surgery as it plateaus
- **Stage-aware corpus curriculum** — richer training data unlocks automatically as the model gains depth

---

## Current Architecture (v2.0.0)

| Parameter | Value |
|-----------|-------|
| Hidden size | 256 |
| Layers | **5** (grew from 4L via Net2Net surgery at Global Epoch 381) |
| Attention heads | 4 |
| Experts | 12 |
| Context length | 128 tokens |
| Vocabulary | 8,000 tokens (BPE) |
| Routing | Top-3 sparse — @sparseskip, 75% experts skipped per step |
| TTL routing | EMA-based trit states: Green (+logit boost) · Orange (scaled output) · Red (suppressed + skipped) |
| Quantization | STE with gamma-scaled ternary, gamma cached every 20 steps |
| LB loss | Switch Transformer load-balancing, λ = 0.03 |
| Optimizer | AdamW, cosine LR 2e-4 → 1e-5 / 500 steps |

**Training state (2026-05-09):** Global Epoch 385 · best loss 6.882 · 290 tensors loaded from checkpoint

---

## Auditor Quick-Reference — Ternary Authenticity

| Claim | File | What to look for |
|-------|------|-----------------|
| STE backward pass | `moe-llm-core/src/model/ste.rs` | `quantize_ternary()` — hard threshold forward, identity gradient backward |
| Training loop with STE | `moe-llm-core/src/bin/train_bible.rs` | `backward()` → `opt.step()` → gradient flows through `TernaryLinear` via STE |
| TernaryLinear (forward pass) | `moe-llm-core/src/model/ternary_linear.rs` | Gamma-scaled quantization, gamma cache every 20 steps, `inference_cache` for pre-ternarized weights |
| MoE routing + @sparseskip | `moe-llm-core/src/model/moe.rs` | Top-3 sparse gating, Red experts skipped entirely |
| TTL routing | `moe-llm-core/src/model/traffic_light.rs` | EMA utilization → trit states, anti-stagnation burst (burst_count × 7 mod 12 rotation) |
| Auto-evolutionary scaling | `moe-llm-core/src/bin/train_bible.rs` | `EvolutionManager` — plateau detection, Net2Net surgery, layer expansion |
| Benchmark suite | `moe-test/src/main.rs` | `run_bench_mode()` — speed + @sparseskip analysis + perplexity, CSV export |

> **Note for auditors:** The training binary is at `moe-llm-core/src/bin/train_bible.rs`. The `moe-test` crate provides the interactive TUI and `--bench` / `--eval` modes.

---

## Quick Start

### Prerequisites
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cd albert-moe-13
cargo build --release --bin train_bible
cargo build --release -p moe-test
```

### Train
```bash
albert-train
```
Opens the live dashboard at `http://localhost:8888`. The orchestrator script (`~/bin/albert-train`) manages the training binary and dashboard server together.

### Benchmark
```bash
# From source
./target/release/moe-test --bench --csv results.csv

# Or one-line installer (downloads binary + model):
curl -fsSL https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/albert-moe-13/bench/install.sh | sh
```

### Interactive TUI
```bash
albert-test
# or: ./target/release/moe-test
```

### Eval mode
```bash
./target/release/moe-test --eval data/corpus/stage_3/bible.txt
```

### Dashboard
The dashboard (`dashboard/index.html`) streams `dashboard/training.log` via HTTP byte-range requests. Panels include: loss curves, expert activity, per-layer weight sparsity, expert routing heatmap (last 60 steps), per-layer gradient norm, and the TTL routing panel (Green/Orange/Red trit states per expert per layer).

---

## Ternary Traffic Light Routing (TTL)

Each expert is assigned a trit execution state every forward pass based on rolling EMA utilization:

| State | Trit | Gate logit modifier | Output scale | Effect |
|-------|------|--------------------|--------------|----|
| Green | +1 | +0.4 | 1.0 | Underloaded — boosted selection probability |
| Orange | 0 | 0.0 | 0.4 | On-target — normal routing, partial output |
| Red | -1 | −0.4 | — | Overloaded — suppressed, @sparseskip'd |

**Anti-stagnation burst:** if all experts stay Orange for 100+ consecutive steps (Nash equilibrium, gate gradients vanish), the system forcibly assigns 3 Green + 3 Red experts for 30 steps to break symmetry. Rotation uses `burst_count × 7 mod 12` (7 is coprime to 12) for full expert coverage across cycles.

---

## Repository Layout

```
albert-moe-13/
├── moe-llm-core/           # Core model and training binary
│   └── src/
│       ├── bin/
│       │   └── train_bible.rs      # Training loop, EvolutionManager, TTL, LR schedule
│       ├── model/
│       │   ├── transformer.rs      # Transformer + Block construction
│       │   ├── attention.rs        # Multi-head attention (causal mask cached)
│       │   ├── moe.rs              # MoE block, top-3 routing, @sparseskip
│       │   ├── traffic_light.rs    # TTL routing — EMA, trit states, burst
│       │   ├── ternary_linear.rs   # TernaryLinear with gamma cache
│       │   ├── ste.rs              # Straight-Through Estimator
│       │   ├── mlp.rs              # Expert MLP
│       │   └── config.rs           # TransformerConfig
│       └── tokenizer/              # BPE tokenizer
├── moe-test/
│   └── src/main.rs                 # Interactive TUI + --bench + --eval modes
├── bench/
│   └── install.sh                  # One-line benchmark installer (Linux + macOS)
├── dashboard/
│   ├── index.html                  # Live training dashboard
│   └── run_server.py               # HTTP server with Range request support
├── data/
│   ├── corpus/                     # Active training corpus (.txt files)
│   └── vocab.json                  # BPE vocabulary (8,000 tokens)
├── models/
│   ├── bible_ternary_v2.0.0.safetensors   # Current checkpoint (~142 MB)
│   ├── bible_ternary_v2.0.0.config.json   # Architecture config
│   ├── bible_ternary_v2.0.0.meta          # Global epoch counter
│   └── README.md                          # Checkpoint registry
├── crates/
│   └── moe-platform/               # Future inference runtime API
└── docs/
    ├── architecture.md
    ├── ternary-compression.md
    └── roadmap.md
```

---

## Stage-Aware Corpus Curriculum

Albert automatically unlocks richer training data as it grows deeper via Net2Net surgery:

| Stage dir | Unlocked at | Content | Purpose |
|-----------|-------------|---------|---------|
| `data/corpus/stage_3/` | 3L (initial) | Bible KJV + Alice in Wonderland | Grammar, vocab, basic syntax |
| `data/corpus/stage_6/` | 6L | Gutenberg novels | Complex narrative, wider vocabulary |
| `data/corpus/stage_7/` | 7L | Simple Wikipedia | Factual, diverse topics |
| `data/corpus/stage_9/` | 9L | `qa_instruction.txt` | `User:/Albert:` instruction format |
| `data/corpus/stage_11/` | 11L | Linux docs, EU AI Act | Technical and specialized language |

**Current state (5L):** stage_3 active. Surgery to 6L will unlock stage_6 and the next curriculum level automatically.

---

## Training Speed

| Configuration | Time per batch |
|---------------|---------------|
| 256H · 4L | ~4.5 s |
| 256H · 5L (current) | ~5.5 s |

Training runs on CPU (HP ZBook 15, i7-4800MQ). The ~4× batch time increase from 3L→5L is linear with layer count, consistent with ternary matmul scaling.

---

## Competitive Positioning

### vs. Microsoft BitNet (1-bit LLMs)

| Dimension | Albert MoE-13 | BitNet b1.58 |
|-----------|--------------|--------------|
| Weight precision | Ternary `{−γ, 0, +γ}` | Ternary `{−1, 0, +1}` |
| Training approach | **Native ternary from init via STE** | Post-training quantization |
| Architecture | **MoE, 12 experts, Top-3, @sparseskip** | Dense transformer |
| Zero state | **First-class routing instruction (@sparseskip)** | Passive compression artifact |
| Routing budget control | **TTL — per-expert trit execution budget** | None |
| Architecture growth | **Autonomous Net2Net surgery** | Fixed at initialization |
| Expert-level skip | **75% experts skipped per decode step (measured)** | No expert-level skip |
| Inference (CPU) | **84.4 tok/s on i7-4800MQ (2013), no GPU** | GPU-optimized |

**Key distinction:** BitNet treats the zero state as a compression artifact. Albert treats it as a first-class computational primitive — the HOLD state is a routing instruction that explicitly skips computation (`@sparseskip`). The MoE architecture means 9 of 12 experts are skipped per decode step at the routing level, compounding the weight-level sparsity savings.

### vs. Standard MoE (Mixtral, Switch Transformer)

Standard MoE uses float32 weights and routes for capacity, not sparsity. Albert achieves two simultaneous savings: weight-level ternary sparsity inside each expert, plus expert-level skip at the routing layer via `@sparseskip`. The TTL adds a third dimension: real-time per-expert execution budget control based on utilization telemetry.

---

<img width="1872" height="931" alt="image" src="https://github.com/user-attachments/assets/ea0198ed-c29d-4872-8fa5-6cf7b2a1513a" />


## Further Reading

- [Architecture](docs/architecture.md) — model internals, routing, STE
- [Checkpoint Registry](models/README.md) — artifact versions and provenance
- [Ternary Compression](docs/ternary-compression.md) — theory and sparsity benchmarks
- [Roadmap](docs/roadmap.md) — what's next
