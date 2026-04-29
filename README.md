# Ternary Intelligence Stack (TIS) 

[![version](https://img.shields.io/badge/version-v1.1.1-black)](#architecture)
[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-88%2B%20passing-brightgreen)](#architecture)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![EU AI Act](https://img.shields.io/badge/EU%20AI%20Act-Article%2013,14+15%20Compliant%20-003399?logo=european-union)](https://ternlang.com/compliance)
[![speedup](https://img.shields.io/badge/@sparseskip-up_to_122x-success)](#sparse-ternary-inference)
[![MCP](https://img.shields.io/badge/MCP-30_tools_free-orange)](#live-api)
[![smithery badge](https://smithery.ai/badge/rfi-irfos/ternlang)](https://smithery.ai/servers/rfi-irfos/ternlang)
[![examples](https://img.shields.io/badge/examples-2,090%2B_.tern_programs-blueviolet)](#example-library)
[![stdlib](https://img.shields.io/badge/stdlib-293_open%20%2B%2028k%2B_premium-blue)](ternlang-root/stdlib/PREMIUM.md)
[![DOI](https://img.shields.io/badge/DOI-10.17605%2FOSF.IO%2FTZ7DC-informational)](https://doi.org/10.17605/OSF.IO/TZ7DC)

Ternlang is a systems programming language, compiler, and high-performance inference runtime built on balanced ternary logic. We provide a fundamental architectural shift for **Explainable AI (XAI)** and European technological sovereignty.

Built by [RFI-IRFOS](https://ternlang.com) · Graz, Austria · Whitepaper [https://osf.io/cyn28/files/8hzux]

The core type is `trit`: three values — `−1` (reject), `0` (hold), `+1` (affirm), the zero state therefore is a first-class routing instruction: *"insufficient confidence — do not act yet."*

Ternlang provides a machine-readable path to human escalation instead of a forced binary guess.
---
## Technical Pillars

- **Deterministic Uncertainty**: Ternlang's `trit` (affirm/tend/reject) provides a first-class routing mechanism for **Uncertainty-Aware AI**, eliminating "hallucinated confidence."
- **Sparsity-Aware Inference Engine**: Native `@sparseskip` optimization achieves up to 122x throughput gains by bypassing zero-signal (`tend`) weights at the hardware primitive level.
- **Explainable AI (XAI) by Design**: Every decision is auditable and traceable, fulfilling **EU AI Act Articles 13, 14, and 15** mandates for algorithmic transparency and human oversight.
- **Post-Binary Systems Architecture**: A full-stack ecosystem including a custom **Instruction Set Architecture (ISA)**, triadic networking, and memory-efficient ternary encoding.

---
## Full Documentation
  
→ **[ternlang-root/README.md](ternlang-root/README.md)** (Full explanation, technical details, and compiler specs)


→ **[ROADMAP.md](ternlang-root/ROADMAP.md)** (Phases 1–18, session log, priority matrix)


→ **[Ternlang Studio Preview](https://ternlang-api.fly.dev/studio)** — Our work-in-progress SDK


→ **[Agent Albert](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/agent_albert_cli
)** —  terminal-native, model-agnostic AI agent built in pure Rust

→ **[Read our BIO and Mission in LEADERSHIP.md](LEADERSHIP.md)**

---

## Performance Benchmarks

| Feature | Performance Gain | Industry Comparison |
|---------|------------------|---------------------|
| **Ternary Inference** | 2.3x (baseline) | Up to 122x at 99%+ Sparsity |
| **Data Density** | 1.25x improvement | 5-trit block packing (8-bit) |
| **Logic Consistency** | 100% Deterministic | Eliminates binary timeout/null-guessing |
| **Safety Latency** | < 1ms hard-veto | Axis-6 Veto Hard Gate |

---

## Quick start

```bash
cargo install ternlang-cli
```

That's it. The `ternlang` binary is now in your PATH:

```bash
ternlang                        # → interactive REPL, start typing trit expressions
ternlang my_program.tern        # → run a .tern file directly
ternlang run my_program.tern    # → same (explicit form)
ternlang build my_program.tern  # → compile to .bet bytecode
ternlang fmt my_program.tern    # → format source file
ternlang repl                   # → interactive REPL (explicit)
ternlang test                   # → run test suite
```

Or build from source:

```bash
git clone https://github.com/eriirfos-eng/ternary-intelligence-stack
cd ternary-intelligence-stack/ternlang-root
cargo build --release
./target/release/ternlang examples/03_rocket_launch.tern
```

---

## Agent Albert — AI Intelligence Layer

[![crates.io](https://img.shields.io/crates/v/albert-cli.svg)](https://crates.io/crates/albert-cli)
[![version](https://img.shields.io/badge/version-v0.1.2-cyan)](https://crates.io/crates/albert-cli)

**Albert** is the sovereign, model-agnostic AI coding CLI and embedded intelligence layer of the Ternary Intelligence Stack. He runs entirely in your terminal, connects to any LLM provider, and never phones home.
---

### Quick Install — One Copy Gets It All

```bash
# Install Albert (brings the full agent engine with it)
cargo install albert-cli

# Launch
albert-cli
```

That's it. Albert spins up the REPL.

---

### What Albert Can Do

| Capability | Details |
|---|---|
| **Multi-provider routing** | Claude, GPT-4o, Gemini, Grok, Ollama, Bedrock, Azure — swap with `/model` |
| **Autonomous agent loop** | `/loop <mission>` — runs up to 10 tool-use turns to complete a goal; Ctrl+C aborts cleanly |
| **Chain execution** | `/plan <task>` — LLM decomposes goal into steps, executes each in sequence |
| **Tool harness** | `read_file`, `write_file`, `edit_file`, `bash`, `glob_search`, `grep_search`, `web_fetch` |
| **Self-reflection memory** | Automatically scores each turn for importance; commits key facts to `~/.ternlang/memory.md` and injects them on startup |
| **Image input** | Attach images with `[image: /path/to/file.png]` syntax (Gemini multimodal) |
| **Slash command library** | `/plan`, `/loop`, `/tdd`, `/bughunter`, `/code-review`, `/build-fix`, `/refactor`, `/commit`, `/pr`, `/compress`, and more — type `/` to browse |
| **Interactive model picker** | `/model` with no args shows a full provider-grouped list with descriptions |
| **Rate-limit resilience** | 429 errors auto-fall-back to a faster model and retry without crashing |
| **Permission layer** | `read-only`, `workspace-write`, `danger-full-access` modes — deny-first AST interception |
| **Session memory** | Sliding-window compaction keeps long sessions coherent |
| **RTK integration** | 60–90% token savings on dev operations |
| **MCP support** | stdio and network transport for any MCP server |

---

### Slash Commands

```
/model          → interactive model picker (all providers)
/loop <goal>    → autonomous multi-turn agent mission
/plan <task>    → decompose + execute step by step
/bughunter      → scan codebase for bugs
/commit         → AI-generated commit message + commit
/compress       → summarise and compact session history
/status         → show model, session, token usage
/help           → browse full command list
```

---

### Ecosystem

| Crate | Role |
|---|---|
| [`albert-cli`](https://crates.io/crates/albert-cli) | The `albert` binary |
| [`albert-runtime`](https://crates.io/crates/albert-runtime) | Session engine, MCP, auth, bash |
| [`albert-api`](https://crates.io/crates/albert-api) | Multi-provider LLM client |
| [`albert-commands`](https://crates.io/crates/albert-commands) | Slash command library |
| [`albert-tools`](https://crates.io/crates/albert-tools) | Tool execution layer |
| [`albert-compat`](https://crates.io/crates/albert-compat) | Manifest extraction harness |

→ **[Source: agent_albert_cli/](agent_albert_cli/)**  
→ **[crates.io: albert-cli v0.1.2](https://crates.io/crates/albert-cli)**

---

## Repository layout

| Directory | Contents |
|-----------|----------|
| [`ternlang-root/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root) | All Rust crates — compiler, VM, API, MCP server, ML stack |
| [`ternlang-root/stdlib/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/stdlib) | 293 open-core `.tern` modules (Tier 1 — free) |
| [`ternlang-root/examples/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/examples) | Runnable `.tern` examples (medical, finance, aerospace, etc.) |
| [`ternlang-root/spec/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/spec) | BET-ISA spec, language reference, grammar, protocol specs |
| [`ternlang-root/ternlang-web/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/ternlang-web) | ternlang.com frontend (GitHub Pages) |
| [`agent_albert_cli/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/agent_albert_cli) | Agent Albert — model-agnostic AI coding CLI + TernStudio intelligence layer |
| `eriirfos-eng/ternlang-premium` *(private)* | 28,495+ proprietary `.tern` modules — Tier 2 / 3 / 4 |

---

## Live API

```bash
# Health check
curl https://ternlang.com/health

# MoE-13 ternary decision via MCP (no key required)
curl -X POST https://ternlang.com/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"moe_orchestrate","arguments":{"query":"Should I proceed?"}}}'
```

---

## StdLib Access

The standard library is split across two repos to protect paid-tier IP:

- **Tier 1 (free):** 293 open-core modules in [`ternlang-root/stdlib/`](ternlang-root/stdlib/) — clone this repo and use immediately
- **Tier 2/3/4 (paid):** 28,495+ proprietary modules in the private `eriirfos-eng/ternlang-premium` repo

After purchasing: visit **[ternlang.com/activate](https://ternlang.com/activate)** — enter your API key + GitHub username and you'll receive a collaborator invite to the private repo automatically.

→ [Full tier breakdown](ternlang-root/stdlib/PREMIUM.md)

---

## Licensing

| Tier | Price | Details |
|------|-------|---------|
| Community (LGPL-3.0) | Free | Compiler, VM, CLI, LSP, 293 stdlib modules + **30 MCP tools (all free)** |
| Pro Standard (BSL-1.1) | €99/month | REST API (10,000 calls/month), server-side 3-layer memory, SSE streaming + Tier 2 stdlib |
| Industrial (BSL-1.1) | €349/month | 50,000 API calls, QNN, SEC, T-HAL, TernAudit + Tier 3 stdlib |
| Enterprise (Proprietary) | From €2,500/month | unlimited API calls | On-premise, FPGA, custom SLA + full Tier 4 stdlib | lifetime support|

Commercial licensing: [licensing@ternlang.com](mailto:licensing@ternlang.com)

---

## Team

The Ternary Intelligence Stack is built by a core team of three co-founders from **RFI-IRFOS**, Graz:

*   **Simeon Kepp**: Head of Research & Systems Architect.
*   **Nikoletta Csonka**: Head of Strategic Outreach & EU Relations.
*   **Zabih Karimi**: Principal Network & ML Engineer.
---

<div align="center">
  <img src="ternlang-root/ternlang-web/assets/ternlang_logo_notext.png" alt="Ternlang Logo" width="100">
</div>
