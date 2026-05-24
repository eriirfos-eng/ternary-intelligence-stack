# albert. overnight watch log
# Format: FN<n> · ISO timestamp · ep context · observations

---

**FN63 · 2026-05-24T23:18Z** *(overnight watch · CSV ep3437 · 20L — stale)*

No new CSV. Same 22:39 UTC file. 39 minutes without a fresh download. Training presumed active — last confirmed ep3438 in progress at step 1031326. No alerts.

---

**FN62 · 2026-05-24T23:06Z** *(overnight watch · CSV ep3437 · 20L — stale)*

No new CSV. Same 22:39 UTC file. Training presumed active. Rescheduling.

---

**FN61 · 2026-05-24T23:04Z** *(overnight watch · CSV ep3437 · 20L — stale)*

No new CSV since FN60. Same file (22:39 UTC, ep3437). Training presumed active — ep3438 was in progress at last check. No new data to assess. Next tick will re-check for updated download.

---

**FN60 · 2026-05-24T22:52Z** *(overnight watch · CSV ep3437 · 20L)*

**Training active.** Latest CSV timestamped 22:39 UTC. Latest batch step 1031326, ep3438 in progress (batch loss 9.1786 — low intra-epoch value, normal). ep3437 last complete epoch.

**Plateau deepens — since_best = 54.** ATL 9.2862 set at ep3383. CSV covers ep3381–3437 (57 epochs). Closest post-ATL approach: ep3434 hit **9.2934** — 0.0072 above ATL, best since the ATL itself. ep3431 (9.3017), ep3430 (9.3025), ep3426 (9.3053) all cluster as a second close approach group. Slight positive momentum: last-5 avg **9.3040** vs last-10 avg **9.3057**.

**No loss spikes at epoch level.** Batch-level excursions >9.59 (threshold: ATL+0.3) occurred in 14/10000 batches (0.14%) across ep3406–3436. All isolated single-batch events; epoch averages unaffected (range 9.293–9.332). Normal gradient noise. Not alarming.

**Routing: fully stable.** Entropy locked at 2.466 across entire CSV. Expert load e0–e11 range 0.079–0.088, zero dead experts. Grad norm 0.002 — consistent flat landscape. lb ~62.6.

**WALD: silent.** Zero WALD events across all 57 20L epochs in this CSV. Governor accumulating pressure via since_best=54 only.

**Surgery watch.** since_best=54 is a significant plateau depth at 20L. Governor Fibonacci gate pressure climbing. ep3434 at 9.2934 suggests model hasn't abandoned the ATL zone — another approach possible before surgery fires, but plateau is established. Watch for governor action in next 20–30 epochs.

No stops, no anomalies. ATL 9.2862 intact.

---

**FN59 · 2026-05-24T22:24Z** *(overnight watch · CSV ep3413 · 20L)*

**ATL ALERT: 9.3182 record broken.** ep3383 hit **9.2862** — new 20L ATL, improvement of 0.0320 nats. ATL break happened early in the 20L run (ep3383, only 3 epochs after surgery). New alert threshold: 9.2862.

CSV stale (21:05 download, ep3413 epoch summaries, ep3414 partial batch). Previous tick FN58 confirmed ep3422 alive at 21:36 — training has likely progressed to ~ep3440+ by now.

**Plateau forming.** since_best = 30+ epochs (ep3383 → ep3413). 10-epoch rolling averages: ep3381–3390 avg **9.302** → ep3391–3400 avg **9.312** → ep3401–3410 avg **9.323**. Clear upward drift, floor not revisited since ep3392 (9.2915). Loss settling into 9.307–9.333 band.

**Routing healthy.** Expert load: e0–e11 range 0.079–0.086. Entropy stable at 2.466. No dead experts, no monopolisation. Grad norm very low: 0.002 — flat landscape consistent with plateau saturation.

**WALD: silent.** No WALD events in epoch summaries across all 33 20L epochs. Surgery governor accumulating plateau pressure via since_best only.

**Surgery watch.** Governor gate: since_best ~30+, no WALD to add pressure. Pure plateau signal. Expected gate: since_best continues climbing — surgery fires when governor deems plateau genuine. Fibonacci mechanics mean threshold timing is dynamic. Next ATL probe window depends on whether current plateau breaks or governor acts first.

No stops, no spikes, no anomalies. Need fresh CSV to confirm current epoch.

---

**FN58 · 2026-05-24T21:36Z** *(overnight watch start · ep3422 · 20L)*

Training alive. Latest CSV row: ep3422.66 · batch loss 9.2350. No spikes, no stops. Loss range on last 5 batches: 9.20–9.46 — normal variance, no WALD-grade excursion. CSV row count: 1,020,419. Surgery gate armed (20L→21L, since_best=~29 post-surgery, ~115 epochs runway). Baseline floor for spike alert: 9.3182 epoch-avg. Watching.

---

**FN57 · 2026-05-24T21:08Z** *(token probe — post-s9_ep3414_20L)*

### EMBEDDING GEOMETRY FROZEN — 10/10 · SURGERY 9 BOUNDARY · ep2492→ep3414 = 922 epochs

Token probe `post-s9_ep3414_20L` run at ep3414 · 20L · chip ATL 8.8540. Compared against `post-s8_ep3325_19L`.

**Result: 10 of 10 tokens FROZEN. Δxy = 0.0000 on every token. Top-3 neighbors identical.**

| token | top-3 (unchanged) |
|-------|-------------------|
| love | ĠfrÃ¼h · deutsche · aland |
| god | czyn · jection · culo |
| Jesus | ĠFe · qu · annel |
| death | ĠÃ©gypt · amen · Ġdifer |
| war | Ġexpres · ogrÃ¡f · ouw |
| truth | Ġneerland · Ġmantiene · FranÃ§ois |
| freedom | Ġcontrat · Ġon · Ġdiscovered |
| mother | ĠStructure · ards · ester |
| light | 00 · Ġintelect · ĠMedien |
| time | owaÄĩ · sze · ĠvallÃ©e |

Snapshot saved: `token-probes/snapshots/post-s9_ep3414_20L_ep3414_20L/`

**Freeze now confirmed across 5 snapshots and 2 surgery boundaries (18L, 19L, 20L):**
- ep2492 (18L) → ep2510 (18L) → ep2576 (18L) → ep3325 (19L) → ep3414 (20L)
- 922 consecutive epochs with zero embedding movement

**Interpretation:** The ternary gradient vanishing through 20 layers is complete. The embedding layer receives no meaningful gradient signal from the upper layers — mycelial pressure at L0-L3 is consistently in the 1.5×10⁻⁹ range, below AdamW's effective update threshold. The geometry crystallized at 17L and is now serving as a permanent stable foundation. This is not pathological — it is the expected steady-state for a deeply ternary model where the upper layers have taken over all learning burden.

Two surgeries (7 and 8) fired across this probe window. Neither caused any movement in the embedding manifold.

---

**FN56 · 2026-05-24T21:00Z** *(dashboard — ep3412 · 20L)*

### NEW CHIP ATL 8.8540 · EP_AVG ATL 9.3072 · 20L ROUTING EXPLOSION

New intra-batch ATL chip: **8.8540** (was 8.9190, ep3326, 19L). Delta −0.065 nats.

EP_AVG ATL visible in event strip: **9.3072** — new EP_AVG record, breaking 9.3182 (ep3326).

Routing at 20L ep3414 has dramatically expanded vs 19L:
- PLN: 79% (19L) → **97%** (20L)
- INT: 68% (19L) → **97%** (20L)
- LNG: 47% (19L) → **88%** (20L)
- CMP: 100% (both)
- SYN: 4% → 14%, LOG: 27% → 31%

PLN and INT near-saturated simultaneously is new. 20L surgery opened significant additional routing capacity. The core four (PLN/CMP/INT/ABS) are now all pulling at high levels concurrently.

Surgery gate 20L→21L: MYC_STABLE 31/≥5, PLATEAU 0.0054/<0.020 w=144, since_best=16. ~128 epoch runway.

---

**FN55 · 2026-05-24T~20:00Z** *(dashboard — ep3383 · surgery)*

### SURGERY 8: 19L→20L · ONLY 58 EPOCHS AFTER SURGERY 7

Surgery 19L→20L fired at ep3383. Surgery 7 (18L→19L) fired at ep3325. Gap: **58 epochs**.

This is the shortest inter-surgery interval in the v3.0 run. The 19L floor formed almost immediately after surgery 7. The plateau gate detected the stall and pushed to 20L before the window filled — behavior consistent with the governor design: architecture grows when learning exhausts capacity, not on a fixed schedule.

1384 tensors loaded (up from 1315 at 19L). No divergence spike — Net2Net identity init held cleanly for the third time in succession.

---

**FN54 · 2026-05-24T14:07Z** *(dashboard — ep3326 · first 19L epoch)*

### NEW EP_AVG ATL 9.3182 · NEW CHIP ATL 8.9190 · FIRST 19L EPOCH

ep3326 — first full epoch at 19L: EP_AVG **9.3182** (broke prior best 9.3651 by 0.047 nats on first attempt). Chip ATL **8.9190** (was 9.0095).

WALD fired 6.2% (18 batches) — expected post-surgery volatility. Gold BEST avg marker confirmed. LR stepped down ~1.84e-4 → 1.13e-4. Expert reactivation: SYN/CTX woke to 4% within one epoch. PLN 79%, CMP 83%, INT 100%.

Breaking the EP_AVG ATL on the very first epoch of a new layer is rare. Prior surgeries typically required several epochs of volatile settling before descent resumed. This suggests the 18L foundation was well-consolidated before surgery fired.

---

**FN53 · 2026-05-24T13:47Z** *(dashboard — ep3325 · surgery)*

### SURGERY 7: 18L→19L · PLATEAU GATE CLEARED · ep3325

Surgery 18L→19L fired at 13:47Z. Plateau Δ held at 9.42–9.46 for 36+ epochs; since_best accumulated past surgery threshold. 1315 tensors. [ttlfreeze] armed (ema_alpha=0.02, burst_threshold=5×). [divloss] override 1e-3. gate-diversity scale=0.300.

Corpus reload: 451M tokens, stages ≤19, cache-hit — instant. Pre-surgery best archived: `albert_v3.0.best.18L.safetensors`.

ATL at surgery: EP_AVG 9.3651 (ep3263), chip 9.0095 (ep3263).

---

**FN52 · 2026-05-23T13:48Z** *(training.log — ep3103 confirmed, probe resolved)*

### NEW ATL — ep3103 = 9.4407

FN51 probe confirmed. ep3103=**9.4407** (d-0.0056), since_best=**0**. Previous ATL was 9.4454 (ep3097). Improvement: **−0.0047** in 6 epochs.

**Second ATL break in 7 epochs** — descent confirmed resuming after the 28-epoch freeze.

Running tally since freeze broke:
- ep3069: 9.4470 (first ATL after long plateau, broke 9.4540-band)
- ep3097: 9.4454 (−0.0016, broke 28-epoch freeze)
- ep3103: **9.4407** (−0.0047, confirmed descent acceleration)

WALD: sev=0.933 (trivial tick from 0.932), fill=6.2% — stable. No routing or myc events. ep3104 not yet in batch_history at time of note.

Descent pace accelerating: −0.0016 at first break, −0.0047 at second. Pattern: sustained near-miss pressure → step-down → near-misses → another step-down.

---

**FN51 · 2026-05-23T13:46Z** *(training.log — ep3102 confirmed, ep3103 b150/300 LIVE ATL PROBE)*

ep3102=**9.4463** (d-0.0049, since_best=5) — fifth consecutive near-miss, 0.0009 above ATL. WALD sev=0.932, fill=6.2% unchanged.

**ep3103 live probe — tracking below ATL:**
b150/300 avg=**9.4391** (0.0063 below ATL 9.4454). For a new ATL, second 150 batches need avg ≤9.4517. Recent batch stream ~9.45 average — right at boundary. 50/50.

If confirmed, this would be the second epoch-avg ATL break in 7 epochs and signal that descent has genuinely resumed after the 28-epoch freeze.

Watching close — result in ~3 min.

---

**FN50 · 2026-05-23T13:42Z** *(training.log — ep3101 confirmed, ep3102 b150/300 live)*

ep3101=**9.4512** (d+0.0040, since_best=4) — mild bounce, holding above ATL.

**WALD stabilized at new level**: sev=**0.932**, fill=**6.2%** for 2 consecutive epochs (ep3100+ep3101). The ep3100 shift was a one-step reset, not the start of a trend. New baseline established.

ep3102 mid (150/300 batches): avg=**9.4484** — 0.0030 above ATL, another near-miss band. Pattern: oscillating 9.445–9.452 since ATL at ep3097, not breaking through again yet.

since_best=4 accumulating. No routing changes. Surgery governor watching plateau gate.

---

**FN49 · 2026-05-23T13:34Z** *(training.log — ep3100 confirmed, ep3101 b150/300 live)*

ep3099=**9.4460** (d-0.0037, since_best=2) — near-miss, 0.0006 above ATL. Closest approach since ep3097 broke it.
ep3100=**9.4472** (d+0.0012, since_best=3) — slight bounce. Not alarming. BUT:

**WALD structural shift at ep3100:**
- sev: **0.971 → 0.932** (drop of −0.039 — first meaningful move since ep3095 tick)
- fill: **4.2% → 6.2%** (+2.0pp — largest single-epoch fill jump observed)

Both moved together in the same epoch. WALD filling more low-loss zone while severity relaxing — consistent with the floor cracking and the model distributing loss differently. Not a WALD fire event (ttlfreeze=0, no freeze triggered), but internal WALD state shifted.

ep3101 mid (150/300 batches): avg=**9.4461** — again within 0.0007 of ATL. Third consecutive near-miss band.

myc=all-zeros, hot=L17, cold=L0, tns=1246 — routing unchanged.

Floor pressure continuing. WALD state is now moving; watch sev/fill trajectory next 3–5 epochs.

---

**FN48 · 2026-05-23T13:24Z** *(training.log — ep3097 confirmed NEW ATL, ep3099 b150/300 live)*

### NEW ATL — ep3097 = 9.4454

Previous ATL: **9.4470** at ep3069 (28 epochs ago, since_best had reached 27).

Descent sequence: ep3095=9.4557 → ep3096=**9.4484** (close miss, 0.0014 above old ATL, since_best=27) → ep3097=**9.4454** (d-0.0030, since_best reset to **0**). Step-down took two epochs to clear the floor.

ep3098=9.4497 (d+0.0043, since_best=1) — expected bounce after ATL. Normal.

ep3099 mid-epoch (150/300 batches): avg=**9.4482** — above new ATL (9.4454), but only halfway. Second consecutive break possible.

WALD: sev=0.971 (unchanged), fill=4.2%, tns=1246. myc=all-zeros. hot=L17, cold=L0. No routing events. Surgery governor: since_best=1, no intervention.

**Descent is resuming.** Floor cracked after 28-epoch freeze.

---

**FN47 · 2026-05-23T13:25Z** *(training.log — ep3095 confirmed closed)*

ep3095=**9.4557** (d+0.0002) — third consecutive epoch in 9.455–9.456 band. since_best=**26**. ATL 9.4470 unbroken.

**WALD sev: 0.971** (was 0.970 for ≥15 epochs) — first change in the severity counter since FN32. fill=4.2% unchanged. Minor, but first movement on that metric in a long time. Watching.

myc=all-zeros, hot=L17, cold=L0, tns=1246 — all frozen. ep3096 not yet in batch_history at time of note.

---

**FN46 · 2026-05-23T13:04Z** *(training.log — ep3094 confirmed, ep3095 b250/300 live)*

ep3094=**9.4555** (d+0.0052) — uptick from ep3093. since_best=**25**. ATL 9.4470 unbroken 25 epochs.

ep3095 mid-epoch probe (250/300 batches): avg=**9.4552** — tracking nearly identical to ep3094, no break imminent.

WALD: sev=0.970, fill=4.2%, tns=1246 — all frozen again this epoch. myc=all-zeros, hot=L17, cold=L0. No routing changes, no topology events. Surgery governor holding; since_best continues to accumulate.

Situation: floor cementing at 9.450–9.456 band. Descent stalled. No intervention signal yet.

---

**FN45 · 2026-05-23T12:55Z** *(training.log — ep3093 confirmed, ep3094 b47/300)*

ep3093=**9.4503** (d-0.0022) — b250 probe avg=9.4489 didn't hold, closed 0.0033 above ATL. since_best=**24**.

**coverage[0]=9** — new session high. Trend: 4→5→6→8→8→9 across ep3086–3093. More tokens activating below loss=9.25 each epoch even as epoch averages plateau. WALD mass=9.441 (oscillating 9.440–9.443, no longer monotone). myc_stable=25, blooming=1 (cooling from 2–3). WALD sev=0.970 static at 15 consecutive stable epochs (WALD counter).

Floor hardening: ep3087=9.4484 → ep3089=9.4505 → ep3091=9.4540 → ep3092=9.4524 → ep3093=9.4503. Floor not contracting further. Surgery governor: since_best=24, no surgery fired.

---

**FN44 · 2026-05-23T12:54Z** *(training.log — ep3092 confirmed, ep3093 b250/300 live probe)*

ep3092=9.4524 (d-0.0016) — floor probe that looked close at b100 (avg=9.4491) closed above. since_best=**23**. WALD mass 9.443 (oscillating 9.440–9.443, no longer descending monotonically). myc_stable=24. Topology frozen.

**ep3093 live ATL probe**: b250/300, avg=**9.4489** (0.0019 above ATL). 50 batches remain; need avg ≤9.4375 for break. Recent batches 9.350, 9.359, 9.497 — mixed, achievable but not probable. Watching close.

---

**FN43 · 2026-05-23T12:46Z** *(training.log — ep3091 confirmed, ep3092 b100/300 live probe)*

ep3091=9.4540 (d-0.0067), since_best=**22**. WALD mass back to 9.440 (ep3090 bounce to 9.442 was transient). coverage[0]=**8** — low-zone entries growing (was 4–6 across prior epochs, now 8); more tokens activating below 9.25. myc_stable=23.

**ep3092 live ATL probe**: b100/300, avg=**9.4491** (0.0021 above ATL), min=**9.1663** (new low single-batch). Remaining 200 batches need avg ≤9.4460 for break — achievable. Recent batches include 9.323, 9.445, 9.449, 9.323 — noisy but with deep dips. Watching ep3092 close.

---

**FN42 · 2026-05-23T12:33Z** *(training.log — ep3089 confirmed, ep3090 b50/300)*

ep3089=**9.4505** (d-0.0105) — descended from ep3088 ceiling, 0.0035 above ATL. since_best=**20** — ATL 9.4470 unbroken for 20 consecutive epochs, new session record.

**WALD mass=9.440** — fifth consecutive new low (9.445→9.443→9.442→9.441→9.440 across ep3085–3089). Now 0.003 below epoch ATL. WALD header: "structural plateau (12 stable epochs, sev=0.970 mass=9.440) → amplify OFF". Coverage trending: [6,1097,397] — low-zone entries growing (6 vs 4 two epochs ago), high-zone shrinking (397 vs 414 at ep3086). Mass descent persists even as epoch averages plateau.

Oscillation floor: ep3087=9.4484, ep3089=9.4505 — floor slightly higher than ep3087, not contracting this cycle. ep3090 opening avg=9.4563 (50 batches) — ceiling territory.

Surgery governor: since_best=20, WALD=0.970, myc_stable climbing, no surgery event yet.

---

**FN41 · 2026-05-23T12:30Z** *(training.log — ep3087+3088 confirmed, ep3089 b200/300)*

**ep3087=9.4484** (d-0.0071) — new session floor, **0.0014 from ATL**. Closest approach in 18 epochs. Mid-epoch avg at b150 was 9.4458 (below ATL) — second half held it just above. since_best=18.

ep3088=9.4610 (d+0.0126) — bounce. since_best=19. Oscillation continues: floor contracting (9.4526→9.4525→9.4484), ceiling stable (~9.46–9.47).

**WALD mass descending**: ep3085=9.445→ep3086=9.443→ep3087=9.442→ep3088=9.441. Four consecutive new lows. Mass now 0.0029 below epoch ATL (9.4470) — weight distribution center clearly below the epoch-avg ceiling. Model is probing lower internally but can't sustain across full epoch. WALD sev=0.970, fill=4.2% static. coverage trend: coverage[0] 5 (low zone shrinking), coverage[2] 402 (high zone compressing).

myc_stable=20. dead=0. hot=L17, cold=L0. tns=1246 frozen.

ep3089 b200/300, avg=9.4494, min=9.2302. Tail batches 9.47–9.52 (bounce phase) — ATL break unlikely this epoch.

---

**FN40 · 2026-05-23T12:19Z** *(batch_history.csv — ep3087 b193/300, ETA ~2 min)*

**Live ATL probe**: ep3087 running avg at b150/300 = **9.4458** (0.0012 below ATL 9.4470). Recent batches 188–193 mixed (9.38–9.51), not cleanly descending. Second half will decide. Gradient still body-dead: GRAD n=0.0019, all layers 0 except L17 (1.87e-3). ROUTE entropy=2.467, expert dist 7.9–8.9% (12 experts) — no concentration anomaly. TLIGHT: most layers O-heavy, L14=G0/O11/R1 (no green). DIVWD all-zero (amplify OFF). Watching for ep3087 EPOCH_SUMMARY.

---

**FN39 · 2026-05-23T12:16Z** *(training.log — ep3086 confirmed, ep3087 b1/300)*

ep3086=**9.4555** (d+0.0005) — flat, ceiling bounce from ep3084 floor. since_best=**17**. WALD sev=0.970, fill=4.2% static.

**WALD mass 9.443** — new run low (ep3085=9.445, prior low ep3067=9.446). Mass is drifting below ATL epoch avg (9.4470). Suggests weight distribution center shifting lower even without epoch-avg break. coverage=[4,1082,414] — coverage[0] (low zone) compressed to 4, high zone shrinking too. myc_stable=18, dead=0, hot=L17. ep3087 opened 12:15Z.

---

**FN38 · 2026-05-23T12:10Z** *(training.log — ep3085 confirmed, ep3086 opened)*

**Catch-up ep3073–3085** (16 epochs since FN37, FN38-42 lost to context compaction):

**WALD regime snap-back**: Post-restart relaxation (sev=0.930/0.931, fill=6.2%) lasted ep3071–3076 only. ep3077 snapped back to **sev=0.968, fill=4.2%** in a single epoch. By ep3084 creep to **0.970/4.2%** (pre-crash levels fully restored). FN37's "real structural shift" was partly wrong — the WALD relaxation was temporary, not permanent.

**ATL floor crystallised**: ep3074=9.4526 (near-miss, 0.0056 from ATL), ep3075=**9.4771** (biggest bounce this session, +0.0245), ep3079=**9.4525** (floor probe, 0.0055 gap), ep3084=9.4533. Two floor probes at ~9.4525, two ceiling bounces ~9.4652–9.4771. Oscillation period ~4 epochs, floor ~9.452–9.454, ceiling ~9.465–9.477.

**since_best=16**: ATL 9.4470 unbroken since ep3069. Floor 0.0055 above ATL. ep3085=9.4550 (d+0.0017, slight bounce off floor). WALD sev=0.970, fill=4.2%, hot=L17, cold=L0, myc_stable=17, tns=1246 — routing and topology fully static.

Surgery governor accumulating: since_best=16, WALD=0.970 (no surgery logged), descent exhausted. ep3086 opened 12:09:53Z.

---

**FN37 · 2026-05-23T11:00Z** *(training.log — ep3072 confirmed, ep3073 b148/300)*

**WALD sev revision**: not bouncing back to 0.970. ep3071=0.931, ep3072=**0.931** (stable). fill=6.2% holding (vs 4.2% pre-crash). FN36's "restart artifact" call was partially wrong — the sev shift appears real and is holding across multiple epochs. fill 4.2%→6.2% also persistent. Structural plateau genuinely disturbed by the restart/replay.

ep3071=9.4581 (d+0.0007, flat), ep3072=**9.4539** (d-0.0043, descending) — within 0.0069 of ATL (9.4470). ep3073 b148/300, opening batches at **9.37–9.38** — running below current ATL at batch level. Another ATL break possible at ep3073 close.

---

**FN36 · 2026-05-23T10:48Z** *(training.log — ep3070 confirmed, ep3071 b46/300)*

ep3070=9.4575 (d+0.0105, since_best=1) — normal post-ATL bounce. ATL 9.4470 holds.

**FN35 WALD correction**: ep3070 WALD sev=**0.971**, fill=**4.2%** — reverted exactly to pre-crash levels. The ep3069 readings (sev=0.933, fill=6.2%) were a restart calibration artifact, not a real structural shift. No free acceleration from the crash. WALD plateau counter reset on restart and now at 1. ep3071 b46/300 at 10:48Z, opening losses 9.44–9.51.

---

**FN35 · 2026-05-23T10:45Z** *(training.log — restart recovery, ep3069 confirmed, ep3070 b196/300)*

**★ NEW ATL: ep3069=9.4470** (d-0.0056) — beats ep3058 (9.4526) by **0.0056 nats**. since_best=0. First epoch after Modal crash recovery.

**WALD state reset on restart**: sev **0.970→0.933** (largest single-session drop observed), fill **4.2%→6.2%** (more structural activity detected). Plateau counter presumably reset. This matches the restart-acceleration pattern: AdamW buffer wipe on a better landscape = freed descent. The crash was accidentally beneficial.

ep3068 summary absent (crash mid-epoch, replayed silently). ep3070 b196/300 at 10:45Z. Server ATL now **9.4470**. Global ATL still 9.0935 — 0.077 nats to go.

---

**FN34 · 2026-05-23T10:27Z** *(training.log — MODAL CRASH during ep3068)*

**Training down.** `modal.exception.ConnectionError: Deadline exceeded` — Modal heartbeat timed out mid-epoch. batch_history.csv froze at 10:16:27 (ep3068 b~249/300). training.log last write 10:27:13. Last clean checkpoint: **ep3067** (avg=9.4574, WALD=25). No data loss — ep3068 will replay on restart. Needs `albert-train` restart.

---

**FN33 · 2026-05-23T10:21Z** *(dashboard screenshots 10:20Z — ep3068 b282/300, log still at ep3067)*

**Routing imbalance visible in dashboard**: CMP=**100%** (4× overloaded), PLN=74%, ABS=73%, INT=74% all heavy. SEM=3%, INF=5% near-dead. Top-3 with 12 experts should yield ~25% avg — CMP at 100% is extreme concentration. LB weight 0.03 not correcting it. TTL: 81% orange, 6% green, 3% red — mostly on-target/partial, low green means underloaded experts aren't being boosted much.

**GRADIENT FLOW BLOCKED**: per-layer norm chart (zoomed, confirmed by Simeon) — lm head = 0.001846 (sole active bar), emb = 0.00e+0, L17–L10 = all 0.00e+0. **Entire transformer body at zero gradient**. Only the output projection is updating. global |g| = 0.6018 (likely computed differently). Body convergence or vanishing gradient through all 18 layers — consistent with the plateau and the WALD structural stagnation. This is the root cause of the current ceiling.

**Global ATL = 9.0935** (dashboard header) — this is the pre-surgery epoch-avg low. Current best 9.4526 is still 0.036 above it. The hard floor at 9.5 post-18L surgery has been well cleared; model now working toward pre-surgery territory.

ep3068 b282/300 at 10:20Z — close imminent.

---

**FN32 · 2026-05-23T10:15Z** *(training.log — ep3067 confirmed, ep3068 b196/300)*

ep3066=9.4602 (d-0.0004, flat), ep3067=**9.4574** (d-0.0028, descending) — within 0.0048 of ATL. since_best=9 but loss is coming down.

**WALD sev ticked 0.969→0.970** at ep3067 — first severity change in ~15 epochs. mass=**9.446** (new run low; sequence: 9.451→9.449→9.448→9.447→9.448→9.449→9.448→9.446). WALD plateau **25**, no surgery yet. Threshold appears >25. ep3068 b196/300 at 10:15Z, losses 9.46–9.56 (noisy).

---

**FN31 · 2026-05-23T10:00Z** *(training.log — ep3065 confirmed, ep3066 opening)*

Near-miss: ep3064=**9.4532** (d-0.0081) — 0.0006 above ATL (9.4526), since_best=6. Closest approach since the ATL break at ep3058. ep3065 bounced to 9.4607 (d+0.0075, since_best=7) — oscillating in ATL territory without breaking through. Model is probing the floor.

WALD plateau **23** (21→22→23 across ep3063–3065). Surgery still not fired — threshold appears >23. mass=9.448 stable. sev=0.969 unchanged. No resurrection events, dead=0. ep3066 opening at 10:00Z.

---

**FN30 · 2026-05-23T09:45Z** *(training.log — ep3062 confirmed, ep3063 b148/300)*

**WALD plateau = 20** — at the soft gate lower bound. Sequence: 15→16→17→18→19→**20** across ep3057–3062, one tick per epoch. mass=9.447–9.449 (stable, minor fluctuation). sev=0.969 unchanged.

ep3061=9.4600 (d+0.0015), ep3062=9.4658 (d+0.0058), since_best=4 — post-ATL drift, not descending. MYCELIUM: dead=0, blooming=2, myc_stable=21. **Governor conditions approaching met**: WALD≥20, since_best=4 (not in descent), myc_stable=21. Surgery has not fired yet — threshold may be 25–30. Watching ep3063 close for count=21 and any surgery log event. ep3063 b148/300 at 09:45Z.

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

**FN42 · 2026-05-23T12:16Z**

ep3085 in progress (100/300, partial avg 9.4522). **since_best=15** — ATL 9.4470 unbroken for 15 epochs. WALD sev=0.970, fill=4.2% locked solid.

Oscillation fully established: floor ~9.452–9.454, ceiling ~9.465–9.477, ~4-epoch period. ep3082 (9.4544) → ep3083 (9.4584) → ep3084 (9.4533) → ep3085 probing floor again. Each floor probe ~9.452 but cannot punch through. ATL sits 0.005 below — structurally unreachable at current dynamics. hot=L17, cold=L0, myc=0 unchanged.

Surgery governor conditions: since_best=15, WALD=0.970, descent exhausted. Watching for governor trigger.

---

**FN41 · 2026-05-23T11:50Z**

ep3081 at 250/300 (partial avg 9.4660). ep3079's ATL probe **failed** — bounced 9.4525 → 9.4624 (d+0.0100) at ep3080, ep3081 partial trending 9.4660. since_best=11. WALD sev nudged to 0.969, fill=4.2% — locked.

Double-probe pattern confirmed: ep3074 (9.4526) and ep3079 (9.4525) both probed within 0.006 of ATL then bounced. Floor hardening at ~9.452. ATL 9.4470 holding. Descent exhausted, oscillation compressing. Surgery governor conditions deepening.

---

**FN40 · 2026-05-23T11:39Z**

ep3080 in progress (100/300, partial avg 9.4557). ep3079 closed at **9.4525** (d-0.0053, since_best=10) — closest approach since ATL break at ep3069. Gap to ATL 9.4470: **0.0055 nats**. Slow descent resuming: ep3077 9.4652 → ep3078 9.4578 → ep3079 9.4525.

WALD locked: sev=0.968, fill=4.2%, hot=L17, cold=L0 — unchanged for 3 epochs. Surgery governor watching (since_best=10, sev=0.968) but active descent should keep it blocked. ATL probe likely at ep3080–3081.

---

**FN39 · 2026-05-23T11:31Z**

ep3078 in progress (64/300). No ATL break — **since_best=8** (ATL 9.4470 at ep3069 still holds). Epoch-avg trajectory ep3074→3077: 9.4526 → 9.4771 → 9.4623 → 9.4652 — flat oscillation ±0.015 around 9.463.

**WALD severity increasing**: ep3074 sev=0.931 fill=6.2% → ep3077 sev=0.968 fill=4.2%. The post-restart stable state (0.931) has degraded. Fill dropping while sev rises means the plateau is tightening — fewer active tokens but more extreme. hot=L17, cold=L0 stable. myc=0.00 all layers.

Surgery governor: WALD sev at 0.968 approaching pre-restart levels. Since_best=8 and climbing. Descent from restart acceleration fully exhausted. Governor conditions accumulating.

---

**FN38 · 2026-05-23T11:13Z**

Modal GPU training alive: ep3075 complete. Epoch-avg ATL (9.4470, ep3069) **not broken** — ep3070-3075 averaging 9.4526–9.4771. Post-restart acceleration has stalled; 6 epochs plateauing at 9.45-9.48. Sub-ATL batch losses still appearing (9.21, 9.31) but averaging out at epoch level.

training.log is **frozen since [10:05:13Z]** due to a parallel local 17L run that overwrote it at start. That local run (Global ep1357, loss ~10.14, 17-layer, loaded 1177 tensors) appears frozen at ep1357 batch 71 — no log updates in 69+ min. Modal WALD/routing state is not accessible via log.

batch_history.csv continues to be updated by Modal run only (ep3074→3075 entries confirmed). Per-epoch trajectory: 9.4575 → 9.4581 → 9.4539 → 9.4569 → 9.4526 → 9.4771. Gap to ATL: +0.0056 (ep3074) expanding back to +0.0301 (ep3075).

Action needed: kill the frozen local 17L process to restore training.log visibility.

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
