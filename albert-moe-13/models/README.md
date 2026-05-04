# Albert-MoE-13 Model Artifacts

This directory contains the production checkpoints for the Copernicus model series.

## Artifact Formats

1.  **`.trit` (Native Ternary Packed)**:
    - **Description**: The primary research artifact. Weights are stored using **ExaTern (5 trits per 8-bit block)** packing.
    - **Efficiency**: ~99% triadic storage efficiency.
    - **Usage**: Load directly using `moe_llm_core::model::loader::TritLoader`.

2.  **`.safetensors` (Reference/Training)**:
    - **Description**: Standard float32 representations of the weights before/during quantization. Used for fine-tuning and cross-platform validation.
    - **Usage**: Load using `candle_core::safetensors`.

## Model Registry

| Version | ID | Epochs | Loss | Format | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **v1.3.5** | `bible-dense-v1` | 20 | 5.8799 | `.trit`, `.safetensors` | [VERIFIED] |
| **v1.3.6** | `bible-pos-v1` | 2 | 7.8845 | `.trit`, `.safetensors` | [VERIFIED] |
| **v1.3.7** | `bible-moe-v1` | 50 (In Progress) | ~10.0 (Current) | `.safetensors` | [EXPERIMENTAL] |

---
**Verified by Reproducibility Verifier v1.3.5**
