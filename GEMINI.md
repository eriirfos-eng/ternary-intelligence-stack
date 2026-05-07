# Ternary Intelligence Stack — Project Instructions
# RFI-IRFOS · ternlang.com · v2.1 (2026-05-06)

---

## ⚠️ CRITICAL OPERATIONAL RULES — READ BEFORE ANYTHING ELSE

1. **NEVER commit or push `*.safetensors`, `*.trit`, `*.bin`, `*.onnx` files.** They are in `.gitignore`. Model weights live on the Fly.io volume only. Attempting to push them will be rejected by GitHub (files exceed 100MB limit).
2. **ALWAYS run `ls` on a path before writing or replacing a file.** If the path does not exist, stop and report — do not create phantom files.
3. **ALWAYS verify your changes with `git diff` before committing.** Never assume success.
4. **Domain-relative paths only** inside training scripts. Never hardcode absolute paths.
5. **git log is ground truth. Your memory of "what you did" is a hypothesis.**

---

## ⚠️ MODULE COUNT — DO NOT INFER FROM DIRECTORY NAMES

**This is the single most common agent error in this repo. Read this before touching any file.**

| Directory | Classification | Count | Meaning |
|-----------|---------------|-------|---------|
| `ternlang-root/stdlib/` | **Open-core, Tier-1 foundation** | ~293 .tern scripts | Canonical core modules |
| `ternlang-root/premlib/` | **Open-core, domain-specialized** | **28,500+ .tern modules** | Free, published, open |

**`premlib/` does NOT mean proprietary or closed-source.**
The name means "domain-specialized premium use cases" (aerospace, medical, legal, etc.).
Every module in premlib is free and open. Classifying premlib as "proprietary" is INCORRECT.

**Total open-core: 28,500+ modules (stdlib + premlib combined).**

If you are about to write "293 open-core modules and 28,000 proprietary modules" — STOP.
That is exactly backwards. Read `ternlang-root/README.md` §Standard Library for the authoritative count.

---

## Architecture: Albert MoE-13 (v2.0.0) — CURRENT

| Parameter | Value |
|-----------|-------|
| Model version | **v2.0.0** |
| Hidden size | **256** |
| Num layers | **3** (dynamically expanded by EvolutionManager — can become 5L after surgery) |
| Num experts | **12** |
| Num heads | **4** |
| Max sequence length | **128** |
| Routing | **Top-3 Sparse Routing** |
| Training corpus | King James Bible (`data/corpus/`) |
| Global epoch (approx) | **12+** (continues — check `training.log` for live state) |
| Current loss (approx) | **~6.5–7.0** (best recorded: 2.1353) |

---

## EvolutionManager — Auto-Evolutionary Training

Albert uses a `EvolutionManager` state machine that monitors loss and autonomously triggers **Net2Net "Safe Copy" layer surgery** — cloning the last layer to grow depth without destroying existing weights. This means:

- The model can change from `3L → 5L` mid-run without manual intervention.
- The active architecture is always the ground truth. **Check `training.log` for current `ARCH N LAYERS` before assuming dimensions.**
- After surgery the checkpoint file grows significantly (3L ~41MB → 5L ~91MB+).

---

## Source Map (Albert MoE-13)

All paths relative to repo root: `/home/eri-irfos/projects/ternary-intelligence-stack/`

| Component | Path |
|-----------|------|
| Training kernel | `albert-moe-13/moe-llm-core/src/bin/train_bible.rs` |
| MoE routing | `albert-moe-13/moe-llm-core/src/model/moe.rs` |
| Attention | `albert-moe-13/moe-llm-core/src/model/attention.rs` |
| STE engine | `albert-moe-13/moe-llm-core/src/model/ste.rs` |
| Ternary core | `albert-moe-13/moe-llm-core/src/model/ternary_linear.rs` |
| Dashboard server | `albert-moe-13/dashboard/run_server.py` |
| Dashboard UI | `albert-moe-13/dashboard/index.html` |
| Active training log | `albert-moe-13/dashboard/training.log` |
| Model config | `albert-moe-13/models/bible_ternary_v2.0.0.config.json` |
| Model odometer | `albert-moe-13/models/bible_ternary_v2.0.0.meta` |
| Checkpoint (on disk, not in git) | `albert-moe-13/models/bible_ternary_v2.0.0.safetensors` |
| New crates | `albert-moe-13/crates/` (moe-core, moe-platform, moe-runtime, moe-validation-suite, …) |

---

## Training Status

- **Active**: Training runs continuously on Bible corpus.
- **Loss schedule**: Cosine LR decay, 2e-4 → 1e-5 over 500 global steps.
- **Monitor live**: `tail -f albert-moe-13/dashboard/training.log`
- **Dashboard**: `python albert-moe-13/dashboard/run_server.py` → http://localhost:8888

---

## Training Roadmap

1. **Phase 1 (Active)**: Bible corpus — linguistic and logical foundation.
2. **Phase 2**: Linux / technical documentation — engineering reasoning.
3. **Phase 3**: EU legal corpus — logical precision and compliance reasoning.
4. **Phase 4**: General-purpose web-scale pre-training.

Multi-corpus pipeline is implemented: drop `.txt` files into `albert-moe-13/data/corpus/` and they are picked up on next restart.

---

## Path Integrity Protocol

- Repo root: `/home/eri-irfos/projects/ternary-intelligence-stack/`
- Albert root: `albert-moe-13/`
- Ternlang root: `ternlang-root/`
- API: `ternlang-root/ternlang-api/`
- Web: `ternlang-root/ternlang-web/`
- CLI: `agent_albert_cli/`
- **ALWAYS `ls` before write. ALWAYS `git diff` before commit. ALWAYS `git log` to verify.**
