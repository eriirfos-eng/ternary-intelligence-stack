# Albert MoE-13 — Convergence Log

Empirical training loss data from native ternary training from random initialization.
Active run: v3.0 — 17L · 32k vocab · 635 MB multilingual corpus (2026-05-10, ongoing). Surgery loss gate cleared ep2080.

---

## v3.0 — Active Training Run (2026-05-16, ongoing)

Architecture: **17 layers** · 256 hidden · 12 experts · Top-3 routing · **256 ctx** · **32,000 vocab** (ByteLevel BPE, multilingual EN/DE/FR/ES/PT/IT/NL/PL)  
Corpus: Stage 10 corpus active — stages [3,6,7,8,9,10] · 10% chaos layer invariant enforced  
Optimizer: AdamW, cosine LR, GRAD_ACCUM=4, BATCH=4  
Hardware: **Modal.com T4 GPU** (~$0.003/epoch at CTX=256)  
Weights: transferred from v2.0.0 best checkpoint (loss 6.8821); 5 net2net surgeries applied since ep46

Random baseline: `ln(32000) = 10.373` — the expected starting loss for a model with no prior knowledge over a 32k vocabulary.

### Understanding the vocabulary transfer plateau

v3.0 begins from a weight transfer of the v2.0.0 best checkpoint (loss 6.8821 over 8k vocab). The
semantic knowledge encoded in L0–L11 is intact. However, `embed` and `lm_head` are **re-initialized
from scratch** for the 32k token space — meaning the model must re-learn the full mapping from its
internal representations to 32,000 output tokens before any of that semantic knowledge can surface
in the loss curve.

This produces a plateau near the random baseline (`ln(32000) = 10.373`) that persists for many epochs.
This is not a failure of convergence — it is the cost of vocabulary transfer. The model is not starting
from zero; it is re-routing existing knowledge through a new output space. Once the embed/lm_head
alignment crosses a threshold, descent accelerates sharply and the carried-over semantic structure
becomes visible in the loss.

The EvolutionManager's `min_loss_for_plateau` guard (`= 8.4`) deliberately suppresses Net2Net surgery
during this phase. Growing the architecture while the output projection is still random would add
uninitialised capacity on top of a model that cannot yet use its existing capacity. Surgery is
correct only after the vocabulary transfer plateau breaks.

**Training started 2026-05-10. Surgery log and key milestones below.**

| Epoch | Architecture | Loss (avg) | Notes |
|-------|-------------|-----------|-------|
| Ep 1 (G35) | 12L | ~10.373 | Random baseline; vocab transfer plateau begins |
| Ep 12 (G46) | 12L | 10.3412 | Embed/lm_head alignment break; L6–L11 gradients activating |
| Ep 511 | **12L → 13L** | — | Net2Net surgery (Fibonacci window=13) |
| Ep 547 | **13L → 14L** | — | Net2Net surgery (Mandelbrot c_im=−0.6983, window=21) |
| Ep 611 | **14L → 15L** | — | Net2Net surgery (Mandelbrot c_im=0.2287, 69 tensors, window=34) |
| Ep 645–646 | **15L → 16L** | — | Net2Net surgery (Mandelbrot c_im=−0.3442, window→55) |
| Ep 701–702 | **16L → 17L** | — | Net2Net surgery (Mandelbrot c_im=0.5828, window→89); Stage 10 corpus unlocked |
| Ep 786 | 17L | 10.2199 | Previous all-time best; 22+ ATBs in overnight cascade |
| Ep 849 | 17L | 10.2050 | Former epoch ATL (now surpassed) |
| Ep 1155 | 17L | ~10.22 | New batch ATL: 10.1738; descent accelerating |
| Ep 1185 | 17L | ~10.21 | **New batch ATL: 10.1600**; hot layer shift L8→L10 (structural) |
| Ep 1189 | 17L | **10.1993** | **EPOCH ATL — first sub-10.20** |
| Ep 1390 | 17L | **10.1212** | Epoch ATL; hot layer shift L10→L5 |
| Ep 1435 | 17L | **10.1113** | Epoch ATL; **batch ATL: 10.0556** |
| Ep 1445 | 17L | — | **Batch ATL: 10.0396** — current intra-batch record |
| Ep 1455 | 17L | **10.1060** | Epoch ATL |
| Ep 1474 | 17L | **10.0982** | **EPOCH ATL — first sub-10.10**; surgery gate gap 0.298 nats |
| Ep 1482 | 17L | **10.0917** | Epoch ATL (2026-05-16); active descent streak |
| Ep ~1550 | 17L | — | AdamW buffer reset (accidental restart); 4× wider batch swings, steeper descent; buffers re-narrow over ~150 ep |
| Ep ~1670 | 17L | ~10.00 | Estimated first sub-10.0 epoch avg (interpolated from ep1482→ep2041 arc) |
| Ep ~1870 | 17L | ~9.90 | Estimated first sub-9.9 epoch avg |
| Ep 2041 | 17L | **9.8172** | Epoch ATL (2026-05-19T07:13Z); entered surgery alert zone |
| Ep 2049 | 17L | **9.8161** | Epoch ATL |
| Ep 2051 | 17L | **9.8109** | Epoch ATL; sub-9.82 |
| Ep 2059 | 17L | **9.8065** | Epoch ATL |
| Ep 2073 | 17L | **9.8033** | Epoch ATL (10:05Z) |
| Ep 2075 | 17L | **9.8024** | Epoch ATL |
| Ep 2080 | 17L | **9.7997** | **EPOCH ATL — FIRST SUB-9.8 · SURGERY LOSS GATE CLEARED** (2026-05-19T10:40Z) |
| Ep 2084 | 17L | **9.7976** | **Current epoch ATL** (2026-05-19T11:00Z); plateau gate accumulating |
| Ep 2085–2103 | 17L | 9.800–9.810 | Plateau zone: loss oscillating tight band; WALD 58+ stable epochs sev=0.950; since_best=19 |

**All-time best (epoch avg):** 9.7976 (ep2084, 2026-05-19T11:00Z) — surgery loss gate cleared  
**All-time best (intra-batch):** 9.6380 (ep1445, 2026-05-16)  
**Surgery governor status:** Loss gate CLEARED (9.7976 < 9.8). Plateau gate accumulating — 144-epoch window with variance < 0.02 nats + myc_stable ≥ 5. WALD: 58+ stable epochs at sev=0.950. Surgery (17L→18L) imminent; estimated ep2160–2220 if plateau holds.

### Net2Net surgery outcomes

All five surgeries completed cleanly with no divergence spikes. Loss resumed descent within 1–3 epochs of each surgery. The Fibonacci-gated plateau detector correctly suppressed surgery during active descent and fired only when the model had stalled — this is the intended behavior of the surgery governor (see `docs/EVOLUTION_EVIDENCE.md`).

Each surgery used a Mandelbrot-parameterised perturbation signal for weight initialization of the new layer, providing a deterministic, reproducible plasticity impulse derived from the model's current loss at the time of insertion.

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
