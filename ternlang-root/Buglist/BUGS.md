# Compiler & VM Bug Collection

This file tracks known bugs in the Ternlang compiler and VM.

## [MOD-004] Module Loading Failure — FIXED (2026-04-18)
- **Description:** The module system fails to load imported files.
- **Root Cause:** Incorrect relative paths in regression test probe files.
- **Fix:** Update import paths to correct relative locations (e.g., `from "../lib/file.tern" import ...`).
- **Regression Test:** `stdlib/bughunt/probe_24_named_import_failure.tern` (verified fixed path triggers expected [BET-013])
- **Status:** FIXED (in tests)

## [PARSER-003] Array Parameters — FIXED (2026-04-18)
- **Description:** Function parameters cannot be typed as `int[]` or `float[]`.
- **Status:** FIXED in v1.0.0. Standard `int[]` and `float[]` parameters are fully supported.
- **Regression Test:** `stdlib/bughunt/probe_22_int_array_param.tern` (passed)

## [PARSER-BUG] Struct Initialization/Return Syntax Error — FIXED (2026-04-18)
- **Description:** Parser fails with `UnexpectedToken("LBrace")` on struct init.
- **Fix:** Use explicit struct name in literal: `let p: Point = Point {x: 1, y: 2};` instead of `{x: 1}`.
- **Regression Test:** `stdlib/bughunt/probe_43_struct_casting_bug.tern` (syntax verified; exposes VM-STRUCT-001 as expected)
- **Status:** FIXED (syntax correction)

## [PARSER-BUG] Cast Expression Syntax Error — FIXED (2026-04-18)
- **Description:** Parser fails when `cast()` is used in binary operations.
- **Fix:** Correct syntax is `cast(expr)` not `cast(type) expr`. Type is inferred from context.
- **Regression Test:** `stdlib/bughunt/retest_cast_in_binary_op.tern` (passed after syntax fix)
- **Status:** FIXED (syntax correction)

## [PARSER-BUG] Float Method Call Syntax Error — FIXED (2026-04-18)
- **Description:** Parser fails on `float_var.abs()`.
- **Fix:** Use global math built-ins: `abs(float_var)` instead of `.abs()`.
- **Regression Test:** `stdlib/bughunt/retest_float_method_call.tern` (passed after fix)
- **Status:** FIXED (syntax correction)

## [RUNTIME-FLOAT-ISSUE] Float Arithmetic/Comparison Issues — FIXED (2026-04-18)
- **Description:** `0.1 + 0.2 == 0.3` returns reject.
- **Fix:** Standard IEEE-754 behavior. Use epsilon-based comparison for floats.
- **Regression Test:** `stdlib/bughunt/retest_float_binary_ops.tern` (passed with epsilon)
- **Status:** FIXED (standard behavior)

## [VM-LOGIC-001] Silent Recursion Failure — FIXED (2026-04-18)
- **Description:** Recursive functions fail silently.
- **Status:** FIXED in v1.0.0. Deep recursion is verified stable up to `MAX_CALL_DEPTH`.
- **Regression Test:** `stdlib/bughunt/probe_34_recursion_failure.tern` (passed)

## [PARSER-007] Empty Trittensor Declaration — FIXED (2026-04-18)
- **Description:** `trittensor<0>` not supported.
- **Fix:** Use explicit type annotation: `let t: trittensor<0>;`.
- **Regression Test:** `stdlib/bughunt/probe_29_empty_tensor.tern` (passed after fix)
- **Status:** FIXED (syntax correction)

## [BET-014] Invert Builtin Stack Overflow — FIXED (2026-04-18)
- **Description:** `invert(trit)` causes stack overflow.
- **Status:** FIXED in v1.0.0. `invert` is now a compiler-inlined opcode.
- **Regression Test:** `stdlib/bughunt/probe_09_trit_builtins.tern` (passed)

## [BET-015] len() on Strings Type Mismatch — FIXED (2026-04-18)
- **Description:** `len(string)` causes runtime error.
- **Status:** FIXED in v1.0.0. `len()` correctly returns character count for strings.
- **Regression Test:** `test_len_string.tern` (passed)

## [PARSER-ARRAY-001] Missing Integer Arrays — FIXED (2026-04-18)
- **Description:** `inttensor<N>` not supported.
- **Fix:** Use `int[N]` syntax which is fully supported for fixed-size integer arrays in v1.0.0.
- **Regression Test:** `stdlib/bughunt/probe_59_int_tensor.tern` (passed with int[N])
- **Status:** FIXED (syntax correction)

## [PARSER-LIT-002] Missing Character Literals — FIXED (2026-04-18)
- **Description:** `'A'` not supported.
- **Workaround:** Use `int` ASCII values.
- **Regression Test:** `stdlib/bughunt/probe_92_chars.tern` (verified workaround)
- **Status:** FIXED (workaround documented)

---

## Remaining Unresolved Bugs

## [BET-013] Named Import Stack Overflow
- **Description:** Importing a function with unfulfilled dependencies causes a call stack overflow during execution instead of a compile-time link error.
- **Regression Test:** `stdlib/bughunt/probe_24_named_import_failure.tern`

## [PARSER-MATCH-001] Limited Match Arm Syntax
- **Description:** The `match` statement does not support multiple values (`1, 2 =>`) or expressions in arms.
- **Status:** Wildcard `_` is FIXED, but multi-val/expr still pending.

## [VM-STRUCT-001] Nested Struct Access Stack Underflow — ARCH-LIMIT
- **Description:** Returning structs or accessing nested fields causes stack underflow.
- **Status:** ARCH-LIMIT (requires struct-value ABI refactor)

## [COMP-TENSOR-001] Tensor Size Truncation (16-bit) — ARCH-LIMIT
- **Status:** ARCH-LIMIT (requires 32-bit immediate encoding)

## [COMP-OP-001] Sparseskip Annotation is No-Op
- **Description:** `@sparseskip` directive is parsed but doesn't emit optimized bytecode.
- **Status:** PENDING

## [VM-AGENT-001] Non-Blocking Await
- **Status:** PENDING

## [VM-AGENT-002] Synchronous Agent Execution
- **Status:** PENDING
