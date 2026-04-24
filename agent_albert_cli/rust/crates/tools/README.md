# albert-tools

Tool execution layer for [Albert CLI](https://crates.io/crates/albert-cli) — part of the [Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack).

[![Crates.io](https://img.shields.io/crates/v/albert-tools)](https://crates.io/crates/albert-tools)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/LICENSE)

## What this crate provides

The tool execution layer Albert uses to interact with the local environment:

| Tool | What it does |
|------|-------------|
| `read_file` | Read file contents with line ranges |
| `write_file` | Write or overwrite files |
| `edit_file` | Targeted string replacement in files |
| `bash` | Execute shell commands in a sandboxed subprocess |
| `glob` | Pattern-based file discovery |
| `grep` | Regex search across the codebase |
| `web_fetch` | Fetch URLs and extract content |
| MCP dispatch | Forward tool calls to connected MCP servers |

All tools go through the `albert-runtime` permission layer before execution — dangerous operations are blocked by policy before they reach the OS.

## Part of the Albert ecosystem

| Crate | Role |
|-------|------|
| [`albert-runtime`](https://crates.io/crates/albert-runtime) | Session, MCP, auth, bash |
| [`albert-api`](https://crates.io/crates/albert-api) | Multi-provider LLM client |
| [`albert-commands`](https://crates.io/crates/albert-commands) | Slash command library |
| `albert-tools` | **This crate** — tool execution |
| [`albert-compat`](https://crates.io/crates/albert-compat) | Manifest extraction harness |
| [`albert-cli`](https://crates.io/crates/albert-cli) | Binary (`albert`) |
