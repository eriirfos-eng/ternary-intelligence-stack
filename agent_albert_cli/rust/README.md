# Albert — AI Intelligence Layer for the Ternary Intelligence Stack

[![crates.io](https://img.shields.io/crates/v/albert-cli.svg)](https://crates.io/crates/albert-cli)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

Albert is a sovereign, model-agnostic AI coding CLI and the embedded intelligence layer of the [Ternary Intelligence Stack](https://ternlang.com). He runs as a standalone terminal agent or wired directly into TernStudio to generate, debug, and explain ternary workflows.

## Install

```bash
cargo install albert-cli    # installs the `albert` binary
albert                      # interactive REPL
albert "your prompt here"   # one-shot mode
```

## Model-agnostic — bring your own LLM

```bash
# Google Gemini (default: gemini-2.0-flash)
export GEMINI_API_KEY=AIza...

# Anthropic Claude
export ANTHROPIC_API_KEY=sk-ant-...

# OpenAI / GPT
export OPENAI_API_KEY=sk-...

# XAI / Grok
export XAI_API_KEY=xai-...

# Ollama (fully local, no key needed)
ollama serve
```

Keys are stored in `~/.config/albert/secrets.json` — never sent anywhere except directly to your chosen provider.

## Slash commands

**Development & Reasoning** — `/plan`, `/tdd`, `/loop`, `/code-review`, `/build-fix`, `/bughunter`, `/refactor`, `/commit`

**Memory & Knowledge** — `/remember`, `/recall`, `/vault` (persistent cross-session memory), `/soul`, `/patterns`, `/security`, `/best-practices` (embedded production reference docs)

**Autonomous & Extensions** — `/cron` (schedule tasks), `/skill` (manage custom automations), `/teach-skill` (teach new behaviors)

**Session Utilities** — `/compress` (context compaction), `/help`, `/status`, `/export`

## Workspace layout

```
crates/
  albert-cli           — TUI and REPL binary
  albert-runtime       — session, MCP, OAuth, bash, file ops, compaction
  albert-api           — multi-provider LLM client, SSE streaming, retry
  albert-commands      — slash command library + spec registry
  albert-tools         — tool dispatch (read/write/edit/bash/glob/grep/MCP)
  albert-compat        — upstream manifest extraction and path resolution
  reference/           — embedded production documentation (SOUL, patterns, security, etc.)
  rtk-integration/     — vendored RTK token filter (external, not published)
```

## Build & Install

```bash
cargo build --workspace --release
cargo install --path crates/albert-cli
```

## Configuration

Albert looks for configuration in `~/.config/albert/` and project-local `.ternlang/`. An `ALBERT.md` file in your workspace root is automatically loaded as agent context at session start.

## TernStudio integration

Albert is designed to be summoned inside TernStudio via `F6` — generating workflows from plain-language prompts, debugging signal paths, and explaining node behaviour. This integration is currently in development as part of the TernStudio roadmap.

## License

MIT — see [LICENSE](LICENSE).
