# Albert MoE-13

**Ternary-native Mixture-of-Experts language model** — weights constrained to {-1, 0, +1}, trained from scratch with real-time dashboard telemetry. GPU training via Modal (T4).

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

Albert MoE-13 is a research organism — a working instance of a philosophy of computing that, until now, only existed as scattered intuitions across sixty years of papers nobody connected. You do not engineer it. You cultivate it: set the substrate, the growth rules, the curriculum ladder, the plateau gates — and observe what develops within those conditions rather than dictating it. The expert specializations, the hot-layer hierarchy, the decision not to grow when growth would interrupt productive learning — these emerge from the system, not from the operator.

The technical substrate: ternary weights `{-γ, 0, +γ}` throughout training reduce matmuls to integer additions — no multiplies — making inference 2–5× cheaper per parameter on standard hardware and far cheaper on future ternary silicon.

The architecture combines:
- **Straight-Through Estimation (STE)** for end-to-end ternary training
- **Mixture-of-Experts (MoE)** routing with 12 domain experts and Top-3 selection
- **@sparseskip** — 9 of 12 experts are skipped per decode step at the routing level, compounding weight-level sparsity savings *(patent pending A50296/2026)*
- **Expert seed biases** — learnable F32 [256] bias per expert (Uniform[-0.01, 0.01] init) breaks routing Nash equilibria; weights persist and amplify expert specialisation over training
- **Ternary Traffic Light Routing (TTL)** — per-expert trit execution budget (Green / Orange / Red) based on rolling EMA utilization, with anti-stagnation burst mechanism
- **WALD module** — Wald-inspired loss-space coverage analysis; detects dead zones in the loss histogram and drives early-layer gradient amplification; self-disables when the dead zone is structural
- **Auto-evolutionary expansion** — the model grows its own depth via Net2Net surgery as it plateaus
- **Stage-aware corpus curriculum** — richer training data unlocks automatically as the model gains depth

---

## Current Architecture (v3.0)

| Parameter | Value |
|-----------|-------|
| Hidden size | 256 |
| Layers | **17** (v2.0.0: 4L→12L over 10 surgeries; v3.0: ep511 12L→13L, ep547 13L→14L, ep611 14L→15L, ep645 15L→16L, ep701 16L→17L) |
| Attention heads | 4 |
| Experts | 12 |
| Context length | 256 tokens |
| Vocabulary | 32,000 tokens (ByteLevel BPE — EN/DE/FR/ES/PT/IT/NL/PL) |
| Routing | Top-3 sparse — @sparseskip, 75% experts skipped per step |
| TTL routing | EMA-based trit states: Green (+logit boost) · Orange (scaled output) · Red (suppressed + skipped) |
| Quantization | STE with gamma-scaled ternary, gamma cached every 20 steps |
| LB loss | Switch Transformer load-balancing, λ = 0.03 |
| Optimizer | AdamW, cosine LR 3e-4 → 1e-5 / 500 steps |

**Training state (2026-05-14):** Global Epoch 791+ · best loss **10.2199** (ep786) · 5 Net2Net surgeries complete (12L→17L) · descent rate ~0.015 nats/10 epochs · training on Modal T4 GPU (~400ms/batch)

---

## Auditor Quick-Reference — Ternary Authenticity

| Claim | File | What to look for |
|-------|------|-----------------|
| STE backward pass | `moe-llm-core/src/model/ste.rs` | `quantize_ternary()` — hard threshold forward, identity gradient backward |
| Training loop with STE | `moe-llm-core/src/bin/train_bible.rs` | `backward()` → `opt.step()` → gradient flows through `TernaryLinear` via STE |
| TernaryLinear (forward pass) | `moe-llm-core/src/model/ternary_linear.rs` | Gamma-scaled quantization, gamma cache every 20 steps, `inference_cache` for pre-ternarized weights |
| MoE routing + @sparseskip | `moe-llm-core/src/model/moe.rs` | Top-3 sparse gating, Red experts skipped entirely |
| TTL routing | `moe-llm-core/src/model/traffic_light.rs` | EMA utilization → trit states, anti-stagnation burst (burst_count × 7 mod 12 rotation) |
| Expert seed biases | `moe-llm-core/src/model/moe.rs` | `expert_seeds: Vec<Tensor>` per MoeBlock — unique F32 bias per expert, vb-tracked |
| WALD loss-space analysis | `moe-llm-core/src/wald.rs` | `WaldModule` — batch histogram, dead zone detection, severity → amplification scale; staleness detection disables amplification on plateau |
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
> Fires `modal run albert-moe-13/train_modal.py` — builds `train_bible` with CUDA on a Modal T4 GPU, streams the training log back to the local dashboard at `http://localhost:8888`. One-time setup: `python3 train_modal.py setup` uploads corpus and checkpoint to the Modal volume. Pull checkpoint back with `albert-train pull`.

### Train (local CPU fallback)
```bash
cargo run --release --bin train_bible -- --root=.
```

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
cargo run --release -p moe-llm-core --bin eval_perplexity
# or against a specific file:
cargo run --release -p moe-llm-core --bin eval_perplexity data/corpus/stage_3/alice.txt
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
│       │   ├── train_bible.rs      # Training loop, EvolutionManager, TTL, LR schedule
│       │   └── eval_perplexity.rs  # Held-out perplexity evaluation
│       ├── model/
│       │   ├── transformer.rs      # Transformer + Block construction
│       │   ├── attention.rs        # Multi-head attention (causal mask cached)
│       │   ├── moe.rs              # MoE block, top-3 routing, @sparseskip, expert seed biases
│       │   ├── traffic_light.rs    # TTL routing — EMA, trit states, burst
│       │   ├── ternary_linear.rs   # TernaryLinear with gamma cache
│       │   ├── ste.rs              # Straight-Through Estimator
│       │   ├── mlp.rs              # Expert MLP
│       │   └── config.rs           # TransformerConfig
│       ├── wald.rs                 # WALD loss-space coverage module
│       └── tokenizer/              # BPE tokenizer
├── moe-test/
│   └── src/main.rs                 # Interactive TUI + --bench + --eval modes
├── bench/
│   └── install.sh                  # One-line benchmark installer (Linux + macOS)
├── train_modal.py                  # Modal GPU training app (setup / run / pull)
├── albert-train                    # Local launcher — fires modal run + dashboard
├── albert-test                     # Local launcher — opens moe-test TUI
├── dashboard/
│   ├── index.html                  # Live training dashboard
│   └── run_server.py               # HTTP server with Range request support
├── data/
│   ├── corpus/                     # Active training corpus (.txt files)
│   ├── corpus/                     # Stage-aware corpus dirs (stage_3/ through stage_11/)
│   ├── multilingual/               # v3.0 — Wikipedia + Europarl multilingual (~446 MB)
│   ├── academic/                   # v3.0 — academic texts (~46 MB)
│   ├── fulltext/                   # v3.0 — Gutenberg fulltext (~68 MB)
│   ├── chaos/                      # v3.0 — 10% chaos layer (~43 MB, invariant enforced)
│   └── vocab_v3.json               # ByteLevel BPE vocabulary (32,000 tokens)
├── models/
│   ├── albert_v3.0.safetensors     # Active checkpoint (v3.0, 12L)
│   ├── albert_v3.0.config.json     # Architecture config
│   ├── albert_v3.0.meta            # Global epoch counter
│   └── README.md                   # Checkpoint registry
├── crates/
│   └── moe-platform/               # Inference runtime API
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

**v3.0 corpus (all stages active):**

| Corpus dir | Size | Content |
|------------|------|---------|
| `data/multilingual/` | ~446 MB | Wikipedia CC BY-SA + Europarl (EN/DE/FR/ES/PT/IT/NL/PL) |
| `data/academic/` | ~46 MB | Academic texts |
| `data/fulltext/` | ~68 MB | Gutenberg multilingual novels |
| `data/chaos/` | ~43 MB | 10% chaos layer — invariant enforced |

**Total v3.0 corpus:** ~635 MB raw. Tokenized per-file to stay within ~1.2 GB peak RAM (see [OOM fix notes](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/docs/session_log.md)).

---

## Training Speed

| Configuration | Hardware | Time per batch |
|---------------|----------|---------------|
| 256H · 4L | CPU (i7-4800MQ) | ~4.5 s |
| 256H · 5L | CPU (i7-4800MQ) | ~5.5 s |
| 256H · 12L | CPU (i7-4800MQ) | ~13 s |
| 256H · 12L (current) | Modal T4 GPU | ~400 ms |

T4 GPU training via Modal gives ~32× speedup over CPU for the 12L architecture. `albert-train` handles the full launch: image build with CUDA, volume-cached crate downloads, live log streaming to local dashboard.

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

<img width="1863" height="930" alt="image" src="https://github.com/user-attachments/assets/d2d60f92-b857-403d-adc5-3a7b0b2fa589" />

---

## Further Reading

- [Architecture](docs/architecture.md) — model internals, routing, STE
- [Checkpoint Registry](models/README.md) — artifact versions and provenance
- [Ternary Compression](docs/ternary-compression.md) — theory and sparsity benchmarks
- [Roadmap](docs/roadmap.md) — what's next
- [Session Log](../ternlang-root/docs/session_log.md) — production fixes and training milestones
- [Benchmarks](../ternlang-root/BENCHMARKS.md) — full sparsity and perplexity data
- [Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack) — full ecosystem
