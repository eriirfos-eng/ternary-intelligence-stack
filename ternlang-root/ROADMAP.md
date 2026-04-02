# Ternlang Roadmap: Bridging the Ternary Software Deficit
### Project: Ternary Intelligence Stack (TIS) | RFI-IRFOS
**Current Version:** v0.1 (Foundational)

## 🎯 Strategic Objective
Position RFI-IRFOS as the definitive middleware provider for ternary computing by commercializing **ternlang** as the standard paradigm for ambiguity-aware AI agents and sparse inference.

---

## ✅ Phase 1: Core Language & VM Stability (Current Focus)
- [x] **Trit Primitives**: Implement `-1`, `0`, `+1` logic (Sum/Carry, Neg, Mul).
- [x] **Lexer**: Tokenize ternary-specific keywords (`trit`, `trittensor`, `?`).
- [x] **Skeletal Parser**: Parse basic expressions and `IfTernary` (`if ?`).
- [x] **BET VM Core**: Basic stack, registers, and 2-bit packing (`0b01=-1`, `0b10=+1`, `0b11=0`).
- [x] **Parser Completion**:
    - [x] Add `Function` and `Program` parsing.
    - [x] Implement `match` with mandatory 3-way exhaustive branching.
- [x] **Codegen (Bytecode Emitter)**:
    - [x] **Jump Resolution**: Calculate offsets for ternary jumps (`TJMP_POS`, etc.).
    - [x] **Register Allocation**: Basic symbol table for register mapping.
- [x] **VM Enhancements**:
    - [x] Proper carry handling in `Tadd`.
    - [x] Rich error reporting for `VmError`.

## 🛠 Phase 2: Standard Library & CLI Integration (Next Focus)
- [x] **CLI Driver**: Build `ternlang-cli` to compile `.tern` files and run them on BET VM.
- [x] **Built-in Functions**:
    - [x] `consensus(a, b)` logic.
    - [x] `invert(x)`.
    - [x] `truth()`, `hold()`, `conflict()`.
- [ ] **Standard Library (`std::trit`)**:
 Initial module structure.

## 🧠 Phase 3: TritTensors & Sparse Inference
- [ ] **TritTensor Type**: AST and Type Checker support for `trittensor<rows x cols>`.
- [ ] **Sparse Execution**:
    - [ ] Implement `@sparseskip` directive in codegen.
    - [ ] VM-level optimizations for skipping `0` states in tensor operations.
- [ ] **ML Kernels**: Basic `matmul` optimized for ternary weights.

## 🤖 Phase 4: Actor Model & Distributed Agents
- [ ] **Agent Definitions**: `agent { ... }` syntax and metadata.
- [ ] **Message Passing**:
    - [ ] `send(ref, msg)` and `await ref`.
    - [ ] `agentref` and `nodeid` type support.
- [ ] **Distributed Runtime**: P2P actor execution (research phase).

## 📡 Phase 5: Hardware & HDL Backends
- [ ] **VM Lowering**: Lower BET bytecode to Verilog/VHDL targets.
- [ ] **Ternary ISA**: Define a standard Instruction Set Architecture for hardware synthesis.
- [ ] **FPGA Prototypes**: RFI-IRFOS hardware integration testing.

---

## 📝 Current Task Context
*Last updated: 2026-04-02*
- Phase 1 & 2 Foundations Complete: CLI, Lexer, Parser, Codegen, VM.
- Next: Implementing `TritTensor` support in AST, Parser, and VM for sparse AI inference.
