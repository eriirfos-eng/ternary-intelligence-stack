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

## [VM-STRUCT-001] Nested Struct Access Stack Underflow — FIXED (2026-04-29)
- **Description:** Returning structs or accessing nested fields caused stack underflow.
- **Fix:** Implemented native `Value::Struct` heap-allocated variant and `Tstruct`/`Tfield` opcodes.
- **Status:** FIXED in v1.2.1

## [COMP-TENSOR-001] Tensor Size Truncation (16-bit) — FIXED (2026-04-29)
- **Description:** Tensor dimensions were capped at 65,535 (u16).
- **Fix:** Upgraded Talloc immediates to 32-bit (u32), supporting up to 4.29B elements.
- **Status:** FIXED in v1.2.1

## [CLI-MATCH-001] Non-Exhaustive Match in CLI — FIXED (2026-04-29)
- **Description:** `ternlang-cli` failed to compile after adding `Value::Struct` due to non-exhaustive matches in `format_value` and REPL.
- **Fix:** Added `Value::Struct` handlers to `main.rs`.
- **Status:** FIXED in v1.2.1

---

---

## TernStudio (Frontend) Bugs — Session 2026-05-01

### [STUDIO-001] Deploy Modal Syntax Error — FIXED (2026-05-01)
- **Description:** Uncaught SyntaxError: invalid assignment left-hand side at studio.js:6152
- **Root Cause:** Invalid `||` operator syntax: `doc.getElementById() || el.style.display` 
- **Fix:** Wrap OR expressions in parentheses: `(el1 || el2).style.display = "flex"`
- **Status:** FIXED in commit 3b7096dfa

### [STUDIO-002] Wire Persistence Lost on View Switch — FIXED (2026-05-01)
- **Description:** Cables disappear when switching from flow view to editor and back to flow
- **Root Cause:** `renderFlow()` only called `updateWires()` if `flowNodes.length === 0`; existing wires not re-rendered when switching back
- **Fix:** Added `else` branch to always call `updateWires()` when nodes exist
- **Status:** FIXED in commit a379d0d59

### [STUDIO-003] Albert Panel Cables Not Rendering on Toggle — FIXED (2026-05-01)
- **Description:** F6 hotkey opens Albert panel but cables to/from it aren't visible; reconnection attempts show "already connected"
- **Root Cause:** Panel visibility toggle (`showPanel()`/`hidePanel()`) didn't call `updateWires()` to re-render wires
- **Fix:** Added `updateWires()` calls in both `showPanel()` and `hidePanel()` functions
- **Status:** FIXED in commit 9447e58cd

### [STUDIO-004] Inspector Drag Null Dereference — FIXED (2026-05-01)
- **Description:** Crash with "Cannot read property 'left' of null" when dragging inspector window
- **Root Cause:** `dragParentRect` initialized to null; accessed without null check in `onMouseMove()`
- **Fix:** Added null check: `if (isDragging && dragParentRect)`
- **Status:** FIXED in commit 437512e8d

### [STUDIO-005] Null Element Access in Initialization — FIXED (2026-05-01)
- **Description:** Potential crash if `apiEndpoint` element missing during early module load
- **Root Cause:** Direct property assignment without null check: `document.getElementById('apiEndpoint').value = origin`
- **Fix:** Added null check before accessing element property
- **Status:** FIXED in commit 437512e8d

### [STUDIO-006] Duplicate Event Listeners on Drag/Resize — FIXED (2026-05-01)
- **Description:** Multiple mousemove/mouseup listeners accumulate on repeated inspector interactions, degrading performance
- **Root Cause:** `addEventListener()` called on each mousedown without removing previous listeners
- **Fix:** Call `removeEventListener()` before `addEventListener()` to prevent accumulation
- **Status:** FIXED in commit 437512e8d

### [STUDIO-007] F6 Cables Hidden Even After Panel Show — FIXED (2026-05-01)
- **Description:** Albert panel wires created while panel is hidden stay invisible even after showing panel
- **Root Cause:** `updateWires()` had condition to skip Albert wires if panel is hidden, but no force re-render on visibility change
- **Fix:** Removed hidden-state skip condition; always render Albert wires if endpoints exist
- **Status:** FIXED in commit a6f2e1k23

### [STUDIO-008] Registry Auto-Adds Deprecated Entries — FIXED (2026-05-01)
- **Description:** Old local node entries ("Agent", "Mesh_Node_A") persistently reappear in Deployed Architectures
- **Root Cause:** Migration code auto-added deprecated entries if missing from localStorage
- **Fix:** Changed migration to remove deprecated IDs instead of adding them
- **Status:** FIXED in commit a6f2e1k23

---

## Training Bugs (albert. MoE-13)

### [TRAINING-001] Gate Reset Footgun — FIXED (2026-05-15)
- **Description:** On every training restart, kaiming-uniform weight init and expert noise injection were applied unconditionally, resetting gate layers even on clean checkpoint resumption. This broke symmetry that had already been resolved by the model, effectively undoing hundreds of epochs of gate differentiation.
- **Root Cause:** Both reset blocks in the training loop ran on every startup without checking whether the checkpoint was a fresh initialization or a resume.
- **Impact:** Estimated ~500 wasted epochs before detection. Symptoms: gate entropy oscillating wildly, routing not stabilizing across restarts.
- **Fix:** Both reset blocks now gated behind `--break-symmetry` CLI flag + EvolutionManager entropy auto-detection. A restart without the flag will never reset gates.
- **Status:** FIXED in commit (2026-05-15). Both reset paths verified conditional.

## Remaining Unresolved Bugs
