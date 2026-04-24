# albert-api

Multi-provider LLM client for [Albert CLI](https://crates.io/crates/albert-cli) — part of the [Ternary Intelligence Stack](https://github.com/eriirfos-eng/ternary-intelligence-stack).

[![Crates.io](https://img.shields.io/crates/v/albert-api)](https://crates.io/crates/albert-api)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/LICENSE)

## What this crate provides

A unified async LLM client that routes to any provider without vendor lock-in:

| Provider | Protocol |
|----------|---------|
| Anthropic (Claude) | Messages API |
| OpenAI (GPT-4o, o1) | Chat Completions |
| Google (Gemini) | GenerativeLanguage API |
| XAI (Grok) | OpenAI-compatible |
| HuggingFace | Inference API |
| Ollama | Local OpenAI-compatible |
| Azure OpenAI | Chat Completions |
| AWS Bedrock | Messages API |

Also handles **OAuth token exchange**, **model discovery** (`list_remote_models`), and **streaming** via SSE.

## Part of the Albert ecosystem

| Crate | Role |
|-------|------|
| [`albert-runtime`](https://crates.io/crates/albert-runtime) | Session, MCP, auth, bash |
| `albert-api` | **This crate** — LLM client |
| [`albert-commands`](https://crates.io/crates/albert-commands) | Slash command library |
| [`albert-tools`](https://crates.io/crates/albert-tools) | Tool execution layer |
| [`albert-compat`](https://crates.io/crates/albert-compat) | Manifest extraction harness |
| [`albert-cli`](https://crates.io/crates/albert-cli) | Binary (`albert`) |
