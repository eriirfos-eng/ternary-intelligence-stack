# Compiler & VM Bug Collection

This file tracks known bugs in the Ternlang compiler and VM.

## [PARSER-BUG] Cast Expression Syntax Error (revisited)
- **Description:** The parser fails when a `cast()` expression is used in various contexts, including within binary operations and as a standalone expression followed by a method call. The parser incorrectly expects a semicolon or a block `{}` instead of continuing to parse the expression. This indicates a fundamental issue with how the parser handles `cast` expressions.
- **Error:** `Parse program error: ExpectedToken("Semicolon", "Ident("neg_trit")")` or `Parse program error: ExpectedToken("RParen", "Ident("val_int")")` (depending on the context).
- **Workaround:** Avoid using `cast` directly in complex expressions or method calls; use intermediate variables to store the cast result first.
- **Regression Test:** `stdlib/bughunt/retest_cast_in_binary_op.tern`

## [VM-LOGIC-001] Silent Recursion Failure
- **Description:** Recursive functions fail silently, producing incorrect results instead of crashing or returning a clear error. The test `stdlib/bughunt/probe_34_recursion_failure.tern` shows that a function expected to return `4` instead resulted in the program exiting with `reject`, confirming a silent failure in the recursion logic.
- **Error:** Returns an incorrect value, causing the program to exit with `reject` in tests, rather than the expected `truth()` or a panic.
- **Workaround:** Avoid recursive functions; use loops instead.
- **Regression Test:** `stdlib/bughunt/probe_34_recursion_failure.tern`

## [RUNTIME-BEHAVIOR] Integer Overflow Graceful Handling
- **Description:** An integer overflow or underflow does not cause a raw Rust panic or a specific `VmError`. Instead, it appears to be handled gracefully, resulting in the program exiting with a `reject` state. This could indicate that Ternlang's `int` type has arbitrary precision, or that overflow is handled implicitly without a distinct error code. The original assumption of a raw panic was incorrect.
- **Error:** `Program exited with error (Reject state).` (Instead of panic)
- **Workaround:** If a panic is desired for overflow, manual bound checks are needed. If graceful `reject` is acceptable, no workaround is strictly needed for correctness but may be for desired behavior.
- **Regression Test:** `stdlib/bughunt/probe_31_int_overflow.tern`

## [PARSER-002] Match on Floats
- **Description:** `match` statements do not support float literals.
- **Error:** `Parse program error: ExpectedToken("pattern (int or trit)", "Float(1.0)")`
- **Workaround:** Use `if` statement chains.
- **Regression Test:** `stdlib/bughunt/probe_21_float_match.tern`
- **Workaround Example:** `stdlib/bughunt/probe_21_float_match_workaround.tern`

## [PARSER-003] Array Parameters
- **Description:** Function parameters cannot be typed as `int[]` or `float[]`.
- **Error:** `Parse program error: ExpectedToken("RParen", "LBracket")`
- **Workaround:** Use `trit[]` or `trittensor` and cast/saturate values.
- **Regression Test:** `stdlib/bughunt/probe_22_int_array_param.tern`
- **Workaround Example:** `stdlib/bughunt/probe_22_int_array_param_workaround.tern`

## [TCALL-BUG] Forward Reference
- **Description:** A function must be defined before it is called. If called before definition, it fails silently by returning the wrong value (e.g., `tend` instead of the expected `truth`) rather than crashing.
- **Workaround:** Define functions before use.
- **Regression Test:** `stdlib/bughunt/probe_23_forward_reference.tern`
- **Workaround Example:** `stdlib/bughunt/probe_23_forward_reference_workaround.tern`

## [BET-013] Named Import Stack Overflow
- **Description:** Importing a function via `from "file" import func_a` without its local dependencies (e.g., `func_b` called by `func_a`) causes `VM Error: [BET-013] Call stack overflow`.
- **Workaround:** Use `from "file" import *`.
- **Regression Test:** `stdlib/bughunt/probe_24_named_import_failure.tern`
- **Workaround Example:** `stdlib/bughunt/probe_24_named_import_workaround.tern`

## [PARSER-007] Empty Trittensor Declaration
- **Description:** Declaring a `trittensor` without an explicit type and initialization is not supported. `trittensor<0>` is not supported.
- **Error:** `Parse program error: ExpectedToken("Colon", "Assign")`
- **Workaround:** Explicitly type and initialize the tensor, e.g., `let my_tensor: trittensor<1> = [0];`. A true empty tensor is not supported.
- **Regression Test:** `stdlib/bughunt/probe_29_empty_tensor.tern`
- **Workaround Example:** `stdlib/bughunt/probe_29_empty_tensor_workaround.tern`

## [PARSER-008] Empty Array Literal
- **Description:** Declaring an empty array literal using `let name: trit[] = []` causes a parser error.
- **Error:** `Parse program error: UnexpectedToken("tensor literal element: RBracket")`
- **Workaround:** There is no known workaround to create an empty dynamic array via a literal. It must be created by a function that returns an empty array.
- **Regression Test:** `stdlib/bughunt/probe_30_empty_array_literal.tern`
