//! # ALBERT-MoE-13 Checkpoint Specification (v0.1)
//! 
//! This document defines the serialized state format used by `albert-run`
//! to load trained model snapshots into memory.

# 1. Header Schema (Metadata)
- `magic_bytes`: `0x41 0x4C 0x42 0x54` ("ALBT")
- `version`: `u32` (Major.Minor)
- `model_id`: `String` (e.g., "copernicus-v1")
- `embedding_dim`: `u32`
- `expert_count`: `u32`
- `threshold`: `f32` (STE threshold)

# 2. Binary Payload (Weights & Routing)
- **Trit-Stream**: Ternary weights stored as bit-packed 2-bit values per trit (00: -1, 01: 0, 10: 1).
- **Expert-Router**: Weight matrix $W_r \in \mathbb{R}^{dim \times num\_experts}$ (FP32).
- **Expert-Weights**: Concatenated weights for all 13 experts (bit-packed trits).

# 3. Directory Structure
`models/registry/{model_id}/`
├── `snapshot.bin` (Binary Payload + Header)
├── `metadata.json` (Runtime config, vocab mapping)
└── `routing_stats.json` (Expert activation history)
