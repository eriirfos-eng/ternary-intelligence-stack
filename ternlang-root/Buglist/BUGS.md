# Compiler & VM Bug Collection

This file tracks known bugs in the Ternlang compiler and VM.

## [BUG-1] ForIn Register Leak — FIXED (2026-04-16)
- **Description:** Each `for x in tensor` loop allocated 4 internal registers (it_reg, r_reg, i_reg, v_reg) without releasing them. After ~6–7 loops in a single function, the register file was exhausted, causing silent dropped stores and zero-value loads.
- **Fix:** `pre_loop_reg` snapshot + `self.next_reg = pre_loop_reg` restore + `self.symbols.remove(var)` in `betbc.rs`.
- **Regression Test:** `stdlib/bughunt/probe_98_forin_nested.tern` (expected output: 65)
- **Status:** FIXED

## [BUG-2] Hard Register File Limit — FIXED (2026-04-16)
- **Description:** VM register file was `[Value; 27]`. Functions with >27 locals silently corrupted: stores dropped, loads returned zero.
- **Fix:** Changed to `Vec<Value>` with auto-grow in TSTORE/TLOAD. `alloc_reg()` warns at >255.
- **Regression Test:** `stdlib/bughunt/probe_99_register_stress.tern` (30 locals, expected sum: 435)
- **Status:** FIXED

## [BUG-3] NodeId Hardcoded Emission — FIXED (2026-04-16)
- **Description:** `Expr::NodeId` emitted a hardcoded `"127.0.0.1:7373"` string into bytecode at compile time, ignoring `--node-addr` / `vm.set_node_id()` at runtime. Distributed modules always announced the wrong address.
- **Fix:** Added opcode `0x36` (TNODEID) to `vm/mod.rs` that pushes `Value::String(self.node_id.clone())`. Updated `betbc.rs` `Expr::NodeId` to emit `0x36` instead of hardcoded bytes.
- **Regression Test:** `stdlib/bughunt/probe_97_nodeid_runtime.tern` (default: `127.0.0.1`; `--node-addr 10.0.0.1:9000`: `10.0.0.1:9000`)
- **Status:** FIXED


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

## [COMP-OP-001] Sparseskip Annotation is No-Op
- **Description:** The `@sparseskip` annotation on functions is correctly parsed but does not result in the emission of the `TSPARSE_MATMUL` (0x07) opcode in the compiled bytecode. The compiler appears to treat it as a standard function.
- **Error:** Bytecode contains standard call logic instead of optimized sparse matrix multiplication.
- **Workaround:** None. Requires fixing the code generation in the compiler.
- **Regression Test:** `stdlib/bughunt/probe_68_sparse_skip.tern`

## [PARSER-STR-001] Missing String Concatenation
- **Description:** The language lacks support for string concatenation using the `+` operator. Attempting to use `+` with strings results in a runtime type mismatch as the VM expects numeric types.
- **Error:** `VM Error: [BET-007] Runtime type mismatch — expected Numeric but found (String(...), String(...)).`
- **Workaround:** Print strings sequentially or use separate variables.
- **Regression Test:** `stdlib/bughunt/probe_67_string_concat.tern`

## [COMP-TENSOR-001] Tensor Size Truncation (16-bit)
- **Description:** The compiler truncates `trittensor<N>` sizes to 16 bits (0-65535) during bytecode emission. Allocating a tensor with a size like 1,000,000 results in a tensor of size 16,960 (1,000,000 % 65536).
- **Error:** `VM Error: [BET-008] Tensor[0]: index ... is out of bounds — tensor only has <truncated size> element(s).`
- **Workaround:** Keep tensor sizes below 65,536 or use multiple tensors.
- **Regression Test:** `stdlib/bughunt/probe_71_large_tensor.tern`, `stdlib/bughunt/probe_72_tensor_limit.tern`

## [COMP-BOOL-001] True/False Literal Stack Underflow
- **Description:** Using the boolean literals `true` or `false` in `if` or `while` conditions results in a VM stack underflow. The compiler appears to emit incorrect bytecode for these literals.
- **Error:** `VM Error: [BET-001] Stack underflow — you tried to pop a truth that wasn't there.`
- **Workaround:** Use integer `1` (true) / `0` (false) or trit `affirm` / `hold` instead.
- **Regression Test:** `stdlib/bughunt/probe_73_inf_loop.tern`, `stdlib/bughunt/probe_78_if_true.tern`

## [VM-BUILTIN-001] Missing/Broken Math Built-ins
- **Description:** Basic math built-ins like `abs(int)`, `pow(int, int)`, `sqrt(float)`, `min(int, int)`, and `max(int, int)` are either completely missing from the global scope or defined in a way that causes an immediate stack overflow.
- **Error:** `VM Error: [BET-013] Call stack overflow — max depth (4096) exceeded.`
- **Workaround:** Explicitly import from `std::trit` for trit versions, or implement manually for `int`.
- **Regression Test:** `stdlib/bughunt/probe_79_math_builtins.tern`, `stdlib/bughunt/probe_81_pow_test.tern`

## [VM-BUILTIN-002] Missing/Broken Trit Built-ins (invert, len)
- **Description:** The `invert(trit)` and `len(array)` built-ins, despite being documented and used in examples, cause stack overflows or are missing during execution.
- **Error:** `VM Error: [BET-013] Call stack overflow — max depth (4096) exceeded.`
- **Workaround:** For `invert`, use a manual `match`. No workaround for `len`.
- **Regression Test:** `stdlib/bughunt/probe_09_trit_builtins.tern`, `stdlib/bughunt/probe_82_array_methods.tern`

## [PARSER-FN-001] Missing First-Class Functions
- **Description:** The parser does not support passing functions as arguments or assigning them to variables. The `fn(...) -> ...` type syntax is not recognized.
- **Error:** `Parse program error: UnexpectedToken("Fn")`
- **Workaround:** None. First-class functions are not supported.
- **Regression Test:** `stdlib/bughunt/probe_88_first_class_fns.tern`

## [PARSER-FN-002] Missing Nested Functions
- **Description:** Defining a function inside another function is not supported.
- **Error:** `Parse program error: UnexpectedToken("Fn")`
- **Workaround:** Define all functions at the top level or module level.
- **Regression Test:** `stdlib/bughunt/probe_87_nested_fns.tern`

## [PARSER-STRUCT-002] Missing Struct Methods
- **Description:** Receiver-style method syntax (e.g., `fn (s: MyStruct) my_method()`) is not supported.
- **Error:** `Parse program error: ExpectedToken("function name", "LParen")`
- **Workaround:** Use regular functions that take the struct as the first argument.
- **Regression Test:** `stdlib/bughunt/probe_86_struct_methods.tern`

## [PARSER-LIT-001] Missing Non-Decimal Literals
- **Description:** Hexadecimal (`0xFF`) and Binary (`0b1010`) integer literals are not supported.
- **Error:** `Parse program error: ExpectedToken("Semicolon", "Ident(...)")`
- **Workaround:** Use decimal representations only.
- **Regression Test:** `stdlib/bughunt/probe_91_literals.tern`

## [PARSER-LIT-002] Missing Character Literals
- **Description:** Character literals (e.g., `'A'`) are not supported.
- **Error:** `Parse program error: UnexpectedToken("Invalid token")`
- **Workaround:** Use `int` ASCII/Unicode values.
- **Regression Test:** `stdlib/bughunt/probe_92_chars.tern`

## [CLI-DISPLAY-001] CLI Register Display Bug
- **Description:** The `ternlang-cli` always reports `Reg 0: trit(tend)` (and other registers as `tend`) after program execution, regardless of the actual return value of the program or the state of the registers.
- **Error:** Incorrect register values displayed in CLI output.
- **Workaround:** Use `println()` to verify actual values during execution.
- **Regression Test:** `test_println.tern`, `test_return_1.tern`

## [PARSER-AGENT-001] Missing Agent Fields (State)
- **Description:** Agents do not support field declarations (e.g., `let count: int = 0;`). The parser only expects function definitions within an `agent` block.
- **Error:** `Parse stmt error: UnexpectedToken("Agent")` (when trying to define a field inside an agent).
- **Workaround:** None. Agents are currently stateless between `handle` calls unless external persistence is used.
- **Regression Test:** `stdlib/bughunt/probe_94_agent_mailbox.tern`

## [VM-AGENT-001] Non-Blocking Await
- **Description:** The `await` expression does not block if the agent's mailbox is empty. Instead, it immediately returns `tend` (0). This prevents standard actor-model patterns where an agent waits for a message.
- **Error:** Logical error; `await` on an empty mailbox returns `tend` instead of blocking or yielding.
- **Workaround:** Use a loop to poll the agent until a non-zero result is received (if applicable).
- **Regression Test:** `stdlib/bughunt/probe_95_agent_concurrency.tern`

## [VM-AGENT-002] Synchronous Agent Execution
- **Description:** Despite the "distributed actor" branding, local agents appear to run synchronously on the same thread as the caller during an `await` call. This is confirmed by the VM implementation in `compiler/legacy_shim/ternlang-core/src/vm/mod.rs` (opcode 0x32).
- **Error:** Lack of true local concurrency.
- **Workaround:** Use remote agents (Phase 5.1) for true parallelism.
- **Regression Test:** `stdlib/bughunt/probe_93_agent_test.tern`

