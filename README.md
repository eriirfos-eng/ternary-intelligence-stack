# Ternary Intelligence Stack (TIS)

[![version](https://img.shields.io/badge/version-v1.2.9-blue)](#architecture)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-88%2B%20passing-brightgreen)](#architecture)
[![DOI](https://img.shields.io/badge/DOI-10.17605%2FOSF.IO%2FTZ7DC-informational)](https://doi.org/10.17605/OSF.IO/TZ7DC)

The Ternary Intelligence Stack (TIS) is a deep-tech research initiative exploring the **fundamental physics of natively trained ternary neural networks**. We move beyond the binary paradigm, providing empirical evidence that balanced ternary architectures ($\{-1, 0, +1\}$) offer a more efficient, stable, and scalable "S-curve" for the next generation of artificial intelligence.

Built by [RFI-IRFOS](https://ternlang.com) · Graz, Austria

---

## 1. What is Ternlang?

Ternlang is a research-grade language and runtime built on balanced ternary logic. Its core type, `trit`, enables **stable ternary manifold computation**, providing the native mathematical foundation required for efficient, large-scale ternary neural networks.

## 2. Research Focus: MoE-13

We are currently training **MoE-13**, a natively-quantized ternary architecture designed to demonstrate that model scaling follows predictable power-law convergence without floating-point requirements.

### Key Research Dimensions
- **Scaling Laws**: Demonstrating predictable loss decay as ternary parameter counts ($N$) scale from 1M to 1T.
- **Ternary Manifold Stability**: Proving that the $\{-1, 0, +1\}$ manifold is mathematically stable under gradient-based learning via Straight-Through Estimators (STE).
- **Native Training**: Moving away from post-training quantization to training natively on the ternary manifold from initialization.
- **Sparse Computational Physics**: Leveraging the sparse geometry of ternary weights for sub-linear memory scaling.

---

## Performance Benchmarks & Scaling Proofs

Our research artifacts are available for replication. We track health through empirical convergence metrics:

| Metric | Scientific Goal |
|--------|-----------------|
| **Loss vs. Parameter Scale ($N$)** | Demonstrate Power-Law convergence at ternary scale. |
| **Manifold Stability ($\alpha$)** | Track layer-wise signal amplitude across depth. |
| **Active Gradient Fraction** | Validate convergence via steady-state STE activity. |

*See [`BENCHMARKS.md`](ternlang-root/BENCHMARKS.md) for current empirical scaling data.*

---

## Repository layout

| Directory | Research Focus |
|-----------|----------------|
| [`ternlang-root/`](ternlang-root/) | Core research engine (compiler, VM, ternary ML kernels) |
| [`albert-moe-13/`](albert-moe-13/) | Experimental MoE-13 training & scaling framework |
| [`ternlang-ml/`](ternlang-root/ternlang-ml/) | Ternary-native training (QAT/STE) and scaling metrics |

---

## Getting Started

### Reproduce Scaling Experiments
```bash
# Verify ternary manifold scaling convergence
cd ternlang-root
cargo run --release --bin scaling_convergence_bench -p ternlang-ml
```

### Reproduce Sparse Compute Efficiency
```bash
cd albert-moe-13
cargo run --release --bin bench_moe -p moe-core
```

---

## License

MIT — Build sovereign, scalable AI.

---

<div align="center">
  <img src="ternlang-root/ternlang-web/assets/ternlang_logo_notext.png" alt="Ternlang Logo" width="100">
</div>
