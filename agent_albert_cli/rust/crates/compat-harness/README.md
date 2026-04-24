# albert-compat

Compatibility harness for [Albert CLI](https://crates.io/crates/albert-cli) — part of the [Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack).

[![Crates.io](https://img.shields.io/crates/v/albert-compat)](https://crates.io/crates/albert-compat)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/LICENSE)

## What this crate provides

Upstream manifest extraction and path resolution utilities that let Albert orient itself inside any project:

| Feature | What it does |
|---------|-------------|
| Manifest extraction | Locate and parse `ALBERT.md` / `.ternlang.json` config files |
| Path resolution | Resolve project root, workspace root, and config search paths |
| Compat layer | Bridge between Albert's runtime expectations and host project layout |

Used internally by `albert-cli` during session initialization to build the cognitive map of the current repo before any slash command runs.

## Part of the Albert ecosystem

| Crate | Role |
|-------|------|
| [`albert-runtime`](https://crates.io/crates/albert-runtime) | Session, MCP, auth, bash |
| [`albert-api`](https://crates.io/crates/albert-api) | Multi-provider LLM client |
| [`albert-commands`](https://crates.io/crates/albert-commands) | Slash command library |
| [`albert-tools`](https://crates.io/crates/albert-tools) | Tool execution layer |
| `albert-compat` | **This crate** — manifest extraction harness |
| [`albert-cli`](https://crates.io/crates/albert-cli) | Binary (`albert`) |
