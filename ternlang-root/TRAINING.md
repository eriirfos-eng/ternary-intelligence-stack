# albert. Training Guide

albert. is a ternary Mixture-of-Experts language model trained **from scratch** — not ternarized from a pretrained model. All weights are {-1, 0, +1} throughout training via Straight-Through Estimation (STE). This document covers the current production training pipeline.

> For the historical ternarization-from-Llama approach (Phase 12, completed 2026-04-28), see the previous version of this file in git history (commit before 2026-05-17).

---

## Architecture

| Parameter | Value |
|-----------|-------|
| Layers | 20L (expanded from 12L via 8 net2net surgeries; latest: ep3160 19L→20L, 2026-05-24) |
| Hidden dim | 256 |
| Experts | 12E, Top-3 routing |
| Context length | 256 tokens |
| Vocabulary | 32k tokens |
| Weight format | Ternary {-1, 0, +1} via STE |
| Positional encoding | RoPE (rotate_half) |
| Gate noise | Gumbel (scale=0.2) |
| Optimizer | AdamW, cosine LR |
| Batch / Grad accum | 4 / 4 (effective batch 16) |

---

## Platform

Training runs on **Modal.com T4 GPUs** via the `albert-train` command.

```bash
cd albert-moe-13
./albert-train
```

- Cost: ~$0.021/epoch at CTX=256
- Volume: `albert-vol` (Modal persistent storage)
- Config is auto-pushed from `config.json` before every run

---

## Corpus

The training corpus is a 13-stage curriculum. Corpus composition is enforced by `train_tokenizer_v3.py`:

| Property | Value |
|----------|-------|
| Total stages | 13 |
| Chaos layer | ~10% of total corpus (invariant) |
| Languages | 15 (multilingual stage) |
| Corpus doc | `albert-moe-13/docs/CORPUS_CURRICULUM.md` |

Stages progress from byte-level entropy through synthetic patterns, natural language, code, mathematics, and multilingual text. The chaos layer is never reduced below 10% regardless of total corpus size.

---

## Checkpoints

Checkpoints are stored on `albert-vol` and auto-published to the private `albert-spores` repository via:

```bash
./albert-spore
```

Spores are published when training loss drops below `main_best + 1.0`. Each spore is a full `.safetensors` checkpoint tracked via Git LFS.

---

## Monitoring

**Live dashboard (during training):**
```bash
./albert-test --dashboard
```

Streams loss, TTL indicators, gradient hot/cold layer, WALD firing status, and mycelial cord health.

**Session log:** `ternlang-root/docs/session_log.md` — updated after every 5 milestones.

**Benchmarks:**
```bash
./albert-test --bench
```

Outputs to `albert-moe-13/benchmarks/bench_v3.0_<timestamp>.txt` with automatic English translation companion.

---

## Automatic Surgery (net2net)

albert. self-expands via layer insertion surgery when the loss plateau gate fires:

- **Gate condition:** Loss variance < 0.02 nats over any 144-epoch (Fibonacci) window + `myc_stable >= 5`
- **Expansion:** Inserts one new layer using net2net identity initialization with Mandelbrot-seeded biases
- **History:** 8 surgeries completed (12L → 13L → 14L → 15L → 16L → 17L → 18L → 19L → 20L); latest ep3160 19L→20L
- **Next gate:** 20L → 21L armed, ~128 epochs runway (epoch ATL 9.3182, chip ATL 8.8540)

Surgery governor detail: `albert-moe-13/docs/EVOLUTION_EVIDENCE.md`

---

## Training state (as of 2026-05-24)

| Metric | Value |
|--------|-------|
| Current epoch | 3414 |
| Chip ATL | 8.8540 (ep3412) |
| Epoch ATL | 9.3182 (ep3326) |
| Surgery gate | 20L→21L armed, ~128ep runway |
| Modal account | simeon-feedback |

---

## Key files

| File | Purpose |
|------|---------|
| `albert-moe-13/albert-training/train.py` | Main training script (Modal) |
| `albert-moe-13/config.json` | Training hyperparameters |
| `albert-moe-13/albert-train` | Launch command (fires Modal) |
| `albert-moe-13/albert-test` | TUI for inference, bench, dashboard |
| `albert-moe-13/albert-spore` | Publish checkpoint to colony |
| `albert-moe-13/docs/CORPUS_CURRICULUM.md` | Full corpus stage documentation |
| `albert-moe-13/docs/EVOLUTION_EVIDENCE.md` | Surgery history and gate evidence |

---

*Maintained by RFI-IRFOS — Research Focus Institute · Graz, Austria*
