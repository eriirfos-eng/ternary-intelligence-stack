# RFI-IRFOS Global Ternary Index (TIS-v3.0)

Module and standards map for the Ternary Intelligence Stack. Cross-reference against individual crate READMEs and `docs/standards/` for specifications.

## 1. Hardware Targets (The Silicon Layer)

| Domain | Module | Standard | Description |
|--------|--------|----------|-------------|
| FPGA / ASIC | `ternlang-hdl` | BET-ISA-v2.0 | Native 2-bit trit synthesis |
| HFT Trading | `ternlang-hft` | T-LATENCY-v1.0 | Low-latency trading primitives |
| Drone RTOS | `ternlang-ros2` | T-AVOID-v1.0 | Deterministic collision avoidance (PX4) |
| Quantum | `ternlang-qutrit` | T-QUANT-v1.0 | 3-state qutrit compilation |
| BitNet-T3 | `ternlang-ml` (`ml::tuann`) | T-SPEC-v2.0 | 1.58-bit MatMul |

## 2. AI and Inference

| Domain | Module | Mechanism | Description |
|--------|--------|-----------|-------------|
| Agent CLI | `agent_albert_cli` | — | Offline-first albert. agent shell |
| Training | `albert-moe-13` | MoE Top-3 routing | Ternary MoE research model |
| ML Integration | `ternlang-ml` | T-WEIGHT-v1.0 | `torch_dispatch` sparse bypass |
| Wasm Inference | `ternlang-wasm` | T-EDGE-v1.0 | v128 SIMD browser inference |
| MoE Runtime | `ternlang-moe` | T-ROUTING-v1.0 | Deterministic kernel dispatch |

## 3. Core Library

| Domain | Module | Standard | Description |
|--------|--------|----------|-------------|
| Networking | `ternlang-net` | T-NET-v1.0 | Triadic congestion control |
| Orbital/GEO | `ternlang-astro` | T-ASTRO-v1.0 | Delay-tolerant routing |
| Filesystem | `ternlang-fs` | T-FS-v1.0 | State-0 memory block isolation |
| Serialization | `ternlang-tson` | TSON-v1.0 | Triadic encoding format |
| POSIX Interface | `ternlang-posix` | T-POSIX | Edge/RTOS scheduler |
| Education | `ternlang-edu` | T-DEV-v1.0 | Language tutorials and tooling |

## 4. Platform and Compliance

| Domain | Module | Mechanism | Description |
|--------|--------|-----------|-------------|
| Public API | `ternlang-api` | REST | ternlang.com hosted inference API |
| Smart Contracts | `ternlang-contract` | T-HOLD-v1.0 | Triadic smart contracts |
| Consensus | `ternlang-consensus` | T-PROOF-v1.0 | Proof-of-ambiguity-resolution |
| Compliance | `ternaudit-guard` | EU AI Act | Art. 13/14/15 audit transparency |
| Auth / Gate | `ternlang-gate` | T-AUTH-v1.0 | Physical I/O authorization |

## 5. Institutional Standards and RFCs

| Identifier | Title | Status |
|------------|-------|--------|
| RFC-001 | Sovereign Trit-Encoding | PROPOSED |
| T-SPEC-v2.0 | Technical Specification (MVL) | PROPOSED |
| T-SEC-v1.0 | Post-Quantum Encryption Shield | DRAFT |
| IETF-TSON-01 | TSON Media Type | DRAFT |
| T-POSIX | OS Interface Standard | DRAFT |

Full specification texts are in `docs/standards/`.

---

**Organisation:** RFI-IRFOS (ZVR: 1015608684)  
**Contact:** contact@ternlang.com  
**Repository:** https://github.com/eriirfos-eng/ternary-intelligence-stack
