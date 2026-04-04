# Ternlang Examples

A growing collection of `.tern` programs demonstrating real-world decision logic in balanced ternary.

Every example follows the same pattern: three states (`-1` / `0` / `+1`) map to something concrete. The magic is always in the middle value — the state that binary systems are forced to throw away.

---

## Quick Reference

| State | Trit | Also called | Meaning |
|-------|------|-------------|---------|
| Reject | `-1` | `conflict()` | Clear negative signal. Do not proceed. |
| Hold | `0` | `hold()` | Not enough data. Wait or ask for more. |
| Affirm | `+1` | `truth()` | Clear positive signal. Proceed. |

---

## Examples

### Fundamentals

| # | File | What it shows |
|---|------|---------------|
| 01 | [hello_trit.tern](01_hello_trit.tern) | All three trit values, `invert()`, `consensus()` — start here |
| 02 | [decision_gate.tern](02_decision_gate.tern) | Safety as a hard gate: safety conflict blocks everything else |

### Real-World Decisions

| # | File | Domain | Key pattern |
|---|------|--------|-------------|
| 03 | [rocket_launch.tern](03_rocket_launch.tern) | Aerospace | Go / No-Go / Hold; range safety as absolute veto |
| 04 | [sensor_fusion.tern](04_sensor_fusion.tern) | Autonomous vehicles | Four-sensor fusion; any obstacle signal dominates |
| 05 | [medical_triage.tern](05_medical_triage.tern) | Healthcare | ER triage; consciousness as hard gate |
| 06 | [git_merge.tern](06_git_merge.tern) | DevOps | CI as hard gate; auto-merge / review / block |
| 07 | [spam_filter.tern](07_spam_filter.tern) | Email | Quarantine ≠ spam folder; hold is an active routing label |
| 08 | [evidence_collector.tern](08_evidence_collector.tern) | AI agents | Low data density detection; formal "I need more" signal |

### Computer Science & Systems

| # | File | Domain | Key pattern |
|---|------|--------|-------------|
| 09 | [risc_fetch_decode.tern](09_risc_fetch_decode.tern) | CPU / Systems | Fetch-decode-execute pipeline; stall = hold; inspired by Brandon Smith's 9-trit RISC simulator |
| 13 | [owlet_bridge.tern](13_owlet_bridge.tern) | Programming languages | Ternary S-expression eval loop; suspended eval = hold; inspired by the Owlet interpreter |
| 14 | [circuit_breaker.tern](14_circuit_breaker.tern) | Microservices | HALF-OPEN state is natively trit = 0; no special-casing needed |

### Human Decisions & Civic Systems

| # | File | Domain | Key pattern |
|---|------|--------|-------------|
| 10 | [confidence_escalator.tern](10_confidence_escalator.tern) | AI agents | Self-assessment before answering; escalate when uncertain |
| 11 | [form_validator.tern](11_form_validator.tern) | UX / Web | Empty ≠ invalid; ternary UX avoids hostile "required field" errors |
| 12 | [vote_aggregator.tern](12_vote_aggregator.tern) | Civic / Governance | Abstain is signal, not silence; quorum detection |
| 15 | [loan_underwriter.tern](15_loan_underwriter.tern) | Finance | Approve / refer to human / decline; automated humility |
| 16 | [content_moderation.tern](16_content_moderation.tern) | Trust & Safety | Allow / review / remove; human in the loop for the hold zone |
| 18 | [treaty_negotiation.tern](18_treaty_negotiation.tern) | Diplomacy | Veto ≠ reserve; failed ratification vs. procedural hold |
| 20 | [hiring_pipeline.tern](20_hiring_pipeline.tern) | HR / Recruiting | Hold bucket is the most valuable stage; references as soft gate |

### Infrastructure & Engineering

| # | File | Domain | Key pattern |
|---|------|--------|-------------|
| 17 | [job_scheduler.tern](17_job_scheduler.tern) | Systems | Defer ≠ cancel; resource pressure produces hold, not cancellation |
| 19 | [cache_invalidation.tern](19_cache_invalidation.tern) | Web / CDN | Stale-while-revalidate is natively trit = 0; named properly at last |

---

## Patterns Demonstrated

### Hard Gate
A signal so important that its negative value overrides everything else:
```
match critical_signal {
    -1 => { return conflict(); }   // veto — no further evaluation needed
     0 => { ... }
     1 => { ... }
}
```
Used in: rocket launch (range safety), medical triage (consciousness), spam filter (blocklist), loan underwriting (DTI ratio).

### Density Check
When fewer than N of M signals are decisive, request more data instead of guessing:
```
// if not enough decisive signals → return hold()
// "I don't know yet — here's what I need"
```
Used in: `08_evidence_collector.tern`, `10_confidence_escalator.tern`, `12_vote_aggregator.tern`.

### Cascading Consensus
Chain `consensus(a, b)` calls to aggregate multiple signals:
```
let ab:  trit = consensus(signal_a, signal_b);
let abc: trit = consensus(ab, signal_c);
```
Used in: nearly every example. The workhorse of ternary aggregation.

### Hold as Routing Label
`hold()` is not "undecided" — it is a first-class output value that tells the caller what to do next:
- Spam filter: quarantine folder
- Circuit breaker: probe mode
- Content moderation: human review queue
- Loan underwriting: human underwriter queue
- Form validator: show "required" hint, not error

---

## Contributing

New examples follow the naming convention `NN_snake_case_name.tern`.

Every example should:
1. Have a header comment explaining the real-world scenario
2. Demonstrate what binary systems get wrong (and why ternary fixes it)
3. Have a concrete scenario at the end that can be traced through manually
4. Return a meaningful trit via a top-level `match` block

---

## Attribution

- `09_risc_fetch_decode.tern` — conceptually informed by Brandon Smith's Python 9-trit RISC simulator
- `13_owlet_bridge.tern` — conceptually informed by the Owlet S-expression ternary interpreter (Node.js)
- Balanced ternary mathematical foundations: Knuth (1997), *The Art of Computer Programming*
- Physical ternary precedent: Setun computer, Moscow State University, 1958
- BitNet b1.58 ternary neural network weights: Ma et al. (2024)
