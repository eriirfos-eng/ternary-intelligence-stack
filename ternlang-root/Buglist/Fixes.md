# Ternlang Compiler & VM Buglist / Fixes
# RFI-IRFOS · 2026

This document tracks historical fixes and new bugs identified during standard library development.

## Historical Fixes (Pre-April 2026)

1. **Register isolation** — VM has a `register_stack`. `TCALL` saves current registers, `TRET` restores them. Caller registers are no longer clobbered by called functions.
2. **Integer literal support** — opcode `0x17` (`TPUSH_INT`) and `0x18` (`TADD_INT`) are implemented. The compiler can emit integer constants.
3. **1D Tensor (Trittensor) logic** — `TSHAPE`, `TIDX`, `TSET` detect 1D tensors correctly (no sqrt on length). DNA/NLP 1D arrays work.
4. **Match/loop codegen stability** — `TDUP` is emitted before conditional jumps. Match and `for..in` no longer cause stack underflows.
5. **Agent opcodes** — `spawn`, `send`, `await` are enabled. Agent `type_id`s are registered in the CLI.
6. **Variable reassignment** — `var = value;` (Set variant) is parsed and handled. Re-assigning a `let` variable works.
7. **Error reporting** — `TypeMismatch` ([BET-007]) returns the full descriptive string (e.g., "Expected Trit but found Int(15)") instead of a blank error.
8. **Consensus logic** — `consensus(a, b)` uses logical merge (1+1=1), not balanced ternary arithmetic (1+1=−1 with carry).

## New Fixes (April 10, 2026)

### 9. Decorated Function Parsing
- **Symptoms:** Compiler failed with `ExpectedToken("Fn", "At")` when encountering `@sparseskip` before a function definition.
- **Root Cause:** `parse_program` expected `Fn` immediately for functions and didn't handle the `@` (`At`) token. `Function` AST node lacked a `directive` field.
- **Fix:**
    - Added `directive: Option<String>` to `Function` struct in `ast.rs`.
    - Updated `parse_function` in `parser.rs` to handle optional `@` prefix.
    - Updated `BytecodeEmitter` in `betbc.rs` to handle `Stmt::Decorated` (previously ignored).
- **Status:** Verified. `@sparseskip` functions now compile and run.

### 10. Floating Point Literal and Arithmetic Support
- **Symptoms:** Using `float` literals like `0.5` caused parser errors (`ExpectedToken("field name", "TritLiteral")`). Numeric operations on floats were not implemented in the VM.
- **Root Cause:** 
    - `Lexer` lacked a Float regex; `0.5` was tokenized as `TritLiteral(0)`, `Dot`, `Int(5)`.
    - `AST` and `Parser` lacked `FloatLiteral` nodes.
    - `VM` `Value` enum lacked a `Float` variant.
    - `VM` opcodes for `Add`, `Mul`, etc., were restricted to `Trit` or `Int`.
- **Fix:**
    - Added high-priority `Float` regex to `lexer.rs`.
    - Added `FloatLiteral(f64)` to `Expr` in `ast.rs`.
    - Implemented polymorphic opcodes in `vm/mod.rs`: `0x02` (Add), `0x03` (Mul), `0x04` (Neg), `0x14` (Less), `0x15` (Greater) now handle `Trit`, `Int`, and `Float`.
    - Added `0x19` (`TpushFloat`) opcode to VM.
    - Updated `BytecodeEmitter` to emit `0x19` for float literals.
- **Status:** Verified. EMA deliberation gates and other float-based logic now work correctly.
