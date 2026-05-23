# albert. overnight watch log
# Format: FN<n> · ISO timestamp · ep context · observations

---

**FN29 · 2026-05-23T09:30Z** *(training.log — ep3060 confirmed, ep3061 b297/300)*

Post-ATL orbit: ep3058=**9.4526** (ATL) → ep3059=9.4603 (d+0.0077, bounce) → ep3060=9.4585 (d-0.0018, returning). since_best=2. Normal post-ATL pattern — model re-approaching ATL territory.

**WALD plateau 18** (was 16 at FN28 — two ticks in two epochs). mass=9.448 stable (stopped ticking). Surgery gate soft-lower=~20, now **2 epochs away**. MYCELIUM ep3060: dead=0, blooming=2, myc_stable=19 — expert deaths have stopped since resurrections at ep3057. Governor still holding (since_best=2, plateau not at threshold). ep3061 b297/300 at 09:30Z.

---

**FN28 · 2026-05-23T09:19Z** *(training.log — ep3058 close confirmed, ep3059 in progress)*

**★ Fourth ATL break: ep3058 avg=9.4526** (d-0.0028) — FN27 "descent slowing" was premature. Cascade continues: 9.4557→9.4556→9.4554→**9.4526**. since_best=0. Slope steepened again from d-0.0003 to d-0.0028.

WALD plateau count **16** (up from 15). mass=**9.448** (tick sequence: 9.451→9.450→9.449→9.448 across last 4 epochs — steady structural erosion). dead_low=6.25 experts, dead_high=5.25 experts. **Surgery gate ~20–30 epochs out; at this rate (1 tick/epoch) ~4 more ATL breaks before gate fires.** Governor holding correctly (since_best=0). hot=L17, cold=L0. ep3059 b99/300 in progress.

---

**FN27 · 2026-05-23T09:15Z** *(training.log — ep3057 close confirmed, ep3058 b255/300)*

ep3057 close confirmed: avg=**9.4554** (d-0.0003), since_best=0. Three-epoch ATL cascade complete: 9.4557→9.4556→9.4554. Descent slowing (d-0.0022 → d-0.0064 → d-0.0003) — possible mini-plateau forming.

**Fourth resurrection round at ep3057:** L2E11←L2E0 and **L8E10←L8E0** — pattern shifted. Previously only E11 dying; now E10 dead in L8. Two expert slots (10 and 11) across multiple layers (L0,L2,L3,L8) being chronically starved of routing. WALD: "15 stable epochs, sev=0.969 mass=9.449" → amplify OFF. **Plateau count 15 — approaching surgery gate (~20–30).** Governor holds while since_best=0. ep3058 b255/300, losses 9.45–9.50, ETA ~50s. TTL step 4800: L9 G4, L15 G4, L17 G3 most active; L0 all-orange (cold).

---

**FN26 · 2026-05-23T09:11Z** *(training.log — ep3057 ATL BREAK #3)*

**★ Third ATL break: ep3057 avg=9.4554** — cascade: 9.4557 → 9.4556 → **9.4554**. since_best=0. Model in sustained descent. WALD mass still dropping (9.449 at ep3056, trajectory continues). dead=2 returned (myc_stable=16, blooming=4) — fourth resurrection round imminent at ep3057 close. Watching for ep3058 summary.

---

**FN25 · 2026-05-23T09:07Z** *(training.log — ep3056 ATL BREAK #2, ep3057 b275/300)*

**★ Second consecutive ATL break: ep3056 avg=9.4556** (d-0.0064) — beats ep3054 (9.4557) by 0.0001. since_best=0. Model in active descent, not orbiting. WALD mass=**9.449** (new run low, dropping every epoch: 9.453→9.451→9.450→9.449). myc_stable=15, dead=0. ep3057 b275/300 at 09:07Z — opening batch 9.3196, b274 loss 9.3980 — running well below ATL at batch level. Third consecutive ATL break possible at ep3057 close (~30s).

---

**FN24 · 2026-05-23T09:00Z** *(training.log — ep3054 ATL BREAK, ep3055 complete, ep3056 opening)*

**★ NEW ATL: ep3054 avg=9.4557** (d-0.0022) — breaks ep3041 (9.4566) by **0.0009 nats**. since_best=0. Best checkpoint saved. 13-epoch orbit (ep3041→3054) finally resolved as predicted. Approach sequence: 9.4603→9.4680→9.4662→9.4605→9.4642→9.4580→**9.4557**.

WALD at break: sev ticked 0.968→**0.969**, mass=**9.451** (new run low). Plateau count logged as 12 stable epochs → amplify still OFF (governor held correctly). ep3055 post-ATL bounce avg=9.4620 (d+0.0063), since_best=1 — normal. WALD mass=**9.450** (another tick down). myc_stable=14, dead=0, blooming=3. ep3056 b0/300 opening at 9.5251.

---

**FN23 · 2026-05-23T08:48Z** *(training.log — ep3053 complete, ep3054 opening)*

ep3053 avg=**9.4580** (d-0.0062), since_best=13. **Gap to ATL now 0.0014 nats** — new closest approach in this run (prev record ep3048: 0.0037). ATL break is imminent; ep3054 needs only a d-0.0015 descent to set a new server-side ATL.

Trajectory tightening hard: 9.4642 → **9.4580**. WALD mass=**9.453** — new run low (9.455→9.453 across ep3052→3053), structural descent confirmed. myc_stable=12, dead=0, blooming=4 — no resurrections this epoch. Amplify still OFF. ep3054 b0/300 opening at 9.4939. Watching closely.

---

**FN22 · 2026-05-23T08:44Z** *(training.log — ep3052 complete, ep3053 b188/300)*

ep3052 avg=9.4642 (d+0.0037), since_best=12. Small bounce — no new resurrections, myc_stable=11, dead=0. WALD mass=**9.455** (new low for this run, prior floor was 9.456). Orbit band has compressed: was 9.467–9.472 at FN17, now oscillating 9.460–9.468 — model descending in structure even without ATL probe. ep3053 b188/300, ETA ~2 min.

---

**FN21 · 2026-05-23T08:37Z** *(training.log — ep3051 complete, ep3052 b130/300)*

ep3051 avg=**9.4605** (d-0.0057), since_best=11. **Gap to ATL now 0.0039** — nearly matching ep3048's closest approach (0.0037). Orbit clearly contracting: 9.4603 → 9.4680 → 9.4662 → **9.4605**. ATL break looking probable in next 1–3 epochs if descent holds.

**Third resurrection: L8E11 ← L8E0** (σ=0.050) at ep3051 close. Pattern: Expert 11 slot dead in L0, L3, L8 in sequence — E11 is a chronically under-routed expert across early–mid layers. Resurrections are firing correctly; myc_stable=10, blooming=4, dead cleared each time. WALD explicitly logged "structural plateau (9 stable epochs)" → amplify OFF. mass=9.456 stable. ep3052 b130/300 in progress; L9 G4 most active. Routing H=2.467, balanced.

---

**FN20 · 2026-05-23T08:28Z** *(training.log — ep3049+3050 complete, ep3051 b10/300)*

ep3049 avg=9.4680 (d+0.0077) — expected post-resurrection bounce (L0E11/L3E11 noise). ep3050 avg=**9.4662** (d-0.0018) — descent immediately resumed. since_best=10. Orbit now: 9.4603 (dip) → 9.4680 (bounce) → 9.4662 — contracting back toward the 9.46 floor. No ATL; gap to ep3041 best (9.4566) = 0.0096.

myc_stable rising: 7→8→9 across ep3048–3050. blooming=4. dead=0 confirmed (resurrections held). WALD mass ticked to 9.459 (small perturbation from resurrection, prior was 9.455–9.456). sev/fill unchanged. ep3051 opening at 9.5006 (normal). WALD plateau count: 10 — halfway to surgery gate (~20–30).

---

**FN19 · 2026-05-23T08:23Z** *(training.log — ep3048 complete, ep3049 b222/300)*

ep3048 avg=**9.4603** (d-0.0071), since_best=8. **Closest ATL approach since break: gap now 0.0037 nats** (ATL=9.4566 ep3041). Model resuming descent after 5-epoch plateau; if ep3049 holds this trajectory, ATL probe is imminent.

**First mycelium resurrections this run:** MYCELIUM resurrected L0E11←L0E0 and L3E11←L3E0 (both σ=0.050) at ep3048 close. dead=2 → repaired. blooming=4, myc_stable=7. Expert 11 seeded in L0 and L3 from their dead Expert 0 slots — structural repair underway, even though myc_L0-L3 pressure still reads zero (EMA not yet warm enough to register).

WALD acknowledged: "structural plateau (6 stable epochs, sev=0.968 mass=9.456) → **amplify OFF**" — surgery governor correctly withheld while descent was active. sev/fill/mass unchanged. LB aux loss still disabled; LB value drifting slightly lower (55.6 vs prior 56.4). ep3049 b222/300, opening batch 9.3532 — below ATL at batch level. Watch ep3049 close.

---

**FN18 · 2026-05-23T08:16Z** *(training.log — ep3047 complete, ep3048 b136/300)*

ep3047 avg=**9.4674** (d-0.0046), since_best=7. No ATL — gap to ep3041 best (9.4566) now 0.0108 nats. Orbit ep3042→3047: 9.4653 · 9.4688 · 9.4677 · 9.4682 · 9.4720 · **9.4674** — tight band, no probe signal yet.

**Mycelium blooming emerging:** ep3045 blooming=0 → ep3046 blooming=2 → ep3047 blooming=3. Structural EMA establishing in early layers (dead=0 in all, hot=L17/cold=L0 unchanged). This is the governor precondition beginning to close — still small (3/18 layers), but not zero anymore. WALD fill=4.2%, mass=9.456, sev=0.968 locked. LB loss disabled (auxiliary penalty off). Routing H=2.467, E balanced. TTL step 1930: L9 briefly G5/O6/R1 — most confident layer seen this run.

---

**FN17 · 2026-05-23T08:07Z** *(training.log — ep3042–3046 complete, ep3047 opening)*

ep strip (post-ATL bounce): 9.4653 · 9.4688 · **9.4677** · 9.4682 · 9.4720. Best still ep3041 **9.4566**, since_best=6. No ATL probe — model orbiting 9.467–9.472, no descent signal. Pattern mirrors FN11 (20-epoch plateau at 9.47–9.48 after ep3014 ATL); since_best=6 is early — surgery threshold ~20–30.

WALD locked: sev=0.968, fill=4.2%, mass 9.455–9.458 (tiny oscillation, essentially flat). hot=L17, cold=L0 — unchanged. Myc L0–L3 still zero (structural EMA not established). TTL fully warmed by step 1470: all layers carrying G/O/R mix (L6/L10 double-R most cautious; L7/L15/L17 triple-G most confident). Routing H=2.4669, E=0.080–0.087, balanced. LB=56.1–56.7. ep3047 b3/300 in progress.

---

**FN16 · 2026-05-23T07:51Z** *(epoch_history.log — ep3042+3043 close)*

ep3042 avg=**9.4653** (d+0.0087) · ep3043 avg=**9.4688** (d+0.0035). No new ATL — best holds at ep3041 **9.4566**, since_best=3. Typical post-restart bounce; model slightly above ATL floor, needs 1–2 more epochs to recapture.

**WALD sev jumped to 0.968** (was 0.926–0.928 at FN11) — fill 4.2% stable, mass 9.456–9.458. Governor accumulating but since_best=3 far from surgery threshold (~20–30). hot=L17, cold=L0 (gradient weight concentrating in final block). Myc L0–L3 all zero — consistent with block gradient near-zero (only lm_head 1.85e-3 visible; structural EMA not established yet). TTL warmed up by step 620: multiple G/R tokens per layer (L4 G4/O7/R1 most active). ep3044 b24/300 in progress.

---

**FN15 · 2026-05-23T07:37Z** *(training log + dashboard — post-crash restart)*

Training resumed ep3042. Crash from previous attempt fixed (index-out-of-bounds in TTL burst loop: `layer_norms` expanded to 20 entries but `grad_norm_ema` was 18-wide — fixed with `.take(config.num_layers)`). Build confirmed live. ep3042 b39/300, loss 9.42, LR 2.96e-4. Routing: H=2.4678, E=0.081–0.089 (balanced, consistent with FN14). TTL reset to warmup (all G0/O12/R0 — runtime EMA not checkpointed, bootstraps each run). Per-layer gradient norm display confirmed working: global gg=0.0022, L8–L9 dominant orange bars, emb+lm_head bars visible. Expert activity: CMP 100% / INT 81% / PLN 59% / ABS 39% — structural cluster dominant. No new ATL (ep3042 mid-epoch). Dashboard chart dips (ep300–700, loss 8.0–8.5 pre-surgery) causing y-axis inflation (4.1–13.6); preseed loss floor fix queued.

---

**FN14 · 2026-05-23T07:14Z** *(training log + dashboard — post-restart observation)*

New build deployed: BATCH_SIZE=8 + F16 attention matmuls (tensor cores). Immediate effect — batch losses dropped from 9.55–9.60 plateau to **9.33–9.49** range within first 20 batches of ep3040. Rapid ATL cascade: ep3040 b20 → 9.3305; ep3041 b288 dashboard ATL **9.2384** (d-0.319 from pre-restart CSV ATL 9.5564). Epoch-avg at ep3041 close: **9.4566** (new server-side ATL, prev ~9.4605 at ep3014). Routing stable: H=2.467, E=0.079–0.088 (balanced). TTL warming: L12 G4/O7/R1 most confident. Training stopped at ep3042 b10 for checkpoint pull + clean restart with fixed build (gradient norm telemetry + SMAS CSV threshold fix). CSV coverage for ep3040–3041: only 28 rows captured (SMAS bug — sub-9.5 batches dropped by old _LOSS_MIN=9.5 guard; fix deployed, active on restart). Manual download data to be stitched on next albert-train preflight.

---

**FN13 · 2026-05-23T06:35Z** *(batch_history.csv — local sync)*

ep3039 in progress (57 sampled batches), CSV avg **9.5564** — new tracking-window low, below previous floor ep3020 (9.5607). Single-epoch delta d-0.0149 is steepest descent observed since FN12. Sequence ep3036→3039: 9.5784 → 9.5692 → 9.5713 → **9.5564**. Server-side estimate ~9.476; ATL threshold requires ~9.540 CSV (still 0.016 above). Mid-epoch — watch ep3039 close. If it holds below 9.560 this would be the first genuine descent below the plateau band.

---

**FN12 · 2026-05-23T06:15Z** *(batch_history.csv — local sync)*

ep3035 in progress (~230/300 batches). CSV orbit ep3028→3035: 9.5638–9.5736. No new ATL probe below ep3031 (9.5638 CSV ≈ 9.484 server-side; ATL requires ≈9.540 CSV). since_best≈27, steady. No WALD event. WALD sev/fill/myc unchanged from FN11 (no Modal volume pull this tick — stale local epoch_history.log). Pattern: same tight orbit as FN11, no signal of break imminent.

---

**FN11 · 2026-05-23T05:37Z** *(epoch_history.log — Modal volume pull)*

ep3028, since_best=20. **Epoch-log ATL confirmed: 9.4605 at ep3014** — significantly better than the ~9.469 estimated from dashboard FN10 (dashboard strip lagged real ATL by ~10 epochs). Post-ATL orbit over 20 epochs: 9.47–9.48, highly stable.

Epoch strip ep3014→3028: 9.4608 · 9.4614 · 9.4742 · 9.4619 · 9.4803 · 9.4715 · **9.4694** · 9.4770 · 9.4717 · 9.4755 · 9.4803 · 9.4814 · 9.4799 · 9.4746 · **9.4748** — oscillating without probing deeper.

WALD: sev 0.926–0.928, fill 6.2% (stable). Mycelium L0–L3 locked at 1.48–1.57e-9 (structural baseline). hot=L10 (shifted from L17 noted at FN10), cold=L7.

Pattern: 20-epoch plateau without ATL probe. WALD sev ticking down very slightly (0.928→0.926) — could be governor accumulating. since_best=20 is approaching where past WALD events have fired. Watch for amplify trigger in next 10–20 epochs.

---

**FN10 · 2026-05-23T05:06Z** *(dashboard read — direct observation)*

ep3024, batch 142/300. **9.5 hard floor broken overnight.** Dashboard EP AVG **9.4755**; recent epoch strip: 9.4755 · 9.4717 · 9.4770 · **9.4694** (new server-side ATL visible). Model consolidated below 9.50 — the floor that held since 18L surgery is behind it.

Expert activity: CMP **100%** · INT 85% · PLN 79% · ABS 67% · LNG 59% · LOG 12% · SEM 15%. Abstract/planning cluster dominant; CMP fully saturated suggests structural work in progress. TTL routing: G 6% / **O 83%** / R 3% — mostly deliberating, low certainty. Gradient hot at L8 (red bar), L17 active.

CSV ATL (batch-level) ep3004 9.5561 unchanged — dashboard server ATL ~9.469 (9.4694 epoch avg, likely latest best).

Pattern: broke floor, now orbiting 9.47–9.48. No indication of WALD surgery yet (amplify state not visible). Next watch: whether orbit tightens below 9.46 or stabilises here.

---

**FN9 · 2026-05-23T04:50Z**

**New CSV ATL: ep3004 at 9.5561** — breaks ep2957 (9.5592) by 0.0031. Estimated epoch-close ≈ **9.476** (~0.08 offset), new server-side ATL (prev ~9.479 at ep2957).

Training progressed ~30 epochs since FN8 (ep2973→ep3021). Post-ep3004 pattern: oscillating in a tight 9.56–9.57 band with multiple near-ATL probes:
ep3009: 9.5575 | ep3011: 9.5623 | ep3014: 9.5598 | ep3017: 9.5629 | ep3020: **9.5607**

ep3020 closed at 9.5607 (gap to ATL: 0.0046) — another close approach. ep3021 in progress (~114/300 batches), running avg 9.5727 (post-ATL bounce).

Local minima not systematically descending post-ep3004 — oscillating rather than probing deeper. Plateau may be forming. WALD/routing unobservable locally.

---

**FN8 · 2026-05-22T23:30Z**

ATL still ep2957 (9.5592). ep2965 closed at **9.5632** — second closest CSV approach ever, gap 0.0040. Did not break. ep2966 mini-bounce to 9.5703 (+0.0071), now descending again: ep2967 at 9.5668 (~96/300 batches).

Orbit tightening: successive near-ATL values 9.5646 (ep2955) → 9.5632 (ep2965), each approach closer than the last. Pattern identical to the ep2938→2946→2951 sequence that preceded the ep2957 ATL break.

Estimated epoch-close for ep2965: ~9.483 (~0.08 offset). If confirmed, new server-side ATL within the next 2–5 epochs.

---

**FN7 · 2026-05-22T23:16Z**

ATL unchanged: ep2957 CSV 9.5592. Post-ATL bounce collapsing faster than prior cycles — peak ep2959 at 9.5831 (+0.0239), then two consecutive descents: ep2963 9.5718 → ep2964 **9.5681** (d-0.0037). Gap to ATL now **0.0089 nats**. ep2965 in progress (20 batches), running avg 9.5664 — already inside the 0.01 margin.

Pattern: shorter bounce, faster return. If ep2964/2965 continue, new ATL probe imminent within 2–4 epochs.

---

**FN6 · 2026-05-22T22:53Z**

ep2957 closed at **CSV 9.5592** — new CSV ATL, breaking ep2955 (9.5646) by 0.0054. Estimated epoch-close ≈ **9.479** (~0.08 offset). If the offset holds, this is a new server-side ATL (prev est. ~9.484 at ep2955, server ATL was 9.4873 at ep2938).

Post-ATL bounce underway: ep2958 +0.0120 → ep2959 +0.0120 → ep2960 -0.0124 → ep2961 +0.0030 (107 batches, in progress). Oscillating, not yet decisive pull-down.

Descent rate: two consecutive CSV ATLs in ~30 epochs (ep2955: 9.5646 → ep2957: 9.5592). Tightening orbit pattern continuing.

WALD/routing/mycelium: unobservable locally. No batch anomalies (range 9.50–9.71).

---

**FN5 · 2026-05-22T22:27Z**

ep2955 closed at **CSV 9.5646** — new CSV-level ATL (prev: ep2946/ep2949 shared 9.5668). Applying consistent ~0.08 offset between CSV-avg and epoch-close (calibrated on ep2938/2946/2949/2951), estimated epoch-close ≈ **9.484** — likely a new server-side ATL (previous: 9.4873 at ep2938). Cannot confirm from Modal logs (epoch_history.log not synced past ep1356).

ep2956 in progress (~202/300 batches), CSV running avg 9.5770 (+0.0124 bounce from ep2955). Normal post-ATL bounce pattern.

Trajectory since FN4 (ep2951 CSV 9.5728):
ep2952: 9.5720 → ep2953: 9.5851 (bounce) → ep2954: 9.5736 → ep2955: **9.5646** (ATL) → ep2956: 9.5770 (in progress)

WALD/routing/mycelium: unobservable (Modal logs not synced locally). Batch loss range ep2952–2956 nominal (9.50–9.71), no anomalies. `amplify` state unknown.

---

**FN4 · 2026-05-22T22:01Z**

ep2951 closed at **9.4887** (d-0.0072) — second closest approach to ATL ever (ep2946 holds the record at 0.0012; ep2951 is 0.0014). ep2952 in progress (~215/300).

WALD structural plateau now **13 consecutive epochs**, mass **9.495** (was 9.499 at ep2950 → dropping). Mass descent within the plateau confirms the model is genuinely moving down, not frozen. `amplify OFF` — governor sees active descent, surgery withheld correctly. `since_best=13`.

Mycelium L0–L3 locked at 1.49–1.56e-9 (stable). hot=L5, cold=L7 consistent.

Pattern: repeated close ATL orbits tightening. ep2938 ATL 9.4873 → ep2946 9.4885 → ep2951 9.4887. The gap each orbit: varies but descent trend clear in WALD mass.

---

**FN3 · 2026-05-22T21:46Z**

ep2949 closed at **9.4927** (d-0.0066). Two-epoch mini-bounce (ep2947: 9.4937, ep2948: 9.4993) followed by descent resuming. Pattern mirrors post-ATL behaviour. No new ATL — 9.4873 holds.

Structural plateau WALD now **11 consecutive epochs**, mass 9.499, sev=0.923. Governor accumulating but descent active → surgery still BLOCKED.

Trajectory since FN2: 9.4885 → 9.4937 → 9.4993 → **9.4927**. Gap to ATL: 0.0054 nats. ep2950 opening.

---

**FN2 · 2026-05-22T21:30Z**

ep2946 closed at **9.4885** (d-0.0045) — **0.0012 nats from ATL 9.4873**. Closest approach since the ATL break at ep2938. ep2947 just opened (first batch 9.6563, normal).

Structural plateau WALD now **8 consecutive epochs** (was 7), mass 9.500, sev=0.923, scale=44.4×. Plateau count accumulating but descent is active — surgery governor will withhold while loss is moving down.

Trajectory ep2943→2946: 9.5032 → 9.4998 → 9.4930 → **9.4885**. Four straight epochs of descent. ATL probe in progress.

---

**FN1 · 2026-05-22T21:25Z**

ep2946 in progress (batch ~16/300). ep2945 closed at **9.4930** (d-0.0068) — closest approach since ATL 9.4873 (ep2938). Gap to ATL: **0.0057 nats**.

Post-ATL bounce trajectory: 9.5026 (ep2939) → 9.4980 → 9.4941 → 9.4948 → 9.5032 (minor bounce ep2943) → 9.4998 → **9.4930** (ep2945). Descent resuming.

Structural plateau WALD firing every epoch since ep2943: 5→6→7 consecutive stable epochs, mass holding 9.500–9.503, sev=0.923, scale=44.4×. Governor accumulating plateau count — surgery gate logic watching this.

No new ATL. Batch losses ep2946 nominal (9.51–9.71), no anomalies. Routing not sampled this tick (CSV ahead of epoch boundary).
