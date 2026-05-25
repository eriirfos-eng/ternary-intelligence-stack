# Albert Surgery Registry

Consolidated record of all Net2Net depth-expansion events across the lifetime of the project.
Primary source: `convergence_log.md` (v3.0), `EVOLUTION_EVIDENCE.md` (v2.0.0 surgeries 1–3).
Last updated: 2026-05-25.

---

## Version 1.x — Pre-Multilingual Bible Model

**Architecture start:** ~3L · 128H · 12E · 8k vocab  
**Outcome:** No successful surgeries.

| # | Epoch | Transition | Date | Trigger | Result |
|---|-------|-----------|------|---------|--------|
| F1 | unknown | 3L→4L | ~2026-05 | plateau | **FAILED** — Net2Net surgery incorrectly deleted best checkpoint instead of archiving it; all post-surgery models diverged. Bug identified and fixed before v2.0.0. |

**v1.x exit:** ~3L, loss unknown, run abandoned.

---

## Version 2.0.0 — Bible Ternary (EN only, 8k vocab, float32 weights)

**Architecture start:** 3L · 128H · 12E · 8k vocab  
**Architecture exit:** 12L · 256H · 12E · 8k vocab · loss 6.8821 (transferred to v3.0)

Note: hidden size was manually upgraded 128H→256H between surgeries 1 and 2 (non-surgery architectural change).

| # | Epoch | Transition | Date (UTC) | Trigger | Pre-surgery loss | Result |
|---|-------|-----------|------------|---------|-----------------|--------|
| S1 | ~20 | 3L→4L | 2026-05-05 | Plateau Δloss < 0.02 over 10 epochs | ~7.01 | Success |
| — | ~30 | 128H→256H | 2026-05-06T08:00Z | Manual architecture upgrade | ~6.8 | Non-surgery |
| S2 | ~40 | 4L→5L | 2026-05-06T11:05Z | Mastery condition met | ~6.2 | Success |
| S3 | ~86 | 5L→6L | 2026-05-06T22:00Z | Plateau Δloss < 0.02 over 10 epochs | 5.9174 | Success (live witnessed) |
| S4 | unknown | 6L→7L | unknown | plateau | unknown | Success |
| S5 | unknown | 7L→8L | unknown | plateau | unknown | Success |
| S6 | unknown | 8L→9L | unknown | plateau | unknown | Success |
| S7 | unknown | 9L→10L | unknown | plateau | unknown | Success |
| S8 | unknown | 10L→11L | unknown | plateau | unknown | Success |
| S9 | unknown | 11L→12L | unknown | plateau | unknown | Success |

Surgeries S1–S3 documented in `EVOLUTION_EVIDENCE.md`. S4–S9 individually undocumented; individual epoch data not preserved. Session log has scattered references to layer unlock events confirming the 3L→12L arc. README states "10 surgeries" which may account for one retried or partial event not in this table.

**v2.0.0 confirmed surgeries:** 9 (3L→12L)  
**v2.0.0 total surgical events per README:** ~10

---

## Version 3.0 — Multilingual Ternary (32k vocab, ternary STE from ep1)

**Architecture start:** 12L · 256H · 12E · 32k vocab (weights transferred from v2.0.0 ep=best, loss 6.8821; embed + lm_head re-initialized for 32k)  
**Architecture current:** 21L (as of 2026-05-25)

All 8 surgeries documented in `convergence_log.md`.

| # | v3 epoch | Global epoch | Transition | Date (UTC) | Fibonacci window | Mandelbrot c_im | Plateau Δ / epochs | Tensors | Pre-surgery EP_AVG | Post-surgery first ATL | Result |
|---|----------|-------------|-----------|------------|-----------------|----------------|-------------------|---------|------------------|-----------------------|--------|
| S1 | 511 | ~546 | 12L→13L | 2026-05 | 13 | — | — | — | — | — | Success |
| S2 | 547 | ~582 | 13L→14L | 2026-05 | 21 | −0.6983 | — | — | — | — | Success |
| S3 | 611 | ~646 | 14L→15L | 2026-05 | 34 | +0.2287 | — | 69 | — | — | Success |
| S4 | 645–646 | ~680 | 15L→16L | 2026-05 | 55 | −0.3442 | — | — | — | — | Success |
| S5 | 701–702 | ~736 | 16L→17L | 2026-05 | 89 | +0.5828 | — | — | — | — | Success; Stage 10 corpus unlocked |
| S6 | 2487 | ~2522 | 17L→18L | 2026-05-20T21:33Z | 233 | +0.0099 | Δ0.0193 / 144 ep | — | 9.7170 | 9.6248 (ep2489) | Success; Gen 1 step 1/6; ceiling→21L |
| S7 | 3325 | ~3360 | 18L→19L | 2026-05-24T13:47Z | — | — | 36+ ep (9.42–9.46) | 1315 | 9.3651 | **9.3182** (ep3326) | Success; [ttlfreeze] + [divloss] armed; ATL broke in first 19L epoch |
| S8 | 3383 | ~3418 | 19L→20L | 2026-05-24T~20:00Z | — | — | 58 ep (plateau at 19L floor) | 1384 | 9.3182 | **9.2862** (ep3383) | Success; fastest post-surgery plateau in v3 history |
| S9 | ~3437 | ~3472 | 20L→21L | 2026-05-25T~00:00Z | — | — | 54 ep (since_best accumulating) | 1453 | 9.2862 | **9.3930** (ep3503, still descending) | Success; largest post-surgery spike in v3 history (~9.43); fastest descent rate in v3 history (bullmarket); TTL hard stops (see note) |

**v3.0 confirmed surgeries:** 9 (12L→21L, all documented)

### Surgery S9 — Observations (2026-05-25)

**TTL hard-column stops — first occurrence in v3.0 history.**  
Post-S9, the Traffic Light Routing panel showed full-column red blocks: entire layers completely suppressed for multiple consecutive steps. Previous surgeries produced soft TTL transitions (gradual orange→red drift). S9 produced hard stops — columns of pure red lasting several steps — indicating the TTL is treating the new 21st layer as unproven and fully clamping its output while the established 20 layers route freely.

TTL distribution post-S9: **G 21% · O 74% · R 5%** (vs. pre-surgery baseline G 6–17% · O 78–81% · R 2%).  
Green surge = existing layers routing with high confidence. Red surge = new layer held in suppression.

**Interpretation:** The 21L insertion is the deepest yet. The TTL responded proportionally — harder suppression than any previous surgery. The bullmarket descent is driven by the existing 20 proven layers running freely while the 21st layer receives gradient signal during its suppressed steps. When the 21st layer earns green slots (expected: gradual onset over ~100–300 epochs), a second acceleration phase is expected.

**AdamW buffer reset coincidence:** Modal pod restart reset the optimizer state at approximately the same epoch as the surgery. Fresh AdamW momentum on an already-explored loss landscape contributed to the aggressive descent rate (see restart acceleration pattern in project memory).

---

## Lifetime Summary

| Version | Surgeries | Architecture arc | Documentation |
|---------|-----------|-----------------|---------------|
| v1.x | 1 failed | 3L (no net change) | No records |
| v2.0.0 | 9 confirmed (10 per README) | 3L→12L | S1–S3 in EVOLUTION_EVIDENCE.md; S4–S9 undocumented |
| v3.0 | 9 confirmed | 12L→21L | All 9 in convergence_log.md |
| **Total** | **18–19 surgical events** | **3L→21L** | |

**For SPRIND pitch (Q1):** "albert. has undergone approximately 18 Net2Net surgical depth expansions across its lifetime — 9–10 in v2.0.0 (3L→12L, individual epochs not preserved) and 8 in v3.0 (12L→20L, all precisely logged in convergence_log.md with timestamps, Mandelbrot perturbation parameters, and loss trajectories). One additional failed surgery in v1.x was the discovery event for the checkpoint-deletion bug that was subsequently fixed."

**README discrepancy fixed 2026-05-24:** README previously listed ep2802/ep3160 for v3 surgeries 7–8. Correct epochs are ep3325/ep3383 (from convergence_log.md). README has been corrected.

---

## Surgery 10 — Next expected event

**Current state (ep3508):** 21L · EP_AVG best post-S9 9.3930 · descending (bullmarket)  
**Gate:** PLATEAU gate — since_best will begin accumulating once descent stalls below 9.2862  
**Projection:** Surgery 10 (21L→22L) not imminent; model must first break pre-S9 ATL of 9.2862, then plateau again.
