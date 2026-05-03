# Albert-MoE-13: Compute Substrate Validation & Tensor Runtime Design

## 1. Compute Substrate Definition

### 1.1 What is a "tensor" in this system?
- Data Structure: A multidimensional array of fixed-point or floating-point values.
- Memory Layout: Contiguous `Vec<f32>` (for FP32 training) or bit-packed `u8` arrays (for ternary inference), aligned for SIMD access.
- Mutability: Immutable during forward passes; mutable during gradient accumulation and optimizer step phases.

### 1.2 What executes tensor operations?
- Current: Non-existent (currently hash-based symbolic logic).
- Proposed: `candle` or `burn` backend. These frameworks are Rust-native, offer efficient CPU/CUDA kernels, and support autograd.

### 1.3 Where does computation actually occur?
- CPU (Current fallback): Executes via `burn`/`candle` CPU backends using AVX2/FMA instructions.
- GPU (Target): Executes via CUDA/NCCL kernels for multi-expert parallel training.

---

## 2. Current vs. Required Gap

| Capability | Current State | Required State |
|------------|--------------|----------------|
| Matrix multiply | Non-existent | High-perf BLAS/SIMD kernels |
| Backpropagation | Non-existent | Autograd DAG graph |
| Embedding lookup | Non-existent | Indexed matrix slice |
| Gradient storage | Non-existent | Buffer of identical shape to params |
| Optimizer step | Non-existent | AdamW implementation |

---

## 3. Minimal Tensor Backend Proposal
- Backend: `candle`. It is lightweight, production-ready, and allows for custom ops (critical for ternary-native arithmetic).
- Component Stack:
    - Tensors: `candle_core::Tensor`.
    - Optimization: `candle_nn::AdamW`.
    - Autograd: `candle_nn::VarBuilder` for state management.

---

## 4. Failure Mode Analysis (Why it breaks without this layer)
- Silent Failures: The training loop will appear to "run" while weights remain static because there is no compute gradient to update parameters.
- Fake Training Loops: Without an autograd DAG, the "training" will be a loop of read-only ops.
- Deterministic Collapse: The model will remain trapped in the initial weight state, giving the illusion of "no learning" due to symbolic logic overriding parameter updates.

---

## 5. Architectural Separation
1. CLI Layer (`albert-test`): User interaction, REPL, audit execution, system bootstrapping.
2. Orchestration Layer: Manages the training loop (epochs, dataset loading) and calls the compute layer.
3. Compute Layer (NEW: `albert-compute`): The tensor backend (`candle` wrapper). Handles ops, gradients, and device placement.
4. Model Layer (`albert-llm-core`): Defines the transformer topology (layers, dimensions, routing).
