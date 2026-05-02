# MoE-13: Scalable Ternary Mixture-of-Experts

**Fundamental research into the physics of ternary-native neural scaling.**

MoE-13 is an experimental framework for training and evaluating **Balanced Ternary Neural Networks**. By moving beyond binary logic and float-based approximations, we explore the mathematical scaling laws of ternary representation, where parameters exist in the {-1, 0, +1} manifold.

---

## The Research Frontier: Ternary Scaling Laws

Current AI scales by increasing precision and parameter counts, leading to massive energy and memory requirements. MoE-13 tests the hypothesis that **ternary-native networks scale more efficiently** by leveraging the inherent sparse geometry of balanced ternary weight spaces.

### Key Research Focus
- **Scaling Dynamics:** How loss scales with parameter count ($N$) when weights are constrained to ternary manifolds.
- **Manifold Stability:** Analyzing the gradient flow through ternary thresholds (Straight-Through Estimators).
- **Expert Specialization:** Quantifying how independent ternary expert paths partition parameter space during training.
- **Emergence via Routing:** Investigating how deterministic routing influences the learned internal representation of experts.

---

## Technical Architecture

MoE-13 is built for empirical scaling analysis:

1.  **Ternary Mapping Layer (TML)**: Native mapping of float-gradients into the ternary manifold.
2.  **ExpertBank13**: 13 independent expert domains, allowing for fine-grained analysis of parameter partitioning.
3.  **Routing Dynamics**: Analyzes how routing scores (gate weights) evolve as models scale, providing insights into the emergence of specialized pathways.
4.  **Hardware-Agnostic Kernels**: Designed for future silicon that can natively process the ternary manifold.

---

## Benchmarks & Reproducibility

We track model health through explicit scaling metrics:

| Metric | Goal |
|--------|------|
| **Loss/Parameter Scaling ($N$)** | Demonstrate Power-Law convergence at ternary scale. |
| **QAT Convergence** | Measure the rate of loss recovery post-ternarization. |
| **Expert Partitioning Efficiency** | Track the "divergence" of experts as training proceeds. |

*See `BENCHMARKS.md` for real-time scaling data and loss curves.*

---

## Research Ecosystem

- **`moe-core`**: The engine. Handles ternary matmuls, expert domain scoring, and the routing pipeline.
- **`moe-runtime`**: Manages the training and inference execution graph.
- **`moe-validation-suite`**: Harnesses for behavioral validation, allowing for controlled testing of emergent phenomena in routed paths.

---

## Quick Start

### 1. Run Baseline Scaling Benchmarks
```bash
# Verify the ternary scaling throughput
cargo run --release --bin bench_moe -p moe-core
```

### 2. Inspect Training Convergence
```bash
# Analyze loss/perplexity degradation
cd ../ternlang-root
cargo run --release --bin perplexity_eval -p ternlang-ml
```

---

## Architectural Principles

**Precision is not a prerequisite for intelligence.**  
We prioritize ternary manifold stability and representation efficiency.

*   **Deterministic Routing**: We use structured routing to isolate the effects of scaling from the "black-box" uncertainty found in standard MoE.
*   **Ternary Manifold**: weights $\in \{-1, 0, 1\}$.
*   **Parameter Partitioning**: Experts specialize based on input manifolds, not via stochastic emergence.

---

## Contribute

This is an open research project. We invite collaborative research on ternary scaling, loss landscape analysis, and ternary-native optimization strategies.

- **Foundational Research**: Deep-dive into `moe-core/src/training`.
- **Scaling Analysis**: Analyze the benchmark logs and propose new scaling dimensions.

---

## License

MIT — Build sovereign, scalable AI.

---

**Built by RFI-IRFOS for the Ternary Intelligence Stack.**
