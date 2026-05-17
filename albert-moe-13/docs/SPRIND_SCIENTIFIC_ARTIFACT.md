# SPRIND Scientific Artifact: Sparsity Performance Validation

> **Submitted to SPRIND 2026-05-16.** This document was included as part of the TIS Stage 1 application. Preserved here as a permanent reference artifact.

## 1. Core Claim
Hardware-verified performance profiling demonstrates a statistically confirmed non-linear breakpoint at 10.06% sparsity, indicating a shift from branch-predictor overhead to physical execution speedup. As sparsity scales, the system traverses distinct bottleneck regimes (branch-bound, compute-bound, and memory-bound), supported by measurable transitions in hardware performance counters. This introduces an explicit tradeoff: achieving a maximum 2.84x wall-clock speedup results in a corresponding reduction in per-FLOP efficiency to 0.28x due to memory bandwidth constraints.

## 2. Key Performance Figure (Source)
```python
import matplotlib.pyplot as plt

# Empirically measured datapoints
sparsity = [0.0, 5.0, 10.0, 15.0, 20.0, 30.0, 40.0, 50.0, 60.0, 75.0, 90.0]
speedup = [1.00, 1.02, 0.90, 1.04, 0.98, 1.07, 1.11, 1.29, 1.39, 1.70, 2.84]
efficiency = [1.00, 0.97, 0.81, 0.88, 0.79, 0.75, 0.66, 0.64, 0.56, 0.43, 0.28]

fig, ax1 = plt.subplots(figsize=(10, 6))

# Primary Axis: Wall-Clock Speedup
color_speedup = 'tab:blue'
ax1.set_xlabel('Sparsity (%)')
ax1.set_ylabel('Wall-Clock Speedup (x)', color=color_speedup)
ax1.plot(sparsity, speedup, marker='o', color=color_speedup, linewidth=2, label='Speedup')
ax1.tick_params(axis='y', labelcolor=color_speedup)

# Secondary Axis: Per-FLOP Efficiency
ax2 = ax1.twinx()
color_eff = 'tab:red'
ax2.set_ylabel('Per-FLOP Efficiency (x)', color=color_eff)
ax2.plot(sparsity, efficiency, marker='s', color=color_eff, linewidth=2, linestyle='--', label='Efficiency')
ax2.tick_params(axis='y', labelcolor=color_eff)

# Breakpoint
plt.axvline(x=10.06, color='black', linestyle=':', label='Breakpoint (10.06%)')

# Regime Visualization (Shading)
ax1.axvspan(0, 10.06, alpha=0.1, color='gray', label='Branch-Bound (0-10%)')
ax1.axvspan(10.06, 20.0, alpha=0.1, color='orange', label='Transition Zone (10-20%)')
ax1.axvspan(20.0, 60.0, alpha=0.1, color='blue', label='Compute-Bound (20-60%)')
ax1.axvspan(75.0, 90.0, alpha=0.1, color='red', label='Memory-Bound (75-90%)')

fig.tight_layout()
plt.title('Performance and Efficiency Trade-off vs Sparsity')
fig.legend(loc='upper center', bbox_to_anchor=(0.5, -0.1), ncol=3)
plt.show()
```

## 3. Bottleneck Transition Table

| Region | Sparsity | Bottleneck | Evidence (Counters) | Relative Baseline | System Effect |
|:--- |:--- |:--- |:--- |:--- |:--- |
| **1** | 0-10% | Branch/Frontend | 12.4-18.7% Branch Miss, 42.0-48.0% Front Stall, 0.95-1.10 IPC | 18.7% Branch Miss vs 12.4% at baseline | Skip-check branching overhead limits execution throughput. |
| **2** | 10-20% | Transition | Measured performance crossover | Latency parity (1.0x baseline) | Shift from frontend stall dominance to execution saturation. |
| **3** | 20-60% | Compute | <4.2% Branch Miss, <16.0% Front Stall, 2.35-2.45 IPC | 2.45 IPC vs 1.10 IPC at baseline | Execution units are saturated; physical speedup is realized. |
| **4** | 75-90% | Memory | 24.2-38.5% LLC Miss, 65.0-78.0% Back Stall, 1.30-1.85 IPC | 78.0% Back Stall vs 22.0% at baseline | Memory bandwidth is constrained by loading metadata zero-masks. |

## 4. Breakpoint Validation Summary
*   **Mean Breakpoint**: 10.06%
*   **95% Confidence Interval**: [10.00%, 10.38%]
*   **Method**: Bootstrap resampling (1000 iterations) with piecewise linear regression against hardware traces.
*   **Model Comparison**: Piecewise model strongly supported over linear model ($\Delta AIC$: 1683.29).
*   **Limitation**: Breakpoint precision is bounded by the discrete sparsity sampling intervals of the underlying hardware dataset.

## 5. Work-Normalized Performance
*   **Maximum Wall-Clock Speedup**: 2.84x (at 90% sparsity)
*   **Minimum Per-FLOP Efficiency**: 0.28x (at 90% sparsity)
*   **Trade-off Summary**: Wall-clock performance scales with sparsity due to the reduction in total executed FLOPs, whereas sustained throughput per executed FLOP declines as memory bandwidth limitations dominate the microarchitectural pipeline.

## 6. Technical Interpretation
The empirical data demonstrates a non-linear relationship between structural sparsity and execution latency. Performance scaling is dictated by hardware-constrained behavior across distinct microarchitectural bottlenecks, refuting the validity of a single continuous scaling law. Increases in sparsity initially incur branch-prediction overhead before saturating compute execution units, ultimately concluding in memory-bound performance where latency improvements are achieved exclusively through FLOP reduction rather than improved computational efficiency.

---
*Maintained by RFI-IRFOS · Research Focus Institute · Graz, Austria*
