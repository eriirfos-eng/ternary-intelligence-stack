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

| Command | What it does |
|---------|-------------|
| `/plan` | Decompose a task into a structured plan before coding |
| `/tdd` | Red-Green-Refactor loop — failing test first |
| `/verify` | Run tests, report failures only |
| `/code-review` | Full correctness + safety + idiom review |
| `/build-fix` | Compile → fix errors → repeat until clean |
| `/refactor` | Targeted cleanup, no behaviour change |
| `/docs` | Generate documentation for current scope |
| `/loop` | Recursive mission loop, up to 10 iterations |
| `/compress` | Aggressive context compaction |

## Workspace layout

```
crates/
  albert-runtime       — session, MCP, OAuth, bash, file ops, compaction
  albert-api           — multi-provider LLM client, SSE streaming, retry
  albert-commands      — slash command library
  albert-tools         — tool dispatch (read/write/edit/bash/glob/grep/MCP)
  albert-compat        — upstream manifest extraction and path resolution
  rusty-ternlang-cli/  — binary crate (package: albert-cli, binary: albert)
  rtk-integration/     — vendored RTK token filter (external, not published)
```

## Build

```bash
cargo build --workspace
cargo install --path crates/rusty-ternlang-cli
```

## Configuration

Albert looks for configuration in `~/.config/albert/` and project-local `.ternlang/`. An `ALBERT.md` file in your workspace root is automatically loaded as agent context at session start.

## TernStudio integration

Albert is designed to be summoned inside TernStudio via `F6` — generating workflows from plain-language prompts, debugging signal paths, and explaining node behaviour. This integration is currently in development as part of the TernStudio roadmap.

## License

MIT — see [LICENSE](LICENSE).
