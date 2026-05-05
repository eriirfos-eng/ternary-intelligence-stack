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
| Architecture | 256H · 3–12L (auto-evolving) · 4H · 12E · 128CTX |
| Corpus | Bible KJV + Alice in Wonderland (active); staged corpus ready |

The checkpoint is overwritten at the end of every 300-batch epoch. The `.meta` file stores the global epoch count across all training sessions so the odometer never resets.

---

## Config Schema

`bible_ternary_v2.0.0.config.json`:
```json
{
  "hidden_size": 256,
  "num_layers": 3,
  "num_heads": 4,
  "max_seq_len": 128,
  "num_experts": 12
}
```

`num_layers` is updated in-place by the `EvolutionManager` when Neural Surgery fires. Load code reads this file on each `train_cycle` start, so the growing architecture is always picked up correctly.

---

## Version History

| Version | Architecture | Key Event | Notes |
|---------|-------------|-----------|-------|
| v2.3 | 256H · 3L · 12E | 256H upgrade + L1 sparsity + per-layer threshold | Current |
| v2.2 | 128H · 3L · 12E | EvolutionManager, Net2Net surgery | Archived |
| v2.0 | 128H · 3L · 12E | 13-node MoE, top-3 routing | Archived |
| v1.3.7 | 96H · 3L · 8E | Stable Biblical Foundation | Registry: `models/registry/` |

Archived snapshots in `models/registry/` include config, report, and evolution metadata.

---

## Checkpoint Loading

The training binary loads the checkpoint with strict shape matching:

```rust
for (name, var) in all_vars.iter() {
    if let Some(tensor) = checkpoint_data.get(name) {
        if tensor.shape() == var.shape() {
            var.set(tensor)?;  // only loaded if shapes match exactly
        }
    }
}
```

**Architecture change (e.g. hidden_size):** shapes mismatch → all vars initialize fresh. Safe to change config without deleting the old checkpoint file — it is silently ignored.

**Layer addition (surgery):** new layer weights are copied from the deepest existing layer before saving. On next load, all shapes match.

---

## Serialization Format

Checkpoints use the [SafeTensors](https://github.com/huggingface/safetensors) format — header-based, memory-mapped, cross-framework compatible. Weights are stored in float32 for training precision; ternary quantization is applied at runtime during the forward pass.

Future: `.trit` packed format (5 trits/byte) for hardware deployment — not yet implemented.

---

## See Also

- [Main README](../README.md)
- [Architecture](../docs/architecture.md)
