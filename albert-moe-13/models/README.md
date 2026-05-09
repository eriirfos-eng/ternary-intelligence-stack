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
| Architecture | 256H · **5L** · 4H · 12E · 128CTX · 8000V |
| Best loss | 6.882 (Global Epoch ~381) |
| Global Epoch | 385+ (as of 2026-05-09) |
| Corpus | Bible KJV + Alice in Wonderland (stage_3 active) |
| Tensors | 290 loaded from checkpoint (5L × 12E) |

The checkpoint is overwritten at the end of every 300-batch epoch. The `.meta` file stores the global epoch count across all training sessions.

---

## Config Schema

`bible_ternary_v2.0.0.config.json`:
```json
{
  "hidden_size": 256,
  "num_layers": 5,
  "num_heads": 4,
  "max_seq_len": 128,
  "num_experts": 12
}
```

`num_layers` is updated in-place by the `EvolutionManager` when Net2Net surgery fires. The training binary reads this file on each `train_cycle` start, so the growing architecture is always picked up correctly.

---

## Version History

| Version | Architecture | Key Event | Global Epoch |
|---------|-------------|-----------|--------------|
| v2.0.0 · 5L | 256H · **5L** · 4H · 12E | Net2Net surgery 4L→5L; TTL routing + anti-stagnation burst; lb_lambda=0.03 | 381→current |
| v2.0.0 · 4L | 256H · 4L · 4H · 12E | 256H upgrade; TTL routing (TARGET fix); LB loss λ 0.01→0.02→0.03 | ~340–381 |
| v1.3.6 | 96H · 3L · 8E | Stable foundation; STE + top-3 MoE routing | ~200–340 |
| v1.3.5 | 96H · 3L · 8E | Initial 13-node MoE | 0–200 |

Archived snapshots in `models/registry/` include config, report, and evolution metadata.

---

## Surgery Log

| Event | Global Epoch | From | To | Notes |
|-------|-------------|------|----|-------|
| 4L → 5L | ~381 | 256H · 4L | 256H · 5L | Net2Net layer copy + σ=0.01 symmetry break; new loss floor 6.328 |
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

**Architecture change (e.g. surgery):** new layer weights are copied from the deepest existing layer before saving. On next load, all shapes match — 290 tensors for 5L.

---

## Serialization Format

Checkpoints use [SafeTensors](https://github.com/huggingface/safetensors) — header-based, memory-mapped, cross-framework compatible. Weights stored in float32 for training; ternary quantization applied at runtime via STE.

Future: `.trit` packed format (5 trits/byte) for hardware deployment.

---

## See Also

- [Main README](../README.md)
- [Benchmark Suite](../bench/install.sh) — one-line installer, exports CSV
- [Architecture](../docs/architecture.md)
