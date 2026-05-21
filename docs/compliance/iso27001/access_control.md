# Access Control Review

**Version:** 1.0-draft  
**Owner:** Simeon (Lead Architect, RFI-IRFOS)  
**Last reviewed:** 2026-05-21  
**Review cycle:** Quarterly or on team composition change

---

## Principle of Least Privilege

Each team member holds only the minimum access required to perform their role. Shared credentials are avoided; per-person scoped tokens are preferred where the service supports it.

---

## Service Access Matrix

Fill in team member names/roles in the header row. Use: **A** = Admin · **W** = Write · **R** = Read · **—** = No access

| Service / Asset | Lead Architect | [Team Member 2] | [Team Member 3] | [Team Member 4] | [Team Member 5] | [Team Member 6] | claude@ternlang.com |
|----------------|---------------|-----------------|-----------------|-----------------|-----------------|-----------------|---------------------|
| GitHub org admin | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| `ternary-intelligence-stack` repo | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | **W** (PAT) |
| `albert-spores` (private) | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| `albert-moe-13` repo | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | **W** (PAT) |
| Modal.com (GPU training) | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| Modal `albert-vol` volume | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| HuggingFace `rfi-irfos` org | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| crates.io (publishing) | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| Fly.io `ternlang-api` app | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| OpenVSX (rfi-irfos publisher) | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| ternlang-api admin key | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| ternlang-api read key | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | **R** |
| Smithery MCP registry | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | — |
| KPI dashboard (`localhost:8888`) | **A** | _fill_ | _fill_ | _fill_ | _fill_ | _fill_ | **R** |
| Patent filing A50296/2026 | **A** | — | — | — | — | — | — |

---

## Token Inventory

Document each issued token here. Rotate on departure or suspected exposure.

| Token | Service | Scope | Issued to | Issued date | Expiry | Last rotated |
|-------|---------|-------|-----------|-------------|--------|--------------|
| PAT-001 | GitHub | repo, workflow | Simeon | 2026-01-xx | No expiry | 2026-05-xx |
| PAT-002 | GitHub | read-only | claude@ternlang.com | 2026-04-xx | No expiry | — |
| MOD-001 | Modal.com | full account | Simeon | — | No expiry | — |
| HF-001 | HuggingFace | WRITE (rfi-irfos) | Simeon | — | No expiry | — |
| CIO-001 | crates.io | publish | Simeon | — | No expiry | — |
| VSX-001 | OpenVSX | publish (rfi-irfos) | Simeon | — | No expiry | — |
| FLY-001 | Fly.io | deploy + secrets | Simeon | — | No expiry | — |

*Fill in actual dates from each service's token management page.*

---

## Access Review Procedure

Quarterly review: verify that every entry in the matrix is still current. Remove access for:
- Anyone no longer on the team
- Anyone whose role no longer requires a given permission
- Any token older than 12 months (rotate even if not suspected compromised)

Record completion of each review with a date in the commit message: `docs: access control review YYYY-MM`.

---

## Privileged Access Controls

The following operations require Lead Architect sign-off (no delegation):

- Publishing to crates.io
- Deploying to Fly.io production (`fly deploy --app ternlang-api`)
- Modifying Modal volume contents (`albert-vol`)
- Creating or revoking GitHub org tokens
- Any change to the Patent A50296/2026 filing

---

## Claude Code Access Notes

`claude@ternlang.com` holds a scoped GitHub PAT with write access to the main development repos. This token:
- Has no admin rights (cannot change org settings, delete repos, or manage billing)
- Is used exclusively for code commits and file edits in the development workflow
- Should be rotated whenever there is a session handoff or if Simeon suspects the token has been exposed
- Does NOT have access to Modal, HF publish, crates.io, or Fly.io
