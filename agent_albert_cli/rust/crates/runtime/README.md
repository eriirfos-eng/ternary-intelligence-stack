# albert-runtime

Core conversation runtime for [Albert CLI](https://crates.io/crates/albert-cli) — part of the [Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack).

[![Crates.io](https://img.shields.io/crates/v/albert-runtime)](https://crates.io/crates/albert-runtime)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/LICENSE)

## What this crate provides

- **Session management** — persistent conversation history, context compaction, sliding-window memory
- **MCP client** — stdio and network transport for the Model Context Protocol
- **OAuth / credentials** — PKCE flow, token storage in `~/.config/albert/`
- **Bash execution** — sandboxed shell with deny-first AST interception
- **Permissions** — runtime policy enforcement; blocks dangerous command patterns before they reach the OS
- **Config loading** — resolves `ALBERT.md`, `.ternlang.json`, and provider credentials in priority order

## Part of the Albert ecosystem

| Crate | Role |
|-------|------|
| `albert-runtime` | **This crate** — session, MCP, auth, bash |
| [`albert-api`](https://crates.io/crates/albert-api) | Multi-provider LLM client |
| [`albert-commands`](https://crates.io/crates/albert-commands) | Slash command library |
| [`albert-tools`](https://crates.io/crates/albert-tools) | Tool execution layer |
| [`albert-compat`](https://crates.io/crates/albert-compat) | Manifest extraction harness |
| [`albert-cli`](https://crates.io/crates/albert-cli) | Binary (`albert`) |
