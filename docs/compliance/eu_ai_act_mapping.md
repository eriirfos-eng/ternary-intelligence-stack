# EU AI Act Compliance Mapping

**Project:** Ternary Intelligence Stack (TIS) v1.3.6  
**Entity:** RFI-IRFOS, Elisabethinergasse 25, 8020 Graz, Austria  
**Patent Pending:** A50296/2026  
**Date:** 2026-05-07  
**Status:** General-Purpose AI System (not listed in Annex III high-risk categories)

---

## Overview

The EU AI Act (Regulation 2024/1689) entered into force on 2024-08-01. TIS is a general-purpose AI tool and development framework — not an end-use system in any Annex III high-risk category. This mapping documents how TIS architecture supports downstream EU AI Act compliance for operators who deploy TIS in regulated contexts.

---

## Risk Classification

| Classification | Basis |
|---------------|-------|
| **General-Purpose AI System (GPAI)** | TIS provides a foundation model + inference runtime usable across domains |
| **Not Annex III High-Risk** | TIS is a developer tool; downstream operators bear deployment classification responsibility |
| **Transparency obligations apply** | GPAI providers must publish technical documentation and model card under Art. 53 |

---

## Article-by-Article Mapping

### Chapter II — Prohibited Practices (Art. 5)

| Prohibition | TIS Status |
|-------------|-----------|
| Subliminal manipulation | **Not applicable** — TIS is a text inference system with no perceptual manipulation capability |
| Social scoring | **Not applicable** — no scoring or ranking of persons |
| Real-time biometric surveillance | **Not applicable** — no biometric capabilities |
| Emotion recognition | **Not applicable** |

TIS does not implement any prohibited practice.

---

### Chapter III — High-Risk AI Systems

TIS is not itself a high-risk system. The `TernAudit` module provides downstream operators with tooling to self-assess their TIS-based deployments:

| Tool | Function |
|------|---------|
| `trit_audit` | Binary habituation ratio + EU AI Act Art. 13/14 heuristic check |
| `llb_check` | Local-Only/Blacklist filesystem gate — prevents unsafe write paths |
| `llb_validate` | Pre-write validation with trit verdict output |
| TernAudit dashboard | Full audit trail with timestamp, query hash, trit-scored output |

---

### Art. 9 — Risk Management System

| Requirement | TIS Implementation |
|-------------|-------------------|
| Systematic risk identification | LLB (Local-Only Blacklist) gate: classifies every write operation into local/advisory/deny |
| Risk mitigation measures | Safety-gated expert routing in Albert-MoE-13 (experts 0–3 require minimum confidence threshold) |
| Residual risk documentation | `SECURITY.md`, `docs/standards/`, `docs/REPRODUCIBILITY.md` |
| Testing before deployment | `albert-test` binary — full inference evaluation pipeline |

---

### Art. 10 — Data and Data Governance

| Requirement | TIS Implementation |
|-------------|-------------------|
| Training data relevance | Multi-corpus pipeline (`data/corpus/`) with documented sources (Gutenberg, Wikipedia, Linux docs) |
| Bias detection | `ternlang-ml` sparsity analysis; per-expert routing telemetry monitors expert collapse |
| Data provenance | `scripts/download_corpus.py` — all sources documented with URLs and license info |
| Test data separation | `data/test_shards/` — held-out evaluation data separated from training corpus |

---

### Art. 11 — Technical Documentation

| Requirement | Document |
|-------------|---------|
| System description | `README.md`, `docs/architecture.md` |
| Training methodology | `docs/albert-moe-13/docs/architecture.md`, `EVOLUTION_EVIDENCE.md` |
| Performance metrics | `albert-moe-13/benchmarks/README.md`, `docs/validation_results.md` |
| Limitations | `models/README.md` (honest capability statement), `docs/albert_capability_reality_report.md` |
| Architecture specifications | `BET-ISA-SPEC.md`, `spec/standards/` |
| Whitepaper | IEEE LaTeX, DOI: [10.17605/OSF.IO/TZ7DC](https://doi.org/10.17605/OSF.IO/TZ7DC) |

---

### Art. 12 — Record-Keeping

| Requirement | TIS Implementation |
|-------------|-------------------|
| Automatic logging | `dashboard/training.log` — every batch, epoch, surgery, collapse event logged |
| Log format | Structured text: `Epoch N (Global G), Batch B: loss = X.XXXX` + TELE/ROUTE/GRAD telemetry lines |
| Checkpoint history | `models/registry/` — versioned model manifest with training state, evolution history |
| Immutability | Git history + `models/.gitignore` allowlist prevents silent log overwrite |

---

### Art. 13 — Transparency and Provision of Information

| Requirement | TIS Implementation |
|-------------|-------------------|
| AI system identity disclosure | API root (`GET /`) returns provider identity, version, model card link |
| Capability description | MCP server card at `/.well-known/mcp/server-card.json` |
| Intended purpose | `smithery.yaml` system prompt, `README.md §1` |
| Known limitations | `models/README.md`, capability reports in `docs/` |
| Human oversight interface | `albert-cli` HITL panel — all tool calls go through approve/deny before execution |

---

### Art. 14 — Human Oversight

| Requirement | TIS Implementation |
|-------------|-------------------|
| Human-in-the-loop capability | `albert-cli` TUI: HITL prompt for every tool call in `Prompt` permission mode |
| Override capability | Approve / Approve for session / Approve with changes / Deny — four-option HITL |
| Session-level delegation | "Approve for session" allows humans to delegate within a bounded scope |
| Monitoring | Live training dashboard (`albert-train`) with loss, sparsity, routing, gradient telemetry |
| Intervention capability | `albert-train` runs in a terminal the operator controls; `Ctrl+C` terminates immediately |

---

### Art. 15 — Accuracy, Robustness, and Cybersecurity

| Requirement | TIS Implementation |
|-------------|-------------------|
| Accuracy metrics | Per-epoch average loss + per-expert routing telemetry; `albert-test` inference evaluation |
| Robustness under input variation | Gating noise in MoE routing (`Tensor::rand(0.98, 1.02, ...)`) prevents routing lock-in |
| Resilience to adversarial inputs | LLB security gate; `LOSS_EXPLOSION_THRESHOLD` skips adversarial-magnitude batches |
| Gradient stability | Real gradient clipping (MAX_GRAD_NORM) + collapse detection + rollback |
| Cybersecurity | API key authentication (X-Ternlang-Key), admin key separation, rate-limited endpoints |

---

### Art. 53 — GPAI Model Obligations

| Obligation | TIS Status |
|-----------|-----------|
| Technical documentation | ✓ Whitepaper (DOI: 10.17605/OSF.IO/TZ7DC), architecture docs, BET-ISA-SPEC.md |
| Training data summary | ✓ `scripts/download_corpus.py`, `data/corpus/` manifest |
| Copyright compliance policy | ✓ All training data from Project Gutenberg (public domain) + open-license Wikipedia |
| Summary for downstream providers | ✓ `README.md`, `docs/GLOBAL-INDEX.md`, `smithery.yaml` |

---

## Ternary Architecture and AI Act Alignment

The ternary weight representation `{-1, 0, +1}` offers a structural compliance advantage over float32 models:

1. **Interpretability**: Ternary weights are human-readable (each weight is one of three states). The proportion of zero weights (sparsity) directly measures how much of the model is "active" for a given layer.

2. **Auditability**: The `.trit` format packs weights at 2 bits/weight — the entire model can be inspected or exported without floating-point precision issues.

3. **Determinism**: Ternary matmuls reduce to integer additions (no floating-point multiply), making inference on reference hardware fully deterministic — supporting reproducibility requirements.

4. **Verifiability**: `albert-test` runs inference against a fixed test set and computes loss — providing a reproducible performance certificate.

---

## Gaps and Roadmap

| Gap | Priority | Planned Resolution |
|-----|----------|--------------------|
| Formal conformity assessment | High | Target Q3 2026 with SPRIND support |
| Public training telemetry archive | High | `/benchmarks/training` endpoint (in progress) |
| GPU reproducibility suite | Medium | CUDA backend planned (see SPRIND blocker #2) |
| EU representative designation | Medium | RFI-IRFOS Graz is EU-based; no separate representative needed |

---

*For questions: contact@ternlang.com · Patent Pending A50296/2026 · Graz, Austria*
