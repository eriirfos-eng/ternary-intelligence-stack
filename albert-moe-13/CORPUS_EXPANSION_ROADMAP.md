# Corpus Expansion Roadmap — albert. v3.0+

**Project:** Albert-MoE-13 / Ternary Intelligence Stack  
**Prepared by:** RFI-IRFOS  
**Date:** 2026-05-14  
**Status:** Active — Stages 1–10 deployed, Stages 11–14 pre-built and waiting

---

## Philosophy

Albert. does not receive new data until its architecture can use it. Adding corpus to a model that hasn't yet extracted the available signal from its current data wastes compute and risks diluting learned structure. The surgery governor enforces this at the architecture level. This roadmap enforces it at the data level.

Each stage is gated by a loss threshold AND an architecture state. Both conditions must be met before a new stage unlocks. The scripts are written in advance, ready to run. The model grows into the data — the data does not chase the model.

**The invariant that never breaks:** the chaos layer is always exactly 10% of total corpus. Every stage includes a proportional chaos complement. The `train_tokenizer_v3.py` enforcer validates this on every tokenizer rebuild. This is non-negotiable and scales with the corpus indefinitely.

---

## Scaling Overview

| Stage | Loss gate | Architecture gate | Corpus size | Tokens (est.) | Chaos complement |
|-------|----------|------------------|-------------|---------------|-----------------|
| 1–10 | — | 17L · 1 stream | ~635 MB | ~150M | included |
| **11** | loss ≤ 7.0 | 17L+ · cord surgery done | ~5 GB | ~1.2B | ~133M tokens |
| **12** | loss ≤ 5.0 | 25L+ · dual stream | ~45 GB | ~10B | ~1.1B tokens |
| **13** | loss ≤ 3.0 | 40L+ · 4-stream cord | ~450 GB | ~100B | ~11B tokens |
| **14** | loss ≤ 2.0 | 65L+ · 8-stream cord | ~4 TB | ~1T | ~111B tokens |

Token estimates assume ~4 characters/token average across mixed multilingual corpus.

---

## Stage 11 — "Deep Roots"

**Unlock condition:** loss_avg ≤ 7.0 AND cord surgery (dual-stream) has fired  
**Target:** ~1.2B tokens (≈5 GB text)  
**Build script:** `albert-moe-13/scripts/build_stage11.py`  
**Output dir:** `data/corpus/stage_11/`

### Rationale
At loss 7.0, the model has developed genuine word-level pattern recognition. It can begin to benefit from scientific abstracts (structured argument), multilingual expansion (cross-lingual transfer), and formal legislative text (precise, rule-governed language). These are out of reach at loss 10+ but well-matched to a model that has broken through the vocabulary transfer plateau.

### Sources

| Source | Format | Est. tokens | License |
|--------|--------|-------------|---------|
| Wikipedia: RU, ZH, JA, AR, HI, KO | Plain text via Wikimedia API | ~400M | CC BY-SA 4.0 |
| arXiv abstracts (all categories) | Structured [ABSTRACT] blocks | ~200M | arXiv ToS (research use) |
| EUR-Lex EU legislation | Plain text, EN/DE/FR/ES | ~150M | Public domain |
| Wikisource multilingual | Public domain literary texts | ~200M | CC BY-SA 4.0 |
| Mathematics Stack Exchange | Q&A threads via SE API | ~80M | CC BY-SA 4.0 |
| Physics / Chemistry / Biology SE | Q&A threads via SE API | ~80M | CC BY-SA 4.0 |
| Chaos complement (10%) | `build_chaos_corpus.py` | ~130M | Generated |

### Run
```bash
cd albert-moe-13
python scripts/build_stage11.py
python scripts/build_chaos_corpus.py --scale stage11
```

---

## Stage 12 — "Mycelial Reach"

**Unlock condition:** loss_avg ≤ 5.0 AND architecture ≥ 25L dual-stream  
**Target:** ~10B tokens (≈45 GB text)  
**Build script:** `albert-moe-13/scripts/build_stage12.py`  
**Output dir:** `data/corpus/stage_12/`

### Rationale
Loss 5.0 corresponds to perplexity ~148 — the model begins producing partially coherent sentences with visible domain structure. At this level, code, biomedical text, and legal reasoning become productive training signal. Code specifically is high-density structured signal: every function has a name, a body, and an implicit contract. The model at this stage can start to learn type-level reasoning.

### Sources

| Source | Format | Est. tokens | License |
|--------|--------|-------------|---------|
| The Stack (Python, Rust, TypeScript, C, Go) | Source code via HuggingFace | ~4B | Apache 2.0 subset |
| PubMed abstracts (all years) | Biomedical [TITLE]/[ABSTRACT] | ~600M | Public domain |
| CourtListener US court opinions | Legal reasoning text | ~500M | Public domain |
| CrossRef academic abstracts | Multi-domain scientific | ~400M | Varies (TDM) |
| EU case law (EUR-Lex CELEX) | Legal reasoning EN/DE/FR | ~300M | Public domain |
| Wikipedia full EN dump | Complete articles | ~4B | CC BY-SA 4.0 |
| Chaos complement (10%) | `build_chaos_corpus.py` | ~1.1B | Generated |

### Run
```bash
cd albert-moe-13
python scripts/build_stage12.py
python scripts/build_chaos_corpus.py --scale stage12
```

---

## Stage 13 — "Fruiting Body"

**Unlock condition:** loss_avg ≤ 3.0 AND architecture ≥ 40L 4-stream cord  
**Target:** ~100B tokens (≈450 GB text)  
**Build script:** `albert-moe-13/scripts/build_stage13.py`  
**Output dir:** `data/corpus/stage_13/`  
**Compute requirement:** Bizon-class hardware (4× GPU) recommended for tokenisation at this scale

### Rationale
Loss 3.0 is GPT-2 medium territory — the model produces fluent prose in familiar domains. At this level, full paper bodies (not just abstracts), patent claims, Internet Archive books, and instruction-tuning dialogue become productive. The model can start reasoning across long documents, not just within sentences. Stage 13 is where cross-domain synthesis becomes learnable.

### Sources

| Source | Format | Est. tokens | License |
|--------|--------|-------------|---------|
| arXiv full papers (bulk S3) | LaTeX → plain text pipeline | ~30B | arXiv ToS |
| Internet Archive books (out of copyright) | Plain text via Archive.org API | ~20B | Public domain |
| USPTO patent abstracts + claims | Structured technical text | ~8B | Public domain |
| Wikipedia all-language dump (top 30) | Full articles | ~15B | CC BY-SA 4.0 |
| OpenCorporates business text | Structured entity descriptions | ~2B | Open |
| Instruction dialogue (curated open datasets) | Multi-turn conversation | ~5B | Varies (open) |
| WikiData facts → natural language | Structured → prose pipeline | ~3B | CC0 |
| Chaos complement (10%) | `build_chaos_corpus.py` | ~11B | Generated |

### Run
```bash
cd albert-moe-13
python scripts/build_stage13.py          # downloads + filters
python scripts/build_chaos_corpus.py --scale stage13
```

---

## Stage 14 — "Sovereign Intelligence"

**Unlock condition:** loss_avg ≤ 2.0 AND architecture ≥ 65L 8-stream cord AND Bizon 4-GPU active  
**Target:** ~1T tokens (≈4 TB text)  
**Build script:** `albert-moe-13/scripts/build_stage14.py`  
**Output dir:** `data/corpus/stage_14/`  
**Storage requirement:** ~8 TB (raw + processed)  
**Compute requirement:** Bizon 4× GPU mandatory — Modal T4 insufficient at this token scale

### Rationale
Loss 2.0 is GPT-3 class. Stage 14 is the frontier expansion: filtered Common Crawl at scale, full multilingual coverage, forum reasoning, long-form journalism, and mathematical proof corpora. This is not about new domains — it is about saturation of existing domains at massive scale. The model at this stage has seen enough language structure that volume of exposure becomes the primary driver of quality.

### Sources

| Source | Format | Est. tokens | License |
|--------|--------|-------------|---------|
| OSCAR 23.01 (filtered Common Crawl) | Multilingual web text | ~600B | CC0 / open |
| The Stack full (all languages) | Source code | ~200B | Apache 2.0 subset |
| mC4 multilingual | Filtered web text | ~100B | ODC-BY |
| StackExchange full data dump | All sites, all years | ~30B | CC BY-SA 4.0 |
| News archive (CC-licensed) | Timestamped journalism | ~20B | CC |
| Mathematical proofs (Lean/Coq/Isabelle) | Formal proof corpora | ~2B | Open |
| Chaos complement (10%) | `build_chaos_corpus.py` | ~110B | Generated |

### Run
```bash
# Requires pre-downloaded OSCAR and Stack dumps (see build_stage14.py --help)
cd albert-moe-13
python scripts/build_stage14.py --oscar-path /data/oscar --stack-path /data/thestack
python scripts/build_chaos_corpus.py --scale stage14
```

---

## The Chaos Invariant at Scale

The 10% chaos layer is not a fixed dataset — it scales proportionally with every stage. At Stage 14, the chaos complement is ~111B tokens. The `build_chaos_corpus.py` script accepts a `--scale` flag that reads the current stage's target token count and generates the appropriate chaos volume:

```
chaos_tokens = total_corpus_tokens * (10 / 90)
```

Chaos content at scale includes:
- Synthetic noise at multiple character/token/sentence levels
- Adversarial mixed-language fragments
- Random byte sequences encoded as BPE tokens
- Reversed, scrambled, and partially corrupted real text
- Cross-domain splice sequences (code → poetry → legal → physics)

The invariant serves two functions that become more important, not less, as the corpus grows: (1) prevents the model from treating any distribution as "the" distribution of language, and (2) maintains robustness to noisy inference-time inputs across all quality levels of deployment hardware.

---

## Architecture × Corpus Scaling Map

The full growth path integrating architectural milestones and corpus stages:

```
NOW:
  17L · 1 stream · 12E · 256H · 150M tokens · loss ~10.2

DEPTH (autonomous, EvolutionManager):
  25L · 1 stream · 12E · 256H · 150M tokens · loss ~8

CORD SURGERY ×1:
  25L · 2 streams · 12E · 256H ─── STAGE 11 UNLOCK (loss ≤ 7.0)
  30L · 2 streams · 12E · 256H · 1.2B tokens · loss ~6

DEPTH (both streams grow together):
  40L · 2 streams · 12E · 256H · 1.2B tokens · loss ~5

CORD SURGERY ×2:                    STAGE 12 UNLOCK (loss ≤ 5.0)
  40L · 4 streams · 12E · 256H · 10B tokens · loss ~4
  50L · 4 streams · 24E · 256H · 10B tokens · loss ~3

CORD SURGERY ×3:                    STAGE 13 UNLOCK (loss ≤ 3.0)
  50L · 8 streams · 24E · 256H · 100B tokens · loss ~2.5
  65L · 8 streams · 24E · 256H · 100B tokens · loss ~2.0

CORD SURGERY ×4:                    STAGE 14 UNLOCK (loss ≤ 2.0)
  65L · 16 streams · 48E · 256H · 1T tokens · loss ~1.5
  80L · 16 streams · 48E · 256H · 1T tokens · loss ~1.2

INSTRUCTION TUNING (separate phase):
  loss ~1.0 on pre-training → target perplexity <5 on instruction-following
```

Expert count grows at cord ×2 to prevent routing bottleneck: 12E × 4 streams would oversaturate the shared expert pool. At 4 streams, expert count doubles to 24 to maintain the effective 3/12 routing ratio per stream.

---

## Compute Projection

| Phase | Hardware | Epochs | Cost estimate | Wall-clock |
|-------|----------|--------|--------------|------------|
| Now → loss 7 | Modal T4 | ~500 ep | ~$3.50 | ~14 days |
| loss 7 → 5 (dual stream) | Modal T4 | ~300 ep | ~$2.50 | ~8 days |
| loss 5 → 3 (4-stream, Stage 12) | Bizon 4×GPU | ~500 ep | ~$15 | ~2 days |
| loss 3 → 2 (8-stream, Stage 13) | Bizon 4×GPU | ~1000 ep | ~$40 | ~4 days |
| loss 2 → 1.2 (16-stream, Stage 14) | Bizon 4×GPU | ~2000 ep | ~$100 | ~7 days |

Bizon estimates assume ~10× Modal T4 throughput per GPU × 4 GPUs = ~40× total speedup. Costs are electricity + infra estimates, not cloud rental. Total path from current state to frontier quality: **~€200 in compute, ~35 days wall-clock on Bizon**.

---

## Tokenizer Rebuilds

Each stage unlock requires a tokenizer rebuild to incorporate the new vocabulary distribution:

```bash
cd albert-moe-13
python scripts/train_tokenizer_v3.py --stages 1,2,3,4,5,6,7,8,9,10,11   # Stage 11 unlock
python scripts/train_tokenizer_v3.py --stages ...,12                      # Stage 12 unlock
```

The tokenizer maintains the 32k vocabulary size — it does not grow. New stages shift the subword distribution toward the new domains. The 10% chaos invariant is validated during every tokenizer rebuild and the build fails if the proportion is out of tolerance.

---

## Milestone Checklist

Before each stage unlock, verify:

- [ ] Loss gate confirmed (3-epoch rolling average, not single-epoch minimum)
- [ ] Architecture gate confirmed (check `albert_v3.0.config.json`)
- [ ] Build script executed cleanly (no truncated files, check byte counts)
- [ ] Chaos complement built and proportion verified
- [ ] Tokenizer rebuilt with new stages
- [ ] New tokenizer validated on held-out WikiText-2 sample
- [ ] `train_tokenizer_v3.py` chaos proportion check passes
- [ ] Modal volume updated with new corpus (or Bizon local storage mounted)
- [ ] `config.json` updated with new active stage list

---

*Prepared by RFI-IRFOS · 2026-05-14 · Graz, Austria*  
*Contact: contact@ternlang.com · Patent: A50296/2026*  
*See also: `docs/MYCELIAL_CORD_ARCHITECTURE.md`, `docs/EVOLUTION_EVIDENCE.md`*
