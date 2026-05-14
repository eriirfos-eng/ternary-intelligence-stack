# Model Card — albert. (Albert-MoE-13)

**Version:** v3.0 (ternary MoE)  
**Maintainer:** RFI-IRFOS, rfi.irfos@gmail.com  
**Repository:** https://github.com/eriirfos-eng/ternary-intelligence-stack  
**License:** LGPL-3.0-or-later  
**Last updated:** 2026-05-14

---

## Model Overview

albert. is a research-grade language model trained from scratch using a
**ternary weight representation** (-1, 0, +1) with a Mixture-of-Experts
(MoE) architecture. It is developed by RFI-IRFOS as a demonstration that
high-quality language modelling is achievable without 32-bit floating-point
weights, targeting inference on edge hardware and low-power devices.

| Property | Value |
|----------|-------|
| Architecture | Ternary MoE (Mixture of Experts) |
| Layers | 17 |
| Hidden size | 256 |
| Experts | 12 (Top-3 routing per token) |
| Context length | 256 tokens |
| Vocabulary | 32,000 tokens (custom BPE) |
| Weight representation | Ternary {-1, 0, +1} with STE training |
| Gate linear | F32 |
| Positional encoding | RoPE (rotate_half) |
| Optimizer | AdamW, cosine LR decay |
| Parameters (total stored) | ~58M ternary |
| Parameters (active per token) | ~13M effective (Top-3 of 12 experts) |

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
| Core | Wikipedia (English, German, French subsets) | ~25% |
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

| Epoch | Loss (avg) | All-time best | Tok/s (T4 GPU) |
|-------|-----------|--------------|----------------|
| Ep54  | ~10.35 | 10.35 | 11.24 (CPU) |
| Ep111 | ~10.36 | 10.36 | 18.52 |
| Ep849 | ~10.21 | 10.2050 | pending |

The benchmark suite runs 5 fixed prompts covering English, German,
multilingual, narrative, and technical domains. Results are reproducible
via the open-source `moe-test` binary.

**Known limitations:**

- At current training depth (~850 epochs), output quality is pre-fluency:
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
- **Net2net surgery stability at scale:** All five documented layer-addition surgeries were performed on a model in the 13M–58M parameter range. Whether the Fibonacci-gated surgery protocol remains stable when applied to models at 200M+ parameters has not been tested. The current plateau-gate mechanism assumes continued smooth descent after insertion — this assumption is unverified beyond current scale.

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
   routing, gradient norms, and anomaly events by the RFI-IRFOS team.
2. **Surgery governor:** Automated architectural growth (layer addition)
   is gated by a plateau detector that requires human review of the
   training state before triggering structural changes.
3. **Checkpoint promotion:** No trained checkpoint is deployed to any
   external service without explicit human review and approval by the
   lead architect.
4. **Rollback capability:** All checkpoints and best-loss weights are
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
rfi.irfos@gmail.com

---

## Contact

**Lead Architect:** Simeon Ari  
**Organisation:** RFI-IRFOS  
**Email:** rfi.irfos@gmail.com  
**Website:** https://ternlang.com  
**Issues:** https://github.com/eriirfos-eng/ternary-intelligence-stack/issues
