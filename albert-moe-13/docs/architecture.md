# Albert MoE-13 — Architecture

## Overview

Albert MoE-13 is a ternary-native transformer with Mixture-of-Experts feed-forward layers. Every weight matrix is quantized to {-1, 0, +1} during both forward and backward passes via Straight-Through Estimation. The architecture is designed to grow its own depth autonomously through the `EvolutionManager`.

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
6. Token masking (`combined_weight > 0.0`) skips computation for tokens not routed to a given expert

---

## 3. Auto-Evolutionary Training

The `EvolutionManager` monitors epoch-average loss and triggers **Net2Net safe-copy surgery** when the model plateaus:

```
history_len    = 10 epochs
plateau_threshold = 0.02   (< 2% improvement across 10 epochs → expand)
mastery_threshold = 4.5    (loss < 4.5 → model has mastered current capacity)
divergence_threshold = 0.1 (loss rising → needs more capacity)
```

**Surgery procedure:**
1. `config.json` `num_layers` is incremented
2. The new layer's weights are initialized as a safe copy of the deepest existing layer (Net2Net — preserves learned representations)
3. Training resumes with the expanded architecture
4. The model grows from 3L up to a maximum of 12L

---

## 4. Training Loop

```
for each epoch (300 batches):
    for each batch:
        sample random 128-token window from corpus
        forward pass through transformer
        ce_loss = cross_entropy(logits, targets)
        l1_penalty = λ * mean(|weights|) for all weight matrices
        total_loss = ce_loss + l1_penalty
        opt.backward_step(total_loss)   ← single optimizer step
        log to dashboard/training.log
    write epoch summary to log
    save checkpoint
    check EvolutionManager → surgery if triggered
```

**Learning rate** — cosine schedule cycling every 500 global steps:
```
lr(t) = lr_min + 0.5 * (lr_max - lr_min) * (1 + cos(π * t / T))
lr_max = 2e-4,  lr_min = 1e-5,  T = 500
```

---

## 5. Corpus Pipeline

`load_corpus()` at startup reads every `.txt` file from `data/corpus/` in alphabetical order and concatenates them into a single token stream. Training samples random 128-token windows from this stream.

Current active corpus: `bible.txt` + `alice.txt` (~4.5MB).  
Staged for future use: Simple English Wikipedia, 12 Gutenberg classics, EU AI Act, Linux kernel docs, TLDR pages (`data/corpus_staged/`).

---

## 6. Dashboard Telemetry

`dashboard/index.html` polls `dashboard/training.log` via HTTP byte-range requests every 1500ms. Each new batch produces one dot on the scatter plot. The dashboard tracks:

- **Loss scatter** with drop animation for new dots
- **SMA-30 trend line**
- **EP AVG reference line** (MT5-style dashed orange) — updates once per completed epoch
- **ARCH display** — live from `ARCH NL NH NE NCTX NV` log lines
- **Epoch filter** (ALL / 200 / 100 / 50 / 20 / 10 / 5 EP) doubles as live auto-follow — viewport advances with training frontier when a filter is active

---

## See Also

- [Main README](../README.md)
- [Checkpoint Registry](../models/README.md)
- [Ternary Compression Theory](ternary-compression.md)
