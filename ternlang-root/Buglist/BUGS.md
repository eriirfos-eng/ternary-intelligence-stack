# Compiler & VM Bug Collection

This file tracks known bugs in the Ternlang compiler and VM.

## [VM-LOGIC-001] Silent Recursion Failure
- **Description:** Recursive functions fail silently, producing incorrect results instead of crashing or returning an error. This points to a logic error in the VM's handling of the call stack or related opcodes during recursion.
- **Error:** None. The program returns an incorrect value, causing silent data corruption.
- **Workaround:** Avoid recursive functions. Use loops instead.
- **Regression Test:** `stdlib/bughunt/probe_34_recursion_failure.tern`

## [RUNTIME-PANIC] Integer Overflow
- **Description:** An integer overflow or underflow causes the VM to exit with a raw Rust panic instead of a graceful `VmError`.
- **Error:** `thread 'main' panicked at [...] attempt to add with overflow`
- **Workaround:** Manually check integer bounds before performing arithmetic that may overflow.
- **Regression Test:** `stdlib/bughunt/probe_31_int_overflow.tern`

## [PARSER-009] Invalid Trittensor Literal
- **Description:** The parser allows integer literals outside the valid range of [-1, 0, 1] inside a `trittensor` literal.
- **Error:** None at parse time. The parser should reject this code but doesn't. This leads to a runtime panic.
- **Workaround:** Manually ensure all values in a `trittensor` literal are valid trits.
- **Regression Test:** `stdlib/bughunt/probe_37_invalid_tensor_literal.tern`

## [RUNTIME-PANIC] Invalid Trit Value
- **Description:** The VM panics when it encounters a non-trit integer value (e.g., `5`) when executing a program, for example when iterating with a `for` loop. It should return a graceful `BET-007` error.
- **Error:** `thread 'main' panicked at 'Invalid trit value: 5'`
- **Workaround:** None. This is a fundamental VM safety issue.
- **Regression Test:** `stdlib/bughunt/probe_37_invalid_tensor_literal.tern` (This test triggers the panic).

## [VM-LOGIC-002] Casting Errors
- **Description:** Type casting, particularly involving negative numbers (trit to float, float to int), produces incorrect results, leading to silent data corruption or incorrect program behavior.
- **Error:** Returns `reject` state when calculations involving casts are incorrect.
- **Workaround:** Avoid casting negative numbers or complex casting chains until fixed.
- **Regression Test:** `stdlib/bughunt/probe_41_casting_logic_bug.tern`

## [BET-001] For-Loop Break Stack Underflow
- **Description:** Using a `break` statement inside a `match` within a `for` loop can lead to `BET-001` Stack Underflow. This indicates an issue with stack management during complex control flow exits.
- **Error:** `VM Error: [BET-001] Stack underflow`
- **Workaround:** Avoid using `break` inside `match` statements within `for` loops.
- **Regression Test:** `stdlib/bughunt/probe_35_stress_test_v2.tern` (This test triggers the underflow).

## [PARSER-BUG] Float Expression Syntax Error
- **Description:** The parser fails when float literals or expressions involving floats are used in binary operations (e.g., `0.1 + 0.2`, `val1 - val2`). It incorrectly expects a block `{}` instead of continuing to parse the expression. This is similar to the `cast` expression bug.
- **Error:** `Parse program error: ExpectedToken("LBrace", "LParen")`
- **Workaround:** Avoid complex float expressions; use intermediate variables or simpler operations.
- **Regression Test:** `stdlib/bughunt/probe_45_mixed_type_arithmetic.tern` and `stdlib/bughunt/probe_47_float_precision.tern` (These tests trigger the parser error).

## [PARSER-002] Match on Floats
- **Description:** `match` statements do not support float literals.
- **Error:** `Parse program error: ExpectedToken("pattern (int or trit)", "Float(1.0)")`
- **Workaround:** Use `if` statement chains.
- **Regression Test:** `stdlib/bughunt/probe_21_float_match.tern`

## [PARSER-003] Array Parameters
- **Description:** Function parameters cannot be typed as `int[]` or `float[]`.
- **Error:** `Parse program error: ExpectedToken("RParen", "LBracket")`
- **Workaround:** Use `trit[]` or `trittensor` and cast/saturate values.
- **Regression Test:** `stdlib/bughunt/probe_22_int_array_param.tern`

## [TCALL-BUG] Forward Reference
- **Description:** A function must be defined before it is called. If called before definition, it fails silently by returning the wrong value (e.g., `tend` instead of the expected `truth`) rather than crashing.
- **Workaround:** Define functions before use.
- **Regression Test:** `stdlib/bughunt/probe_23_forward_reference.tern`

## [BET-013] Named Import Stack Overflow
- **Description:** Importing a function via `from "file" import func_a` without its local dependencies (e.g., `func_b` called by `func_a`) causes `VM Error: [BET-013] Call stack overflow`.
- **Workaround:** Use `from "file" import *`.
- **Regression Test:** `stdlib/bughunt/probe_24_named_import_failure.tern`

## [PARSER-007] Empty Trittensor Declaration
- **Description:** Declaring a `trittensor` without an explicit type and initialization is not supported. `trittensor<0>` is not supported.
- **Error:** `Parse program error: ExpectedToken("Colon", "Assign")`
- **Workaround:** Explicitly type and initialize the tensor, e.g., `let my_tensor: trittensor<1> = [0];`. A true empty tensor is not supported.
- **Regression Test:** `stdlib/bughunt/probe_29_empty_tensor.tern`

## [PARSER-008] Empty Array Literal
- **Description:** Declaring an empty array literal using `let name: trit[] = []` causes a parser error.
- **Error:** `Parse program error: UnexpectedToken("tensor literal element: RBracket")`
- **Workaround:** There is no known workaround to create an empty dynamic array via a literal. It must be created by a function that returns an empty array.
- **Regression Test:** `stdlib/bughunt/probe_30_empty_array_literal.tern`

---

## Proposed Rust Implementation for New Parser Errors

**File to Edit:** `compiler/legacy_shim/ternlang-core/src/parser.rs`

To fix **[PARSER-BUG] Float Expression Syntax Error**, the parser needs to be more robust in handling binary operations involving float literals and results of float expressions. This is similar to the `cast` expression issue. The parser should correctly identify float literals as part of expressions and not mistake them for block delimiters.

**1. Update `ParseError` Enum:**

No new enum variant is strictly necessary if the existing `ExpectedToken` or `UnexpectedToken` can be made more descriptive. However, a dedicated error for this specific issue could be added for clarity.

**2. Update Parser Logic:**

The logic in `parse_binary_expr` and `parse_primary_expr` needs to be refined. When encountering float literals or expressions that evaluate to floats, the parser must correctly continue parsing binary operators (`+`, `-`, `*`, `/`) rather than prematurely expecting a block (`LBrace`).

*(No direct code snippet to add here as it's a logic refinement in existing parsing functions.)*
