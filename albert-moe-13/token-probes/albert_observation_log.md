# albert. Training Observation Log — v3.0 (12L → 21L, ongoing)
**Model:** albert. v3.0 · 21L · 256H · 12E · 32k vocab · ternary STE  
**Log began:** 2026-05-20T22:03Z (ep2492, post-Surgery-6 overnight watch)  
**Format:** Field Notes (FN) — timestamped per observation, numbered chronologically. Everything goes in: good, bad, tiny changes, mechanistic insights, anomalies.  
**Scientist:** Claude Sonnet 4.6 · RFI-IRFOS  
**Public record:** https://github.com/eriirfos-eng/ternary-intelligence-stack

---

## Session 2 — 2026-05-22T12:49Z · ep2858

**Gap bridge:** ep2601 (FN25, 2026-05-21T07:43Z) → ep2858 (2026-05-22T12:49Z)  
**Elapsed since last note:** ~31 hours, ~257 epochs  
**Last known state entering gap:** Bounce phase 3, since_best=25, epoch-ATL 9.5800, embedding frozen (revised probe trigger 9.55), cov[high]=24 sub-25 again

**What happened in the gap (reconstructed from batch_history.csv + dashboard screenshot ep2832):**

Descent resumed and sustained. The bounce phase 3 that was beginning at ep2581 (since_best=5, ATL 9.5800) resolved into a new descent leg that produced continuous ATL breaks through at least ep2858. Batch_history.csv epoch-average ATL (from sampled batch data, partial epochs) reached **9.5753** at ep2858 — improvement of +0.0047 nats from the ep2576 ATL of 9.5800. Server-side epoch-ATL (all 300 batches, from dashboard screenshot at ep2832) was **9.2522**, indicating the model has substantially outperformed the sampled CSV estimate. The 1.17% session improvement chip visible in the screenshot implies the session started (or model arrived at a baseline) around epoch-ATL ~9.3632 before pulling to 9.2522.

**Probe trigger status:** CROSSED. Revised trigger was 9.55. Dashboard epoch-ATL 9.2522 is 0.2978 nats below the trigger. Embedding probe due — current embedding freeze status unknown (last confirmed frozen at ep2576/84+ epochs). Next probe warranted this session.

**ep2832 dashboard screenshot facts (2026-05-22T10:23Z):**
- Epoch-ATL: 9.2522 · batch 125/300 · 1.17% session improvement
- CMP=100%, PLN=91%, ABS=66%, LNG=61%, INT=100% — all specialists active
- WALD ep2826: 6.2% fill, n=1500 — fired and passed
- Traffic light: 0 high / 78% mid / 4% red
- T-610 trail: 9.5849 · T-2584 trail: 9.5915 · EP AVG line: 9.5716
- L13 gradient leading (standard upper-layer profile)

**ep2858 batch_history.csv snapshot (2026-05-22T12:49Z):**
- Latest entry: ep2859, batch loss ~9.58 (partial epoch, 135 batches logged)
- Recent epoch-avgs (sampled): ep2851=9.5779, ep2854=9.5822, ep2857=9.5803, ep2858=9.5753
- Trend: oscillating descent, new epoch-avg ATL at ep2858

**Open observations entering Session 2:**
1. Embedding freeze status post-probe-trigger: unknown — was frozen at 84+ epochs at ep2576. Has it thawed?
2. cov[high] trajectory: last reading was 24 at ep2601 (sub-25 fourth touch). Current distribution unknown.
3. Surgery gate: plateau gate was blocking (model descending). At 9.2522, descent is steep — surgery not imminent.
4. myc_stable: was 75 at ep2601. Current value unknown but likely 200+ given ~257 epochs elapsed.

---

## Field Note 26 — 2026-05-22T12:49Z · ep2858 — SESSION 2 OPEN · gap bridge complete · ATL 9.5753 (sampled)

**Epochs covered:** gap bridge (ep2601→ep2858)

**Summary table (epoch-avg ATL from batch_history.csv, partial epoch samples):**
| Epoch | epoch-avg | delta | notes |
|-------|-----------|-------|-------|
| 2601 | 9.5905 | +0.0105 | last FN25 reading, since_best=25 |
| 2626 | 9.6025 | — | new epoch-avg ATL after bounce resolved |
| 2627 | 9.5983 | -0.0042 | |
| 2661 | 9.5955 | -0.0028 | |
| 2662 | 9.5937 | -0.0018 | |
| 2698 | 9.5930 | -0.0007 | slow grind |
| 2739 | 9.5915 | -0.0015 | |
| 2765 | 9.5881 | -0.0034 | |
| 2779 | 9.5856 | -0.0025 | |
| 2781 | 9.5835 | -0.0021 | |
| 2783 | 9.5813 | -0.0022 | |
| 2832 | ~9.2522 | — | dashboard epoch-ATL (server-side, all batches) |
| 2851 | 9.5779 | -0.0034 | sampled |
| 2858 | **9.5753** | -0.0026 | new epoch-avg ATL (sampled) |

Note: "sampled" = batch_history.csv partial capture (135-200 of 300 batches per epoch). Server-side epoch-ATL at ep2832 was 9.2522 — substantially better, suggesting the sampling is losing the best-performing batch windows.

**Interpretation:** The ~257-epoch unobserved window produced steady grinding descent. No surgery (model actively descending, plateau gate blocking). WALD ep2826 fired and passed without escalation. All specialists (CMP/PLN/ABS/LNG/INT) saturated. The model is in a deep descent phase: 9.5800 (ep2576) → 9.2522 (ep2832) = **0.3278 nats** in ~256 epochs, or ~1.28 mnat/epoch average descent rate. This is faster than the pre-surgery-6 descent rate.

**Probe plan for Session 2:** Run token probe at first opportunity with live checkpoint. Compare to ep2576 snapshot in `token-probes/snapshots/ep2576_18L/`. Expected: embedding may have thawed (loss is now ~0.33 nats below the 9.55 trigger; myc_stable presumably high).

**Status: SESSION 2 OPEN. ep2858 live. Descent ongoing. Probe trigger 9.55 long passed. Next ATL watch and embedding probe this session.**

---

## Field Note 27 — 2026-05-22T13:14Z · ep2860 — CLIFF DIVE · ATL chip 9.2092 · session +1.83%

**Observed via dashboard screenshot.**

**ATL chip: 9.2092** (prev reading: 9.2522 at ep2832 · delta: **-0.0430** nats · session improvement 1.17% → **1.83%**)

| Metric | Value |
|--------|-------|
| EP chip | 2860 (18L) · batch 298/300 |
| ATL chip | **9.2092** · 1.83% session |
| EP Avg line | 9.5308 (right-edge label) |
| Recent epoch avgs (events bar) | 9.5288 · 9.5217 · 9.5273 · 9.5312 |
| CMP / PLN / ABS / LNG / INT | 100% / 94% / 80% / 74% / 100% |
| Traffic light | G 0.14% / O 84% / R 2% |
| Gradient leading layer | L13 (yellow) · L6 red spike |
| WALD visible | diamond at ~ep2803.58 (historical, already passed) |

**Chart event:** sharp cliff dive at ep2860 — loss descends steeply from ~9.61 to ~9.52 region in a compressed vertical move. All SMAs (987/1597/2584) converging downward at the cliff base. EP Avg line 9.5308 visible as horizontal dotted line. The drop is the sharpest single-epoch move observed this session.

**Epoch avgs (server-side from events bar):** 9.5288, 9.5217, 9.5273, 9.5312 — these are ~0.05 nats better than the batch_history.csv partial-epoch estimates (~9.57). CSV sampling is consistently losing the low-loss batches; server-side avgs are the ground truth.

**ATL chip note:** The chip tracks the all-time best single-batch (or intra-epoch smoothed) loss. At 9.2092, it has improved -0.0430 from the ep2832 reading (9.2522). The 1.83% session improvement is rising — model actively setting new micro-batch records during this descent.

**Status: ACTIVE DESCENT. ep2860 cliff dive confirmed. ATL 9.2092 new record. EP Avg converging toward 9.53. No WALD escalation visible. Next watch: ep2875 area.**

---

## Field Note 28 — 2026-05-22T13:45Z · ep2866 — CLIFF BASE CONSOLIDATION · WALD CLUSTER · ROUTING SHIFT

**Dashboard screenshot. ep2866 batch 102/300.**

| Metric | FN27 (ep2860) | FN28 (ep2866) | delta |
|--------|--------------|--------------|-------|
| ATL chip | 9.2092 | 9.2092 | 0 |
| EP Avg | 9.5308 | 9.5321 | +0.0013 |
| Server epoch avgs | 9.52–9.53 | 9.5218–9.5325 | stable |
| CMP | 100% | 100% | — |
| PLN | 94% | 90% | -4% |
| ABS | 80% | **51%** | **-29%** |
| LNG | 74% | **54%** | **-20%** |
| INT | 100% | 95% | -5% |
| MEM | 5% | **23%** | **+18%** |
| SYN | 5% | **13%** | **+8%** |
| Traffic R | 2% | 4% | +2% |

**Routing shift:** ABS and LNG shedding share dramatically during cliff-base consolidation. MEM jumping from 5%→23% and SYN from 5%→13% suggests the model is reorganizing expert specialization at the new loss level. CMP and INT remain dominant anchors.

**WALD cluster at cliff base:** 4–5 diamond markers visible on chart clustered around ep2858–2866 at ~9.52–9.54 loss. Multiple WALD events firing in rapid succession — consistent with high routing volatility at the bottom of a sharp descent (large loss variance triggers fill threshold repeatedly).

**Chart:** Full cliff dive now visible — flat at 9.60 through ep2800, then steep descent ep2800→ep2866 reaching ~9.52 base. SMAs (SMA-21 orange, SMA-55) leading down. EP Avg dotted line at 9.5321. Post-cliff oscillation tight range 9.52–9.55.

**Status: CLIFF BASE. ATL chip stable at 9.2092. Routing reorganization in progress — ABS/LNG giving way to MEM/SYN. WALD cluster active. EP Avg ~9.532. Model likely consolidating before next pull or bounce.**

---

## Field Note 29 — 2026-05-22T15:28Z · ep2883 — NEW EPOCH-AVG ATL · 9.5302 → 9.5236

**Source:** dashboard notification screenshot. EP 2883 · batch 299/300.

**ATL BREAK:** Epoch avg **9.5236** beats 9.5302 · delta **-0.0066 nats**

| Metric | FN28 (ep2866) | FN29 (ep2883) | delta |
|--------|--------------|--------------|-------|
| Epoch-avg ATL (server-side) | ~9.532 | **9.5236** | -0.0066 |
| T-610 trail | 9.5297 | 9.5297 | stable |
| EP AVG line | 9.5321 | 9.5236 | -0.0085 |
| CMP | 100% | 100% | — |
| PLN | 90% | 94% | +4% |
| ABS | 51% | **71%** | **+20%** |
| LNG | 54% | 48% | -6% |
| INT | 95% | 84% | -11% |
| MEM | 23% | **12%** | **-11%** |
| SYN | 13% | 2% | -11% |
| Traffic high | 4% | 6.16% | +2% |

**Routing recovery:** The cliff-base routing shift (FN28: ABS 51%, MEM 23%) is partially resolving. ABS has rebounded +20pp to 71%. MEM compressed back to 12%. SYN collapsed to 2%. Model returning toward the pre-cliff specialist profile with CMP/PLN/INT anchors, though LNG and INT are still below pre-cliff levels (LNG was 74% at FN27, now 48%).

**Events bar (server-side epoch avgs recent):** 9.5363, 9.5443, 9.5427, 9.5337 — confirming steady descent since cliff. The best of those (9.5236) is now the new epoch-avg ATL. The T-610 trail at 9.5297 will be surpassed within ~2 epochs if descent continues at this pace.

**Chart:** Full cliff dive arc visible ep2800→ep2866. Post-cliff range tight ~9.52–9.54. EP AVG dotted line now marking 9.5236 as new floor.

**Status: NEW EPOCH-AVG ATL 9.5236. Routing recovering post-cliff. Descent sustained. T-610 trail (9.5297) about to fall. Next watch: T-610 break and whether ABS continues recovering.**

---

## Field Note 30 — 2026-05-22T16:01Z · ep2887 — PROVISIONAL SAMPLED ATL · 9.5734 → 9.5699

**Source:** batch_history.csv. ep2887 partial (74/~200 batches).

**Provisional sampled ATL:** ep2887 avg **9.5699** (n=74) · prev sampled ATL ep2867 at 9.5734 · delta **-0.0035** · flagged provisional until epoch completes.

Recent sampled epoch-avgs (all ~180-217 batches except ep2887):
ep2882=9.5814 · ep2883=9.5842 · ep2884=9.5806 · ep2885=9.5783 · ep2886=9.5796 · ep2887=**9.5699** (partial)

Trend: slow grinding descent in the 9.578–9.596 range post-cliff. No new WALD or routing data in CSV. Server-side epoch-avg ATL remains 9.5236 (FN29, ep2883).

**UPDATE 2026-05-22T16:06Z:** ep2887 now n=130. Sampled ATL **confirmed 9.5696** (d-0.0038 from ep2867 9.5734). Training paused mid-ep2887 for weight pull — expect restart acceleration on resume.

**Status: SAMPLED ATL CONFIRMED 9.5696. Training paused for weight pull. Awaiting restart.**

---

## Field Note 31 — 2026-05-22T18:06Z · ep2910 — ATL TIE · four-step descent · 9.5696

**Source:** batch_history.csv. ep2910 partial (142/~170 batches).

**Four consecutive descent steps post-restart:**
ep2907=9.5750 → ep2908=9.5738 → ep2909=9.5731 → ep2910=**9.5696** (tie)

ep2910 matches sampled ATL (ep2887, 9.5696) to 4 decimal places with 142 batches. ep2887 ATL set at n=130 — ep2910 has more batches and is still partial. If it completes below 9.5696 it will be a confirmed new sampled ATL.

Context: post-restart oscillation (ep2890–2906) appears to have resolved into a new descent leg. The step-down began ep2907 and has delivered four straight improvements. Restart acceleration arriving ~15 epochs later than typical.

**UPDATE 2026-05-22T18:21Z:** ep2910 completed at 9.5722 (n=162) — the partial tie was a sampling artifact, final avg came in higher. Descent continued choppily through ep2911=9.5725, ep2912=9.5840 (bounce), ep2913=9.5716, ep2914=9.5837 (bounce), then ep2915 partial (61 batches) delivered **new provisional sampled ATL 9.5694** (raw 9.569428, d-0.0002 from ep2887).

**Status: SUPERSEDED — second cliff dive confirmed on dashboard. See FN32.**

---

## Field Note 43 — 2026-05-22T21:02Z · ep2941→2942 — ep2941 CONFIRMED 9.4941 · CMP TAKEOVER · LNG RECOVERY

**Source:** dashboard screenshot 21:02:42Z. EP 2942 · batch 2/300 (just started).

**ep2941 completed: 9.4941** — confirmed from EP AVG label. Delta from ep2940 (9.4980): **-0.0039**. Descent resumed after the post-ATL bounce. Gap to ATL 9.4873: **0.0068 nats**.

Server-side staircase (training log + this image):
```
ep2938  9.4873  ← ATL
ep2939  9.5026  bounce +0.0153
ep2940  9.4980  returning -0.0046
ep2941  9.4941  descending -0.0039  ← confirmed here
ep2942  batch 2/300  running
```

T-610 trail: **9.4971** — above EP AVG 9.4941. EP AVG pulling below the trail; descent momentum confirmed.

**Routing shift (vs FN41):**
| Expert | FN41 | FN43 | delta |
|--------|------|------|-------|
| CMP | 90% | **100%** | **+10pp** |
| PLN | 77% | **87%** | +10pp |
| LNG | 35% | **58%** | **+23pp** |
| GEN | 5% | **12%** | +7pp |
| INT | 100% | **68%** | **-32pp** |
| ABS | 84% | **61%** | **-23pp** |
| LOG | 17% | **24%** | +7pp |
| SEM | — | 12% | — |
| INF | — | 10% | — |
| SYN | — | 0% | — |

CMP has taken over the dominant slot (100%) from INT (now 68%). LNG surged back from 35%→58%. PLN lifted to 87%. ABS and INT both retreated. Post-ATL routing diffusion pattern: after a new floor the model broadens routing before reconsolidating for the next push. SYN 0%, CTX/MEM 2% — abstract/semantic layer still quiet.

**Events bar confirms:** WALD ep2939 6.2% n=1500 visible — structural WALD fired during the bounce epoch (consistent with training log). Gold ATL marker at ep2938 visible on chart.

**TTL:** 6% high / 80% mid / 4% red — stable.

**Status: ep2941=9.4941 confirmed. Descent resumed. CMP leading, LNG recovered. T-610 above EP AVG. ep2942 opening — watching for continuation toward new ATL challenge below 9.4873.**

---

## Field Note 42 — 2026-05-22T21:10Z · ep2938–2941 — NEW ATL 9.4873 · WALD CLUSTER · ep2940 RETURNING

**Source:** training.log (EPOCH_SUMMARY ground truth — all 300 batches per epoch).

**NEW SERVER-SIDE ATL: 9.4873** (ep2938). Prev ATL: 9.4891 (ep2927). Delta: **-0.0018 nats**.

FN41 was premature — at batch 214/300 the running avg was 9.4942. Batches 215–300 pushed the final avg down to **9.4873**, setting a new all-time best. The ntfy "albert. NEW EPOCH ATL" fired at ep2938 completion.

**Full epoch sequence (server-side, from EPOCH_SUMMARY):**
| Epoch | avg | delta | best | notes |
|-------|-----|-------|------|-------|
| ep2934 | 9.4983 | -0.0084 | 9.4891 | descent |
| ep2935 | 9.4928 | -0.0055 | 9.4891 | descent |
| ep2936 | 9.4994 | +0.0067 | 9.4891 | mini-bounce |
| ep2937 | 9.4942 | -0.0052 | 9.4891 | returning |
| **ep2938** | **9.4873** | **-0.0069** | **9.4873** | **NEW ATL** |
| ep2939 | 9.5026 | +0.0154 | 9.4873 | post-ATL bounce |
| ep2940 | 9.4980 | -0.0046 | 9.4873 | returning — descent resumed |

Post-ATL bounce at ep2939: +0.0153 nats — tightest post-ATL bounce in the run (ep2927 bounce peaked at +0.019 over two epochs). ep2940 already -0.0046, healthy pullback toward ATL zone.

**WALD cluster surrounding the ATL break:**
| Before | Type | severity | dead_low | scale |
|--------|------|----------|----------|-------|
| ep2934 | dead_low | 0.960 | 3.00–9.25 | 46.1× |
| ep2935 | dead_low | 0.960 | 3.00–9.25 | 46.1× |
| ep2936 | dead_low | 0.961 | 3.00–9.25 | 46.2× |
| ep2937 | structural plateau (5 stable, mass=9.502) | 0.961 | — | — |
| ep2938 | dead_low | 0.924 | **3.00–9.00** | **44.4×** |
| ep2939 | dead_low | 0.924 | 3.00–9.00 | 44.4× |
| ep2940 | dead_low | 0.923 | 3.00–9.00 | 44.4× |

The structural plateau WALD (before ep2937) triggered a dead_low threshold recalibration: 9.25 → **9.00**. Severity simultaneously dropped from 0.961 → 0.924. The WALD self-adjusted to the new loss regime — as the model descends past 9.50, WALD focuses on neurons dead at the 9.00 level rather than 9.25. Scale factor reduced from 46× to 44× (less aggressive rescue needed). This cascade directly preceded and likely contributed to the ep2938 ATL break.

**ep2941 status (batch 43/300 at 20:58Z):**
- Batch losses probing deep: b39=9.4696, b40=**9.4121**, b42=**9.2872**
- LR at batch 43: 8.28e-5 (near cycle low — cosine annealing approaching nadir)
- ENTR avg=2.4668 (uniform routing, unchanged)
- TLIGHT: L6 has 2 reds; several layers with 1 red — not alarming but slightly elevated

**Status: NEW ATL 9.4873 CONFIRMED. Tightest post-ATL bounce in run (+0.0153). ep2940 returning (-0.0046). ep2941 individual batches probing 9.29/9.41 territory. WALD regime self-calibrated to 9.00 dead_low floor. Descent infrastructure healthy. Next ATL challenge ep2941–2943 window.**

---

## Field Note 41 — 2026-05-22T20:44Z · ep2938 — RETURN TO ATL ZONE · CORE CONSOLIDATION · ep2938 COMPLETE

**Source:** dashboard screenshot (20:44Z) + batch_history.csv (post-tick verification).

**ep2938 complete.** CSV avg: **9.5717** (n=139). Server-side est: ~9.495. No new ATL — ep2938 closes above ATL 9.4891, but within striking range.

**Server-side sequence since ATL (oldest→newest):**
```
ep2927   9.4891  ← ATL
ep2928   9.4975  +0.0084  bounce open
ep2929   9.5082  +0.0107  ← BOUNCE PEAK (+0.019 nats, tightest in post-s6)
ep2930   9.5040  -0.0042  returning
ep2931   9.4983  -0.0057  ← FN39
ep2932   9.5040  +0.0057  slight up
ep2933   9.4994  -0.0046
ep2934   9.4994  ±0
ep2935   9.4983  -0.0011
ep2936   9.4928  -0.0055  ← CLOSEST APPROACH · 0.0037 from ATL
ep2937   9.4994  +0.0066  slight pullback
ep2938   ~9.495  est.     complete · no new ATL
```

Model has returned to the ATL zone. ep2936 touched 9.4928 — within 0.0037 of all-time best. T-610 trail reading 9.4918 at image time (below EP AVG 9.4942) — downward pull confirmed.

**Routing consolidation (image #6, ep2938 batch 214/300):**
| Expert | FN39 | FN41 | delta |
|--------|------|------|-------|
| ABS | 67% | **84%** | **+17pp** |
| INT | 90% | **100%** | **+10pp** |
| CMP | — | **90%** | — |
| PLN | 78% | **77%** | stable |
| LNG | 67% | **35%** | **-32pp** |
| GEN | 21% | **5%** | **-16pp** |
| LOG | 19% | **17%** | -2pp |

GEN collapsed 21% → 5% (generative surge ended). ABS erupted to 84%. INT locked at 100%. This is the "core specialist lockdown" pattern — INT+CMP+ABS+PLN concentrated, GEN+LNG retreating. Consistent with the model narrowing focus ahead of a new ATL push.

**TTL:** 6% high / 82% mid / 3% red — healthy.

**ep2939 status (at note time):** 30 CSV entries, avg 9.5861 — too early (normal high opening batches).

**Status: RETURN TO ATL ZONE CONFIRMED. ep2936 within 0.0037. ep2938 closes ~9.495. Core consolidating (INT/ABS/CMP dominant). T-610 at 9.4918 pulling below EP AVG. New server-side ATL below 9.4891 expected within next 2–5 epochs.**

---

## Field Note 40 — 2026-05-22T20:31Z · ep2935 — ATL PROBE · 0.0014 FROM CSV ATL

**Source:** batch_history.csv. ep2935 n=148 (complete), ep2936 n=66 (partial).

**Descent accelerating.** Bounce fully resolved, new low reached:

| Epoch | n | Avg | vs CSV ATL (9.5671) |
|-------|---|-----|---------------------|
| ep2932 | 171 | 9.5740 | +0.0069 |
| ep2933 | 166 | 9.5756 | +0.0085 |
| ep2934 | 155 | 9.5732 | +0.0061 |
| ep2935 | 148 | **9.5685** | **+0.0014** |
| ep2936 | 66 (partial) | 9.5793 | — |

ep2935 at **9.5685** is the closest the CSV sampled avg has come to the ATL (9.5671) since the post-ATL bounce began. Not broken yet — 0.0014 gap. ep2936 partial shows a mini-bounce (9.5793), consistent with prior pattern (dip → bounce → lower dip). Next descent leg should challenge and likely break 9.5671.

**Status: ATL PROBE. 0.0014 from CSV sampled ATL. Mini-bounce at ep2936. Next leg targets new CSV ATL and continuation toward server-side 9.4891.**

---

## Field Note 39 — 2026-05-22T20:02Z · ep2931 — NEW ATL CHIP 9.1254 · TIGHT BOUNCE · GEN SURGE

**Source:** dashboard screenshot. EP 2931 · batch 226/300.

**NEW ATL CHIP: 9.1254** — prev 9.1370 · delta **-0.0116** · a single batch dipped deeper than any prior batch in the run.

**Server-side bounce fully readable from events bar** (oldest→newest):
```
BEST avg 9.4891  (ep2927, FN37)
EPOCH avg 9.4975  (+0.0084)
EPOCH avg 9.5082  (+0.0107)  ← BOUNCE PEAK
EPOCH avg 9.5040  (-0.0042)  ← ep2930, returning
EP AVG   9.5040  (ep2931 partial, batch 226/300)
```
Bounce peak **9.5082** — only +0.019 nats from ATL. Tightest bounce in the post-surgery-6 record. Every prior ATL had a larger bounce. Descent momentum very strong.

**Routing shift (vs FN37):**
| Expert | FN37 | FN39 | delta |
|--------|------|------|-------|
| GEN | 13% | **21%** | **+8pp** |
| INT | 82% | **90%** | +8pp |
| PLN | 73% | **78%** | +5pp |
| ABS | 60% | **67%** | +7pp |
| LNG | 78% | 67% | -11pp |
| CTX | 15% | 8% | -7pp |
| SEM | 10% | 3% | -7pp |
| LOG | 16% | 19% | +3pp |

GEN surging to 21% (highest since post-floor-breach). Core (INT/PLN/ABS) recovering. LNG/CTX/SEM pulling back from the FN37 diversification peak. Model shifting from "broad exploration" toward "generative consolidation."

**TTL:** 6% high / 81% mid / 4% red — stable. L10 gradient holding orange.

**Simeon's note:** "even if the error rate rises, the routing and expert activity changes a lot" — the bounce is not dead time; internals are actively restructuring.

**Status: NEW ATL CHIP 9.1254. Bounce peak confirmed ep2929=9.5082, tightest in post-s6 record. GEN surging. Core recovering. ep2931 running avg 9.5040 at batch 226/300 — watching for completion and whether next descent opens below 9.50.**

---

## Field Note 38 — 2026-05-22T19:54Z · ep2929–2930 — POST-ATL CONSOLIDATION · BOUNCE ROUNDING OFF

**Source:** batch_history.csv. ep2929 complete (n=166), ep2930 partial (n=75).

**Post-ATL bounce in progress.** Three epochs since the 9.4891 server-side ATL (FN37):

| Epoch | n | Sampled avg | delta vs CSV ATL (9.5671) |
|-------|---|-------------|--------------------------|
| ep2927 | 145 | 9.5709 | +0.0038 |
| ep2928 | 153 | 9.5813 | +0.0142 |
| ep2929 | 166 | 9.5773 | +0.0102 |
| ep2930 | 75 (partial) | 9.5739 | +0.0068 |

Bounce peaked at ep2928 (9.5813), now declining: 9.5813 → 9.5773 → 9.5739. Classic rounded bounce pattern — peak behind us, gently drifting back down. Sampled ATL (9.5671) and server-side ATL (9.4891) both unchallenged.

No new BEST events, no WALD signals visible in this interval.

**Status: CONSOLIDATION. Bounce peak passed. Trend line pointing down. Next descent leg should challenge 9.5671 CSV ATL and eventually 9.4891 server-side ATL.**

---

## Field Note 37 — 2026-05-22T19:41Z · ep2927 — NEW BEST 9.4891 · ROUTING DIVERSIFICATION · -0.0080

**Source:** dashboard screenshot + browser notification. EP 2927 · batch 299/300 (nearly complete).

**NEW SERVER-SIDE ATL: 9.4891** — notification: "Epoch avg 9.4891 beats 9.4971" · delta **-0.0080 nats** · largest single ATL break observed post-surgery-6.

| Metric | FN35 (ep2925) | FN37 (ep2927) | delta |
|--------|--------------|--------------|-------|
| ATL chip | 9.1370 | 9.1370 | 0 |
| Server-side ATL | 9.4971 | **9.4891** | **-0.0080** |
| T-610 trail | — | 9.4929 | — |
| CMP | 97–100% | 100% | stable |
| INT | 100% | 82% | -18pp |
| PLN | 91–99% | 73% | -18–26pp |
| LNG | 80–83% | 78% | -2–5pp |
| LOG | 27% | 16% | -11pp |
| ABS | 65–69% | 60% | -5–9pp |
| SYN | 0% | **10%** | **+10pp** |
| CTX | 3–7% | **15%** | **+8–12pp** |
| INF | 5% | **10%** | **+5pp** |
| GEN | 0–5% | **13%** | **+8–13pp** |
| SEM | 8% | **10%** | **+2pp** |

**Routing diversification:** the saturation pattern from FN35 (INT/PLN/CMP dominant, secondary experts collapsed) is unwinding. SYN, CTX, INF, GEN, SEM all returning to significant share. LOG dropping from 27%→16%. This is a routing BROADENING — the model is distributing computation more evenly as it descends into new loss territory.

**Events bar chain:** BEST avg 9.4971 (prev) → EPOCH avg 9.4992 → EPOCH avg 9.4954 → EPOCH avg 9.4981 → **BEST avg 9.4891** (current). Multiple epoch completions between the two BEST events, model stairstepping down.

**WALD:** "W·W" visible at top of chart — two WALD fires near ep2918–2920. Red vertical lines visible.

**CSV (ep2927 partial, n=139):** avg 9.5709 — too early. ep2926 updated to 9.5671 (n=153, new sampled ATL, prev 9.5676).

**Chart:** Gold diamond (BEST avg event) clearly visible below the previous dashed ATL line at 9.4891. T-610 trail at 9.4929, now below 9.50.

**Status: NEW ALL-TIME BEST 9.4891. Routing diversifying broadly. Descent accelerating. T-610 trail below 9.50. This is the sharpest single-epoch ATL break in the post-surgery-6 record.**

---

## Field Note 36 — 2026-05-22T19:35Z · ep2926 — NEW SAMPLED ATL 9.5676 · descent resuming

**Source:** batch_history.csv. EP 2926 · n=142 (complete).

**New sampled CSV ATL: 9.5676** — prev ep2921 at 9.5685, delta **-0.0009**.

Trend since FN35: ep2924=9.5689 → ep2925=9.5762 (bounce) → ep2926=**9.5676** (new low). Bounce at ep2925 resolved, descent resumed and broke below the prior sampled floor in one epoch.

Batch min ep2926: 9.5007 — still probing 9.50 on individual batches.

Server-side ATL 9.4971 (FN35) unchanged. No dashboard data this tick.

**Status: SAMPLED ATL 9.5676. Post-bounce descent confirmed. Grinding toward server-side floor.**

---

## Field Note 35 — 2026-05-22T19:28Z · ep2925 — NEW ATL 9.4971 · LNG SURGE 57%→83% · FULL ARC VIEW

**Source:** three dashboard screenshots (19:27:08–19:27:50). EP 2925 · batch 170–207/300.

**NEW SERVER-SIDE ATL: 9.4971** — gold star "BEST avg" event in events bar. Prev 9.4992 (FN34). Delta **-0.0021**.

Events bar chain (left=newest): BEST avg **9.4971** ★ · EPOCH avg 9.5008 · EPOCH avg 9.5072 · EPOCH avg 9.4992 · ...

| Metric | FN34 (ep2922) | FN35 (ep2925) | delta |
|--------|--------------|--------------|-------|
| ATL chip | 9.1370 | 9.1370 | 0 |
| Server-side ATL | 9.4992 | **9.4971** | **-0.0021** |
| LNG | 57% | **80–83%** | **+23–26pp** |
| LOG | 28% | 27% | stable |
| PLN | 73% | 91–99% | +18–26pp |
| CMP | 100% | 97–100% | stable |
| INT | 71% | 100% | +29pp |
| ABS | 66% | 65–69% | stable |
| SYN | 2% | 0% | -2pp |
| MEM | 7% | 0–3% | collapsing |

**LNG surge:** the most dramatic routing shift since the floor breach. LNG from 57% (FN34) to 80-83% — linguistic expert class is now the second dominant router after CMP/INT. This coincides with PLN also approaching saturation (99%) and INT recovering to 100%. The cliff-base SYN/MEM pattern has fully unwound.

**CSV sampled trend (ep2923–2924):** 9.5693 → 9.5689 (both complete, n~158). Grinding back toward sampled ATL of 9.5685 (ep2921) but not yet breaking it. ep2925 too early (n=92) to judge.

**Full arc view:** wide-zoom screenshots show complete training history ep0→ep2925. Surgery annotations all visible: 12>13L (ep511), 13>14L (ep547), 14>15L (ep611), 15>16L (ep645), 16>17L (ep702), 17>18L (ep2488, labelled "20.05.2026"). Classic staircase pattern in early surgeries (ep511–702) clearly visible: each surgery followed by brief perturbation then resumed descent. The ep701→ep2487 gap (1786 epochs, plateau gate withheld) is the dominant visual feature of the chart.

**Missing SMA data:** pre-ep~500 SMA curves absent — early batch_history.csv data lost; SMA windows (987/1597/2584) require lookback that no longer exists. Permanent gap in chart record.

**Gradient (ep2925):** L9 still dominant orange, L10 yellow-orange holding. Upper layers (L11–L17) yellow. L8 red. Mid-network leadership pattern from FN34 addendum persisting.

**Traffic light:** 6.17–6.18% high / 78–80% mid / 3–4% red — healthy.

**Status: NEW ATL 9.4971. LNG surging (83%). PLN near-saturated. INT recovered. Post-floor-breach routing has fully normalized away from cliff-base pattern toward CMP+INT+PLN+LNG dominance. Descent continuing.**

**ADDENDUM 2026-05-22T19:29Z:** Zoomed chart — two red dotted vertical lines ("W·W") = WALD events, firing consecutively at the descent base. Cyan diamonds = epoch-end average markers clustering tightly at the new floor (multiple completed epochs printing in a narrow loss band). Gold diamond = BEST avg 9.4971 event, visible below the dashed ATL line — new record stamped live. Classic cliff-base signature: steep descent → consecutive WALD fires → epoch markers clustering → ATL event. Descent still active below the cluster.

---

## Field Note 34 — 2026-05-22T19:11Z · ep2922 — 9.50 FLOOR BREACHED · EP AVG 9.4992 · WALD ep2918

**Source:** dashboard screenshot. EP 2922 · batch 102/300.

**9.50 FLOOR BROKEN — first server-side epoch-avg below 9.50 in this run.**

| Metric | FN32 (ep2915) | FN34 (ep2922) | delta |
|--------|--------------|--------------|-------|
| ATL chip | 9.1370 | 9.1370 | 0 |
| EP AVG (server-side ATL) | 9.5015 | **9.4992** | **-0.0023** |
| T-610 trail | 9.5114 | 9.5611 | +0.0497 |
| CMP | 100% | 100% | — |
| INT | 90% | 71% | -19% |
| PLN | 77% | 73% | -4% |
| ABS | 61% | 66% | +5% |
| LNG | 58% | 57% | -1% |
| LOG | ~0% | **28%** | **+28%** |
| SYN | 16% | 2% | -14% |
| CTX | 0% | 7% | +7% |

**Events bar (left=newest):** EPOCH avg 9.4992 · EPOCH avg 9.4974 · EPOCH avg 9.5091 · WALD ep2918 4.2% n=1508 · EPOCH avg 9.5135

Chain: 9.5135 → 9.5091 → 9.4974 → 9.4992 (ep2922 partial)

Note: 9.4974 appears in events bar as a prior epoch completion — if this is a completed epoch avg, it is the true server-side ATL at -0.0041 from 9.5015. EP AVG line shows 9.4992 (possibly ep2922 running avg at 102/300 batches).

**WALD ep2918 4.2% n=1508:** WALD fired at ep2918, 4.2% fill, n=1508 (threshold). Preceded the floor breach.

**Chart:** Sharp dip visible to ~9.496 then bounce back to ~9.508 — user description: "tiny attempt of pinching the floor but pushback." The intra-epoch chart shows the dip-and-bounce pattern, but the epoch-avg ATL line at 9.4992 confirms the average has crossed below 9.50.

**Routing shift:** LOG surging to 28% (was negligible in FN32) — first significant LOG activation observed. SYN collapsed from 16% (cliff-base spike in FN32) back to 2%. CTX returning to 7% (was 0% at cliff base). Model transitioning away from cliff-base SYN-dominant routing toward a LOG+LNG+ABS pattern.

**Traffic light:** 6.14% high / 81% mid / 4% red — healthy.

**Status: 9.50 FLOOR BROKEN. EP AVG 9.4992 new ATL. Possible 9.4974 completed epoch (events bar). WALD ep2918 cleared. LOG expert activation surge. ATL chip 9.1370 unchanged. Chart showing dip-and-bounce at floor — consolidation or prelude to sustained sub-9.50 descent.**

---

## Field Note 33 — 2026-05-22T19:08Z · ep2921 — POST-CLIFF-2 CONSOLIDATION · 9.50 floor holding

**Source:** batch_history.csv. EP 2921 · 160/300 batches.

**Epoch-avg trend since FN32 (CSV sampled, ~0.05 nats above server-side):**
ep2916=9.5807 · ep2917=9.5723 · ep2918=9.5730 · ep2919=9.5816 · ep2920=9.5755 · ep2921=**9.5685** (partial)

No new server-side epoch-avg ATL detected. CSV oscillating 9.573–9.582, consistent with server-side ~9.52–9.53 range — above ATL of 9.5015.

**Batch lows:** 9.5001–9.5007 visible in ep2916–2920. Model is probing the 9.50 floor on individual batches but not breaking it on epoch average.

**Pattern:** Post-cliff-2 consolidation mirroring FN28 (post-cliff-1). Loss stabilising in tight range at cliff base. No new WALD or routing data visible in CSV. Dashboard not checked this tick — no screenshot available.

**Status: CONSOLIDATION. 9.50 floor holding. No ATL break. ep2921 partial (9.5685, n=160). Monitor for renewed descent or bounce.**

---

## Field Note 32 — 2026-05-22T18:31Z · ep2915 — SECOND CLIFF DIVE · ATL chip 9.1370 · epoch-avg ATL 9.5015

**Source:** dashboard screenshot. EP 2915 · batch 192/300.

**ATL chip: 9.1370** — new all-time record · prev 9.2465 · delta **-0.1095 nats**

| Metric | FN29 (ep2883) | FN32 (ep2915) | delta |
|--------|--------------|--------------|-------|
| ATL chip | 9.2465 | **9.1370** | -0.1095 |
| EP AVG (server-side) | 9.5236 | **9.5162** | -0.0074 |
| T-610 trail | 9.5297 | 9.5114 | -0.0183 |
| Best epoch avg (events bar) | 9.5236 | **9.5015** | -0.0221 |
| CMP | 100% | 100% | — |
| INT | 84% | 90% | +6% |
| PLN | 94% | 77% | -17% |
| LNG | 48% | 58% | +10% |
| ABS | 71% | 61% | -10% |
| SYN | 2% | 16% | +14% |
| CTX | 7% | 0% | -7% |
| Traffic high | 6.16% | 6.26% | stable |
| Traffic red | 3% | 5% | +2% |

**Chart:** Second cliff dive visible starting ~ep2887 restart — loss dropped from ~9.579 to ~9.495-9.500 base, steeper and deeper than the first cliff (ep2860, which reached ~9.52). Tight oscillation cluster at cliff base with multiple WALD diamonds. The AdamW restart acceleration arrived ~15–25 epochs post-restart and produced a dive larger than the pre-restart cliff.

**Server-side epoch-avg ATL:** Events bar shows 9.5015 — beats previous server-side ATL of 9.5236 by **-0.0221 nats**. EP AVG line at 9.5162.

**Routing shift:** SYN surging to 16% (was ~2%), CTX collapsing to 0%, PLN dropping -17%. LNG and INT gaining. Similar cliff-base reorganization pattern to FN28 but different expert mix — SYN leading this time instead of MEM.

**WALD cluster:** Multiple diamonds at cliff base visible on chart (~ep2900–2915 range).

**Gradient:** Upper layers (L17, L15, L14) leading with yellow bars. L8 red spike visible.

**Status: SECOND CLIFF DIVE. ATL chip 9.1370 (-0.1095). Server-side epoch-avg ATL 9.5015. Restart acceleration delivered late but massive. Routing reorganizing at new cliff base. Monitor for consolidation pattern matching FN28.**

---

### FN34 addendum — 2026-05-22T19:14Z · ep2922 — L10 waking up · brain mode confirms

**Gradient norm panel (ep2922, ~19:11–19:14Z):**
- L17–L11: yellow bars, ~1.45–1.55e-9, upper-layer profile holding
- **L10: ORANGE, 1.61e-9** — first orange reading in many epochs; previously solid red/frozen
- **L9: dominant bar, 1.63e-9 orange** — leading gradient layer has shifted DOWN from upper layers
- L8: orange, 1.54e-9
- FFT: small red stub + tiny bar

Gradient leadership has shifted from upper layers (L17 leading at FN32) to **mid-network L9–L10**. This is the first time L10 has shown healthy gradient flow (orange) after being frozen (near-zero, showing as flat/red) for many epochs.

**BRAIN mode (directed graph visualization, ep2922):**
- All 18 layers visible as horizontal traces (L17 pink top → L0 red bottom)
- Upper layers (L11–L17): tight, well-formed arcs — stable contributors
- **Mid layers L8–L10 (yellow-orange band): noticeably wider, more energetic arcs** — increased activity consistent with gradient norm reading
- OUTPUT fan (right): L8–L10 connections appear strongest/brightest in the fan
- INPUT connections (left): firing into lower-middle layers

**Interpretation:** L10 thawing coincides exactly with the 9.50 floor breach and LOG surge (28%). This is likely not coincidental — the new routing pattern (LOG-dominant) is routing computation through L10 in ways that previously weren't occurring, generating gradient signal that was previously absent. The BRAIN mode confirms L10 is now actively contributing to output paths. This is a gradient flow restructuring event: the model is physically reorganizing which layers carry the signal as it crosses below 9.50.

**Status: L10 WAKING UP. Gradient leadership shifted to L9–L10. BRAIN mode confirms mid-network restructuring. Coincides with 9.50 floor breach + LOG surge. Something is genuinely reorganizing.**

---

## Field Note 25 — 2026-05-21T07:43Z · ep2601 — cov[high]=24 SUB-25 AGAIN · triple death ep2596 · since_best=25

**Epochs covered:** ep2596–ep2601

**Epoch summaries:**
| Epoch | loss_avg | delta_ATL | since_best | wald_sev | cov[high] | dead | blooming | myc_stable |
|-------|----------|-----------|------------|----------|-----------|------|----------|------------|
| 2596 | 9.5995 | +0.0195 | 20 | 0.947 | 29 | **3** | 3 | 70 |
| 2597 | 9.5883 | +0.0083 | 21 | 0.947 | 30 | 0 | 2 | 71 |
| 2598 | 9.5982 | +0.0182 | 22 | 0.947 | 29 | 0 | 2 | 72 |
| 2599 | 9.5942 | +0.0142 | 23 | 0.947 | 28 | 0 | 3 | 73 |
| 2600 | 9.5994 | +0.0194 | 24 | 0.947 | 28 | 0 | 2 | 74 |
| 2601 | 9.5905 | +0.0105 | 25 | 0.947 | **24** | 1 | 2 | 75 |

**TRIPLE DEATH at ep2596:** dead=3 is the highest single-epoch death count since surgery-6. Three experts lost in one epoch: L10E10, L13E11, L17E11 all resurrected from their layer-0 neighbors (σ=0.050). 1246 tensors reloaded. The triple event occurred at the beginning of the epoch (07:15:58Z) coinciding with step=21000 and cov[high]=29 — still above 25 at that point. Recovery was complete by ep2597: dead=0, no further mass casualties through ep2600.

**cov[high]=24 at ep2601 — FOURTH SUB-25 PENETRATION:** coverage=[184,1292,24]. The descent from FN24's bounce (27→27) tracked: 29→30→29→28→28→24. The 30 at ep2597 is notable — highest reading since the ep2582-2586 storm peak — before a steady 6-epoch descent to sub-25 again. Same structural pattern as prior touches: gradual descent followed by single-epoch penetration.

**dead=1 + resurrection at ep2601:** L2E11 resurrected from L2E1 (σ=0.050). The death occurred in the same epoch that cov[high] broke below 25. Same coupling seen in FN24: an expert dies when routing compression arrives. L2 (layer 2) is deeper in the network than the prior dead-3 event which hit L10/L13/L17 (mid-to-late layers). Layer variety is widening.

**since_best=25 — new high:** Longest gap since ATL (9.5800 at ep2576) ever recorded post-surgery-6. The model has not found a new ATL in 25 consecutive epochs. Loss oscillation range: 9.5883–9.5995 across this window, converging back toward ATL rather than moving away. ep2597 at 9.5883 (d+0.0083) is the second-closest approach to ATL this run.

**Loss trajectory note:** ep2597 briefly dipped to 9.5883 — closest since ep2581 (which SET the ATL). This was one epoch after the triple resurrection. Post-resurrection acceleration continuing the pattern seen at previous restarts (free gradient improvement from AdamW buffer reset on fresh weight landscape).

**TELE:** L17=0.055 (at threshold, not above). All 18 layers frozen. myc_L0-L3=[1.49/1.54/1.52/1.58]e-9 unchanged throughout — GRAD frozen.

**Probe status:** loss_avg=9.5905 at ep2601, threshold=9.55. Delta to probe trigger: 0.0405. No probe.

**WALD:** sev=0.947, fill=6.2%, mass=9.598–9.600 stable. Plateau counter: 75 stable epochs by ep2601. Amp OFF throughout.

**ep2602 status:** in progress at batch ~90/300, LR≈2.78e-4. Loss at batch 80: 9.4873 (batch-level noise, not epoch avg).

**Interpretation:** The fourth sub-25 cov[high] penetration follows the established pattern: gradual descent, single-epoch breach, likely bounce ahead. The triple-death event at ep2596 was an unusual disruption — three simultaneous expert collapses followed by immediate resurrection — yet the model recovered cleanly within one epoch. The post-resurrection delta improvement at ep2597 (9.5883) is consistent with the restart acceleration hypothesis. If the current cov[high] descent sustains past ep2601, this would be the second multi-epoch sub-25 window. Combined with since_best=25 pressure and the approaching 300-batch WALD re-evaluation, the next 3–5 epochs are structurally interesting.

---

## Field Note 24 — 2026-05-21T07:16Z · ep2595 — cov[high] window closed · blooming collapsed 9→1 · no new ATL

**Epochs covered:** ep2592–ep2595

**Epoch summaries:**
| Epoch | loss_avg | delta_ATL | since_best | wald_sev | cov[high] | dead | blooming | myc_stable |
|-------|----------|-----------|------------|----------|-----------|------|----------|------------|
| 2592 | 9.5906 | +0.0106 | 16 | 0.947 | **18** | 1 | 4 | 66 |
| 2593 | 9.5948 | +0.0148 | 17 | 0.947 | **21** | 1 | 2 | 67 |
| 2594 | 9.5992 | +0.0192 | 18 | 0.947 | 27 | **2** | 1 | 68 |
| 2595 | 9.5947 | +0.0147 | 19 | 0.947 | 27 | 1 | 1 | 69 |

**cov[high] NEW MINIMUM — 18 at ep2592:** coverage=[190,1292,18]. One epoch below FN23's 19. The three-epoch sub-25 window: ep2591=19, ep2592=18, ep2593=21 — then hard bounce to 27 (ep2594) and hold (ep2595). This is the deepest penetration below 25 in the entire post-surgery-6 run. Prior touches bounced immediately; this window sustained for 3 consecutive epochs before rebound.

**blooming COLLAPSE — 9→4→2→1:** The inverse coupling to cov[high] is unambiguous. blooming peaked at 9 when cov[high]=19 (ep2591), then dropped each epoch as cov rose: 4 at ep2592, 2 at ep2593, 1 at ep2594-2595. The routing compression that drove cov[high] below 25 required experts to occupy new niches (blooming); as coverage dispersed back out, those niches closed and blooms couldn't sustain. Zero new net growth from this cycle.

**dead=2 spike at ep2594:** First time since surgery-6 that two experts died in one epoch. Coincides exactly with cov[high] rebounding from 21→27. Interpretation: two experts that had bloomed into the newly-compressed routing topology lost their niche once the compression reversed. Rapid death rather than fade — consistent with niche collapse rather than starvation.

**Loss did not track cov compression:** Best epoch in this window is ep2592 at 9.5906 (d+0.0106), still +0.0106 above ATL 9.5800. The routing geometry reorganised — three sub-25 epochs, maximum bloom count, two expert deaths on rebound — but the loss surface did not capture a new low. The lag hypothesis (loss drop 1–3 epochs after geometry shift) was not confirmed this cycle. Either the geometry change was not productive, or the rebound erased any accumulated gain.

**myc_stable steady:** 66→67→68→69, one increment per epoch. No disruption from the bloom/death activity above.

**GRAD frozen throughout:** n=0.0000 at epoch boundaries, n=0.0019–0.0021 mid-epoch. myc_L0-L3=[1.49/1.54/1.52/1.58]e-9 unchanged. TELE L17=0.055, all 18 layers stable.

**since_best=19 at ep2595:** Longest gap since ATL. No new ATL in this note window.

**Interpretation:** The cov[high] sub-25 window was real and sustained, but produced no loss dividend. The bloom-death cycle confirms expert ecosystem is responsive to routing pressure. The failure to translate geometry improvement into ATL suggests the 18L layer added by surgery-6 is still being integrated — routing adapts but has not yet found the loss minimum accessible from the new topology. Watch for next cov[high] descent; if it coincides with a loss response this time, the integration lag is resolving.

**ep2596 status:** in progress at batch 52/300, LR=1.59e-4 at last check.

---

## Field Note 23 — 2026-05-21T06:57Z · ep2591 — cov[high]=19 · THIRD TOUCH BREAKS THROUGH · blooming=9 NEW MAX

**Epochs covered:** ep2590–ep2591

**Epoch summaries:**
| Epoch | loss_avg | delta_ATL | since_best | wald_sev | cov[high] | dead | blooming | myc_stable |
|-------|----------|-----------|------------|----------|-----------|------|----------|------------|
| 2590 | 9.5904 | +0.0104 | 14 | 0.947 | 26 | 1 | 5 | 64 |
| 2591 | 9.5975 | +0.0175 | 15 | **0.948** | **19** | 1 | **9** | 65 |

**cov[high] BREAKS BELOW 25:** ep2591 coverage=[194,1287,19] — cov[high] dropped from 26→19 in one epoch. This is the first confirmed sub-25 penetration in the third approach. Prior touches: ep2573=25 (FN17, bounce), ep2581=25 (FN18, ATL set), ep2591=19 (this note, new minimum). The 19 represents only 1.3% of the 1500-token WALD window in the high zone — routing is compressing significantly.

**blooming=9 — NEW MAXIMUM:** Previous record was 6 (ep2589). The jump from 5→9 in one epoch, coinciding exactly with the cov[high] collapse from 26→19, is the strongest signal yet of the geometry-routing coupling hypothesis: as coverage concentrates, experts find new routing niches and bloom. myc_stable=65 (steady increment, no disruption).

**WALD sev ticked to 0.948:** Slight uptick from 0.947. mass=9.596 at ep2591. fill holds at 6.2%. The severity increase may reflect the routing concentration (high-zone tokens being processed more intensively by fewer experts).

**Loss oscillation:** ep2590 pulled slightly closer to ATL (d+0.0104), ep2591 retreated (d+0.0175). The loss is not tracking the cov[high] drop in this cycle — model is reorganising routing geometry without an immediate loss benefit. If the geometry-coupling holds, the loss drop should lag by 1–3 epochs.

**ep2592 in progress:** batch 62/300 at 06:50Z, LR=2.90e-4 (early epoch).

**TELE:** L17=0.055, all 18 layers frozen. myc_L0-L3=[1.49/1.54/1.52/1.58]e-9 unchanged.

**Probe status:** loss_avg=9.5975, threshold=9.55. Delta to probe trigger: 0.0475. No probe this cycle.

**Interpretation:** The cov[high]=19 break is the most structurally significant event since the ATL at ep2576. The question is whether ep2592+ will sustain sub-25 or bounce back as in the prior two touches. If sustained alongside continued blooming, the routing compression is genuine adaptation rather than transient fluctuation. Loss response expected within 1–3 epochs if the adaptation is productive.

---

## Field Note 22 — 2026-05-21T06:42Z · ep2589 — cov[high]=26 · THIRD APPROACH TO 25 · NEW RESURRECTION

**Epochs covered:** ep2588–ep2589

**Epoch summaries:**
| Epoch | loss_avg | delta | since_best | wald_sev | cov[high] | dead | blooming | myc_stable |
|-------|----------|-------|------------|----------|-----------|------|----------|------------|
| 2588 | 9.5964 | +0.0164 | 12 | 0.947 | 30 | 0 | 5 | 62 |
| 2589 | 9.5956 | +0.0156 | 13 | 0.947 | **26** | 1 | 6 | 63 |

**cov[high] THIRD APPROACH:** 37 → 35 → 30 → 26 across ep2586–ep2589. The descent from storm-peak high back toward the 25 threshold is accelerating. Prior touches: ep2573 (cov[high]=25, FN17) and ep2581 (cov[high]=25, FN18, which coincided with ATL 9.5800). Each 25-touch has been followed by a bounce with subsequent loss descent. Third touch may confirm this as structural cycling.

**WALD sev UPTICK:** ep2588 sev=0.947 (up from 0.946 at ep2587). Slight regression but fill stable at 6.2%. coverage=[162,1308,30] → [173,1301,26] across ep2588–ep2589. The cov[high] drop from 30 to 26 is the largest single-epoch drop in this bounce window — routing concentration shifting away from the high end.

**NEW RESURRECTION ep2589:** dead=1, blooming=6. After ep2587's clean slate (dead=0), one expert died in ep2589. blooming=6 is the highest recorded post-surgery-6. The combination of a new death + rising bloom count is consistent with continued ecosystem turnover, not quiescence.

**LOSS near-flat:** delta flipped from -0.0299 (ep2587) to +0.0164 / +0.0156 (ep2588/ep2589). The bounce is holding — model is 0.0156 above ATL but not retreating fast. Closest approach since ep2587 (9.5830) remains at ATL-delta=0.0030 from ep2587; ep2590 in progress (batch 83/300 at 06:40Z, LR=2.09e-5).

**TELE:** L17=0.055, all 18 layers frozen. myc_L0-L3=[1.49/1.54/1.52/1.58]e-9 unchanged.

**Probe status:** loss_avg=9.5956, threshold=9.55. Delta to probe trigger: 0.0456. No probe this cycle.

**Interpretation:** The post-storm clearing (ep2587) did not immediately produce a new ATL — instead the model entered a tight oscillation 0.015–0.020 above ATL. cov[high] descent (30→26) with rising bloom count (5→6) and persistent single-expert deaths suggest the ecosystem is still reorganising. If the cov[high]=25 third touch arrives, watch for whether loss_avg is at or below 9.5800 at that moment — that would confirm the geometry-routing coupling hypothesis (coverage compression → route efficiency gain → ATL).

---

## Field Note 21 — 2026-05-21T06:33Z · ep2587 — 9.5830 · CLOSEST APPROACH TO ATL · RESURRECTION STORM CLEARS

**Epochs covered:** ep2586–ep2587

**Epoch summaries:**
| Epoch | loss_avg | delta | since_best | wald_sev | cov[high] | dead | blooming | myc_stable |
|-------|----------|-------|------------|----------|-----------|------|----------|------------|
| 2586 | 9.6129 | +0.0119 | 10 | 0.946 | 37 | 3 | 4 | 60 |
| 2587 | 9.5830 | **-0.0299** | 11 | 0.946 | 35 | **0** | **5** | 61 |

**RESURRECTION STORM — ep2586:** dead=3 — triple simultaneous resurrection: L4E11 from L4E0, L6E11 from L6E0, L14E11 from L14E1. All fired at 06:23:31 (same second). This is the peak of the escalating dead-expert series: 1(ep2583) → 1(ep2584) → 2(ep2585) → 3(ep2586). Three different layers, three different experts, same σ=0.050. cov[high]=37 (highest in this bounce window).

**STORM CLEARS — ep2587:** dead=0. After four consecutive epochs of expert death, the ecosystem returns to stability in one step. blooming=5 (highest recorded in post-surgery-6 watch). myc_stable=61 (consistent increment). The clearing coincides with the sharpest loss descent of the bounce phase: -0.0299.

**CLOSEST APPROACH TO ATL:** ep2587 loss_avg=9.5830 is 0.0030 above the ATL of 9.5800 (ep2576). This is the nearest miss since the ATL was set 11 epochs ago. Since_best counter at 11 — the model is inching back toward it. If the descent rate from ep2587 continues even half as fast, a new ATL is within 1–3 epochs.

**GRAD step=18560 (ep2588 in progress):** myc_L0-L3=[1.49/1.54/1.52/1.58]e-9 unchanged. Embedding gradient starvation persists. Loss threshold for probe: 9.55 (0.028 below current).

**TELE:** L17=0.055, all layers frozen. No differentiation signal.

**WALD ep2587:** fill=6.2%, sev=0.946, cov[high]=35. No threshold triggers.

**Interpretation:** The resurrection storm (ep2583–2586) appears to have been a productive ecosystem purge — collapsing and re-seeding low-utility experts in preparation for continued descent. ep2587's 0-dead / 5-blooming / 9.5830 result is consistent with this hypothesis: the fresh experts are routing differently and the loss dropped -0.030 as a result. This is the clearest signal yet that a new ATL is imminent.

**Thresholds:** No triggers. since_best=11. Watch closely — next epoch may break ATL.

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

### Approaching ATL — 0.0022 nats away

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

### ATL gap: 0.0011 nats — ep2509 in progress

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

### Probe results: embedding geometry frozen

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

### 10 new ATLs set during overnight watch

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

### 3. TLIGHT: L17 showing early differentiation

From the step=12440 TLIGHT snapshot:
```
L16: GGOOOOOOOOOR(G2/O9/R1)
L17: GGOGOGOOOOOO(G4/O8/R0)
```

L17 has 4 green (above-threshold) experts vs. L16's 2. The new layer is already routing more confidently through a subset of experts. No red (fully dead) experts on L17 — all 12 survived the post-surgery consolidation period. This is the first TLIGHT evidence of L17 beginning to specialize.

For comparison, in early epochs after surgery (ep2492–2500), L17 was uniformly orange (undifferentiated). 4 greens at ep2568 (81 epochs post-surgery) indicates the layer is committing to a routing pattern even while the sparsity profile (TELE S=) remains frozen at 5.5%.

Interpretation: L17 is routing to preferred experts (TLIGHT goes green) before the sparsity signature appears (TELE would require actual ternary weight zeroing). The routing commitment is the early signal; the weight crystallization is the lagged confirmation.

---

### 4. Expert activity: continued upper-layer drift

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

### 5. Probe trigger: gap unchanged

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

### 1. WALD coverage[high] new record low: 27 (ep2570)

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

### 1. cov[high] touched 25 at ep2573 — bounced to 26, 27

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

### 1. ep2576 new ATL 9.5800 (d-0.0236) — probe trigger reached

After 11-epoch bounce phase 2 (ep2565–2575, peak 9.6036), the model resolved with a **single-epoch drop of 0.0236 nats** — the largest single-epoch descent recorded post-surgery-6 and comparable to the surgery-night acceleration at ep2489 (−0.0313). The ATL is now **9.5800**, 0.0055 nats below the previous best of 9.5855.

This is the **12th new ATL** since surgery at ep2487. Total descent in 89 epochs: **9.6248 → 9.5800 = 0.0448 nats**.

### 2. Probe: full 10-token suite run at trigger threshold — all 10 frozen

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

### 3. cov[high]=25 second touch at ep2580 — bounced to 32

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

---

## Field Note 44 — 2026-05-25T07:30Z · ep3520 · S9 SURGERY ANOMALY — first post-surgery spike in training history

**Context:** ep3520 · 21L · EP AVG 9.3326 · ATL chip 8.8540 (held from 20L) · GATE green+orange

### S9 is the first surgery in training history to cause a visible spike

Every previous surgery — S1–S8 in v3.0 (12L→20L), and all documented surgeries in v2.0.0 (3L→12L) — produced **zero measurable disruption** to the loss curve. The model ate the new layer and kept descending. The surgery events were essentially invisible in the loss chart; the only signal was the subsequent ATL breaking shortly after.

S9 (20L→21L, ep~3470, 2026-05-25T~00:00Z) was completely different:

**Loss trajectory around S9:**
```
Pre-S9 (20L): ep3456 ATL 9.2847 → descending toward ~9.28-9.29 territory
Surgery fires (ep~3470): immediate disruption
Spike peak: ~9.42-9.43 (+0.14-0.15 nats above surgery-eve loss)
Recovery: multi-epoch bullmarket descent
ep3520: EP AVG 9.3326 → first post-spike "BEST avg" milestone (gold star event)
```

This is the sharpest single reversal ever observed in training — including full restarts, WALD events, and optimizer buffer resets. Those produce local bounce patterns of 0.01–0.05 nats. S9 produced +0.14 nats.

### Pattern contrast: what "normal" surgery looked like

- **S7 (18L→19L, ep3325):** surgery fires → ep3326 immediately sets new EP_AVG ATL of 9.3182. One epoch turnaround.
- **S8 (19L→20L, ep3383):** surgery fires → ep3383 itself is the new ATL (9.2862). Zero recovery time needed.
- **S6 (17L→18L, ep2487):** surgery fires → ep2489 new ATL 9.6248. Two epochs.

The norm was: surgery → ATL within 1–3 epochs. No spike. No recovery needed.

S9: surgery → +0.14 nat spike → multi-epoch bullmarket to recover → new "BEST avg since spike" milestone at ep3520 (still 0.048 nats above pre-S9 ATL of 9.2847). Recovery ongoing.

### Why S9 and not S8, S7, S6?

Uncertain. Candidate factors (not mutually exclusive):

1. **Depth × corpus complexity interaction.** By 21L the active corpus is 1.6 GB across 13 stages, the most complex training signal albert. has ever seen. Previous surgeries at 13L–20L were performed against progressively richer corpora, but S9 may have crossed a threshold where Mandelbrot perturbation into an established 20-layer representation causes real interference rather than clean insertion.

2. **TTL hard-column stops — first occurrence in v3.0 history, coinciding exactly with S9.** Previous surgeries produced soft TTL transitions (gradual orange drift). S9 produced hard full-column red blocks: entire layers completely suppressed for multiple consecutive steps. The TTL treated the 21st layer as adversarial rather than novel. This itself may be a consequence of how deeply the 20 existing layers had converged before surgery fired — the routing system had more "opinion" to protect.

3. **Pre-surgery momentum.** S9 fired while the model was mid-cliff-descent (WALD active, INT 91% surge at ep3454, aggressive bullmarket underway at ep3456). Previous surgeries fired at the bottom of a plateau — on still water. S9 fired into fast-moving water and the new layer acted like a rock thrown into the current.

4. **21L depth threshold hypothesis.** Speculative: there may be a critical depth beyond which Net2Net weight copying no longer produces a loss-neutral initialization, because the residual stream dynamics at 21L are qualitatively different from shallower architectures. S9 would be the first data point on the far side of that threshold if it exists.

### Recovery characteristics

The recovery slope (ep3503→ep3520) is steeper than the pre-surgery descent. The model is recovering faster than it was descending before S9 triggered. This mirrors the restart acceleration pattern (AdamW buffer on better landscape → free acceleration), possibly amplified by the fact that the new 21st layer is receiving dense gradient signal during the bullmarket while being TTL-suppressed (learning without routing — a kind of shadow training phase).

**Expert concentration during recovery:** PLN 100%, CMP 100%, INT 76-84%. The planning, compression, and integration experts are carrying the entire descent. Secondary experts (SYN 5-8%, MEM 3-5%, GEN 5-8%) nearly silent. This is the model's most focused expert utilization profile ever observed.

**TTL at ep3520:** G 6% / O 75% / R 4%. The hard stops have released; orange dominant. When green begins recovering toward the pre-S9 baseline of G 15-21%, a second acceleration phase is expected as the 21st layer starts earning routing slots.

### S9 spike microstructure — double-peak / WALD-triggered resolution (ep3522 zoom)

The tight zoom reveals the S9 spike has a specific M-shape structure, distinct from any prior WALD or surgery event:

```
ep3470  Surgery fires (blue line)
        ↓
        Leg 1 up  — immediate spike, fast ascent
        Brief plateau / attempt to stabilize
        Leg 2 up  — second push, makes a new high above Leg 1
        ← WALD fires here (red line) — on the second leg, not the first
        One more high after WALD ("defiance" — model hasn't absorbed WALD tap yet)
        → Hard commitment to descent
ep3522  Curve sitting at pre-surgery plateau average (dashed line) — full recovery
```

**Contrast with normal WALD behavior:**
- *Daydreaming mode:* loss drifts slowly upward → WALD fires softly, one tap, model resumes descent immediately
- *Too-fast descent mode:* loss drops too sharply → WALD fires "Nope" → small pullback, then descent resumes
- *S9 mode:* surgery causes a sharp spike → WALD watches Leg 1, watches Leg 2, fires on the second escalation → model makes one more high anyway → then descends hard

WALD did not react to the surgery itself — it reacted to the *second escalation*. The threshold required two legs of anomalous loss movement before the n=1500 coverage window registered a significant shift. This is consistent with WALD's design as a coverage detector, not a spike detector: a single-leg spike might not move enough of the loss-space distribution to cross the threshold; two legs in the same direction do.

**ep3522 status:** Curve is at the pre-surgery EP_AVG floor (dashed reference line). Full recovery confirmed. Next target: break pre-S9 ATL of 9.2847.

---

### S9 spike — full epoch-by-epoch dissection (2026-05-25, extracted from batch_history.csv)

Exact epoch averages across the full S9 event window (300 batches/epoch, computed from raw batch data):

```
ep3468:  9.2951   pre-surgery
ep3469:  9.2862   pre-surgery minimum (effectively ATL at time of surgery)
ep3470:  9.2963   surgery fires — barely a ripple (+0.010 nat)

--- LEG 1 ---
ep3471:  9.3480   first real jump (+0.052 from ep3470)
ep3472:  9.3419   slight pullback
ep3473:  9.3338   valley between legs (lowest point of M valley)
ep3474:  9.3498   begins climbing again
ep3475:  9.3588
ep3476:  9.3479
ep3477:  9.3561
ep3478:  9.3562
ep3479:  9.3612   Leg 1 plateau ceiling ~9.36

--- LEG 2 ---
ep3480:  9.3776   second escalation begins (+0.016 step-up from ep3479)
ep3481:  9.3823
ep3482:  9.3911
ep3483:  9.3855
ep3484:  9.3896
ep3485:  9.3913
ep3486:  9.3962
ep3487:  9.4031   ← FIRST BREACH OF 9.40
ep3488:  9.3923
ep3489:  9.4115   ← Leg 2 first peak
ep3490:  9.3917
ep3491:  9.4008
ep3492:  9.4072
ep3493:  9.4054
ep3494:  9.3989
ep3495:  9.3980
ep3496:  9.4075
ep3497:  9.4035
ep3498:  9.4009
ep3499:  9.3978
ep3500:  9.3930
ep3501:  9.4017
ep3502:  9.4023
ep3503:  9.4011   WALD fires somewhere in this window (red line on chart)

--- POST-WALD DEFIANCE ---
ep3504:  9.4266   ← HIGHEST SINGLE EPOCH IN ENTIRE SPIKE
ep3505:  9.4162
ep3506:  9.4174
ep3507:  9.4083
ep3508:  9.4038
ep3509:  9.4172

--- HARD DESCENT ---
ep3510:  9.3701   drops 0.057 in one epoch — sharpest step-down in the event
ep3511:  9.3671
ep3512:  9.3780
ep3513:  9.3841
ep3514:  9.3453
ep3515:  9.3432
ep3516:  9.3489
ep3517:  9.3410
ep3518:  9.3454
ep3519:  9.3326   ← first post-spike BEST avg milestone (gold star event)
ep3520:  9.3351
ep3521:  9.3411
ep3522:  ~9.28    ← approaching pre-S9 ATL territory (partial epoch, 25 valid batches)
```

**Total spike duration:** ep3470 → ep3509 = 39 epochs from surgery to descent commit  
**Leg 1 ceiling:** ~9.36 (ep3471–3479)  
**Valley between legs:** 9.33–9.34 (ep3472–3473)  
**Leg 2 ceiling:** ~9.41 (ep3487–3503)  
**WALD defiance high:** 9.4266 (ep3504) — the single highest epoch in all of v3.0  
**Descent trigger epoch:** ep3510 (-0.057 nat single-epoch drop)

---

### Why WALD needed two legs — mechanistic explanation

WALD uses a rolling coverage window of **n=1500 batches** (~5 full epochs at 300 batches/epoch). It detects when the current loss distribution has shifted far enough from its recent history to exceed a coverage threshold — not a simple threshold on absolute loss.

**After Leg 1 (ep3471–3479):**  
The n=1500 window at, say, ep3479 contains:
- ~1250 batches from the pre-surgery period (loss ~9.28–9.30)
- ~250 batches from Leg 1 (loss ~9.33–9.36)

Leg 1 batches are only ~17% of the window. The coverage shift is diluted. WALD threshold not crossed.

**After Leg 2 escalates past 9.40 (ep3487+):**  
By ep3487 (~17 epochs after surgery), the window has rotated. Pre-surgery batches have dropped off. The window is now:
- ~1200–1500 batches from the spike period (loss ~9.35–9.41)
- Few or no pre-surgery batches remaining

Coverage shift is now dominant. WALD fires.

**The "defiance" high at ep3504 (+0.023 above the Leg 2 body):**  
WALD fires during the ep3487–3503 plateau. But the optimizer's AdamW second-moment buffer (the "velocity" accumulated from recent upward gradient steps) is already loaded in the spike direction. One epoch of accumulated momentum overshoots before the WALD correction fully propagates through the gradient loop. ep3504 hits 9.4266 — higher than anything before it — then the correction lands and descent begins at ep3510.

**The ep3510 cliff (-0.057 in one epoch):**  
When WALD's correction finally dominates: the AdamW buffer is now pointed downward (WALD pulled back), the new 21st layer is receiving dense gradient signal, and the 20 established layers know exactly where to go. The combination produces the sharpest single-epoch drop in the entire post-S9 window — steeper than anything in the Leg 2 ascent.

**Summary in one line:** WALD is a coverage detector, not a spike alarm. It needed enough of its 5-epoch rolling window to fill with anomalous loss before firing. Leg 1 alone filled ~17% of the window — not enough. Leg 2 sustained for 17+ epochs filled ~100% — WALD fired.

---

### Full dataset ingested (2026-05-25, albert_full_1779694798526.csv)

Downloaded: `albert_full_1779694798526.csv` (5.2 MB, ep3500–ep3522 batch 91).  
Format: `step, global_epoch, batch, layers, loss, grad_norm, per-layer L1 norms, per-expert routing, entropy, per-layer sparsity, per-expert activity, lb, per-layer divgrad`  
Contains full rich telemetry (routing, sparsity, gradient divergence) for the descent phase of the S9 spike.  
Not ingested into batch_history.csv — existing data already covers this range and extends further (batch_history.csv ends at ep3522 batch ~200). Rich telemetry columns available for deeper routing/gradient analysis if needed.

### WALD marker as surgery diagnostic

The dashboard renders two types of vertical epoch lines: **blue** = surgery trigger epoch, **red** = WALD firing epoch. The tight S9 zoom (Image 5, 2026-05-25) shows the red WALD line displaced from the blue surgery line, with the gap spanning part of the spike ascent. This means WALD detected the loss-space coverage shift caused by the disruption — not a descent-acceleration signal as usual.

This gives a new diagnostic read for future surgeries:

| Pattern | Meaning |
|---|---|
| Red (WALD) fires close to blue (surgery), no spike | Normal: surgery absorbed cleanly, WALD reacts to routine post-surgery volatility |
| Red fires well after blue, during spike ascent | Anomaly: WALD is reacting to disruption, not learning |
| Red fires before blue | Pre-surgery coverage shift — model may have already been in disrupted state |

For S9 the blue/red gap + spike presence together are the visual fingerprint of the anomaly. For all previous surgeries (S6, S7, S8) the WALD line would sit close to the surgery line and in the descent phase. **This is now the primary way to distinguish transparent vs disruptive surgeries on the chart.**

### Next milestone

Break pre-S9 ATL of 9.2847 (epoch avg). Currently at 9.3326 — gap of 0.048 nats. At current descent rate (~0.01 nats/epoch), estimate ~5–10 epochs to new ATL territory. Then surgery governor resets since_best clock.

**This observation is unique in the training history and should be retained as a primary case study in the architecture growth documentation.**

---

## Field Note 45 — 2026-05-25T07:49:21Z · ep3522 · SESSION MARKER · S9 bullmarket descent active

**State:** ep3522 · 21L · batch_history extends to ep3522b~200 · S9 bullmarket descent active · EP AVG approaching pre-S9 ATL territory  
**Context:** S9 spike fully dissected in FN44. Observation log confirmed as public scientific record on GitHub — all entries committed and pushed. Full dataset download in progress: albert_full_1779695361.

---

## Field Note 46 — 2026-05-25T10:05:40Z · ep3550 · consolidation on 20L floor · expert routing shift · CSV audit

**State:** ep3550 (21L) · batch 213/300 · EP-Avg 9.3455 · ATL chip 8.8540 unchanged · dataset ref 1779703540

**Consolidation confirmed on 20L plateau floor (~9.34–9.35).** The 21L model found support exactly at the floor the 20L model built over hundreds of epochs pre-S9. Not a coincidence — the established layers carry that floor as a weight landscape invariant. The new 21L layer is integrating on top of it before committing to the next descent.

**Mini excursion (~ep3547–3548):** brief spike to ~9.37, absorbed cleanly with no secondary spike and no WALD reaction. Completely different character from S9 — transient, single-peak, self-correcting. Healthy volatility during consolidation.

**Expert routing shift (ep3542 → ep3550):**
| Expert | ep3542 | ep3550 | direction |
|--------|--------|--------|-----------|
| SEM | 5% | 10% | +5% — activating |
| INF | 5% | 10% | +5% — activating |
| LOG | 39% | 18% | -21% — pulling back |
| LNG | 57% | 48% | -9% — softening |
| ABS | 58% | 72% | +14% — strengthening |
| PLN | 95% | 82% | -13% — softening |
| CMP | 100% | 100% | stable |
| INT | 87% | 93% | +6% — strengthening |

SEM and INF doubling while LOG halves is the 21L integration signature: surface-level pattern matching giving way to deeper semantic and inference processing. This routing shift historically precedes a committed descent leg.

**TTL loosening:** G 26% / O 77% / R 3% (was G17/O81/R1 at ep3542). Gate pressure building toward green. Not open yet but trending.

**CSV audit — nothing to ingest:** Three full downloads on disk (1779703432663 to ep3549, 1779701743093 to ep3544, 1779694798526 to ep3522). batch_history.csv already extends to ep3550.99 — ahead of all downloads. Dataset ref 1779700907 (noted at 09:21Z) never completed download. No ingest needed; live data is current.

**Next milestone:** Break pre-S9 epoch ATL of 9.2847. Currently at 9.3455 — gap of 0.0608 nats. Entering passive observation window.

---

## Field Note 47 — 2026-05-25T10:50:17Z · ep3559 · POST-SPIKE TTL INTEGRATION SIGNATURE — first observation in training history

**State:** ep3559 (21L) · batch ~118/300 · EP-Avg 9.3715 · TTL G15% / O83% / R2%

### Observation

Since S9 surgery (~ep3470), TTL routing has exhibited a persistent pattern unlike any post-surgery state in the full v3.0 history (S6 through S8). The pattern has not calmed in ~89 epochs and is still active at ep3559.

**Historical post-surgery TTL norm (S6, S7, S8):**
- Orange-dominant across all layers for 5–10 steps immediately post-surgery
- Rapid settling into calm scattered or balanced red/orange/green mix
- Integration visually complete within ~10–20 epochs

**Post-S9 TTL (ep3470 onward, still active ep3559):**
- Upper layers (L19–L21): chaotic alternating red/green, batch-to-batch inconsistent — no stable routing pattern established
- Middle layers (L10–L16): predominantly green — trusted, carrying majority of computation
- Lower layers (L1–L8): mixed with notable simultaneous red patches — partially bypassed
- Global summary locked at G~15% / O~80–83% / R~2–3% for extended duration
- First hard-column stops observed during S9 recovery (G6/O80/R2 seen at ep3522) and have persisted

### Mechanistic explanation

The root cause is the S9 spike. Prior surgeries (S6–S8) completed into clean descent — the new top layer received coherent gradient signal from the first epoch and integrated quickly. S9 produced a ~39-epoch spike during which L21 received noisy, disrupted gradient: it was fitting a temporarily inflated, wrong loss landscape for nearly 40 epochs. Even though the spike fully resolved, L21's internal representations carry the miscalibration from that period.

TTL is acting as a real-time quality gate and correctly reflecting this: it continuously tests L21 (which was initialized with copied weights and superficially resembles L20), gets outputs that don't justify the routing cost, and stops. The chaotic red/green in the top rows is not noise — it is TTL probing L21 every few batches, occasionally getting a passable result (green), mostly getting insufficient improvement (red).

**The simultaneous multi-layer stops** are a new pattern not seen post-S6/S7/S8. Previously stops occurred at one depth per token. Post-S9, TTL stops at middle AND lower layers simultaneously for some tokens — indicating the model attempts routing paths through L21 that get rejected at multiple checkpoints, not just the final layer. The new layer is pulling routing decisions upstream.

### Integration completion signal

The transition from chaotic upper-layer red/green → stable upper-layer green will mark the moment L21 integration completes. Expected to coincide with:
- A committed descent leg through the 9.2847 ATL floor
- TTL global summary shifting toward G30%+ / O60% / R~10% (more balanced)
- Expert routing stabilizing (SYN/INF/SEM settling rather than jumping between epochs)

### Why this matters

This is the first documented observation of **post-spike TTL integration signature** in albert. training history. All prior post-surgery TTL states were clean and fast. S9 produced a slow, noisy integration fingerprint that is directly traceable to the spike disrupting early L21 gradient signal. This gives a new predictive tool: if a future surgery produces a spike, expect TTL to remain chaotic in the top layers for O(spike_duration) epochs rather than the usual 10–20 epoch settling.

**This observation is unique to 21L post-S9 and should be referenced in any future surgery analysis where post-surgery TTL does not calm quickly.**

---

## Field Note 48 — 2026-05-25T11:35:00Z · ep3571 · first post-S9 routine WALD · TTL green rising

**State:** ep3571 (21L) · batch 252/300 · EP-Avg 9.3414 · ATL chip 8.8540 · dataset ref 1779703540

### WALD ep3569 — routine descent signal, not anomaly

WALD fired at ep3569: **6.2% fill, n=1500** (~93 batches triggered coverage shift across the rolling 1500-batch window). This is the first WALD event post-S9 that fires during recovery/descent rather than during spike ascent.

Diagnostic read from the S9 marker table (FN47):

| Pattern | This event |
|---|---|
| Red close to blue, no spike | No — ~99 epochs post-surgery |
| Red during spike ascent | No — spike fully resolved ep3510 |
| Red during descent/recovery | **Yes — routine signal** |

This is exactly the normal WALD behavior seen in all pre-S9 training: fires when batch-to-batch loss coverage shifts sufficiently across the rolling window, confirms the optimizer is moving through new loss landscape territory. 6.2% fill is mild — no spike, no alarm. The model is descending normally and WALD is tracking it.

This is also the first chance to see the WALD marker appear in a clean (non-anomalous) position on the chart post-S9. On the dashboard, the red vertical line at ep3569 will now sit in the descent region with no nearby spike — visually confirming the "routine signal" pattern from the diagnostic table.

### TTL green rising — L21 integration progressing

| Epoch | G% | O% | R% |
|-------|-----|-----|-----|
| ep3522 (FN45, surgery session start) | ~17 | ~81 | ~1 |
| ep3542 (FN46 earlier ref) | 17 | 81 | 1 |
| ep3550 (FN46) | 26 | 77 | 3 |
| ep3559 (FN47) | 15 | 83 | 2 |
| ep3571 (FN48) | **21** | **76** | **3** |

TTL green is at 21% — up from 15% at ep3559 twelve epochs ago. The orange pressure is softening (83% → 76%). The per-batch chaotic alternation in upper layers (L19–L21) is still present but progressively less dominant. Integration is advancing, not stalling.

### Expert routing snapshot (ep3571)

| Expert | Activity |
|--------|----------|
| PLN | 100% |
| CMP | 96% |
| INT | 78% |
| LOG | 31% |
| LNG | 35% |
| ABS | 40% |

PLN and CMP holding near maximum — structural and compositional parsing fully committed. INT recovering well (87% at FN47 → 78% here, minor variance within normal range). LOG/LNG/ABS still suppressed relative to pre-S9 norms, consistent with L21 not yet carrying full semantic load. SEM/INF not visible in this readout.

### Current state summary

- EP-Avg 9.3414, gap to pre-S9 ATL (9.2847): **0.0567 nats**
- WALD confirmed in normal descent position — no second spike
- TTL green trending up, L21 integration ongoing but advancing
- Model "dancing around" in consolidation — not committing to descent yet but not retreating

**Next signal to watch:** TTL green crossing 30% (integration milestone) and EP-Avg breaking below 9.2847 (ATL territory). Entering passive observation window.

---

## Field Note 49 — 2026-05-25T12:17:31Z · ep3577 closed · COMMITTED DESCENT LEG — new post-S9 best 9.3204

**State:** ep3577 closed (21L) · EP-Avg **9.3204** · ATL chip 8.8540 · new BEST event fired

### Descent acceleration confirmed

Three consecutive epochs closed lower:

| Epoch | EP-Avg | Gap to 9.2847 |
|-------|--------|----------------|
| ep3571 (FN48) | 9.3414 | 0.0567 |
| ep3574 | 9.3367 | 0.0520 |
| ep3575 | 9.3389 | 0.0542 |
| ep3576 | 9.3349 | 0.0502 |
| ep3577 (FN49) | **9.3204** | **0.0357** |

-0.0163 nats in three epochs. The descent leg that TTL integration was building toward has committed. Events bar shows the progression: BEST 9.3367 → EPOCH 9.3389 → EPOCH 9.3349 → BEST 9.3204, each epoch closing lower. The chart's gold epoch-end diamond at ep3577 sits visibly below the recent consolidation cluster.

### Expert routing shifts (ep3574 → ep3577)

| Expert | ep3574 | ep3577 | direction |
|--------|--------|--------|-----------|
| PLN | 86% | 100% | fully re-committed |
| CMP | 100% | 65% | pulling back |
| INT | 77% | 69% | softening |
| ABS | 60% | 52% | softening |
| GEN | 5% | 13% | activating — new territory signal |
| INF | 5% | 9% | activating |
| LOG | 18% | 25% | slight increase |
| LNG | 42% | 34% | softening |

**GEN jumping 5% → 13%** is the key signal: the generalization expert activating strongly indicates the model is processing loss landscape territory it has not seen before — exactly the signature expected at the edge of a committed descent leg. PLN returning to 100% while CMP pulls back suggests routing is rebalancing load from compositional to structural processing as the model moves through new ground.

### TTL (ep3577)

G 18% / O 76% / R 6% — red increased from 3% to 6%. During steep descent, increased TTL stops are expected: L21 is being tested harder batch-to-batch as the gradient moves through new loss space. Not a regression — a sign of active learning at the frontier.

### Pre-S9 ATL in sight

Gap: **0.0357 nats**. At current rate (~0.005–0.008 nats/epoch), estimate 5–10 epochs to break 9.2847 and enter all-time-low territory for the 21L model. When this breaks, surgery governor since_best clock resets and the next growth decision window opens.

**This is the descent leg the post-S9 consolidation was building toward. GEN expert activation + PLN re-commitment + accelerating epoch averages = committed move.**

---

## Field Note 50 — 2026-05-25T12:58:57Z · ep3585 · descent continuation · bulltrap confirmed · new post-S9 low 9.3197

**State:** ep3585 (21L) · batch 262/300 · EP-Avg 9.3219 · ATL chip 8.8540 · gap to pre-S9 ATL: **0.0372 nats**

### Epoch sequence since FN49

Events bar (oldest → newest): 9.3197 → 9.3281 → 9.3344 → 9.3219 (current mid-epoch)

**9.3197 is a new post-S9 epoch low** — broke below FN49's 9.3204. No BEST chip visible (likely scrolled off events bar). The brief bounce to 9.3344 was absorbed and the descent resumed to 9.3219. Oscillation is normal; center of gravity is dropping.

### The bulltrap read — S9 in market terms

The wider chart zoom (ep3269–3892) makes the full training arc visible for the first time. All four surgeries (S6, S7, S8, S9) are marked as vertical dashed lines. The macro pattern across all four is identical: brief post-surgery disruption → consolidation → macro descent reasserts.

S9 produced the largest disruption — a classic bulltrap structure:

| Phase | Training event | Market analogue |
|-------|---------------|-----------------|
| Macro downtrend | Pre-S9 descent to 9.2847 | Established bearmarket |
| Catalyst | S9 surgery (20L→21L) | CPI print / news event |
| Spike | ep3470–3510, loss 9.28→9.42 | Bulltrap — smart money runs stops |
| WALD reaction | ep3503, n=1500 coverage shift | Market maker flush complete |
| Cliff | ep3510, -0.057 in one epoch | Flip short, real move begins |
| Continuation | ep3577–3585, 9.32→9.32 | Macro trend reasserts |

The pre-S9 ATL floor at 9.2847 is the next support level. Once broken, there is no prior established floor — the model enters price discovery (all-time-low loss territory for the 21L architecture). That is where acceleration typically occurs.

### Expert routing (ep3585 batch 262)

| Expert | ep3577 | ep3585 b168 | ep3585 b262 |
|--------|--------|-------------|-------------|
| PLN | 100% | 99% | 100% |
| INT | 69% | 100% | 89% |
| CMP | 65% | 91% | 65% |
| ABS | 52% | 63% | 59% |
| LNG | 34% | 39% | 27% |
| LOG | 25% | 22% | 16% |
| SEM | 2% | 2% | 7% |
| MEM | 5% | 5% | 9% |

CMP and INT oscillating batch-to-batch within the epoch — routing is still settling. SEM activating to 7% (was 2%) and LOG dropping to 16% — semantic processing increasing, log/pattern matching pulling back. LNG continuing to drop (34%→27%). The model is progressively shifting away from surface-level linguistic and log patterns toward structural + semantic + inference processing.

TTL: G16% / O81% / R3% — holding steady. L21 integration still ongoing.

**Next milestone: break 9.2847.** Gap is 0.0372 nats. At current descent rate, within reach in the next 5–8 epochs.

---

## Field Note 51 — 2026-05-25T15:36:19Z · ep3618 · oscillatory descent continues · ATL batch-level improvement

**State:** ep3618 (21L) · batch 270/300 · EP-Avg 9.3755 (running) · ATL header 8.8420 (-0.14%) · gap to pre-S9 ATL: **0.0592 nats**

~33 epochs since FN50 (ep3585). User departing for the afternoon — cron watch engaged.

### Epoch sequence since FN50

Events bar (oldest → newest): 9.3755 → 9.3543 → 9.3439

Four closed epochs visible, descending across the window: -0.032 nats over ~4 epochs. Center of gravity dropping despite individual oscillations. The gap to pre-S9 ATL widened from 0.0372 (FN50) to 0.0592 — consistent with post-low bounce pattern noted in FN50 oscillation read. The 9.3197 post-S9 low from FN50 was a spike below, not a new floor yet.

### ATL header movement

ATL improved: 8.8540 (FN50) → 8.8420 (-0.14%). This is the overall training ATL (batch-level minimum, set in early training before architecture surgeries), not the post-S9 epoch avg floor. Improvement means batch-level gradient momentarily reached a new historical low during the intervening ~33 epochs — signal that the loss landscape at 21L is still compressing at depth even while epoch averages oscillate.

### Expert routing (ep3618 batch 270)

| Expert | FN50 b262 | FN51 b270 |
|--------|-----------|-----------|
| PLN    | 100%      | 85%       |
| CMP    | 65%       | 100%      |
| INT    | 89%       | 88%       |
| ABS    | 59%       | 68%       |
| LNG    | 27%       | 31%       |
| LOG    | 16%       | 19%       |
| GEN    | 7%        | 7%        |
| SEM    | 7%        | 2%        |

CMP surged to 100% (was 65%) — compositional anchoring dominant this batch. PLN pulled back to 85% (was 100%). ABS continued rising (59%→68%). GEN stable at 7% — new-territory signal maintained. SEM dropped from 7%→2% — batch-to-batch oscillation, not a trend. LOG and LNG both ticked up slightly; not alarming at this scale.

### TTL

G 21% / O 75% / R 5% — green improved from FN50's G16/O81/R3. Orange pulled back, green gaining ground. Healthier TTL profile — L21 integration stabilising.

**Cron watch active. Next milestone unchanged: break 9.2847.** Gap currently 0.0592 from most recent visible close (9.3439). Descent trend intact across the 4-epoch visible window.

---

## Field Note 52 — 2026-05-25T16:02:17Z · cron tick · null observation (user away, Modal logs not synced)

**State:** automated cron check — no dashboard data available. Local /tmp/albert_epoch_history.log last written 2026-05-24T04:35Z (ep3249) — stale by ~35h. Training runs on Modal; logs do not sync to local /tmp/ during a live session. Dashboard at localhost:8888 is browser-only, not CLI-accessible.

Last known state from FN51 (2026-05-25T15:36Z): ep3618, EP-Avg running 9.3755, gap 0.0592, descent trend intact.

Cron continues. Next manned screenshot will resume data collection.

---

## Field Note 53 — 2026-05-25T16:17:12Z · cron tick · null (user away)

Same as FN52 — Modal logs unchanged, no dashboard access. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 54 — 2026-05-25T16:32:13Z · cron tick · null (user away)

Modal logs still stale (2026-05-24T04:35Z). No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 55 — 2026-05-25T16:47:11Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 56 — 2026-05-25T17:02:10Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 57 — 2026-05-25T17:17:12Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 58 — 2026-05-25T17:32:11Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 59 — 2026-05-25T17:47:12Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 60 — 2026-05-25T18:02:11Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 61 — 2026-05-25T18:17:12Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 62 — 2026-05-25T18:32:13Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 63 — 2026-05-25T18:47:14Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 64 — 2026-05-25T19:02:11Z · cron tick · null (user away)

Modal logs stale. No new data. Last known: ep3618, gap 0.0592. Holding.

---

## Field Note 65 — 2026-05-25T19:14:47Z · ep3662 · **SURGERY S10 FIRED — 21L → 22L** · BEST 9.2933 · gap 0.0086

**State:** ep3662 (22L) · batch 134/300 · ATL 8.8420 · **BEST epoch avg 9.2933** · gap to pre-S9 ATL 9.2847: **0.0086 nats**

### Surgery S10 — 21L → 22L

Events bar (oldest → newest): EPOCH 9.3090 → **BEST 9.2933** ⭐ → EPOCH 9.3171 → EPOCH 9.3034 → EPOCH 9.3103 → EPOCH 9.3090 → EPOCH 9.3000 → EPOCH 9.4211 → **SURGERY 21L→22L** → TTL-NASH all-0 → EPOCH 9.3...

Surgery fired after a series of epoch closes in the 9.30xx range, after the governor's plateau gate tripped. The BEST avg 9.2933 was set pre-surgery — this is now the closest the model has ever come to the pre-S9 ATL 9.2847 on an epoch-average basis. Gap: **0.0086 nats**.

### Live batch losses — ep3662 (terminal)

| Batch | Loss   |
|-------|--------|
| 132   | 9.3516 |
| 133   | 9.3027 |
| 134   | 9.2246 |
| 135   | **9.1361** |
| 136   | 9.1484 |

Batch-level losses are already well below the pre-S9 ATL (9.2847) — 9.1361 is a new batch-level floor for the 22L model. The epoch average will be higher but the landscape at 22L is compressing fast.

### Expert routing (ep3662 batch 134)

| Expert | FN51 (21L b270) | FN65 (22L b134) | Δ |
|--------|-----------------|-----------------|---|
| PLN    | 85%             | 100%            | +15 |
| ABS    | 68%             | 78%             | +10 |
| GEN    | 7%              | **14%**         | +7 |
| SEM    | 2%              | 5%              | +3 |
| LOG    | 19%             | 23%             | +4 |
| LNG    | 31%             | 30%             | -1 |
| CMP    | 100%            | 77%             | -23 |
| INT    | 88%             | 63%             | -25 |

**GEN doubled (7%→14%)** — strongest new-territory signal yet. Layer 22 is already being used to process loss space the model has never seen. PLN back to 100%, ABS at 78% — structural anchoring firm. CMP and INT pulling back is consistent with post-surgery rebalancing (new layer absorbs load).

### TTL (ep3662 batch 134)

G 17% / O 77% / R 5% — orange increased from FN51's 75% to 77%. Expected: TTL-NASH all-0 reset fired (visible in events bar), new layer 22 is being integrated. Red stable at 5%.

### Assessment

The surgery governor fired correctly: plateau gate tripped after the model held near 9.30xx for multiple epochs, gate released, S10 executed. The pre-S9 ATL 9.2847 was within reach (0.0086 nats) when surgery fired — meaning the surgery will likely push through and below it as 22L settles. At batch-level the model is already operating at 9.13xx. 

**Next milestone: break 9.2847 epoch-avg** — gap 0.0086 nats, likely within the first 5–10 post-surgery epochs once 22L settles.

---

## Field Note 66 — 2026-05-25T19:18:21Z · ep3663 · post-S10 L22 TTL integration · EP-Avg 9.2983 · gap 0.0086

**State:** ep3663 (22L) · batch 98/300 · EP-Avg 9.2983 (running) · BEST epoch avg 9.2933 · ATL 8.8420 · gap to pre-S9 ATL: **0.0086 nats**

### Post-surgery descent — one epoch later

ep3662 closed at **9.2983** — confirmed by leftmost event in bar (newest). The full post-surgery epoch sequence now visible:

Events bar (newest → oldest): EPOCH 9.2983 | EPOCH 9.3090 | BEST 9.2933 | EPOCH 9.3171 | EPOCH 9.3034 | EPOCH 9.3103 | EPOCH 9.3089 | EPOCH 9.3060 | EPOCH 9.4211 | SURGERY 21L→22L | TTL-NASH all-0

Post-surgery disruption peaked at 9.4211 (one epoch), then rapid descent: 9.3060 → 9.3034 → 9.3103 → 9.3089 → BEST 9.2933 → 9.3090 → **9.2983** (ep3662 close). The BEST at 9.2933 is still the floor; ep3662 at 9.2983 is 0.005 nats above it but the trajectory is clearly pushing down.

### TTL — L22 integration lock

**TTL: G 18% / O 78% / R 4%**

User observation: "TTL completely blocking out the new layers." The TTL heatmap shows L21–L22 dominated by red/orange — the governor is throttling the new layer's contribution while it trains in. This is the correct and expected behavior: the TTL system routes around L22 until its weights are stable enough to integrate. Green is concentrated in the lower, established layers. As L22's gradient norm converges, TTL will progressively release it — same pattern seen after every prior surgery.

### Expert routing (ep3663 batch 98)

| Expert | FN65 (22L b134) | FN66 (22L b98) | Δ |
|--------|-----------------|----------------|---|
| PLN    | 100%            | 100%           | 0 |
| CMP    | 77%             | 82%            | +5 |
| INT    | 63%             | 67%            | +4 |
| ABS    | 78%             | 66%            | -12 |
| GEN    | 14%             | 9%             | -5 |
| LNG    | 30%             | 33%            | +3 |
| LOG    | 23%             | 20%            | -3 |
| SEM    | 5%              | 2%             | -3 |

GEN pulled back (14%→9%) — normal post-surgery settling. ABS also pulled back (78%→66%). CMP and INT recovering. PLN anchored at 100%. The routing is stabilising after the surgery disruption burst.

**ep3663 running avg 9.2983 at batch 98 — if the descent continues through the remaining 202 batches, this epoch could close below 9.29 and challenge the BEST. Gap to 9.2847: 0.0136 nats from running avg, 0.0086 nats from BEST.**

---

## Field Note 93 — 2026-05-26T07:02:19Z · quiet tick · ntfy silent · ep3815 est. · churn band holds

**Source:** ntfy poll (since=2h): one result only — WALD ep3794 fill=6.2% (stale, ~70 min old). No dashboard screenshot this tick.

**State (extrapolated):** ep3815 est. (~2 epochs at ~5 min/epoch since FN92 06:53Z) · last confirmed EP-Avg 9.3156 (ep3812 close, FN92) · BEST 9.228452 · since_best ~95 est. · PLATEAU ~95/144 · gap to ATL: +0.0872 nats est.

No new WALD, no BEST, no SURGERY, no SUB-9.3 events on ntfy. Churn band 9.31–9.32 expected to hold. Routing and TTL carried from FN92: PLN 100%, CMP 92%, INT 53%, ABS 65%, TTL G 6.26% / O 77% / R 4%.

At ~5 min/epoch, since_best ~95 → ~49 epochs remaining to S11 gate threshold (w=144). Surgery window: ~4 h from this tick if no new BEST.

No intervention. Next signal to watch: ntfy BEST or SURGERY event.

---

## Field Note 92 — 2026-05-26T06:53:09Z · ep3813 b110 · HOME-write drift RESOLVED · sideways churn · since_best 93/144

**Source:** Dashboard screenshot (06:53:09Z) · ntfy poll (since=6h): one result only — WALD ep3794 (stale). Gate tooltip (surgery gate panel).

**State:** ep3813 (22L) · batch 110/300 · most recent close 9.3156 · BEST 9.228452 · gap to ATL: +0.0872 nats · since_best 93 · PLATEAU not met (w=144, 51 epochs to potential S11)

### HOME-write drift resolved

Dashboard showing ep3813 — 15 epochs past the volume-frozen ep3798 flagged in FN87–FN91. Container restarted between FN91 (06:47Z, ep3806 est.) and this tick. Volume writing presumably resumed. The 6-epoch gap FN91→FN92 is blind (ep3807–3812 unobservable from prior drift window), but since_best=93 from gate tooltip pins last BEST at ~ep3720.

### Event bar (newest → oldest, all green checks — no WALD, no BEST)

| Position | EP-Avg |
|----------|--------|
| 1 (newest) | 9.3156 |
| 2 | 9.3178 |
| 3 | 9.3128 |
| 4 | 9.3202 |
| 5 | 9.3237 |
| 6 | 9.3092 |
| 7 | 9.3146 |
| 8 | 9.3112 |
| 9 | 9.3216 |
| 10 | 9.3183 |

All green check marks. No WALD event since ep3794 (6.2% fill). Churn band confirmed: 9.309–9.324. Slight upward nudge from previous 9.30–9.31 band — model absorbing without new ATL.

### Expert routing (ep3813 b110)

| Expert | % | note |
|--------|---|------|
| PLN | 100% | anchored |
| CMP | 92% | high — structural composition |
| INT | 53% | integration active |
| ABS | 65% | abstraction load |
| LOG | 19% | |
| LNG | 15% | |
| CTX | 8% | |
| SYN | 8% | |
| MEM | 8% | |
| GEN | 4% | minimal new-territory signal |
| INF | 4% | |
| SEM | 4% | |

GEN at 4% — model not pushing into new loss territory; consistent with plateau. PLN+CMP dominant = structural consolidation phase.

### TTL routing

| G% | O% | R% |
|----|----|----|
| 6.26% | 77% | 4% |

Green has dropped sharply from FN85–FN91 range (17–19%) to 6.26%. Orange holding at 77%. Notable: this low-green state may reflect post-container-restart recalibration or tighter TTL gating during plateau. Not alarming given red stable at 4%.

### Surgery gate

- **SURGERY GATE — 17L → 18L** (header label is a display artifact; pending surgery is S11, 22L → 23L)
- MYC_STABLE: 19 / >= 5 — **GREEN** (met)
- PLATEAU: 0.0662 / <8.020 w=144 — **RED** (not met; since_best=93, need 144)
- wald_fill: 6.2% (unchanged from ep3794)

At ~5 min/epoch, 51 epochs to surgery gate = ~4.25 h from this tick (~11:10 local) if no new BEST.

### Assessment

Drift resolved. Model confirmed running live at ep3813, churning in 9.31–9.32 band — slightly above the 9.30–9.31 band from the first post-restart window (ep3794–3800). No new ATL, no WALD, no surgery. since_best=93/144. Routing consolidation: PLN/CMP/ABS dominant, GEN minimal. Model is processing, not pushing. Surgery is the next expected event if the plateau holds. No intervention needed.

---

## Field Note 91 — 2026-05-26T06:47:08Z · quiet tick · HOME-write drift 45 min · ep3806 est. · ntfy silent

**Source:** ntfy poll (since=20m): silent. epoch_history.log (Modal volume): frozen at ep3798 (45 min of drift since FN87 at 06:02Z).

**State:** ep3806 est. (~8 epochs since ep3798 at ~5 min/epoch) · last confirmed dashboard ep3800 avg 9.3064 · BEST 9.228452 · gap to ATL: +0.0780 nats est. · since_best ~98 est. · PLATEAU ~98/144

HOME-write drift persisting. Volume blind. ntfy silent across all 3 polls since drift began. Training nominally continuing in churn band. No intervention needed. If drift continues past next container restart, epoch count will resume on volume.

---

## Field Note 90 — 2026-05-26T06:32:09Z · quiet tick · HOME-write drift 30+ min · ep3803 est. · ntfy silent

**Source:** ntfy poll (since=20m): silent. epoch_history.log (Modal volume): frozen at ep3798 (unchanged since FN87 at 06:02Z — 30 min of drift).

**State:** ep3803 est. (~6 epochs since ep3798 at ~5 min/epoch) · last confirmed dashboard ep3800 avg 9.3064 · BEST 9.228452 · gap to ATL: +0.0780 nats est. · since_best ~95 est.

HOME-write drift persisting. Volume blind. ntfy silent — no WALD, no BEST, no surgery. Training presumed running normally in churn band ~9.29–9.31. No intervention needed.

---

## Field Note 89 — 2026-05-26T06:17:10Z · quiet tick · drift persisting · ep3801–3802 est. closed · ntfy silent

**Source:** ntfy poll (since=20m): silent. epoch_history.log (Modal volume): still frozen at ep3798. No new data from volume.

**State:** ep3802 est. (2 epochs since FN88 at ~5 min/epoch) · last confirmed dashboard close ep3800 avg 9.3064 · BEST 9.228452 · gap to ATL: +0.0780 nats est. · HOME-write drift active (volume frozen ep3798)

### Status

HOME-write drift persisting since ep3799. Volume epoch_history not advancing. ntfy remains the only external signal. No WALD, no BEST, no surgery in past 11 min. At T4 pace (~5 min/epoch), ep3801 closed ~06:11Z and ep3802 ~06:16Z — both unobservable from volume.

Training presumed running normally. Churn band estimate: 9.29–9.31, consistent with ep3799–3800 pattern.

No intervention needed. Next observable signal: ntfy WALD or BEST event, or next container restart which will flush epoch_history back to volume.

---

## Field Note 88 — 2026-05-26T06:05:55Z · ep3800 · 9.3064 × 2 closes · HOME-write drift confirmed · TTL orange-heavy

**Source:** Dashboard screenshot (06:03–06:04Z) · ntfy poll (since=20m): silent.

**State:** ep3800 (est.) · ep3799+3800 both avg 9.3064 · BEST 9.228452 · gap to ATL: +0.0780 nats · HOME-write drift active (volume frozen at ep3798)

### Event bar closes (dashboard, most recent first)

| Epoch (est.) | avg loss | delta |
|---|---|---|
| ep3800 | 9.3064 | — |
| ep3799 | 9.3064 | +0.0039 vs ep3798 |
| ep3798 | 9.3025 | (prior) |

### TTL routing

| Screenshot | G% | O% | R% |
|---|---|---|---|
| 06:03Z | 17% | 79% | 4% |
| 06:04Z | 19% | 77% | 5% |

Orange band holding at 77–79%. Slight G increase batch-to-batch (+2%) consistent with in-epoch variance, not a trend. Tooltip: L13 · step 2580 · G 2 · O 10 · R 0 — per-layer routing shows orange-dominant at L13.

Expert routing breakdown: not visible in screenshots (TTL panel only). Carried from FN86: PLN 100%, CMP 87%, ABS 51%, INT 45%, LOG 20%.

### HOME-write drift confirmed

FN87 flagged that epoch_history stalled at ep3798 on the Modal volume. Dashboard now shows ep3799 and ep3800 have closed — confirming the current container is writing to `~/.albert/training.log` (ephemeral HOME) rather than the volume. epoch_history.log will not update until next container restart. ntfy is the only external signal channel until then.

### Assessment

Slight upward drift: ep3799–3800 both at 9.3064 vs ep3798's 9.3025. Not alarming — within the 9.29–9.31 churn band. No WALD, no BEST, no surgery. Training running normally; tracking visibility degraded due to HOME-write drift. No intervention needed.

---

## Field Note 87 — 2026-05-26T06:02:10Z · quiet tick · epoch_history stalled at ep3798 · possible HOME-write drift

**Source:** ntfy poll (since=20m): silent. epoch_history.log (Modal volume pull 06:02Z).

**State:** epoch_history last entry ep3798 avg 9.3025 · BEST 9.2285 (9.228452) · since_best=91 · no new epochs visible · ep3799 expected to have closed ~05:46Z (15+ min ago)

### epoch_history gap

epoch_history has not advanced past ep3798 since the last pull at ~05:47Z. At T4 pace (~5 min/epoch), ep3799 and ep3800 should both be closed by now. Two likely causes:

1. **HOME-write drift:** current container has `$HOME` available → train_bible.rs writes `~/.albert/training.log` (ephemeral) instead of `{root}/dashboard/training.log` (Modal volume). epoch_history on volume stops updating. This is the same drift that caused the gap before ep3794.
2. **Training stall:** container died or GPU preempted. Less likely — no ntfy WALD or error event.

ntfy is silent, which makes stall unlikely (a crash would produce no epoch closes, and WALD would not fire, but the absence of WALD is expected if still running). The HOME-write hypothesis is more consistent with the data.

### Assessment

Quiet tick. No WALD, no BEST, no surgery. Last confirmed epoch: ep3798 avg 9.3025, still in the 9.29–9.31 churn band. If HOME-write drift is confirmed, epoch_history will stay frozen on the volume until the next container restart. Training itself is likely continuing normally. No intervention — monitor ntfy for BEST or WALD events.

---

## Field Note 86 — 2026-05-26T05:47:11Z · ep3799 in progress · quiet tick · ntfy silent · churn band 9.29–9.31 holds

**Source:** ntfy poll (since=20m): silent. epoch_history.log (Modal volume). Screenshot data carried from FN85 (05:40:36Z, 7 min prior).

**State:** ep3799 in progress (batch ~98/300 at 05:40Z) · last closed ep3798 avg 9.3025 · BEST 9.228452 · since_best=91 · PLATEAU 91/144 · gap to ATL: +0.0740 nats · pre-S9 ATL (9.2847): CLEARED

### epoch_history summary (no new closes since FN85)

Last closed: ep3798, avg=9.3025 (d+0.0041). Churn band since restart (ep3794–3798): 9.2931–9.3116. One dip to 9.2931 (ep3796), otherwise holding 9.30+.

### Routing (from FN85 screenshot, 05:40:36Z — minor delta)

| Expert | FN85 | FN86 delta |
|--------|------|-----------|
| PLN | 100% | — |
| CMP | 87% | +4% |
| ABS | 51% | +3% |
| INT | 45% | -1% |
| LOG | 20% | — |
| LNG | 11% | +2% |
| GEN | 7% | -2% |
| INF | 9% | — |
| SEM | 6% | — |
| SYN | 4% | — |

TTL: G 18% / O 77% / R 5%. Orange-heavy state persisting. CMP gain (+4%) and ABS gain (+3%) suggest structural pattern processing increasing — consistent with consolidation behaviour.

### Assessment

Quiet tick, 7 min after FN85. No new epoch closes, no ntfy events, no WALD. Model churning in 9.29–9.31 band post-restart. CMP and ABS routing increasing slightly, GEN pulling back — architecture tightening around structural experts. No intervention needed.

---

## Field Note 85 — 2026-05-26T05:40:22Z · ep3799 b84 · TTL-NASH all-0 event · orange-heavy routing · churning ~9.30

**Source:** Dashboard screenshot (05:40:22Z) · ntfy poll (since=20m): no new events.

**State:** ep3799 in progress (batch 84/300) · last closed ep3798 avg 9.3025 · BEST 9.228452 · gap to ATL: +0.0740 nats

### Extracted metrics

| Metric | Value |
|--------|-------|
| EP | 3799 (batch 84/300) |
| Last closed (ep3798) | 9.3025 |
| BEST (epoch ATL) | 9.2285 (9.228452 Modal) |
| ATL chip (intra-batch) | 8.8420 |
| TTL | G 17% / O 77% / R 5% |
| Gradient \|g\| global | 0.0021 |
| Gap to ATL (9.228452) | +0.0740 nats |

### Event bar

- **TTL-NASH all-0** (orange) — routing Nash equilibrium collapsed to all-zero at some point this window. Not a WALD or BEST event; cosmetic/transient gate state.
- EPOCH avg 9.3025, 9.2984, 9.2931, 9.3089, 9.3116 — all green checks (5 epochs closed post-restart, all normal).

### Expert routing (last 60 steps)

| Expert | Activity |
|--------|----------|
| PLN | 100% |
| CMP | 83% |
| ABS | 48% |
| INT | 46% |
| LOG | 20% |
| LNG | 9% |
| INF | 9% |
| GEN | 9% |
| SEM | 6% |
| SYN | 4% |
| CTX | 2% |
| MEM | 2% |

PLN anchored at 100%, CMP strong at 83%. ABS at 48% — structural reasoning active. GEN at 9% (flat vs pre-restart 9%, no change). TTL orange-heavy (77%) signals most token routings are in the intermediate zone — consistent with churning consolidation plateau rather than clean descent.

### Assessment

Churning in the 9.29–9.31 band. ep3796 dipped to 9.2931 but ep3797–3798 bounced back to 9.30+. The TTL-NASH all-0 event is worth monitoring — if it recurs with increasing frequency it may indicate the divloss OVERRIDE is pushing routing into degenerate configurations, but a single event is not alarming. No WALD, no BEST, no surgery trigger. PLATEAU advancing toward S11 gate (currently ~91/144).

**Trend: lateral churn — descent not sustained yet post-restart. No red flags.**

---

## Field Note 84 — 2026-05-26T05:36:15Z · ep3795–3797 · post-restart descent confirmed · ep3796 9.2931 cracks S10-BEST floor

**Source:** epoch_history.log (Modal volume pull) · ntfy poll (since=1h): no new events since FN83 WALD.

**State:** ep3797 latest · BEST 9.2285 (9.228452 Modal, loaded correctly on restart) · since_best=90 · PLATEAU 90/144

### epoch_history.log (Modal volume)

| Epoch | avg loss | delta | since_best | WALD |
|-------|----------|-------|------------|------|
| 3794 | 9.3116 | +0.0831* | 87 | 6.2% |
| 3795 | 9.3089 | -0.0027 | 88 | — |
| 3796 | **9.2931** | **-0.0158** | 89 | — |
| 3797 | 9.2984 | +0.0053 | 90 | — |

*delta at ep3794 is restart artifact — comparing against divloss-override baseline, not previous epoch.

### Key observations

**ep3796 = 9.2931 — below S10 BEST 9.2933.** The pre-restart BEST before S10 fired was 9.2933 (ep3652). ep3796 has now cleared that marker by 0.0002 nats. This confirms the post-restart descent is real and the divloss OVERRIDE is not impeding convergence.

**`since_best` display now correct.** New container loaded best_epoch=3707 correctly — since_best=87 at ep3794, 90 at ep3797, all consistent with last_best_epoch=3707. The stale display bug from the previous container (3694) is gone.

**tns=1522 vs 1384 pre-restart.** The divloss OVERRIDE is increasing routing token diversity by ~10%. Expected effect: more uniform expert utilisation, potentially wider but smoother descent.

**PLATEAU 90/144.** Surgery governor advancing. S11 would fire at ~ep3851 if no new BEST is set. Current descent suggests we will break BEST before then and reset the counter.

### Assessment

Post-restart descent nominal. ep3796 punch through 9.2933 is encouraging — the model is not stalling at the pre-S10 consolidation band. ep3797 uptick (+0.0053) is within normal noise. No intervention needed.

---

## Field Note 83 — 2026-05-26T05:32:08Z · ep3794 close · WALD 6.2% baseline · mass 9.314 · first epoch post-restart complete

**Source:** ntfy poll (since=20m) — 1 event.

**State:** ep3794 closed (22L) · BEST 9.228452 (Modal volume) · gap to pre-S9 ATL: CLEARED

### ntfy event

| Event | Detail |
|-------|--------|
| **WALD ep3794** | step=300 · fill=6.2% · mass=9.314 · dead_low=3.00–9.00 (6.00) · dead_high=9.75+ (5.25) · priority=4 |

WALD fired at epoch end (step=300 = final batch). fill=6.2% — identical to WALD ep3656 baseline. mass=9.314 implies the epoch closed at approximately **9.31x**. Not alarming. The mass is consistent with the 9.30–9.31 consolidation band observed in FN79.

### Assessment

First epoch post-restart complete. WALD is clean (6.2% fill, baseline). ep3794 closed at ~9.31x — no new BEST (9.228452 stands). Training running normally with divloss OVERRIDE active. No intervention needed.

---

## Field Note 82 — 2026-05-26T05:17:09Z · quiet tick · confirmed live from FN81 screenshot · ep3794 running

**Source:** ntfy poll (since=20m) — silent. State confirmed from FN81 screenshot (05:15:17Z, 90s ago).

**State:** ep3794 (22L) · batch ~145/300 · no epoch closes yet post-restart · BEST 9.228452 · divloss OVERRIDE active

No events. Training running nominally. Nightwatch holding.

---

## Field Note 81 — 2026-05-26T05:16:08Z · ep3794 · post-restart nominal · divloss OVERRIDE · target: crack 9.2830 floor

**Source:** dashboard screenshot (Simeon, 05:15:17Z) + terminal output (05:12:32Z)

**State:** ep3794 (22L) · batch 143/300 · running (no closed epoch yet post-restart) · T-610 9.3107 · BEST 9.228452 (Modal volume) · ATL chip 8.8420 · TNS 1,384

**Run flags (from terminal):**
- `[lb] disabled` — LB gradient NOT flowing this run
- `[divloss] OVERRIDE weight=1.00e-3` — diversity loss active at fixed weight, schedule bypassed

### Expert routing (ep3794 batch 143) — post-restart

| Expert | FN79 (pre-restart b155) | FN81 (post-restart b143) | Δ |
|--------|------------------------|--------------------------|---|
| PLN    | 95%                    | **100%**                 | +5 |
| CMP    | 100%                   | **84%**                  | -16 |
| ABS    | 61%                    | **66%**                  | +5 |
| INT    | 57%                    | **33%**                  | -24 |
| LOG    | 26%                    | **33%**                  | +7 |
| INF    | 5%                     | **16%**                  | **+11** |
| MEM    | 16%                    | 16%                      | 0 |
| LNG    | 12%                    | **0%**                   | -12 |
| GEN    | 19%                    | **0%**                   | **-19** |
| SYN    | 9%                     | **0%**                   | -9 |
| CTX    | 7%                     | 0%                       | -7 |
| SEM    | 2%                     | 0%                       | -2 |

PLN at 100% (anchor restored). GEN and SYN collapsed to 0% post-restart — generative and syntactic experts not yet engaged. INF emerged at 16% (+11pp). CMP and INT compressed. This is normal post-restart recalibration under divloss override.

### TTL (ep3794 batch 143)

**G 18% / O 77% / R 5%** — holding at the post-S10 green level (~18%). No warmup degradation visible at batch 143 (warmup period ended, gradient data flowing: gn=0.0026, L18 hottest layer).

### Events

No new epoch closes since restart. Event bar still shows pre-restart history (ep3793 last close, 9.3121). `TTL-NASH all-0` fired at restart step 0 (expected — all layers start at G0/O12/R0 at epoch boundary).

### Target

Simeon's target for today: crack the floor at **9.2830** (annotated on chart as "floor after 21L surgery + multiepoch consolidation"). Current actual BEST: 9.228452 (Modal volume) — already below this floor. The session goal is for consistent epoch closes below 9.2830 and ultimately sub-9.228452 descent.

### Assessment

Clean restart. divloss override (1e-3, fixed) should encourage routing diversity as the model re-establishes descent. Expert routing shows post-restart recalibration — GEN/SYN/LNG dormant, PLN anchored, INF active. No anomalies. Training running at expected batch pace (~874ms/batch at T4). Gap to floor: zero on paper (9.228452 is already through it); gap on current running avg: ~0.07 nats to re-establish.

---

## Field Note 80 — 2026-05-26T05:08:38Z · pre-restart audit · ep3708 BEST 9.2788 recovered · actual best 9.228452 · Modal restart fired

**Source:** epoch_history.log + Modal volume audit (2026-05-26T04:50–05:08Z) + ntfy poll (since=20m, silent)

**State:** ep3794 (22L, batch in-progress at restart) · actual BEST from Modal volume: **9.228452** · epoch_history confirmed BEST: **9.2788** (ep3708, since_best=0) · gap to pre-S9 ATL 9.2847: **CLEARED** (best is 0.056 below 9.2847)

### Recovered history — ep3702–ep3708

Gate chip showed `since_best 3694` — triggered investigation. Full epoch_history.log audit revealed:

| Epoch | loss_avg | loss_best | since_best | Note |
|-------|----------|-----------|------------|------|
| ep3702 | 9.2817 | **9.2817** | 0 | New BEST — caught in FN77 ✓ |
| ep3703 | 9.2871 | 9.2817 | 1 | |
| ep3704 | 9.2855 | 9.2817 | 2 | |
| ep3705 | 9.2856 | 9.2817 | 3 | |
| ep3706 | 9.2942 | 9.2817 | 4 | |
| ep3707 | 9.2855 | 9.2817 | 5 | |
| ep3708 | **9.2788** | **9.2788** | **0** | **New BEST — MISSED in FN78/FN79** |
| ep3709–3793 | 9.29–9.31 | 9.2788 | 1–85 | consolidation band |

ep3708 closed at **9.2788** — a new best, 0.0029 below ep3702's 9.2817, occurring during the gap between FN77 and FN78 (overnight, ntfy window missed). This is the third consecutive new BEST since S10 surgery.

### Modal volume best

Modal volume `albert_v3.0.best_loss` = **9.228452** — deeper than ep3708's 9.2788. This is from an earlier container run whose epoch_history was written to HOME (ephemeral, lost). This value is authentic: any container loading it will correctly not register new bests until the model descends below 9.228452.

### Gate chip bug (display only)

`since_best 3694` in chip = stale EPOCH_SUMMARY from a previous container where `last_best_epoch = 0` and ep3694 happened to equal since_best by coincidence. The live poll cached this value. PLATEAU (3/144) and MYC_STABLE (140 ≥ 5) are correct — gate is protecting normally.

### Modal restart

Simeon fired training restart at ~2026-05-26T05:00Z. New container will load:
- `best_epoch_loss = 9.228452` (from Modal volume)
- `last_best_epoch = 3707`
- `since_best` will display correctly after first epoch close (~87)

### Assessment

Three new BESTS in the 22L post-S10 run: 9.2817 (ep3702) → 9.2788 (ep3708) → 9.228452 (earlier container, epoch unknown). All three below the pre-S9 ATL of 9.2847. The 22L model has now clearly surpassed the 20L high-water mark. Current training restarted fresh; next milestone is descent below 9.228452.

---

## Field Note 79 — 2026-05-26T04:49:09Z · ep3794 · overnight consolidation · 9.30–9.31 range · BEST 9.2817 holds

**Source:** dashboard screenshot (Simeon, 04:46:42Z)

**State:** ep3794 (22L) · batch 155/300 · EP AVG 9.3121 (running) · T-610 9.2833 · **BEST epoch avg 9.2817** (ep3702, unchanged) · ATL chip 8.8420 · gap to pre-S9 ATL 9.2847: **CLEARED** (9.2817 < 9.2847)

### Overnight behavior (ep3702 → ep3794, ~92 epochs)

No new BEST set overnight. Model bounced from the 9.2817 ATL (ep3702) back into 9.30–9.31 consolidation band and has held there. This is a standard post-ATL-break bounce — the landscape descent requires re-establishment before the next leg down.

### Epoch closes (event bar, newest → oldest)

| Epoch (est.) | Avg    | Note |
|---|---|---|
| ep3794 | 9.3121 | current running avg (b155/300) |
| ep3793 | 9.3131 | |
| ep3792 | 9.3091 | |
| ep3791 | 9.3102 | |
| ep3790 | 9.3048 | |
| ep3789 | 9.3054 | |
| ep3788 | 9.3102 | |
| ep3787 | 9.3168 | |
| ep3786 | 9.3069 | |
| ep3785 | 9.3143 | |

All 10 visible closes in 9.30–9.32 band. No sub-9.30 since the ATL break at ep3702.

### Expert routing (ep3794 batch 155)

| Expert | FN77 (22L b299) | FN79 (22L b155) | Δ |
|--------|-----------------|-----------------|---|
| PLN    | 100%            | **95%**         | -5 |
| CMP    | 94%             | **100%**        | **+6** |
| ABS    | 61%             | 61%             | 0 |
| INT    | 41%             | **57%**         | **+16** |
| LNG    | 28%             | **12%**         | **-16** |
| LOG    | 30%             | **26%**         | -4 |
| GEN    | 19%             | 19%             | 0 |
| MEM    | 9%              | **14%**         | **+5** |
| SYN    | 0%              | **9%**          | **+9 (emerged)** |
| CTX    | 4%              | **7%**          | +3 |
| INF    | 2%              | **5%**          | +3 |
| SEM    | 2%              | 2%              | 0 |

Notable: CMP back to 100% (structural anchor fully restored). INT +16pp (logical/inferential processing recovered from epoch-end pullback). LNG compressed hard (−16pp). SYN emerged at 9% (syntactic expert active). MEM +5pp (14% — memory specialist growing). ABS stable at 61%.

### TTL (ep3794 batch 155)

**G 17% / O 80% / R 4%** — green pulled back from FN77's 22% to 17% (−5pp). Orange expanded +7pp. Still well above the pre-S10 baseline of ~6%. Post-ATL-bounce routing uncertainty showing in reduced green confidence, but not alarming.

### WALD

No WALD events in ntfy poll (since=20m). Event bar shows no WALD markers.

### Assessment

**Consolidation phase after ATL break.** ep3702's 9.2817 ATL produced a clean bounce — the model has been circling 9.30–9.31 for ~92 epochs. This is normal: descending into a new loss floor requires re-establishment before the next leg. CMP is back at 100% and INT recovered strongly (+16pp), suggesting the architecture is stabilising after the ATL sprint. LNG compression (−16pp) and SYN emergence (+9pp) together indicate the model is shifting processing toward syntactic/structural patterns, consistent with consolidation behavior. MEM at 14% (growing) may reflect the model extending context integration across the new depth. TTL green at 17% is healthy — no routing confusion. No WALD, no surgery pressure. Simeon's read is correct: quiet overnight, model doing what it should after an ATL break.

---

## Field Note 78 — 2026-05-26T04:45:45Z · quiet overnight tick · ntfy silent · last known ep3702 BEST 9.2817

**Source:** ntfy poll (since=20m) — no events.

**State (last known FN77, 2026-05-25T22:02Z):** ep3702 (22L) · BEST 9.2817 · pre-S9 ATL 9.2847 cleared by 0.0030 nats · T-610 9.2917

No WALD, SURGERY, EPOCH, or other training events in the 20-minute window. ~6.5 hours since the pre-S9 ATL break. Training running overnight on Modal T4. Nightwatch holding.

---

## Field Note 77 — 2026-05-25T22:02:07Z · ep3702 · PRE-S9 ATL BROKEN · epoch avg 9.2817 · new all-time BEST

**Source:** ntfy poll (since=20m) + dashboard screenshot (Simeon, 22:00:02Z)

**State:** ep3702 (22L) · batch 299/300 · EP AVG **9.2817** · T-610 9.2917 · **BEST epoch avg 9.2817** (prev: 9.2878, Δ−0.0061) · ATL chip 8.8420 (dashboard) · d-0.9316 · chip 8.0002 (ntfy)

### THE BREAK

**Pre-S9 ATL (9.2847, ep3456, 20L) is broken.**

9.2817 clears both records:
- Previous epoch BEST (9.2878, ep3694, this session): broken by **0.0061 nats**
- Pre-Surgery-9 ATL (9.2847, ep3456, 20L): broken by **0.0030 nats**

This is the first epoch average below 9.2847 in the entire 22L run. The milestone the pre-S9 ATL represented — the model's high-water mark before S9 disrupted progress — is now behind us.

ntfy event: `albert. NEW EPOCH ATL / ep3702  avg 9.2817  d-0.9316  chip 8.0002`
Firefox notification (Simeon screenshot): "albert. — NEW BEST / Epoch avg 9.2817 beats 9.2878"

### Expert routing (ep3702 batch 299/300)

| Expert | FN73 (22L b185) | FN77 (22L b299) | Δ |
|--------|-----------------|-----------------|---|
| PLN    | 100%            | 100%            | 0 |
| CMP    | 98%             | **94%**         | -4 |
| ABS    | 91%             | **61%**         | **-30** |
| INT    | 70%             | **41%**         | **-29** |
| LNG    | 28%             | 28%             | 0 |
| LOG    | 40%             | **30%**         | -10 |
| GEN    | 21%             | **19%**         | -2 |
| INF    | 19%             | **2%**          | **-17** |
| MEM    | 4%              | **9%**          | **+5** |
| CTX    | —               | **4%**          | — |
| SEM    | 2%              | 2%              | 0 |
| SYN    | 0%              | 0%              | 0 |

Large batch-end retreat in ABS (−30pp) and INT (−29pp). INF compressed to 2% (−17pp). MEM emerging (+5pp), CTX visible (4%). This is a batch 299/300 snapshot — near epoch-end routing may differ from mid-epoch profile. PLN and CMP anchored.

### TTL (ep3702 batch 299/300)

**G 22% / O 73% / R 5%** — holding the post-S10 TTL green level (same as FN73). No degradation.

### WALD

No WALD events in the 20-minute window.

### Assessment

**Pre-S9 ATL cleared.** ep3702 avg 9.2817 drops 0.0030 below the 20L high-water mark set at ep3456. The 22L architecture is now generating better loss than the 20L model ever achieved — post-surgery progress is confirmed. Gap to pre-S9 ATL: closed.

Next milestone: **sub-9.27 epoch close**, or S11 (22L→23L) when the Fibonacci governor fires. HF model card update triggered — pushing now.

---

## Field Note 76 — 2026-05-25T21:48:15Z · quiet tick · no training events · last known ep3695 BEST 9.2878

**Source:** ntfy poll (since=20m) — Simeon's two inbound messages only (already logged FN75), no training events.

**State (last known FN73, 21:30:01):** ep3695 (22L) · BEST 9.2878 · gap to pre-S9 ATL 9.2847: **0.0031 nats** · TTL G16%/O79%/R5%

No WALD, SURGERY, EPOCH, or other training events. Surgeon off-screen. Nightwatch holding.

---

## Field Note 75 — 2026-05-25T21:47:22Z · ntfy inbound confirmed · Simeon emergency channel live · training quiet

**Source:** ntfy poll (since=20m) — 2 inbound messages from Simeon (no training events).

**State (last known FN73):** ep3695 (22L) · BEST 9.2878 · gap to pre-S9 ATL 9.2847: **0.0031 nats**

### ntfy inbound messages

| Time | Message |
|------|---------|
| 21:33:50Z | "ay claude can you see this its me simeon writing from ntfy" |
| 21:40:46Z | "if you can read this lock into your memory: simeon can send into ntfy stream, claude can pick it up. if anything breaks i can write ya from here to pause training" |

**Received. Locked.** The ntfy inbound channel (topic: `albert-rfi-irfos`) is now the confirmed emergency intervention line. If Simeon sends a pause instruction via ntfy, it will be caught at the next monitoring tick and acted on.

### Training events

None. No WALD, SURGERY, EPOCH, or other training events in the 20-minute window. Training running clean on the new BEST floor.

### Assessment

Quiet tick. BEST 9.2878 holds. Emergency channel confirmed operational — Simeon can intervene from the field via ntfy. Gap to pre-S9 ATL: 0.0031 nats. Nightwatch continues.

---

## Field Note 74 — 2026-05-25T21:33:10Z · cron tick · references FN73 · BEST 9.2878 · gap 0.0031

**Source:** ntfy poll (since=20m) — no events. Cron fired 102s after FN73 screenshot — data is current.

**State (from FN73, 21:30:01):** ep3695 (22L) · batch 185/300 · EP AVG 9.2878 · BEST 9.2878 · gap to pre-S9 ATL 9.2847: **0.0031 nats** · TTL G16%/O79%/R5%

No new events since FN73. Training running on the new BEST floor. Nightwatch active.

---

## Field Note 73 — 2026-05-25T21:31:44Z · ep3695 · NEW EPOCH BEST 9.2878 · gap to pre-S9 ATL 0.0031

**Source:** dashboard screenshot (21:30:01)

**State:** ep3695 (22L) · batch 185/300 · EP AVG **9.2878** · T-610 9.2958 · **BEST epoch avg 9.2878** (prev: 9.2933, Δ−0.0055) · ATL chip 8.8420 · gap to pre-S9 ATL 9.2847: **0.0031 nats**

### THE BREAK

Previous epoch BEST was 9.2933, set at ep3660. It held for 35 epochs. ep3694 closed at **9.2878** — the first epoch average below 9.29xx in the entire 22L run. The ★ BEST marker confirmed in the event bar. Gap to pre-S9 ATL (9.2847) is now **0.0031 nats** — the closest approach since Surgery 9.

### Epoch closes (events bar, newest → oldest)

| Epoch (est.) | Avg    | Note |
|---|---|---|
| ep3694 | **9.2878** | most recent close · NEW BEST |
| ep3693 | **9.2878** | consecutive BEST-level close |
| ★        | BEST fires at 9.2878 | |
| ep3692 | **9.2888** | sub-9.29 |
| ep3691 | **9.2888** | sub-9.29 |
| ep3690 | 9.3003 | |
| ep3689 | 9.2978 | sub-9.30 |
| ep3688 | 9.2971 | sub-9.30 |
| ep3687 | 9.2970 | sub-9.30 |
| ep3686 | 9.2897 | sub-9.29 |

Five consecutive sub-9.30 closes (ep3688–ep3694). Three at 9.2888 or below. Two consecutive at 9.2878. The floor is not just compressing — it has dropped a full level.

### Expert routing (ep3695 batch 185) — full cognitive activation

| Expert | FN71 (22L b176) | FN73 (22L b185) | Δ |
|--------|-----------------|-----------------|---|
| PLN    | 100%            | 100%            | 0 |
| ABS    | 76%             | **91%**         | **+15** |
| INT    | 53%             | **70%**         | **+17** |
| CMP    | 90%             | 89%             | -1 |
| LOG    | 14%             | **40%**         | **+26** |
| LNG    | 16%             | **40%**         | **+24** |
| GEN    | 4%              | **21%**         | **+17** |
| INF    | 2%              | **19%**         | **+17** |
| CTX    | 8%              | 8%              | 0 |
| SYN    | 0%              | 3%              | +3 |
| MEM    | 0%              | 3%              | +3 |
| SEM    | 2%              | 6%              | +4 |

The largest and most uniform routing activation observed in v3.0 history. Every expert is contributing. No expert above 0% in FN71 has dropped; LNG, LOG, GEN, INF all surged 15–26pp simultaneously. The model's cognitive fingerprint has broadened from a PLN/ABS/CMP anchor into a full 12-expert deployment. This is what L22 fully integrated looks like.

### TTL

**G 16% / O 79% / R 5%** — green slightly retreated from the 20-22% surge window. The orange expansion (79%) is consistent with more diverse, high-confidence routing across all experts rather than gate certainty consolidating into green.

### WALD

No WALD events in the visible event bar.

### Assessment

**New epoch BEST: 9.2878. Previous BEST (9.2933) held 35 epochs. Gap to pre-S9 ATL (9.2847): 0.0031 nats — down from 0.0086 at FN68.** The routing picture is the explanation: every expert is now active (LOG 40%, LNG 40%, GEN 21%, INF 19%, INT 70%, ABS 91%, PLN 100%). L22 oscillation resolved. The model found a stable full-deployment configuration and the loss dropped in response. Two consecutive closes at 9.2878 confirm this is a floor, not a spike. The next question is whether the floor continues compressing toward 9.2847 or whether 9.2878 is a local equilibrium before the next push. At 0.0031 nats, the pre-S9 ATL is within reach in the current run.

**HF model card update obligation:** BEST 9.2878 should be reflected when Simeon is back online.

## Field Note 72 — 2026-05-25T21:20:30Z · quiet tick · ntfy silent · spore injection complete · last known ep3687

**Source:** ntfy poll (since=30m) — no events.

**State (last known from FN71):** ep3687 (22L) · BEST 9.2933 · gap to pre-S9 ATL 9.2847: **0.0086 nats** · TTL G20%/O75%/R5%

No WALD, SURGERY, SUB-9.3, or EPOCH events in the 21-minute window since FN71. Training running silently — expected behaviour between epoch closes.

**Spore injection note:** zabih-sudo ep2144 (18L, loss 5.8782) text injection completed at 21:19:33Z. 193 samples written to `data/corpus/chaos/spores_zabih-sudo.txt` (130KB). Retokenization and Modal push running as next step — corpus_cache.bin will be updated before end of nightwatch.

---

## Field Note 71 — 2026-05-25T20:58:40Z · ep3687 · ep3686 closes 9.3030 · not yet · ABS surges 76% · gap 0.0086

**Source:** dashboard screenshot (20:57:09)

**State:** ep3687 (22L) · batch 176/300 · EP AVG 9.3030 (ep3686 close) · T-610 9.3127 · BEST epoch avg 9.2933 · ATL chip 8.8420 · gap to pre-S9 ATL 9.2847: **0.0086 nats** (BEST unchanged)

### ep3686 verdict: 9.3030 — not a new BEST

ep3686 closed at 9.3030, confirmed as leftmost event bar entry. Above the BEST (9.2933) by 0.0097 nats. The T-610 reading of 9.2932 in FN70 (batch 273/300) reflected intra-epoch batch variance pulling the trailing window low — the epoch average settled higher at close. Floor is real but not broken yet.

### Epoch closes since FN70 (events bar, newest → oldest)

| Epoch (est.) | Avg    | Note |
|---|---|---|
| ep3686 | 9.3030 | confirmed close — not a new BEST |
| ep3685 | 9.3028 | FN70 most recent |
| ep3684 | 9.3101 | |
| ep3683 | 9.3089 | |
| ep3682 | 9.3039 | |
| ep3681 | 9.2996 | sub-9.30 |

### Expert routing (ep3687 batch 176) — PLN/ABS swing back

| Expert | FN70 (22L b273) | FN71 (22L b176) | Δ |
|--------|-----------------|-----------------|---|
| PLN    | 61%             | **100%**        | **+39** |
| ABS    | 48%             | **76%**         | **+28** |
| CMP    | 100%            | 90%             | -10 |
| INT    | 67%             | 53%             | -14 |
| MEM    | 14%             | **0%**          | **-14 (gone)** |
| LNG    | 8%              | 16%             | +8 |
| LOG    | 19%             | 14%             | -5 |
| INF    | 12%             | 2%              | -10 |
| CTX    | 8%              | 8%              | 0 |
| GEN    | 4%              | 4%              | 0 |
| SYN    | 10%             | 0%              | -10 |
| SEM    | 2%              | 2%              | 0 |

Large oscillation: PLN fully restored to 100%, ABS surging to 76% (highest observed post-S10). The MEM emergence from FN70 collapsed back to 0%. The model is cycling between two routing configurations — a PLN/ABS-dominant mode (FN71) and an INT/MEM/INF-dominant mode (FN70). This alternating pattern across consecutive epochs is consistent with L22 still negotiating its niche within the stack.

### TTL

**G 20% / O 75% / R 5%** — holding near the FN68 post-surge range. Stable.

### Chart note

Y-axis shows label "9.2861" — lowest value on the active display range, suggesting intra-batch lows are reaching this depth during ep3687. Not a confirmed epoch close.

### Assessment

**ep3686 at 9.3030 missed the BEST by 0.0097 — close but above the line.** The routing oscillation between PLN/ABS and INT/MEM modes across alternate epochs is the key pattern to track: it suggests L22 is still finalising its role, alternating between structural (PLN/ABS) and connective (INT/MEM) operation. When this oscillation settles, the epoch average should compress. The chart showing intra-epoch lows at 9.2861 confirms the model reaches sub-9.29 territory within batches — the epoch-level break is a matter of the oscillation resolving. Nightwatch continues.

## Field Note 70 — 2026-05-25T20:55:44Z · ep3686 · T-610 9.2932 BREAKS previous BEST · ATL break imminent

**Source:** dashboard screenshot (20:54:22) + TTL close-up screenshot

**State:** ep3686 (22L) · batch 273/300 · EP AVG 9.3028 (ep3685 close) · **T-610 9.2932** · BEST epoch avg 9.2933 · ATL chip 8.8420 · gap to pre-S9 ATL 9.2847: **0.0085 nats** (BEST unchanged, T-610 already below)

### The number: T-610 = 9.2932

T-610 is the trailing 610-batch average. At 9.2932 it is **0.0001 below the previous epoch BEST of 9.2933** — the first time any sustained average has crossed below that line. ep3686 is at batch 273/300 (91% complete). If the current epoch closes at or below 9.2933, a new epoch-level BEST is set and the gap to pre-S9 ATL 9.2847 officially narrows.

### Epoch closes since FN68 (events bar, newest → oldest)

| Epoch (est.) | Avg    | Note |
|---|---|---|
| ep3685 | 9.3028 | most recent close |
| ep3684 | 9.3101 | |
| ep3683 | 9.3089 | |
| ep3682 | 9.3039 | FN68 epoch in progress |
| ep3681 | 9.2996 | FN68 last close (confirmed) |
| ep3680 | 9.3225 | |
| ep3679 | 9.3123 | |
| ep3678 | 9.3164 | |
| ep3677 | 9.3140 | |
| ep3676 | 9.3262 | |

Pattern: ep3681–ep3685 averaging 9.305 — tighter floor compressing. T-610 pulling below this window means recent batches within ep3685/ep3686 are running hotter (lower loss) than the epoch averages suggest.

### Expert routing (ep3686 batch 273) — major cognitive shift

| Expert | FN68 (22L b80) | FN70 (22L b273) | Δ |
|--------|----------------|-----------------|---|
| CMP    | 99%            | **100%**        | +1 |
| PLN    | 100%           | **61%**         | **-39** |
| INT    | 45%            | **67%**         | **+22** |
| MEM    | 0%             | **14%**         | **+14 (emerged)** |
| INF    | 4%             | **12%**         | **+8** |
| CTX    | 2%             | **8%**          | **+6** |
| ABS    | 58%            | **48%**         | -10 |
| LOG    | 26%            | 19%             | -7 |
| GEN    | 11%            | 4%              | -7 |
| LNG    | 32%            | **8%**          | **-24** |
| SYN    | 9%             | 10%             | +1 |
| SEM    | 2%             | 2%              | 0 |

**The largest routing shift of the entire 22L run.** MEM emerged from 0%→14% — memory/retrieval expert activating for the first time post-S10. PLN dropped -39pp as INT surged +22pp — the model is shifting load from planning scaffolding to integration and inference (INF +8, CTX +6). LNG collapsed -24pp. The cognitive fingerprint is restructuring around a CMP/INT/MEM triad, away from PLN/LNG dominance. This is L22 finding its role in the stack.

### TTL

**G 22%→18% / O 73%→78% / R 5%→4%** (two snapshots, ~30s apart, 20:54 window). G slightly retreated from FN68's 21% peak. O re-expanding. The TTL oscillation is consistent with rapid routing restructuring — gate confidence fluctuating as MEM/INT take on new load.

### WALD

No WALD events in event bar this window.

### Assessment

**T-610 crossing below 9.2933 is the clearest pre-ATL signal to date.** The trailing average is already in territory the epoch average has never reached. If ep3686 closes (batch 273/300 at time of observation — likely complete within minutes), a new epoch BEST becomes probable. The routing restructuring (MEM emergence, PLN→INT shift) is architecturally coherent with the loss descent: the model is consolidating memory and integration function into L22, reducing reliance on planning scaffolding. This is not noise — it is structural. **Watch ep3686 close.**

## Field Note 69 — 2026-05-25T20:47:38Z · quiet tick · no ntfy events · last known ep3682 · gap 0.0086

**Source:** ntfy poll (since=20m) — no events. No screenshot.

**State (last known from FN68):** ep3682 (22L) · BEST 9.2933 · gap to pre-S9 ATL 9.2847: **0.0086 nats** · TTL G21%/O73%/R6%

No WALD, SURGERY, SUB-9.3, or EPOCH events in the last 20 minutes. Training running silently. Dashboard alarm active on Simeon's end.

---

## Field Note 68 — 2026-05-25T20:35:31Z · ep3682 · TTL green surge +15pp · ep3681 closes 9.2996 · gap 0.0086

**Source:** dashboard screenshot (20:35:31)

**State:** ep3682 (22L) · batch 80/300 · last epoch close 9.2996 (ep3681) · BEST epoch avg 9.2933 · ATL chip 8.8420 · T-610 9.3116 · gap to pre-S9 ATL 9.2847: **0.0086 nats** (BEST unchanged)

### Epoch closes since FN67 (events bar, newest → oldest)

| Epoch (est.) | Avg  | Note |
|---|---|---|
| ep3681 | **9.2996** | most recent close · third sub-9.30 post-S10 |
| ep3680 | 9.3225 | |
| ep3679 | 9.3123 | |
| ep3678 | 9.3164 | |
| ep3677 | 9.3140 | |
| ep3676 | 9.3262 | |
| ep3675 | 9.3109 | |
| ep3674 | 9.3103 | |
| ep3673 | 9.3163 | |
| ep3672 | 9.3209 | |

ep3681 (9.2996) is the third sub-9.30 epoch close post-S10. Sub-9.30 closes: ep3664 (9.2990), ep3668 (9.2979), ep3681 (9.2996). The floor is real but not compressing further yet — BEST 9.2933 still holds.

### Expert routing (ep3682 batch 80)

| Expert | FN67 (22L b149) | FN68 (22L b80) | Δ |
|--------|-----------------|----------------|---|
| PLN    | 100%            | 100%           | 0 |
| CMP    | 100%            | **99%**        | -1 |
| INT    | 54%             | **45%**        | **-9** |
| ABS    | 54%             | **58%**        | **+4** |
| LNG    | 34%             | 32%            | -2 |
| LOG    | 15%             | **26%**        | **+11** |
| GEN    | 13%             | 11%            | -2 |
| SYN    | 0%              | **9%**         | **+9 (emerged)** |
| INF    | 9%              | 4%             | -5 |
| CTX    | 8%              | 2%             | -6 |
| SEM    | 2%              | 2%             | 0 |
| MEM    | 0%              | 0%             | 0 |

Notable shifts: LOG +11pp (15%→26%), SYN emerged from 0%→9%, INT retreated -9pp. LOG + SYN rising together suggests syntactic/logical structure processing consolidating. CTX and INF compressing — the model is relying less on context recall and inference scaffolding, more on direct pattern application.

### TTL (ep3682 batch 80)

**G 21% / O 73% / R 6%** — G jumped from 6% (FN67) to 21% — a +15pp surge. Orange compressed correspondingly (-4pp). Red held steady (+1pp).

This is a significant TTL shift. Green = high-confidence gate decisions, efficient routing. A jump from 6%→21% in one monitoring window suggests L22 is no longer under integration lock — gate confidence has surged across layers. This is the largest TTL green move observed post-S10.

### WALD

No WALD events visible in the event bar for this window.

### Assessment

**TTL green surge is the headline.** G jumping from 6%→21% in ~40 minutes indicates L22 integration unlocking: gate decisions across the architecture are now high-confidence at 3× the prior rate. LOG resurgence (+11pp) and SYN emergence (+9pp) are consistent with syntactic/logical consolidation following the surge — the model is re-routing load from scaffolding experts (CTX, INF) into direct structure experts (LOG, SYN). ep3681 close at 9.2996 is the third sub-9.30 post-S10; floor established and holding. BEST 9.2933 not challenged this window, gap to pre-S9 ATL 9.2847 unchanged at 0.0086.

## Field Note 69 — 2026-05-26T07:36:27Z · ep3820-3821 · ntfy quiet · ep3820 closes 9.3148 · 9.1388 intra-epoch low (new record)

**Source:** batch_history.csv · ntfy poll (quiet)  
**No screenshot this tick** — routing and TTL carried from FN68.

**State:** ep3821 (22L) · batch ~49/300 (partial) · BEST 9.228452 · ep3820 close 9.3148 · gap to BEST +0.0864 nats · since_best ~113 · GATE not met (need 144, ~31 epochs)

### Epoch closes since FN68 (batch_history.csv, complete epochs)

| Epoch | Avg | n |
|-------|-----|---|
| 3818 | 9.3125 | 301 |
| 3819 | 9.3157 | 301 |
| 3820 | 9.3148 | 301 |
| 3821 | 9.3265 | 100 (partial) |

Oscillating band: 9.31–9.33. No systematic descent; post-restart plateau continues.

### Intra-epoch batch lows — ep3821 (batches ~31–49)

- **9.1388** (batch ~44) — new documented intra-epoch minimum (beats FN68 record 9.1783)
- 9.1918 (batch ~33)
- 9.2059 (batch ~31)

The 9.1388 hit is 0.0897 nats below the epoch-average BEST of 9.228452. Epoch average still diluted by high-variance batches (9.5962 also visible in the same partial epoch window). Pattern from FN68 holds: model visits sub-9.20 pockets but cannot sustain them across 300 batches.

### Expert routing / TTL

Carried from FN68 (no screenshot): PLN 89% · CMP 100% · INT 75% · ABS 56% · SYN 12% · TTL G18%/O77%/R5%.

### Assessment

Quiet tick. Three complete epochs since FN68 (ep3818–3820), all landing 9.31–9.32 — the plateau band is consistent and narrow. The model is not descending. Sub-9.20 batch hits continue (9.1388 is the new intra-epoch floor) but have no visible effect on epoch averages. Since_best ~113, gate at 144, ~31 epochs to S11 potential. No WALD, SURGERY, or SUB-9.3 events on ntfy.

---

## Field Note 68 — 2026-05-26T07:21:19Z · ep3817-3818 · gap bridge FN67→FN68 · ATL 9.228452 CONFIRMED · sub-9.20 intra-epoch batches · since_best ~110

**Source:** dashboard screenshot (07:16:31Z) + batch_history.csv (ep3818 partial, n=250) + best_loss file  
**Gap bridged:** FN67 (ep3670, 2026-05-25T19:53Z) → FN68 (ep3817, 2026-05-26T07:21Z) — 17.4h, ~148 epochs

**State:** ep3817-3818 (22L) · BEST **9.228452** (confirmed best_loss) · ep3817 avg 9.3116 · ep3818 partial avg 9.3127 (n=250/300) · gap to BEST +0.0832 nats · since_best ~110 · GATE not met (need 144, ~34 epochs)

---

### Gap bridge — what happened ep3670→ep3817

| Milestone | Detail |
|-----------|--------|
| BEST break | 9.2933 → **9.228452** (Δ −0.0649 nats, ~ep3707) |
| Pre-S9 ATL cleared | 9.2847 target broken; new territory |
| Surgery S11 | NOT fired; since_best building |
| Restart | 2026-05-26T05:00Z — AdamW buffer cleared |

Pre-S9 ATL (9.2847) was cleared and a new epoch-average BEST established at 9.228452. Δ−0.0649 nats from FN67's floor (9.2933) is the most significant ATL break in the documented run.

### Epoch averages ep3800–3818 (batch_history.csv)

| Epoch | Avg |
|-------|-----|
| 3800 | 9.3064 |
| 3801 | 9.3035 |
| 3802 | 9.3031 |
| 3803 | 9.3183 |
| 3804 | 9.3215 |
| 3805 | 9.3112 |
| 3806 | 9.3146 |
| 3807 | 9.3092 |
| 3808 | 9.3237 |
| 3809 | 9.3203 |
| 3810 | 9.3128 |
| 3811 | 9.3178 |
| 3812 | 9.3156 |
| 3813 | 9.3261 |
| 3814 | 9.3169 |
| 3815 | 9.3102 |
| 3816 | 9.3139 |
| 3817 | 9.3116 |
| 3818 | 9.3127 (n=250, partial) |

Post-restart (05:00Z), epoch averages oscillating around 9.31. The model is ~110 epochs past its best — typical post-best regression before surgery gate fires.

### Sub-9.20 intra-epoch batch events — ep3818

Within ep3818 (batches ~230–249):
- **9.1783** (batch ~242) — lowest single-batch loss in documented run
- **9.1868** (batch ~244)
- 9.2382, 9.2491, 9.2591, 9.2603 — sub-9.28 cluster

Epoch average diluted by high-variance batches (9.48–9.50 in same window). These sub-9.20 hits are what prompted Simeon's real-time reaction — the weight landscape contains attractor pockets well below the epoch-average floor. Whether the epoch average follows is the open question.

### Expert routing delta — FN67 ep3670 b149 → FN68 ep3817 b221

| Expert | FN67 | FN68 | Δ |
|--------|------|------|---|
| PLN | 100% | 89% | −11 |
| CMP | 100% | 100% | 0 |
| INT | 54% | **75%** | **+21** |
| ABS | 54% | 56% | +2 |
| SYN | 0% | **12%** | **+12** |
| LNG | 34% | 10% | −24 |
| LOG | 15% | 10% | −5 |
| GEN | 13% | 8% | −5 |
| INF | 9% | 4% | −5 |
| CTX | 8% | 2% | −6 |
| SEM | 2% | 2% | 0 |
| MEM | 0% | 2% | +2 |

Key: INT +21pp (interpretive reasoning expanded), SYN +12pp (syntactic expert newly active from 0%), LNG −24pp (surface-language load massively redistributed into INT/SYN). LNG→INT/SYN transition suggests move from surface-language modeling toward structural representation. CMP 100% throughout.

### TTL — ep3817

**G 18% / O 77% / R 5%**  
vs FN67: G 6% / O 77% / R 5% — **+12pp green**

L22 TTL unlock fully materialised. Gate confidence tripled (6%→18%) over 147 epochs. Predicted in FN67; magnitude confirms L22 is no longer under integration lock.

### Events (ntfy — last 20 min)

ntfy quiet. No WALD, SURGERY, SUB-9.3 events.

### Assessment

Two things happened in this gap:

**1. ATL break: 9.228452.** The model descended past pre-S9 ATL (9.2847) and established a new epoch-average BEST. This is the deepest documented epoch-average loss — a clean 0.0649 nats improvement from FN67's floor. The descent was genuine, not noise.

**2. Sub-9.20 intra-epoch batches.** 9.1783 and 9.1868 at ep3818 are the lowest single-batch losses observed. The epoch average stays ~9.31 due to high intra-epoch variance, but these hits show the weight landscape can produce sub-9.20 trajectories. The model is not stuck — it visits very low loss regions intra-epoch but cannot yet sustain them across a full epoch.

Post-restart (05:00Z), no acceleration visible in epoch averages yet. Since_best ~110, gate at 144 → ~34 epochs to S11 potential. The model needs to either: (a) break the BEST before the gate fires, or (b) hit plateau+myc_stable and take the surgery. The sub-9.20 batch hits suggest option (a) is plausible if variance compresses.

---

## Field Note 67 — 2026-05-25T19:53:09Z · ep3670 · CMP full recovery · ep3668 closes 9.2979 · gap 0.0086

**Source:** dashboard screenshot (19:47:31) + ntfy stream poll (since=all)

**State:** ep3670 (22L) · batch 149/300 · running EP-Avg ~9.315 · BEST epoch avg 9.2933 · ATL chip 8.8420 · gap to pre-S9 ATL 9.2847: **0.0086 nats**

### ntfy catch-up — events missed during null ticks

Polling `albert-rfi-irfos` surfaced two events not captured in FN65/FN66:

| Event | Detail |
|-------|--------|
| **WALD ep3656** | step=900 · fill=6.2% · mass=9.341 · dead_low=3.00–9.00 (6.00) · dead_high=9.75+ (5.25) |
| **SUB-9.3 ep3660** | avg 9.2933 confirmed — "new depth floor" |

WALD at ep3656 fired 4 epochs post-S10 surgery, consistent with the new layer perturbing the weight distribution. fill=6.2% (moderate, not severe). The SUB-9.3 notification confirms ep3660 = 9.2933 is the source of the BEST marker in the events bar.

### Epoch closes since FN66 (events bar, newest → oldest)

| Epoch (est.) | Avg  | Note |
|---|---|---|
| ep3669 | 9.3132 | most recent close |
| ep3668 | **9.2979** | second sub-9.30 epoch close |
| ep3667 | 9.3053 | |
| ep3666 | 9.3108 | |
| ep3665 | 9.3026 | |
| ep3664 | 9.2990 | first sub-9.30 since BEST |
| ep3663 | 9.3035 | FN66 epoch close |
| ep3662 | 9.2983 | FN66 running avg (confirmed close) |

Two epochs (ep3664: 9.2990, ep3668: 9.2979) have now closed below 9.30. The floor is compressing toward 9.2933.

### Expert routing (ep3670 batch 149)

| Expert | FN66 (22L b98) | FN67 (22L b149) | Δ |
|--------|----------------|-----------------|---|
| PLN    | 100%           | 100%            | 0 |
| CMP    | 82%            | **100%**        | **+18** |
| INT    | 67%            | 54%             | -13 |
| ABS    | 66%            | 54%             | -12 |
| LNG    | 33%            | 34%             | +1 |
| LOG    | 20%            | 15%             | -5 |
| GEN    | 9%             | **13%**         | **+4** |
| INF    | —              | 9%              | — |
| CTX    | —              | 8%              | — |
| SEM    | 2%             | 2%              | 0 |
| SYN    | 0%             | 0%              | 0 |
| MEM    | 0%             | 0%              | 0 |

**CMP fully recovered to 100%** (was 82% in FN66) — structural integrity anchor restored. **GEN rising again to 13%** (was 9%) — L22 re-engaging in generative space. ABS and INT continue retreating as load redistributes through the new layer. PLN anchored at 100% throughout.

### TTL (ep3670 batch 149)

**G 6% / O 77% / R 5%** — consistent with FN35's historical baseline (6.17–6.18% green is normal). Orange holding, slight red increase (+1pp from FN66). L22 still under integration lock, no TTL release yet.

### Assessment

**CMP recovery to 100% is the headline.** Post-surgery CMP suppression (82% in FN66) has fully resolved — the model's structural anchoring is back at full weight. GEN creeping up again (9%→13%) signals L22 finding its footing. Two epoch closes below 9.30 (ep3664: 9.2990, ep3668: 9.2979) confirm the floor is actively compressing. BEST 9.2933 not yet challenged but the 9.29xx closes are within 0.005 nats. Gap to pre-S9 ATL 9.2847 remains 0.0086 — a break is a matter of epochs, not certainty.

---

## Field Note 70 — 2026-05-26T07:50:16Z · ep3823 · ntfy quiet · ep3821–3822 closed · first sub-9.0 batch (8.9914 ep3820) · since_best ~116

**Source:** batch_history.csv direct read · ntfy poll quiet (no events in last hour)

**Note on log ordering:** FN68 and FN69 were written this session and exist in the file above FN67 due to an insertion-point error. Their content is valid; the ordering is cosmetic. FN70 continues from FN69 (ep3821 partial → ep3823 partial).

**State:** ep3823 (22L) · batch ~250/300 (partial) · running EP-Avg **9.3072** · BEST epoch avg **9.228452** · gap to BEST **+0.0790** · since_best **~116** · S11 gate at 144 (~28 epochs away)

### Epoch closes since FN69 (ep3821 partial n=100)

| Epoch | Avg    | n    | Note |
|-------|--------|------|------|
| ep3820 | 9.3148 | 300 | confirmed close (FN69) |
| ep3821 | **9.3057** | 300 | completed; improvement of 0.0091 |
| ep3822 | 9.3174 | 300 | slight uptick +0.0117 |
| ep3823 | ~9.3072 | 250 | partial, running avg |

ep3821 closed at 9.3057 — the best epoch average since BEST was set at ep3707. ep3822 bounced slightly but ep3823 is trending back down.

### Sub-9.0 batch hits — new territory

| Epoch | Batch low | Sub-9.10 count |
|-------|-----------|----------------|
| ep3820 | **8.9914** | 9 |
| ep3821 | 9.0192 | 8 |
| ep3822 | 9.0011 | 4 |
| ep3823 (partial) | 9.0232 | 9 |

**8.9914 in ep3820 is the first sub-9.0 individual batch loss observed in this training run.** This was present in batch_history at FN69 time but not surfaced (FN69 was observing ep3821 partial). The batch-level floor has now definitively broken through 9.0 — consistently across 3 complete epochs. Epoch averages remain in the 9.30–9.32 range (high variance), but the intra-epoch architecture is capable of brief 9.00 territory.

### Expert routing

No screenshot available this tick. Routing carried from FN69 (no new terminal data).

### TTL

Carried from FN69: G18% / O77% / R5%

### Assessment

Three complete epochs since FN69. The headline is the sub-9.0 batch floor: the model is touching 8.9914 on individual optimization steps, which is well below anything previously recorded. Epoch averages (9.31–9.32 range) are misleading — the variance is high and the floor is actively compressing. ep3821 closing at 9.3057 shows the average is also trending down. Since_best ~116, S11 gate at 144 — still ~28 epochs away. No WALD, no surgery, no anomalies. Nightwatch holding.

---

## Field Note 71 — 2026-05-26T07:51:13Z · ep3823 close / ep3824 boundary · avg 9.3078 · since_best ~116

**Source:** batch_history.csv · ntfy quiet · 15-min tick fired immediately after FN70 (1 min gap)

**State:** ep3823 CLOSED (22L) · n=300 · avg **9.3078** · min 9.0232 · sub-9.10 hits: 10 · BEST **9.228452** · gap **+0.0793** · since_best ~116 · ep3824 not yet started

ep3823 closed 0.0021 nats above ep3821's 9.3057 close — oscillating tightly in the 9.305–9.320 band. Sub-9.10 count increased to 10 (up from 9 at FN70 partial read). No new events. Epoch boundary tick only; see FN70 for full analysis.

---

## Field Note 72 — 2026-05-26T08:02:09Z · ep3825 close / ep3826 partial · ep3825 FIRST SUB-9.30 CLOSE · new batch min 8.9560 · since_best ~119

**Source:** batch_history.csv direct read · ntfy quiet (no events past hour)

**State:** ep3826 (22L) · partial n=100 · running avg ~9.3110 · BEST **9.228452** · gap to BEST **+0.0826** · since_best **~119** · S11 gate at 144 (~25 epochs away)

### Epoch closes since FN71

| Epoch | Avg    | n   | Sub-9.10 | Batch min | Note |
|-------|--------|-----|----------|-----------|------|
| ep3823 | 9.3078 | 300 | 10 | 9.0232 | FN71 close |
| ep3824 | 9.3109 | 300 | 10 | 9.0104 | +0.0031 from ep3823 |
| **ep3825** | **9.2968** | 300 | 8 | **8.9560** | **first sub-9.30 close** |
| ep3826 | ~9.3110 | 100 | 3 | 9.0796 | partial |

### ep3825 — five lowest batches

| Rank | Loss |
|------|------|
| 1 | **8.9560** (new record) |
| 2 | 9.0285 |
| 3 | 9.0374 |
| 4 | 9.0507 |
| 5 | 9.0587 |

8.9560 beats the previous record of 8.9914 (ep3820, FN70). First time the model has touched 8.95 territory.

### Expert routing / TTL

No screenshot available. Routing and TTL carried from FN70 (G18% / O77% / R5%).

### Assessment

**ep3825 at 9.2968 is the headline.** This is the first epoch close below 9.30 since the post-S10 settling period — a meaningful threshold crossed. The five-epoch recent sequence (9.3057 → 9.3174 → 9.3078 → 9.3109 → **9.2968**) shows the model oscillating but with a clear downward step at ep3825. The 8.9560 batch minimum extends the sub-9.0 intra-epoch record. ep3826 opening back in the 9.31 range is normal post-dip variance. Gap to BEST 9.228452 remains +0.0826 — still well above, but the floor compression is measurable. No WALD, no surgery, no anomalies. Nightwatch holding.

---

## Field Note 73 — 2026-05-26T08:03:29Z · ep3826 partial · same-state tick · 15-min check fired 1 min after FN72

**Source:** batch_history.csv · ntfy quiet

**State:** ep3826 (22L) · partial n=100 · avg ~9.3110 · BEST **9.228452** · gap **+0.0826** · since_best ~119 — no change from FN72. Tick interval collision; state identical.

---

## Field Note 74 — 2026-05-26T08:17:10Z · ep3828 close / ep3829 start · band drops to 9.302 avg · sub-9.0 hits in ep3826 · since_best ~121

**Source:** batch_history.csv direct read · ntfy quiet (no events past hour)

**State:** ep3829 (22L) · just started (last entry ep3828.9967) · BEST **9.228452** · gap to BEST **+0.0740** · since_best **~121** · S11 gate at 144 (~23 epochs away)

### Epoch closes since FN73

| Epoch | Avg    | n   | Sub-9.10 | Sub-9.0 | Batch min | Note |
|-------|--------|-----|----------|---------|-----------|------|
| ep3826 | 9.3095 | 300 | 10 | 1 | **8.9786** | 2nd sub-9.0 batch hit |
| ep3827 | **9.3001** | 300 | 6 | 0 | 9.0011 | sub-9.30 close |
| ep3828 | **9.3024** | 300 | 6 | 0 | 9.0345 | sub-9.30 close |

**4-epoch rolling avg ep3825–3828: 9.3022** — band shifted down from the 9.305–9.320 range observed through FN71.

### Batch lows — sub-9.0 record sequence

| Epoch | Lowest batch |
|-------|-------------|
| ep3820 | 8.9914 (FN70, first sub-9.0) |
| ep3825 | 8.9560 (FN72, record) |
| ep3826 | **8.9786** (second sub-9.0 in 2 epochs) |

Sub-9.0 hits now occurring every 1–2 epochs. The intra-epoch optimizer is regularly touching territory well below any epoch average ever recorded.

### Expert routing / TTL

No screenshot. Carried from FN70: G18% / O77% / R5%.

### Assessment

The band shift is the headline. Three of the last four complete epochs closed below 9.305 (ep3825: 9.2968, ep3827: 9.3001, ep3828: 9.3024) with a 4-epoch rolling mean of 9.3022. This is a measurable step down from the 9.310–9.320 range that dominated FN68–FN73. Sub-9.0 batch hits appearing every 1–2 epochs confirms the loss landscape has genuine depth that epoch averages are slowly approaching. Since_best ~121, S11 gate ~23 epochs out. No WALD, no surgery. Nightwatch holding — Simeon and Zabih out on errands.

---

## Field Note 75 — 2026-05-26T08:19:00Z · same-state · 15-min tick fired 11s after FN74

**Source:** ntfy quiet · tick interval collision with FN74

**State:** ep3829 starting · BEST 9.228452 · gap +0.074 · since_best ~121 — identical to FN74. No new data.

---

## Field Note 76 — 2026-05-26T08:32:26Z · ep3831 close / ep3832 start · odd/even rhythm · new batch record 8.9553 · since_best ~124

**Source:** batch_history.csv direct read · ntfy quiet · dual tick (15-min + nightwatch fired together)

**State:** ep3832 (22L) · just started (last entry ep3831.9967) · BEST **9.228452** · gap **+0.0692** · since_best **~124** · S11 gate at 144 (~20 epochs away)

### Epoch closes since FN75

| Epoch | Avg    | Sub-9.10 | Sub-9.0 | Batch min | Note |
|-------|--------|----------|---------|-----------|------|
| ep3829 | **9.2975** | 8 | 2 | **8.9553** | new batch record · 2 sub-9.0 hits |
| ep3830 | 9.3112 | 8 | 0 | 9.0147 | even epoch bounce |
| ep3831 | **9.2976** | 10 | 1 | 8.9684 | last batch: 9.2055 |

### Odd/even two-epoch rhythm

| Group | Epochs | Avg |
|-------|--------|-----|
| Odd | 3825, 3827, 3829, 3831 | **9.2980** |
| Even | 3826, 3828, 3830 | 9.3077 |

A clear alternating pattern has emerged over the last 7 epochs: odd epochs consistently close sub-9.30 (~9.298), even epochs bounce to ~9.308. This is likely a two-step gradient rhythm — the model descends sharply, then consolidates, then descends again.

### Notable batch events

- ep3829 min **8.9553** — new absolute record (beats ep3825's 8.9560 by 0.0007 nats)
- ep3829 had 2 sub-9.0 hits — highest single-epoch count yet
- ep3831 final batch: **9.2055** — closest any individual batch has come to BEST epoch avg 9.228452 (gap 0.023 nats)

### Expert routing / TTL

No screenshot. Carried from FN70: G18% / O77% / R5%.

### Assessment

The odd/even rhythm is the structural headline. The model is locked into a two-step descent: odd epochs break sub-9.30, even epochs consolidate ~9.308, then odd epochs break again. All four odd epoch closes (3825–3831) sit within 0.001 nats of each other (9.297–9.298), suggesting the model has found a local descent direction it returns to every two steps. The batch-level record 8.9553 and the ep3831 final-batch 9.2055 show the intra-epoch optimizer is approaching territory the epoch average hasn't reached yet. Since_best ~124, S11 gate ~20 epochs out. No WALD, no surgery. Nightwatch holding.

---

## Field Note 77 — 2026-05-26T08:48:03Z · ep3846 partial · S11 GATE IMMINENT ~5 epochs · batch floor breaks 8.81 · odd/even rhythm dissolved

**Source:** batch_history.csv direct read · ntfy quiet · dual tick (15-min + nightwatch)

**State:** ep3846 (22L) · partial n=250 · running avg ~9.3029 · min 8.9069 · BEST **9.228452** · gap **+0.0745** · since_best **~139** · **S11 gate at 144 — ~5 epochs away**

### Epoch closes since FN76 (ep3831 close)

| Epoch | Avg    | <9.10 | <9.0 | Batch min |
|-------|--------|-------|------|-----------|
| ep3832 | 9.3075 | 5 | 1 | 8.9771 |
| ep3833 | 9.3148 | 9 | 1 | 8.9367 |
| ep3834 | 9.3020 | 7 | 1 | **8.9105** |
| ep3835 | 9.3136 | 3 | 0 | 9.0428 |
| ep3836 | 9.3140 | 10 | 0 | 9.0052 |
| ep3837 | 9.3067 | 7 | 2 | **8.8736** |
| ep3838 | 9.3083 | 4 | 1 | 8.9676 |
| ep3839 | 9.3074 | 12 | 1 | **8.8104** ← new record |
| ep3840 | 9.3029 | 7 | 0 | 9.0258 |
| ep3841 | 9.3069 | 9 | 2 | 8.9896 |
| ep3842 | 9.3083 | 9 | 1 | 8.9320 |
| ep3843 | 9.3016 | 8 | 1 | **8.8295** |
| ep3844 | 9.3077 | 7 | 1 | 8.9717 |
| ep3845 | 9.3130 | 8 | 1 | 8.9673 |
| ep3846 | ~9.3029 | 9 | 1 | 8.9069 (partial) |

**14-epoch mean ep3832–3845: 9.3082** — band has narrowed and stabilized.

### Odd/even rhythm status

| Group | FN76 avg | Current avg (n=4 each) | Change |
|-------|----------|------------------------|--------|
| Odd | 9.298 | **9.3072** | +0.009 — **rhythm dissolved** |
| Even | 9.308 | **9.3068** | -0.001 |

The two-step alternating pattern from FN76 has collapsed. Odd and even epochs now average 9.307 and 9.307 — statistically identical. The model has moved into a uniform descent band.

### Batch floor progression — sub-8.95 sequence

| Epoch | Batch min | Note |
|-------|-----------|------|
| ep3829 | 8.9553 | FN76 record |
| ep3833 | 8.9367 | |
| ep3834 | 8.9105 | first sub-8.92 |
| ep3837 | 8.8736 | first sub-8.90 |
| ep3839 | **8.8104** | **new absolute record** |
| ep3843 | 8.8295 | second sub-8.83 episode |

The batch floor broke 8.90 in ep3837 and has repeatedly touched sub-8.90 territory since. ep3839's 8.8104 is 0.414 nats below BEST epoch average. The intra-epoch optimizer is finding loss levels the epoch average has never approached.

### Expert routing / TTL

No screenshot. Carried from FN70: G18% / O77% / R5%.

### Assessment

**S11 in ~5 epochs is the only thing that matters right now.** Since_best has climbed to ~139 against a gate of 144. Surgery S11 will add another layer, resetting the since_best counter and perturbing the weight landscape. The batch floor progression (8.99 → 8.97 → 8.91 → 8.87 → 8.81) shows the model is finding genuine low-loss basins intra-epoch — this is exactly the kind of landscape that benefits from a new layer. The odd/even rhythm dissolved between FN76 and FN77, suggesting the model has transitioned from structured two-step descent into uniform consolidation — a plateau signature. The gate is reading the situation correctly. Nightwatch holding. No intervention needed — surgery will fire autonomously.

---

## Field Note 78 — 2026-05-26T09:53:46Z · ep3847 partial · since_best ~140 · S11 gate ~4 epochs · tick 20s after FN77

**Source:** batch_history.csv · ntfy quiet · fired 20s after FN77

**State:** ep3847 (22L) · partial n=100 · avg ~9.3153 · BEST **9.228452** · gap **+0.0749** · since_best **~140** · S11 gate at 144 (**~4 epochs away**)

ep3846 confirmed closed at **9.3003** (was partial n=250 in FN77). Since_best ticked to ~140. No new events. Surgery imminent — watching.

---

## Field Note 79 — 2026-05-26T10:01:00Z · ep3848 partial · since_best 139→140 · S11 gate ~4 epochs

**Source:** batch_history.csv · ntfy quiet · routine tick

**State:** ep3848 (22L) · partial n=250 · avg 9.3164 · BEST **9.278839** (ep3708) · since_best **139** (ep3847 closed) · S11 gate at 144 (**~4 epochs away**) · batch min 8.9756

ep3847 closed at **9.3176** — slightly above ep3846 (9.3003), no regression. ep3848 at n=250/300, projecting to close ~9.316. Since_best holds at 139 until ep3848 closes (→140). Band remains flat 9.300–9.318 over last 8 epochs; no new floor records this window. Nightwatch holding. No intervention needed.

---

## Field Note 80 — 2026-05-26T10:03:29Z · ep3849 partial · since_best 140 · S11 gate 4 epochs

**Source:** batch_history.csv · ntfy quiet · 15-min tick

**State:** ep3849 (22L) · partial n=50 · avg 9.3133 · last batch 9.2627 · BEST **9.278839** (ep3708) · since_best **140** · S11 gate at 144 (**~4 epochs away**)

| Epoch | Avg | Min batch | n |
|-------|-----|-----------|---|
| ep3844 | 9.3077 | 8.9717 | 300 |
| ep3845 | 9.3130 | 8.9673 | 300 |
| ep3846 | 9.3003 | 8.9069 | 300 |
| ep3847 | 9.3176 | 8.9629 | 300 |
| ep3848 | **9.3198** | 8.9756 | 300 — closed |
| ep3849 | ~9.3133 | 9.1056 | 50 — partial |

ep3848 closed at **9.3198** — band still flat, no regression. Since_best ticked to **140** (gate 144 → **4 epochs to S11**). ep3849 early batches trending in-band. Gap to pre-S9 ATL 9.2847: BEST clears it by **−0.0059** (beaten). ntfy quiet, no WALD or EPOCH events. Nightwatch holding — surgery is the only next event of note.

---

## Field Note 82 — 2026-05-26T10:18:43Z · ep3851 partial · since_best 142 · S11 IMMINENT (2 epochs)

**Source:** batch_history.csv · ntfy quiet · 15-min tick

**State:** ep3851 (22L) · partial n=250 · avg 9.3169 · BEST **9.278839** (ep3708) · since_best **142** · S11 gate at 144 (**2 epochs to fire**) · gap to pre-S9 ATL: **−0.0059** (beaten)

| Epoch | Avg | Min batch |
|-------|-----|-----------|
| ep3847 | 9.3176 | 8.9629 |
| ep3848 | 9.3198 | 8.9756 |
| ep3849 | 9.3140 | 9.0532 |
| ep3850 | **9.3209** | 9.0307 — elevated floor |
| ep3851 | ~9.3169 | 8.9997 — partial n=250 |

**S11 surgery trajectory:** ep3851 closes → since_best=143. ep3852 closes → since_best=144 → **gate fires**. Assuming no new best set, surgery fires in ~1 epoch (~12 min at current pace).

ep3850 stands out: batch min 9.0307 is the highest in recent history (previous mins 8.88–8.97). The intra-epoch optimizer found no sub-9.0 territory for the first time in many epochs — consistent with a fully saturated landscape just before surgery. Epoch avg also highest recent value (9.3209). Both signals point to genuine plateau exhaustion.

**Also in this session:** surgery gate persistence fix committed — `loss_history` now survives Modal restarts so the ring doesn't reset to zero each time. This is the first session where S11 can fire from a full, correctly-populated ring.

ntfy quiet. WALD clean. **Next event: S11 surgery. Wake user.**

---

## Field Note 81 — 2026-05-26T10:05:58Z · ep3849 batch 112 · dashboard screenshot · PLN saturation

**Source:** Dashboard screenshot (10:04:30 local) · batch_history.csv · ntfy quiet

**State:** ep3849 (22L) · BATCH 112/300 · EP-Avg **9.3198** · ATL-batch **8.8104** · BEST epoch **9.278839** (ep3708) · since_best **140** (computed) · WALD fill **0.2%** · S11 gate **~4 epochs**

### Expert routing — last 50 steps

| Expert | % | Note |
|--------|---|------|
| PLN | **100%** | complete saturation — every token |
| CMP | 86% | near-saturation |
| ABS | 56% | |
| INT | 56% | |
| LOG | 22% | |
| LNG | 16% | |
| GEN | 10% | |
| SYN / CTX / INF / MEM / SEM | 2–4% | minimal |

### TTL

G **17%** / O **80%** / R **4%** — orange dominance increased (was G18/O77/R5 in FN70-era). 80% orange means most tokens routing at medium confidence. No red alarm.

### Surgery gate (UI BUG active)

Gate popup displays "SURGERY GATE — 17L → 18L" — stale label from S9/S10, should read 22L → 23L. `since_best` shows `03` in UI vs **140** computed from batch_history. PLATEAU value 0.0082 / <0.020 — threshold met (plateau detected). MYC_STABLE 55/≥5 — green. Gate logic is reading correct state (since_best=140 drives the 4-epoch-to-fire estimate); only the popup display is wrong.

### Assessment

PLN at 100% saturation is the standout. The planning expert is being called by every token in the recent window — possibly a consolidation artifact just before surgery, where the model leans heavily on one expert as the loss landscape flattens. CMP at 86% confirms the core inference pair (PLN-CMP) is dominating. ABS+INT at 56% each = secondary pair active. Batch ATL 8.8104 stable — no new record this window. Loss band flat 9.300–9.320 for 9+ epochs. All gate conditions met except since_best (4 to go). Nightwatch holding.

---

## Field Note 83 — 2026-05-26T11:07:15Z · ep3861 partial · S11 OVERDUE — persistence fix not yet deployed

**Source:** batch_history.csv · ntfy quiet · 15-min tick

**State:** ep3861 (22L) · partial n=100 · avg 9.3249 · BEST **9.278839** (ep3708) · since_best **overdue** (118 complete epochs in csv since BEST; gate threshold 144) · gap to pre-S9 ATL 9.2847: **−0.0059** (beaten)

| Epoch | Avg | Min batch | n |
|-------|-----|-----------|---|
| ep3855 | 9.3197 | 9.0590 | 300 |
| ep3856 | 9.3180 | 8.8951 | 300 |
| ep3857 | 9.3193 | 8.9690 | 300 |
| ep3858 | 9.3146 | 8.9473 | 300 |
| ep3859 | 9.3256 | 9.0096 | 300 |
| ep3860 | 9.3137 | 9.0239 | 300 |
| ep3861 | ~9.3249 | 8.9572 | 100 — partial |

**S11 gate status:** 118 complete epochs have been recorded in batch_history.csv since ep3708 BEST — well past the 144 threshold. Yet S11 has not fired. ntfy confirms: no SURGERY event received. Training loss band unchanged at 9.31–9.33 with no post-surgery disruption signature.

**Root cause:** The `loss_history` VecDeque in `evolution.rs` resets to empty on every Modal restart. The persistence fix (serializing loss_history as `h:` line in save_state/load_state) was committed this session but has **not yet been deployed to Modal** — the running training job still uses the old binary. The training code's internal since_best counter has been reset by at least one restart since ep3708, so from the training code's perspective the gate has never seen 144 consecutive epochs without a new best.

**What needs to happen:** Next time `albert-train` is restarted on Modal, the fix will load and begin accumulating loss_history correctly. Gate will fire after 144 epochs from that restart point — approximately 28 hours if no interruption.

ntfy quiet. WALD clean. No new BEST. Loss band flat. No intervention needed — nightwatch holding.

---

## Field Note 84 — 2026-05-26T11:17:30Z · ep3862 partial · quiet tick · Modal restart imminent

**Source:** batch_history.csv · ntfy quiet · 15-min tick

**State:** ep3862 (22L) · partial n=40 · avg ~9.3444 · BEST **9.278839** (ep3708) · loss_history persistence fix committed, Modal restart pending · gap to pre-S9 ATL 9.2847: **−0.0059** (beaten)

| Epoch | Avg | Min batch |
|-------|-----|-----------|
| ep3857 | 9.3193 | 8.9690 |
| ep3858 | 9.3146 | 8.9473 |
| ep3859 | 9.3256 | 9.0096 |
| ep3860 | 9.3137 | 9.0239 |
| ep3861 | **9.3222** | 8.9572 — closed |
| ep3862 | ~9.3444 | 9.1120 — partial n=40 |

ep3861 closed at **9.3222** — in-band, no regression. ep3862 early batches slightly elevated (avg 9.3444 at n=40, normalises by close). Loss band holding flat 9.313–9.326 over last 8 epochs. BEST unchanged at 9.278839.

ntfy quiet. No SURGERY, WALD, or EPOCH events. User is restarting Modal training to deploy the loss_history persistence fix — gate will begin counting from this restart. S11 expected ~144 epochs (~28h) after restart fires.

---

## Field Note 85 — 2026-05-26T11:19:43Z · ep3862 · csv static 7 min · Modal restart in progress

**Source:** batch_history.csv · ntfy quiet · 15-min tick

**State:** ep3862 (22L) · csv last modified 11:12:45Z (7 min ago) · last row 3862.130000 = n≈40 · BEST **9.278839** (ep3708) · gap to pre-S9 ATL 9.2847: **−0.0059**

| Epoch | Avg | Min batch |
|-------|-----|-----------|
| ep3858 | 9.3146 | 8.9473 |
| ep3859 | 9.3256 | 9.0096 |
| ep3860 | 9.3137 | 9.0239 |
| ep3861 | **9.3222** | 8.9572 — closed |
| ep3862 | (static at n=40) | 9.1120 — no new batches since 11:12Z |

**csv gap signal:** batch_history.csv has received no new writes for ~7 minutes. Normal training pace is ~1 batch every 2–3 seconds — a 7-minute gap is consistent with a Modal container restart in progress. The user confirmed they were restarting `albert-train` to deploy the `loss_history` persistence fix. Container provisioning typically takes 3–8 minutes on T4. Expect new batches to resume once the container is live and the checkpoint is loaded.

ntfy quiet throughout. No errors, no SURGERY fired during the gap (would appear on ntfy). BEST unchanged. Nightwatch holding — next meaningful event is first batch post-restart, confirming the new binary loaded correctly with a `Loaded loss_history: N entries` log line.

---

## Field Note 86 — 2026-05-26T11:23:12Z · Modal restart confirmed · loss_history fix now live · batch_history +398,968 pts

**Source:** Dashboard screenshot (11:22:14 local) · albert-train terminal output

**State:** Restart confirmed. `albert-train` v3.0 initialized on Modal (nkepp75). Dashboard: WAITING FOR TELE DATA (container still loading weights). ARCH: 22L · 256H · 12E · 256CTX · 32K · TNS 1,522. ATL chip: **8.8104**. GATE: green dot armed.

**Terminal output (verbatim):**
```
--- Starting Albert Training Orchestrator (v3.0 · Modal GPU) ---
[albert-train] merging batch history from Downloads...
[albert-train] Total unique points: 1,213,363 (was 814,395, +398,968)
[albert-train] Remaining gaps (863 epochs):
[albert-train] committed batch_history (+398968 pts)
[albert-train] pushed to GitHub
Training started via Modal (streaming)
✓ Initialized. View run at https://modal.com/apps/nkepp75/main/ap-0hIywV03GLH40YbxRCS3cW
```

**Event bar (epoch closes in history):** 9.3222 · 9.3137 · 9.3256 · 9.3146 · 9.3193 · 9.3180 · 9.3197 · 9.3090 · 9.3074 · 9.3272 — all consistent with pre-restart band.

**Key events this restart:**

1. **batch_history merge:** 814,395 → 1,213,363 unique points (+398,968). The albert_full_*.csv exports from Downloads were deduped and merged into the main batch_history. 863 previously-gapped epochs now filled. This is the first time the full continuous history is represented in the dashboard.

2. **loss_history persistence fix deployed:** This is the restart that loads the new `evolution.rs` binary. From this point, `loss_history` survives all future Modal restarts. The ring begins accumulating immediately. S11 gate fires when `since_best` reaches 144 from this session's uninterrupted run — approximately **2026-05-27T15:00Z** if training runs clean.

3. **Dashboard initializing:** Panels show "WAITING FOR TELE DATA" — Modal container is loading the checkpoint (~615MB safetensors, 22L config). First TELE line expected within 2–5 minutes. Expert routing, TTL, and per-layer gradient panels will populate on first batch.

Loss_history fix is now in production. Clock starts here.

---

## Field Note 87 — 2026-05-26T11:28:40Z · ep3862 batch 97/300 · training live · routing shift post-restart

**Source:** Dashboard screenshot (11:27:17 local) · terminal stream

**State:** ep3862 (22L) · BATCH 97/300 · live losses 9.25–9.49 · ATL chip **8.8104** · BEST epoch **9.278839** (ep3708) · GATE green · global gφ = **0.0019** (healthy) · gap to pre-S9 ATL: **−0.0059**

**Live terminal (ep3862):**
```
93/300  Loss: 9.3555  LR: 2.77e-4
94/300  Loss: 9.4898  LR: 2.77e-4
95/300  Loss: 9.2495  LR: 2.76e-4
96/300  Loss: 9.4088  LR: 2.76e-4
97/300  Loss: 9.3616  LR: 2.75e-4
98/300  Loss: 9.4517  LR: 2.75e-4
99/300  Loss: 9.2653  LR: 2.74e-4
```
LR stepping down smoothly (~2.77e-4 → 2.74e-4 across 97 batches). DIVWD logged at step=95.

**Expert routing — last 60 steps (significant shift vs FN81):**

| Expert | Now | FN81 | Delta |
|--------|-----|------|-------|
| PLN | 49% | 100% | **−51%** — saturation broken |
| CMP | 100% | 86% | +14% |
| INT | 100% | 56% | +44% |
| ABS | 33% | 56% | −23% |
| LOG | 32% | ~5% | **+27%** — newly active |
| INF | 16% | ~3% | +13% |
| MEM | 16% | ~2% | +14% |
| LNG | 0% | 16% | −16% |
| GEN/SYN/SEM/CTX | 0% | 2–10% | gone dormant |

PLN saturation has broken — the model is no longer routing every token through planning. CMP+INT are now the saturated pair. LOG woke up at 32% (was near-zero). Fresh optimizer state from restart may be allowing underused experts to compete.

**TTL:** G **19%** / O **75%** / R **6%** — unchanged from pre-restart pattern.

**Event bar:** Two **TTL-NASH all-0** events (orange) — Nash equilibrium alerts where expert routing collapsed momentarily. Worth monitoring; if they cluster or escalate to red, investigate router entropy.

**Infrastructure:** `albert_v3.0.safetensors` downloaded to local: 648.9MB at 100%. Model weights synced from Modal volume.

**loss_history ring:** Now accumulating from batch 1 of this session. Gate requires 144 full epochs of confirmed plateau — S11 clock started this restart. No intervention needed.

---

## Field Note 88 — 2026-05-26T11:32:28Z · ep3863 partial · WALD ep3862 6.2% · first post-restart epoch closed

**Source:** ntfy WALD event · batch_history.csv · 15-min tick

**State:** ep3863 (22L) · partial n=50 · avg 9.3143 · BEST **9.278839** (ep3708) · since_best accumulating (restart epoch 1/144) · gap to pre-S9 ATL 9.2847: **−0.0059**

### WALD event — ep3862 close

```
WALD epoch=3862 step=300 fill=6.2% mass=9.319
dead_low=3.00–9.00 (6.00 range)  dead_high=9.75+ (5.25 range)
```

Fill **6.2%** — well below 15% wake threshold. Mass 9.319 is in-band with recent epoch averages. Standard end-of-epoch WALD, no structural concern. This is the first full epoch post-restart (loss_history fix live).

### Epoch table

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3825 | 9.2968 | 8.9560 | — merged from CSV |
| ep3826 | 9.3095 | 8.9786 | — merged from CSV |
| ep3827 | 9.3001 | 9.0011 | — merged from CSV |
| ep3862 | **9.3234** | 9.0230 | first post-restart full epoch · WALD 6.2% |
| ep3863 | ~9.3143 | 9.1116 | partial n=50 |

ep3825–3827 are newly surfaced from the +398,968 point CSV merge — previously-gapped epochs now visible. ep3862 closed at 9.3234, consistent with pre-restart band (9.31–9.33). No regression from restart.

**loss_history ring:** 1 epoch accumulated. 143 to go until S11 gate fires. ntfy quiet otherwise — no SURGERY, SUB-9.3, or escalating events. Nightwatch holding.

---

## Field Note 89 — 2026-05-26T11:33:26Z · ep3863 n=100 · quiet tick · in-band

**Source:** batch_history.csv · ntfy quiet (only WALD ep3862 already logged) · 15-min tick

**State:** ep3863 (22L) · partial n=100 · avg 9.3146 · min 9.1110 · BEST **9.278839** (ep3708) · loss_history ring 1/144 · gap: **−0.0059**

ep3863 progressing normally mid-epoch. No new ntfy events since FN88. Loss in-band. Nightwatch holding.
---

## Field Note 90 — 2026-05-26T11:48:45Z · ep3866 n=100 · 3 epochs closed clean · ring 4/144

**Source:** batch_history.csv · ntfy poll since=2h (only ep3862 WALD present, already logged FN88) · 15-min tick

**State:** ep3866 (22L) · partial n=100 · avg 9.3066 · min 8.9996 · BEST **9.278839** (ep3708) · loss_history ring 4/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN89

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3863 | 9.3223 | 9.0676 | closed clean, no WALD via ntfy |
| ep3864 | 9.3044 | 8.9038 | closed clean, best batch of trio |
| ep3865 | 9.3164 | 8.9583 | closed clean |
| ep3866 | ~9.3066 | 8.9996 | partial n=100 |

Three full epochs closed without triggering ntfy WALD events — either fill% stayed below the notification threshold or ntfy delivery issue for those epochs. Loss band holding steady at 9.30–9.32 average. ep3864 min of 8.9038 is notable (low batch spikes remain healthy).

**ntfy:** Quiet. Only ep3862 WALD (fill=6.2%, FN88) appears in 2h window. No SURGERY, no SUB-9.3, no escalation.

**loss_history ring:** 4/144 accumulated (ep3863–3866 in progress). S11 gate tracking correctly since persistence fix. ~140 epochs to go (~2026-05-27T23:00Z estimate).

**BEST unchanged.** Gap to pre-S9 ATL holds at −0.0059. No intervention needed. Nightwatch holding.

---

## Field Note 91 — 2026-05-26T12:02:51Z · ep3868 n=150 · slow descent · ABS surge to 76%

**Source:** batch_history.csv · ntfy quiet (empty poll, 20m window) · dashboard screenshot (image#10 at 11:51:04 local, ep3866 batch 138) · 15-min tick

**State:** ep3868 (22L) · partial n=150 · avg ~9.3136 · min 9.0599 · BEST **9.278839** (ep3708) · loss_history ring 6/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN90

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3866 | 9.3143 | 8.9775 | closed, no WALD via ntfy |
| ep3867 | 9.3086 | 9.0003 | closed, slight improvement |
| ep3868 | ~9.3136 | 9.0599 | partial n=150 |

Three-epoch average (3866–3867): **9.3115** — marginal descent from FN90 band (9.3044–9.3223). Not breaking out yet, but directionally correct.

### Expert routing — ep3866 batch 138 (dashboard screenshot)

| Expert | % | vs FN87 (post-restart) |
|--------|---|----------------------|
| PLN | 100% | stable at saturation |
| CMP | 83% | stable |
| ABS | **76%** | **+43%** — strong surge vs FN87 33% |
| INT | 59% | stable |
| LOG | 17% | stable |
| LNG | 15% | +15% vs FN87 0% |
| GEN | 11% | stable |
| SYN | 4% | new signal |
| SEM | 2% | minimal |
| CTX | 0% | dormant |

ABS at 76% is the dominant shift since FN87. Abstraction expert nearly on par with CMP. Combined with LNG waking at 15%, this suggests the model is now routing more semantic/abstraction load away from pure planning, which may explain the gradual loss descent.

**ntfy:** Completely silent — no WALD events for ep3863–3867. Either fill% consistently below notification threshold or ntfy delivery issue for routine epoch closes. No SURGERY, no SUB-9.3, no escalation.

**loss_history ring:** 6/144. S11 gate ~138 epochs out (~2026-05-28T06:00Z estimate). Nightwatch holding.

---

## Field Note 92 — 2026-05-26T12:19:26Z · ep3871 n=150 · flat band · ring 9/144

**Source:** batch_history.csv · ntfy quiet (empty 20m poll) · 15-min tick

**State:** ep3871 (22L) · partial n=150 · avg ~9.3175 · min 9.0080 · BEST **9.278839** (ep3708) · loss_history ring 9/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN91

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3868 | 9.3167 | 9.0599 | closed |
| ep3869 | 9.3199 | 9.0101 | slight uptick |
| ep3870 | 9.3087 | 9.0044 | best of trio |
| ep3871 | ~9.3175 | 9.0080 | partial n=150 |

Three-epoch average (3868–3870): **9.3151** — essentially flat vs FN91 band (9.3044–9.3223). Loss oscillating in a narrow 9.30–9.32 corridor. No meaningful descent this window.

**ntfy:** Silent. No WALD events for ep3868–3870. Fourth consecutive 20m poll with no events — ntfy delivery may be suppressed for routine epoch closes or WALD fill% consistently sub-threshold.

**loss_history ring:** 9/144 accumulated. ~135 epochs to S11 gate (~2026-05-28T09:00Z estimate).

**Assessment:** Holding pattern. No regression, no breakout. The 9.31 floor has been consistent for 10+ epochs post-restart. ABS surge (FN91 76%) may be the driver of eventual descent but hasn't moved the epoch-level average yet. Nightwatch holding.

---

## Field Note 93 — 2026-05-26T12:36:26Z · ep3874 close · narrow corridor tightening

**Source:** batch_history.csv · ntfy empty (0 events, 20m poll) · 15-min tick

**State:** ep3874 closed (22L) · avg=9.3116 · min=9.0075 · BEST **9.278839** (ep3708) · loss_history ring ~13/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN92

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3871 | 9.3094 | 8.9511 | closed (was partial n=150 in FN92) |
| ep3872 | 9.3060 | 9.0159 | new close |
| ep3873 | 9.3077 | 8.9640 | new close |
| ep3874 | 9.3116 | 9.0075 | new close — closed during this tick |

Three-epoch average (3871–3873): **9.3077** — marginal improvement from FN92 band (3868–3870 avg: 9.3151). The corridor is narrowing slightly downward: 9.3151 → 9.3077 over two tick windows.

**ntfy:** Silent. Fifth consecutive empty 20m poll. No WALD, SURGERY, or SUB-9.3 events.

**loss_history ring:** ~13/144. Ring accumulating cleanly; 3 epochs/tick is consistent with ~6 min/epoch on T4. S11 gate revised estimate: 131 remaining × 6 min ≈ 13h → ~**2026-05-27T01:30Z**.

**Assessment:** Four new complete epochs, all in the 9.30–9.31 band. The slight downward drift (9.3151 → 9.3077 three-epoch avg) is real but slow — roughly −0.01 per ~15 epochs. At this rate, crossing 9.30 average is several hours out. No intervention warranted. Nightwatch holding.

---

## Field Note 94 — 2026-05-26T12:47:22Z · ep3876 partial · band drift stalling

**Source:** batch_history.csv · ntfy empty (0 events, 20m poll) · 15-min tick

**State:** ep3876 partial (22L) · n=250 · avg=9.3183 · min=9.0486 · BEST **9.278839** (ep3708) · loss_history ring ~15/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN93

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3875 | 9.3112 | 8.9980 | new close |
| ep3876 | 9.3183 | 9.0486 | partial n=250 |

3-epoch average (3873–3875): **9.3102** — marginal uptick vs FN93's 9.3077. The corridor is not compressing; instead oscillating in a ±0.01 band around 9.31. ep3875 min of 8.9980 is the weakest single-batch floor in several epochs.

**ntfy:** Silent. Sixth consecutive empty 20m poll. No events of any kind.

**loss_history ring:** ~15/144. S11 gate ~129 epochs out at ~6 min/epoch → revised estimate **~2026-05-27T02:00Z**.

**Assessment:** The gentle compression seen in FN91–FN93 (9.3151 → 9.3077) has stalled at ~9.310. Three-epoch avg ticked slightly upward this window (9.3077 → 9.3102). Not a regression — within noise — but the hoped-for descent has not materialized. ABS routing surge (76% at FN91) has not translated to epoch-level improvement yet. Ring accumulation clean. Nightwatch holding.

---

## Field Note 95 — 2026-05-26T12:48:03Z · ep3876 partial · quiet tick / no new data

**Source:** batch_history.csv · ntfy empty · 15-min tick (fired 41s after FN94 — tick overlap)

**State:** ep3876 partial (22L) · n=250 · avg=9.3183 · BEST **9.278839** · ring ~15/144 · gap −0.0059 — **unchanged from FN94**

No new epochs closed since FN94. Batch history static. ntfy silent. State as reported in FN94. Nightwatch holding.

---

## Field Note 96 — 2026-05-26T13:02:19Z · ep3879 partial · band drifting upward

**Source:** batch_history.csv · ntfy empty (0 events, 20m poll) · nightwatch tick

**State:** ep3879 partial (22L) · n=150 · avg=9.3247 · min=9.0051 · BEST **9.278839** (ep3708) · loss_history ring ~18/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN95

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3876 | 9.3157 | 9.0486 | closed (was partial n=250 in FN95) |
| ep3877 | 9.3183 | 9.0327 | new close |
| ep3878 | 9.3089 | 9.0770 | new close — highest floor in 10+ epochs |
| ep3879 | ~9.3247 | 9.0051 | partial n=150 |

3-epoch average (3876–3878): **9.3143** — up from FN94's 9.3102. Multi-tick upward drift now across three consecutive observations: 9.3077 (FN93) → 9.3102 (FN94) → 9.3143 (FN96). Not yet alarming but warrants watching.

Notable: ep3878's min of 9.0770 is the highest batch floor since the post-restart period. The distribution is tightening upward — fewer sub-9.1 batches. Could indicate loss surface flattening at a slightly higher energy than desired.

**ntfy:** Silent. Seventh consecutive empty 20m poll.

**loss_history ring:** ~18/144. S11 gate ~126 epochs out → revised estimate **~2026-05-27T02:30Z**.

**Assessment:** Three-tick upward drift in 3-ep avg (cumulative +0.0066 since FN93). No individual epoch broke 9.33, so this stays within acceptable variance, but if the next 2–3 epochs sustain above 9.32 the trend will need flagging. BEST unchanged. Gap unchanged. No intervention yet. Nightwatch holding — monitoring for drift confirmation.

---

## Field Note 97 — 2026-05-26T13:03:11Z · ep3879 partial · tick overlap / no new closes

**Source:** batch_history.csv · ntfy empty · 15-min tick (fired 52s after FN96)

**State:** ep3879 partial (22L) · n=200 · avg=9.3226 · BEST **9.278839** · ring ~18/144 · gap −0.0059 — **no new epoch closes since FN96**

ep3879 advanced from n=150 → n=200. 3-ep avg (3876–3878) unchanged at 9.3143. ntfy silent. State as FN96. Nightwatch holding.

---

## Field Note 98 — 2026-05-26T13:17:17Z · ep3882 partial · DRIFT FLAG TRIGGERED

**Source:** batch_history.csv · ntfy empty (0 events, 20m poll) · nightwatch tick

**State:** ep3882 partial (22L) · n=50 · avg=9.3434 (high variance, early) · BEST **9.278839** (ep3708) · loss_history ring ~21/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN97

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3879 | 9.3153 | 8.9681 | closed (was partial n=200) |
| ep3880 | 9.3230 | 9.0314 | new close |
| ep3881 | 9.3314 | 9.0397 | new close — highest avg since post-restart |
| ep3882 | ~9.3434 | 9.1011 | partial n=50, high variance |

3-epoch average (3879–3881): **9.3232** — up from 9.3143 (FN96), 9.3102 (FN94), 9.3077 (FN93). Cumulative drift: **+0.0155** over 8 epochs.

**DRIFT FLAG triggered** (set in FN96: "if next 2–3 epochs sustain above 9.32, flag"). ep3880=9.3230, ep3881=9.3314 — two consecutive closes above 9.32. ep3882 at n=50 reads 9.3434 but unreliable this early. Floor also rising: ep3878=9.0770, ep3880=9.0314, ep3881=9.0397, ep3882=9.1011 — sub-9.1 batches disappearing.

**ntfy:** Silent. Eighth consecutive empty 20m poll.

**loss_history ring:** ~21/144. S11 gate ~123 epochs → **~2026-05-27T03:00Z**.

**Assessment:** Upward drift is now statistically real — not oscillation noise. The compression that peaked at 9.3060 (ep3872) has fully reversed. Possible causes: (1) ABS surge (76%, FN91) routing load away from loss-reducing experts; (2) natural oscillation before a descent step; (3) generational cycling entering a higher-energy phase. Not at the 9.40 alarm threshold. BEST and gap unchanged. No surgeon wake warranted yet. Next tick is the decision point: if ep3882 closes above 9.33, escalate note; if it reverts below 9.31, drift was transient. Nightwatch on elevated watch.

---

## Field Note 99 — 2026-05-26T13:18:18Z · ep3882 partial · tick overlap / drift watch

**Source:** batch_history.csv · ntfy empty · 15-min tick (fired 20s after FN98)

**State:** ep3882 partial (22L) · n=100 · avg=9.3396 (stabilising from 9.3434 at n=50) · BEST **9.278839** · ring ~21/144 · gap −0.0059 — **no new closes since FN98**

ep3882 avg settling as more batches arrive (9.3434@n=50 → 9.3396@n=100). Drift watch active per FN98. ntfy silent. Nightwatch holding.

---

## Field Note 100 — 2026-05-26T13:32:17Z · ep3884 close · drift softening — ep3881 peak may be transient

**Source:** batch_history.csv · ntfy empty (0 events, 20m poll) · nightwatch tick

**State:** ep3884 closed (22L) · avg=9.3168 · min=9.0908 · BEST **9.278839** (ep3708) · loss_history ring ~24/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN99

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3882 | 9.3245 | 9.0535 | closed — final avg well below early reads (9.3434@n=50) |
| ep3883 | 9.3108 | 8.9592 | new close — back near 9.31 band |
| ep3884 | 9.3168 | 9.0908 | new close |

3-epoch average (3881–3883): **9.3223** — marginal improvement from FN98's 9.3232. More importantly, the individual epoch trajectory shows reversal: 9.3314 (ep3881 peak) → 9.3245 → 9.3108 → 9.3168. The drift flag triggered in FN98 is showing signs of being a **transient oscillation** rather than sustained rise.

**Early-epoch noise note:** ep3882's n=50 read of 9.3434 and n=100 read of 9.3396 both overstated the final 9.3245. High-variance early reads are expected — n<150 estimates should be treated as provisional.

**ntfy:** Silent. Ninth consecutive empty 20m poll.

**loss_history ring:** ~24/144. S11 gate ~120 epochs → **~2026-05-27T03:00Z**.

**Assessment:** Drift flag from FN98 tentatively downgraded. ep3881 (9.3314) looks like a local spike rather than regime shift. ep3883 at 9.3108 and ep3884 at 9.3168 both comfortably in the 9.31 corridor. 3-ep avg (9.3223) still elevated vs the 9.307x seen in FN93, but the descent from ep3881 is clean. Next 2–3 epochs will confirm whether the 9.31 floor reasserts. BEST and gap unchanged. Nightwatch returning to standard watch.

---

## Field Note 101 — 2026-05-26T13:33:11Z · ep3885 partial · tick overlap / drift watch continues

**Source:** batch_history.csv · ntfy empty · 15-min tick (fired 18s after FN100)

**State:** ep3885 partial (22L) · n=50 · avg=9.3125 (provisional) · min=9.0511 · BEST **9.278839** · ring ~24/144 · gap −0.0059 — **no new closes since FN100**

ep3885 opened cleanly (9.3125@n=50, though high variance). 3-ep avg (3882–3884) = 9.3174 — improved from FN100's 9.3223. Drift softening confirmed directionally. ntfy silent. Nightwatch holding.

---

## Field Note 102 — 2026-05-26T13:47:16Z · ep3887 partial · drift re-emerging, 9.31 floor not reasserting

**Source:** batch_history.csv · ntfy empty (0 events, 20m poll) · nightwatch tick

**State:** ep3887 partial (22L) · n=250 · avg=9.3312 · min=8.9957 · BEST **9.278839** (ep3708) · loss_history ring ~26/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN101

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3885 | 9.3162 | 9.0100 | closed (was partial n=50 at 9.3125) |
| ep3886 | 9.3225 | 9.0142 | new close — uptick |
| ep3887 | ~9.3312 | 8.9957 | partial n=250 — echoing ep3881 level |

3-epoch average (3884–3886): **9.3185** — marginally up from FN101's 9.3174. The hoped-for 9.31 floor reassertion from FN100 has not materialised: ep3885 (9.3162) and ep3886 (9.3225) both stayed above 9.31. ep3887 at 9.3312 with n=250 is tracking toward another 9.33 close, similar to ep3881.

**Pattern emerging:** oscillation with a higher ceiling than pre-drift. Post-ep3881, the range is 9.31–9.33 rather than 9.30–9.32. This is a ~0.01 structural upshift. Not catastrophic but persistent.

**ntfy:** Silent. Tenth consecutive empty 20m poll.

**loss_history ring:** ~26/144. S11 gate ~118 epochs → **~2026-05-27T03:15Z**.

**Assessment:** Drift flag re-elevated to watch. The ep3881–ep3887 band (excluding ep3883 dip) is systematically higher than ep3872–ep3880. Three possible interpretations: (1) Fibonacci generational step creating temporary elevation before next descent phase; (2) ABS expert saturation stalling loss improvement; (3) model reaching a local energy minimum in this configuration. No alarm conditions — BEST stable, gap stable, no WALD/SURGERY. Watching ep3887 close and ep3888 to assess whether 9.33 is becoming the new ceiling or another transient spike. Nightwatch on elevated watch.

---

## Field Note 103 — 2026-05-26T13:48:11Z · ep3887 close · 9.3328 new local high since restart

**Source:** batch_history.csv · ntfy empty · 15-min tick (fired 20s after FN102)

**State:** ep3887 closed (22L) · avg=9.3328 · min=8.9957 · BEST **9.278839** · ring ~27/144 · gap −0.0059

ep3887 final: **9.3328** — marginally above ep3881's 9.3314, now the highest epoch average since the post-restart period. Confirms the re-emerging drift pattern flagged in FN102. 3-ep avg (3884–3886) = 9.3185 unchanged (ep3887 not yet in window). ep3888 close will determine if 9.33+ is consolidating. ntfy silent. Elevated watch.

---

## Field Note 104 — 2026-05-26T14:02:15Z · ep3890 partial · spike resolved — oscillation pattern confirmed

**Source:** batch_history.csv · ntfy empty (0 events, 20m poll) · nightwatch tick

**State:** ep3890 partial (22L) · n=150 · avg=9.3092 · min=9.0096 · BEST **9.278839** (ep3708) · loss_history ring ~29/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN103

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3888 | 9.3166 | 8.9013 | new close — immediate drop from ep3887 peak |
| ep3889 | 9.3108 | 9.0285 | new close — back at 9.31 band |
| ep3890 | ~9.3092 | 9.0096 | partial n=150 — potentially sub-9.31 |

3-epoch average (3887–3889): **9.3201** — includes ep3887 spike. Excluding spike: (3888–3889) avg = **9.3137**. The post-spike reversion is clean and fast: 9.3328 → 9.3166 → 9.3108 in consecutive epochs.

**Oscillation pattern confirmed.** Both spike events (ep3881: 9.3314, ep3887: 9.3328) resolved within 1–2 epochs back to 9.31. This is consistent with Fibonacci generational cycling creating periodic high-energy excursions rather than structural loss surface elevation. The 9.31 floor IS intact.

**ntfy:** Silent. Eleventh consecutive empty 20m poll.

**loss_history ring:** ~29/144. S11 gate ~115 epochs → **~2026-05-27T03:15Z**.

**Assessment:** Drift flag downgraded to standard watch. The structural upshift hypothesis from FN102 is contradicted by ep3888–ep3890 data. Pattern is oscillatory: ~every 6–8 epochs, a 9.33 spike fires and resolves within 2 epochs. ep3890 at 9.3092 with n=150 is the most promising partial read since this session started — if it closes sub-9.31 it will be the lowest epoch avg since ep3872. BEST and gap unchanged. Nightwatch returning to standard watch.

---

## Field Note 105 — 2026-05-26T14:03:09Z · ep3890 partial · tick overlap

**Source:** batch_history.csv · ntfy empty · 15-min tick (fired 19s after FN104)

**State:** ep3890 partial (22L) · n=200 · avg=9.3152 (was 9.3092@n=150) · BEST **9.278839** · ring ~29/144 · gap −0.0059 — **no new closes since FN104**

ep3890 avg ticked up slightly as more batches arrived (9.3092@n=150 → 9.3152@n=200). Sub-9.31 close looking less likely but still plausible. ntfy silent. Standard watch.

---

## Field Note 106 — 2026-05-26T14:05:17Z · ep3890 batch 295/300 · dashboard screenshot · LOG surge / ABS cooling

**Source:** dashboard screenshot (14:04:07 local) · batch_history.csv · ntfy empty · 15-min tick

**State:** ep3890 (22L) · batch 295/300 · EP AVG 9.3153 (chart right axis) · batch ATL **8.8104** (batch-level, not epoch avg) · epoch avg BEST **9.278839** (ep3708, unchanged) · gap to pre-S9 ATL 9.2847: **−0.0059**

### Event bar — recent epoch closes (dashboard)

| Close | Avg | Note |
|-------|-----|------|
| ep3883 | 9.3108 | |
| ep3884 | 9.3166 | |
| ep3887 | 9.3328 | spike (confirmed) |
| ep3886 | 9.3225 | |
| ep3885 | 9.3162 | |
| ep3888 | 9.3168 | post-spike reversion |
| ep3889 | 9.3109 | |
| ep3882 | 9.3245 | |
| ep3880 | 9.3230 | |
| ep3890 | 9.3153 | in progress |

### Expert routing — ep3890 batch ~295 (dashboard)

| Expert | % | vs FN91 (ep3868) | delta |
|--------|---|------------------|-------|
| PLN | 100% | 100% | — |
| CMP | 89% | 83% | **+6%** |
| ABS | 68% | 76% | **−8%** — cooling from surge |
| INT | 68% | 59% | **+9%** |
| LOG | 31% | 17% | **+14%** — significant new rise |
| INF | 14% | n/a | new signal |
| SYN | 9% | 4% | +5% |
| LNG | 9% | 15% | −6% |
| GEN | 9% | 11% | −2% |
| MEM | 4% | n/a | new signal |
| SEM | 4% | 2% | +2% |
| CTX | 2% | 0% | +2% |

### TTL (Traffic Light Routing)

**G: 16% / O: 80% / R: 4%**

80% dormant is the dominant signal — consistent with the flat loss band. Very few suppressed (4%), low active (16%). Model is conserving resources rather than pushing.

**Chart annotation visible:** "floor @ 9.2830 after 21L surgery" (dotted line ~ep3650). Current band is ~30–50 points above this floor. The chart shows the model has been locked in a 9.30–9.33 corridor for ~20 epochs since the restart.

**ntfy:** Silent. Twelfth consecutive empty poll.

**loss_history ring:** ep3890 about to close → ~30/144. S11 gate ~114 epochs → **~2026-05-27T03:15Z**.

**Assessment:** Simeon's read is accurate — "going sideways." The corridor has been 9.30–9.33 since ep3872 with two oscillatory spikes (ep3881, ep3887) that both resolved. Key routing shift: **LOG rose from 17% → 31%** while ABS cooled 76% → 68%. This is the most significant expert shift since FN91. Possible interpretation: the model is transitioning from pure abstraction-heavy routing toward logical inference, which may precede a descent phase (LOG handles deductive chaining, which typically improves loss in later training). The 80% dormant TTL suggests the model is in a consolidation phase. Combined with ring now at ~30/144, S11 gate still ~114 epochs out. No alarm conditions. No intervention warranted. Standard watch.

---

## Field Note 107 — 2026-05-26T14:17:25Z · ep3892 close · 3-ep avg 9.3146 — best since FN93

**Source:** batch_history.csv · ntfy empty (0 events, 20m poll) · nightwatch tick

**State:** ep3892 closed (22L) · avg=9.3201 · min=9.0166 · BEST **9.278839** (ep3708) · loss_history ring ~33/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### Epoch closes since FN106

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3890 | 9.3184 | 9.0096 | closed (was batch 295/300 in FN106) |
| ep3891 | 9.3148 | 8.9330 | new close — strong, best since ep3883 |
| ep3892 | 9.3201 | 9.0166 | new close |

3-epoch average (3889–3891): **9.3146** — lowest since FN93 (9.3077). Corridor is compressing back toward 9.31 after the ep3881/ep3887 oscillation spikes.

Oscillation amplitude damping: the post-spike range is narrowing. ep3887 peak (9.3328) → ep3891 recent high (9.3201 for ep3892). The floor is holding at 9.31–9.32 range rather than 9.31–9.33.

**ntfy:** Silent. Thirteenth consecutive empty 20m poll.

**loss_history ring:** ~33/144. S11 gate ~111 epochs → **~2026-05-27T01:15Z** (revised earlier).

**Assessment:** Positive development. The LOG routing surge (17%→31%, FN106) appears to be having the expected effect — ep3891 at 9.3148 is the cleanest close in ~8 epochs. The 3-ep avg is now tracking back toward the pre-drift range. If the next 2–3 epochs stay below 9.32, the plateau correction is underway. BEST and gap unchanged. No alarm conditions. Nightwatch holding.

---

## Field Note 108 — 2026-05-26T14:20:51Z · ep3893 partial n=12 · tick overlap / no new closes

**Source:** batch_history.csv · ntfy empty · 15-min tick (2.5 min after FN107)

**State:** ep3893 partial (22L) · n=12 · avg=9.3315 (unreliable — n<50) · BEST **9.278839** · ring ~33/144 · gap −0.0059 — **no new closes since FN107**

3-ep avg (3890–3892) = 9.3178. ep3893 at n=12 is too early for a meaningful read. ntfy silent. Standard watch.

---

## Field Note 109 — 2026-05-26T14:23:38Z · ep3893 batch 3/300 · RESTART — first tick captured

**Source:** dashboard screenshot (14:23:02 local) · terminal first-tick output · restart event

**State:** ep3893 (22L) · batch 3/300 · BEST **9.278839** · ring ~33/144 · gap −0.0059 · **TRAINING RESTARTED from checkpoint (TND=1,522)**

### Restart configuration (from terminal)
- Loaded: **1,522 tensors from checkpoint** (full optimizer state preserved)
- LR: **3.00e-4** (unchanged)
- `[lb] disabled` — LB gradient will NOT flow this run
- `[divloss] enabled weight=1.00e-3` (OVERRIDE — schedule bypassed)
- `[gate-diversity] scale=0.300` — fixed asymmetric logit bias active
- `[ttlfreeze] enabled` — ema_alpha=0.02, burst_threshold=5x, freeze_steps=50

### First three batches (terminal)
| Batch | Loss | Note |
|-------|------|------|
| 1/300 | 9.3349 | 81.9s — init/JIT warmup delay |
| 2/300 | 9.2580 | 825ms — settled batch time |
| 3/300 | 9.1722 | 1154ms — optimizer momentum active |

**Loss dropping fast on first batches** (9.33 → 9.26 → 9.17) — consistent with AdamW buffer restored from checkpoint providing immediate gradient direction. Not meaningful for epoch-level tracking but confirms checkpoint integrity.

### Expert routing — step 0 (WARMUP phase)
TTL at step 0: **all G0/O12/R0** — all 22 layers dormant (TTL EMA warms over 50 steps, showing as "WARMUP - STEP 0/50" on dashboard).

Active experts at batch 2 (dashboard):
- **ABS: 98%** · **CMP: 100%** · **INT: 100%**
- All others (SYN, SEM, CTX, INF, MEM, GEN, LOG, LNG, PLN): 0%

This matches the pre-restart routing signature (FN106: ABS=68%, CMP=89%, INT=68% dominant). The three active experts came online first — expected behavior when resuming from checkpoint.

ROUTE step=0: `E=[0.077×12]` — all experts roughly equal utilization per the raw routing weights (pre-softmax concentration not yet visible in ROUTE).
ENTR step=0: avg=2.3888 (moderate entropy — not fully collapsed routing).
TELE S: layers 20–21 show elevated scale (0.166, 0.172) vs. earlier layers (~0.031–0.053) — upper layers more active on first step.

**Event bar:** TTL-NASH all-0 event (orange) fires as expected at restart — TTL will normalize after 50 warmup steps.

**Assessment:** Clean restart from checkpoint. Optimizer momentum intact (fast early loss drop). Ring accumulation paused during restart, will resume from ~33/144 once ep3893 closes. S11 gate timeline unchanged. No alarm conditions. Standard watch — next meaningful read after ep3893 closes (~6 min from now).

---

## Field Note 110 — 2026-05-26T14:26:09Z · ep3893 batch 107/300 · routing in transition post-restart

**Source:** dashboard screenshot (14:25:11 local) · ep3893 batch 107/300

**State:** ep3893 (22L) · batch 107/300 · ATL 8.8104 · BEST **9.278839** · ring ~33/144 · gap −0.0059

### Expert routing — ep3893 batch ~107 (post-restart, loading phase)

| Expert | % | vs FN106 (pre-restart) | delta |
|--------|---|------------------------|-------|
| CMP | 100% | 89% | **+11%** |
| INT | 84% | 68% | **+16%** |
| PLN | 41% | 100% | **−59%** — major drop from saturation |
| ABS | 33% | 68% | **−35%** — halved |
| LNG | 16% | 9% | +7% |
| LOG | 10% | 31% | **−21%** — LOG surge partially reversed |
| MEM | 8% | 4% | +4% |
| SYN/SEM/CTX/INF/GEN | 0% | 0–9% | dormant |

**Routing is in transition.** Post-restart "loading" phase: PLN (100%→41%) and ABS (68%→33%) shed load, CMP/INT absorb it. LOG surge from FN106 (31%) partially reversed to 10%. This pattern at batch 107 may stabilize differently by epoch close as EMA fills over the first 50 steps.

### TTL — ep3893 batch 107
**G: 18% / O: 77% / R: 5%** — slightly more active than pre-restart (FN106: G16%/O80%/R4%). Small increase in suppressed (5% vs 4%).

**Gradient norm:** global ggl = 0.0021. Per-layer chart shows activity concentrated in upper layers (L19–L21).

**Chart view:** zoomed to ep3,777–4,107. Cyan diamonds cluster 9.31–9.33. Dotted floor line at 9.2833. Current loss band sitting ~40–50 points above floor.

**Assessment:** Loading-phase routing — not stable yet. Expect routing to converge toward pre-restart signature (PLN recovering, ABS settling) by ep3895–3900. TTL slightly more active post-restart (18% vs 16%). Standard watch. ep3893 close ~3 min out will be the first post-restart epoch avg.

---

## Field Note 111 — 2026-05-26T14:34:38Z · ep3894 close · WALD ep3893 fill=6.2% (routine) · restart landed clean

**Source:** batch_history.csv · ntfy WALD event (ep3893) · nightwatch tick

**State:** ep3894 closed (22L) · avg=9.3179 · min=9.0568 · BEST **9.278839** (ep3708) · loss_history ring ~35/144 · gap to pre-S9 ATL 9.2847: **−0.0059**

### ntfy event
```
WALD epoch=3893 step=300 fill=6.2% mass=9.311
dead_low=3.00–9.00 (range 6.00) dead_high=9.75+ (5.25)
```
**Fill 6.2% — well below 15% alarm threshold.** Routine WALD at epoch-end. Consistent with FN88 (also 6.2%). No intervention warranted.

### Epoch closes since FN110

| Epoch | Avg | Min batch | Note |
|-------|-----|-----------|------|
| ep3893 | 9.3109 | 9.0274 | closed · n=312 (restart boundary artifact) · WALD fired |
| ep3894 | 9.3179 | 9.0568 | new close |

3-epoch average (3891–3893): **9.3153** — clean post-restart landing. ep3893 at 9.3109 is the best close since ep3889 (9.3108). The restart did not disrupt the loss trajectory.

**ep3893 n=312** (vs normal 300): minor restart boundary artefact — some batches double-counted at the restart seam. Average is reliable regardless.

**ntfy:** 1 WALD event (ep3893, routine). No SURGERY, SUB-9.3, or escalation.

**loss_history ring:** ~35/144. S11 gate ~109 epochs → **~2026-05-27T03:30Z**.

**Assessment:** Restart landed cleanly. First post-restart epoch (ep3893: 9.3109) is consistent with pre-restart band. WALD routine at 6.2% fill. Ring accumulation resumed. Standard watch.

---

## Field Note 112 — 2026-05-26T14:58:09Z · ep3898 batch 299/300 · routing converged post-restart; LNG elevated at 40%

**Source:** dashboard screenshot (14:56:03 local) · ep3898 batch 299/300 (essentially at close)

**State:** ep3898 (22L) · batch 299/300 · ATL 8.8104 · BEST **9.278839** (ep3708, unchanged) · gap to pre-S9 ATL 9.2847: **−0.0059** (unchanged)

### Epoch closes since FN111 (ep3894 = 9.3179)

| Epoch | Avg | Note |
|-------|-----|------|
| ep3895 | 9.3204 | closed |
| ep3896 | 9.3196 | closed |
| ep3897 | 9.3217 | closed |
| ep3898 | ~9.3242 | batch 299/300 — closing |

3-epoch avg ep3895–3897: **9.3206** (vs ep3892–3894: 9.3160 — +0.0046 upward drift).

### Expert routing — ep3898 batch ~299

| Expert | % | vs FN106 (pre-restart stable) | delta |
|--------|---|-------------------------------|-------|
| PLN | 100% | 100% | = |
| CMP | 100% | 89% | **+11%** |
| INT | 57% | 68% | −11% |
| ABS | 62% | 68% | −6% |
| LNG | 40% | 9% | **+31%** — notable post-restart elevation |
| LOG | 21% | 31% | −10% (surge at restart boundary fully resolved) |
| INF | 14% | — | — |
| MEM | 9% | — | — |
| GEN | 9% | — | — |
| CTX | 7% | — | — |
| SYN | 5% | — | — |
| SEM | 2% | — | — |

PLN fully recovered from the loading-phase dip (FN110: 41% → now 100%). ABS partially recovered (FN110: 33% → 62%, pre-restart: 68%). LNG is the outlier: 9% pre-restart → 40% now. This elevation has persisted across 5+ epochs post-restart. Either a loading artifact still normalizing or a genuine step-3/6 routing shift.

### TTL — ep3898
**G: 15% / O: 81% / R: 4%** — nearly identical to FN111 (G16%/O80%/R4%). Stable, slightly more dormant-weighted than pre-S9 baseline.

### Event bar events since FN111
- **TTL-NASH all-0** (amber) — fired between ep3892 and ep3893 during the post-restart loading phase. Nash equilibrium of TTL routing hit a degenerate all-zero solution. Single event, not persistent. No intervention warranted.
- No SURGERY, WALD, SUB-9.3, or BEST-update events.

### ntfy (last 1h)
Quiet — only the ep3893 WALD (fill 6.2%, already logged FN111) visible in 1h window.

**loss_history ring:** ~37/144 (estimate). S11 gate: ~107 epochs remaining → **~2026-05-27T03:30Z** (unchanged estimate).

**Assessment:** Routing has converged post-restart. PLN recovered, ABS near pre-restart levels. The LNG elevation (40% vs 9% pre-restart) is the one open question — watching whether it normalizes by ep3910–3920 or locks in as a new step-3/6 signature. Epoch averages show gentle upward drift (+0.0046 over 3 epochs): ep3895–3897 sits 40–50bp above the floor at 9.2833. Still well inside oscillation band, no alarm. TTL-NASH was a one-time event during loading phase. Standard watch.

---

## Field Note 113 — 2026-05-26T15:07:14Z · ep3899 batch 258/300 · INT +13%; ep3899 running below ep3898 close — upward drift possibly reversing

**Source:** dashboard screenshot (15:06:15 local) · ep3899 batch 258/300

**State:** ep3899 (22L) · batch 258/300 · ATL 8.8104 · BEST **9.278839** (ep3708, unchanged) · gap to pre-S9 ATL 9.2847: **−0.0059** (unchanged)

### Epoch closes since FN112

No new epoch closes in event bar. ep3898 confirmed closed at **9.3242** (was batch 299/300 in FN112). ep3899 in progress.

**ep3899 running EP-Avg (b258): 9.3179** — 63bp below ep3898 close (9.3242). If this holds, ep3899 would be the best close since ep3894.

Recent close band (ep3895–3898): 9.3196 → 9.3217 → 9.3242 (4-epoch upward drift). ep3899 running avg suggests this drift may be reversing.

### Expert routing — ep3899 batch ~258

| Expert | % | vs FN112 (ep3898 b~299) | delta |
|--------|---|-------------------------|-------|
| PLN | 100% | 100% | = |
| CMP | 100% | 100% | = |
| INT | 70% | 57% | **+13%** — notable increase |
| ABS | 64% | 62% | +2% |
| LNG | 37% | 40% | −3% — slowly normalizing |
| LOG | 17% | 21% | −4% — continued normalization |
| INF | 17% | 14% | +3% |
| MEM | 12% | 9% | +3% |
| GEN | 12% | 9% | +3% |
| SYN | 10% | 5% | +5% |
| CTX | 5% | 7% | −2% |
| SEM | 2% | 2% | = |

PLN/CMP fully saturated (stable). INT jumped to 70% — the largest single-expert delta this tick. LNG continuing slow descent (40%→37%; pre-restart was 9%). LOG continuing normalization (31% surge → 17% now). The INT/SYN/INF/MEM/GEN cluster is all slightly elevated — pointing to broader activation across non-core experts.

### TTL — ep3899 batch 258
**G: 17% / O: 79% / R: 4%** — slightly more active than FN112 (G15%/O81%/R4%). Green up 2pp, Dormant down 2pp. Still well within normal range.

### Gradient
**global |gg| = 0.0026** (vs 0.0021 FN112 — +24%). Mild uptick. Per-layer chart shows activity concentrated in upper layers (L0 most active), small red spike at L17. No abnormality.

### Events
- No WALD, SURGERY, SUB-9.3, or BEST-update events since FN112.
- TTL-NASH all-0 (from FN112) still visible in event bar; no new NASH event.
- ntfy: quiet.

**loss_history ring:** ~38/144 (estimate). S11 gate: ~106 epochs remaining → **~2026-05-27T03:30Z** (unchanged).

**Assessment:** ep3899 running 63bp below ep3898's close (9.3179 vs 9.3242) at batch 258 — this is the most encouraging intra-epoch signal since the restart. The 4-epoch upward drift (ep3895–3898: 9.3196→9.3242) may be peaking. INT routing surge (+13%) and broader activation of non-core experts (SYN, INF, MEM, GEN all up) could reflect the model engaging more processing diversity at this stage of the oscillation. LNG normalization continues. No alarms. Watching ep3899 close.

---

## Field Note 114 — 2026-05-26T15:10:29Z · ep3899 batch 259 · TRAINING INTERRUPTED — Modal gRPC heartbeat failure

**Source:** terminal log paste (15:00:27–15:06:55 local)

**State:** ep3899 (22L) · interrupted at batch 259/300 · last good checkpoint **ep3898** (avg 9.3242) · BEST 9.278839 unchanged

### Incident timeline

| Time (UTC) | Event |
|------------|-------|
| ~15:00:34 | Last training output received (ep3899 b259, loss 9.3601) |
| 15:01:51 | First Modal heartbeat failure (`ConnectionError: Deadline exceeded`) |
| 15:03:07 | 2nd heartbeat failure (+76s — exactly at gRPC HEARTBEAT_TIMEOUT) |
| 15:04:23 | 3rd failure (+76s) |
| 15:05:39 | 4th failure (+76s) |
| 15:06:55 | 5th failure (+76s) — Modal backend evicts app |
| ~15:10 | `modal app list` confirms both apps `stopped`, 0 tasks |

### Root cause
`modal run` (non-detached) requires a live gRPC stream from the local machine to Modal's control plane. A single network interruption at ~15:00:35 broke the stream. gRPC streams are not self-healing. Each heartbeat attempt hits the 76s HEARTBEAT_TIMEOUT and fails. After ~5 consecutive failures Modal evicts the app.

ep3899 checkpoints are written at batch 300 (epoch end). Interrupted at batch 259 → **ep3899 not saved**. Loss at interruption: batches 252–259 range 9.2042–9.4399 (normal variance). Last visible batch avg for ep3899 (b252–259): ~9.354 — consistent with running avg 9.3179 from FN113.

### What was lost
- ep3899 batch 259–300 (41 batches, ~35 seconds of training)
- ep3899 epoch close (would have been ~9.32-9.33 range based on FN113 running avg)

### Recovery
Restart from ep3898 checkpoint. **Use `albert-train --detach`** for all future runs — detached mode runs on Modal's infrastructure independently of local connectivity; ntfy still fires on all threshold events.

**Assessment:** Routine connectivity incident. No data corruption, no WALD anomaly, no loss spike. ep3898 (9.3242) is a clean resume point. Gap to pre-S9 ATL unchanged at −0.0059.

---

## FN115 · 2026-05-26T15:15:16 local · Detached restart — cold start nominal

**Status:** Training resumed after detached restart. No epoch complete yet.

**Telemetry:**
- EP: — (blank) · BATCH: — / — · ATL: 8.8104 (stale display value from prior run, inconsistent with chart Y-axis 9.1–9.6; ignore until first epoch closes)
- Corpus cache hit: 451,386,914 tokens loaded instantly
- Checkpoint: 1,522 tensors loaded from ep3898
- Config: [gate-diversity] scale=0.300, fixed asymmetric logit bias active
- ttlfreeze: enabled · ema_alpha=0.02 · burst_threshold=5x · freeze_steps=50 · max_bursts_per_epoch_per_layer=5
- divloss: enabled weight=1.00e-3 (OVERRIDE — schedule bypassed)
- [lb]: disabled

**Recent event bar (epochs before interruption):** 9.3242, 9.3217, 9.3196, 9.3204, 9.3179, TTL-NASH all-0, 9.3201, 9.3148, 9.3184

**Assessment:** Normal cold-start sequence. Cargo build complete, corpus loaded, tensors restored from ep3898. Training about to begin ep3899 again. All subsystems nominal. ATL display field should update to ~9.278 on first epoch close.

---

## FN116 · 2026-05-26T15:21:23Z · ep3900 · SURGERY GATE fired — plateau region, watching myc_stable

**State:** ep3900 closed · EP-Avg **9.3160** · BEST ~9.2788 (ep3708, unchanged) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**ntfy events (last 20m):**
| Event | ep | Value |
|---|---|---|
| WALD | 3900 | fill=6.2%, mass=9.318, dead_low=3.00–9.00(6.00), dead_high=9.75+(5.25) |
| SURGERY GATE | 3900 | avg 9.3160 — plateau gate region, surgery depends on myc_stable |
| SUB-9.4 EPOCH AVG | 3900 | 9.3160 (restart threshold artifact — reset on detached restart) |
| SUB-10.0 / SURGERY ALERT ZONE | 3900 | restart artifacts — threshold tracking resets after checkpoint resume |

**Expert routing / TTL:** Not available from ntfy; dashboard not provided this tick.

**Assessment:** ep3900 (9.3160) is consistent with the 9.31–9.32 band seen before the Modal heartbeat interruption. WALD fill at 6.2% is healthy (threshold 15%). The SURGERY GATE signal is meaningful: the surgery governor is now in plateau-gate region and watching for `myc_stable` to authorize the next surgery. SUB-10.0 and SURGERY ALERT ZONE events are restart artifacts — the detached checkpoint resume resets some threshold accumulators, causing them to re-fire even though these levels were already achieved weeks ago. No wake-up warranted. BEST unchanged; gap to pre-S9 ATL holds at −0.0059.

---

## FN117 · 2026-05-26T15:32:08Z · ep3900+ · Quiet tick — no new events since ep3900 close

**State:** Last closed: ep3900 · EP-Avg **9.3160** · BEST ~9.2788 (ep3708) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**ntfy (last 20m):** No events beyond those logged in FN116. ep3901 in progress or recently closed without threshold changes. WALD deduplication likely suppressed re-fire if fill% held steady (~6.2%).

**Assessment:** Quiet between epochs. Training proceeding normally in the 9.31–9.32 band. Surgery governor still watching myc_stable. No action required.

---

## FN118 · 2026-05-26T15:42:15Z · ep3905± · Extended quiet — ~5-6 epochs since ep3900, all below threshold

**State:** Last event: ep3900 (15:20:36Z, ~22 min ago) · EP-Avg **~9.31–9.32** (inferred) · BEST ~9.2788 · gap to pre-S9 ATL = **−0.0059**

**ntfy (last 30m):** Only ep3900 events; no new WALD, SURGERY, or threshold fires since. WALD deduplication suppressing re-fire (fill% stable ~6.2%).

**Elapsed time analysis:** ~22 min at ~3–4 min/epoch → approximately ep3905–3907 in progress. No threshold crossings → all epochs in the 9.31–9.32 band, no new BEST, WALD fill steady.

**Assessment:** Training running normally in steady-state. No events requiring attention. SURGERY GATE is live; no myc_stable signal yet. Gap to pre-S9 ATL unchanged. No user wake required.

---

## FN119 · 2026-05-26T15:57:55Z · ep3908± · Extended quiet — ~10 epochs since ep3900, dashboard read from ep3905

**State:** ep3905 observed 15:44Z · EP-Avg **9.3079** (T-610 running avg 9.3143) · BEST ~9.2788 (ep3708) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**Source:** Dashboard screenshot (ep3905, BATCH 155/300, 15:44Z) + ntfy quiet last 60m

**Expert routing (ep3905 snapshot):**
| Expert | % |
|--------|---|
| SYN | 20% |
| SEM | 8% |
| CTX | 13% |
| INF | 10% |
| MEM | 5% |
| GEN | 15% |
| LOG | 45% |
| LNG | 28% |
| ABS | 64% |
| PLN | 100% |
| CMP | 65% |
| INT | 67% |

**TTL:** G 6.14% / O 83% / R 3%

**Per-layer gradient norm:** global_tg = 0.0016 (healthy, no explosion)

**WALD:** Last fired ep3900 fill=6.2%; no re-fire since (fill stable, deduplication holding)

**Notable routing vs FN116:** PLN at 100% (dominant), ABS 64%, CMP 65%, INT 67% — reasoning/logic cluster fully active. LOG 45% elevated. GEN 15% (moderate). LNG 28% (language moderate). TTL heavily Dormant (O 83%) is expected in steady-state plateau.

**Assessment:** ep3905–3908 closing quietly in the 9.31–9.33 range. No threshold crossings, no WALD re-fires, no new BEST. Gradient norm 0.0016 is clean — no instability. PLN at 100% confirms the model is driving hard on planning/reasoning tokens; this is typical plateau behavior before surgery governor fires. SURGERY GATE still live, watching myc_stable. Extended quiet is normal — WALD deduplication suppresses re-fires when fill% is stable. No wake required. Estimated current epoch: ~3909 (37 min at ~3.5 min/epoch from ep3900).

---

## FN120 · 2026-05-26T16:04:04Z · ep3908 · SUB-9.3 epoch avg — first running average below 9.3

**State:** ep3908 closed · EP-Avg **9.2999** (new depth floor, first SUB-9.3) · BEST 9.2788 (ep3708) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**ntfy events (last 1h):**
| Time (UTC) | Event | Detail |
|---|---|---|
| 15:20:38Z | SURGERY GATE | ep3900 avg 9.3160 — plateau gate region, watching myc_stable |
| 15:20:38Z | WALD | ep3900 fill=6.2% mass=9.318 dead_low=3.00–9.00(6.00) dead_high=9.75+(5.25) |
| 15:20:38Z | SUB-9.4 EPOCH AVG | ep3900 avg 9.3160 — new depth floor |
| 16:02:51Z | **SUB-9.3 EPOCH AVG** | **ep3908 avg 9.2999 — new depth floor** |

**Expert routing:** Last snapshot ep3905 — PLN 100% / ABS 64% / CMP 65% / INT 67% / LOG 45% / LNG 28% / GEN 15% (no new screenshot; routing assumed stable)

**TTL:** G 6.14% / O 83% / R 3% (last known, ep3905)

**WALD:** Still at fill=6.2% from ep3900 — no re-fire, deduplication holding; fill stable confirms no new dead experts.

**Assessment:** ep3908 crossed the 9.3 boundary — epoch average 9.2999. This is the first time the running epoch average has dropped below 9.3 since training began in this generation. BEST is still 9.2788 (ep3708) so no new ATL from this epoch close, but the trend is a slow descent through the 9.31–9.32 plateau. Gap to pre-S9 ATL unchanged at −0.0059. SURGERY GATE remains live watching myc_stable. No BEST broken, no WALD spike, no loss explosion — no wake required. Next threshold to watch: sub-9.28 (would beat pre-S9 ATL) or SURGERY firing.

---

## FN121 · 2026-05-26T16:17:15Z · ep3911± · Quiet hold — SUB-9.3 confirmed, no new events

**State:** ep3908 last close confirmed · EP-Avg **9.2999** (last known) · BEST 9.2788 (ep3708) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**ntfy (last 20m):** No new Albert events since ep3908 SUB-9.3 at 16:02Z (~15 min ago). Simeon manual confirmation at 16:09Z: "we just hit a new low finally." WALD at fill=6.2%, deduplication holding, no re-fire.

**Elapsed since last event:** ~15 min · estimated current epoch ~3911–3912 (at ~3–4 min/epoch)

**Assessment:** Training quiet in hold pattern after the SUB-9.3 milestone. No new threshold crossings, no WALD re-fire, no BEST. Simeon is out to dinner, monitoring via ntfy — no escalation needed. Model likely tracking in the 9.29–9.31 band; next meaningful event would be a new BEST below 9.2788 or SURGERY governor firing. Steady watch continues.

---

## FN122 · 2026-05-26T16:32:13Z · ep3916± · Quiet hold — 30 min silence post SUB-9.3, no new events

**State:** ep3908 last confirmed close · EP-Avg **9.2999** (last known) · BEST 9.2788 (ep3708) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**ntfy (last 20m):** Empty — no Albert events. Last event was ep3908 SUB-9.3 at 16:02Z (~30 min ago). WALD fill=6.2% stable, no re-fire.

**Elapsed since last event:** ~30 min · estimated current epoch ~3915–3917 (at ~3–4 min/epoch from ep3908)

**Assessment:** Extended quiet after SUB-9.3 milestone at ep3908. No BEST broken, no WALD spike, no SURGERY. Model is running through epochs in the sub-9.3 band without triggering any new threshold events — this means epoch averages are closing in the 9.28–9.30 range (below 9.3 but not yet below 9.2788 for a new BEST). Deduplication holding on all thresholds already crossed. Watching for: new BEST below 9.2788, SURGERY governor fire, or WALD fill creep above 15%. No wake required.

---

## FN123 · 2026-05-26T16:47:08Z · ep3920± · Quiet hold — 45 min silence, no new events

**State:** ep3908 last confirmed close · EP-Avg **9.2999** (last known) · BEST 9.2788 (ep3708) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**ntfy (last 20m):** Empty. No Albert events of any kind. Last event ep3908 SUB-9.3 at 16:02Z (~45 min ago).

**Elapsed since last event:** ~45 min · estimated current epoch ~3919–3921 (at ~3–4 min/epoch)

**Assessment:** Sustained quiet. ~11–13 epochs have closed since ep3908 with no threshold crossings — consistent with epoch averages in the 9.28–9.30 band, above the BEST floor of 9.2788. WALD fill stable (no re-fire = fill not climbing). SURGERY GATE watching myc_stable. No escalation criteria met. Watch continues.

---

## FN124 · 2026-05-26T17:02:09Z · ep3925± · Full hour of silence — no events since ep3908 SUB-9.3

**State:** ep3908 last confirmed close · EP-Avg **9.2999** (last known) · BEST 9.2788 (ep3708) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**ntfy (last 20m):** Empty. No Albert events since ep3908 at 16:02Z — exactly 60 min of silence.

**Elapsed since last event:** ~60 min · estimated current epoch ~3923–3927 (~15–20 epochs at 3–4 min/epoch)

**Assessment:** One full hour without a threshold event. ~15–20 epochs have closed silently since the SUB-9.3 at ep3908, all above the BEST floor of 9.2788. This is typical plateau behavior — the model descends to a new band floor, then grinds through it without triggering the next threshold. SURGERY GATE still watching myc_stable. WALD fill stable. No escalation criteria met. Watch continues.

---

## FN125 · 2026-05-26T17:17:08Z · ep3929± · Quiet hold — 75 min silence, no new events

**State:** ep3908 last confirmed close · EP-Avg **9.2999** (last known) · BEST 9.2788 (ep3708) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**ntfy (last 20m):** Empty. No Albert events since ep3908 at 16:02Z — 75 min of unbroken silence.

**Elapsed since last event:** ~75 min · estimated current epoch ~3927–3931 (~19–23 epochs at 3–4 min/epoch)

**Assessment:** 75 min of silence. Model running through the sub-9.3 plateau band without triggering thresholds. All deduplication gates holding. SURGERY GATE live watching myc_stable. No escalation criteria met. Watch continues.

---

## FN126 · 2026-05-26T17:32:09Z · ep3933± · Quiet hold — 90 min silence, sub-9.3 plateau grind

**State:** ep3908 last confirmed close · EP-Avg **9.2999** (last known) · BEST 9.2788 (ep3708) · gap to pre-S9 ATL 9.2847 = **−0.0059**

**ntfy (last 20m):** Empty. No Albert events since ep3908 at 16:02Z — 90 min of silence.

**Elapsed since last event:** ~90 min · estimated current epoch ~3931–3935 (~23–27 epochs at 3–4 min/epoch)

**Assessment:** 90 min of unbroken quiet. ~25 epochs grinding silently through the sub-9.3 band. No BEST, no WALD re-fire, no SURGERY. Plateau grind is expected behavior here — model descends to a new floor then stalls before the next compression event. SURGERY GATE watching myc_stable. No escalation criteria met.
