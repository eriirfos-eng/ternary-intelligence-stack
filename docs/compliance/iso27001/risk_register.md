# Risk Register

**Version:** 1.0-draft  
**Owner:** Simeon (Lead Architect, RFI-IRFOS)  
**Last reviewed:** 2026-05-21  
**Review cycle:** Quarterly

---

## Asset Inventory

| ID | Asset | Location | Owner | Classification |
|----|-------|----------|-------|----------------|
| A01 | albert. model weights (checkpoints) | Modal `albert-vol` volume + local backups | Simeon | Confidential |
| A02 | Training corpus (`albert_full_*.csv`) | `~/Desktop/downloads/`, Modal volume | Simeon | Internal |
| A03 | GitHub Personal Access Token | Local env, team secrets | Simeon | Confidential |
| A04 | Modal API token | Local env, CI | Simeon | Confidential |
| A05 | HuggingFace WRITE token (rfi-irfos org) | Local env | Simeon | Confidential |
| A06 | crates.io API token | Local env | Simeon | Confidential |
| A07 | OpenVSX PAT | Local env | Simeon | Confidential |
| A08 | Fly.io API token + ternlang-api service | Fly.io cloud | Simeon | Confidential |
| A09 | Source code (all repos) | GitHub `rfi-irfos` org | Team | Internal |
| A10 | Patent documentation (A50296/2026) | Local + filing system | Simeon | Confidential |
| A11 | Whitepaper (pre-publication drafts) | `ternlang-root/docs/whitepaper/` | Team | Internal |
| A12 | Training pipeline (`train_modal.py`, `albert-train`) | GitHub + Modal | Team | Internal |
| A13 | KPI HTTP pipeline | `skybase_server.py`, Fly.io | Simeon | Internal |
| A14 | Smithery MCP server configuration | `smithery.yaml`, Fly.io | Simeon | Internal |
| A15 | Training telemetry logs (`training.log`) | Local `~/.albert/`, Modal | Team | Internal |
| A16 | SPORE weight sharing keys (`spore.rs`) | `albert-spores` private repo | Simeon | Confidential |

---

## Threat Matrix

**Likelihood:** 1 (rare) · 2 (unlikely) · 3 (possible) · 4 (likely) · 5 (almost certain)  
**Impact:** 1 (negligible) · 2 (minor) · 3 (moderate) · 4 (major) · 5 (catastrophic)  
**Risk = Likelihood × Impact**

| ID | Threat | Affected Assets | Likelihood | Impact | Risk | Treatment |
|----|--------|-----------------|------------|--------|------|-----------|
| R01 | API credential leaked in public commit | A03–A08 | 3 | 4 | **12** | `llb-hook.sh` pre-commit scan; token scoping |
| R02 | Unauthorised weight download from Modal vol | A01 | 2 | 5 | **10** | Modal access restricted to owner account |
| R03 | GitHub repo accidentally set public (private content) | A09, A10, A16 | 2 | 4 | **8** | `albert-spores` private; org visibility review |
| R04 | Modal account compromised (stolen token) | A01, A04, A12 | 2 | 5 | **10** | Rotate tokens on any suspected exposure |
| R05 | Training data poisoning (adversarial CSV injection) | A02, A01 | 2 | 4 | **8** | `LOSS_EXPLOSION_THRESHOLD` batch skip; checksum verify |
| R06 | Fly.io service disruption (ternlang-api) | A08, A13, A14 | 3 | 3 | **9** | Health checks; restart policy in `fly.toml` |
| R07 | Insider credential misuse (6-person team) | A03–A08 | 2 | 4 | **8** | Scoped tokens per person; principle of least privilege |
| R08 | Patent draft leaked before filing | A10 | 2 | 5 | **10** | Confidential classification; no public commit |
| R09 | Private repo accidentally exposed via dependency | A09, A16 | 2 | 3 | **6** | No private imports in public Cargo.toml |
| R10 | Loss of only training checkpoint (Modal vol deleted) | A01 | 2 | 5 | **10** | Periodic local backup before epoch milestones |
| R11 | Skybase server exploited via open port | A13, A15 | 2 | 3 | **6** | Bind to localhost only; auth token required |
| R12 | LR/training configuration drift silently corrupts run | A01, A12 | 3 | 3 | **9** | Modal config.json auto-push before every train.remote() |

---

## Risk Treatment Summary

| Priority | Risk IDs | Action |
|----------|----------|--------|
| **HIGH (≥10)** | R01, R02, R04, R08, R10 | Immediate controls required; see individual treatments |
| **MEDIUM (6–9)** | R03, R05, R06, R07, R09, R11, R12 | Controls in place or planned; review quarterly |
| **LOW (<6)** | None at present | Accept and monitor |

---

## Residual Risk Statement

After applying the treatments listed above, all HIGH risks are reduced to MEDIUM or LOW. The residual risk profile is accepted by the ISMS owner (Simeon, Lead Architect). Formal re-assessment required after any: new team member, new external service integration, or checkpoint publication.
