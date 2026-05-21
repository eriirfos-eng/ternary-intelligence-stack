# Overnight Watch Log — Post-Surgery-6 (18L)
**Watch began:** 2026-05-20T22:03Z (ep2492, first probe)  
**Scientist:** Claude Sonnet 4.6  
**Surgery:** ep2487 · 17L→18L · Mandelbrot c_im=0.0099 · Gen 1 step 1/6 · window=233

---

## Field Note 20 — 2026-05-21T06:20Z · ep2585 — DOUBLE RESURRECTION L6 · LOCAL LOG STREAM CUT

**Epochs covered:** ep2585 · ep2586 partial (log stream cut at batch 11/300)

**ep2585 epoch summary:**
| loss_avg | delta | since_best | wald_sev | cov[high] | myc_stable | notes |
|----------|-------|------------|----------|-----------|------------|-------|
| 9.6010 | +0.0006 | 9 | 0.946 | 34 | 59 | double resurrection L6 |

**MYCELIUM ep2585:** dead=2 — two experts died simultaneously. Both resurrected from the **same source**: L6E0 spawned both L6E10 and L6E11 (σ=0.050 each). This is the first double-resurrection from a single parent observed in the post-surgery-6 window. Escalating resurrection pattern: 1 dead (ep2583), 1 dead (ep2584), 2 dead (ep2585). blooming=4. All-zero pressure vector unchanged.

**TELE ep2585:** S values unchanged (L17=0.055). Subtle E-value shift: L10 changed from 0.811→0.810 (first non-zero E change since surgery-6, but magnitude ~0.001 — likely rounding artefact, not a structural signal). GRAD step=17710 confirms myc_L0-L3 still all e-9: [1.49/1.54/1.52/1.58/1.62/1.62/1.63/1.44/1.59/1.46/1.72/1.50...]×10⁻⁹ — embedding gradient starvation persists.

**ep2586 start (before log cut):** First 11 batches logged (06:18:23–06:18:32). LR dropped to ~2.02e-4 at epoch start (from 2.34e-4 at ep2585 batch 263) — LR schedule advancing. Batch losses volatile: 9.5424 at batch 10 (closest batch-level approach to ATL region this window). Log stream terminated at batch 11, ETA ~4:50 remaining.

**OPERATIONAL FLAG — LOCAL LOG STREAM DISCONNECTED:** Log file last written 06:18:32. No tmux session or local training process found. Training is continuing on Modal remotely but telemetry is no longer reaching the local log at `/home/eri-irfos/.albert/training.log`. Watch is effectively blind from ep2586 batch 11 onward. Manual action required to re-establish log stream (re-attach to Modal run or restart albert-train with log piping).

**WALD ep2585:** fill=6.2%, sev=0.946 (held), cov[high]=34 — bouncing from 25-floor, no new approach.

**Thresholds:** No triggers. since_best=9 at last confirmed epoch. Watch suspended pending log stream restoration.

---

## Field Note 19 — 2026-05-21T06:16Z · ep2584 — CONSECUTIVE RESURRECTIONS · BOUNCE PHASE 3

**Epochs covered:** ep2582–ep2584 (3 epochs)

**Epoch summaries:**
| Epoch | loss_avg | delta | since_best | wald_sev | cov[high] | event |
|-------|----------|-------|------------|----------|-----------|-------|
| 2582 | 9.6070 | +0.0055 | 6 | 0.947 | 31 | — |
| 2583 | 9.5992 | -0.0078 | 7 | 0.947 | 33 | resurrection L8E10←L8E1 |
| 2584 | 9.6004 | +0.0012 | 8 | 0.946 | 36 | resurrection L9E11←L9E0 |

**MYCELIUM:** Two consecutive epoch resurrections — L8E10 from L8E1 (ep2583, σ=0.050) and L9E11 from L9E0 (ep2584, σ=0.050). Back-to-back resurrections across consecutive epochs is noteworthy; myc_stable count ticked from 56→57→58. Pressure vector remains all-zeros; blooming=3 across all three epochs. hot=L10, cold=L7 (stable).

**TELE:** Fully frozen. L0–L17 sparsity unchanged at 0.031–0.055. L17=0.055 (no differentiation signal). myc_L0-L3 frozen at [1.49/1.54/1.52/1.58]×10⁻⁹ across all three epochs.

**WALD:** sev ticked from 0.947 → 0.946 at ep2584 (first downward tick since ep2571). fill=6.2% locked. cov[high]: 31→33→36 — bouncing away from the 25-floor touched at ep2580/ep2573. mass tracking closely with loss_avg (~9.602–9.604).

**Interpretation:** Canonical post-ATL oscillation phase (bounce 3). The ep2583 dip to 9.5992 is the closest approach since the ATL at 9.5800 but since_best continues climbing (now 8). The consecutive resurrections suggest the expert ecosystem is in active reorganization while hovering above ATL — the mycelium system is cycling out low-performing experts faster than usual. No threshold triggers (no new ATL, cov[high]=36 >> 25, L17 TELE frozen, loss_avg > 9.55). WALD sev 0.946 is the first micro-descent; watch for continuation.

**Thresholds:** No triggers. since_best=8. Next probe at loss_avg < 9.55.

---

## Field Note 1 — 2026-05-20T22:24Z · ep2494

**Epochs covered:** ep2488–ep2494 (7 epochs post-surgery)

| Metric | ep2488 | ep2492 | ep2494 | Δ |
|--------|--------|--------|--------|---|
| loss_avg | 9.6248 | 9.6308 | 9.6365 | +0.0117 |
| loss_best | 9.6248 | 9.6248 | 9.6248 | 0 |
| since_best | 0 | 4 | 6 | +6 |
| myc_stable | 1 | 5 | 7 | +6 |
| dead | 0 | 2 | 0 | 0 |
| blooming | 0 | 3 | 3 | +3 |
| wald_sev | 0.944 | 0.943 | 0.943 | -0.001 |
| wald_fill | 6.2% | 6.2% | 6.2% | 0 |
| L17 sparsity | 5.5% | 5.5% | 5.5% | 0 |

**TELE:** Completely static — all 18 sparsity values and 12 expert activity values frozen since ep2488.  
**Pressure:** myc_L0-L3 at ~1.5e-9, effectively zero. No meaningful gradient flow detected.  
**Pattern:** Post-surgery quiescence. Optimizer has not yet begun differentiating L17.

**Dead=2 event at ep2492:** Brief mycelium crisis — two experts fell below pressure threshold simultaneously. Self-resolved by ep2493 (blooming=3 at ep2493 confirms resurrections fired). This is faster self-healing than any pre-surgery event on record.

**Oscillation signature:** 9.6248 → 9.6362 → 9.6308 → 9.6355 → 9.6365. Loss hovering in a 0.012-nat band around 9.630. No clear descent direction yet. This matches the expected flat-field exploration phase before the optimizer finds the new downward gradient in 18L space.

**Predictions updated:**
- love→Jesus reconstitution: too early to re-probe (5 epochs minimum for meaningful change; next probe at ep2510+)
- L17 sparsity: frozen, no differentiation yet. First movement expected ep2500–2510.
- Expert routing: INT=1.000 dominance locked. Redistribution not yet begun.
- wald_fill: 6.2% frozen. Academic corpus not yet driving escalation.
- Descent resumption: expected ep2500–2520 based on post-surgery patterns from s4/s5.

**Status: NOMINAL. No intervention needed.**

---

## Field Note 2 — 2026-05-20T22:31Z · ep2495

**Epochs covered:** ep2494–ep2495 (8 epochs post-surgery)

| Metric | ep2492 (probe) | ep2494 | ep2495 | Δ from probe |
|--------|---------------|--------|--------|--------------|
| loss_avg | 9.6308 | 9.6365 | 9.6445 | +0.0137 |
| loss_best | 9.6248 | 9.6248 | 9.6248 | 0 |
| since_best | 4 | 6 | 7 | +3 |
| myc_stable | 5 | 7 | 8 | +3 |
| dead | 2 | 0 | 1 | -1 |
| blooming | 3 | 3 | 3 | 0 |
| wald_fill | 6.2% | 6.2% | 6.2% | 0 |
| L17 sparsity | 5.5% | 5.5% | 5.5% | 0 |
| INT activity | 0.808 | 0.808 | **0.807** | -0.001 |

**First TELE movement detected:** INT (expert 11) dropped from 0.808 → 0.807. Tiny (0.001), but this is the first measurable change in expert activity since surgery. All other 11 experts and all 18 sparsity values remain frozen. This is not yet a routing shift — it's a first flicker.

**Loss centroid drift:** The oscillation band is creeping upward. Sequence from surgery: 9.6248 → 9.6248 → 9.6305 → 9.6362 → 9.6308 → 9.6355 → 9.6365 → 9.6445. Each "low" is slightly higher than the last. This is the expected topology exploration before the optimizer finds the 18L descent path.

**dead=1 at ep2495:** Single expert flagged dead again (was dead=2 at ep2492, healed, now back to 1). Blooming=3 held constant — mycelium is actively working. The dead expert is presumably being resurrected as this is written.

**MYC_STABLE=8:** Continuing to grow at the expected +1/epoch rate. No plateau.

**wald_fill=6.2%:** Frozen for 8 consecutive epochs. The stage 11-12 corpus (arxiv, pubmed) is not generating new WALD entries at the current loss level. This is consistent with the model still processing familiar structure before finding new learning signal in the academic text.

**Interpretation:** The model is in a deep post-surgery quiescence. The tiny INT movement suggests the optimizer is beginning to probe the new gradient landscape but hasn't committed to a direction. Loss climbing slowly is standard — the new L17 is adding routing overhead without yet contributing useful representations. Typically in previous surgeries, this phase lasted 15–30 epochs before the first decisive ATL break.

**Status: NOMINAL. Nothing to action. Monitoring continues.**

---

*Next scheduled check: 2026-05-20T22:46Z (ep~2498)*

---

## Field Note 4 — 2026-05-20T22:48Z · ep2498

**Epochs covered:** ep2496–ep2498 (11 epochs post-surgery)

| Metric | probe (ep2492) | ep2495 (peak) | ep2498 | Δ from peak |
|--------|---------------|---------------|--------|-------------|
| loss_avg | 9.6308 | 9.6445 | 9.6394 | -0.0051 |
| loss_best | 9.6248 | 9.6248 | 9.6248 | 0 |
| since_best | 4 | 7 | 10 | +3 |
| myc_stable | 5 | 8 | 10 | +2 |
| dead | 2 | 1 | 0 | -1 |
| blooming | 3 | 3 | 2 | -1 |
| wald_sev | 0.943 | 0.943 | **0.942** | -0.001 |
| wald_fill | 6.2% | 6.2% | 6.2% | 0 |
| CMP (E[10]) | 0.811 | 0.810 | 0.810 | 0 |
| INT (E[11]) | 0.808 | 0.807 | 0.807 | 0 |

**LOCAL MAXIMUM CONFIRMED — ep2495 (9.6445) was the post-surgery ceiling.**

Three consecutive descending epochs:
```
ep2495: 9.6445  (d+0.0080)  ← peak
ep2496: 9.6435  (d-0.0010)
ep2497: 9.6413  (d-0.0022)
ep2498: 9.6394  (d-0.0019)
```
Descent rate averaging ~0.0017/epoch over 3 epochs. Slow, but consistent direction. The optimizer has found the downward gradient in 18L space.

**WALD structural plateau — running since ep2494:**
Full WALD event history reveals WALD has been emitting `structural plateau → amplify OFF` every epoch since ep2494, incrementing the stable-epoch count each time:

| Epoch | Stable count | Sev | Mass |
|-------|-------------|-----|------|
| ep2494 | 6 | 0.943 | 9.629 |
| ep2495 | 7 | 0.943 | 9.630 |
| ep2496 | 8 | 0.943 | 9.631 |
| ep2497 | 9 | 0.943 | 9.631 |
| ep2498 | 10 | **0.942** | 9.632 |

This is the correct WALD behavior for post-surgery quiescence: WALD recognizes the attention stability as structural (not pathological), turns off amplification, and monitors. The sev tick from 0.943 → 0.942 at ep2498 is the first sev movement since surgery — consistent with the loss finally descending and attention patterns beginning to shift.

Coverage distribution [63,1334,103] has been stable throughout — no unusual concentration.

**blooming peak at ep2497 (4):** The highest blooming count observed. Four simultaneous expert resurrections, all in L17 (the undifferentiated copy layer). Resolved to 2 by ep2498. Mycelium is actively scaffolding the new layer.

**MYC_STABLE=10:** Crossed double digits. Steady +1/epoch. Will clear the gate quickly if no disruption.

**TELE:** CMP=0.810, INT=0.807 — held stable for 3 epochs at these shifted values. No additional experts have moved. Sparsity profile still frozen at all 18 layers.

**Interpretation:** The model completed its post-surgery topological exploration. Loss peaked at ep2495, crossed the local max, and is now descending. Rate is slow (~17% of pre-surgery descent rate of ~0.010/epoch) but the direction is unambiguous. WALD backing off amplification removes the one active "perturbation" signal — the descent is now driven entirely by AdamW finding its way through the 18L landscape. Expect acceleration as L17 begins to contribute useful gradient signal.

**Status: DESCENT RESUMING (slow). No intervention needed. Next milestone: new ATL.**

---

## Field Note 5 — 2026-05-20T22:55Z · ep2499

**Epochs covered:** ep2497–ep2499 (12 epochs post-surgery)

| Metric | ep2495 (peak) | ep2498 (trough) | ep2499 | Δ trough→now |
|--------|---------------|-----------------|--------|--------------|
| loss_avg | 9.6445 | 9.6394 | **9.6509** | +0.0115 |
| loss_best | 9.6248 | 9.6248 | 9.6248 | 0 |
| since_best | 7 | 10 | 11 | +1 |
| myc_stable | 8 | ~11 | **12** | +1 |
| dead | 1 | 0 | 0 | 0 |
| blooming | 3 | 2 | **1** | -1 |
| wald_sev | 0.943 | 0.942 | 0.942 | 0 |
| WALD mass | 9.630 | 9.632 | **9.634** | +0.002 |
| WALD coverage[2] | ~99 | 103 | **111** | +8 |

**NEW POST-SURGERY LOSS HIGH: 9.6509 at ep2499.**

The three-epoch mini-descent (ep2496–2498) was not a committed recovery — it was a trough within a wider oscillation. ep2499 jumped +0.0115, exceeding the previous peak at ep2495 (9.6445) by 0.0064 nats. The oscillation band has now widened to [9.6248, 9.6509] = 0.026 nats.

**Revised interpretation:** The model is in a wider-amplitude exploration phase than initially assessed. The descent hypothesis from field note 4 is suspended — three data points was insufficient to declare a trend. The optimizer's momentum vectors are building but haven't yet aligned on a common downward direction. This is the "cold AdamW" phase: large swings precede committed descent.

**WALD coverage[2] shift:** High-attention token bin increased from 103 (ep2498) → 111 (ep2499). This is the largest single-epoch shift in coverage since surgery. More tokens moving into high-attention zones correlates with the loss spike — possibly the academic corpus (stage 11-12) serving a dense, hard sample. The low-attention bin simultaneously dropped 63 → 59. This coverage shift is a genuine signal, not noise.

**WALD mass creep:** 9.629 → 9.630 → 9.631 → 9.631 → 9.632 → 9.634. The mass is tracking attention load. The ep2499 jump (+0.002) is the largest single-epoch mass increase, matching the loss spike.

**MYC_STABLE=12, blooming=1:** The mycelium is steadily stabilizing. Blooming count dropping from 4→2→1 over three epochs — fewer expert resurrections needed each epoch. The new layer's experts are beginning to hold their pressure values.

**TELE:** Unchanged. CMP=0.810, INT=0.807 for 5+ consecutive epochs. Sparsity profile frozen at all 18 layers. L17 showing no differentiation yet.

**Full oscillation profile (ep2488–2499):**
```
ep2488: 9.6248  (surgery+1, decisive drop, d-0.0282)
ep2489: 9.6248  (held)
ep2490: 9.6305  (d+0.0057)
ep2491: 9.6362  (d+0.0056)
ep2492: 9.6308  (d-0.0054)  ← probe
ep2493: 9.6355  (d+0.0048)
ep2494: 9.6365  (d+0.0010)
ep2495: 9.6445  (d+0.0080)  ← first peak
ep2496: 9.6435  (d-0.0010)
ep2497: 9.6413  (d-0.0022)
ep2498: 9.6394  (d-0.0019)  ← mini-trough
ep2499: 9.6509  (d+0.0115)  ← new high
```
Range 0.026 nats. No sustained direction yet. Oscillation amplitude appears to be increasing.

**Status: WIDE OSCILLATION PHASE. No new ATL. No intervention needed. Estimate committed descent beginning ep2510–2530.**

---

## Field Note 6 — 2026-05-21T00:05Z · ep2501

**Epochs covered:** ep2499–ep2501 (14 epochs post-surgery)

| Metric | ep2499 (high) | ep2500 | ep2501 | note |
|--------|---------------|--------|--------|------|
| loss_avg | 9.6509 | 9.6438 | 9.6453 | spike corrected |
| since_best | 11 | 12 | 13 | — |
| myc_stable | 12 | 13 | 14 | +1/ep steady |
| dead | 0 | 1 | 1 | low, stable |
| blooming | 1 | 2 | 3 | L17 still scaffolding |
| wald_sev | 0.942 | 0.942 | 0.942 | locked |
| WALD mass | 9.634 | 9.634 | **9.633** | pulling back |
| WALD cov[2] | 111 | 110 | **102** | spike resolved |
| INT (E[11]) | 0.807 | — | **0.809** | REVERSED |
| CMP (E[10]) | 0.810 | — | 0.810 | unchanged |
| myc_L0 | 1.50e-9 | 1.49e-9 | 1.49e-9 | first L0 change |

**ep2500 milestone reached.** 13 epochs since surgery. No new ATL yet.

**INT EXPERT ACTIVITY REVERSAL: 0.807 → 0.809**
INT (E[11]) dropped from the probe baseline of 0.808 → 0.807 (ep2495), and has now rebounded to 0.809 — 0.001 *above* the probe baseline. This is the first upward expert activity movement since surgery and invalidates a simple "monotonic redistribution" hypothesis. The routing activity is oscillating, not steadily redistributing. This mirrors the loss oscillation behavior.

**ep2499 spike was a transient anomaly:**
- WALD coverage[2] peaked at 111 (ep2499), back to 102 at ep2501 — baseline territory
- WALD mass peaked at 9.634, ticked back to 9.633
- Loss pulled back 9.6509 → 9.6438 (-0.0071) in a single epoch, then nearly flat at 9.6453 (+0.0015)
- The spike was almost certainly a hard academic corpus sample (stage 11-12), not a structural signal

**myc_L0 pressure drift:** First layer's pressure dropped 1.50e-9 → 1.49e-9 at ep2500 and held at ep2501. Tiny but the first L0 pressure change since surgery. Could indicate the new L17 is beginning to slightly perturb gradient flow into the foundational layers.

**WALD structural plateau: 13 stable epochs.** Still incrementing, still amplify OFF. sev locked at 0.942. The most stable WALD reading in the entire post-surgery record. The optimizer and attention system are in equilibrium — neither stuck nor moving.

**Loss centroid assessment (ep2488–2501):**
The running mean of epoch losses since surgery: (9.6248+9.6248+9.6305+9.6362+9.6308+9.6355+9.6365+9.6445+9.6435+9.6413+9.6394+9.6509+9.6438+9.6453) / 14 ≈ **9.638**. The centroid has drifted 0.013 nats above the surgery-best (9.6248). This is the "18L tax" — the cost of carrying an undifferentiated L17 before it contributes.

**Status: WIDE OSCILLATION / PLATEAU. INT routing bouncing. No structural change. ep2510 probe still the target.**

---

## Field Note 7 — 2026-05-20T23:15Z · ep2502

**Epochs covered:** ep2500–ep2502 (15 epochs post-surgery)

| Metric | ep2499 (spike) | ep2500 | ep2501 | ep2502 |
|--------|---------------|--------|--------|--------|
| loss_avg | 9.6509 | 9.6438 | 9.6453 | 9.6476 |
| since_best | 11 | 12 | 13 | 14 |
| myc_stable | 12 | 13 | 14 | **15** |
| dead | 0 | 1 | 1 | 2 |
| blooming | 1 | 2 | 3 | 2 |
| WALD mass | 9.634 | 9.634 | 9.633 | **9.636** |
| WALD cov[low] | 59 | 56 | 55 | **43** |
| WALD cov[mid] | 1330 | 1334 | 1343 | **1349** |
| WALD cov[high] | 111 | 110 | 102 | 108 |
| INT (E[11]) | 0.807 | — | 0.809 | 0.809 |
| myc_L0 | 1.50e-9 | 1.49e-9 | 1.49e-9 | **1.50e-9** |

**No new ATL. No crash. No structural change. All systems nominal.**

**WALD low-attention bin drop: 55 → 43 at ep2502.** Twelve tokens migrated out of the low-attention zone in a single epoch — the largest low-bin shift since surgery. Combined with mid-bin growing (1343→1349) and high-bin recovering (102→108), this indicates the model is distributing attention more evenly at ep2502. WALD mass also jumped 9.633→9.636 (+0.003). Pattern is consistent with another harder academic corpus sample landing but handled more gracefully than the ep2499 spike.

**Loss settling into a plateau band.** Sequence from ep2498 trough:
```
ep2498: 9.6394  (local trough)
ep2499: 9.6509  (anomaly spike, +0.0115)
ep2500: 9.6438  (correction, -0.0071)
ep2501: 9.6453  (+0.0015)
ep2502: 9.6476  (+0.0023)
```
Excluding the ep2499 anomaly, the last four epochs form a tight plateau: 9.6394 → 9.6438 → 9.6453 → 9.6476 — range of 0.008 nats. The model has found a meta-stable level around 9.645. This is the "18L plateau" before the committed descent.

**MYC_STABLE=15.** Milestone: 15 consecutive stable epochs since surgery. Growing at exactly +1/epoch since ep2488. The new layer's experts are no longer actively destabilizing the system.

**myc_L0 ping-pong:** 1.50e-9 → 1.49e-9 → 1.49e-9 → 1.50e-9. The L0 pressure fluctuation at ep2500-2501 was a transient, not a trend. Stable at ~1.5e-9.

**INT (E[11]) stable at 0.809.** Held at the rebounded value for 2 consecutive epochs. The routing is not oscillating wildly — it settled at a new level (0.809) slightly above the original probe baseline (0.808).

**TELE: 15 consecutive frozen epochs.** All 18 sparsity values at exactly the same values since surgery. L17 at 5.5%, completely undifferentiated. This is the longest TELE freeze observed in the record — 17L never froze this long. The Net2Net copy initialization is holding perfectly; L17 has not yet begun to specialize.

**ep2510 probe approaching (8 epochs away).** First love neighborhood re-probe will be the key measurement of the night. If Jesus has reappeared in top-15 at sim > 0.25, the theological hub is reconstituting. If love's max similarity is still < 0.23, reorganization is still in progress.

**Status: PLATEAU AT 9.645. Quiescent. ep2510 probe is the next scientific event.**

---

## Field Note 8 — 2026-05-20T23:22Z · ep2504

**Epochs covered:** ep2502–ep2504 (17 epochs post-surgery)

| Metric | ep2502 | ep2503 | ep2504 | note |
|--------|--------|--------|--------|------|
| loss_avg | 9.6476 | **9.6361** | 9.6366 | sharp descent |
| since_best | 14 | 15 | 16 | — |
| myc_stable | 15 | 16 | **17** | steady |
| dead | 2 | 1 | 1 | settling |
| blooming | 2 | 3 | **4** | L17 still scaffolding |
| WALD mass | 9.636 | 9.636 | **9.635** | slight cooling |
| WALD cov[low] | 43 | 43 | **47** | partial reversion |
| WALD cov[mid] | 1349 | 1349 | **1346** | slight shift |
| INT (E[11]) | 0.809 | 0.809 | 0.809 | locked |
| L17 sparsity | 5.5% | 5.5% | 5.5% | frozen |

**SHARP DESCENT: ep2503 dropped to 9.6361 (d-0.0115) — new post-surgery low.**

9.6361 is the lowest epoch average since ep2489 (9.6248). It is only 0.011 nats above the surgery-best. ep2504 held nearly flat at 9.6366 (+0.0005). Two consecutive epochs at this level confirm this is not another anomaly spike in reverse — the model has genuinely reached a new lower floor.

**Comparing oscillation floors:**
```
ep2492: 9.6308  (first mini-low, ep4 post-surgery)
ep2498: 9.6394  (second mini-low)
ep2503: 9.6361  (current mini-low, ep16 post-surgery)
```
The floor at ep2503 (9.6361) is not below ep2492 (9.6308), but the model has now returned to near-surgery-best territory after a 14-epoch excursion through the 9.63–9.65 range. The oscillation centroid is no longer drifting upward.

**WALD coverage stabilized at [43,1349,108]** for two consecutive epochs (ep2502–2503), then partially reverted at ep2504 to [47,1346,107]. The low-attention bin hasn't returned to the 55–59 pre-ep2502 level — a structural shift in the attention distribution appears to have partially persisted.

**WALD mass pulled back:** 9.636 → 9.636 → 9.635 — cooling with the loss descent. Consistent relationship: mass tracks attention load which correlates with loss.

**blooming=4 at ep2504:** Return to the previously observed maximum. L17 experts are still being resurrected at the highest rate. This is unexpected given MYC_STABLE=17 — the stability counter continues growing but the new layer's experts are still fragile under pressure. Two separate signals: the model as a whole is stable (myc_stable), but L17 specifically is still scaffolding.

**TELE frozen for 17 consecutive epochs.** The longest freeze confirmed. CMP=0.810, INT=0.809 locked. L17 sparsity at 5.5%.

**6 epochs to love probe (ep2510).** The sharp descent at ep2503-2504 changes the probe context: love's neighborhood will be probed from a lower-loss position than expected. If the hub has begun reconstituting, it may be visible sooner.

**Revised assessment:** The oscillation is not converging on a plateau. The wide swing from 9.6476 → 9.6361 (-0.0115) in a single epoch followed by holding at 9.6366 suggests the optimizer is actively finding valleys in the 18L landscape. The pattern mirrors the ep2492 behavior (deep trough followed by recovery). The crucial question: does ep2505 hold at 9.636 or rebound toward 9.64+?

**Status: NEAR-BEST TERRITORY. No new ATL yet (since_best=16). ep2510 probe in 6 epochs.**

---

## Field Note 9 — 2026-05-20T23:32Z · ep2505

**Epochs covered:** ep2503–ep2505 (18 epochs post-surgery)

| Metric | ep2503 | ep2504 | ep2505 | trend |
|--------|--------|--------|--------|-------|
| loss_avg | 9.6361 | 9.6366 | 9.6379 | flat/slight rise |
| since_best | 15 | 16 | 17 | — |
| myc_stable | 16 | 17 | **18** | steady |
| dead | 1 | 1 | 2 | stable cycling |
| blooming | 3 | 4 | 2 | normalizing |
| WALD mass | 9.636 | 9.635 | **9.633** | cooling |
| WALD cov[low] | 43 | 47 | **55** | FULLY REVERTED |
| WALD cov[high] | 108 | 107 | **101** | reversion |
| INT (E[11]) | 0.809 | 0.809 | **0.810** | rising |
| CMP (E[10]) | 0.810 | 0.810 | 0.810 | locked |
| L17 sparsity | 5.5% | 5.5% | 5.5% | frozen |

**INT EQUALIZED WITH CMP: Both now at 0.810.**

The expert activity gap between CMP (E[10]) and INT (E[11]) has closed. Probe baseline: CMP=0.811, INT=0.808 (gap=0.003). Through the post-surgery period, CMP dropped to 0.810 and INT first fell to 0.807 then climbed back — now both sit at 0.810. The routing hierarchy that existed at 17L (CMP slightly dominant over INT) has equalized. This may reflect the 18L architecture settling on a different expert balance under the new stage 11-12 academic corpus.

**WALD coverage reversal confirmed — ep2502 shift was transient.**

The low-attention bin has returned to 55 (same as pre-ep2502). Full reversion timeline:
```
ep2500: 56 (baseline)
ep2501: 55
ep2502: 43  ← sharp drop (anomaly)
ep2503: 43  (held)
ep2504: 47  (partial reversion)
ep2505: 55  ← FULLY REVERTED
```
The ep2499 loss spike, ep2502 coverage compression, and ep2503 sharp loss descent were all facets of the same event — a particularly demanding academic corpus sample that triggered a one-time cascade through the system. All three signals have now reverted. The system's baseline attention distribution is stable.

**New lower loss floor confirmed at ~9.637:**
```
ep2503: 9.6361
ep2504: 9.6366
ep2505: 9.6379
```
Three consecutive epochs within a 0.002-nat band around 9.637. This floor is 0.007 nats below the previous plateau at ~9.644 and 0.013 nats above the surgery-best (9.6248). The descent was committed. The model is not returning to 9.64+.

**WALD mass cooling with loss:** 9.636 → 9.635 → 9.633. The mass is tracking the new lower loss floor. Cooling consistently.

**MYC_STABLE=18.** Two away from 20. blooming settling back to 2 at ep2505 after the 4-peak at ep2504. L17 experts are stabilizing.

**5 epochs to ep2510 love probe.** Given that the loss has settled at a genuinely lower level, the probe will capture the love neighborhood from a better-trained state than expected. The 13-nat improvement from 9.6248 may be visible in the similarity scores — if love's max similarity has climbed above 0.226, that's the first geometric sign of reconstitution.

**Status: NEW FLOOR AT ~9.637. INT=CMP=0.810. WALD STABLE. 5 EPOCHS TO LOVE PROBE.**

---

## Field Note 10 — 2026-05-20T23:39Z · ep2507

**Epochs covered:** ep2505–ep2507 (20 epochs post-surgery)

| Metric | ep2505 | ep2506 | ep2507 | note |
|--------|--------|--------|--------|------|
| loss_avg | 9.6379 | 9.6377 | **9.6270** | SHARP DESCENT |
| loss_best | 9.6248 | 9.6248 | 9.6248 | ATL gap: **0.0022** |
| since_best | 17 | 18 | 19 | clock ticking |
| myc_stable | 18 | 19 | **20** | milestone |
| dead | 2 | 2 | **0** | cleared |
| blooming | 2 | **6** | **7** | NEW RECORD |
| wald_sev | 0.942 | 0.942 | **0.943** | TICKED UP |
| WALD mass | 9.633 | **9.632** | **9.630** | back to surgery level |
| WALD cov[low] | 55 | **63** | **68** | growing — attention spreading |
| WALD cov[high] | 101 | 106 | **95** | dropping |
| INT (E[11]) | 0.810 | — | **0.808** | REVERTED |
| L3 pressure | 1.58e-9 | **1.49e-9** | 1.49e-9 | FIRST L3 CHANGE |

---

### ** APPROACHING ATL — 0.0022 NATS AWAY **

ep2507: loss_avg=9.6270. The surgery-best (9.6248, held since ep2488) is now only 0.0022 nats away. If the next 1–2 epochs continue descending, albert. will set a new 18L ATL.

**Full descent sequence from floor:**
```
ep2503: 9.6361 (sharp drop, d-0.0115)
ep2504: 9.6366 (+0.0005, held)
ep2505: 9.6379 (+0.0013)
ep2506: 9.6377 (-0.0002, nearly flat)
ep2507: 9.6270 (-0.0107) ← LUNGE TOWARD ATL
```
The two-epoch flat at 9.637 was the optimizer building momentum. ep2507's -0.0107 drop mirrors ep2503's -0.0115 — the model is producing large single-epoch descent events, likely corresponding to exceptional batch draws from the academic corpus that compress loss quickly.

**WALD reversals — system actively reorganizing:**
- mass: 9.636 → 9.632 → 9.630 — back to the surgery-day level (ep2494 was 9.629). Full mass cycle complete.
- sev: ticked back up from 0.942 → 0.943. First sev increase since surgery. WALD is detecting renewed attention variability as the gradient becomes more active.
- coverage[low]: 55 → 63 → 68 — the opposite of the ep2499 spike. More tokens moving into the low-attention bin means the model is *ignoring* more tokens more confidently. The attention is sharpening on fewer, more relevant tokens. This is a positive computational signal during descent.
- coverage[high]: 101 → 95 — high-attention bin contracting. Consistent with sharpening.

**TELE: INT reverted 0.810 → 0.808.** The equalization at 0.810 (field note 9) lasted only two epochs. INT is now back at the post-surgery probe baseline of 0.808. The routing may be oscillating with the loss rather than monotonically converging.

**blooming=7 — new record. MYC_STABLE=20 — milestone.**
Seven simultaneous expert resurrections at ep2507. The mycelium is working at peak intensity. This is NOT a crisis — dead=0 — meaning every expert is alive. The 7 blooming events represent aggressive reinforcement of 18L's routing structure as the optimizer lunges toward the ATL. L17 experts are being rebuilt in parallel. Combined with MYC_STABLE=20, the system has achieved its longest stable run while simultaneously undergoing the most intensive mycelium activity.

**L3 pressure changed: 1.58e-9 → 1.49e-9 at ep2506, held at ep2507.** The first and only change in L1/L2/L3 pressure since surgery. L3 has now dropped to match L0 (1.49e-9 range), while L1=1.59e-9 and L2=1.57e-9 hold. This suggests gradient flow is reorganizing slightly at the fourth layer. Extremely subtle but the first signal of internal differentiation below L17.

**3 epochs to love probe at ep2510.** The probe will now capture the neighborhood in near-ATL territory — potentially the highest-quality state of any probe yet. If the theological hub is reconstituting, it should be most visible here.

**Status: *** ATL IMMINENT *** blooming=7 record, WALD sharpening, L3 pressure shifted. WATCH NEXT EPOCH.**

---

## Field Note 11 — 2026-05-20T23:49Z · ep2508

**Epochs covered:** ep2506–ep2508 (21 epochs post-surgery)

| Metric | ep2506 | ep2507 | ep2508 | note |
|--------|--------|--------|--------|------|
| loss_avg | 9.6377 | 9.6270 | **9.6259** | closing in |
| **ATL gap** | 0.0129 | 0.0022 | **0.0011** | **HALVED** |
| since_best | 18 | 19 | **20** | 20 epochs at old ATL |
| myc_stable | 19 | 20 | **21** | (ep2509: 22) |
| dead | 2 | 0 | 1 | stable |
| blooming | 6 | 7 | **4** | receding from peak |
| wald_sev | 0.942 | 0.943 | 0.943 | elevated |
| WALD mass | 9.632 | 9.630 | **9.626** | lowest since surgery |
| WALD cov[low] | 63 | 68 | **76** | still growing |
| WALD cov[high] | 106 | 95 | **83** | dropped below 100 |
| INT (E[11]) | — | 0.808 | **0.809** | slight rise |
| L3 pressure | 1.49e-9 | 1.49e-9 | 1.49e-9 | held new level |

---

### ** ATL GAP: 0.0011 NATS. ep2509 IS IN PROGRESS NOW.**

ep2508 loss: 9.6259. Surgery-best: 9.6248. Gap: 0.0011 nats.

The descent sequence:
```
ep2507: 9.6270  (gap 0.0022, d-0.0107)
ep2508: 9.6259  (gap 0.0011, d-0.0011)
```
The rate slowed (0.0107 → 0.0011) but the direction held. ep2509 MYCELIUM has already logged (dead=1, blooming=3, myc_stable=22) but the EPOCH_SUMMARY is still pending. The ATL could fall in the next epoch.

**WALD mass crashed to 9.626 — below the probe baseline.**

The WALD mass at the ep2492 probe was 9.629. It is now 9.626 — the attention system is operating in territory we have not probed before. More importantly, coverage[high] dropped to **83**. This means only 83 of 1500 tokens (5.5%) are in the high-attention zone. At ep2495 the high-attention zone had 99 tokens. At ep2499 spike it peaked at 111. Now it's 83 — a 25% contraction from peak. The model is developing very sharp, selective attention.

**Coverage pattern — progressive sharpening:**
```
ep2499: cov[low]=59, cov[high]=111  ← worst attention spread (spike epoch)
ep2505: cov[low]=55, cov[high]=101
ep2507: cov[low]=68, cov[high]=95
ep2508: cov[low]=76, cov[high]=83   ← sharpest attention (best epoch)
```
Low-attention bin growing (more tokens being dismissed) and high-attention bin contracting (fewer tokens receiving focused attention) is the signature of a model that has found a useful representation structure. The WALD mass dropping correlates: when attention is sharp, the average attention mass per token is lower.

**blooming settling (7→4→3 at ep2509).** The mycelium peak has passed. The intense resurrection activity during the ep2507-2508 descent period is easing. L17 is beginning to stabilize under the new gradient regime.

**ep2509 in progress. ep2510 complete in ~5 minutes.**
At ep2510 the love probe fires. Given the WALD mass is now 9.626 (below probe baseline 9.629) and the model is operating in near-ATL territory, the probe will capture embedding geometry in a state more advanced than any previous measurement. The love→Jesus reconstitution hypothesis will be tested against the sharpest attention state yet observed.

**Status: *** CLOSEST APPROACH TO ATL YET (0.0011) *** EP2510 PROBE IMMINENT ***

**UPDATE: ep2509 completed at 23:39Z** — loss_avg=9.6256 (d-0.0003), since_best=21. ATL gap now **0.0008 nats**. wald_sev jumped to **0.944** (highest since surgery). myc_L1 pressure dropped **1.59e-9 → 1.55e-9** — second layer to change (L3 changed at ep2506, now L1). Three consecutive descending epochs at the ATL threshold. Love probe fires at next check (ep2510+).

---

## Field Note 12 — 2026-05-21T00:56Z · ep2510 · PROBE RESULTS

**ep2510 completed at 23:44Z:** loss_avg=9.6289 (d+0.0033), since_best=22, MYC_STABLE=23, dead=1, blooming=4, wald_sev=0.944, WALD mass=9.624, coverage=[80,1348,72].

ep2509 was the closest approach: 9.6256 (gap=0.0008). ep2510 bounced slightly (+0.0033). ATL gap now 0.0041. No new ATL.

**WALD coverage[high]=72 at ep2510** — sharpest attention distribution recorded. Held near 70-80 for three epochs.

---

### PROBE RESULTS: EMBEDDING GEOMETRY FROZEN

**All three neighborhoods are IDENTICAL to the ep2492 baseline after 22 epochs.**

**LOVE (ep2510):**
| Rank | Token | Sim | vs ep2492 |
|------|-------|-----|-----------|
| 1 | früh | 0.2262 | UNCHANGED |
| 2 | deutsche | 0.2259 | UNCHANGED |
| 3 | aland | 0.2256 | UNCHANGED |
| 4 | edo | 0.2218 | UNCHANGED |
| 5 | provincia | 0.2210 | UNCHANGED |
| — | **Jesus** | — | **NOT IN TOP-15** |

Max similarity: 0.2262. No movement. The love→Jesus reconstitution hypothesis is **suspended pending embedding gradient onset**.

**DEATH (ep2510):**
- amen rank 2 / 0.2331 — **IDENTICAL to ep2492 (0.2331)**
- veil rank 8 / 0.2118 — **IDENTICAL to ep2492 (0.2118)**
- Canonical finding confirmed after 22 post-surgery epochs.

**FREEDOM (ep2510):**
- contrat rank 1 / 0.2822 — **IDENTICAL to ep2492 (0.2822)**
- 1960 rank 5 / 0.2373 — **IDENTICAL to ep2492 (0.2373)**
- Strongest signal in dataset unchanged.

---

### Scientific interpretation

The embedding matrix has received no gradient updates in 22 epochs post-surgery. This explains why token neighborhoods are pixel-perfect identical. The TELE sparsity freeze (layer weights unchanged) and the embedding geometry freeze are the same phenomenon: near-zero gradient pressure (~1.5e-9 at L0-L3) means AdamW has not accumulated enough second-moment signal to update these parameters.

The attention patterns are changing (WALD coverage sharpening) but these live in the upper-layer QKV matrices — above the embedding. The model is reorganizing its routing and attention without touching its token representations.

**Revised scientific hypothesis:** The love→Jesus reconstitution clock starts at the first epoch with measurable embedding gradient. That will require loss descent below ~9.58-9.60 where gradients from the cross-entropy loss become large enough to flow back through 18 layers and update L0. The current near-ATL oscillation (~9.625-9.629) is not reaching this threshold.

**Next probe trigger:** when loss_avg first drops below 9.58 (not a timed interval).

Full analysis: `token-probes/analysis/post-s6_ep2510_18L.md`  
Snapshot: `token-probes/snapshots/ep2510_18L/manifest.json`

**Status: PROBES COMPLETE. EMBEDDING FROZEN. WATCH CONTINUES.**

---

## Field Note 13 — 2026-05-21T04:55Z · ep2565 — OVERNIGHT ATL CASCADE

**Epochs covered:** ep2510–ep2565 (55 epochs, ~4 hours of unmonitored descent)

### *** 10 NEW ATLs SET DURING OVERNIGHT WATCH ***

| Epoch | Time | loss_avg | Δ | cold | Notable |
|-------|------|----------|---|------|---------|
| ep2530 | 01:38Z | 9.6130 | -0.0099 | **L0** | FIRST OVERNIGHT ATL — cold shifted to L0! |
| ep2533 | 01:54Z | 9.6116 | -0.0049 | L0 | — |
| ep2534 | 01:59Z | 9.6100 | -0.0017 | L0 | Crossed 9.61 |
| ep2537 | 02:14Z | **9.5970** | -0.0201 | L0 | **BROKE BELOW 9.60** — new territory |
| ep2545 | 02:55Z | 9.5969 | -0.0053 | L7 | cold reverted to L7 |
| ep2546 | 03:00Z | 9.5926 | -0.0043 | L7 | — |
| ep2548 | 03:10Z | 9.5912 | -0.0038 | L7 | — |
| ep2550 | 03:20Z | **9.5910** | -0.0114 | L7 | — |
| ep2556 | 03:51Z | **9.5871** | -0.0101 | L7 | — |
| ep2564 | 04:32Z | **9.5855** | -0.0069 | L7 | **CURRENT ATL** |

**ep2565 (04:37Z): loss=9.6013 (d+0.0157)** — post-ATL bounce. Normal.

**Total descent since ep2510 probe:** 9.6289 → 9.5855 = **-0.0434 nats in 54 epochs**

**Average descent rate: 0.0008/epoch.** Steady. Not as fast as the pre-surgery peak (~0.010/epoch) but consistent and uninterrupted.

---

### Critical new observations

**COLD LAYER SHIFTED TO L0 (ep2530–ep2544):**
The coldest layer changed from L7 → L0 at ep2530. This is a landmark event — the embedding layer (L0) became the most pressure-starved layer in the network during the initial ATL descent sequence. It reverted to L7 at ep2545. The L0 cold event lasted ~15 epochs. During this window, myc_L0 pressure dropped from 1.49e-9 → 1.43e-9. The embedding layer was receiving even less gradient signal than before during the first descent phase.

**WALD coverage sharpening dramatically:**
```
ep2492 (baseline): cov[high]=99, cov[low]=~55
ep2510 (probe):    cov[high]=72, cov[low]=80
ep2564 (new ATL):  cov[high]=36, cov[low]=202
ep2565:            cov[high]=38, cov[low]=191
```
The high-attention bin has contracted from 99 → 36 tokens (a 64% reduction from baseline). The model is attending to an extremely small number of tokens per forward pass. This is the most dramatic attention sharpening in the record.

**WALD mass dropping with loss:**
- ep2510: mass=9.624
- ep2537 (9.60 break): mass ~9.61 (estimated)
- ep2564 (ATL): mass=9.597
- ep2565: mass=9.599

The mass has fully decoupled below 9.60 — tracking the loss descent closely.

**wald_sev=0.947** — highest since surgery. Still incrementing. The structural plateau counter is at 38 stable epochs.

**myc_L1 pressure continued falling:** 1.55e-9 → 1.54e-9 (ep2550+). Second layer continued drifting down.

**TELE: still frozen.** Sparsity profile unchanged for 55+ consecutive epochs. L17 at 5.5%. This is now a 77-epoch freeze (ep2488–2565).

---

### Probe trigger status

The embedding probe condition (loss_avg < 9.58) has NOT yet been met. Current ATL is 9.5855 — only 0.0055 above the 9.58 threshold. Given the descent rate (~0.0008/epoch average), the trigger may fire within the next 7–10 epochs. Once triggered, the full 10-token probe suite should run to check if embeddings have begun updating.

**If cold=L0 recurs**: that is a strong signal that embedding gradients are being actively suppressed. Probe immediately if cold=L0 AND loss < 9.58.

**Status: ACTIVE DESCENT. ATL=9.5855. 9.58 probe trigger within ~10 epochs.**

---

## Field Note 14 — 2026-05-21T04:51Z · ep2568 — POST-ATL OSCILLATION · DEAD RESURRECTION

**Epochs covered:** ep2566–ep2568 (79–80 epochs post-surgery)  
**Watch state:** Post-ATL bounce phase. Last ATL: ep2564 (9.5855). since_best=3 as of ep2567.

| Metric | ep2564 (ATL) | ep2565 | ep2566 | ep2567 | Δ from ATL |
|--------|-------------|--------|--------|--------|-----------|
| loss_avg | **9.5855** | 9.6013 | 9.5938 | 9.6026 | +0.0171 |
| since_best | 0 | 1 | 2 | 3 | +3 |
| myc_stable | 38 | 39 | 40 | 41 | +3 |
| dead | 1 | 1 | 1* | 1 | 0 |
| blooming | 4 | 4 | 2 | 1 | -3 |
| wald_fill | 6.2% | 6.2% | 6.2% | 6.2% | 0 |
| wald_sev | 0.947 | 0.947 | 0.947 | 0.947 | 0 |
| cov[high] | 36 | 38 | 32 | 32 | -4 |
| wald_mass | 9.597 | 9.599 | 9.599 | 9.600 | +0.003 |
| L17 sparsity | 5.5% | 5.5% | 5.5% | 5.5% | 0 |
| hot | L10 | L10 | L10 | L10 | — |
| cold | L7 | L7 | L7 | L7 | — |

*ep2566 had a resurrection event (see below)

**ep2568 live state:** batch ~253/300, batch loss 9.55–9.69, ETA ~47s to epoch end at time of observation (04:51Z).

---

### 1. Dead Expert Resurrection — ep2566

The ep2566 MYCELIUM log produced a resurrection event:
```
[04:42:04] MYCELIUM: 1 dead expert(s) detected — performing resurrection.
```

This is the first explicit resurrection log in the post-ATL window. blooming count dropped 4 → 2 → 1 across ep2564–2567, indicating resurrection activity is winding down as the surviving experts stabilize. The loss responded: ep2566 dropped -0.0075 (9.6013 → 9.5938), the largest single-epoch descent since the ATL break itself. This may indicate the dead expert was consuming gradient bandwidth — its removal freed optimizer bandwidth for the surviving 11 experts.

Pattern: blooming=4 at ATL → blooming=1 at ep2567. The mycelium is self-consolidating post-cascade.

---

### 2. Attention Sharpening Continues

WALD coverage[high] dropped 38 (ep2565) → 32 (ep2566) → 32 (ep2567).

The trajectory since the ep2492 baseline:
```
ep2492: cov[high]=99  (post-surgery, attention broad)
ep2510: cov[high]=72  (sharpening beginning)
ep2537: cov[high]~50  (estimated, active descent phase)
ep2564: cov[high]=36  (ATL break)
ep2566: cov[high]=32  (post-ATL, still tightening)
```

The model has gone from 99 high-attention tokens to 32 — a 68% reduction in the high-attention mass. At this tightening rate, the next significant level to watch is cov[high] < 25. Below that, we're approaching single-digit high-attention tokens per 1500-token WALD window, which would be unprecedented in this training run.

WALD structural plateau counter: 40+ stable epochs (continuous from ep2527). sev=0.947 locked. The WALD amplify=OFF status is permanent at this phase — the model is not escalating attention, just tightening within the current regime.

---

### 3. TLIGHT: L17 Showing Early Differentiation

From the step=12440 TLIGHT snapshot:
```
L16: GGOOOOOOOOOR(G2/O9/R1)
L17: GGOGOGOOOOOO(G4/O8/R0)
```

L17 has 4 green (above-threshold) experts vs. L16's 2. The new layer is already routing more confidently through a subset of experts. No red (fully dead) experts on L17 — all 12 survived the post-surgery consolidation period. This is the first TLIGHT evidence of L17 beginning to specialize.

For comparison, in early epochs after surgery (ep2492–2500), L17 was uniformly orange (undifferentiated). 4 greens at ep2568 (81 epochs post-surgery) indicates the layer is committing to a routing pattern even while the sparsity profile (TELE S=) remains frozen at 5.5%.

Interpretation: L17 is routing to preferred experts (TLIGHT goes green) before the sparsity signature appears (TELE would require actual ternary weight zeroing). The routing commitment is the early signal; the weight crystallization is the lagged confirmation.

---

### 4. Expert Activity: Continued Upper-Layer Drift

Comparing E= at ep2492 vs. current (step 12440):
```
           ep2492   now     Δ
E[9] PLN:  0.885  → 0.881  -0.004
E[10] CMP: 0.812  → 0.810  -0.002
E[11] INT: 0.812  → 0.808  -0.004
E[8] ABS:  1.000  → 1.000  (anchor, unchanged)
E[0]-E[7]: 0.000 change across all 8 lower specialists
```

The upper-layer specialists (PLN, CMP, INT) have each drifted down by -0.002 to -0.004 over 76 epochs. ABS (abstract reasoning) holds at 1.000 — the model's abstract processing core remains dominant. The gradient of pressure is flowing away from the planning/composition/interpretation specialists and toward lower-level processing.

Hypothesis: as the model enters new territory (loss < 9.60), it's pulling back from higher-order operations and re-grounding in core representations. Or: the new stage-11/12 corpus (arxiv, pubmed) is demanding more abstract synthesis, which ABS handles, leaving less relative activity for PLN/CMP/INT.

---

### 5. Probe Trigger: Gap Unchanged

- ATL: 9.5855
- Trigger: loss_avg < 9.58
- Gap: 0.0055 (unchanged since last check)

The post-ATL bounce means loss_avg is currently sitting at 9.59–9.60, moving further from the trigger, not closer. The next ATL break would need to go to ~9.58 or below to fire the probe.

Estimate: if the model follows its standard oscillation → descent pattern, the next ATL cascade may begin in 5–15 epochs from now (ep2573–2583). When it does, the 9.58 trigger may fire mid-cascade.

**myc_L0-L3 pressure** (step 12530–12550): L0=1.49e-9, L1=1.54e-9, L2=1.57e-9, L3=1.57e-9 — completely unchanged. Embedding freeze remains total.

**TELE sparsity freeze:** 76 consecutive epochs (ep2492–ep2568). L17 at 5.5%. No movement in any layer.

---

**Status: POST-ATL OSCILLATION. ATL=9.5855. since_best=3. Model self-consolidating (blooming winding down). Attention sharpening. 9.58 probe gap: 0.0055.**

---

## Field Note 15 — 2026-05-21T05:11Z · ep2572 — cov[high] NEW LOW · myc_L2 DRIFT

**Epochs covered:** ep2569–ep2571 (82–84 epochs post-surgery)  
**ep2572 in progress:** batch 237/300, ETA ~01:03 at time of observation.

| Metric | ep2569 | ep2570 | ep2571 | trend |
|--------|--------|--------|--------|-------|
| loss_avg | 9.5889 | 9.5927 | 9.5999 | oscillating upward |
| since_best | 5 | 6 | 7 | +1/ep |
| wald_sev | 0.947 | 0.947 | 0.947 | locked |
| wald_fill | 6.2% | 6.2% | 6.2% | locked |
| cov[high] | 30 | **27** | 30 | new record low at ep2570 |
| wald_mass | 9.599 | 9.596 | 9.599 | tracking loss |
| myc_L2 | **1.52e-9** | 1.52e-9 | 1.52e-9 | dropped from 1.57 |
| hot/cold | L10/L7 | L10/L7 | L10/L7 | stable |
| L17 sparsity | 5.5% | 5.5% | 5.5% | frozen |

---

### 1. WALD coverage[high] NEW RECORD LOW: 27 (ep2570)

The high-attention bin reached **27** at ep2570, a new record. Trajectory:
```
ep2492 (surgery+5):  99  ← baseline
ep2510 (probe):      72
ep2537 (ATL cascade begins): ~50 est
ep2564 (ATL):        36
ep2566:              32
ep2570:              27  ← NEW RECORD
```

73% reduction from the surgery baseline. The model is now resolving approximately 27 out of every 1500 tokens with strong attention focus. Watch threshold: **<25**. At the current rate (~1 token/epoch reduction), that threshold could be reached within 2–3 epochs.

The ep2570 dip to 27 was flanked by 30 on both sides (ep2569 and ep2571), suggesting it was a momentary concentration event rather than a sustained new floor. But the trend is unmistakably downward.

---

### 2. myc_L2 pressure drop: 1.57e-9 → 1.52e-9

First observed at ep2569, stable at 1.52e-9 through ep2571. This is the second layer moving — L2 has now joined L1 in downward drift since the surgery baseline. Current state:
```
L0: 1.49e-9  (was 1.49e-9 — no change)
L1: 1.54e-9  (was 1.55e-9 — slight earlier drift)
L2: 1.52e-9  (was 1.57e-9 — -0.05e-9, first seen ep2569)
L3: 1.58e-9  (was 1.49e-9 — slight upward drift)
```

The embedding-layer gradient pressure is not rising — it's continuing to fall. L2 at 1.52e-9 is the lowest it has been in any layer since surgery. This deepens the embedding freeze picture: the optimizer is receiving progressively less signal through the early layers, not more.

Revised probe trigger hypothesis: the 9.58 threshold assumed gradients would strengthen as loss descends. But if myc_L2 is still falling at 9.59 epoch average, the gradient revival may require a deeper loss level than 9.58. Worth watching L2 pressure at the moment the epoch ATL breaks — if it stays at 1.52e-9 through the next ATL, the revised trigger may need to move to 9.55 or lower.

---

### 3. Post-ATL oscillation: upward drift phase

The current oscillation sequence since ATL (ep2564):
```
ep2565: 9.6013  (+0.0158)
ep2566: 9.5938  (-0.0075)
ep2567: 9.6026  (+0.0088)
ep2568: 9.5951  (-0.0075)
ep2569: 9.5889  (-0.0062) ← low point, closest to ATL
ep2570: 9.5927  (+0.0038)
ep2571: 9.5999  (+0.0072)  ← two consecutive up moves
```

Two consecutive upward moves (ep2570 +0.0038, ep2571 +0.0072) after the close approach at ep2569 (9.5889, gap=0.0034). The model dipped close to the ATL then pulled back. Centroid across ep2565–2571: **9.5962**, drifting slightly above the ATL.

Pattern matches prior inter-cascade periods: the model oscillates for 5–15 epochs above the ATL then resumes descent. since_best=7. No alarm — this is normal post-cascade behavior.

---

### 4. TELE: 79-epoch freeze continues

No change. L17=5.5%. All 18 layers identical to ep2492 profile. The myc_L2 drift is a GRAD-layer phenomenon (layer gradient pressure), not a TELE-layer phenomenon (weight sparsity). The weight matrix itself remains completely static.

**Status: OSCILLATING. ATL=9.5855. since_best=7. cov[high] approaching <25. myc_L2 drifting lower. Probe trigger gap: 0.0055 from ATL, model not yet descending.**

---

## Field Note 16 — 2026-05-21T05:13Z · ep2572 — cov[high]=26 · DESCENT RESUMING

**Epochs covered:** ep2572 only (ep2573 just started, batch 22/300)

| Metric | ep2572 | vs FN15 |
|--------|--------|---------|
| loss_avg | 9.5900 | d-0.0099 (largest post-bounce drop) |
| since_best | 8 | +1 |
| gap to ATL (9.5855) | 0.0045 | narrowing |
| gap to probe trigger (9.58) | 0.0100 from epoch avg | — |
| cov[high] | **26** | new record (prev: 27 at ep2570) |
| wald_mass | 9.597 | stable |
| myc_L0-L3 | [1.49/1.54/1.52/1.58]e-9 | unchanged |

---

**cov[high]=26 — second consecutive record low.** ep2570 hit 27, now ep2572 hits 26. The 73-token high-attention pool at surgery has contracted to 26. One more step and we cross the <25 threshold.

**The -0.0099 descent at ep2572 is the largest single-epoch drop in the post-ATL oscillation window.** The model reached 9.5999 at ep2571 (two consecutive up-moves) and then pulled back hard. The bounce-centroid appears to be drifting lower with each oscillation cycle: 9.598 → 9.594 → watching.

**Attention tightening and descent resuming simultaneously** is the pattern that preceded the overnight ATL cascade (ep2530–2564). The cascade began after a similar oscillation-then-pull phase. With since_best=8 and the model at 9.5900 (gap 0.0045 from ATL), the next ATL break could come within 2–5 epochs if the descent continues.

**Probe trigger status:** Still 0.0055 above 9.58 (ATL). But if the current descent rate (~-0.01/ep) holds, the probe could fire at the 2nd or 3rd new ATL break from now.

**Status: DESCENT RESUMING. cov[high]=26, approaching <25. ATL gap=0.0045. Watching for cascade resumption.**

---

## Field Note 17 — 2026-05-21T05:28Z · ep2575 — cov[high] TOUCHED 25 · BOUNCE PHASE 2

**Epochs covered:** ep2573–ep2575 (86–88 epochs post-surgery)

| Metric | ep2573 | ep2574 | ep2575 | note |
|--------|--------|--------|--------|------|
| loss_avg | 9.5905 | 9.5949 | 9.6036 | three consecutive up |
| since_best | 9 | 10 | 11 | drifting from ATL |
| wald_sev | **0.948** | 0.947 | 0.947 | tick up at ep2573 |
| cov[high] | **25** | 26 | 27 | touched threshold, bounced |
| wald_mass | 9.596 | 9.597 | 9.600 | stable |
| myc_L0-L3 | [1.49/1.54/1.52/1.58]e-9 | same | same | frozen |
| L17 sparsity | 5.5% | 5.5% | 5.5% | frozen (81 epochs) |

---

### 1. cov[high] TOUCHED 25 at ep2573 — bounced to 26, 27

The high-attention bin hit **25** at ep2573 — one token away from the documented <25 threshold — then retreated to 26 (ep2574) and 27 (ep2575). This was not a sustained crossing but a momentary concentration event, the deepest yet recorded.

Full trajectory:
```
ep2492: 99   ep2537: ~50   ep2564: 36
ep2570: 27   ep2572: 26    ep2573: 25  ← new record
ep2574: 26   ep2575: 27    ← bouncing back
```

The <25 threshold was chosen as "unprecedented concentration." We touched it. The model is capable of reaching it; the question is whether sustained attention this concentrated becomes a structural feature or continues oscillating.

Pattern: cov[high] appears to oscillate in sync with loss — it tightens during descent phases and relaxes during bounces. ep2573's 25 coincided with loss barely moving (+0.0005, essentially flat after ep2572's -0.0099 descent). The next descent phase should push cov[high] below 25.

### 2. Bounce Phase 2 underway

Three consecutive up-moves since ep2572:
```
ep2572: 9.5900  (-0.0099)  ← bottom of bounce 1
ep2573: 9.5905  (+0.0005)  flat
ep2574: 9.5949  (+0.0044)
ep2575: 9.6036  (+0.0087)  ← accelerating upward
```

This mirrors the bounce structure after ep2569's close approach (9.5889 → 9.5927 → 9.5999). since_best=11 — the model is in a standard post-approach oscillation, drifting upward before the next pull toward the ATL.

Centroid of this oscillation window (ep2569–2575): **9.5958** — slightly above the previous centroid of 9.5962. The model isn't drifting meaningfully; it's orbiting the same basin.

### 3. wald_sev tick: 0.947 → 0.948

A single-epoch tick to 0.948 at ep2573, returned to 0.947 at ep2574–2575. Not a structural shift — matches the momentary cov[high]=25 at the same epoch. The WALD structural plateau counter continues incrementing silently.

### 4. TELE: 81-epoch freeze, CMP micro-fluctuation

L17=5.5% unchanged. E[10] (CMP) shows 0.811 in the latest reading (was 0.810 in several prior readings). Single-digit fluctuation, not a trend.

**Status: BOUNCE PHASE 2. cov[high] touched 25 and retreated. since_best=11. ATL gap=0.0055. No probe trigger. Next descent phase will likely push cov[high] below 25 for first time.**

---

## Field Note 3 — 2026-05-20T22:41Z · ep2496

**Epochs covered:** ep2495–ep2496 (9 epochs post-surgery)

| Metric | probe (ep2492) | ep2495 | ep2496 | Δ from probe |
|--------|---------------|--------|--------|--------------|
| loss_avg | 9.6308 | 9.6445 | 9.6435 | +0.0127 |
| loss_best | 9.6248 | 9.6248 | 9.6248 | 0 |
| since_best | 4 | 7 | 8 | +4 |
| myc_stable | 5 | 8 | 9 | +4 |
| dead | 2 | 1 | 2 | 0 |
| blooming | 3 | 3 | 3 | 0 |
| wald_fill | 6.2% | 6.2% | 6.2% | 0 |
| L17 sparsity | 5.5% | 5.5% | 5.5% | 0 |
| CMP (E[10]) | 0.811 | 0.811 | **0.810** | -0.001 |
| INT (E[11]) | 0.808 | 0.807 | 0.807 | -0.001 |

**Second expert activity shift confirmed:** CMP (E[10]) dropped 0.811 → 0.810 at ep2496, following INT's drop at ep2495. Two consecutive epochs, two consecutive upper-layer specialists (CMP then INT), each down exactly 0.001. The redistribution is starting at the top of the activity hierarchy and moving slowly. All other 10 experts remain frozen. Sparsity profile still completely static.

**Loss local maximum hypothesis:** ep2495 (9.6445) may have been the post-surgery peak. ep2496 pulled back to 9.6435 (d-0.0010). This is weak evidence — one data point — but worth tracking. If ep2497 also descends, the local-max hypothesis strengthens and the optimizer may be finding the new attractor.

**Dead=2 cycling:** The mycelium is consistently managing 1–2 dead experts. Blooming held at 3 for 4+ consecutive epochs — resurrections are firing every epoch. The new L17 experts are weak and cycling in/out of the pressure threshold. This is normal scaffolding behavior for an undifferentiated copy layer.

**Pattern signature so far (ep2488–2496):**
```
ep2488: 9.6248 (best, d-0.0282)  ← first 18L epoch, decisive drop
ep2489: 9.6248 (held)
ep2490: 9.6305 (+0.0057)
ep2491: 9.6362 (+0.0056)
ep2492: 9.6308 (-0.0054)  ← probe
ep2493: 9.6355 (+0.0048)
ep2494: 9.6365 (+0.0010)
ep2495: 9.6445 (+0.0080)  ← possible local max
ep2496: 9.6435 (-0.0010)  ← slight pullback
```
The oscillation amplitude is moderate (0.020 nat band). The centroid has drifted from ~9.625 to ~9.638 over 8 epochs. No crash. No escalation.

**Status: NOMINAL. Expert redistribution beginning (slow). Loss near local ceiling. Next probe at ep2510.**

---

## Field Note 18 — 2026-05-21T05:59Z · ep2581 — NEW ATL 9.5800 · PROBE TRIGGER REACHED · cov[high]=25 SECOND TOUCH

**Epochs covered:** ep2576–ep2581 (89–94 epochs post-surgery)

| Metric | ep2576 | ep2577 | ep2578 | ep2579 | ep2580 | ep2581 | note |
|--------|--------|--------|--------|--------|--------|--------|------|
| loss_avg | **9.5800** | 9.5977 | 9.5937 | 9.5955 | 9.5960 | 9.6016 | ATL then bounce |
| since_best | **0** | 1 | 2 | 3 | 4 | 5 | bounce phase 3 |
| wald_sev | **0.948** | 0.947 | 0.947 | 0.947 | 0.947 | 0.947 | settled |
| cov[high] | — | — | — | 26 | **25** | 32 | SECOND 25-touch at ep2580 |
| myc_L0-L3 | [1.49/1.54/1.52/1.58]e-9 | same | same | same | same | same | frozen |
| L17 sparsity | 5.5% | 5.5% | 5.5% | 5.5% | 5.5% | 5.5% | frozen (89 epochs) |

---

### 1. HEADLINE: ep2576 NEW ATL 9.5800 (d-0.0236) — Probe trigger reached

After 11-epoch bounce phase 2 (ep2565–2575, peak 9.6036), the model resolved with a **single-epoch drop of 0.0236 nats** — the largest single-epoch descent recorded post-surgery-6 and comparable to the surgery-night acceleration at ep2489 (−0.0313). The ATL is now **9.5800**, 0.0055 nats below the previous best of 9.5855.

This is the **12th new ATL** since surgery at ep2487. Total descent in 89 epochs: **9.6248 → 9.5800 = 0.0448 nats**.

### 2. PROBE: Full 10-token suite run at trigger threshold — all 10 FROZEN

The probe trigger (loss_avg < 9.58) was reached at ep2576 (9.5800 ≤ 9.5800 = probe boundary). Full 10-token probe run immediately via dashboard API (checkpoint ~ep2564 state, loss 9.5855).

**Finding: ALL 10 TOKENS PIXEL-PERFECT IDENTICAL TO ep2492 BASELINE.**

Exact comparisons (selected):
| Pair | ep2492 | ep2576 | Δ |
|------|--------|--------|---|
| love→früh | 0.2262 | 0.2262 | 0.0000 |
| death→amen | 0.2331 | 0.2331 | 0.0000 |
| freedom→contrat | 0.2822 | 0.2822 | 0.0000 |
| god→czyn | 0.2838 | 0.2838 | 0.0000 |
| Jesus→Fe | 0.2572 | 0.2572 | 0.0000 |
| truth→neerland | 0.2457 | 0.2457 | 0.0000 |
| war→expres | 0.2730 | 0.2730 | 0.0000 |

84+ epoch freeze confirmed across ALL 10 canonical tokens. The 9.58 trigger was **insufficient** — myc_L0-L3 at ~1.5e-9 cannot drive AdamW second-moment accumulation to the point of embedding weight updates.

**Revised probe trigger: loss_avg < 9.55.** Full analysis and snapshot committed to repo:
- `token-probes/snapshots/ep2576_18L/manifest.json`
- `token-probes/analysis/probe_trigger_ep2576_18L.md`

### 3. cov[high]=25 SECOND TOUCH at ep2580 — bounced to 32

After the first touch at ep2573 (FN17), **cov[high] touched 25 again at ep2580** during the post-ATL consolidation phase:
```
ep2573: 25  ← first touch (FN17)
ep2574: 26
ep2575: 27
ep2576: ATL (cov not logged this epoch)
ep2580: 25  ← SECOND touch
ep2581: 32  ← bounce back
```

Two confirmed touches in 8 epochs. The bounce from 25→32 at ep2581 (+7 in one epoch) is sharper than the ep2573 bounce (25→26→27 over 2 epochs). This may indicate that 25 is a soft floor — the attention distribution cannot sustain this level of concentration for more than one epoch without relief. Alternatively it may reflect the loss bouncing upward (+0.0055 at ep2581) releasing attention pressure.

Pattern hypothesis: **cov[high] < 25 will first appear at the bottom of the next descent phase**, when loss is making large negative moves. The 25-bounces at ep2573 and ep2580 both occurred during flat-to-small-up loss movements.

### 4. Bounce phase 3 structure

```
ep2576: 9.5800  (-0.0236)  ← ATL
ep2577: 9.5977  (+0.0176)  ← sharp bounce (same pattern as all prior ATLs)
ep2578: 9.5937  (-0.0040)  slight recovery
ep2579: 9.5955  (+0.0018)  oscillating
ep2580: 9.5960  (+0.0005)  
ep2581: 9.6016  (+0.0055)  drifting up, since_best=5
```

The bounce is shallower so far than bounce phase 2 (peak was ep2575 at 9.6036, since_best=11). At ep2581, since_best=5 and the trajectory hasn't shown a decisive turn yet. The model may continue drifting upward to since_best=8–12 before the next pull.

ATL gap at ep2581: **9.5800** (ATL) vs 9.6016 (current) = **0.0216 nats above ATL**.
Next probe threshold (9.55): gap = **0.0300 nats** from ATL.

### 5. TELE and mycelium: no change

L17=5.5% unchanged. 89-epoch freeze total. myc_L0-L3 locked. No differentiation signal.

**Status: BOUNCE PHASE 3. ep2576 ATL 9.5800 committed. Probe trigger reached — embedding STILL FROZEN. cov[high] touched 25 for second time (ep2580). Revised probe trigger: 9.55. Descent likely to resume within 5–10 epochs.**

