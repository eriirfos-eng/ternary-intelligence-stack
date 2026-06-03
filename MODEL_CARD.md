---
language:
- en
- de
- fr
- es
- pt
- it
- nl
- pl
- multilingual
license: lgpl-3.0
tags:
- mixture-of-experts
- ternary-weights
- moe
- edge-ai
- research
- candle
- rust
- federated-learning
- low-precision
- sprind
- dual-stream
- cord-surgery
- net2net
pipeline_tag: text-generation
---

# Model Card — albert. (Albert-MoE-13)

**Version:** v3.0 (ternary MoE)  
**Maintainer:** RFI-IRFOS, contact@ternlang.com  
**Repository:** https://github.com/eriirfos-eng/ternary-intelligence-stack  
**License:** LGPL-3.0-or-later (model weights, training code, inference runtime). Platform infrastructure (API server, MCP tooling, HDL) is BSL-1.1. See [README §Licensing](README.md#licensing) for the full tier breakdown.  
**Last updated:** 2026-05-27  
**Training status:** Paused (Modal billing ceiling, ep4234) — **26L dual-stream** · 13 depth surgeries + 1 cord surgery complete. Cord surgery fired autonomously ep4202, 2026-05-27T16:44Z — first documented single-to-dual-stream bifurcation mid-training. S13 (25L→26L) fired ep~4207. Chip ATL **8.6852** (post-S13). EP_AVG ATL **9.2847** (ep3456, 20L). fib_index=7 · window=34 · Gen3 step1/6. Resuming on Modal T4 once billing settled.

---

## Model Overview

albert. is a research-grade language model trained from scratch using a
**ternary weight representation** (-1, 0, +1) with a Mixture-of-Experts
(MoE) architecture. It is developed by RFI-IRFOS as a demonstration that
high-quality language modelling is achievable without 32-bit floating-point
weights, targeting inference on edge hardware and low-power devices.

| Property | Value |
|----------|-------|
| Architecture | **Dual-stream** Ternary MoE (Mixture of Experts) |
| Streams | **2** (bifurcated via cord surgery ep4202, 2026-05-27) |
| Layers | **28** per stream |
| Hidden size | **2×256H** (256H per stream) |
| Anastomosis gates | **6** — bidirectional F32 cross-stream fusion at Fibonacci layers [2,3,5,8,13,21] |
| Experts | 12 per stream (Top-3 routing; shared FFN weights, independent routing gates) |
| Context length | 256 tokens |
| Vocabulary | 32,000 tokens (ByteLevel BPE — EN/DE/FR/ES/PT/IT/NL/PL) |
| Weight representation | Ternary {-1, 0, +1} with STE training |
| Gate linear | F32 |
| Positional encoding | RoPE (rotate_half) |
| Optimizer | AdamW, cosine LR decay, BATCH=1 (post-cord) |
| Parameters (total) | **~194.4M** |
| Safetensors | **2,044 tensors · 741.4 MB** |
| Surgeries | **13 depth (S1–S13)** + **1 cord surgery** = 14 total surgical events |

The central technical innovation is the **@sparseskip** primitive — a
learned sparse-skip layer that dynamically bypasses computation paths
based on token-level activation patterns, enabling sub-linear inference
scaling without pruning.

---

## Intended Use

**Intended uses:**

- Research into ternary and low-precision neural network architectures
- Benchmarking inference performance on CPU and edge GPU hardware
- Academic study of Mixture-of-Experts routing dynamics
- Demonstration platform for the SPRIND AI funding initiative (Germany)

**Out-of-scope uses:**

- Production deployment as a general-purpose assistant without further
  fine-tuning and safety evaluation
- Safety-critical applications (medical, legal, financial decisions)
- Any use requiring factual accuracy guarantees
- Deployment to users without appropriate transparency disclosure

---

## Training Data

See [DATA_PROVENANCE.md](DATA_PROVENANCE.md) for full source documentation
and governance details.

**Summary:**

albert. is trained on a curated multilingual corpus composed of:

| Tier | Content | Approximate Share |
|------|---------|------------------|
| Core | Project Gutenberg (public domain books, multilingual) | ~30% |
| Core | Wikipedia (15 languages: EN, DE, FR, HU, ZH, AR, KO, SV, FI, NL, PL, RU, JA + more) | ~25% |
| Core | OpenWebText (filtered Common Crawl) | ~15% |
| Technical | GitHub issues, developer blogs, HN discussions | ~10% |
| Chaos | Synthetic noise, adversarial patterns, mixed-language text | ~10% |
| Structured | Code samples, structured data (JSON/YAML/TSV) | ~5% |
| Multilingual | Additional EU language samples | ~5% |

The **10% chaos layer** is a structural invariant enforced by the training
pipeline (`train_tokenizer_v3.py`). It exists to prevent the model from
over-fitting to clean text distributions and to improve robustness to
noisy inputs.

---

## Evaluation

**Primary metric:** Cross-entropy loss on a held-out WikiText-2 sample
(`eval_sample.txt`, not seen during training).

**Benchmark results (benchmark suite v2.0.0):**

| Epoch | Loss (avg) | Epoch ATL | Batch ATL | Tok/s (T4 GPU) |
|-------|-----------|-----------|-----------|----------------|
| Ep54  | ~10.35 | 10.35 | — | 11.24 (CPU) |
| Ep111 | ~10.36 | 10.36 | — | 18.52 |
| Ep849 | ~10.22 | 10.2050 | — | pending |
| Ep1177 | 10.2076 | 10.2059 (ep1158) | 10.1738 (ep1155) | pending |
| Ep1390 | 10.1212 | 10.1212 (ep1390) | 10.0670 (ep1385) | pending |
| Ep1435 | 10.1113 | 10.1113 (ep1435) | **10.0556** (ep1435) | pending |
| Ep1438 | 10.1071 | 10.1071 (ep1438) | **10.0556** (ep1435) | pending |
| Ep1441 | 10.1067 | 10.1067 (ep1441) | 10.0556 (ep1435) | pending |
| Ep1455 | 10.1060 | **10.1060** (ep1455) | **10.0396** (ep1445) | pending |
| Ep1474 | 10.0982 | **10.0982** (ep1474) | **10.0396** (ep1445) | pending |
| Ep1553 | ~10.07 (est) | 10.0982 (ep1474) | **9.9948** (ep1553) ← first sub-10.0 batch | pending |
| Ep2040 | ~9.82 (est) | 9.7976 (ep2084) | **9.6380** (ep1445) | 9.6–18.5 (CPU) |
| Ep2084 | **9.7976** | **9.7976** ← epoch ATL | 9.6380 (ep1445) | pending (T4) |
| Ep2104 | ~9.81 (est) | 9.7976 (ep2084) | 9.6380 (ep1445) | **9.9–21.3 (CPU)** |
| Ep2109 | 9.7975 | **9.7975** (ep2109) | 9.6380 (ep1445) | pending |
| Ep2114 | 9.7891 | 9.7891 (ep2114) | **9.6235** (ep2114) ← batch ATL | pending |
| Ep2116 | **9.7884** | **9.7884** ← epoch ATL | **9.6235** (ep2114) | pending |
| Ep2487 | S6 fired | 18L→19L surgery | — | 2026-05-20T21:33Z; Gen1 step1/6 |
| Ep2922 | **9.4992** | **9.4992** ← first sub-9.50 | 9.1370 (chip) | 2026-05-22; LOG expert 0%→28% awakening |
| Ep3263 | **9.3651** | ← epoch ATL | **9.0095** (chip) | Broke 139-epoch plateau |
| Ep3325 | S7 fired | 18L→19L surgery | — | 2026-05-24T13:47Z; 1315 tensors |
| Ep3326 | **9.3182** | ← epoch ATL (first 19L ep) | **8.9190** (chip) | +0.047 nat improvement over prior best |
| Ep3383 | S8 fired | 19L→20L surgery | — | Only 58 epochs after S7 |
| Ep3456 | **9.2847** | ← epoch ATL (20L) | **8.8540** (chip) | WALD ep3454 INT 91% cliff |
| Ep~3470 | S9 fired | 20L→21L surgery | — | Largest post-surgery spike in history (+0.14 nat) |
| Ep~3652 | S10 fired | 21L→22L surgery | — | Pre-surgery best 9.2933 |
| Ep~4098 | S11 fired | 22L→23L surgery | — | 2026-05-27 morning |
| Ep~4140 | S11b fired | 23L→24L surgery | — | Rapid plateau ~42 ep after S11 |
| Ep4202 | S12 fired | 24L→25L surgery | — | 2026-05-27T16:43Z; Gen3 plateau |
| **Ep4202** | **CORD surgery** | **25L → 2×25L dual-stream** | — | **2026-05-27T16:44Z — first ever autonomous single→dual-stream bifurcation** |
| Ep~4203 | 9.3241 | ← first post-cord epoch avg | **8.7123** (chip, new ATL) | Dual-stream live |
| Ep~4207 | S13 fired | 25L→26L surgery (both streams) | **8.6852** (chip, new ATL) | 2026-05-27T17:40Z; fib_index 6→7 |

The benchmark suite runs 5 fixed prompts covering English, German,
multilingual, narrative, and technical domains. Results are reproducible
via the open-source `moe-test` binary.

**Surgery gate prediction (recorded 2026-05-16T18:40Z) — outcome update 2026-05-19:**

A trendline fitted to the ep400–ep1459 loss curve predicted the surgery gate threshold (9.8 epoch-avg) at approximately **ep~2000**. **Prediction confirmed**: the loss gate was cleared at ep2080 (9.7997, 2026-05-19T10:40Z), within the predicted ep2000–2150 window.

The gate fires when loss plateaus below 9.8 for a 144-epoch window with `myc_stable ≥ 5`. The loss gate was cleared at ep2080. Following that, albert. entered an **alternating descent phase**: five new epoch ATLs in seven epochs (ep2109–ep2116), dropping from 9.7976 → 9.7884 in under two hours of wall time. WALD sev=0.953; myc_L3 showed its first activity uptick (1.61→1.68 ×10⁻⁹) at ep2114. The plateau gate cannot fire during active descent — surgery timing is now conditioned on when the model settles into the next attractor, not on a fixed epoch countdown.

**Milestone (2026-05-17T05:48Z):** First sub-10.0 batch loss in albert. history — **9.9948** at ep1553 batch 149/300.  
**Milestone (2026-05-19T10:40Z):** First sub-9.8 epoch average — **9.7997** at ep2080. Surgery loss gate cleared.  
**Milestone (2026-05-19T11:00Z):** New epoch ATL — **9.7976** at ep2084.  
**Milestone (2026-05-19T13:29Z):** New batch ATL — **9.6235** at ep2114 (prev 9.6282).  
**Milestone (2026-05-19T13:40Z):** Alternating descent confirmed — five new epoch ATLs in seven epochs; epoch ATL reaches **9.7884** at ep2116.

**Known limitations:**

- At current training depth (~1459 epochs), output quality is pre-fluency:
  the model produces partially coherent text in familiar domains but lacks
  consistent grammatical structure across longer sequences.
- Context window of 256 tokens is shorter than contemporary LLMs; cannot
  maintain coherence over longer passages.
- Ternary quantization trades weight precision for size — at this scale,
  some representational capacity is lost relative to F32 equivalents.
- No instruction-following fine-tuning has been applied.
- No RLHF, Constitutional AI, or safety fine-tuning of any kind.
- Bias evaluation is pending (see below).

**Open research questions (scaling risks):**

- **STE gradient approximation at scale:** Straight-Through Estimation is the training mechanism for ternary weights. Its stability and convergence properties are well-characterised at current scale (~58M params). Whether STE remains stable through training runs at 500M–1B+ parameters is an open empirical question — no published work has demonstrated ternary STE convergence at frontier scale.
- **@sparseskip speedup baseline:** The 83 tok/s inference figure is measured against albert.'s own F32-weight dense equivalent on the same hardware. It is not a direct comparison with INT4-quantized industry inference (TensorRT-LLM, llama.cpp Q4). The relevant claim is that ternary weights eliminate a quantization step entirely — the speedup over post-hoc INT4 quantization of a larger model is a separate, untested question.
- **Net2net surgery stability at scale:** All five documented layer-addition surgeries were performed on a model in the 13M–58M parameter range. Whether the Fibonacci-gated surgery protocol remains stable when applied to models at 200M+ parameters has not been tested. The plateau-gate's **withhold behavior** is now validated across six independent events (ep791 non-firing + alternating descent phase ep2109–ep2120 — see below); the question of whether these properties hold at 200M+ scale remains open.

**Validated finding — surgery governor robustness (ep2120):** The plateau gate demonstrated robustness against premature surgery triggering: at ep2120, despite crossing the loss threshold (9.7997 < 9.80) at ep2081, the model continued descending through the projected plateau zone, invalidating three pre-computed surgery timing scenarios. The governor correctly withheld surgery while the model was still actively learning — a validation of the design principle that architecture should grow only when learning has genuinely exhausted current capacity. Five new epoch ATLs were recorded in seven epochs (9.7976→9.7884) during the withheld window. Full technical record: [convergence_log.md — Alternating Descent Phase section](docs/convergence_log.md).

---

## Bias and Fairness

A formal bias and fairness evaluation has not yet been conducted. Known
risk factors:

- **Language imbalance:** English-dominant corpus; non-English outputs
  will be lower quality.
- **Temporal bias:** Training data has a knowledge cutoff; the model
  has no awareness of events after its corpus snapshot dates.
- **Domain gaps:** Limited coverage of non-Western cultural contexts,
  legal jurisdictions outside EU/US/DE, and specialized professional
  domains.

A structured bias evaluation using standard benchmarks (WinoBias,
BBQ, multilingual MMLU) is planned for the v3.1 milestone.

---

## Human Oversight

albert. is a research model under active development. The following
oversight mechanisms are in place:

1. **Training dashboard:** Real-time monitoring of loss curves, expert
   routing, gradient norms, WALD dead-zone events, and anomaly events
   by the RFI-IRFOS team.
2. **Surgery governor:** Architectural growth (layer addition via net2net)
   is fully autonomous — the `EvolutionManager` fires on a Fibonacci-gated
   plateau detector with no human intervention required. **13 depth surgeries
   (12L→26L) + 1 cord surgery (single→dual-stream bifurcation)** have been
   executed autonomously to date. The cord surgery (ep4202, 2026-05-27) is the
   first documented autonomous single-to-dual-stream bifurcation in a live
   ternary MoE.
3. **SPORE federated training (live):** Collaborators contribute CPU-trained
   checkpoints as weight spores via the `albert-spores` private repository.
   The `SporeManager` blends accepted spores at α=0.08 each epoch boundary
   with fitness (loss gate) and architecture guards. Colony is active as of
   2026-05-16 with external contributors. Spores are stored via Git LFS;
   each contributor runs `albert-train` locally and submits via `albert-spore`.
4. **Checkpoint promotion:** No trained checkpoint is deployed to any
   external service without explicit human review and approval by the
   lead architect.
5. **Rollback capability:** All checkpoints and best-loss weights are
   preserved on persistent storage. Any version can be reverted.

See [SECURITY.md](SECURITY.md) for the incident reporting process.

---

## EU AI Act Compliance Notes

albert. is developed in the European Union and is subject to Regulation
(EU) 2024/1689 (EU AI Act). RFI-IRFOS self-classifies albert. as a
**General-Purpose AI (GPAI) model** under Article 3(63).

| Obligation | Article | Status |
|------------|---------|--------|
| Technical documentation | Annex XI | This document |
| Training data summary | Art. 53(1)(d) | [DATA_PROVENANCE.md](DATA_PROVENANCE.md) |
| Copyright compliance summary | Art. 53(1)(c) | [DATA_PROVENANCE.md](DATA_PROVENANCE.md) |
| Human oversight measures | Art. 53(1)(e) | Described above |
| Incident reporting | Art. 53(2) | [SECURITY.md](SECURITY.md) |
| Bias/fairness assessment | Art. 53(1)(b) | Planned v3.1 |

For questions about compliance or to report concerns:
contact@ternlang.com

---

## Team

| Name | Role | Contact |
|------|------|---------|
| Simeon Kepp | Lead Architect — full stack (compiler, BET VM, training, MCP) | s.kepp@ternlang.com |
| Louis Paul Ehrig | Head of Public Affairs, Dataset Curation, Corporate Secretary | l.ehrig@ternlang.com |
| Lisa Scharler | Head of Social Technology & Ecocentric Systems | l.scharler@ternlang.com |
| Zabih Karimi | Co-Founder, IT & Infrastructure, Stress-Testing | z.karimi@ternlang.com |
| Nikoletta Csonka | Global Reach, Fundraising & Fund Applications | csonikoletta@ternlang.com |
| Claude (Anthropic) | AI Collaborator — architecture, implementation, monitoring | claude@ternlang.com |

**Organisation:** Research Focus Institute — Interdisciplinary Research Facility for Open Sciences (RFI-IRFOS)  
**Address:** Elisabethinergasse 25, 8020 Graz, Austria  
**Website:** https://ternlang.com  
**Issues:** https://github.com/eriirfos-eng/ternary-intelligence-stack/issues  
**General contact:** contact@ternlang.com
