# Ternlang — VS Code Extension

**Syntax highlighting and language intelligence for `.tern` files.**  
Part of the [RFI-IRFOS Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-).

---

## Features

- **Syntax highlighting** — keywords, trit literals (`-1`, `0`, `+1`), types (`trit`, `trittensor`, `agentref`), directives (`@sparseskip`), operators, strings, comments
- **19 code snippets** — `fn`, `let`, `match` (3-way exhaustive), `if-ternary`, `for-in`, `loop`, `agent`, `struct`, `@sparseskip`, all builtin functions
- **Hover documentation** — hover any keyword to see its ternary semantics
- **Live diagnostics** — syntax errors underlined as you type (via ternlang-lsp)
- **Auto-close pairs** — `{}`, `()`, `[]`, `""`, `''`
- **Bracket matching** and **comment toggling** (`//`)

---

## Language Overview

Ternlang is a systems language built around **balanced ternary logic** — where every value is one of:

| Value | Symbol | Meaning |
|---|---|---|
| `-1` | conflict | active disagreement |
| `0` | hold | active neutral — not null, not absent |
| `+1` | truth | confirmed |

```tern
fn decide(evidence: trit) -> trit {
    match evidence {
        -1 => conflict()
         0 => hold()
        +1 => truth()
    }
}

// Sparse inference — zero weights skipped at VM level
@sparseskip let result: trittensor<8 x 8> = matmul(input, weights);

// Actor model
agent Voter {
    fn handle(msg: trit) -> trit {
        consensus(msg, hold())
    }
}
let v: agentref = spawn Voter;
send v truth();
let decision: trit = await v;
```

---

## LSP Setup

The extension connects to `ternlang-lsp` for hover and diagnostics.

1. Build the language server:
   ```bash
   cd ternlang-root && cargo build --release
   ```
2. The extension will auto-start `ternlang-lsp` when you open any `.tern` file.

---

## The Full Stack

This extension is the editor front-end for a complete ternary computing platform:

- **BET VM** — stack-based balanced ternary VM, 27 registers, sparse matmul opcode
- **ML kernels** — BitNet-style ternary quantization, 2.3× sparse inference speedup
- **FPGA backend** — Verilog-2001 codegen, full BET processor synthesisable for any FPGA
- **Distributed actors** — `spawn remote "addr" AgentName` for multi-node ternary systems
- **MCP integration** — connect any AI agent to ternary decision logic
- **Package manager** — `ternpkg init`, `ternpkg install owner/repo@tag`

[GitHub](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-) · [ISA Spec](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/blob/main/ternlang-root/BET-ISA-SPEC.md) · [Ecosystem Map](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/blob/main/ternlang-root/TERNARY-ECOSYSTEM.md)

---

**RFI-IRFOS · LGPL-3.0**
