# albert. — Parameter Count & On-Disk Footprint (ground truth)

**Measured 2026-05-31 from `models/albert_v3.0.best.safetensors` (28L dual-stream checkpoint).**
All values are exact reads from the safetensors header, not estimates. This document is the
canonical source for footprint / bits-per-param numbers — keep it in sync after each surgery.

## Parameter count

| Quantity | Value |
|----------|-------|
| **Total parameters (exact)** | **207,976,972** |
| Ternary matmul weights (packable) | 191,019,008 (91.84%) |
| f32 embeddings (`embed.weight` 32000×256 + `lm_head.weight` 32000×256, untied) | 16,384,000 |
| f32 norms / biases / gates | 573,964 |
| Tensor count | 2,200 |
| dtype on disk | F32 (all tensors) |

## On-disk sizes

| Artifact | Bytes | MB |
|----------|------:|---:|
| Training safetensors (`albert_v3.0.safetensors` = `albert_v3.0.best.safetensors`, byte-identical) | 832,133,656 | 793.58 |
| **Deployable packed checkpoint (28L)** | **not yet generated** | — |
| Stale 21L pack (`albert_v3.0_21L-256H-12E.trit`) — WRONG ARCH, do not use | 60,862,252 | 58.04 |

> There is currently **no packed 28L `.trit` on disk.** The only `.trit` artifact is the stale 21L
> export from 2026-05-25. Regenerate with the export pipeline below before quoting a real packed byte count.

## Packing scheme

**5 trits per byte.** Each `u8` holds 5 base-3 digits (`3^5 = 243 ≤ 256`), so the density is exactly
**8 / 5 = 1.600 bits per trit**. Quantization threshold `0.05`.

- Pack function: `moe-llm-core/src/model/packing.rs:4` — `pack_tensor()`, 5-trit loop at lines 13–21.
- Exporter: `moe-llm-core/src/bin/quantize_model.rs:46` (binary `quantize_model`).
- **Embeddings, pos-embed, and layernorm tensors are kept raw f32** — `quantize_model.rs:35-42`.
  Only matmul weights are ternary-packed.

## Bits-per-param — discrepancy resolved

There are two legitimate numbers, and conflating them is the source of the discrepancy:

| Figure | Packed bytes | MB | bits/param | What it covers |
|--------|-------------:|---:|-----------:|----------------|
| **Pure-ternary (headline)** | 41,595,395 | 39.67 | **1.600** | *Only* the 191M matmul weights, 5-trit packed. Assumes embeddings are also packed — **the exporter does not do this.** |
| **Realistic deployable** | 106,035,658 | 101.12 | **4.079** | Ternary weights 5-trit packed (38,203,802 B / 36.43 MB) **+ f32 embeddings & norms kept raw (67,831,856 B / 64.69 MB)**. This is what `quantize_model` actually produces. |

`bits_per_param = packed_bytes × 8 / 207,976,972`.

**Bottom line:** the "1.6-bit" claim is true for the ternary matmul weights, which are 91.8% of params.
But the 16.4M f32 embedding params (input embed + untied lm_head) occupy ~64.7 MB and dominate the
packed file, pushing the *effective* footprint to **~101 MB / 4.08 bits per parameter**. To realize the
true 1.6-bit footprint (~39.7 MB), the embeddings must also be ternary-packed — a known, deliberate
exporter limitation, not a measurement error.

## Export pipeline (regenerate the packed checkpoint)

```bash
cargo build --release -p moe-llm-core --bin quantize_model
./target/release/quantize_model \
  models/albert_v3.0.best.safetensors \
  models/albert_v3.0_28L-256H-12E.trit
```
