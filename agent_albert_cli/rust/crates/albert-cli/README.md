# albert-cli

The `albert-cli` binary — part of the [Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack).

[![Crates.io](https://img.shields.io/crates/v/albert-cli)](https://crates.io/crates/albert-cli)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/LICENSE)

## Install

```bash
cargo install albert-cli
```

Then run:

```bash
albert-cli
```

## What Albert does

Albert is a sovereign AI development CLI that runs in your terminal. It connects to any LLM provider and gives you a full agentic coding environment without any cloud dependency:

| Feature | Details |
|---------|---------|
| Multi-provider | Claude, GPT-4o, Gemini, Grok, Ollama, Bedrock, Azure |
| Slash commands | `/plan`, `/tdd`, `/loop`, `/code-review`, `/build-fix`, `/bughunter`, `/refactor`, `/commit` |
| Tool execution | `read_file`, `write_file`, `edit_file`, `bash`, `glob`, `grep`, `web_fetch` |
| MCP support | stdio and network transport for any MCP server |
| Permission layer | Deny-first AST interception blocks dangerous shell patterns before OS |
| Session memory | Sliding-window context compaction keeps long sessions coherent |

## Part of the Albert ecosystem

| Crate | Role |
|-------|------|
| [`albert-runtime`](https://crates.io/crates/albert-runtime) | Session, MCP, auth, bash |
| [`albert-api`](https://crates.io/crates/albert-api) | Multi-provider LLM client |
| [`albert-commands`](https://crates.io/crates/albert-commands) | Slash command library |
| [`albert-tools`](https://crates.io/crates/albert-tools) | Tool execution layer |
| [`albert-compat`](https://crates.io/crates/albert-compat) | Manifest extraction harness |
| `albert-cli` | **This crate** — binary (`albert-cli`) |
