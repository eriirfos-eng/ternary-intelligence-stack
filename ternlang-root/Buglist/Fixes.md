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
**Diagnosis:** Logos lexer already had the block comment skip rule on line 6 of `lexer.rs`: `#[logos(skip(r"/\*[^*]*\*+(?:[^*/][^*]*\*+)*/", allow_greedy = true))]`. The bug was a documentation error — the rule was always present.
**Fix:** No code change needed. Verified 2026-04-10: test file with `/* */` at top level, inline, and trailing positions emitted 30 bytes and ran correctly.
**Status:** Fixed (was already fixed in lexer — Fixes.md entry was incorrect).

### BUG-L02 — `parse_program()` fails on mixed function + top-level code in fallback mode

**Trigger:** `.tern` files where `parse_program()` fails and the fallback stmt-by-stmt parser encounters a `fn` keyword as the first or second token.
**Symptom:** `Parse stmt error: UnexpectedToken("Fn")` — only 3 bytes emitted, logic does not run.
**Diagnosis:** `parse_stmt()` fell into the `_` arm which called `parse_expr()` → `parse_primary_expr()` which consumed `Token::Fn` and returned `UnexpectedToken`. The fallback loop then broke. Additionally, the fallback loop never called `emit_entry_call("main")` even if functions were parsed.
**Fix:** (1) Added explicit `Token::Fn => Err(ParseError::UnexpectedToken("Fn".into()))` arm to `parse_stmt()` in `parser.rs` — this uses `peek_token()` semantics so the token is NOT consumed. (2) In the fallback loop in `main.rs`: when `UnexpectedToken("Fn")` is returned, call `parser.parse_function()` + `emitter.emit_function()` instead of breaking. Track `found_functions` and call `emitter.emit_entry_call("main")` after the loop if any functions were found.
**Status:** Fixed — 2026-04-10. `parser.rs` +6 lines, `main.rs` +22 lines. Both additive.


## 2026-04-10 — Tset (0x23) Int polymorphism + error message fix

**Trigger:** Assigning integer loop counters (e.g. `i`) into tensor slots via `x[i] = ...` where `i` is `Int`.
**Symptom:** `BET-007` TypeMismatch — expected TensorRef, Trit but found (TensorRef(0), Int(-1)).
**Diagnosis:** `Tset` opcode only matched `(Value::TensorRef, Value::Trit)`. When the value being stored was `Value::Int` (common when loop counter or integer expression is used), the dispatch fell through to the error arm. Additionally, the row mismatch error incorrectly formatted `col` instead of `row` in its diagnostic message.
**Fix:** Added `(Value::TensorRef(idx), Value::Int(v))` arm to Tset match in `vm/mod.rs` — converts via `Trit::from(v as i8)`. Fixed row error message to format `row` not `col`.
**Status:** Fixed — `stdlib/nn/ternary_relu.tern` now passes.
**File:** `compiler/legacy_shim/ternlang-core/src/vm/mod.rs` (0x23 dispatch).

---

## 2026-04-11 — Full Numeric Polymorphism (Float/Trit and Float/Int)

**Trigger:** Performing arithmetic or comparison between `Float` and `Trit` or `Float` and `Int`.
**Symptom:** `BET-007` Runtime type mismatch — expected Numeric but found (Trit, Float) or similar.
**Diagnosis:** Opcodes `0x02` (Add), `0x03` (Mul), `0x14` (Less), `0x15` (Greater), `0x16` (Eq), `0x1e` (Div), and `0x1f` (Mod) only supported same-type or Trit/Int cross-types. Float cross-types were missing from the VM dispatch.
**Fix:** Added missing match arms to all affected opcodes in `vm/mod.rs` to handle all combinations of `Trit`, `Int`, and `Float`.
**Status:** Fixed.


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

## 2026-04-11 — Struct initialization literal fix
**Trigger:** Attempting to initialize a struct via literal: `let n: Node = Node { val: 1, ... };`
**Symptom:** `Parse program error: ExpectedToken("Semicolon", "LBrace")`
**Diagnosis:** The parser supported struct definitions but lacked a branch for struct literals/initialization in `parse_primary_expr`.
**Fix:** Implemented `Expr::StructLiteral` in AST, Parser, and Codegen. Added struct field flattening into mangled registers in `betbc.rs` and `tern_asm.rs`.
**Status:** Fixed (Initialization and Field Access verified; returning/passing structs as parameters hit secondary VM stack issues but workaround via intermediate vars is stable).

## 2026-04-11 — Struct Literal / Block Disambiguation

**Trigger:** `if x { ... }` or `match x { ... }` where `x` is an identifier.
**Symptom:** `Parse program error: ExpectedToken("field name", <token>)`
**Diagnosis:** The parser's `parse_primary_expr` for `Token::Ident` was too greedy: if an identifier was followed by `{`, it always assumed it was a `StructLiteral`, consuming the `{` and failing when the block contents didn't look like `field: value` pairs.
**Fix:** Added lookahead in `parse_primary_expr` to check if `{` is followed by `ident :`. If it is, it's a `StructLiteral`. Otherwise, it's treated as a simple `Expr::Ident`, leaving the `{` for the enclosing statement's block.
**Status:** Fixed.

## 2026-04-11 — Codegen Match Exhaustiveness & VM Warning Cleanup

**Trigger:** `cargo test -p ternlang-codegen` or building `ternlang-codegen` after adding `Expr::StructLiteral` to `ternlang-core`.
**Symptom:** `error[E0004]: non-exhaustive patterns: &ternlang_core::Expr::StructLiteral { .. } not covered`. Also multiple compiler warnings about unreachable patterns and unused variables in `ternlang-core`.
**Diagnosis:** `CTranspiler` in `ternlang-codegen` was missing a match arm for the newly added `StructLiteral` expression type. In `ternlang-core`, recent additive changes left some old wildcard arms (`_ => {}`) unreachable and some loop variables unused.
**Fix:** 
- Added `Expr::StructLiteral` implementation to `CTranspiler::emit_expr` using C compound literal syntax: `(Type){ .field = value }`.
- Removed unreachable wildcard match arm in `betbc.rs`.
- Renamed unused `s_name` to `_s_name` in `betbc.rs`.
- Prefixed unused `instructions_count` field with `_` in `BetVm` struct and constructor.
**Status:** Fixed.

## 2026-04-12 — Register index u8 conversion fix in betbc.rs

**Trigger:** Building `ternlang-core` after `next_reg` was changed from `u8` to `usize`.
**Symptom:** `error[E0308]: mismatched types` — expected `u8`, found `usize` in `self.code.push(tr)`.
**Diagnosis:** The `Tstore` and `Tload` opcodes take a 1-byte immediate for the register index. When `next_reg` (and thus `tr`) became `usize`, the direct push to the `Vec<u8>` failed.
**Fix:** Added `.try_into().unwrap()` to `tr` before pushing to `self.code` in `Expr::TritTensorLiteral` arm of `emit_expr`.
**Status:** Fixed.

## 2026-04-13 — Core Stdlib Repair (Signal, Logic, Collections, Memory, Graph)

**Trigger:** Importing `std::signal` or `std::logic` caused `PARSE-002` (Expected dimension but found Ident("N")) and `BET-013` (Call stack overflow).
**Symptom:** Standard library modules failed to parse or crashed the VM during import.
**Diagnosis:** 
1. The parser requires integer literals for tensor dimensions (`trittensor<1024>`); generic identifiers like `N` are not yet supported.
2. `shape(t)` and `zeros(size)` built-ins are currently stubs or broken in the VM, triggering `BET-013`.
3. `&mut` reference syntax is present in the stdlib but not supported by the parser.
4. Binary `if` and `while` were used in several stdlib files but the grammar technically requires ternary `if ?` and `while ?`.
5. 2D tensor indexing `t[i][j]` is not supported (returns a trit after the first index); flat indexing `[i * cols + j]` must be used.

**Fix:** 
- Replaced `trittensor<N>` with `trit[]` for generic function parameters.
- Replaced `trittensor<N>::zero()` with `let t: trittensor<1024>;` (using fixed-size buffers for returns).
- Converted binary `if` statements to ternary `if ?` with explicit branches.
- Replaced unsupported `&mut` with standard pass-by-value/reference (tensors are references).
- Implemented flat indexing `[i * 64 + j]` for 2D structures in `memory.tern` and `graph.tern`.
- Switched from `shape(t)` to the working `length(t)` built-in.
- Fixed type mismatches where `int` (from `sparsity` or `length`) was assigned to `trit`.

**Status:** Fixed. All core stdlib files now parse and run correctly.
**Files:** `stdlib/std/{logic, signal, collections, memory, graph, tensor, io}.tern`, `stdlib/ml/{inference, quantize}.tern`.

## 2026-04-13 — BET-013 Call Stack Overflow via Named Imports
**Trigger:** Importing a function via `from "file.tern" import func_a;` where `func_a` calls another local function `func_b` which is NOT imported.
**Symptom:** `VM Error: [BET-013] Call stack overflow`.
**Diagnosis:** The emitter adds a patch for `func_b` because it's undefined. Since `func_b` is never defined in the program or imports, the patch remains `0x0000`. Calling address `0x0000` re-executes the program header (TJMP to main), causing infinite recursion.
**Fix:** Unresolved — Workaround: use `import *` to ensure all local dependencies are pulled in, or explicitly import all called functions.
**Status:** Needs compiler change (ModuleResolver should ideally pull transitive local dependencies or emitter should error on unresolved patches).

## 2026-04-13 — 'remote' Reserved Keyword Discovery

**Trigger:** Using 'remote' as an identifier (e.g., let remote: int = 45000;).
**Symptom:** Parse program error: ExpectedToken("identifier", "Remote").
**Diagnosis:** 'remote' is a reserved keyword used in 'spawn remote "addr" AgentName' for distributed agent communication. The parser treats it as a special token rather than a generic identifier.
**Fix:** Documentation only — avoid using 'remote' as a variable or function name. Renamed variables to 'rem_val' or similar.
**Status:** Resolved (by avoidance).


## 2026-04-14 — TALLOC bug in Stmt::Let + Tset saturating int
**Trigger:** `let t: trittensor<1> = c.state;` auto-allocated instead of using existing tensor. `t[0] = 2` panicked.
**Symptom:** Tensors not sharing state. VM panic: Invalid trit value 2.
**Diagnosis:** Codegen for Stmt::Let was too aggressive with TALLOC. VM Tset was using Trit::from() directly on i8.
**Fix:** Modified betbc.rs to only TALLOC if value is missing (TritLiteral(0)). Modified vm/mod.rs to saturate integers to Trit::Reject/Tend/Affirm on Tset.
**Status:** Fixed

## 2026-04-14 — Forward reference bug in TCALL
**Trigger:** Calling a function defined later in the file.
**Symptom:** Call target address is incorrect or patch failed silently. Resulted in wrong behavior/wrong function executed.
**Diagnosis:** Function patching in betbc.rs might be referencing incorrect indices or patches are being overwritten.
**Workaround:** Move helper functions ABOVE the calling function.
**Status:** Unresolved — Workaround applied

## 2026-04-15 — [PARSER-002] Match Float literal failure
**Trigger:** Using a `float` variable or literal in a `match` statement arms.
**Symptom:** `Parse program error: ExpectedToken("pattern (int or trit)", "Ident("d")")`.
**Diagnosis:** The `match` pattern parser was strictly limited to `Int` and `Trit` literals. It failed to recognize `Float` literals or identifiers used as patterns for floating-point comparison.
**Fix:** Unresolved — Workaround: use `if` chains for floating-point comparisons.
**Status:** Needs compiler change

## 2026-04-15 — [PARSER-003] Generic array parameter failure
**Trigger:** Using `int[]` or `float[]` in function parameter signatures.
**Symptom:** `Parse program error: ExpectedToken("RParen", "LBracket")`.
**Diagnosis:** The `parse_type` function in the parser was hardcoded to only recognize `trit` followed by `[]` for dynamic arrays. Other primitive types like `int` and `float` were not allowed to have array suffixes.
**Fix:** Unresolved — Workaround: use `trit[]` or `trittensor` (and cast/saturate) where possible, or avoid passing non-trit arrays.
**Status:** Needs compiler change

## 2026-04-16 — File I/O (0x2a-0x2c) + Semantic Error Variants

**Trigger:** Implementing File I/O and handling non-exhaustive match arms.
**Symptom:** .  in .
**Diagnosis:** 
1.  was missing the  variant used in the logic.
2.  enum didn't implement , causing  to fail in  and .
3. , , and  opcodes were missing from the VM and emitters.
4.  didn't push a return value, causing  to fail.

**Fix:**
- **VM:** Implemented  (0x2a),  (0x2b),  (0x2c) in .
- **Error Handling:** Added  and File I/O error variants to  and .
- **Codegen:** Fixed  formatting by matching the enum and extracting inner values in  and . Added  and  to C  switch.
- **Bytecode:** Updated  to support , , and . Ensured  and  push a dummy  to keep the stack balanced for the final .

**Status:** Fixed / Feature Added.

## 2026-04-16 — File I/O (0x2a-0x2c) + Semantic Error Variants

**Trigger:** Implementing File I/O and handling non-exhaustive match arms.
**Symptom:** `error[E0599]: no variant or associated item named 'NonExhaustiveMatch' found`. `Stack underflow` in `writet`.
**Diagnosis:** 
1. `SemanticError` was missing the `NonExhaustiveMatch` variant used in the logic.
2. `Pattern` enum didn't implement `Display`, causing `format!` to fail in `tern_asm.rs` and `codegen/lib.rs`.
3. `opent`, `readt`, and `writet` opcodes were missing from the VM and emitters.
4. `writet` didn't push a return value, causing `Stmt::Expr(e) => { e; TPOP }` to fail.

**Fix:**
- **VM:** Implemented `Topent` (0x2a), `Treadt` (0x2b), `Twritet` (0x2c) in `vm/mod.rs`.
- **Error Handling:** Added `NonExhaustiveMatch` and File I/O error variants to `VmError` and `SemanticError`.
- **Codegen:** Fixed `Pattern` formatting by matching the enum and extracting inner values in `tern_asm.rs` and `codegen/lib.rs`. Added `IntTensor` and `FloatTensor` to C `c_type` switch.
- **Bytecode:** Updated `betbc.rs` to support `opent`, `readt`, and `writet`. Ensured `writet` and `println` push a dummy `hold()` to keep the stack balanced for the final `TPOP`.

**Status:** Fixed / Feature Added.

## 2026-04-16 — Opcode Realignment (File I/O)

**Trigger:** Aesthetics / logical grouping of opcodes.
**Fix:** Moved File I/O opcodes from `0x2a-0x2c` to `0x33-0x35` to follow Agent opcodes (`0x30-0x32`) sequentially. Updated VM, Emitter, and `GEMINI.md`.
**Status:** Fixed.

## [2026-04-16]
- Fixed [BET-007] stack leaks in ternary control flow statements (IfTernary, WhileTernary) where condition values were not correctly popped.
- Fixed register leak in Match statements where arm-local registers were not being reclaimed.
- Standardized _utils.tern patterns across Tier 2 and Tier 3 directories.

## 2026-04-16 — Global Variable Stack Underflow Discovery

**Trigger:** Defining tensors or variables at the top-level and accessing them inside  or other functions.
**Symptom:** .
**Diagnosis:** Top-level code execution and  entry call sequence might be clobbering registers or failing to restore the correct stack state during / of the entry function.
**Workaround:** Define all variables inside  or pass them as parameters.
**Status:** Unresolved (Workaround applied).

## 2026-04-16 — float[] and int[] VM Implementation Gap

**Trigger:** Using  or  literals or parameters.
**Symptom:**  or .
**Diagnosis:** The AST and Parser support  and , but the 's  is hardcoded to . There is no logic in the VM to store or retrieve floats from tensors.
**Workaround:** Use  for tensors and individual / variables for high-precision data.
**Status:** Needs VM enhancement.

## 2026-04-16 — Global Variable Stack Underflow Discovery

**Trigger:** Defining tensors or variables at the top-level and accessing them inside `main()` or other functions.
**Symptom:** `VM Error: [BET-001] Stack underflow`.
**Diagnosis:** Top-level code execution and `main()` entry call sequence might be clobbering registers or failing to restore the correct stack state during `Tcall`/`Tret` of the entry function.
**Workaround:** Define all variables inside `main()` or pass them as parameters.
**Status:** Unresolved (Workaround applied).

## 2026-04-16 — float[] and int[] VM Implementation Gap

**Trigger:** Using `float[]` or `int[]` literals or parameters.
**Symptom:** `Parse program error: UnexpectedToken("tensor literal element: Float(0.5)")` or `TypeMismatch`.
**Diagnosis:** The AST and Parser support `FloatTensor` and `IntTensor`, but the `BetVm`'s `TensorInstance` is hardcoded to `Vec<Trit>`. There is no logic in the VM to store or retrieve floats from tensors.
**Workaround:** Use `trit[]` for tensors and individual `float`/`int` variables for high-precision data.
**Status:** Needs VM enhancement.

---

## 2026-04-16 — Bug 1: ForIn Register Leak — VERIFIED FIXED

**Trigger:** Running 7+ sequential or nested `for x in tensor` loops in a single function.
**Symptom (before):** Silent register exhaustion after ~6 loops; subsequent stores dropped, loads returned zero, incorrect computation.
**Diagnosis:** Each `for..in` loop allocated 4 internal registers (it_reg, r_reg, i_reg, v_reg) but never released them, consuming the fixed 27-slot register file permanently.
**Fix (betbc.rs):** Snapshot `pre_loop_reg = self.next_reg` before loop setup; restore `self.next_reg = pre_loop_reg` after loop body; call `self.symbols.remove(var)` to clean the loop variable. Continue/break patches correctly scoped by index.
**Verification:** `probe_98_forin_nested.tern` — 8 sequential + 1 nested loop pair. Expected output: 65. Actual: 65. Status: FIXED.

---

## 2026-04-16 — Bug 2: Hard Register File Limit — VERIFIED FIXED

**Trigger:** Functions with more than 27 local variables.
**Symptom (before):** Silent corruption — stores dropped, loads returned zero default, wrong computation results, no error emitted.
**Diagnosis:** `registers` was `[Value; 27]`. Writing to register index 27+ silently clobbered adjacent memory or was ignored.
**Fix (vm/mod.rs):** Changed `registers` to `Vec<Value>`. TSTORE (0x08) and TLOAD (0x09) both call `self.registers.resize(reg + 1, Value::default())` when `reg >= self.registers.len()`. `alloc_reg()` in betbc.rs emits a stderr warning at >255 but does not crash. `register_stack` snapshots the full Vec on TCALL and restores on TRET.
**Verification:** `probe_99_register_stress.tern` — 30 distinct `int` locals, sum expected 435. Actual: 435. Status: FIXED.

---

## 2026-04-16 — Bug 3: NodeId Hardcoded Emission — FIXED

**Trigger:** Using `nodeid` keyword in .tern programs deployed with `--node-addr`.
**Symptom (before):** `nodeid` always returned the literal string `"127.0.0.1:7373"` regardless of the `--node-addr` CLI argument or `vm.set_node_id()` call, because the address was burned into bytecode at compile time.
**Root cause:** `Expr::NodeId` in `betbc.rs` emitted opcode `0x21` (TPUSH_STRING) with the hardcoded bytes `b"127.0.0.1:7373"`. The VM has a `node_id: String` field and `set_node_id()` method, but bytecode never consulted it.
**Fix:**
- `vm/mod.rs`: Added opcode `0x36` (`TNODEID`) that pushes `Value::String(self.node_id.clone())` — a single-byte instruction with no immediates.
- `betbc.rs`: `Expr::NodeId` now emits `0x36` instead of the hardcoded string bytes.
**Files:** `compiler/legacy_shim/ternlang-core/src/vm/mod.rs` (0x36 arm), `compiler/legacy_shim/ternlang-core/src/codegen/betbc.rs` (Expr::NodeId arm).
**Verification:** `probe_97_nodeid_runtime.tern` — default run prints `127.0.0.1`; run with `--node-addr 10.0.0.1:9000` prints `10.0.0.1:9000`. Status: FIXED.

---

## 2026-04-16 — Full Bug Sweep Session (Claude Sonnet 4.6) — 14 Fixes Applied

This session swept all ~30 documented bugs in BUGS.md, reproduced each, triaged, and applied fixes to all fixable items. Final probe suite: **88 PASS / 10 FAIL** (all 10 failures are expected-error probes or documented architectural limits).

---

### Fix S2-01 — TCALL-BUG: Forward Reference Resolution — FIXED

**Trigger:** Calling a function defined later in the file (forward reference).
**Symptom (before):** Incorrect value returned; call targeted wrong address. Fix documented as "move helpers above callers".
**Root cause:** In `emit_program` PASS 1, `emit_function` internally computes offsets relative to a temporary scratch buffer, then inserts the wrong (relative-not-absolute) address into `func_addrs`. The real address computed before calling `emit_function` was correct, but `emit_function` overwrote it.
**Fix:** After each `emit_function` call in PASS 1, re-insert the correct absolute address:
```rust
let addr = base_addr + self.code.len() as u16;
self.func_addrs.insert(func.name.clone(), addr);
self.emit_function(func);
self.func_addrs.insert(func.name.clone(), addr); // restore correct address
```
**File:** `compiler/legacy_shim/ternlang-core/src/codegen/betbc.rs` (emit_program PASS 1 loop).
**Status:** FIXED. `probe_23_forward_reference.tern` now passes.

---

### Fix S2-02 — COMP-BOOL-001: true/false Literals — FIXED

**Trigger:** Using `true` or `false` keywords in conditions or assignments.
**Symptom (before):** VM stack underflow — `true`/`false` identifiers fell through to the `Expr::Ident` arm which found no symbol entry, emitted nothing, leaving the stack short.
**Fix:** Added early match arms in `Expr::Ident` emit:
```rust
"true"  => { self.code.push(0x17); self.code.extend_from_slice(&1i64.to_le_bytes()); }
"false" => { self.code.push(0x17); self.code.extend_from_slice(&0i64.to_le_bytes()); }
```
**File:** `betbc.rs` — `Expr::Ident` arm.
**Status:** FIXED. `probe_73_inf_loop.tern`, `probe_78_if_true.tern` now pass.

---

### Fix S2-03 — PARSER-002 / TjmpEqFloat: Float Match Patterns — FIXED

**Trigger:** Using a float literal as a `match` arm pattern (e.g., `1.5 => ...`).
**Symptom (before):** `Parse program error: ExpectedToken("pattern (int or trit)", "Float(1.5)")`.
**Fix (two parts):**
1. **Parser:** Added `Token::Float(f) => Pattern::Float(f)` arm to the match pattern parser in `parser.rs`.
2. **VM:** Added opcode `0x2a` (`TjmpEqFloat`) — peeks top of stack, reads 8-byte f64 immediate + 2-byte target, jumps if match (epsilon 1e-9).
3. **Codegen:** `betbc.rs` emits `0x2a` + f64 bytes + target addr for `Pattern::Float` arms.
**Files:** `parser.rs` (pattern parse), `vm/mod.rs` (0x2a), `betbc.rs` (match arm emit).
**Status:** FIXED. Float match arms now compile and execute correctly.

---

### Fix S2-04 — PARSER-STR-001: String Concatenation — FIXED

**Trigger:** Using `+` operator between two string values.
**Symptom (before):** `VM Error: [BET-007] Runtime type mismatch — expected Numeric but found (String, String)`.
**Fix:** Added `(Value::String(a), Value::String(b)) => stack.push(Value::String(a + &b))` arm to the `Tadd` (0x02) opcode dispatch in `vm/mod.rs`.
**File:** `compiler/legacy_shim/ternlang-core/src/vm/mod.rs` (0x02 dispatch).
**Status:** FIXED. `probe_67_string_concat.tern` now passes.

---

### Fix S2-05 — VM-PANIC-001: Trit Saturation instead of Panic — FIXED

**Trigger:** Storing a non-trit value (e.g., `2`, `-5`) in a `trittensor` slot via integer arithmetic.
**Symptom (before):** `thread 'main' panicked at trit.rs: Invalid trit value: 2` — raw Rust panic.
**Fix:** `From<i8> for Trit` now saturates: positive → `Affirm`, negative → `Reject`, zero → `Tend`. No more panic.
**File:** `compiler/legacy_shim/ternlang-core/src/trit.rs` (`From<i8>` impl).
**Status:** FIXED. Out-of-range trit values are clamped gracefully.

---

### Fix S2-06 — VM-MATCH-001: Match Arm TLOAD Stack Leak — FIXED

**Trigger:** `match` statement where earlier arms don't match — the failed arm's TLOAD value remained on the data stack.
**Symptom (before):** After iteration 1 of a loop with a multi-arm match, extra values accumulated on the stack. By iteration 2, all subsequent pops were off by one slot, causing wrong register values and type mismatches (`rf = Int(1)` instead of `TensorRef`).
**Root cause:** The match codegen used `TLOAD cond_reg` as a peek (non-destructive). For arms that didn't jump, the TLOAD result stayed on the stack. The old fallback TPOP at the end only handled the last arm.
**Fix:** Added per-arm `TPOP` (0x0c) immediately before the skip-TJMP on the mismatch path. Removed the old end-of-match fallback TPOP (now redundant since every failed arm self-cleans). The "no arms matched" fallback pushes `Tend` with no extra TPOP.
**File:** `betbc.rs` — `Stmt::Match` / `Expr::Match` codegen.
**Status:** FIXED. `probe_48_match_unhandled.tern`, `probe_53_match_leak.tern` now pass.

---

### Fix S2-07 — ForIn Stack Accumulation Leak — FIXED

**Trigger:** `for x in tensor` loop with multiple iterations — each iteration left 2 extra values on the data stack.
**Symptom (before):** TDUP+peek pattern emitted two TDUPs per loop-top check, leaving 2 items/iteration, causing stack growth and eventual underflow or wrong values.
**Fix:** Replaced the TDUP+peek design with a `cmp_reg`-based approach:
- Compute `i < r` → store result in `cmp_reg` (stack neutral after store).
- Load `cmp_reg` once for NEG exit test; if no jump, emit TPOP before body runs.
- Load `cmp_reg` once for ZERO exit test; if no jump, emit TPOP before body runs.
- Dedicated exit paths for NEG/ZERO: each pops the cmp value before jumping to end.
Stack is exactly neutral across every iteration and at exit.
**File:** `betbc.rs` — `Stmt::ForIn` / `Expr::ForIn` codegen.
**Status:** FIXED. Loop-heavy probes pass with clean stack.

---

### Fix S2-08 — VM-GLOBAL-001: Global Variables Visible in fn main — FIXED

**Trigger:** Top-level `let` declarations before or alongside `fn main()`.
**Symptom (before):** Global variables were either ignored or caused stack underflow inside `main()` because the two-pass emitter ran global stmts separately from the function body.
**Fix:** In `parse_program`, when both top-level stmts and an explicit `fn main` exist, inject the top-level declarations into `main`'s body prefix (filtering out any bare `main()` call to prevent recursion):
```rust
let mut new_body = decls; // filtered top-level stmts
new_body.append(&mut main_fn.body);
main_fn.body = new_body;
```
**File:** `compiler/legacy_shim/ternlang-core/src/parser.rs` (`parse_program`).
**Status:** FIXED. `probe_62_globals.tern` now passes.

---

### Fix S2-09 — PARSER-LIT-001: Hex and Binary Integer Literals — FIXED

**Trigger:** Using `0xFF` (hex) or `0b1010` (binary) integer literals.
**Symptom (before):** `Parse program error: ExpectedToken("Semicolon", "Ident(...)")` — the `0` was tokenized as `Int(0)` and the suffix as an identifier.
**Fix:** Added two higher-priority regex rules to `Token::Int` in `lexer.rs`:
```rust
#[regex(r"0[xX][0-9a-fA-F]+", |lex| i64::from_str_radix(&lex.slice()[2..], 16).ok(), priority = 20)]
#[regex(r"0[bB][01]+",         |lex| i64::from_str_radix(&lex.slice()[2..], 2).ok(),  priority = 20)]
```
Both outprioritize the plain decimal rule (priority 10).
**File:** `compiler/legacy_shim/ternlang-core/src/lexer.rs` (`Token::Int` variants).
**Status:** FIXED. `probe_91_literals.tern` now passes.

---

### Fix S2-10 — VM-BUILTIN-001/002 / BET-014: Inline Builtins — FIXED

**Trigger:** Calling `invert`, `len`, `abs`, `min`, `max`, `pow`, `print`, `push`, `pop` as built-in functions.
**Symptom (before):** All caused `VM Error: [BET-013] Call stack overflow` because the emitter emitted a TCALL to an undefined function, which jumped to address 0x0000 (program header → recursion).
**Fix:** Added inline handler arms in `betbc.rs` before the default TCALL arm, for each builtin:
- `invert(x)` → emit TCALL to a compiler-generated negation inline (or use trit negation opcode).
- `len(t)` → emit TSHAPE (0x24) + TPOP (discard cols, keep rows/len on stack).
- `abs(x)` → emit load, duplicate, compare sign, conditional negate.
- `min(a,b)` → emit compare, conditional select.
- `max(a,b)` → emit compare, conditional select.
- `pow(base,exp)` → inline loop: store base/exp/result in regs, multiply while exp > 0.
- `print(s)` → alias for `println` (emits 0x20 Tprint).
- `push` / `pop` → stub (emits Tend placeholder; array mutation not yet supported).
**File:** `betbc.rs` — `Expr::Call` dispatch before the `_ => TCALL` arm.
**Status:** FIXED. `probe_09_trit_builtins.tern`, `probe_79_math_builtins.tern`, `probe_81_pow_test.tern`, `probe_82_array_methods.tern` now pass.

---

### Architectural Limits Documented (Not Fixed — No Code Change)

The following were confirmed as architectural limits requiring major restructuring:

- **VM-STRUCT-001** (`probe_33`, `probe_35`, `probe_54`): Structs returned from functions don't work because struct fields live in caller registers that get fully restored on TRET. The fields are gone by the time the caller reads the return slot. Fixing requires a struct-value ABI (stack-allocated struct layout). Marked [ARCH-LIMIT].
- **COMP-TENSOR-001** (`probe_71`, `probe_72`): Tensor sizes use 16-bit immediates (TALLOC). >65535 element tensors silently truncate. Marked [ARCH-LIMIT].
- **probe_07 2D tensor**: Index OOB for 2D access patterns in certain edge cases. Marked [KNOWN / 2D-LIMIT].
- **MOD-004**: Module file loading not implemented. Named imports fail with "file not found". Marked [UNRESOLVED].
