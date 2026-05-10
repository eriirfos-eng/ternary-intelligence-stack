# Albert MoE-13 — Architecture

## Canonical Source File Map

This section exists to resolve audit path confusion. The public repository uses a flat
workspace layout — not the `crates/moe-core/src/` hierarchy that automated tools may
expect from conventional monorepos.

| Conceptual Component | Actual Path in Repository |
|---|---|
| STE backward-pass math | `moe-llm-core/src/ste.rs` |
| Ternary linear layer | `moe-llm-core/src/model/ternary_linear.rs` |
| MoE routing & SparseSkip | `moe-llm-core/src/model/moe.rs` |
| Transformer / attention | `moe-llm-core/src/model/transformer.rs` |
| MLP expert | `moe-llm-core/src/model/mlp.rs` |
| Training loop (main) | `moe-llm-core/src/bin/train_bible.rs` |
| EvolutionManager | `moe-llm-core/src/model/transformer.rs` (inline) |
| Dataset / corpus loader | `moe-llm-core/src/lib.rs` → `load_corpus()` |
| BPE tokenizer | `moe-llm-core/src/tokenizer.rs` |
| SparseskipThroughput bench | `moe-llm-core/src/bin/sparseskip_throughput.rs` |
| Workspace manifest | `albert-moe-13/Cargo.toml` |
| Dashboard front-end | `dashboard/index.html` |
| Dashboard HTTP server | `dashboard/run_server.py` |
| Training launch script | `~/bin/albert-train` (system-wide) |
| Model checkpoints | `models/bible_ternary_v*.safetensors` |
| Inference / test client | `moe-test/src/main.rs` |

> **Note for auditors:** The directory `crates/` in this workspace contains the legacy
> MoE orchestration crates (moe-core, moe-runtime, moe-platform, etc.) which govern
> the MCP control plane. The *neural training stack* is in `moe-llm-core/`. Both are
> members of the same Cargo workspace (`albert-moe-13/Cargo.toml`).

---

## Overview

Albert MoE-13 is a ternary-native transformer with Mixture-of-Experts feed-forward layers. Every weight matrix is quantized to {-1, 0, +1} during both forward and backward passes via Straight-Through Estimation. The architecture grows its own depth autonomously through the `EvolutionManager`, reaching maximum depth of 12 layers at Global Epoch 454.

**Current state (2026-05-10):** 256H · 12L · 4H · 12E · 128CTX · 8000V · Global Epoch 477+ · best loss 6.8821

---

## 1. Ternary Quantization (STE)

Each `TernaryLinear` layer holds float32 weights in the optimizer but applies ternary quantization on every forward pass:

```
w_ternary = sign(w) * I(|w| > τ)   [τ = per-layer threshold]
y = w_ternary * γ                   [γ = mean(|w|), cached every 20 steps]
```

The Straight-Through Estimator passes the gradient through the quantization step unchanged:

```
∂L/∂w ≈ ∂L/∂y   (gradient bypasses the non-differentiable threshold)
```

**Per-layer thresholds** — early layers stay dense, late layers grow sparse:

| Layer | Threshold τ | Effect |
|-------|------------|--------|
| 0 | 0.01 | Dense — learns syntax and token co-occurrence |
| 1 | 0.02 | Medium — learns phrase-level patterns |
| 2+ | 0.03+ | Sparse — abstract representations, most weights zeroed |

**TELE sparsity telemetry (confirmed at 12L):** L0 at 10.6% ternary weight sparsity, rising monotonically to L11 at 26.5%. Deeper layers accumulate more zero-weight trits, reinforcing `@sparseskip` efficiency at the layers where it matters most.

**Gamma cache** — `mean(|w|)` is expensive on large weight matrices. It is recomputed only every 20 forward calls; weights change slowly enough that the cached value is mathematically equivalent.

**L1 sparsity regularization** — the training loss includes a small L1 penalty on all weight matrices (`λ = 1e-5`). This gives the optimizer a gradient signal to push marginal weights toward zero, so ternary zeroing is strategic rather than random.

---

## 2. Transformer Block

Each block follows a standard pre-norm transformer layout:

```
x → LayerNorm → MultiHeadAttention → residual
  → LayerNorm → MoEBlock → residual
```

### Multi-Head Attention

- 4 heads, head dimension = hidden_size / 4
- Q, K, V, O projections are all `TernaryLinear`
- Causal mask is cached in a `RefCell<Option<(usize, Tensor)>>` — rebuilt only when `seq_len` changes, eliminating a 128×128 allocation per attention call per layer

### MoE Block

12 experts, each a two-layer MLP (`hidden_size → hidden_size × 4 → hidden_size`), all ternary.

**Routing:**
1. Gate network (`TernaryLinear`: `hidden_size → 12`) produces logits
2. Uniform noise (0.98–1.02×) is added for exploration
3. Top-3 experts are selected via sequential argmax-and-mask
4. **Asymmetric safety gating** — Experts 0–3 are suppressed to zero when their gate value is below 0.05 (low-confidence guard)
5. Selected expert outputs are weighted by softmax over the top-3 gate values and summed
6. **`@sparseskip`** (Patent A50296/2026) — 9/12 experts not executed per decode step at Top-3 routing; 4.58× throughput multiplier at 75% sparsity

---

## 3. Ternary Traffic Light Routing (TTL)

TTL assigns each expert in each layer a trit state: **Green** (actively routing), **Orange** (warming up), or **Red** (stagnant). States are maintained as EMA-smoothed activation counts:

```
ema_i = α * ema_i + (1 − α) * routed_i    [α = 0.95, per-step update]
```

States transition based on thresholds:
- `G` — ema > high_threshold: expert is active and routing normally
- `O` — ema in [low_threshold, high_threshold]: expert is in transition
- `R` — ema < low_threshold: expert is receiving insufficient routing signal

**Anti-stagnation burst:** When all experts in a layer remain Orange for ≥100 consecutive steps, TTL forces a routing pattern of 3G + 3R for 30 steps, rotating via `burst_count × 7 mod 12`. The offset 7 is coprime to 12, ensuring every expert is visited across burst cycles before the pattern repeats.

**Cycling reds (observed at 12L):** R-state experts migrate autonomously through cold layers (L2→L3→L5) over 2–3 epochs and self-resolve without intervention. Three such episodes observed during the 12L run; all resolved with dead=0.

**TLIGHT log format:** `TLIGHT ep=N layer=L [G:a,b,c O:d,e,f R:g,h,i]`

---

## 4. Mycelium Expert Health Monitor

Mycelium maintains a 20-epoch rolling gradient pressure window per expert per layer, producing a pressure history `P[l,i]`:

```
P[l,i] = mean(grad_norm[l,i] over last 20 epochs)
```

Experts whose pressure falls below threshold for ≥8 consecutive epochs are classified dead. `generate_resurrections()` copies the most-active neighbour's weights into the dead expert with a small Gaussian perturbation (σ = 0.02), restoring gradient signal without disrupting surrounding routing structure.

**MYCELIUM log format:** `MYCELIUM epoch=N dead=D blooming=B hot=LH cold=LC pressure=[p0,...,p11]`

**Layer crystallization (confirmed at 12L):** Pressure gradient is structurally consistent:
- L0–L3: `~0.00022` (nearly frozen — early layers have locked in stable feature representations)
- L4–L7: `~0.002–0.006` (moderate activity)
- L8–L11: `0.013–0.022` (hot — deepest layers continue active learning)

This internal differentiation — both in gradient flow and weight sparsity — emerged from training alone without architectural intervention.

**Result over full 12L run:** `dead=0` in every epoch summary across 13+ monitored overnight epochs. All 12 experts remain alive and routing.

---

## 5. Auto-Evolutionary Training (EvolutionManager)

The `EvolutionManager` monitors epoch-average loss and triggers **Net2Net safe-copy surgery** when the model plateaus:

```
history_len        = 10 epochs
plateau_threshold  = 0.02   (< 2% improvement across 10 epochs → expand)
mastery_threshold  = 4.5    (loss < 4.5 → model has mastered current capacity)
divergence_threshold = 0.1  (loss rising → needs more capacity)
max_layers         = 12     (hard cap — enforced on both plateau and COLLAPSE→SURGERY paths)
```

**Surgery procedure:**
1. `config.json` `num_layers` is incremented
2. The new layer's weights are initialized as a safe copy of the deepest existing layer (Net2Net — preserves learned representations)
3. Training resumes with the expanded architecture

**Corpus unlocking:** The EvolutionManager queries the current layer count at each surgery event and enables newly unlocked corpus stages automatically. Stage 6 (Gutenberg) unlocks at 6L; Stage 7 (Simple Wikipedia) at 7L. This couples architectural depth to corpus breadth.

Albert grew from 3L to 12L across ten autonomous surgery events between Global Epoch ~200 and ~454.

---

## 6. Training Loop

```
for each global epoch (300 batches):
    for each batch:
        sample random 128-token window from corpus
        forward pass through transformer
        ce_loss = cross_entropy(logits, targets)
        lb_loss = load_balancing_aux_loss(routing_weights)  [λ = 0.03]
        l1_penalty = λ * mean(|weights|) for all weight matrices  [λ = 1e-5]
        total_loss = ce_loss + lb_loss + l1_penalty
        opt.backward_step(total_loss)
        log to dashboard/training.log
    write epoch summary to log
    save checkpoint
    check EvolutionManager → surgery if triggered
    update TTL states per layer
    update Mycelium pressure window
```

**Learning rate** — cosine schedule over 500 global epochs:
```
lr(t) = lr_min + 0.5 * (lr_max - lr_min) * (1 + cos(π * t / T))
lr_max = 3e-4,  lr_min = 1e-5,  T = 500 global epochs
```
At Global Epoch 477, the LR is effectively at floor (cosine near zero). Training completes at epoch 500.

---

## 7. Corpus Pipeline

Stage-aware curriculum: `load_corpus()` reads `.txt` files from the active stage directories. The EvolutionManager enables stages as layer count grows.

| Stage | Unlock | Corpus | Size |
|-------|--------|--------|------|
| 3 | 3L | King James Bible + Alice in Wonderland | ~4.3 MB |
| 6 | 6L | + 12 Gutenberg classics (Dickens, Austen, Tolstoy, et al.) | ~7.8 MB |
| 7 | 7L | + Simple English Wikipedia | ~8.0 MB |

All three stages active at 12L. Total active corpus: ~20.1 MB / ~4M tokens.

Training samples random 128-token windows from the concatenated token stream.

### v3.0 Corpus Structure

The v3.0 tokenizer training corpus adds three layers above the stage corpus:

| Layer | Directory | Content | Target size |
|-------|-----------|---------|-------------|
| Wikipedia base | `data/corpus/multilingual/` | Wikipedia dumps × 8 languages | ~446 MB |
| Academic abstracts | `data/corpus/academic/` | arXiv, HAL, SciELO, Zenodo OAI-PMH | ~100 MB |
| Full papers | `data/corpus/fulltext/` | ar5iv, PMC, PERSEE, HAL full text | ~160 MB |
| **Chaos** | `data/corpus/chaos/` | Redacted, truncated, OCR-corrupted, unanswered questions, failed trades | ~30 MB |

### The 90/10 Invariant — Standing Team Rule

**The chaos layer must always represent ~10% of total tokenizer corpus volume. This ratio is non-negotiable and must be maintained as the corpus grows.**

Rationale: A model trained exclusively on clean, resolved text acquires a dangerous prior — it assumes the world closes. The 10% chaos layer shifts that prior, ensuring albert. learns at the statistical baseline that some inputs have no clean resolution. This is the *uncertainty tithe*: expressed in the corpus, in the routing entropy floor (ENTR ≥ 2.480), in the TTL orange-default, and in @sparseskip withholding 75% of expert capacity per step.

**Enforcement:**
- When adding new clean corpus data, add proportional chaos data to maintain the ratio
- `train_tokenizer_v3.py` prints the chaos % at training time and warns if outside 8–12%
- `build_chaos_corpus.py --target-mb N` generates N MB of chaos; scale N with corpus growth
- Do not merge a corpus expansion PR without updating the chaos layer to match

---

## 8. Dashboard Telemetry

`dashboard/index.html` polls `dashboard/training.log` via HTTP byte-range requests every 1500ms. Each new batch produces one dot on the scatter plot. The dashboard tracks:

- **Loss scatter** with drop animation for new dots
- **SMA-30 trend line**
- **EP AVG reference line** (MT5-style dashed orange) — updates once per completed epoch
- **ARCH display** — live from `ARCH NL NH NE NCTX NV` log lines
- **Epoch filter** (ALL / 200 / 100 / 50 / 20 / 10 / 5 EP) doubles as live auto-follow — viewport advances with training frontier when a filter is active
- **TLIGHT heatmap** — expert routing state per layer, color-coded G/O/R
- **MYCELIUM panel** — dead/blooming/hot/cold counts, pressure array per epoch

---

## See Also

- [Main README](../README.md)
- [Checkpoint Registry](../models/README.md)
- [Roadmap](../docs/roadmap.md)
- [Ternary Compression Theory](ternary-compression.md)
