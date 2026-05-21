# ISO/IEC 27001:2022 — Pre-Audit Evidence Package

**Entity:** RFI-IRFOS, Elisabethinergasse 25, 8020 Graz, Austria  
**Scope:** Ternary Intelligence Stack (TIS) — training pipeline, model weights, inference API, source code  
**Standard:** ISO/IEC 27001:2022 (incorporating Annex A, 93 controls)  
**Status:** Pre-audit evidence preparation (certificate not yet held)  
**Date:** 2026-05-21

---

## Documents in this package

| Document | Purpose | Status |
|----------|---------|--------|
| [risk_register.md](risk_register.md) | Asset inventory + threat/impact matrix | Draft |
| [soa.md](soa.md) | Statement of Applicability — all 93 Annex A controls | Draft |
| [access_control.md](access_control.md) | Per-service access matrix for all 6 team members | Draft |
| [incident_response.md](incident_response.md) | Runbooks for credential leak, weight exfiltration, API compromise | Draft |
| [data_classification.md](data_classification.md) | Four-tier classification policy for all TIS data assets | Draft |

---

## What this establishes for a reviewer

A complete ISO 27001 certification requires a third-party audit body. This package is not a substitute.
What it does demonstrate to an auditor, enterprise procurement team, or SPRIND evaluator:

- The team has performed a documented risk assessment
- All 93 Annex A controls have been considered with explicit include/exclude decisions
- Access to sensitive systems is controlled and reviewed
- There is a written incident response procedure
- Data assets are formally classified

This is the standard of evidence that separates a team that takes security seriously from one that does not.

---

## To complete for full certification readiness

- [ ] Management review and sign-off on risk register (ISMS owner)
- [ ] Populate actual team member names in `access_control.md`
- [ ] Run first internal audit against SoA
- [ ] Select a certification body (e.g. TÜV Austria, Bureau Veritas)
- [ ] Address all HIGH-priority gaps in `soa.md`
