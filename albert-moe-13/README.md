# Albert MoE-13

**Ternary-native Mixture-of-Experts language model** — weights constrained to {-1, 0, +1}, trained from scratch on CPU with real-time dashboard telemetry.

Part of the [Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack) | RFI-IRFOS, Graz · Patent Pending A50296/2026

---

## What it is

Albert MoE-13 is a research model exploring whether ternary weight quantization can match or exceed float32 performance at a fraction of the inference cost. Ternary matmuls reduce to integer additions — no multiplies — making inference 2–5× cheaper per parameter on standard hardware and far cheaper on future ternary silicon.

The architecture combines:
- **Straight-Through Estimation (STE)** for end-to-end ternary training
- **Mixture-of-Experts (MoE)** routing with 12 domain experts and top-3 selection
- **Auto-evolutionary expansion** — the model grows its own depth layer by layer as it plateaus
- **Per-layer sparsity gradient** — early layers stay dense, deep layers become sparse
- **L1 sparsity reward** in the loss function so the model learns strategically where to be zero

---

## Auditor Quick-Reference — Ternary Authenticity

For independent verification of the native ternary training claim:

| Claim | File | What to look for |
|-------|------|-----------------|
| STE backward pass | `moe-llm-core/src/model/ste.rs` | `quantize_ternary()` — hard threshold forward, identity gradient backward |
| Training loop with STE | `moe-llm-core/src/bin/train_bible.rs` | `backward()` → `opt.step()` → gradient flows through `TernaryLinear` via STE |
| TernaryLinear (forward pass) | `moe-llm-core/src/model/ternary_linear.rs` | Gamma-scaled quantization, gamma cache every 20 steps, `inference_cache` for pre-ternarized weights |
| MoE routing | `moe-llm-core/src/model/moe.rs` | Top-3 sparse gating, routing telemetry, F32 gate (not ternary — intentional: 256→12 resolution) |
| Auto-evolutionary scaling | `moe-llm-core/src/bin/train_bible.rs` | `EvolutionManager` — plateau detection, Net2Net surgery, layer expansion |
| Empirical loss convergence | `docs/convergence_log.md` | 25-epoch loss curve showing 7.87 → 6.95 descent from random init |

> **Note for auditors:** The training binary is at `moe-llm-core/src/bin/train_bible.rs`, not `src/training.rs`. The `src/` path does not exist — the workspace uses the standard Cargo `src/bin/` layout under `moe-llm-core/`.

---

## Current Architecture (v2.3)

| Parameter | Value |
|-----------|-------|
| Hidden size | 256 |
| Layers | 3 (grows to max 12 via evolution) |
| Attention heads | 4 |
| Experts | 12 |
| Context length | 128 tokens |
| Routing | Top-3 sparse with asymmetric safety gating |
| Threshold | Per-layer: 0.01 (layer 0) → 0.03+ (deepest) |
| Quantization | STE with gamma-scaled ternary, gamma cached every 20 steps |
| Optimizer | AdamW with cosine LR (2e-4 → 1e-5 / 500 steps) |

---

## Quick Start

### Prerequisites
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the training binary
cd albert-moe-13
cargo build --release --bin train_bible
```

### Train
```bash
albert-train
```
Opens the live dashboard at `http://localhost:8888` and starts training. The orchestrator script (`~/bin/albert-train`) manages the training binary and dashboard server together.

### Dashboard
The dashboard (`dashboard/index.html`) streams `dashboard/training.log` via HTTP byte-range requests. Each batch logs one line:
```
Epoch 3 (Global 42), Batch 127: loss = 6.4283
```
An epoch summary line is written at the end of each 300-batch epoch:
```
=== Epoch 3L done | Avg Loss: 6.4283 | 04:58 elapsed ===
```
An ARCH line is written once per train cycle so the dashboard can display live architecture state:
```
ARCH 3L 256H 12E 128CTX 8192V
```

### Add training data
Drop any `.txt` file into `data/corpus/` and restart training — it's picked up automatically. To download a curated set of corpora (Wikipedia, Gutenberg, EU AI Act, Linux docs):
```bash
python3 scripts/download_corpus.py
# Files land in data/corpus_staged/ — move to data/corpus/ when ready
mv data/corpus_staged/gutenberg_war_and_peace.txt data/corpus/
```

---

## Repository Layout

```
albert-moe-13/
├── moe-llm-core/           # Core model and training binary
│   └── src/
│       ├── bin/
│       │   └── train_bible.rs      # Training loop, EvolutionManager, LR schedule
│       ├── model/
│       │   ├── transformer.rs      # Transformer + Block construction
│       │   ├── attention.rs        # Multi-head attention (causal mask cached)
│       │   ├── moe.rs              # MoE block, top-3 routing, gating noise
│       │   ├── ternary_linear.rs   # TernaryLinear with gamma cache
│       │   ├── ste.rs              # Straight-Through Estimator
│       │   ├── mlp.rs              # Expert MLP
│       │   └── config.rs           # TransformerConfig + layer_threshold()
│       └── tokenizer/              # BPE tokenizer
├── dashboard/
│   ├── index.html                  # Live training dashboard (Chart.js)
│   └── run_server.py               # HTTP server with Range request support
├── data/
│   ├── corpus/                     # Active training corpus (.txt files)
│   ├── corpus_staged/              # Downloaded but not yet active
│   └── vocab.json                  # BPE vocabulary
├── models/
│   ├── bible_ternary_v2.0.0.safetensors   # Current checkpoint
│   ├── bible_ternary_v2.0.0.config.json   # Architecture config
│   ├── bible_ternary_v2.0.0.meta          # Global epoch counter
│   └── README.md                          # Checkpoint registry
├── scripts/
│   └── download_corpus.py          # Corpus downloader (Wikipedia, Gutenberg, etc.)
├── crates/
│   └── moe-platform/               # Future inference runtime API
└── docs/
    ├── architecture.md             # Architecture deep-dive
    ├── ternary-compression.md      # Ternary theory and benchmarks
    └── roadmap.md                  # Development roadmap
```

---

## Training Speed

| Configuration | Time per batch |
|---------------|---------------|
| 128H, buggy 16× accumulation | ~50s |
| 128H, fixed | ~300ms |
| 256H, fixed | ~2s |

The ~50× speedup over the original implementation came from fixing a gradient accumulation bug where `opt.backward_step()` was called 16 times per batch instead of once.

---

## Held-Out Perplexity (v2.0.0 best checkpoint)

Evaluated on a deterministic 5% held-out split of the training corpus (seed 42 — never seen during training):

| Metric | Value |
|--------|-------|
| Avg loss (held-out) | **7.1537** |
| Perplexity | **1,278.8** |
| Unigram random baseline | 8,000.0 |
| Reduction vs baseline | **84.0%** |

Reproduce: `python3 scripts/eval_perplexity.py --checkpoint models/bible_ternary_v2.0.0.best.safetensors`

---

## Stage-Aware Corpus Curriculum

Albert automatically unlocks richer training data as it grows deeper — each Net2Net surgery increments `num_layers`, which triggers the corpus loader to include the next stage directory:

| Stage dir | Unlocked at | Content | Purpose |
|-----------|-------------|---------|---------|
| `data/corpus/stage_3/` | 3L (initial) | Bible + Alice in Wonderland | Grammar, vocab, basic syntax |
| `data/corpus/stage_6/` | 6L | Gutenberg novels | Complex narrative, wider vocabulary |
| `data/corpus/stage_7/` | 7L | Simple Wikipedia | Factual, diverse topics |
| `data/corpus/stage_9/` | 9L | `qa_instruction.txt` | `User:/Albert:` instruction format |
| `data/corpus/stage_11/` | 11L | Linux docs, EU AI Act | Technical and specialized language |

This is automatic curriculum learning: the model cannot overfit on complex text before it has the architectural capacity to represent it. Surgery fires → depth increases → corpus expands → new challenge. No manual intervention required.

Implementation: `train_bible.rs` `load_corpus()` function, lines 581–630.

---

## Competitive Positioning

### vs. Microsoft BitNet (1-bit LLMs)

| Dimension | Albert MoE-13 | BitNet b1.58 |
|-----------|--------------|--------------|
| Weight precision | Ternary `{−γ, 0, +γ}` | Ternary `{−1, 0, +1}` |
| Training approach | **Native ternary from init via STE** | Post-training quantization of float model |
| Architecture | **Mixture-of-Experts, 12 experts, Top-3** | Dense transformer |
| Zero-state | **First-class routing instruction (@sparseskip)** | Passive compression artifact |
| Architecture growth | **Autonomous Net2Net surgery** | Fixed at initialization |
| Inference skip | **3.97× MLP speedup at 75% sparsity (measured)** | No expert-level skip mechanism |
| Hardware target | CPU-first, ASIC roadmap | GPU-optimized |
| Scale | Research prototype, 3L→12L growth pipeline | Production, 100B+ params |

**Key distinction:** BitNet treats the zero state as a compression artifact. Albert treats it as a first-class computational primitive — the `HOLD` state is a routing instruction that explicitly skips computation (`@sparseskip`). The MoE architecture means 9 of 12 experts are skipped per decode step at the routing level, compounding the weight-level sparsity savings.

**Is the frontier claim novel?** The combination of (1) native ternary training, (2) MoE expert routing, (3) autonomous architectural growth, and (4) zero-as-routing-primitive is architecturally distinct from BitNet. This is not incremental improvement on BitNet — it is a different architectural hypothesis about where the zero state belongs in the compute graph.

### vs. Standard MoE (Mixtral, Switch Transformer)

Standard MoE implementations use float32 weights and route to experts for capacity, not for sparsity. Albert's routing achieves two simultaneous savings: weight-level ternary sparsity inside each expert, plus expert-level skip at the routing layer. The `@sparseskip` primitive makes the zero routing weight a branch condition, not a multiply-by-zero.

---

## Further Reading

- [Architecture](docs/architecture.md) — model internals, routing, STE
- [Checkpoint Registry](models/README.md) — artifact versions and provenance
- [Ternary Compression](docs/ternary-compression.md) — theory and sparsity benchmarks
- [Roadmap](docs/roadmap.md) — what's next
