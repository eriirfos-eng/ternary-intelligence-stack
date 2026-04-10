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

15. Stack Management for Early Exits
    - Fixed stack depth tracking in the emitter to ensure `?` and `return` don't leave dangling values on the VM stack during nested calls.

## Recent Fixes (Batch 3 - 2026-04-10)

16. **Match Statement for Integers**
    - `Match` now supports `Int` literals and negative patterns (e.g., `-5 => ...`).
    - Moved exhaustiveness check from Parser to Semantic analyzer.
    - `TjmpPos` (0x05), `TjmpZero` (0x06), `TjmpNeg` (0x07) updated to **peek** instead of pop, and use **exact equality** (1, 0, -1) rather than sign checks.
    - Implemented `TjmpEqInt` (0x25) for arbitrary integer pattern matching.
    - Codegen now correctly cleans the stack after match completion or fallthrough.

17. **1D Square Tensor Fix** (2026-04-10)
    - **Trigger:** Loop over 1D tensor with perfect-square length (e.g., 4, 9, 16).
    - **Symptom:** `BET-008` TensorIndexOutOfBounds or skipped elements during `for..in` loops.
    - **Diagnosis:** VM opcodes `TIDX`/`TSET`/`TSHAPE` used `sqrt(len)` to guess if a tensor was 2D. 1D tensors of size 4 were misidentified as 2x2 matrices, causing incorrect index calculation `row * 2 + col`.
    - **Fix:** Updated `BetVm` to store explicit `rows` and `cols` in `TensorInstance`. Updated `Talloc` (0x0f) to take 4 immediate bytes (`rows: u16`, `cols: u16`). Updated `betbc.rs` to emit dimensions during allocation.
    - **Status:** Fixed.

18. **TritLiteral Lexer Priority Fix** (2026-04-10)
    - **Trigger:** Using `1` or `0` literals in `.tern` programs.
    - **Symptom:** Failing compiler tests (`lexer::tests::test_lexer`) and type mismatches in function calls expecting `trit`.
    - **Diagnosis:** `TritLiteral` was only defined as `"-1"`, and `Int` had higher priority (10) than the generic digit pattern. `1` and `0` were tokenized as `Int(1)` and `Int(0)`.
    - **Fix:** Updated `lexer.rs` to define `TritLiteral` as `regex(r"1|0|-1")` with priority 11.
    - **Status:** Fixed.

19. **Tmod/Tdiv Numeric Polymorphism** (2026-04-10)
    - **Trigger:** Using `trit` values (like `1`, `0`, `-1`) as operands for `%` or `/`.
    - **Symptom:** `BET-007` TypeMismatch: expected Int but found Trit.
    - **Diagnosis:** `Tmod` and `Tdiv` only handled `Int % Int` or `Float / Float`. With `1` and `0` now being trits, they must be allowed in integer math.
    - **Fix:** Updated `vm/mod.rs` to allow `Trit` operands for `Tmod` and `Tdiv`, treating them as `i64` equivalents.
    - **Status:** Fixed.

20. **WhileTernary Codegen Fix** (2026-04-10)
    - **Trigger:** Using `while` loops with conditions that evaluate to `0` or `-1`.
    - **Symptom:** Infinite loop even when the condition should terminate the loop.
    - **Diagnosis:** `WhileTernary` codegen always looped back to the top after executing ANY of the three arms (`on_pos`, `on_zero`, `on_neg`). For standard `while` loops, only the `on_pos` arm should loop back.
    - **Fix:** Updated `betbc.rs` to only emit the back-jump for the `on_pos` arm by default. `on_zero` and `on_neg` now correctly jump to the end of the loop.
    - **Status:** Fixed.

21. **Tensor length() Built-in** (2026-04-10)
    - **Feature:** Added `length(tensor)` built-in function to retrieve the primary dimension of a `trit[]` or `trittensor`.
    - **Implementation:** Updated `betbc.rs` to emit `TSHAPE` (0x24) followed by `TPOP` (0x0c) to leave the rows/length on the stack. Updated `semantic.rs` to recognize the `length` signature.
    - **Benefit:** Enables idiomatic loop bounds and sequence processing in `.tern` programs.

22. **Unified Numeric Literals & Coercion** (2026-04-10)
    - **Feature:** Moved `1`, `0`, and `-1` to be `Int` literals by default in the lexer, with implicit `Int` -> `Trit` coercion in the semantic analyzer.
    - **Reason:** Prevented loop counters (often starting at 0 or 1) from being treated as trits, which caused arithmetic overflows (e.g., `1 + 1 = -1` in ternary).
    - **Implementation:** Updated `lexer.rs` to remove `TritLiteral` number matching. Updated `semantic.rs` to allow `Int` in places where `Trit` is expected (assignments, returns, arguments). Updated compiler tests to reflect the new flexibility.
    - **Status:** Fixed / Feature Added.

## Known Limitations / Unresolved (2026-04-10)

### BUG-L01 — Block comments (`/* */`) are not supported

**Trigger:** Any `.tern` file containing `/* ... */` block comments.
**Symptom:** `Parse stmt error: UnexpectedToken("Slash")` — program may exit successfully but emits only 3 bytes and skips all real logic.
**Diagnosis:** The Ternlang lexer does not have a block comment token rule. The `/` character is tokenized as a divide operator, and `*` starts multiplication, causing the comment to be parsed as an expression — which fails.
**Fix:** Not fixed (compiler change required to add block comment lexer rule).
**Workaround:** Use `//` line comments exclusively. Replace `/* comment */` with `// comment` or remove entirely. Empty match arms that previously held only a block comment must be given a valid statement (e.g., `let _h: trit = hold();`).
**Status:** Unresolved — needs lexer change in `ternlang-core/src/` to recognize `/* ... */` token span.
**Affected file:** `stdlib/safety/confidence_gate.tern` was broken by this. Rewritten 2026-04-10 to use `fn main()` entry point + helper function pattern; now passes.

### BUG-L02 — `parse_program()` fails on mixed function + top-level code in fallback mode

**Trigger:** `.tern` files where `parse_program()` fails and the fallback stmt-by-stmt parser encounters a `fn` keyword as the first or second token.
**Symptom:** `Parse stmt error: UnexpectedToken("Fn")` — only 3 bytes emitted, logic does not run.
**Diagnosis:** The fallback parser treats `fn` as an unexpected statement-level token. Full `parse_program()` must succeed for function-containing files to work.
**Fix:** Not fixed in compiler.
**Workaround:** Ensure all function-containing files parse cleanly with `parse_program()`. Use `fn main() -> trit { ... }` as the entry point. Avoid top-level code mixed with function definitions if parse errors occur.
**Status:** Unresolved — affects `stdlib/math/ternary_median.tern` (pre-existing).


## 2026-04-10 — Add LessEqual (<=) and GreaterEqual (>=) operators

**Trigger:** Writing stdlib/safety/confirmation_gate.tern which required count >= required.
**Symptom:** Parse error: UnexpectedToken("Assign") because >= was tokenized as > followed by =.
**Diagnosis:** Lexer and Parser lacked support for double-character comparison operators.
**Fix:** 
- Updated ast.rs to add LessEqual and GreaterEqual to BinOp.
- Updated lexer.rs to add Token::LessEqual (<=) and Token::GreaterEqual (>=).
- Updated parser.rs to handle these tokens in precedence and binop conversion.
- Updated semantic.rs to recognize these as returning Type::Trit.
- Updated codegen/betbc.rs to emit opcodes 0x26 and 0x27.
- Updated vm/mod.rs to implement opcodes 0x26 (TlessEqual) and 0x27 (TgreaterEqual).
**Status:** Fixed.

## 2026-04-10 — Fix missing agent type registration in CLI

**Trigger:** Attempting to use agents (spawn, send, await).
**Symptom:** Agent types not found in VM, spawn fails.
**Diagnosis:** BytecodeEmitter had agent type info, but it was never passed to the BetVm instance in the CLI's Run command.
**Fix:** Added emitter.register_agents(&mut vm) call in main.rs after VM initialization.
**Status:** Fixed.
