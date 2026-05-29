# Ternary Intelligence Stack — Information Security Posture

**Status:** ISO/IEC 27001:2022 **aligned** (controls implemented + certification roadmap). **NOT yet certified.**
**Owner:** RFI-IRFOS — Research Focus Institute (regulated **not-for-profit**; ZVR 1015608684 · GISA 39261441 · Steuernummer 68 028/0989; ≥90 % of surplus reinvested per statute).
**Document version:** 0.1 · **Last reviewed:** 2026-05-29 · **Review cadence:** quarterly + on material change.

> **Honesty statement.** ISO/IEC 27001 is an *Information Security Management System* — an organizational program (defined scope, risk assessment, Statement of Applicability over Annex A's 93 controls, documented policies, internal audit, management review, then an external certification audit by an accredited body). We make **no claim of certification.** This document records the controls we have already implemented, the live risk register, and the roadmap to certification. Every "implemented" item below maps to specific code or infrastructure and is independently verifiable in the public repository.

---

## 1. Scope of the ISMS

The systems handling third-party data and access:

| Asset | Description | Hosting | Sensitive data |
|---|---|---|---|
| `ternlang-api` | REST/MCP API gateway, OpenAI-compatible shim, API-key issuance & billing webhook | Fly.io (Rust/axum) | Customer email, API keys, tiers, usage |
| Inference serving | albert. MoE-13 model serving | Modal (GPU), Fly | Prompt content (transient) |
| `lighthouse` (planned) | Internal CRM / user-management dashboard | Private, institution-only | Customer records, key lifecycle |
| KPI / dashboards | Telemetry, training observability | Fly `/data`, local | Operational metrics (no PII) |

**Out of scope (this revision):** the open-source compiler/VM/stdlib crates (no data processing), the local research training rig.

---

## 2. Security objectives (CIA)

- **Confidentiality** — customer credentials (API keys, email) and admin access protected; keys stored with least-privilege file permissions; secrets never in source.
- **Integrity** — billing events cryptographically verified before granting access; architecture/state changes reconciled to prevent silent corruption.
- **Availability** — durable persistence for customer records; graceful degradation; fail-*closed* on security-critical misconfiguration (a configuration error must deny, never silently grant).

---

## 3. Risk register (live)

Risks rated by impact × likelihood. Status reflects 2026-05-29.

| # | Risk | Impact | Status | Control |
|---|---|---|---|---|
| R1 | Forged Stripe webhook mints free paid keys | **Critical** | **MITIGATED** | Webhook now HMAC-SHA256 verified, constant-time, ±5 min replay window; **fails closed** if secret unset (was fail-open). |
| R2 | Known-default admin key → full key takeover | **Critical** | **MITIGATED** | `TERNLANG_ADMIN_KEY` is mandatory in production; service refuses to start without it (no `admin-dev` default outside explicit dev). |
| R3 | Customer key store on ephemeral FS → access lost on deploy | High | **MITIGATED (code) / pending deploy** | Store + DB default to a mounted volume (`/data`); dir auto-created. Requires Fly volume + secrets at deploy. |
| R4 | Paid inference endpoints unauthenticated/unmetered | Medium (revenue) | **OPEN — pending product decision** | Decide public-demo vs metered-per-key; enforce key + usage accounting if paid. |
| R5 | No user-management / visibility into who holds access | Medium | **PLANNED** | `lighthouse` internal CRM: authenticated dashboard, key lifecycle, payment→key linkage, revoke. |
| R6 | SQL injection via API query surface | High | **NOT PRESENT** | Query endpoints use a static `ALLOWED_QUERIES` allowlist; raw caller SQL is rejected. |
| R7 | Secrets committed to source | High | **CONTROLLED** | Secrets read from environment / Fly secrets manager; none hardcoded. |

---

## 4. Annex A control mapping (2022 themes)

Legend: ✅ implemented · ◯ partial · ⏳ planned.

### A.5 Organizational (selected)
- ✅ **A.5.15 Access control** — tiered API keys; separate admin-key middleware for `/admin`; provisional/authorized/unauthorized triadic auth model (`ternlang-auth`).
- ✅ **A.5.23 Cloud services security** — Fly.io secrets manager for credentials; per-service isolation.
- ⏳ **A.5.1 Policies / A.5.2 Roles / A.5.9 Asset inventory / A.5.24–.27 Incident management** — to be documented in the ISMS policy set.

### A.6 People
- ⏳ **A.6.3 Awareness / A.6.6 Confidentiality agreements** — small team (RFI-IRFOS); to be formalized.

### A.7 Physical
- ✅ **A.7.x** — inherited from Fly.io / Modal (SOC 2 / ISO-aligned providers); documented as supplier-managed.

### A.8 Technological
- ✅ **A.8.5 Secure authentication** — HMAC-verified billing; fail-closed secrets.
- ✅ **A.8.24 Use of cryptography** — HMAC-SHA256 (billing integrity); TLS in transit (Fly); constant-time verification on secrets.
- ✅ **A.8.9 Configuration management** — config reconciliation prevents silent state rollback (volume = source of truth on resume).
- ✅ **A.8.28 Secure coding** — Rust (memory-safe); input allowlisting; security headers (`X-Frame-Options: DENY`, `X-Content-Type-Options: nosniff`, `Referrer-Policy`); key file `chmod 0600`.
- ◯ **A.8.15 Logging / A.8.16 Monitoring** — operational telemetry + ntfy alerting exist; security-event audit logging to be formalized.
- ⏳ **A.8.12 Data leakage prevention / A.8.10 Information deletion** — retention + deletion policy for customer PII to be documented.

---

## 5. Data handling

- **PII collected:** customer email (from Stripe checkout), associated API key + tier + usage.
- **Storage:** persisted on a mounted volume, key file restricted to `0600`. No card data is ever handled by us — payments are processed entirely by **Stripe** (PCI-DSS Level 1); we receive only a signed event.
- **Transmission:** TLS only.
- **Retention/deletion:** to be formalized (target: delete on subscription end + request; GDPR Art. 17). RFI-IRFOS is EU-based (Graz, Austria) — GDPR applies.

---

## 6. Roadmap to certification

1. **Stage 0 — Foundation (in progress, 2026-05):** close critical technical risks (R1–R3 done), draft this posture, stand up `lighthouse` for access visibility.
2. **Stage 1 — ISMS documentation:** formal scope, risk-assessment methodology, **Statement of Applicability**, core policies (access control, cryptography, incident response, supplier, data retention).
3. **Stage 2 — Operate + internal audit:** run the ISMS for a defined period, evidence collection, internal audit, management review.
4. **Stage 3 — External certification audit** by an accredited body (Stage 1 documentation review → Stage 2 implementation audit).

**Pitch-safe phrasing:** "27001-aligned ISMS with critical controls implemented and a defined certification roadmap." Not "certified."

---

*This is a living document. Findings and fixes are tracked in the security task log; technical controls are verifiable in `ternlang-root/ternlang-api/src/main.rs` and infrastructure config.*
