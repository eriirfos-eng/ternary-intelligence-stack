# GEMINI.md — Ternlang Precision Parameter Sheet
# RFI-IRFOS · ternlang.com · v1.0 (2026-04-10)
# Read this at session start. These values are ground truth. Do not guess.

---

## 1. VM Architecture — exact constants

| Parameter | Value | Note |
|-----------|-------|------|
| Registers | **27** (indices 0–26) | 3³ — "That's 3³. No more." |
| Register default | `trit(tend)` = 0 | Unset registers hold tend |
| Stack | unbounded Vec | Dynamic, no fixed limit |
| Call stack | unbounded Vec | Return addresses only |
| Carry register | `Trit::Tend` on init | Single trit |
| Agent node_id default | `"127.0.0.1:7373"` | Set via `set_node_id()` |
| Max tensor pool | unbounded Vec | Indexed by usize handle |

---

## 2. Value types — exact Rust enum

```
Value::Trit(Trit)          → trit scalar
Value::Int(i64)            → int
Value::Float(f64)          → float
Value::String(String)      → string
Value::TensorRef(usize)    → handle into tensors[]
Value::AgentRef(usize, Option<String>)  → spawned agent
```

Default value (unset register): `Value::Trit(Trit::Tend)` = 0

---

## 3. Trit values — both identifier forms (both compile, pick one and be consistent)

| Numeric | Rust enum | Built-in fn | Bare identifier | Meaning |
|---------|-----------|-------------|-----------------|---------|
| `1`  | `Trit::Affirm` | `truth()`    | `affirm` | positive signal, proceed |
| `0`  | `Trit::Tend`   | `hold()`     | `tend`   | insufficient evidence, wait |
| `-1` | `Trit::Reject` | `conflict()` | `reject` | negative signal, block |

**Rule:** Do NOT mix forms in one file. If the file uses `affirm`/`tend`/`reject`, stay with those. If it uses `truth()`/`hold()`/`conflict()`, stay with those. The compiler accepts both but mixing looks wrong.

---

## 4. Complete opcode table — BET VM

All opcodes as of 2026-04-10. Do not add opcodes to .tern files that aren't in this table.

| Opcode | Name | Stack effect | Immediate bytes | Notes |
|--------|------|-------------|-----------------|-------|
| `0x01` | Tpush | → val | 1 (trit: 0x01=affirm, 0x00=tend, 0xFF=reject) | push trit literal |
| `0x02` | Tadd | a, b → result | — | polymorphic: Trit/Int/Float |
| `0x03` | Tmul | a, b → result | — | polymorphic |
| `0x04` | Tneg | a → -a | — | polymorphic |
| `0x05` | TjmpPos | peek(+1) → jump | 2 (addr u16) | **PEEK not pop**; exact match Int(1) |
| `0x06` | TjmpZero | peek(0) → jump | 2 (addr u16) | **PEEK not pop**; exact match Int(0) |
| `0x07` | TjmpNeg | peek(-1) → jump | 2 (addr u16) | **PEEK not pop**; exact match Int(-1) |
| `0x08` | Tstore | val → reg | 1 (reg idx) | pops val, stores to register |
| `0x09` | Tload | → val | 1 (reg idx) | pushes register onto stack |
| `0x0a` | Tdup | a → a, a | — | duplicate top of stack |
| `0x0b` | Tjmp | → | 2 (addr u16) | unconditional jump |
| `0x0c` | Tpop | a → | — | discard top of stack |
| `0x0e` | Tcons | a, b → result | — | consensus(a, b): logical merge |
| `0x0f` | Talloc | size → TensorRef | 4 (size u32) | allocate tensor pool entry |
| `0x10` | Tcall | → | 2 (addr u16) | saves registers, pushes return addr |
| `0x11` | Tret | → | — | restores registers, pops return addr |
| `0x14` | Tless | a, b → trit | — | a < b → affirm, else reject (polymorphic) |
| `0x15` | Tgreater | a, b → trit | — | a > b → affirm, else reject (polymorphic) |
| `0x16` | Teq | a, b → trit | — | a == b → affirm, else reject (polymorphic) |
| `0x17` | TpushInt | → Int(n) | 8 (i64 little-endian) | push integer literal |
| `0x18` | TaddInt | a, b → Int | — | integer addition |
| `0x19` | TpushFloat | → Float(f) | 8 (f64 little-endian) | push float literal |
| `0x1e` | Tdiv | a, b → result | — | float or int division |
| `0x1f` | Tmod | a, b → result | — | modulo |
| `0x20` | Tprint | val → | — | prints + appends to print_log |
| `0x21` | TpushString | → String(s) | 4-len + bytes | push string literal |
| `0x22` | Tidx | TensorRef, idx → val | — | tensor element access |
| `0x23` | Tset | TensorRef, idx, val → | — | tensor element write |
| `0x24` | Tshape | TensorRef → (rows, cols) | — | tensor dimensions |
| `0x25` | TjmpEqInt | peek(n) → jump | 8 (i64) + 2 (addr u16) | PEEK; jump if top == imm_int |
| `0x30` | Tspawn | → AgentRef | 2 (type_id u16) | spawn registered agent type |
| `0x31` | Tsend | msg, AgentRef → | — | send message to agent |
| `0x32` | Tawait | AgentRef → result | — | receive result from agent |

**Missing opcodes in the table = not implemented.** If you need 0x12, 0x13, 0x1a–0x1d, etc., they do NOT exist. Do not write .tern code that would require them.

---

## 5. Error codes — BET VM

| Code | Enum | Trigger |
|------|------|---------|
| BET-001 | StackUnderflow | pop on empty stack |
| BET-002 | BetFault | 0b00 bit pattern in trit encoding |
| BET-003 | Halt | clean end of bytecode (not an error) |
| BET-004 | InvalidOpcode | unknown opcode byte |
| BET-005 | InvalidRegister | reg index > 26 |
| BET-006 | PcOutOfBounds | jump target outside bytecode |
| BET-007 | TypeMismatch | wrong Value variant for opcode |
| BET-008 | TensorIndexOutOfBounds | idx >= tensor.len() |
| BET-009 | TensorNotAllocated | TensorRef handle not in pool |
| BET-010 | AgentTypeNotRegistered | type_id not in agent_types map |
| BET-011 | AgentIdInvalid | agent index out of agents[] |
| BET-012 | RuntimeError | catch-all string message |

---

## 6. Canonical file template — copy this exactly

```tern
// stdlib/<category>/<name>.tern
// Demonstrates: <one sentence — what this file shows>
// Tier: <N> — <Open Core | Pro Standard | Industrial | Enterprise>

// Helper functions go here, defined BEFORE fn main()
fn helper(x: trit) -> trit {
    match x {
        -1 => { return conflict(); }
         0 => { return hold();    }
         1 => { return truth();   }
    }
}

fn main() -> trit {
    let result: trit = helper(1);

    match result {
        -1 => { return conflict(); }
         0 => { return hold();    }
         1 => { return truth();   }
    }
}
```

**Invariants:**
- All three match arms always present: `-1`, `0`, `1`
- Every arm body has at least one statement (no empty `{ }`)
- `fn main() -> trit` is the ONLY entry point
- All params typed: `fn f(x: trit, n: int)` — never `fn f(x)`
- Comments are `//` only — `/* */` causes silent 3-byte failure

---

## 7. Type syntax — exact forms

```tern
trit                  // scalar, values: -1 / 0 / 1
int                   // i64 internally
float                 // f64 internally
trit[]                // 1D trit array (variable length)
trittensor<3>         // 1D tensor of size 3  ← size REQUIRED
trittensor<4 x 4>     // 2D tensor 4×4        ← spaces around 'x' required
string                // string literal
```

**Invalid forms that will fail:**
- `trittensor` — bare, no size → parse error
- `trit[5]` — sized array literal → not supported, use `trit[]`
- `bool` — not a type in current compiler

---

## 8. Arithmetic and comparison operators

```tern
a + b     // addition (Tadd: polymorphic Trit/Int/Float)
a * b     // multiplication (Tmul: polymorphic)
-a        // negation (Tneg: polymorphic)
a / b     // division (Tdiv: Float or Int)
a % b     // modulo (Tmod)
a < b     // less-than → trit result (Tless)
a > b     // greater-than → trit result (Tgreater)
a == b    // equality → trit result (Teq)
```

Note: comparison operators return `trit` (affirm/reject), NOT `bool`.
Use them in `if` conditions: `if a > b { ... }` works because the VM treats affirm as branch-taken.

---

## 9. Test command — exact

```bash
cd "/home/eri-irfos/Desktop/Ternary Intelligence Stack (TIS)/ternlang-root"
./target/debug/ternlang-cli run stdlib/<category>/<name>.tern
```

**PASS signature:** output contains `Program exited successfully.`
**FAIL signature:** output contains `VM Error:` or `Program exited with error`

**Silent failure signature (bug, not pass):**
```
Parse stmt error: UnexpectedToken("Fn")
Emitted 3 bytes of bytecode
Program exited successfully.
```
→ 3 bytes = only the `Halt` opcode was emitted. Logic did NOT run. This is a `fn main()` entry point failure. Do not commit.

**Bytecode sanity check:**
- Minimal working file: > 10 bytes
- Typical single-function file: 100–200 bytes
- Complex multi-function file: 300–600 bytes
- If you see 3 bytes: your `fn main()` is missing or the file has a top-level parse error

---

## 10. Commit format — exact

```bash
cd "/home/eri-irfos/Desktop/Ternary Intelligence Stack (TIS)/ternlang-root"
git add stdlib/<category>/<filename>.tern
git commit -m "stdlib(<category>): add <concept> [tier<N>] — <one sentence"
git push origin main
```

If compiler fix included:
```bash
git add stdlib/<category>/<filename>.tern \
        compiler/legacy_shim/ternlang-core/src/<file>.rs \
        Buglist/Fixes.md
git commit -m "stdlib(<category>): add <concept> [tier<N>] — fix <what> in <file>"
git push origin main
```

**Git root is at the `TIS/` level**, not `ternlang-root/`. All `git` commands must run from `ternlang-root/` OR use `ternlang-root/` as path prefix.

---

## 11. Session-start checklist

Run ALL of these before writing any file:

```bash
# 1. Build must succeed
cd "/home/eri-irfos/Desktop/Ternary Intelligence Stack (TIS)/ternlang-root"
cargo build --bin ternlang-cli 2>&1 | tail -10

# 2. Read the Buglist first — do not re-investigate known issues
cat Buglist/Fixes.md

# 3. Check recent commits — do not duplicate
git log --oneline -20

# 4. Scan target directory — do not duplicate concepts
ls stdlib/<target_category>/

# 5. Confirm clean branch
git status --short
```

Do not write a single file until step 1 succeeds.

---

## 12. Compiler fix authorization table

| File | May add | May NOT touch |
|------|---------|---------------|
| `vm/mod.rs` | New opcode match arms | Any existing arm |
| `codegen/betbc.rs` | New emit_ cases | Any existing case |
| `parser.rs` | New syntax branches | Any existing branch |
| `ast.rs` | New AST node variants | Any existing variant |
| `semantic.rs` | New type-check cases | Any existing case |
| `ternlang-cli/src/main.rs` | Agent type_id registrations ONLY | Everything else |

**Line count rule:** your edit must not reduce the file by more than 10 lines. Run `wc -l <file>` before and after. If count drops > 10, you deleted features. Revert and re-approach.

---

## 13. Known-fail patterns — do not write these

```tern
/* this is a block comment */    // BANNED — UnexpectedToken("Slash")

let x: trittensor;               // BANNED — must specify size
let x: trittensor<4>;            // OK

fn f(x) -> trit { }             // BANNED — param needs type annotation

match v {
    -1 => { return conflict(); }
     1 => { return truth();   }
}                                // BANNED — missing 0 arm → compile error

0 => { }                         // BANNED — empty arm body → runtime issue
                                 // FIX: 0 => { let _h: trit = hold(); }

let result: trit = my_fn(x);
match result { ... }             // ONLY OK if inside fn main()
                                 // At top level, parser may fail on 'fn' keyword
```

---

## 14. Known-good patterns — use freely

```tern
// Gate pattern: one signal overrides all
fn hard_gate(critical: trit, rest: trit) -> trit {
    match critical {
        -1 => { return conflict(); }
         0 => { return hold();    }
         1 => { return rest;      }
    }
}

// Accumulator pattern: count affirm signals via helper
fn count_signal(s: trit, count: int) -> int {
    match s {
        -1 => { return -1; }         // veto flag
         0 => { return count; }      // neutral, pass through
         1 => { return count + 1; }  // increment
    }
}

// EMA pattern: float deliberation
fn ema(prior: float, signal: float, alpha: float) -> trit {
    let smoothed: float = alpha * signal + (1.0 - alpha) * prior;
    if smoothed > 0.75 { return truth();    }
    if smoothed < 0.25 { return conflict(); }
    return hold();
}

// @sparseskip (Tier 2+, patent-pending RFI-IRFOS)
@sparseskip
fn sparse_layer(w: trit[], x: trit[]) -> trit {
    return consensus(w[0], x[0]);
}
```
