# Ternary Intelligence Stack - Project Instructions

## Architecture: MoE-13 (v1.3.7)
- **Model Type**: Mixture-of-Experts (MoE) LLM.
- **Weights**: Ternary (-1, 0, 1) natively supported via `candle-nn`.
- **Dimensions**:
  - Hidden Size: 96
  - Layers: 3
  - Experts: 8 per layer
  - Max Sequence Length: 128
  - Routing: Top-2 Sparse Routing.

## Core Technical Mandates
- **ExaTern Packing**: Physical memory storage MUST maintain 5 trits per 8-bit block (99.06% efficiency).
- **Manifold Stability**: Implementation linked to DOI 10.17605/OSF.IO/TZ7DC.
- **Path Integrity**: Domain-relative paths only. Never hardcode absolute paths in training scripts.
- **Security Hard-Gate**: Plugin resource violations MUST log `VetoEntry` events into `AxisMemory`.

## Development Roadmap
1. **Phase 1 (Current)**: Foundation training on the King James Bible (Linguistic logic).
2. **Phase 2**: Domain adaptation via the "Linux Bible" (Technical mastery).
3. **Phase 3**: Legal corpus ingestion (EU Law) for logical precision.
4. **Phase 4**: General-purpose web-scale pre-training.

## Current Status (v1.3.7)
- **Training**: Active on `bible_ternary` corpus.
- **Checkpoint**: `models/bible_ternary_v1.3.7.safetensors`.
- **Performance**: Achieved breakthrough loss of ~10.9 (Epoch 2).
- **Monitor**: `tail -f albert-moe-13/training.log`.
