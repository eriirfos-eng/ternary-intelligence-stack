# albert. — AI Intelligence Layer for the Ternary Intelligence Stack

[![crates.io](https://img.shields.io/crates/v/albert-cli.svg)](https://crates.io/crates/albert-cli)
[![LGPL-2.1](https://img.shields.io/badge/license-LGPL--2.1-blue)](LICENSE)

albert. is a sovereign, model-agnostic AI coding CLI and the embedded intelligence layer of the [Ternary Intelligence Stack](https://ternlang.com). Runs as a standalone terminal agent or wired directly into TernStudio to generate, debug, and explain ternary workflows.

## Install

```bash
# One line — installs Rust (if needed) + albert-cli, ready immediately
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source "$HOME/.cargo/env" && cargo install albert-cli
albert                      # interactive TUI
albert "your prompt here"   # one-shot mode
```
> **Note:** Do not use `sudo apt install cargo` — Ubuntu's packaged Rust is too old (1.75). The line above installs the current toolchain via rustup.

## Model-agnostic — bring your own LLM

```bash
albert /auth anthropic       # Claude (all models)
albert /auth openai          # OpenAI / GPT
albert /auth google          # Gemini
albert /auth xai             # Grok
albert /auth nvidia          # NVIDIA NIM (80+ models, OpenAI-compat)
albert /auth openrouter      # OpenRouter (300+ models)
                             # Ollama: fully local, no key needed
```

Switch models at any time inside a session with `/model`. Keys are stored in `~/.ternlang/` — never sent anywhere except directly to your chosen provider.

## TUI highlights

- **Thinking typewriter** — reasoning tokens stream line-by-line with a `│` spine for any model that exposes extended thinking (Sonnet, R1, etc.)
- **Live Plan block** — `/plan` decomposes the task and TodoWrite calls animate pending/running/done states directly in the TUI
- **Multi-provider streaming** — all OpenAI-compatible providers (NVIDIA NIM, OpenRouter, XAI…) stream via a buffered path that avoids SSE format mismatches
- **Session report card** — `Ctrl-C` shows tokens used, duration, and every model switched during the session
- **Workspace trust** — trust decisions persist to `~/.albert/trusted_dirs.json`; no prompt on re-entry

## Slash commands

**Development & Reasoning** — `/plan`, `/tdd`, `/loop`, `/code-review`, `/build-fix`, `/bughunter`, `/refactor`, `/commit`

**Memory & Knowledge** — `/remember`, `/recall`, `/vault` (persistent cross-session memory), `/soul`, `/patterns`, `/security`, `/best-practices`

**Autonomous & Extensions** — `/cron` (schedule tasks), `/skill` (manage automations), `/teach-skill`

**Session Utilities** — `/auth`, `/model`, `/compress`, `/help`, `/status`, `/export`

## Workspace layout

```
crates/
  albert-cli           — TUI binary + agent loop
  albert-runtime       — session, MCP, OAuth, bash, file ops, compaction
  albert-api           — multi-provider LLM client, SSE/buffered streaming
  albert-commands      — slash command library + spec registry
  albert-tools         — tool dispatch (read/write/edit/bash/glob/grep/MCP)
  albert-compat        — upstream manifest extraction and path resolution
  moe-reference        — embedded production documentation (SOUL, patterns, security)
  moe-llb              — MCP server bridge for albert. tool exposure
  rtk-integration/     — vendored RTK token filter (external, not published)
```

## Build from source

```bash
cargo build --workspace --release
cargo install --path crates/albert-cli
```

## Configuration

albert. looks for configuration in `~/.ternlang/` and project-local `.ternlang/`. An `ALBERT.md` file in your workspace root is automatically loaded as agent context at session start.

## TernStudio integration

albert. is designed to be summoned inside TernStudio via `F6` — generating workflows from plain-language prompts, debugging signal paths, and explaining node behaviour. In active development as part of the TernStudio roadmap.

## License

LGPL-2.1 — see [LICENSE](LICENSE).
