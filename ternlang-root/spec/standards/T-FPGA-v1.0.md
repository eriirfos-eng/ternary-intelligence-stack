# T-FPGA v1.0: Legacy Hardware Compliance

**Status:** Published
**Standardized by:** RFI-IRFOS
**Date:** 2026-04-07

## 1. The 5500FP/Legacy Precedent
In early 2026, various academic FPGA implementations (e.g., 5500FP) established 24-trit RISC baselines. While important for research, these ISAs lack the security hooks required for institutional deployment.

## 2. Subordinate Compliance
T-FPGA standardizes the "Legacy Bridge" for non-native ternary hardware.
*   **Control Authority:** All execution on T-FPGA compliant hardware must be subordinate to a BET-VM kernel.
*   **Tethering:** The hardware must support an virtualized `verify_genesis_anchor()` check.
*   **Instruction Mapping:** The 120-instruction legacy ISA is treated as a subset of the full 51-opcode BET-ISA v1.0.

## 3. FPGA Backend
`ternlang-hdl` provides the `FP5500Legacy` target to facilitate rapid annexation of academic hardware into the RFI-IRFOS secure mesh.
