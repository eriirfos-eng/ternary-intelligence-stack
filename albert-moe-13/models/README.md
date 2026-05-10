# Albert MoE-13 — Checkpoint Registry

Training artifacts for the Albert MoE-13 ternary language model.

---

## Active Checkpoint

**`bible_ternary_v2.0.0`** — current training state

| Field | Value |
|-------|-------|
| Format | `.safetensors` (float32 weights, HuggingFace standard) |
| Config | `bible_ternary_v2.0.0.config.json` |
| Epoch counter | `bible_ternary_v2.0.0.meta` (plain text integer) |
| Architecture | 256H · **12L** · 4H · 12E · 128CTX · 8000V |
| Best loss | 6.8821 (Global Epoch 454+) |
| Global Epoch | 477+ (as of 2026-05-10) |
| Corpus | Bible KJV + Alice + Gutenberg + Simple English Wikipedia (stages 3/6/7 active) |
| Tensors | 689 (blocks.0–11: 684 · ln_f: 2 · pos_embed: 1 · embed: 1 · lm_head: 1) |
| File size | ~316 MB |

The checkpoint is overwritten at the end of every 300-batch epoch. The `.meta` file stores the global epoch count across all training sessions.

---

## Config Schema

`bible_ternary_v2.0.0.config.json`:
```json
{
  "hidden_size": 256,
  "num_layers": 12,
  "num_heads": 4,
  "max_seq_len": 128,
  "num_experts": 12,
  "vocab_size": 8000
}
```

`num_layers` is updated in-place by the `EvolutionManager` when Net2Net surgery fires. The training binary reads this file on each `train_cycle` start, so the growing architecture is always picked up correctly.

---

## Version History

| Version | Architecture | Key Event | Global Epoch |
|---------|-------------|-----------|--------------|
| v2.0.0 · 12L | 256H · **12L** · 4H · 12E | Max depth reached; layer crystallization (L0-L3 frozen, L11 hot); TTL cycling reds self-resolving | 454→current |
| v2.0.0 · 5L–11L | 256H · 5L→11L · 4H · 12E | Autonomous surgery chain; EvolutionManager max_layers=12 cap enforced on all paths | ~385–454 |
| v2.0.0 · 5L | 256H · **5L** · 4H · 12E | Net2Net surgery 4L→5L; TTL routing + anti-stagnation burst; lb_lambda=0.03 | ~381–385 |
| v2.0.0 · 4L | 256H · 4L · 4H · 12E | 256H upgrade; TTL routing (TARGET fix); LB loss λ 0.01→0.02→0.03 | ~340–381 |
| v1.3.6 | 96H · 3L · 8E | Stable foundation; STE + top-3 MoE routing | ~200–340 |
| v1.3.5 | 96H · 3L · 8E | Initial 13-node MoE | 0–200 |

Archived snapshots in `models/registry/` include config, report, and evolution metadata.

---

## Surgery Log

| Event | Global Epoch | From | To | Notes |
|-------|-------------|------|----|-------|
| 11L → 12L | ~454 | 256H · 11L | 256H · 12L | Max depth reached; training continues at 12L floor |
| 10L → 11L | ~445 | 256H · 10L | 256H · 11L | Autonomous plateau surgery |
| 9L → 10L | ~435 | 256H · 9L | 256H · 10L | Autonomous plateau surgery |
| 8L → 9L | ~425 | 256H · 8L | 256H · 9L | Autonomous plateau surgery |
| 7L → 8L | ~415 | 256H · 7L | 256H · 8L | Autonomous plateau surgery |
| 6L → 7L | ~405 | 256H · 6L | 256H · 7L | Autonomous plateau surgery |
| 5L → 6L | ~395 | 256H · 5L | 256H · 6L | Autonomous plateau surgery |
| 4L → 5L | ~381 | 256H · 4L | 256H · 5L | Net2Net layer copy + σ=0.01 symmetry break |
| 3L → 4L | ~340 | 256H · 3L | 256H · 4L | First 256H surgery |
| 128H → 256H | ~200 | 128H · 3L | 256H · 3L | Hidden size expansion |

---

## Checkpoint Loading

The training binary loads with shape matching:

```rust
for (name, var) in all_vars.iter() {
    if let Some(tensor) = checkpoint_data.get(name) {
        let _ = var.set(tensor);
    }
}
```

**Architecture change (e.g. surgery):** new layer weights are copied from the deepest existing layer before saving. On next load, all shapes match — 689 tensors for 12L.

---

## v3.0 Init Checkpoint (pending)

When v2.0.0 training completes at Global Epoch 500:

1. Archive final checkpoint as `bible_ternary_v2.0.0_final.safetensors`
2. Run `scripts/transfer_weights_v3.py` to produce `albert_v3.0_init.safetensors`
3. Transfer: 687 vocabulary-agnostic tensors (`blocks.*`, `ln_f.*`, `pos_embed.weight`)
4. Rebuild: `embed.weight` and `lm_head.weight` at new vocab size (32,000 tokens)

The transfer preserves 500 epochs of learned sequence modelling, routing specialisation, and expert differentiation. Only the vocabulary interface is replaced.

---

## Serialization Format

Checkpoints use [SafeTensors](https://github.com/huggingface/safetensors) — header-based, memory-mapped, cross-framework compatible. Weights stored in float32 for training; ternary quantization applied at runtime via STE.

Future: `.trit` packed format (5 trits/byte) for hardware deployment.

---

## See Also

- [Main README](../README.md)
- [Benchmark Suite](../bench/install.sh) — one-line installer, exports CSV
- [Architecture](../docs/architecture.md)
- [Roadmap](../docs/roadmap.md)
