# Compiler & VM Bug Collection

This file tracks known bugs in the Ternlang compiler and VM.

## [PARSER-BUG] Cast Expression Syntax Error
- **Description:** The parser fails when a `cast()` expression is used as part of a binary operation (e.g., `cast(int) + float` or `(cast(int) val_int) + val_float`). This indicates the parser cannot correctly handle `cast` expressions when they are operands in binary arithmetic. The error suggests it expects a block `{}` instead of continuing the expression. This bug persists.
- **Error:** `Parse program error: ExpectedToken("RParen", "Ident("val_int")")` (or similar, depending on the exact expression)
- **Workaround:** Avoid using `cast` directly within binary expressions; use an intermediate variable to store the cast result first.
- **Regression Test:** `stdlib/bughunt/retest_cast_in_binary_op.tern`

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
