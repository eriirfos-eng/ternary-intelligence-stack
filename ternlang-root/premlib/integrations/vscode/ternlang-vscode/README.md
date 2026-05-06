# Ternlang — VS Code Extension

**Ternlang** is the official VS Code extension for the [Ternary Intelligence Stack (TIS)](https://ternlang.com) — a balanced ternary systems programming language built on the BET VM.

## Features

- **Syntax highlighting** — full `.tern` grammar: keywords, trit literals (`affirm`, `hold`, `reject`), match arms, agent blocks, tensor types, and operators
- **Snippets** — 8 ready-to-use code snippets (`fn`, `match`, `while`, `for..in`, `agent`, `let`, `trittensor`, `struct`)
- **Run commands** — execute `.tern` files directly from the editor
- **Diagnostics** — `check-on-save` surfaces parse errors as inline squiggles
- **Language config** — bracket pair coloring, auto-close, comment toggle (`//` and `/* */`)

## Requirements

Install the Ternlang CLI:

```bash
cargo install ternlang-cli
```

The CLI binary (`ternlang`) must be on your `PATH`.

## Commands

| Command | Description |
|---------|-------------|
| `Ternlang: Run File` | Run the current `.tern` file in terminal |
| `Ternlang: Run File (Debug)` | Run with `--debug` flag (shows register state) |
| `Ternlang: Build to .tbc Bytecode` | Compile to bytecode without executing |
| `Ternlang: Check File (Validate)` | Parse + type-check without running |
| `Ternlang: Open REPL` | Launch the interactive ternlang REPL |

## Extension Settings

| Setting | Default | Description |
|---------|---------|-------------|
| `ternlang.executablePath` | `"ternlang"` | Path to the CLI binary |
| `ternlang.checkOnSave` | `true` | Run parse check on every save |

## Quick Example

```tern
fn classify(x: trit) -> trit {
  match x {
    -1 => { print("reject"); return reject; }
     0 => { print("hold");   return hold;   }
     1 => { print("affirm"); return affirm; }
  }
  return hold;
}

fn main() -> trit {
  let t: trit = classify(affirm);
  return t;
}
```

## What is Ternlang?

Ternlang uses **balanced ternary** (base-3 with values -1, 0, +1) instead of binary. This enables:

- Native three-state logic (`affirm` / `hold` / `reject`) without boolean hacks
- Trit tensors for ternary neural networks and MoE inference
- EU AI Act–compliant reasoning via the TIS MCP server
- Industrial decision pipelines with built-in uncertainty quantification

**[ternlang.com](https://ternlang.com)** · [GitHub](https://github.com/eriirfos-eng/ternary-intelligence-stack) · [MCP Server](https://smithery.ai/server/rfi-irfos/ternlang)

---

© RFI-IRFOS · License: LGPL-3.0-or-later · Patent A50296/2026
