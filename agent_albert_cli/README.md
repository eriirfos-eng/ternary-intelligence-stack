<div align="center">

# Albert CLI

**The sovereign AI development CLI for the Ternary Intelligence Stack**

[![Rust](https://img.shields.io/badge/built%20with-Rust-CE422B?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](../LICENSE)
[![Crates.io](https://img.shields.io/crates/v/albert-cli?color=orange)](https://crates.io/crates/albert-cli)
[![Part of TIS](https://img.shields.io/badge/part%20of-Ternary%20Intelligence%20Stack-6366f1)](https://github.com/eriirfos-eng/ternary-intelligence-stack)
[![Studio](https://img.shields.io/badge/studio-live-22c55e)](https://ternlang-api.fly.dev/studio)

Albert is a terminal-native, model-agnostic AI agent built in pure Rust. It runs a hardened agentic loop — research → strategy → execute — with every action validated through ternary logic before anything touches your system.

</div>

---
<img width="960" height="816" alt="image" src="https://github.com/user-attachments/assets/ea793697-949d-4c6f-98f3-3bcfa74aca03" />

### Quick Install — One Copy Gets It All

```bash
# Install Albert (brings the full agent engine with it)
cargo install albert-cli


# Launch
albert-cli
```

That's it. Albert spins up the REPL and the onboarding will start.


## How it thinks

Albert operates in exactly three states:

| Signal | Meaning | What happens |
|--------|---------|--------------|
| `+1` | Affirm | Safe to proceed — action is executed |
| ` 0` | Tend | Context missing — Albert pauses and asks before continuing |
| `-1` | Reject | Failure detected — Albert self-corrects and retries |

This isn't just a design philosophy. It's enforced at the engine level. Albert will not hallucinate an answer, silently skip a step, or execute an ambiguous command.

---

## Features

### Sovereign Security
- **Deny-first AST interception** — every shell command is parsed by a native Rust pipeline before it reaches the OS. Command substitution, dangerous pipes (`curl | bash`), and unauthorized redirects are blocked at the source.
- **Revoked flag access** — the LLM cannot touch sandbox flags or security policies at runtime. Zero prompt-injection surface for privilege escalation.
- **Zero-trust key storage** — API keys live in `~/.config/albert/credentials.json`, never in environment variables passed to subprocess.

### LLM-Agnostic Core
Albert routes to any provider you configure. No vendor lock-in, no hardcoded defaults.

| Provider | Auth |
|----------|------|
| Anthropic (Claude) | API key |
| OpenAI (GPT-4o, o1) | API key |
| Google (Gemini) | API key |
| XAI (Grok) | API key |
| HuggingFace | API key |
| Ollama | Local — no key needed |
| Azure OpenAI | API key + endpoint |
| AWS Bedrock | API key + region |

### Slash Command Library

| Command | Description |
|---------|-------------|
| `/auth [provider]` | Configure provider credentials |
| `/init` | Build a cognitive map of the current repository |
| `/plan <task>` | Deep multi-step execution plan with risk analysis |
| `/tdd <interface>` | Scaffold → failing test → implement loop |
| `/loop <mission>` | Autonomous agent loop until mission complete |
| `/code-review` | Quality, security, and maintainability audit |
| `/build-fix` | Detect build errors and dispatch resolver agents |
| `/bughunter` | Hunt and triage bugs across the codebase |
| `/refactor [scope]` | Remove dead code, consolidate duplicates |
| `/commit` | AI-generated commit message from staged diff |
| `/pr` | Draft and open a pull request |
| `/compress` | Sliding-window context compaction |
| `/status` | Current session state and active context |
| `/help` | Full command reference |

### Memory & Context
- **Session persistence** — conversations survive restarts, stored in `~/.config/albert/sessions/`
- **Knowledge graph** — entity/relation memory that tracks project-specific context across sessions
- **RepoMap** — automated structural navigation so Albert understands your codebase before it touches it
- **MCP client** — native Model Context Protocol support; plug in any third-party MCP server

### RTK — Rust Token Killer
Integrated token-optimization proxy that intercepts outgoing context and applies structural filters. Typical savings: **60–90% on development operations** like `git status`, `cargo clippy`, and file reads.

---

## Cognitive Persistence & Identity

Albert builds a mental model of your project through two primary systems:

###  The Knowledge Graph (`knowledge_graph.json`)
This is Albert's **Knowledge Base**. When you say "lock this into memory," Albert uses a specialized tool to store structured facts, relationships, and observations.
- **Persistence**: Stored in `~/.ternlang/memory/knowledge_graph.json`.
- **Neurosymbolic Gap Recovery**: If a tool enters an ambiguous state (**State 0**), Albert autonomously queries this graph. If he finds a matching entity or observation, he uses it to resolve the ambiguity and proceed to **State +1** without bothering you.
- **Content**: Tracks project team members, architectural decisions, and hardware context.

###  Identity & Working Agreement (`ALBERT.md`)
This is Albert's **Core Directive**. It is the instruction set that defines his "brain" configuration for the current workspace.
- **Customizable Behavior**: By editing `ALBERT.md` in your project root, you can "re-program" Albert's personality, tone, and priorities.
- **Directives**: You can set specific rules like "always use async/await" or "respond with dry wit."
- **Scope**: Loaded dynamically on startup; changes take effect in the next session.

###  Reflection Log (`memory.md`)
A global, high-level summary of significant insights. Albert periodically runs a "reflection pass" (`llm_reflect`) to distill your conversations into single-line memories saved in `~/.ternlang/memory.md`.

---

## Installation



**Prerequisites:** Rust toolchain (`cargo`) — [install here](https://rustup.rs/)

```bash
# Install Albert (brings the full agent engine with it)
cargo install albert-cli

# Set your LLM key (pick any provider)
export GEMINI_API_KEY=AIza...          # Google Gemini — default, highest quota
export ANTHROPIC_API_KEY=sk-ant-...   # Anthropic Claude
export OPENAI_API_KEY=sk-...          # OpenAI / GPT-4o
# export XAI_API_KEY=xai-...          # xAI / Grok
# (Ollama: just run `ollama serve` — no key needed)

# Launch
albert-cli
```

---

## Quickstart

```bash
# 1. Add your provider credentials
albert /auth

# 2. Map the current repository
albert /init

# 3. Start the REPL
albert-cli
```

On first launch Albert walks you through a setup sequence: cognitive style, provider routing, and model selection. After that, you drop straight into the REPL.

---

## Repository layout

```
agent_albert_cli/
├── rust/
│   └── crates/
│       ├── api/              # Multi-provider LLM client (albert-api)
│       ├── runtime/          # Core loop, OAuth, MCP, session management (albert-runtime)
│       ├── commands/         # Slash command library (albert-commands)
│       ├── tools/            # Tool execution layer — bash, file ops, search (albert-tools)
│       ├── compat-harness/   # Manifest extraction and path resolution (albert-compat)
│       └── rusty-ternlang-cli/  # Main binary: albert (albert-cli)
├── src/                      # Python/legacy reference source
├── tests/                    # Validation surfaces
├── ALBERT.md                 # Agent working agreement for this repo
└── PARITY.md                 # Feature parity gap analysis vs. TypeScript origin
```

---

## Ecosystem

Albert is one node in the **Ternary Intelligence Stack** — a full agentic platform built around ternary logic.

| Component | Description | Link |
|-----------|-------------|------|
| **TernStudio** | Visual flow IDE — drag-and-drop ternary agent graphs | [Live →](https://ternlang-api.fly.dev/studio) |
| **ternlang-api** | Rust/Axum backend — LLM routing, simulation, WebSocket tracer | [API →](https://ternlang-api.fly.dev) |
| **Albert CLI** | Terminal agent — this repo | — |

---

## Development

```bash
cd rust

# Check everything compiles
cargo build --workspace

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Test
cargo test --workspace
```

The workspace uses `resolver = "2"` with `unsafe_code = "forbid"` and pedantic Clippy enforced across all crates.

---

<div align="center">

*A sovereign project of [RFI-IRFOS](https://github.com/eriirfos-eng) — building the infrastructure for agentic intelligence.*

</div>
