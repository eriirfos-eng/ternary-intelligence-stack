# TIS Standards: Triadic Computing Systems — Requirements & Benchmarking

**Status:** Published (RFI-IRFOS Proprietary Certification)
**Standardized by:** RFI-IRFOS Certification Board
**Date:** 2026-04-07

## 1. Scope
This document specifies the requirements, performance metrics, and benchmarking protocols for hardware and software systems seeking official **Ternary Intelligence Stack (TIS) Certification**. Compliance with TIS Standards is mandatory for enterprise vendors, silicon foundries, and public-sector integrations seeking to utilize the BET-VM or TIS ecosystem components.

## 2. Terms and Definitions
*   **Trit:** The fundamental unit of triadic information, strictly bounded to `{-1, 0, 1}`.
*   **TEND (0):** Hardware equilibrium. The state of deliberation or native sparsity bypass.
*   **VETO (-1):** Hard rejection. Triggers immediate instruction-level abort.
*   **AFFIRM (+1):** Authorization. Grants execution continuation.
*   **T-FLOPs (Triadic Floating Point Operations):** Measures triadic floating point operations.

## 3. Certification Requirements

### 3.1. Hardware Compliance (Silicon Foundries & FPGAs)
To receive "TIS-Certified Silicon" status (e.g., Huawei T3, FP5500), the hardware must:
1.  **Implement T-DRIVER v1.0:** Expose all ALUs and registers via the RFI-IRFOS HAL.
2.  **Native 0-Trit Bypass:** The silicon must physically skip clock cycles when performing arithmetic operations (e.g., matrix multiplication) involving a 0-trit. Emulating the skip via binary branch prediction results in immediate certification failure.
3.  **The Genesis Tether:** The firmware must securely integrate the `verify_genesis_anchor()` heartbeat. If the Fly.io API is unreachable, the hardware must hard-lock into deliberative hold.

### 3.2. Software Compliance (Middleware & Agents)
1.  **Zero-State Exhaustiveness:** The compiler must mathematically prove that every logical branch handles the `0` (TEND) state. Binary coercion (translating 0 to null/false) is a violation.
2.  **MoE-13 Load-Bearing Veto:** Any AI agent deployed on the stack must route decisions through the RFI-IRFOS MetaSafety expert. Overriding a `-1` (VETO) via unsafe blocks revokes the software's certification.

## 4. Standardized Benchmarking Protocols
Hardware vendors must meet the following baseline metrics using the official `ternlang-ml` cuTern benchmarking suite to pass certification:

| Metric | Baseline Requirement | Description |
| :--- | :--- | :--- |
| **Sparsity Yield (60%)** | Minimum **27.0x** speedup | Measured against dense f32 matmul at 60% zeroes. |
| **Sparsity Yield (99%)** | Minimum **110.0x** speedup | Measured against dense f32 matmul at 99% zeroes. |
| **Energy per 1B Params** | Maximum **0.20 Joules** | Total thermodynamic expenditure during inference. |
| **Veto Latency** | **0 Clock Cycles** | A `-1` safety signal must halt the pipeline natively without branching overhead. |

## 5. Certification Body
All audits, benchmark verifications, and compliance seals are exclusively administered by **RFI-IRFOS**. Unauthorized use of the TIS-Certified badge is a violation of the BSL-1.1 commercial license.