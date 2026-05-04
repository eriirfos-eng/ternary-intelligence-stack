# Albert-MoE-13: Ternary-Native Research Framework

This repository provides the computational framework for balanced ternary neural networks (weights $\in \{-1, 0, 1\}$).

## 1. Technical Specifications: Ternary Scaling
The framework implements training on a ternary manifold via Straight-Through Estimation (STE).

### Forward Pass
$$f(x) = \text{sign}(x) \cdot \mathbb{I}(|x| > \tau), \quad \tau \in \mathbb{R}^+$$
Weights are constrained to the ternary set $\{-1, 0, 1\}$.

### Backward Pass
$$\frac{\partial L}{\partial x} \approx \frac{\partial L}{\partial y} \cdot \mathbb{I}(|x| \leq \tau)$$
Gradients are bounded by the threshold $\tau$.

## 2. Distributed Architecture
The implementation supports expert-parallel distribution:
*   **Expert Partitioning**: MoE layers are distributed across compute nodes.
*   **Data Ingestion**: Zero-copy pipeline converts input streams to ternary trit-streams.
*   **Synchronizer**: Orchestration layer performs gradient averaging via bit-masking operations.
*   **Persistence**: Bit-packed weights (5 trits/byte) are serialized for state consistency.

## 3. Computational Kernel
*   **SIMD Optimization**: AVX2/AVX-512 sparse ternary matmul kernels utilize the `@sparseskip` operation to omit computation for null (0) trits.
*   **Diagnostic Suite**: Weight migration telemetry is logged to quantify convergence within the ternary manifold.

## 4. Operational Workflow

### 4.1. Execution
The system uses an orchestrator binary to manage the training process and the local dashboard server.
```bash
albert-train
```
Telemetry data (loss/epoch) is streamed to `Desktop/training_log/training.log`.

### 4.2. Validation
Test suites are executed via the platform binary:
```bash
cargo run --bin moe-test
```

## 5. Artifact Registry
Refer to [models/README.md](./models/README.md) for serialized artifact provenance and integrity protocols.


