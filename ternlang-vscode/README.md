# Ternlang — VS Code Extension

**Syntax highlighting, snippets, and a full 4-tier feature stack for `.tern` files.**  
Part of the [RFI-IRFOS Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-).

---

## Tiers

| Tier | Features | Price |
|------|----------|-------|
| **1 — Free** | Syntax highlighting · 19 snippets · Run command (`Ctrl+Shift+R`) · Graceful LSP · Status bar | Free forever |
| **2 — Pro** | + Inline trit value hints · Cloud diagnostics (ternlang.com LSP) | €99/mo |
| **3 — Industrial** | + BET VM step debugger · Tensor visualizer · `@sparseskip` coverage overlay | €349/mo |
| **4 — Enterprise** | + Remote cluster panel · Multi-node agent monitor | €2,500+/yr |

Set `ternlang.apiKey` in settings to unlock Tier 2–4.  
Get a key at [ternlang.com](https://ternlang.com/#pricing).

---

## Tier 1 Features (Free)

### Syntax Highlighting

Keywords, trit literals (`-1 / 0 / +1` and `reject / tend / affirm`), types (`trit`, `trittensor`, `agentref`), directives (`@sparseskip`), operators, strings, comments.

### 19 Code Snippets

| Prefix | What it inserts |
|--------|----------------|
| `fn` | Function declaration |
| `fnmain` | `fn main() -> trit` entry point |
| `let` / `letmut` | Immutable / mutable binding |
| `match` | 3-arm exhaustive match (`-1 / 0 / 1`) |
| `if` | If-else block |
| `for` | For-in loop over tensor rows |
| `while` / `loop` | While / infinite loop |
| `struct` | Struct definition |
| `agent` | Actor-model agent with `handle` method |
| `spawn` | Spawn agent + agentref binding |
| `sendawait` | Send message + await response |
| `sparseskip` | `@sparseskip` matmul line |
| `tensor` | Trittensor declaration |
| `cast` | Type cast |
| `consensus` / `invert` | Builtin call |
| `use` | Standard library import |

### Run .tern File

Press `Ctrl+Shift+R` (or `⌘⇧R` on macOS) — or click the **▶** button in the editor title bar — to run the current file through `ternlang-cli`. Output appears in the **Ternlang** output panel.

Configure the binary path if needed:

```json
"ternlang.cliPath": "/path/to/ternlang-cli"
```

### Language Intelligence (LSP)

Hover documentation and live diagnostics when `ternlang-lsp` is available locally:

```bash
cd ternlang-root && cargo build --release
```

If the binary is missing, the extension runs without it — no crash, one-time dismissible warning.

---

## Language Overview

Ternlang is a systems language built around **balanced ternary logic** — every value is one of:

| Value | Keyword | Meaning |
|-------|---------|---------|
| `-1` | `reject` | active disagreement |
| `0` | `tend` | active neutral — not null, not absent |
| `+1` | `affirm` | confirmed |

```tern
fn decide(evidence: trit) -> trit {
    match evidence {
        -1 => { return reject; }
         0 => { return tend; }
         1 => { return affirm; }
    }
}

// Sparse inference — zero weights skipped at VM level
@sparseskip let result: trittensor<8 x 8> = matmul(input, weights);

// Actor model — distributed ternary agents
agent Voter {
    fn handle(msg: trit) -> trit {
        return consensus(msg, tend);
    }
}
let v: agentref = spawn Voter;
send v affirm;
let decision: trit = await v;
```

---

## The Full Stack

This extension is the editor front-end for a complete ternary computing platform:

| Component | Description |
|-----------|-------------|
| **BET VM** | Stack-based balanced ternary VM, 27 registers, 50 opcodes |
| **ternlang-ml** | BitNet-style ternary quantization, 2.3×–122× sparse matmul |
| **ternlang-moe** | MoE-13 orchestrator — 13-expert ternary deliberation engine |
| **ternlang-lsp** | Full LSP 3.17: hover, completion, live diagnostics |
| **ternlang-api** | REST + MCP transport live at ternlang.com |
| **ternlang-hdl** | Verilog-2001 BET processor — FPGA synthesisable |
| **ternlang-runtime** | Distributed actors via TCP (`spawn remote "addr"`) |
| **ternpkg** | Package manager — `ternpkg init`, `ternpkg install owner/repo@tag` |

[GitHub](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-) · [ISA Spec](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/blob/main/ternlang-root/BET-ISA-SPEC.md) · [ternlang.com](https://ternlang.com)

---

**RFI-IRFOS · LGPL-3.0 (Tier 1) · BSL-1.1 (Tier 2–3) · Proprietary (Tier 4)**
