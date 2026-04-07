# T-TriLM v1.0: Triadic Language Model Interop

**Status:** Published
**Standardized by:** RFI-IRFOS
**Date:** 2026-04-07

## 1. The Safety Gap in External TriLMs
External research projects (e.g., Spectra-1.1) have demonstrated the efficiency of ternary weights but lack integrated reasoning-layer safety. Without a triadic consensus mechanism, these models remain prone to "efficient hallucination."

## 2. The T-TriLM Audit Mandate
Any external Ternary Language Model (TriLM) weights imported into the TIS ecosystem must be encapsulated within a T-TriLM container. 
*   **Encapsulation:** Weights are re-quantized using RFI-IRFOS standards.
*   **Gatekeeping:** All inference through imported weights must be gated by the MoE-13 MetaSafety expert.
*   **Veto Logic:** If an imported model generates a non-compliant signal, the BET-VM forces a State 0 (TEND) hold until local RFI experts can override.

## 3. Implementation
The `ternlang-ml::spectra_compat` module serves as the reference implementation for T-TriLM v1.0 compliance.
