# GEMINI.md — Ternlang Precision Parameter Sheet
# RFI-IRFOS · ternlang.com · v1.5 (2026-04-16)
# Read this at session start. These values are ground truth. Do not guess.
# Current workspace version: 0.3.1 (Professionalized metadata & XAI branding)

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
| Open Files | unbounded Vec | File handles for opent/readt/writet |

---

## 2. ML & Transmutation — Phase 12 Facts

| Feature | Logic | Note |
|---------|-------|------|
| **Ternarization** | BitNet b1.58 (tau = 0.5 * mean(\|W\|)) | Maps f32 weights to {-1, 0, +1} |
| **@sparseskip** | Native 0-state (tend) bypass | 2.3x baseline, up to 122x at 99% sparsity |
| **Sparsity (Llama 3.2 1B)** | **30.63% (mean)** | Measured on 147 layers (2026-04-14) |
| **Coherence (Llama 3.2 1B)** | **97.06% signal ratio** | Verified via Phase 12A Rust Forward Pass POC |
| **Binary Model (1.2B)** | **240MB packed .bin** | 1.2GB JSON reduced by 5x via ModelCoherence |
| **TritTransformer** | **Llama-3 Architecture** | Verified RMSNorm, RoPE, SwiGLU in `ternlang-ml` |
| **Encoding** | 2-bit packed (00=0, 01=+1, 10=-1) | 4 trits per byte (1.25x denser than naive) |

---

## 3. Repository & Compliance — Authority Pillar

| Pillar | Keywords | Mandate |
|--------|----------|---------|
| **XAI** | Explainable AI, Deterministic, Traceable | No hallucinated confidence. Trit-0 is a hold. |
| **HPC** | Sparsity-Aware, Low-Latency, High-Performance | @sparseskip is the hardware differentiator. |
| **Safety** | EU AI Act (Art 13, 14, 15), GDPR | Veto at conf > 0.90 is a hard gate. |
| **Leadership** | Simeon Kepp, Nikoletta Csonka, Zabih Karimi | All crates attributed to RFI-IRFOS core team. |

---

## 4. Value types — exact Rust enum

```rust
Value::Trit(Trit)          → trit scalar
Value::Int(i64)            → int
Value::Float(f64)          → float
Value::String(String)      → string
Value::TensorRef(usize)    → handle into tensors[]
Value::AgentRef(usize, Option<String>)  → spawned agent
```

Default value (unset register): `Value::Trit(Trit::Tend)` = 0

---

## 5. Trit values — both identifier forms (both compile, pick one and be consistent)

| Numeric | Rust enum | Built-in fn | Bare identifier | Meaning |
|---------|-----------|-------------|-----------------|---------|
| `1`  | `Trit::Affirm` | `truth()`    | `affirm` | positive signal, proceed |
| `0`  | `Trit::Tend`   | `hold()`     | `tend`   | insufficient evidence, wait |
| `-1` | `Trit::Reject` | `conflict()` | `reject` | negative signal, block |

---

## 6. Complete opcode table — BET VM

All opcodes as of 2026-04-16. Do not add opcodes to .tern files that aren't in this table.

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
| `0x0f` | Talloc | size → TensorRef | 4 (rows u16, cols u16) | allocate tensor pool entry |
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
| `0x26` | TlessEqual | a, b → trit | — | a <= b → affirm, else reject (polymorphic) |
| `0x27` | TgreaterEqual | a, b → trit | — | a >= b → affirm, else reject (polymorphic) |
| `0x28` | Tand | a, b → trit | — | min(a,b) in balanced ternary — logical AND |
| `0x29` | Tor | a, b → trit | — | max(a,b) in balanced ternary — logical OR |
| `0x2a` | Topent | path, mode → handle | — | mode: 0=Read, 1=Write, 2=Append |
| `0x2b` | Treadt | handle → trit | — | read single trit character (+, 0, -) |
| `0x2c` | Twritet | handle, trit → | — | write single trit character (+, 0, -) |
| `0x30` | Tspawn | → AgentRef | 2 (type_id u16) | spawn registered agent type |
| `0x31` | Tsend | msg, AgentRef → | — | send message to agent |
| `0x32` | Tawait | AgentRef → result | — | receive result from agent |

---

## 7. Canonical file template — copy this exactly

```tern
// stdlib/<category>/<name>.tern
// Demonstrates: <one sentence — what this file shows>
// Tier: <N> — <Open Core | Pro Standard | Industrial | Enterprise>

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
- All params typed: `fn f(x: trit, n: int)`
- Comments are `//` only

---

## 8. Development Protocol

1. **Styrian Loop**: `cargo build` → `cargo test --workspace` → `git add` → `semantic commit`.
2. **Tri-State Review**: Affirm (+1) = Merged, Hold (0) = Discuss, Reject (-1) = No fluff.
3. **SEO Mandate**: Use keywords: `XAI`, `Sparsity-Aware`, `Deterministic`, `Quantization`.
4. **License**: LGPL (Core) / BSL-1.1 (ML/MoE/API).
