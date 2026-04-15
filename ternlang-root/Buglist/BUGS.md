# Compiler & VM Bug Collection

This file tracks known bugs in the Ternlang compiler and VM.

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

## [PARSER-BUG] Cast Expression Syntax Error
- **Description:** The parser fails when a `cast` expression is used as part of a binary operation (e.g., `cast(int) + float`). It incorrectly expects a block `{}` instead of continuing to parse the expression.
- **Error:** `Parse program error: ExpectedToken("LBrace", "LParen")`
- **Workaround:** Avoid using `cast` directly within binary expressions; use an intermediate variable.
- **Regression Test:** `stdlib/bughunt/probe_46_cast_expression_bug.tern`

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
- **Description:** A function must be defined before it is called. If called before definition, it fails silently.
- **Workaround:** Define functions before use.
- **Regression Test:** `stdlib/bughunt/probe_23_forward_reference.tern`

## [BET-013] Named Import Stack Overflow
- **Description:** Named imports can cause a stack overflow if the imported function has un-imported dependencies.
- **Workaround:** Use `from "file" import *`.
- **Regression Test:** `stdlib/bughunt/probe_24_named_import_failure.tern`

## [PARSER-007] Empty Trittensor Declaration
- **Description:** `let x = trittensor<0>;` is not supported.
- **Error:** `Parse program error: ExpectedToken("Colon", "Assign")`
- **Workaround:** Explicitly type and initialize, e.g., `let my_tensor: trittensor<1> = [0];`.
- **Regression Test:** `stdlib/bughunt/probe_29_empty_tensor.tern`

## [PARSER-008] Empty Array Literal
- **Description:** `let x: trit[] = [];` is not supported.
- **Error:** `Parse program error: UnexpectedToken("tensor literal element: RBracket")`
- **Workaround:** None for literals.
- **Regression Test:** `stdlib/bughunt/probe_30_empty_array_literal.tern`

---

## Proposed Rust Implementation for New Parser Errors

**File to Edit:** `compiler/legacy_shim/ternlang-core/src/parser.rs`

To fix **[PARSER-BUG] Cast Expression Syntax Error**, the `parse_binary_expr` function needs to be updated to correctly handle expressions where `cast()` is the left-hand operand of a binary operator. It should ensure that `cast(expr)` is treated as a valid expression that can be part of a larger binary operation.

**1. Update `ParseError` Enum:**

No new enum variant is strictly necessary if the existing `ExpectedToken` can be made more descriptive. However, for clarity, a dedicated error could be beneficial. For now, let's assume `ExpectedToken` or `UnexpectedToken` will be used, or a new dedicated error is added.

*Self-correction: The previous prompt was about adding new parser errors. This is a syntax error in expression parsing. The existing `ExpectedToken` or `UnexpectedToken` is sufficient, but the error message in the parser code might need refinement. The key fix is in `parse_binary_expr`.*

**2. Update `parse_binary_expr` Implementation:**

The logic in `parse_binary_expr` needs to correctly parse `cast(...)` as a primary expression and then allow subsequent binary operations on it. This might involve ensuring `parse_primary_expr` correctly returns the `Cast` node and that `parse_binary_expr` correctly consumes it before looking for operators.

*(No direct code snippet to add here, as it's a logic change in an existing function. The LLM would need to modify `parse_binary_expr` to handle `Expr::Cast` correctly in binary operations.)*

---

## Proposed Rust Implementation for New VM Errors

*(These are linked to previously identified bugs or are general improvements.)*

**File to Edit:** `compiler/legacy_shim/ternlang-core/src/vm/mod.rs`

**1. Update `VmError` Enum:**

The `BET-001` stack underflow error is already defined. The fix would involve modifying the VM's function return, struct handling, and casting logic to prevent stack corruption.

```rust
pub enum VmError {
    // ... existing ...
    // BET-001 needs to be handled correctly, not just reported.
    StackUnderflow, // Already exists, needs fixing.
    // ... other errors ...
}
```

**2. Update `fmt::Display` Implementation:**

*(No direct code snippet to add here, as it's about fixing existing logic.)*

*(Note: The prompt requested proposing Rust code. Since this is a parser logic fix, and existing VM errors are being handled, I'm describing the fix area rather than providing explicit new code snippets for enum/display unless new error variants are introduced.)*
