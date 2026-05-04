# Albert-MoE-13: Artifact Provenance & Registry

This documentation details the production checkpoints, artifact serialization, and provenance chain for the Copernicus model series (MoE-13).

## 1. Artifact Lifecycle & Serialization
Each model version generates three distinct artifact types to ensure research reproducibility and production-ready performance.

| Extension | Purpose | Technical Spec |
| :--- | :--- | :--- |
| `.safetensors` | Training & Interop | Uncompressed float32/fp16 weights for compatibility with standard ML frameworks. |
| `.trit` | Native Production | **ExaTern Packing**: 5 trits per 8-bit block. Used for high-speed inference. |
| `.meta` | State Metadata | JSON-encoded configuration (hyperparameters, threshold $\tau$, epoch count). |

## 2. Provenance Chain
All artifacts are tracked via the **Reproducibility Verifier**. A model checkpoint is considered `[VERIFIED]` only after passing the core integrity suite:
1. **Weight Mapping**: Symmetry between `.safetensors` reference and `.trit` packed state.
2. **Convergence Verification**: Loss consistency check against the training telemetry (captured in `training.log`).
3. **Hardware Compatibility**: Verification against the AVX-512 SIMD kernels and ternary-native hardware simulation.

## 3. Model Registry

| Version | ID | Epochs | Loss | Status | Key Artifacts |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **v1.3.5** | `bible-dense-v1` | 20 | 5.8799 | [VERIFIED] | Base dense model, primary ternary threshold baseline. |
| **v1.3.6** | `bible-pos-v1` | 170+ | 1.1245 | [STABLE] | MoE routing improvements, positional encoding stability. |
| **v1.3.7** | `bible-moe-v1` | 175 (Live) | ~1.1032 | [CONVERGING] | Live-training artifact with real-time weight streaming. |

## 4. Operational Guidelines
- **Continuous Deployment**: v1.3.7+ artifacts are updated continuously by the `albert-train` orchestrator.
- **Artifact Retention**: Production-stable checkpoints (`v1.3.6`) are immutable. Experimental checkpoints are rotated.
- **Recovery**: Use the latest `.meta` file to restore the exact state of the `RobustHandler` dashboard if the orchestrator is restarted.

---
*For questions regarding artifact provenance, refer to the main repository `docs/` folder or the `training_lab/` source.*
