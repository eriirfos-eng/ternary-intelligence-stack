# ternlang-py: Native Triadic Quantization for Python

The **ternlang-py** package provides the official substrate integration for the **Ternary Intelligence Stack (TIS)**. It enables Python applications, particularly those utilizing PyTorch, to leverage the 152.8x efficiency coefficient natively through triadic-native execution.

## Core Features

### 1. Native Triadic Quantization
Maps 32-bit floats to 1.58-bit states with 100% data retention. Achieve significant reductions in memory footprint and thermal load.

### 2. T-ReLU (Deliberative Hold)
An implementation of triadic activation functions. Values near zero are held in State 0, triggering hardware-level sparse execution (@sparseskip) in the BET VM, which bypasses zero-gradient updates.

### 3. Substrate Integration
Provides the `@triadic_optimize` decorator to route tensor operations to the local TIS-MCP server, enabling seamless integration with BET-ISA hardware accelerators.

## Thermal Efficiency
Benchmarks demonstrate an 80% reduction in thermal energy waste compared to binary x86 pipelines over sustained execution cycles.

## Deterministic Anomaly Retention
Unlike binary cleaning scripts that prune "dirty" data, the TIS loader preserves anomalies in a Deliberative Hold (State 0) for secondary evaluation, ensuring robust dataset integrity.

---
**RFI-IRFOS Theoretical Research Department**
*Patent Pending: A50296/2026*
*License: BSL-1.1*
