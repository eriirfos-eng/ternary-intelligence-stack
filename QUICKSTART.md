# QUICKSTART

Get started with the Ternary Intelligence Stack (TIS): ternary language, ternary compiler, ternary inference runtime, and albert. — a self-evolving ternary MoE language model trained from scratch.

---

## Prerequisites

- Rust stable (`rustup toolchain install stable`)
- Python 3.11+
- Modal account (`pip install modal && modal setup`) — for GPU training only

---

## 1. Build the stack

```bash
cd ternlang-root
cargo build --release
```

This builds all core crates: compiler, VM, inference runtime, albert-test TUI.

---

## 2. Run albert. inference (local CPU)

```bash
cd albert-moe-13
./albert-test
```

Opens the interactive TUI. Type a prompt and press Enter. albert. runs ternary inference on CPU at 83–125 tok/s via `@sparseskip` sparse routing.

**Benchmarks:**
```bash
./albert-test --bench
```

Runs 15 standard prompts and reports tokens/second. Output lands in `albert-moe-13/benchmarks/`.

---

## 3. Run the Ternlang compiler

Compile a `.tern` program to BET bytecode and run it:

```bash
cd ternlang-root
cargo run --release --bin ternlang -- run examples/hello.tern
```

Or use the installed binary:

```bash
ternlang run <file.tern>
```

---

## 4. GPU training (Modal)

albert. is trained on Modal T4 GPUs via the `albert-train` command:

```bash
cd albert-moe-13
./albert-train
```

This uploads the latest config to the Modal volume and starts a training epoch. The dashboard streams live loss. Training costs ~$0.021/epoch.

Monitor training:
```bash
./albert-test --dashboard
```

---

## 5. MCP server

The ternlang MCP server exposes 30 tools to any Claude/AI agent:

```bash
cd ternlang-root/ternlang-mcp
npm install && node index.js
```

Or connect via Smithery: `smithery.ai/server/rfi-irfos/ternlang`

---

## Key directories

| Path | Contents |
|------|----------|
| `ternlang-root/` | Compiler, VM, stdlib, all core crates |
| `albert-moe-13/` | albert. model, training pipeline, benchmarks |
| `agent_albert_cli/` | albert-cli: interactive AI assistant using albert. as backend |
| `ternlang-vscode/` | VS Code extension for `.tern` syntax highlighting |
| `docs/` | Architecture docs, compliance, reports |

---

## Further reading

- [albert. architecture](albert-moe-13/docs/architecture.md)
- [Training guide](ternlang-root/TRAINING.md)
- [Benchmarks](ternlang-root/BENCHMARKS.md)
- [SPORE protocol](albert-moe-13/docs/SPORE_PROTOCOL.md)
- [Language reference](ternlang-root/spec/ternlang-language-reference-v0.1.md)
