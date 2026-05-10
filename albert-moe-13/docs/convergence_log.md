# Albert MoE-13 — Convergence Log

Empirical training loss data from native ternary training from random initialization.
Active run: v3.0 — 12L · 32k vocab · 635 MB multilingual corpus (2026-05-10).

---

## v3.0 — 12L Multilingual Training Run (2026-05-10, active)

Architecture: 12 layers · 256 hidden · 12 experts · 4 heads · 128 ctx · **32,000 vocab** (ByteLevel BPE, multilingual EN/DE/FR/ES/PT/IT/NL/PL)  
Corpus: ~635 MB (Wikipedia CC BY-SA + Europarl + Gutenberg fulltext + academic texts + 10% chaos layer)  
Optimizer: AdamW, cosine LR 3e-4 → 1e-5 / 500 steps  
Hardware: HP ZBook (CPU-only)  
Weights: transferred from v2.0.0 best checkpoint (loss 6.8821); embed and lm\_head re-initialized for 32k vocab

Random baseline: `ln(32000) = 10.373` — the expected starting loss for a model with no prior knowledge over a 32k vocabulary.

**Training started 2026-05-10. Loss data will be appended as epochs complete.**

| Epoch | Avg Loss | Δ vs prev | Notes |
|-------|----------|-----------|-------|
| — | — | — | Corpus tokenization in progress at session start |

### Expected trajectory

The first 5–10 epochs should show rapid descent from ~10.373 toward the carry-over semantic knowledge
embedded in the transferred L0–L11 weights. The embed and lm\_head layers are random, so the model
must re-learn the mapping between its internal representations and the 32k token space.
Layer crystallization (L0–L3 near-frozen from v2.0.0 training) may slow initial descent
in the lower layers; gradient amplification (5× for layers with norm < 0.001) is active to
prevent freezing.

---

## v2.0.0 — 3L Training Run (2026-05-07, current)

Architecture: 3 layers · 256 hidden · 12 experts · 4 heads · 128 ctx · 8000 vocab  
Optimizer: AdamW, cosine LR 3e-4 → 1e-5 / 500 steps  
Hardware: HP ZBook (CPU-only)

| Epoch | Avg Loss | Δ vs prev | Clipped batches | Notes |
|-------|----------|-----------|-----------------|-------|
| 1  | 7.8687 | — | 300/300 | Start from random init |
| 2  | 7.5978 | −0.271 | 300/300 | |
| 3  | 7.3676 | −0.230 | 300/300 | |
| 4  | 7.2316 | −0.136 | 300/300 | |
| 5  | 7.1496 | −0.082 | 300/300 | |
| 6  | 7.0899 | −0.060 | 300/300 | |
| 7  | 7.0059 | −0.084 | 300/300 | |
| 8  | 6.9892 | −0.017 | 300/300 | Clipping tapering |
| 9  | 6.9757 | −0.013 | 256/300 | |
| 10 | 6.9916 | +0.016 | 88/300  | |
| 11 | **6.9542** | **−0.037** | 30/300 | **3L capacity ceiling — best epoch avg** |
| 12 | 6.9821 | +0.028 | 10/300  | Clipping nearly gone |
| 13 | 7.0777 | +0.096 | 1/300   | Divergence begins — LR overshooting minimum |
| 14 | 7.0665 | −0.011 | 7/300   | |
| 15 | 7.1131 | +0.047 | 3/300   | |
| 16 | 7.2306 | +0.118 | 0/300   | |
| 17 | 7.2632 | +0.033 | 1/300   | |
| 18 | 7.4088 | +0.146 | 1/300   | |
| 19 | 7.4606 | +0.052 | 0/300   | |
| 20 | 7.6036 | +0.143 | 0/300   | |
| 21 | 7.7064 | +0.103 | 0/300   | |
| 22 | 7.8589 | +0.153 | 0/300   | EvolutionManager plateau/divergence window |
| 23 | 7.9890 | +0.130 | 0/300   | |
| 24 | 8.0392 | +0.050 | 0/300   | |
| 25 | 8.1024 | +0.063 | 0/300   | Surgery expected to fire ~ep 25–30 |

**Best single-batch loss observed:** 5.5471 (epoch ~11, batch-level minimum)  
**Best epoch average:** 6.9542 (epoch 11)

### Interpretation

The training curve shows three phases consistent with 3L capacity dynamics:

1. **Rapid descent (ep 1–8):** All batches clipped — model making large gradient updates, learning fast. Loss drops 7.87 → 6.99 (−0.88 in 8 epochs).
2. **Capacity ceiling (ep 9–11):** Clipping tapers as gradients normalize. Model finds its 3L minimum at 6.9542.
3. **Divergence (ep 12+):** Without clipping dampening the effective LR, the optimizer overshoots the minimum. Loss rises steadily. This is the expected signal for EvolutionManager surgery to 4L.

This pattern — descent, plateau, divergence — is the intended behavior. The EvolutionManager detects the divergence and fires Net2Net surgery, copying the 3L best checkpoint into a 4L architecture for the next training phase.

---

## v1.x — Previous runs

Previous runs reached loss ~6.10 at 3L before an architecture bug (Net2Net surgery deleting the best checkpoint instead of archiving it) caused all post-surgery models to diverge to random baseline (8.987). This was diagnosed and fixed on 2026-05-07. See `ternlang-root/docs/session_log.md` entry `2026-05-07 · 22:00` for full root cause analysis.
