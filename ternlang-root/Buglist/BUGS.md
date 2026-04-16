# Compiler & VM Bug Collection

This file tracks known bugs in the Ternlang compiler and VM.

## [MOD-004] Module Loading Failure
- **Description:** The module system fails to load imported files, reporting them as not found or not readable. This prevents the testing of import-related bugs, such as `[BET-013] Named Import Stack Overflow`, as the necessary dependency files cannot be accessed by the VM during execution.
- **Error:** `[MOD-004] Could not load file '<filename>' — file not found or not readable.`
- **Workaround:** None identified. Requires fixing the module loading mechanism.
- **Regression Test:** `stdlib/bughunt/probe_24_named_import_failure.tern`

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
- **Description:** A function must be defined before it is called. If a function is called before its definition, it fails silently by returning an incorrect value (e.g., `reject` in regression tests, instead of the expected `truth` or a panic). This indicates an issue with the compiler's or VM's symbol resolution or call stack management for forward references.
- **Error:** Returns an incorrect value, causing the program to exit with `reject` in regression tests, rather than a compile-time error or a clear runtime failure.
- **Workaround:** Define functions before they are called.
- **Regression Test:** `stdlib/bughunt/probe_23_forward_reference.tern`

## [BET-013] Named Import Stack Overflow
- **Description:** This bug is currently blocked by a module loading failure (`[MOD-004]`). The intended test case involves importing a function with unfulfilled dependencies, which is expected to cause a `VM Error: [BET-013] Call stack overflow`. However, the import itself fails before the VM can execute the faulty logic.
- **Error:** Blocked by module loading failure. Expected `VM Error: [BET-013] Call stack overflow`.
- **Workaround:** Not applicable until module loading is fixed.
- **Regression Test:** `stdlib/bughunt/probe_24_named_import_failure.tern` (This test currently fails due to module loading).

## [PARSER-BUG] Struct Initialization/Return Syntax Error
- **Description:** The parser fails when attempting to initialize or return structs, particularly when casting or complex expressions are involved within the struct definition or function return. The specific error `Parse program error: UnexpectedToken("LBrace")` suggests the parser is misinterpreting struct syntax or related casting operations, expecting a block `{}` incorrectly. This prevents testing deeper VM issues like stack underflow related to struct returns.
- **Error:** `Parse program error: UnexpectedToken("LBrace")`
- **Workaround:** None currently. Struct syntax in these contexts is not parsable.
- **Regression Test:** `stdlib/bughunt/probe_43_struct_casting_bug.tern`

## [PARSER-BUG] Cast Expression Syntax Error (revisited)
- **Description:** The parser fails when a `cast()` expression is used in various contexts, including within binary operations and as a standalone expression followed by a method call. The error suggests it expects a block `{}` instead of continuing the expression. This indicates a fundamental issue with how the parser handles `cast` expressions.
- **Error:** `Parse program error: ExpectedToken("Semicolon", "Ident("neg_trit")")` or `Parse program error: ExpectedToken("RParen", "Ident("val_int")")` (depending on the context).
- **Workaround:** Avoid using `cast` directly in complex expressions or method calls; use intermediate variables to store the cast result first.
- **Regression Test:** `stdlib/bughunt/retest_cast_in_binary_op.tern`

## [PARSER-BUG] Float Method Call Syntax Error
- **Description:** The parser fails when attempting to call methods on float variables (e.g., `float_var.abs()`). The parser incorrectly expects a semicolon after the variable name or the start of a block `{}` instead of recognizing the dot notation for method calls.
- **Error:** `Parse program error: ExpectedToken("Semicolon", "LParen")`
- **Workaround:** None. Method calls on floats are not supported or are not parsable.
- **Regression Test:** `stdlib/bughunt/retest_float_method_call.tern`

## [RUNTIME-FLOAT-ISSUE] Float Arithmetic/Comparison Issues
- **Description:** Basic float arithmetic (e.g., addition `0.1 + 0.2 == 0.3`, division `1.0 / 3.0`) and comparisons appear to function at a basic level, but the test `retest_float_binary_ops.tern` exited with `reject`, indicating that either the arithmetic result, the comparison logic, or both are not producing the expected outcome. This could be due to precision limitations or a logic error in the VM's float handling.
- **Error:** `Program exited with error (Reject state).`
- **Workaround:** Avoid complex float calculations or rely on exact ternary representations where possible.
- **Regression Test:** `stdlib/bughunt/retest_float_binary_ops.tern`

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

## [PARSER-007] Empty Trittensor Declaration
- **Description:** Declaring a `trittensor` without an explicit type and initialization is not supported. `trittensor<0>` is not supported.
- **Error:** `Parse program error: ExpectedToken("Colon", "Assign")`
- **Workaround:** Explicitly type and initialize the tensor, e.g., `let my_tensor: trittensor<1> = [0];`. A true empty tensor is not supported.
- **Regression Test:** `stdlib/bughunt/probe_29_empty_tensor.tern`
- **Workaround Example:** `stdlib/bughunt/probe_29_empty_tensor_workaround.tern`

## [BET-014] Invert Builtin Stack Overflow
- **Description:** Calling the built-in `invert(trit)` function causes a VM stack overflow, even in non-recursive contexts.
- **Error:** `VM Error: [BET-013] Call stack overflow — max depth (4096) exceeded.`
- **Workaround:** Manually invert trits using `match` or `if/else` (though `match` has its own issues).
- **Regression Test:** `stdlib/bughunt/probe_09_trit_builtins.tern` (also reproduced in `test_invert.tern`)

## [VM-MATCH-001] Match Arm Leak & Type Pollution
- **Description:** If a `match` statement is given a value that is not covered by any of its arms, it fails by returning the *input value* itself. If the input is an `int`, this non-trit value is leaked into a `trit` register, polluting the ternary state.
- **Error:** Pollution of `trit` registers with `int` values. Subsequent use of this polluted register in built-ins (like `consensus`) causes a `VM Error: [BET-001] Stack underflow`.
- **Workaround:** Ensure all possible values are covered in `match` arms (but see `[PARSER-MATCH-001]` regarding lack of `_`).
- **Regression Test:** `stdlib/bughunt/probe_48_match_unhandled.tern`, `stdlib/bughunt/probe_53_match_leak.tern`

## [PARSER-MATCH-001] Limited Match Arm Syntax
- **Description:** The `match` statement is highly restrictive. It does not support:
    - Wildcard/Default arm (`_`).
    - Multiple values per arm (e.g., `1, 2 => ...`).
    - Expressions in arms (e.g., `a + b => ...`).
- **Error:** `Parse program error: ExpectedToken("pattern (int or trit)", ...)` or `ExpectedToken("FatArrow", "Comma")`.
- **Workaround:** Use nested `match` or `if/else` chains.
- **Regression Test:** `stdlib/bughunt/probe_49_match_expression.tern`, `stdlib/bughunt/probe_50_match_multi_val.tern`

## [VM-STRUCT-001] Nested Struct Access Stack Underflow
- **Description:** Accessing a field of a nested struct (e.g., `frame.origin.x`) causes a VM stack underflow. Direct access to top-level fields (e.g., `frame.id`) works correctly.
- **Error:** `VM Error: [BET-001] Stack underflow — you tried to pop a truth that wasn't there.`
- **Workaround:** Copy nested structs to local variables before accessing their fields.
- **Regression Test:** `stdlib/bughunt/probe_54_struct_chaos.tern`

## [VM-PANIC-001] Trittensor Non-Trit Panic
- **Description:** Storing a non-trit value (e.g., 2, -5) in a `trittensor<N>` fixed array causes a raw Rust panic in the compiler's `trit.rs` rather than a graceful VM error or compile-time type error.
- **Error:** `thread 'main' panicked at .../trit.rs: Invalid trit value: 2`
- **Workaround:** Ensure only valid trits (-1, 0, 1) are stored in `trittensor`.
- **Regression Test:** `stdlib/bughunt/probe_56_deep_nesting.tern`

## [VM-GLOBAL-001] Global Variable Access Stack Underflow
- **Description:** Accessing a global variable (defined at the top level with `let`) from within a function causes a VM stack underflow.
- **Error:** `VM Error: [BET-001] Stack underflow — you tried to pop a truth that wasn't there.`
- **Workaround:** Pass global state as function parameters.
- **Regression Test:** `stdlib/bughunt/probe_62_globals.tern`

## [PARSER-ARRAY-001] Missing Integer Arrays
- **Description:** The language lacks support for dynamic integer arrays (`int[]`) or fixed-size integer tensors (`inttensor<N>`). Only `trittensor<N>` is supported for fixed-size arrays.
- **Error:** `Parse program error: ExpectedToken("Semicolon", "LBracket")` or `ExpectedToken("Semicolon", "LAngle")`.
- **Workaround:** Use `trit[]` or `trittensor` if values are small, or separate variables.
- **Regression Test:** `stdlib/bughunt/probe_58_int_array.tern`, `stdlib/bughunt/probe_59_int_tensor.tern`

## [CLI-DISPLAY-001] CLI Register Display Bug
- **Description:** The `ternlang-cli` always reports `Reg 0: trit(tend)` (and other registers as `tend`) after program execution, regardless of the actual return value of the program or the state of the registers.
- **Error:** Incorrect register values displayed in CLI output.
- **Workaround:** Use `println()` to verify actual values during execution.
- **Regression Test:** `test_println.tern`, `test_return_1.tern`

## [PARSER-008] Empty Array Literal
- **Description:** Declaring an empty array literal using `let name: trit[] = []` causes a parser error.
- **Error:** `Parse program error: UnexpectedToken("tensor literal element: RBracket")`
- **Workaround:** There is no known workaround to create an empty dynamic array via a literal. It must be created by a function that returns an empty array.
- **Regression Test:** `stdlib/bughunt/probe_30_empty_array_literal.tern`
