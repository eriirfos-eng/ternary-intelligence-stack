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

To fix **[PARSER-009]**, the `parse_primary_expr` function should validate the values inside a `TritTensorLiteral` block and return a new `InvalidTritLiteralValue` error if they are out of range.

**1. Update `ParseError` Enum:**

```rust
#[derive(Debug)]
pub enum ParseError {
    // ... existing ...
    EmptyArrayLiteral,
    // New error type for PARSER-009
    InvalidTritLiteralValue(String),
}
```

**2. Update `fmt::Display` Implementation:**

```rust
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ... existing arms ...
            Self::EmptyArrayLiteral =>
                write!(f, "[PARSER-008] Empty array literal `[]` is not supported..."),
            
            // New error message:
            Self::InvalidTritLiteralValue(val) =>
                write!(f, "[PARSER-009] Invalid value '{val}' in trittensor literal. Only -1, 0, and 1 are allowed."),
        }
    }
}
```

---

## Proposed Rust Implementation for New VM Errors

**File to Edit:** `compiler/legacy_shim/ternlang-core/src/vm/mod.rs`

**1. Update `VmError` Enum:**

Add variants for the integer overflow, invalid trit value, and invalid cast errors.

```rust
pub enum VmError {
    // ... existing ...
    RuntimeError(String),
    CallStackOverflow,
    // New error types:
    IntegerOverflow,
    InvalidTritValue(i64),
    InvalidCast(String), // For invalid casting operations
}
```

**2. Update `fmt::Display` Implementation:**

Add new match arms. The VM's arithmetic opcodes should use `checked_...` methods and return `IntegerOverflow`. The VM's loop and value-handling logic should validate trits and return `InvalidTritValue` instead of panicking. The VM's casting operations should check for valid conversions and return `InvalidCast`.

```rust
impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ... existing arms ...
            Self::CallStackOverflow => write!(f, "[BET-013] Call stack overflow..."),
            
            // New error messages:
            Self::IntegerOverflow => write!(f, "[BET-014] Integer overflow. You tried to count past the stars and ran out of numbers."),
            Self::InvalidTritValue(val) => write!(f, "[BET-015] Invalid trit value: {val}. The VM found a pretender in its midst."),
            Self::InvalidCast(msg) => write!(f, "[BET-016] Invalid cast: {msg}. Cannot convert value to target type."),
        }
    }
}
```
