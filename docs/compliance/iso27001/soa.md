# Statement of Applicability (SoA)

**Standard:** ISO/IEC 27001:2022, Annex A  
**Entity:** RFI-IRFOS — Ternary Intelligence Stack  
**Version:** 1.0-draft  
**Date:** 2026-05-21  
**Owner:** Simeon (Lead Architect)

---

## How to read this document

For each of the 93 Annex A controls, we state:

- **Decision:** Included (I) · Partial (P) · Excluded (X)
- **Justification:** Why this decision was made for TIS
- **Evidence / implementation pointer:** Where to find the current control implementation

Excluded controls require a documented justification. A third-party auditor will challenge any exclusion without justification.

---

## A.5 — Organisational Controls (37 controls)

| Control | Title | Decision | Justification / Evidence |
|---------|-------|----------|--------------------------|
| A.5.1 | Policies for information security | **I** | `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`; this compliance package |
| A.5.2 | Information security roles and responsibilities | **I** | Simeon = ISMS owner; `MAINTAINERS.md` lists all roles |
| A.5.3 | Segregation of duties | **X** | 6-person startup; full segregation impractical. Mitigated by PR review requirement |
| A.5.4 | Management responsibilities | **I** | Lead Architect sign-off required for all privileged operations (see `access_control.md`) |
| A.5.5 | Contact with authorities | **P** | Contact via `contact@ternlang.com`; no formal regulatory liaison established yet |
| A.5.6 | Contact with special interest groups | **P** | SPRIND engagement; academic network via whitepaper DOI |
| A.5.7 | Threat intelligence | **P** | GitHub Dependabot alerts active; no formal threat intel subscription |
| A.5.8 | Information security in project management | **I** | Security considerations in every PR; `docs/SPRIND_PITCH_CHECKLIST.md` |
| A.5.9 | Inventory of information and other associated assets | **I** | `risk_register.md` asset inventory |
| A.5.10 | Acceptable use of information and other associated assets | **I** | `CODE_OF_CONDUCT.md`; commercial license terms |
| A.5.11 | Return of assets | **I** | Team member departure procedure in `incident_response.md` Scenario 5 |
| A.5.12 | Classification of information | **I** | `data_classification.md` — L1/L2/L3 three-tier policy |
| A.5.13 | Labelling of information | **P** | SPDX license headers in all source files; explicit L3 labelling in `risk_register.md`; no automated labelling tool |
| A.5.14 | Information transfer | **I** | SPORE protocol (`spore.rs`) for weight sharing; HTTPS for all API; GitHub for code |
| A.5.15 | Access control | **I** | `access_control.md` matrix; scoped PATs; no shared passwords |
| A.5.16 | Identity management | **I** | Per-person GitHub accounts; no shared login |
| A.5.17 | Authentication information | **I** | `llb-hook.sh` prevents credential commits; tokens scoped and rotated |
| A.5.18 | Access rights | **I** | Quarterly access review; Lead Architect approval for privileged operations |
| A.5.19 | Information security in supplier relationships | **P** | Modal ToS reviewed; Fly.io ToS reviewed; no formal supplier security requirements document |
| A.5.20 | Addressing information security within supplier agreements | **P** | Standard ToS accepted; custom DPA not in place for Modal/Fly.io |
| A.5.21 | Managing information security in the ICT supply chain | **P** | Cargo dependency audit via `cargo audit`; no formal supply chain risk programme |
| A.5.22 | Monitoring, review and change management of supplier services | **P** | Status pages monitored informally; no formal SLA tracking |
| A.5.23 | Information security for use of cloud services | **I** | Modal (GPU/storage) and Fly.io (API) documented in `risk_register.md`; access restricted |
| A.5.24 | Information security incident management planning and preparation | **I** | `incident_response.md` — five scenarios with step-by-step runbooks |
| A.5.25 | Assessment and decision on information security events | **I** | Severity levels P1–P4 defined in `incident_response.md` |
| A.5.26 | Response to information security incidents | **I** | Per-scenario response procedures in `incident_response.md` |
| A.5.27 | Learning from information security incidents | **P** | Lessons learned section in incident log template; no formal review cadence yet |
| A.5.28 | Collection of evidence | **P** | Log preservation noted in `incident_response.md`; no forensics toolkit in place |
| A.5.29 | Information security during disruption | **P** | Training checkpoints backed up locally; no formal BCP |
| A.5.30 | ICT readiness for business continuity | **P** | Modal fallback to different account documented in memory; no written BCP |
| A.5.31 | Legal, statutory, regulatory and contractual requirements | **I** | EU AI Act mapping (`eu_ai_act_mapping.md`); patent A50296/2026; GDPR (training data is public domain) |
| A.5.32 | Intellectual property rights | **I** | Patent pending A50296/2026; all code under commercial license; Gutenberg/Wikipedia corpus (public domain + CC-BY) |
| A.5.33 | Protection of records | **I** | `training.log`, git history, `convergence_log.md`, `session_log.md` — immutable via git |
| A.5.34 | Privacy and protection of PII | **P** | Training corpus is public domain text (no personal data). `DATA_PROVENANCE.md` documents sources |
| A.5.35 | Independent review of information security | **X** | No external audit conducted yet — this is the pre-audit preparation package |
| A.5.36 | Compliance with policies, rules and standards | **I** | This SoA + quarterly access review constitutes the compliance cycle |
| A.5.37 | Documented operating procedures | **I** | `CONTRIBUTING.md`, `QUICKSTART.md`, `incident_response.md`, `REPRODUCIBILITY.md` |

---

## A.6 — People Controls (8 controls)

| Control | Title | Decision | Justification / Evidence |
|---------|-------|----------|--------------------------|
| A.6.1 | Screening | **X** | Startup; no formal HR screening process. Mitigated by small trusted team |
| A.6.2 | Terms and conditions of employment | **P** | Commercial license governs IP ownership; no formal employment security policy |
| A.6.3 | Information security awareness, education and training | **I** | `llb-hook.sh` credential scanning as active training; team norms on credential hygiene |
| A.6.4 | Disciplinary process | **X** | Startup; no formal disciplinary policy. Excluded |
| A.6.5 | Responsibilities after termination or change of employment | **I** | Departure procedure in `incident_response.md` Scenario 5 |
| A.6.6 | Confidentiality or non-disclosure agreements | **P** | Commercial license requires attribution; no separate NDA template in place — **GAP** |
| A.6.7 | Remote working | **I** | Distributed team; HTTPS for all service access; no VPN required (all services use token auth) |
| A.6.8 | Information security event reporting | **I** | `SECURITY.md` — 48h acknowledgement SLA; `contact@ternlang.com`; no public issue for vulns |

---

## A.7 — Physical Controls (14 controls)

| Control | Title | Decision | Justification / Evidence |
|---------|-------|----------|--------------------------|
| A.7.1 | Physical security perimeters | **X** | Cloud-first; no owned datacenter. Modal and Fly.io responsible for physical security |
| A.7.2 | Physical entry | **X** | No owned datacenter |
| A.7.3 | Securing offices, rooms and facilities | **P** | Developer workstations in private offices; no server room |
| A.7.4 | Physical security monitoring | **X** | No owned datacenter |
| A.7.5 | Protecting against physical and environmental threats | **X** | Cloud-first; Modal/Fly.io responsible |
| A.7.6 | Working in secure areas | **X** | No secure areas; remote work only |
| A.7.7 | Clear desk and screen | **P** | Team norm; no formal policy |
| A.7.8 | Equipment siting and protection | **X** | No owned hardware other than developer workstations |
| A.7.9 | Security of assets off-premises | **P** | Developer laptops are the primary assets off-premises; full-disk encryption recommended — **GAP: not formally verified** |
| A.7.10 | Storage media | **P** | Model weights on Modal vol (cloud) + local backup; no physical media policy |
| A.7.11 | Supporting utilities | **X** | Cloud-first; provider responsibility |
| A.7.12 | Cabling security | **X** | No owned datacenter |
| A.7.13 | Equipment maintenance | **X** | Developer workstations; no special maintenance policy |
| A.7.14 | Secure disposal or re-use of equipment | **P** | No formal device disposal policy — **GAP** |

---

## A.8 — Technological Controls (34 controls)

| Control | Title | Decision | Justification / Evidence |
|---------|-------|----------|--------------------------|
| A.8.1 | User endpoint devices | **I** | Developer workstations; token-based access; `llb-hook.sh` on all dev machines |
| A.8.2 | Privileged access rights | **I** | Privileged operations require Lead Architect approval; documented in `access_control.md` |
| A.8.3 | Information access restriction | **I** | GitHub repo visibility; Modal volume restricted to owner account |
| A.8.4 | Access to source code | **I** | Private repos gated by GitHub org; `albert-spores` private |
| A.8.5 | Secure authentication | **I** | Scoped PATs per service; no password sharing; MFA on GitHub (verify status — **GAP if not enabled**) |
| A.8.6 | Capacity management | **P** | Modal budget tracked (`project_modal_training.md`); no formal capacity plan |
| A.8.7 | Protection against malware | **I** | `llb-hook.sh` blocks credential patterns pre-commit; `cargo audit` for dependencies |
| A.8.8 | Management of technical vulnerabilities | **P** | GitHub Dependabot active; no formal patch SLA |
| A.8.9 | Configuration management | **I** | Modal config.json auto-pushed before every train.remote(); `fly.toml` in version control |
| A.8.10 | Information deletion | **P** | No formal weight deletion policy; Modal volumes can be deleted manually — **GAP** |
| A.8.11 | Data masking | **X** | Training data is public domain; no PII masking required |
| A.8.12 | Data leakage prevention | **P** | `llb-hook.sh` credential scan; no full DLP tooling |
| A.8.13 | Information backup | **P** | Checkpoint backups before milestones; no automated backup schedule — **GAP** |
| A.8.14 | Redundancy of information processing facilities | **P** | Modal handles GPU redundancy; Fly.io has health checks; no multi-region for API |
| A.8.15 | Logging | **I** | `training.log` (every batch/epoch); Fly.io access logs; `session_log.md` |
| A.8.16 | Monitoring activities | **I** | Live training dashboard (`dashboard/run_server.py`); KPI pipeline; ntfy overnight monitoring |
| A.8.17 | Clock synchronisation | **I** | All timestamps from system time (UTC); ISO timestamp habit enforced |
| A.8.18 | Use of privileged utility programs | **I** | No special utility programs; standard Rust/Python toolchain |
| A.8.19 | Installation of software on operational systems | **P** | Modal Docker image pinned; Fly.io image tagged; no formal change approval for dev machine installs |
| A.8.20 | Networks security | **P** | All services over HTTPS; API key authentication; skybase server localhost-only |
| A.8.21 | Security of network services | **P** | Fly.io manages TLS for ternlang-api; no custom network security controls |
| A.8.22 | Segregation of networks | **X** | No owned network infrastructure |
| A.8.23 | Web filtering | **X** | Not applicable |
| A.8.24 | Use of cryptography | **P** | HTTPS enforced everywhere; no at-rest encryption for weights — **GAP: weights on Modal vol are not encrypted at rest** |
| A.8.25 | Secure development lifecycle | **I** | `CONTRIBUTING.md`; pre-commit hooks (`llb-hook.sh`); PR review process |
| A.8.26 | Application security requirements | **I** | `SECURITY.md` defines in-scope components; API key auth; LLB security gate |
| A.8.27 | Secure system architecture and engineering principles | **I** | Ternary architecture limits attack surface (integer-only inference); LLB gate; STE-based training |
| A.8.28 | Secure coding | **I** | Rust memory safety; `// SAFETY:` comments on all `unsafe` blocks; `cargo clippy` |
| A.8.29 | Security testing in development | **P** | `albert-test` binary for inference validation; no dedicated security test suite — **GAP** |
| A.8.30 | Outsourced development | **X** | No outsourced development (Claude Code is an AI assistant, not an outsourced contractor) |
| A.8.31 | Separation of development, test and production environments | **P** | Modal is dev/train; Fly.io is production API; not formally separated by policy |
| A.8.32 | Change management | **I** | Git history; `session_log.md`; PR-based change flow |
| A.8.33 | Test information | **I** | `data/test_shards/` separated from training data; held-out evaluation documented |
| A.8.34 | Protection of information systems during audit testing | **P** | No formal audit testing procedure yet |

---

## Gap Summary

The following controls are marked **GAP** and require action before certification:

| Control | Gap | Priority | Suggested action |
|---------|-----|----------|-----------------|
| A.6.6 | No NDA template for contributors/partners | Medium | Draft a one-page NDA template |
| A.7.9 | Developer laptop encryption not formally verified | Medium | Verify FDE enabled on all team machines; document |
| A.7.14 | No device disposal policy | Low | Add a paragraph to A.5.1 policy document |
| A.8.5 | MFA on GitHub not confirmed for all accounts | **High** | Verify and enforce MFA for all org members |
| A.8.10 | No formal weight deletion / retention policy | Medium | Add to `data_classification.md` retention schedule |
| A.8.13 | No automated checkpoint backup schedule | Medium | Add cron or post-epoch hook to back up to second location |
| A.8.24 | Model weights on Modal vol not encrypted at rest | Medium | Modal supports volume encryption — enable and document |
| A.8.29 | No security test suite for ternlang-api | Medium | Add at least one auth bypass test to CI |

---

*This SoA was prepared for pre-audit evidence. It is not a certification and does not constitute legal advice.*
