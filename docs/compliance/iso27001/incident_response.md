# Incident Response Runbook

**Version:** 1.0-draft  
**Owner:** Simeon (Lead Architect, RFI-IRFOS)  
**Contact:** contact@ternlang.com  
**Last reviewed:** 2026-05-21

---

## Severity Levels

| Sev | Definition | Response time |
|-----|-----------|---------------|
| **P1 — Critical** | Active exploitation or confirmed data breach | Immediate (within 30 min) |
| **P2 — High** | Credential exposed; no confirmed misuse yet | Within 2 hours |
| **P3 — Medium** | Suspected exposure; investigation required | Within 24 hours |
| **P4 — Low** | Policy violation; no immediate risk | Within 7 days |

---

## Scenario 1 — Credential Leaked in Public Commit

**Triggers:** `llb-hook.sh` fires and blocks commit; team member notices a token in a diff; GitHub secret scanning alert

**Severity:** P2 (P1 if already exploited)

**Step-by-step response:**

1. **Immediately revoke the exposed token** at the issuing service:
   - GitHub PAT: Settings → Developer settings → Personal access tokens → Revoke
   - Modal: `modal token revoke` or Modal dashboard → API tokens
   - HuggingFace: Settings → Access tokens → Delete
   - crates.io: Account settings → API tokens → Revoke
   - Fly.io: `flyctl auth logout` then `flyctl tokens revoke <id>`

2. **Audit access logs** for the time window between the commit and revocation:
   - GitHub: Settings → Security log
   - Modal: Dashboard → Usage
   - Fly.io: `flyctl logs --app ternlang-api`

3. **Remove the token from git history** (if committed):
   ```bash
   git filter-repo --path <file> --invert-paths   # or BFG Repo-Cleaner
   git push --force-with-lease origin main
   ```

4. **Issue a new token** with the minimum required scope. Update all references (CI, local env, Modal config).

5. **Document the incident**: date/time, what was exposed, duration of exposure, access log findings, remediation taken. File in `docs/compliance/iso27001/incidents/YYYY-MM-DD_credential_leak.md`.

6. If any external service was accessed with the exposed credential, escalate to P1 and consider notifying affected parties.

---

## Scenario 2 — Model Weights Exfiltrated

**Triggers:** Unexpected Modal storage usage spike; unknown download activity; team member reports suspicious access

**Severity:** P1

**Step-by-step response:**

1. **Immediately revoke all Modal tokens** and regenerate. This terminates any active session.

2. **Assess scope:**
   - Check Modal volume access logs for download events (Dashboard → Volumes → `albert-vol` → Access log)
   - Identify if weights were copied to an external storage or downloaded

3. **Preserve evidence:** Do not delete the Modal volume until access logs are exported. Export logs to `docs/compliance/iso27001/incidents/`.

4. **Isolate the volume:** Remove public access if any; set Modal function permissions to owner-only.

5. **Legal consideration:** If exfiltration is confirmed:
   - The weights are covered by Patent Pending A50296/2026
   - Contact patent attorney to assess IP infringement exposure
   - If misuse is commercial, notify counsel

6. **Internal review:** Determine how access was obtained (leaked token, compromised account, insider). Apply root cause fix before re-granting access.

7. Document fully. If GDPR personal data was processed in the training corpus, assess whether a data breach notification is required (unlikely for TIS corpus which is public domain, but verify).

---

## Scenario 3 — API Key Exposed via Public Code or Log

**Triggers:** Plaintext API key (ternlang-api `X-Ternlang-Key`) visible in a public GitHub issue, log file, or screenshot

**Severity:** P2

**Step-by-step response:**

1. **Rotate the API key immediately:**
   ```bash
   # In ternlang-api configuration — regenerate the key and redeploy
   fly secrets set TERNLANG_API_KEY=<new-key> --app ternlang-api
   ```

2. **Audit API access logs** for the exposure window: `flyctl logs --app ternlang-api | grep <old-key-prefix>`

3. **Remove from public source:** Edit the GitHub issue/comment/file. Note: GitHub edit history retains the old content — open a support ticket to purge if the key was highly sensitive.

4. **Notify affected API consumers** if the key was shared with external parties (e.g., SPRIND demo recipients).

5. **Review rate limits and quotas** to detect any abuse during the exposure window.

---

## Scenario 4 — ternlang-api Service Compromised (Fly.io)

**Triggers:** Unexpected process execution; anomalous outbound connections; modified response behaviour

**Severity:** P1

**Step-by-step response:**

1. **Take the service offline immediately:**
   ```bash
   fly scale count 0 --app ternlang-api
   ```

2. **Capture machine state before shutdown** for forensic analysis:
   ```bash
   flyctl ssh console --app ternlang-api   # if still accessible
   # capture: /proc/net/tcp, running processes, /tmp contents
   ```

3. **Assess the attack vector:**
   - Review Fly.io access logs
   - Review recent deploy history: `flyctl releases list --app ternlang-api`
   - Check if Fly.io API token was compromised (Scenario 1 overlap)

4. **Redeploy from a clean image:**
   ```bash
   git checkout <last-known-good-tag>
   fly deploy --app ternlang-api
   ```

5. **Rotate all secrets** the service had access to (admin key, any connected tokens).

6. **Document and conduct post-mortem.** If user data was at risk, assess GDPR notification obligation.

---

## Scenario 5 — Team Member Departure (Access Revocation)

**Not an incident but a mandatory access control procedure.**

On any team member departure, within 24 hours:

- [ ] Revoke their GitHub org membership and all repo access
- [ ] Rotate any shared credentials they had access to (Modal, HF, Fly.io)
- [ ] Review git history for any credentials committed by them
- [ ] Transfer or archive any private repos they owned
- [ ] Revoke Fly.io collaborator access
- [ ] Document in `docs/compliance/iso27001/access_control.md` with date

---

## Incident Log Location

All incident records: `docs/compliance/iso27001/incidents/YYYY-MM-DD_<type>.md`  
Template fields: date · discovered by · severity · assets affected · timeline · root cause · remediation · lessons learned
