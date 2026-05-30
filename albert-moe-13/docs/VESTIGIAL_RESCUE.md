# Vestigial Expert Rescue — flux-gated self-repair

*2026-05-30 · `moe-llm-core` · status: implemented, default OFF*

## The problem: weight-dead ≠ TLIGHT-dead

albert.'s mycelium module has always had autonomous self-repair: each epoch it
detects **dead experts** and issues `ResurrectionJob`s that copy a healthy
neighbour's weights (+ Gaussian noise) into the dead slot. This is his immune
system — not external forcing, an organ he already owns.

But the dead detector keys on a **single axis**: the TLIGHT traffic-light state.
An expert is "dead" only if it has been consistently **Red** (no routing /
gradient pressure) for `dead_threshold` epochs.

Under **LB-off training** (`--lb-weight=0.0`) + ternary-STE weight pruning, a
failure mode appears that this single axis *structurally cannot see*:

> an expert that is still **routed** (~uniform ~8% share, so TLIGHT Green/Orange,
> never Red) but whose MLP weights have been driven into the ternary-`0` state —
> i.e. **weight-dead while routing-alive.**

It contributes ~nothing to the residual stream, yet it never trips the Red
detector, so resurrection never fires. This was found live on 2026-05-30: the
dashboard "EXPERT ACTIVITY" panel (a weight-magnitude readout) showed SEM and CTX
at 0% while the `ROUTE` line stayed near-uniform. See **TERNARY_FINDINGS F9** and
the token-probe observation log FN266b/FN266c.

## The model: two independent ternary axes

Health is the **composition of two axes**, each bucketed to a balanced trit
against the population median (`classify_flux` in `mycelium.rs`):

```
substance ∈ {-1 starved, 0 thin, +1 dense}    — mean |weight| of the expert's MLP
flow      ∈ {-1 unrouted, 0 trickle, +1 busy}  — routing mass to the expert

healthy   ⟺ (+1, +1)
vestigial ⟺ (-1, flow ≥ 0)   ← routed but weight-starved — the missed state
dormant   ⟺ (-1, -1)         ← idle on both axes — viable seed-bank reserve
```

A binary alive/dead detector collapses "vestigial" and "dormant" into "alive"
(both are still, or recently, routed). Ternary keeps them distinct — and they
have **opposite correct responses**: resurrect the stalled one, leave the
recovering one alone.

## The afferent-nerve framing

- The **flux telemetry** is the missing *afferent nerve* — the sensor.
- **Resurrection** is the existing *effector* — the muscle, already organic.
- The gap was that the nerve wasn't connected to the muscle.

The fix is **not** a new load-balancing loss. That would be central planning —
forcing global balance, exactly the `no-forced-*` doctrine albert. is built
against. The fix is to let the self-repair he already has **see correctly**:
extend its trigger to vestigial, not just routing-dead. Healing the eye, not
seizing the hand.

## The non-negotiable guard: patience + recovery

CTX recovered 0% → 2% on its own. That proves the **dormant → germinate** path is
real. If resurrection fired the instant a slot read vestigial, it would *destroy*
a recovery in progress. So a slot is rescued only when nature has **demonstrably
stalled**, not merely paused. `vestigial_experts()` flags `(layer, expert)` only
when, over the last `vestigial_patience` epochs (default 12):

1. weight mass < **10%** of the layer's median expert mass — *every* epoch (starved),
2. TLIGHT is **not** all-Red and is **currently non-Red** (still routed — this is
   what separates vestigial from dead), and
3. mass is **flat or declining** — newest-half mean ≤ 1.05× oldest-half mean. A
   rising slot (like CTX) is *recovering* and is left untouched.

Median-relative starvation means a globally balanced (LB-on) regime flags nobody:
every expert sits near the median, so none falls below 10% of it. Zero false
positives by construction (unit-tested).

## Activation

**Default ON** (since 2026-05-30 — wired in completely so a plain `albert-train`
picks it up). The patience + recovery guards below are what make default-on safe:
only a demonstrably stalled slot is ever touched, and a recovering one is spared.

```bash
# default: vestigial rescue active, patience 12 epochs
albert-train ...

# tune the patience window
albert-train ... --vestigial-patience=15

# opt out entirely (no rebuild needed) — falls back to pure telemetry
albert-train ... --no-vestigial-rescue
```

Either way the per-epoch `FLUX` and `MYCELIUM … vestigial=N …` lines are emitted,
so the count is always observable. The startup banner records the live state:
`[mycelium] vestigial-rescue ON (patience=12 epochs) …`. The library default
(`MyceliumModule::new`) stays OFF — only the albert trainer opts in by default.

## Implementation map

| Piece | Location |
|-------|----------|
| Two-axis classifier + `FLUX` line | `mycelium.rs::classify_flux` / `FluxReport::log_line` |
| Per-(layer,expert) substance history | `MyceliumModule::record_substance` |
| Vestigial detector (patience + recovery guard) | `MyceliumModule::vestigial_experts` |
| Rescue wiring (gated) | `generate_resurrections` + `set_vestigial_rescue` |
| Report count (always emitted) | `MyceliumReport::vestigial_expert_count` |
| Substance feed each epoch | `train_bible.rs::per_layer_expert_abs_weight` |
| CLI flags | `--vestigial-rescue`, `--vestigial-patience=N` |
| Tests | `mycelium::flux_tests`, `mycelium::vestigial_rescue_tests` (5 total) |

## Open tuning questions

- **Patience (12) and starvation fraction (10%)** are first guesses. Once a real
  run logs `vestigial=N` over time we can calibrate against how long genuine
  recoveries (CTX-like) actually take vs. permanent starvation.
- **Seed selection.** Resurrection seeds from the most-Green expert in the layer.
  A vestigial expert is routed (Green/Orange) so it could itself rank high by
  TLIGHT yet be a poor seed (weight-starved). Worth eventually seeding by
  *substance* (highest weight-mass), not just TLIGHT-green count.
- **Granularity.** Detection/rescue is per-(layer,expert); the dashboard panel
  and `FLUX` line are per-expert-index aggregates. Both are intentionally kept.
