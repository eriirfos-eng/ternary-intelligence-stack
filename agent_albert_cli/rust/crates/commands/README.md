# albert-commands

Slash command library for [Albert CLI](https://crates.io/crates/albert-cli) — part of the [Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack).

[![Crates.io](https://img.shields.io/crates/v/albert-commands)](https://crates.io/crates/albert-commands)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/LICENSE)

## What this crate provides

Parser and registry for all Albert slash commands. Each command is a typed enum variant with spec metadata (description, arguments, resume support).

| Command | Description |
|---------|-------------|
| `/auth` | Configure provider credentials |
| `/init` | Build cognitive map of the current repo |
| `/plan` | Multi-step execution plan with risk analysis |
| `/tdd` | Scaffold → failing test → implement loop |
| `/loop` | Autonomous agent loop until mission complete |
| `/code-review` | Quality, security, and maintainability audit |
| `/build-fix` | Detect and repair build errors |
| `/bughunter` | Hunt and triage bugs across the codebase |
| `/refactor` | Remove dead code, consolidate duplicates |
| `/commit` | AI-generated commit message from staged diff |
| `/compress` | Sliding-window context compaction |
| `/help` | Full command reference |

## Part of the Albert ecosystem

| Crate | Role |
|-------|------|
| [`albert-runtime`](https://crates.io/crates/albert-runtime) | Session, MCP, auth, bash |
| [`albert-api`](https://crates.io/crates/albert-api) | Multi-provider LLM client |
| `albert-commands` | **This crate** — slash command library |
| [`albert-tools`](https://crates.io/crates/albert-tools) | Tool execution layer |
| [`albert-compat`](https://crates.io/crates/albert-compat) | Manifest extraction harness |
| [`albert-cli`](https://crates.io/crates/albert-cli) | Binary (`albert`) |
