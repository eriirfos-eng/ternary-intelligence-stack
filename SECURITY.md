# Security Policy

## Reporting a Vulnerability

**Contact:** contact@ternlang.com  
**Response time:** We aim to acknowledge reports within 48 hours and
provide a resolution timeline within 7 days.

Please do **not** open a public GitHub issue for security vulnerabilities.
Use the email above. Include:

- A description of the vulnerability
- Steps to reproduce or a proof-of-concept
- Your assessment of severity and potential impact
- Whether you would like credit in the disclosure

We will coordinate a fix and public disclosure with you. We request a
**90-day coordinated disclosure window** before public disclosure, unless
the issue is already being actively exploited.

---

## Scope

The following components are in scope for security reports:

| Component | Description |
|-----------|-------------|
| `ternlang-api` | Public HTTP API at ternlang.com |
| `albert-cli` / `agent_albert_cli` | CLI tool distributed via crates.io |
| `moe-test` benchmark binary | Distributed via GitHub Releases |
| `install.sh` / `install.ps1` | Benchmark installer scripts |
| Training pipeline | `train_modal.py`, Modal GPU integration |
| KPI upload endpoint | `POST /api/kpi/upload/{filename}` |

The following are **out of scope**:

- Social engineering attacks
- Physical access attacks
- Denial of service against the Fly.io-hosted API (report to Fly.io)
- Vulnerabilities in third-party dependencies that have no active CVE

---

## EU AI Act Incident Reporting (Art. 53(2))

albert. is a General-Purpose AI model subject to Regulation (EU) 2024/1689.
RFI-IRFOS is committed to cooperating with national competent authorities
on incident reporting.

**What constitutes a serious incident for reporting purposes:**

Under Article 3(49) of the EU AI Act, a serious incident includes:

- Death or serious harm to a natural person caused by albert. outputs
- Significant disruption to critical infrastructure
- Infringement of fundamental rights at scale
- Outputs that constitute illegal content under EU law

**Reporting process:**

1. Internal assessment within 24 hours of discovery
2. Notification to the German national competent authority
   (Bundesnetzagentur / relevant AI supervisory body) within 72 hours
   where the incident meets the serious incident threshold
3. Cooperation with authority investigation
4. Post-incident report within 30 days

**Internal contact for incident escalation:**  
contact@ternlang.com (Lead Architect, Simeon Ari)

---

## Known Limitations Relevant to Safety

The following limitations of albert. are disclosed proactively:

1. **No safety fine-tuning.** albert. has received no RLHF, Constitutional
   AI, or instruction-following fine-tuning. It will not refuse harmful
   requests.

2. **Pre-fluency outputs.** At current training depth, outputs are often
   incoherent. Do not rely on albert. for factual information.

3. **No content filtering.** The model has no output filter. Deployers
   must implement their own content moderation if exposing albert. to
   end users.

4. **Research-only.** albert. is not certified for any safety-critical
   application.

---

## Supported Versions

| Version | Supported |
|---------|-----------|
| v3.0 (current) | Security fixes applied |
| v2.x and earlier | Not supported |

---

## Acknowledgements

We thank security researchers who report issues responsibly. Credit will
be given in release notes unless the reporter prefers anonymity.
