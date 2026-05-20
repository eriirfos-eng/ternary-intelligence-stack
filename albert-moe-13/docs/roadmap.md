# Albert-MoE-13 Model Roadmap

---

## Completed — v2.0.0 (Global Ep454, 12L, best loss 6.8821)

*   [x] Straight-Through Estimator (STE) ternary training from random init — weights constrained to `{-γ, 0, +γ}` throughout
*   [x] MoE architecture — 12 experts, Top-3 routing, @sparseskip (patent pending A50296/2026)
*   [x] Ternary Traffic Light Routing (TTL) — EMA-based trit states (G/O/R) per expert per layer, anti-stagnation burst mechanism
*   [x] EvolutionManager — autonomous Net2Net surgery, 4L→12L (max_layers=12 cap enforced on both plateau and COLLAPSE→SURGERY paths)
*   [x] Mycelium — 20-epoch rolling gradient pressure monitor; dead=0 throughout entire 12L run
*   [x] Stage-aware corpus curriculum — stage_3 through stage_7+ active at 12L (Bible, Alice, Gutenberg, Wikipedia EN)
*   [x] 83 tok/s sustained CPU inference on HP ZBook i7-4800MQ (measured at 4L config, 2026-05-07)
*   [x] Layer crystallization — L0–L3 grad norms ~0.00002–0.001 (nearly frozen), L11 hot at 0.013–0.022; internal differentiation without architecture growth
*   [x] Sparsity gradient — L0:10.6% → L11:26.5% across the 12-layer stack (TELE confirmed)
*   [x] Cycling reds self-organization — TTL R3 states migrate through cold layers (L2→L3→L5), self-resolving, dead=0
*   [x] Global Epoch 454+ · best loss 6.8821 · 12L checkpoint (316 MB)

---

## Active — v3.0 — Current Run (ep2490+, 18L, epoch-ATL 9.6248)

**Status (2026-05-20):** Global Epoch 2490+, Modal T4 GPU. Six autonomous Net2Net surgeries completed (12L→18L). Stages 1–13 corpus active. Gen 1 step 1/6, plateau window 233 epochs, ceiling 21L.

### Completed milestones
*   [x] 32k multilingual ByteLevel BPE vocabulary (EN/DE/FR/ES/PT/IT/NL/PL)
*   [x] Weight transfer from v2.0.0 (12L→v3.0 init); embed + lm_head rebuilt at 32k
*   [x] Wikipedia + Europarl multilingual corpus (~446 MB), academic (~46 MB), fulltext (~68 MB), chaos (~43 MB) — 635 MB total
*   [x] Net2Net surgery ×5: 12L→13L (ep511) · 13L→14L (ep547) · 14L→15L (ep611) · 15L→16L (ep645) · 16L→17L (ep701)
*   [x] **Net2Net surgery ×6: 17L→18L (ep2487, 2026-05-20)** — first surgery under full Fibonacci+Mandelbrot+Gen cycling; no spike; immediate descent; batch ATL 9.3866 by ep2489
*   [x] Stage 10 corpus unlocked at 16L: dev_blogs, github_bugs, hn_discussions, gourmet_recipes, repair_guides, trails_travel
*   [x] WALD module — loss-space coverage analysis, dead-zone detection, early-layer gradient amplification
*   [x] Expert seed biases — F32 [256] per expert, breaks routing Nash equilibria
*   [x] Gate reset footgun fixed — kaiming-uniform + expert noise now gated behind `--break-symmetry` / entropy auto-detection
*   [x] SPORE federated weight sharing — SporeManager implemented, collaborator Zabih onboarded
*   [x] Mycelial Cord Architecture — implemented (anastomosis.rs, dual-stream), not yet activated (awaits width wall)
*   [x] WMMA INT8 kernel — written, validated (max_err=0.000000), disabled mid-run; re-enable on next fresh run
*   [x] Modal.com T4 GPU training — ~40× speedup over CPU, ~$0.003/epoch at CTX=256
*   [x] HuggingFace deployment — rfi-irfos/albert, model card live with full team roster
*   [x] batch_history.csv persistence — survives Modal worker preemptions, dashboard SMA pre-seeded on startup
*   [x] Dashboard TTL panel — 17-layer scrollable canvas, all layers render correctly
*   [x] **Epoch ATL 10.1993 at ep1189 — first sub-10.20 in albert. v3.0 history**
*   [x] **Batch ATL 10.1600 at ep1185 (-0.45% vs prior record)**
*   [x] Benchmark suite: 84.4 tok/s · 11.8 ms/tok · 75% expert skip rate (HP ZBook i7-4800MQ, CPU-only)

### Pending
*   [ ] 7th Net2Net surgery (18L→19L) — Gen 1 step 2/6, plateau window 377 epochs; ceiling 21L
*   [ ] Stage 11–12 corpus fully integrated (arxiv, eurlex, science_SE, crossref, pubmed — unlocked at ep2487 surgery)
*   [ ] Mycelial Cord activation (`--cord-surgery`) — diagnose width wall, then fire once manually
*   [ ] WMMA INT8 kernel re-enable on next fresh training run
*   [ ] Bias/fairness evaluation (WinoBias, BBQ, multilingual MMLU) — planned v3.1 milestone
*   [ ] Element-level @sparseskip within active expert MLPs (next sparsity tier after expert-level skip)
*   [ ] Hungarian + endangered language corpus integration

---

## Phase Hardware — European Silicon

---

## Phase Hardware — European Silicon

*   [ ] Lower @sparseskip and ternary matmul primitives to FPGA/HDL targets
*   [ ] Native support for European AI silicon as it matures
*   [ ] Multi-node distributed inference for v3.0+ models
*   [ ] On-device inference target: ternary weights at 32k vocab fit in ~20MB — viable for edge deployment
