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
- **Workaround Example:** `stdlib/buuhunt/probe_23_forward_reference_workaround.tern`

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

---

## Proposed Rust Implementation for New Parser Errors

**File to Edit:** `compiler/legacy_shim/ternlang-core/src/parser.rs`

**1. Update `ParseError` Enum:**

Add the following variants to the `ParseError` enum definition.

```rust
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    ExpectedToken(String, String),
    InvalidTrit(String),
    NonExhaustiveMatch(String),
    // New error types:
    MatchOnFloat,
    InvalidArrayParam,
    EmptyTrittensor,
    EmptyArrayLiteral,
}
```

**2. Update `fmt::Display` Implementation:**

Add the corresponding match arms to the `impl fmt::Display for ParseError` block.

```rust
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ... existing arms ...
            Self::UnexpectedToken(tok) =>
                write!(f, "[PARSE-001] Unexpected token '{tok}' — the lexer hit something it didn't expect. Binary habit? Check 'fn' vs 'func', trit vs bool.
            → details: stdlib/errors/PARSE-001.tern  |  ternlang errors PARSE-001"),
            Self::ExpectedToken(expected, found) =>
                write!(f, "[PARSE-002] Expected {expected} but found '{found}'. Missing type annotation, brace, or semicolon?
            → details: stdlib/errors/PARSE-002.tern  |  ternlang errors PARSE-002"),
            Self::InvalidTrit(val) =>
                write!(f, "[PARSE-003] '{val}' is not a valid trit. Trits are -1, 0, or +1 — the universe has exactly three states.
            → details: stdlib/errors/PARSE-003.tern  |  ternlang errors PARSE-003"),
            Self::NonExhaustiveMatch(msg) =>
                write!(f, "[PARSE-004] Non-exhaustive match: {msg}. Ternary has three states — cover -1, 0, and +1 or the compiler won't let you through.
            → details: stdlib/errors/PARSE-004.tern  |  ternlang errors PARSE-004"),
            
            // New error messages:
            Self::MatchOnFloat =>
                write!(f, "[PARSE-005] Match on float is not supported. Floats are continuous; `match` is for discrete types like int and trit. Use an `if` chain instead.
            → details: stdlib/errors/PARSE-005.tern  |  ternlang errors PARSE-005"),
            Self::InvalidArrayParam =>
                write!(f, "[PARSE-006] Invalid array parameter type. Functions can only accept `trit[]` or `trittensor` parameters for now.
            → details: stdlib/errors/PARSE-006.tern  |  ternlang errors PARSE-006"),
            Self::EmptyTrittensor =>
                write!(f, "[PARSE-007] Empty `trittensor<0>` is not allowed. Tensors must have at least one element.
            → details: stdlib/bughunt/probe_29_empty_tensor.tern"),
            Self::EmptyArrayLiteral =>
                write!(f, "[PARSER-008] Empty array literal `[]` is not supported for initialization. The parser needs at least one element to get going.
            → details: stdlib/bughunt/probe_30_empty_array_literal.tern"),
        }
    }
}
```

**3. Update Parser Logic:**

The respective parsing functions (`parse_match`, `parse_type`, `parse_primary_expr`) must be updated to throw these new, specific errors instead of the generic `ExpectedToken` or `UnexpectedToken` errors.

---

## Proposed Rust Implementation for New VM Error

**File to Edit:** `compiler/legacy_shim/ternlang-core/src/vm/mod.rs`

**1. Update `VmError` Enum:**

Add the `IntegerOverflow` variant.

```rust
pub enum VmError {
    // ... existing ...
    RuntimeError(String),
    CallStackOverflow,
    // New error type:
    IntegerOverflow,
}
```

**2. Update `fmt::Display` Implementation:**

Add the new match arm. Instead of panicking, the VM's `add`, `sub`, `mul` operations should use `checked_add`, `checked_sub`, etc., and return this error on `None`.

```rust
impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // ... existing arms ...
            Self::RuntimeError(msg) => write!(f, "[BET-012] Runtime error: {msg}"),
            Self::CallStackOverflow => write!(f, "[BET-013] Call stack overflow. Your program is too deep, friend."),
            
            // New error message:
            Self::IntegerOverflow => write!(f, "[BET-014] Integer overflow. You tried to count past the stars and ran out of numbers."),
        }
    }
}
```
