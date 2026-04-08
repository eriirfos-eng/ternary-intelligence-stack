# ARI-v1.0: Ambiguity Resolution Index

**Status**: DRAFT  
**Author**: RFI-IRFOS  
**Objective**: Empirically measure the efficiency of non-binary systems in resolving informational conflict.

## 1. The Core Metric: TRCE (Trit-Resolution Cost Efficiency)
Binary benchmarks measure accuracy in a single turn. ARI measures the cost to reach a **Stable State (+1 or -1)** from an **Ambiguous Input (0)**.

$$ARI = \frac{\Delta \text{Certainty}}{\text{Total Energy (Joules)} \times \text{Turns}}$$

## 2. Evaluation Criteria
### A. False Certainty Penalty (FCP)
Binary models are penalized for providing a confident but incorrect answer to an ambiguous prompt.
- **TIS Advantage**: Using State-0 (HOLD) to request clarifying context yields a 0 penalty.

### B. State-0 Pruning Efficiency
Measures the reduction in gate-level switching during the deliberation phase.
- **Benchmark Task**: Given a conflicting input vector, quantify the power saved by executing State-0 (HOLD) instructions instead of speculative binary branches.

### C. Multi-Turn Calibration
Evaluates the model's ability to minimize the "Entropy of Intent" over 5-10 turns of context gathering.

## 3. Reference Tasks
- **Conflict Resolution**: Resolving mutually exclusive sensor data in a simulated triadic HAL.
- **Sparse Inference**: Quantifying the speedup of 1.58-bit (BitNet) kernels using native TIS triadic opcodes.
- **Safety Veto**: Measuring the latency of an MoE-13 safety override during a simulated system panic.

---
**Standard Authority: RFI-IRFOS**
