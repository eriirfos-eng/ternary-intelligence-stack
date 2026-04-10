# Ternlang Compiler & VM Fixes Ledger

This file tracks all architectural improvements, bug fixes, and feature additions to the Ternlang core crates (compiler and VM).

## Known Fixes (Legacy)

1. **Register Isolation** (2026-04-10)
   - VM now uses a `register_stack`.
   - `TCALL` saves current registers; `TRET` restores them.
   - Prevents caller registers from being clobbered by called functions.

2. **Integer Literal Support** (2026-04-10)
   - Implemented Opcode `0x17` (`TPUSH_INT`) and `0x18` (`TADD_INT`).
   - Compiler now correctly emits integer constants.

3. **1D Tensor (Trittensor) Logic** (2026-04-10)
   - `TSHAPE`, `TIDX`, `TSET` now detect 1D tensors correctly.
   - Fixed length calculation (removed incorrect square root on 1D length).
   - Enables DNA and NLP 1D array processing.

4. **Match/Loop Codegen Stability** (2026-04-10)
   - `TDUP` is now emitted before conditional jumps.
   - Resolved stack underflows in `match` and `for..in` blocks.

5. **Agent Opcodes** (2026-04-10)
   - Enabled `spawn`, `send`, and `await`.
   - Agent `type_id`s are now registered in the CLI.

6. **Variable Reassignment** (2026-04-10)
   - `Set` variant implemented in the parser and codegen.
   - Re-assigning `let` variables (acting as mutable by default in current VM) now works: `x = value;`.

7. **Error Reporting Improvement** (2026-04-10)
   - `TypeMismatch` ([BET-007]) now returns descriptive strings (e.g., "Expected Trit but found Int(15)").

8. **Consensus Logic** (2026-04-10)
   - `consensus(a, b)` implemented as logical merge (max signal wins: 1+1=1), not arithmetic addition.

## Recent Fixes (Batch 2 - 2026-04-10)

9. **Numeric Polymorphism**
   - Opcodes `0x02` (Add), `0x03` (Mul), `0x04` (Neg), `0x14` (Less), `0x15` (Greater), `0x16` (Eq) are now fully polymorphic across `Trit`, `Int`, and `Float`.

10. **New Opcodes for Math & IO**
    - `0x19` (`TpushFloat`): Support for floating-point literals.
    - `0x1e` (`Tdiv`): Floating-point and integer division.
    - `0x1f` (`Tmod`): Modulo operator support.
    - `0x20` (`Tprint`): Native printing support for `print()` and `println()`.

11. **Lexer Priority Disambiguation**
    - Resolved digit conflicts by setting token priorities: `Float` (100), `Int` (10), `TritLiteral` (2).

12. **While Loop & Continue**
    - Implemented `WhileTernary` and `Continue` in `betbc.rs` and the VM.
    - `while` loops now handle ternary conditions (executing on `1`, exiting on `0` or `-1` or via `break`).

13. **Postfix Propagation (?) Operator**
    - Implemented early-exit behavior for the `?` operator.
    - Automatically returns `conflict()` (-1) if the expression evaluates to `-1`.

14. **Directive Support (@)**
    - Parser and AST updated to handle `@` decorated functions (e.g., `@sparseskip`).

15. **Stack Management for Early Exits**
    - Fixed stack depth tracking in the emitter to ensure `?` and `return` don't leave dangling values on the VM stack during nested calls.
