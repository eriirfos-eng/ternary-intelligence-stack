# Ternary Intelligence Stack — Project Instructions
# RFI-IRFOS · ternlang.com · v3.0 (2026-05-14)

---

## CRITICAL OPERATIONAL RULES — READ BEFORE ANYTHING ELSE

1. **NEVER commit or push `*.safetensors`, `*.trit`, `*.bin`, `*.onnx` files.** They are in `.gitignore`. Model weights live on the Modal.com volume only. Attempting to push them will be rejected by GitHub (files exceed 100MB limit).
2. **ALWAYS run `ls` on a path before writing or replacing a file.** If the path does not exist, stop and report — do not create phantom files.
3. **ALWAYS verify your changes with `git diff` before committing.** Never assume success.
4. **Domain-relative paths only** inside training scripts. Never hardcode absolute paths.
5. **git log is ground truth. Your memory of "what you did" is a hypothesis.**
6. **No emojis.** Use minimalistic SVGs for icons. Text output must be plain.

---

## MODULE COUNT — DO NOT INFER FROM DIRECTORY NAMES

**This is the single most common agent error in this repo. Read this before touching any file.**

| Directory | Classification | Count | Meaning |
|-----------|---------------|-------|---------|
| `ternlang-root/stdlib/` | Open-core, Tier-1 foundation | ~293 .tern scripts | Canonical core modules |
| `ternlang-root/premlib/` | Open-core, domain-specialized | 28,500+ .tern modules | Free, published, open |

**`premlib/` does NOT mean proprietary or closed-source.**
The name means "domain-specialized premium use cases" (aerospace, medical, legal, etc.).
Every module in premlib is free and open. Classifying premlib as "proprietary" is INCORRECT.

**Total open-core: 28,500+ modules (stdlib + premlib combined).**

---

## Architecture: albert. (Albert-MoE-13) v3.0 — CURRENT

| Parameter | Value |
|-----------|-------|
| Model version | **v3.0** |
| Hidden size | **256** |
| Num layers | **17** (grown from 3L via 6 Net2Net surgeries) |
| Num experts | **12** (Top-3 sparse routing per token) |
| Context length | **256 tokens** |
| Vocabulary | **32,000 tokens** (custom BPE, `tokenizer_v3/`) |
| Weight representation | **Ternary {-1, 0, +1}** with STE training |
| Gate linear | F32 |
| Positional encoding | RoPE (rotate_half) |
| Parameters (effective) | ~13M |
| Global epoch | **851+** (check `training.log` for live state) |
| All-time best loss | **10.2050** (ep849) |
| Training platform | **Modal.com GPU (T4)** via `albert-train` |

The central technical innovation is `@sparseskip` — a learned sparse-skip layer
that dynamically bypasses computation paths based on token-level activation patterns,
enabling sub-linear inference scaling without pruning. Patent pending A50296/2026.

---

## Training Corpus (v3.0)

| Tier | Content | Share |
|------|---------|-------|
| Core | Project Gutenberg (public domain books, multilingual) | ~30% |
| Core | Wikipedia (English, German, French) | ~25% |
| Core | OpenWebText (filtered Common Crawl) | ~15% |
| Technical | GitHub issues, developer blogs, HN discussions | ~10% |
| Chaos | Synthetic noise, adversarial patterns, mixed-language text | ~10% |
| Structured | EU AI Act text, Linux kernel docs, TLDR pages | ~5% |
| Multilingual | EU language samples | ~5% |

The **10% chaos layer** is a structural invariant enforced by `train_tokenizer_v3.py`.
Do not reduce it. See `DATA_PROVENANCE.md` for full governance details.

---

## Source Map

All paths relative to repo root.

| Component | Path |
|-----------|------|
| Training kernel | `albert-moe-13/moe-llm-core/src/bin/train_bible.rs` |
| MoE routing | `albert-moe-13/moe-llm-core/src/model/moe.rs` |
| Attention | `albert-moe-13/moe-llm-core/src/model/attention.rs` |
| STE engine | `albert-moe-13/moe-llm-core/src/model/ste.rs` |
| Ternary core | `albert-moe-13/moe-llm-core/src/model/ternary_linear.rs` |
| sparseskip primitive | `albert-moe-13/moe-llm-core/src/model/sparseskip.rs` |
| Modal training launcher | `albert-moe-13/train_modal.py` |
| albert-train binary | `albert-moe-13/albert-train` |
| albert-test binary | `albert-moe-13/albert-test` |
| Dashboard server | `albert-moe-13/dashboard/server.py` |
| Dashboard UI | `albert-moe-13/dashboard/index.html` |
| Active training log | `albert-moe-13/dashboard/training.log` |
| Model config | `albert-moe-13/models/albert_v3.0.config.json` |
| Model odometer | `albert-moe-13/models/albert_v3.0.meta` |
| Checkpoint (not in git) | `albert-moe-13/models/albert_v3.0.best.safetensors` |
| Tokenizer | `albert-moe-13/tokenizer_v3/` |
| Corpus scripts | `albert-moe-13/scripts/` |
| EU AI Act docs | `MODEL_CARD.md`, `DATA_PROVENANCE.md`, `SECURITY.md` |
| Checkpoint gate | `albert-moe-13/scripts/promote_checkpoint.sh` |

---

## Training Workflow

- **Launch training:** `albert-train` (fires Modal GPU job; auto-pushes config before remote call)
- **Run benchmark:** `albert-test /bench` (requires checkpoint on disk)
- **Monitor live:** `tail -f albert-moe-13/dashboard/training.log`
- **Dashboard:** `python albert-moe-13/dashboard/server.py` → http://localhost:8888
- **Promote checkpoint:** `albert-moe-13/scripts/promote_checkpoint.sh` (human authorization gate, EU AI Act Art. 53(1)(e))

---

## Surgery Governor

Net2Net layer surgery fires when training has plateaued, not while descending.
The surgery governor monitors loss trajectory and withholds growth signals during
active descent, only firing when the plateau detector confirms stagnation.
Threshold: `loss_avg <= 9.8`. Current gap: ~0.4 at ep851+. Estimated ~500 further
epochs at current rate before the 6th surgery trigger.

---

## EU AI Act Compliance (GPAI — Art. 3(63))

| Obligation | Article | Document |
|------------|---------|----------|
| Technical documentation | Annex XI | `MODEL_CARD.md` |
| Training data provenance | Art. 53(1)(c-d) | `DATA_PROVENANCE.md` |
| Human oversight | Art. 53(1)(e) | `albert-moe-13/scripts/promote_checkpoint.sh` |
| Incident reporting | Art. 53(2) | `SECURITY.md` |

---

## Path Integrity Protocol

- Repo root: `ternary-intelligence-stack/`
- albert. root: `albert-moe-13/`
- Ternlang root: `ternlang-root/`
- API: `ternlang-root/ternlang-api/`
- CLI agent: `agent_albert_cli/`
- **ALWAYS `ls` before write. ALWAYS `git diff` before commit. ALWAYS `git log` to verify.**
