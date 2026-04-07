# BET VM Instruction Set Architecture (ISA) v1.0
**Status:** Canonical Draft | **Sponsor:** RFI-IRFOS | **Architecture:** 9-Trit RISC

## 1. Core Architectural Philosophy
The Balanced Execution Ternary Virtual Machine (BET VM) ISA is fundamentally distinct from x86, ARM, or RISC-V. It is structurally immune to binary boolean coercion and is designed for native sparse inference operations.

### 1.1 The "Sparsity Yield" Mandate
Hardware compliance with BET-ISA v1.0 requires that arithmetic logic units (ALUs) must physically bypass clock cycles when executing instructions against operands evaluating to `0` (TEND).

## 2. Definitive Opcodes

### 2.1 Arithmetic & Matrix (T-Ops)
- `TADD (rd, rs1, rs2)`: Ternary addition.
- `TMUL (rd, rs1, rs2)`: Ternary multiplication.
- `TSPARSE_MATMUL (rd, ptr_a, ptr_b)`: **The Core Tensor Opcode.** Emits a hardware-level interrupt to bypass cycles on `0` operands, natively yielding a 50-80% thermal reduction compared to binary INT8 precision.

### 2.2 Control Flow (Triadic Branching)
Binary architectures use `JMP` (Jump if True/False). BET-ISA requires exhaustive routing:
- `TMATCH (rs1, off_pos, off_tend, off_neg)`: Branches to one of three hardcoded offsets depending on the state of `rs1` (+1, 0, -1). Failure to supply an `off_tend` offset generates a compile-time Veto.
- `THOLD (rs1, duration)`: Sleep or yield thread execution indefinitely if `rs1 == 0` until a hardware interrupt or new consensus breaks the equilibrium.

### 2.3 Structural Safety (The Veto Primitives)
- `TVETO`: Triggers an immediate, uncatchable hardware panic when a `-1` (Reject) is encountered on a memory region flagged as `Hard Safety` by the MoE orchestrator.
- `TLOCK (ptr)`: Acquires a native ternary mutex, defaulting to `0` (TEND) natively rather than busy-waiting.

## 3. Hardware Licensing
All physical or FPGA representations of the BET-ISA v1.0 opcodes must conform to the **ISO "Certified Uncertainty"** specification. Any commercial, non-open-source tape-out requires explicit licensing from the RFI-IRFOS standards board.
