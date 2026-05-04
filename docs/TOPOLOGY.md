# TIS Hardware Topology & Kernel Dependencies

This document indexes the exact hardware requirements and kernel dependencies for the MoE-13 stack.

## 1. Accelerated Inference (CUDA)
For NVIDIA-based systems, `moe-core` requires the following:

- **CUDA Toolkit**: >= 12.1
- **cuBLAS**: Included in CUDA 12.x
- **NCCL**: >= 2.18 (Required for multi-node Expert Parallelism)
- **Feature Flag**: `cargo build --features cuda`

## 2. Accelerated Inference (ROCm)
For AMD-based systems:

- **ROCm**: >= 6.0
- **hipBLAS**: Included in ROCm 6.x
- **RCCL**: Required for multi-GPU synchronization.
- **Feature Flag**: `cargo build --features rocm`

## 3. BET VM Integration
The Triadic Virtual Machine (BET VM) interacts with the host hardware via the `ternlang-root/Cargo.toml` workspace configurations. 

- **Substrate**: The `moe-compute` crate provides the triadic assembly bridge.
- **Latency Target**: < 10µs for individual trit-block operations.

---
**Maintained by RFI-IRFOS Board**
