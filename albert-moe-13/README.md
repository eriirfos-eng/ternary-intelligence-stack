# Albert-MoE-13: Ternary-Native Frontier Research Framework

**The foundational research framework for balanced ternary intelligence (weights $\in \{-1, 0, 1\}$).**

Albert-MoE-13 investigates the fundamental physics of ternary neural networks. By operating on a discrete ternary manifold, we enable **sub-linear memory scaling**, **auditable decision pathways**, and **energy-efficient inference** that bypasses the limitations of floating-point arithmetic.

---

## 1. Mathematical Foundation: Ternary-Native Scaling
Unlike binary quantization (where floats are downcasted), Albert-MoE-13 trains directly on a ternary manifold using **Straight-Through Estimation (STE)**.

### Forward Pass (Ternary Mapping)
$$f(x) = \text{sign}(x) \cdot \mathbb{I}(|x| > \tau), \quad \tau \in \mathbb{R}^+$$
*Weights are strictly locked to $\{-1, 0, 1\}$, where $0$ acts as a formal "HOLD" (uncertainty/deferral) state.*

### Backward Pass (Gradient Approximation)
$$\frac{\partial L}{\partial x} \approx \frac{\partial L}{\partial y} \cdot \mathbb{I}(|x| \leq \tau)$$
*Gradients are gated by the ternary threshold $\tau$, preserving signal amplitude stability ($\alpha \approx 0.55$).*

---

## 2. Distributed Training Architecture
To support 1T+ parameter regimes, Albert-MoE-13 implements a multi-node, expert-parallel architecture:

*   **Rank-Based Expert Partitioning**: The MoE layers are distributed across nodes, with each node owning a subset of the expert domain (e.g., Logic, Medical, Technical).
*   **Asynchronous Data Ingestion**: Zero-copy `DataPipeline` handles petabyte-scale streaming, converting raw text shards directly into ternary trit-streams.
*   **Ternary Synchronizer**: A dedicated orchestration layer (`DistributedOrchestrator`) manages gradient averaging across the discrete manifold, using bit-mask operations to minimize all-reduce latency.
*   **Checkpointing**: Sparse, bit-packed weight storage (5 trits/byte) ensures I/O-efficient persistence for massive model states.

---

## 3. High-Performance Execution Toolkit
Our research toolkit is built for hardware-level efficiency:

*   **SIMD Kernels**: AVX2/AVX-512 optimized sparse ternary matmul kernels, enabling the `@sparseskip` operation—skipping computation for all $0$ (HOLD) trits.
*   **Trit-Drift Diagnostics**: Precision logging to quantify weight migration within the ternary manifold, ensuring convergence stability.
*   **Bayesian Threshold Tuner**: An automated hyperparameter framework that optimizes STE thresholds ($\tau$) to maximize convergence and minimize gradient vanish in deep MoE layers.

---

## 4. Research Scope & Scaling Roadmap
| Phase | Goal | Focus |
|---|---|---|
| **Phase 1** | Stability Validation | Empirical convergence of small-scale MoE clusters |
| **Phase 2** | Expert Specialization | Learned routing accuracy across 13 epistemic domains |
| **Phase 3** | 1T Parameter Scaling | Benchmarking sub-linear memory growth on distributed clusters |
| **Phase 4** | Native HW Bridging | Implementation on ternary-native FPGA/QNN hardware |

---

## 5. Repository Integrity & Transparency
This project is an **Open-Core** research framework. All core training, orchestration, and ternary logic are fully open-source (LGPL-3.0/BSL-1.1).

*   **Crate Documentation**: Full documentation is provided for each core crate in `crates/moe-core/`.
*   **Reproducibility**: All experiments, including the convergence sweep and benchmark suite, are included in the repository.
*   **Compliance**: Built-in audit trails and hard-safety gating ensure compliance with EU AI Act frameworks.

---

## 6. Project Structure
To maintain a clean and professional research environment, artifacts are organized as follows:

*   `benchmarks/`: Python analysis scripts (`robust_analysis.py`), hardware counter collection tools, and raw CSV results.
*   `crates/`: Core Rust implementation of the MoE runtime and SIMD kernels.
*   `data/`: Sharded training corpora and test datasets.
*   `docs/`: Detailed architectural specs, roadmaps, and the verified **SPRIND Scientific Artifact**.
*   `models/`: Checkpoints and metadata for the Copernicus model series.

---

## 7. Unified Training & Dashboard Workflow

We have unified the training, monitoring, and dashboard orchestration under the `albert-train` command, which links to our Python orchestrator (`albert-run`).

### 7.1. Running the Production Stack
To start the full training suite (training binary + dashboard server + UI):

```bash
# From project root
albert-train
```

*   **Training Binary**: The system streams batch data directly to `/home/eri-irfos/Desktop/training_log/training.log`.
*   **Dashboard Server**: Automatically spins up a `RobustHandler` server on `http://localhost:8888`.
*   **UI Visualization**: Automatically launches a browser/window showing the real-time scatter plot with:
    *   **High-precision X-axis**: Global Epoch + (Batch/300).
    *   **Live Updates**: Turquoise drops represent batch loss, orange lines show the Simple Moving Average (SMA).
    *   **Data Cleanup**: Automatically filters loss < 1.1 for clear convergence visualization.

### 7.2. System Monitoring
*   **Live Logs**: View raw telemetry at `Desktop/training_log/training.log`.
*   **Model States**: Checkpoints are stored in `models/bible_ternary_v1.3.6.safetensors` and `models/bible_ternary_v1.3.6.meta`.
*   **Orchestration**: `albert-run` handles subprocess lifecycle management for the Rust binaries and the Python dashboard server.

### 7.3. Testing
To run the standard test suite:
```bash
cargo run --bin moe-test
```

