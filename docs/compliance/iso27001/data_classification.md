# Data Classification Policy

**Version:** 1.0-draft  
**Owner:** Simeon (Lead Architect, RFI-IRFOS)  
**Last reviewed:** 2026-05-21

---

## Classification Tiers

| Level | Label | Description | Handling |
|-------|-------|-------------|---------|
| **L3** | Confidential | Disclosure would cause material harm to RFI-IRFOS | Encrypted at rest; access logged; no public commit |
| **L2** | Internal | Not for public release; low harm if disclosed | Not committed to public repos; shared within team |
| **L1** | Public | Intentionally published; no restrictions | May appear in public GitHub, whitepaper, crates.io |

---

## Asset Classification Table

### Model Weights and Checkpoints

| Asset | Level | Rationale |
|-------|-------|-----------|
| albert. checkpoint files (`.pt`, `.safetensors`) | **L3 Confidential** | Commercial IP; patent pending A50296/2026 |
| SPORE weight deltas (`albert-spores`) | **L3 Confidential** | Federated sharing protocol; private repo |
| Quantised `.trit` binary weights (release-grade) | **L1 Public** | Intended for public benchmark download |
| Benchmark weight snapshots in `benchmarks/` | **L1 Public** | Publicly archived benchmark evidence |

### Training Data

| Asset | Level | Rationale |
|-------|-------|-----------|
| `albert_full_*.csv` (full corpus, ~100k rows) | **L2 Internal** | Curated and pre-processed; competitive advantage |
| Raw source data (Gutenberg, Wikipedia mirrors) | **L1 Public** | Publicly available; no restrictions |
| Tokenized shards (`data/train_shards/`) | **L2 Internal** | Pre-processed; competitive advantage |
| Test shards (`data/test_shards/`) | **L2 Internal** | Held-out eval set; publish only aggregated results |
| Chaos layer samples (`data/chaos/`) | **L2 Internal** | Adversarial; publish methodology only |

### Source Code

| Asset | Level | Rationale |
|-------|-------|-----------|
| `ternlang-root` (public GitHub) | **L1 Public** | Open source core |
| `albert-moe-13` (public GitHub) | **L1 Public** | Open source model |
| `albert-spores` (private GitHub) | **L3 Confidential** | Federated protocol, unreleased |
| Training scripts (`train_modal.py`) | **L2 Internal** | Infrastructure detail; not public |
| Infrastructure config (`fly.toml`, Modal configs) | **L2 Internal** | Contains structural detail; not credentials |

### Credentials and Secrets

| Asset | Level | Rationale |
|-------|-------|-----------|
| GitHub PAT | **L3 Confidential** | Full repo write access |
| Modal API token | **L3 Confidential** | GPU billing; weight storage |
| HuggingFace WRITE token | **L3 Confidential** | Model publication rights |
| crates.io API token | **L3 Confidential** | Package publication |
| OpenVSX PAT | **L3 Confidential** | Extension publication |
| Fly.io API token | **L3 Confidential** | Production service control |
| API admin key (ternlang-api) | **L3 Confidential** | All-endpoint access |
| API read key (ternlang-api) | **L2 Internal** | Limited to GET endpoints |

### IP and Legal

| Asset | Level | Rationale |
|-------|-------|-----------|
| Patent filing A50296/2026 (draft) | **L3 Confidential** | Pre-grant; public disclosure voids novelty |
| Published whitepaper (DOI: 10.17605/OSF.IO/TZ7DC) | **L1 Public** | Intentionally public |
| SPRIND pitch materials | **L2 Internal** | Pre-submission; competitive |

### Telemetry and Logs

| Asset | Level | Rationale |
|-------|-------|-----------|
| `training.log` | **L2 Internal** | Contains architectural detail and weight geometry clues |
| KPI pipeline data | **L2 Internal** | Performance data; not public until benchmarked |
| Session logs (`session_log.md`) | **L2 Internal** | Internal development record |
| Public benchmark results (`BENCHMARKS.md`) | **L1 Public** | Intentionally published |

---

## Handling Rules by Level

### L3 Confidential
- Never committed to any public or private repository unencrypted
- Never shared via unencrypted channel (no email plaintext; use end-to-end encrypted messaging)
- Access restricted to team members with explicit need
- Any suspected exposure triggers incident response immediately (see `incident_response.md`)
- Credentials rotated at minimum annually and on any team member departure

### L2 Internal
- May be committed to **private** repositories
- Not published to GitHub public, blog, or social media
- May be shared within team without special controls
- Review before any partner or third-party share

### L1 Public
- No restrictions; freely shareable
- Verify classification before publishing — downgrade is not reversible once public
