# Ternary Compression & Weight Adaptation

> **Current state (2026-05-15):** albert. v3.0 is a 17L ternary MoE trained natively from scratch via STE — not via post-training quantization. The adaptation pipeline described below applies to external models. albert.'s own weights are ternary throughout training; no quantization step is needed. Measured inference: 84.4 tok/s on HP ZBook i7-4800MQ (CPU-only) via @sparseskip + ternary weight cache + KV-cache.

## The Efficiency Challenge
Large Language Models (LLMs) in the 20B–30B parameter range typically require 40GB–60GB of VRAM for inference (at 16-bit precision). This creates a "Memory Wall" that prevents local-first, sovereign deployment on standard hardware.

## The Ternary Solution
By mapping weights into `{-1, 0, +1}`, we achieve massive compression:
*   **Storage Density**: 5 trits per byte (99.06% efficiency) via ternary packing.
*   **Memory Footprint**: Collapse from ~52 GB to **10–15 GB**.
*   **Throughput**: Skipping the zero-state (0) allows for hardware-level sparsity gains.

## The Adaptation Pipeline (Native Adaptation)
Unlike simple post-training quantization (PTQ) which often destroys model coherence, Albert-MoE-13 uses **Quantization-Aware Adaptation**:
1.  **Signal Normalization**: Aligning the base model weights to a zero-centered distribution.
2.  **Straight-Through Estimation (STE)**: Propagating gradients through the discrete ternary threshold during fine-tuning.
3.  **Structural Pruning**: Adapting the sparsity mask to align with the hardware's SIMD alignment (e.g., AVX-512).

## Comparison

| Metric | Binary (f16) | Ternary (Albert) |
|--------|--------------|------------------|
| Weight States | Continuous | {-1, 0, +1} |
| Memory (30B) | ~60 GB | ~12 GB |
| Sparsity Skip | 0% | 50–70% |
| Determinism | Probabilistic | Triadic-Deterministic |
