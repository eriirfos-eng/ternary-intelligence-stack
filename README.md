# Ternary Intelligence Stack (TIS) 

[![version](https://img.shields.io/badge/version-v1.2.6-turqouise)](#architecture)
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
- **Sovereign Compliance Infrastructure**: Native compliance with **EU AI Act Articles 13, 14, and 15** via the ternary 0-state escalation protocol.
- **Sparsity-Aware Inference Engine**: Native `@sparseskip` optimization achieves up to 122x throughput gains by bypassing zero-signal (`tend`) weights at the hardware primitive level.
- **Explainable AI (XAI) by Design**: Every decision is auditable and traceable via TernAudit Guard.
- **Post-Binary Systems Architecture**: A full-stack ecosystem including a custom **Instruction Set Architecture (ISA)**, triadic networking, and memory-efficient ternary encoding.

---
## Core Modules & Ecosystem

| Module | Role | Compliance / Utility |
|---|---|---|
| [`pytern`](https://crates.io/crates/pytern) | Python-to-Ternary Transpiler | High-performance MoE model training |
| [`ternaudit-guard`](https://crates.io/crates/ternaudit-guard) | Sovereign Compliance SDK | EU AI Act (Arts 13, 14, 15) Audit Logs |
| [`albert-cli`](https://crates.io/crates/albert-cli) | Agentic Coding CLI | Model-agnostic AI agent engine |
| [`ternlang-mcp`](https://crates.io/crates/ternlang-mcp) | Ternary MCP Server | AI-to-Balanced Ternary Logic Bridge |
| [`ternlang-core`](https://crates.io/crates/ternlang-core) | Ternlang Compiler | Core ISA and runtime |

---
## Full Documentation
  
→ **[ternlang-root/README.md](ternlang-root/README.md)** (Full explanation, technical details, and compiler specs)

→ **[ROADMAP.md](ternlang-root/docs/ROADMAP.md)** (Phases 1–18, session log, priority matrix)

→ **[Ternlang Studio Preview](https://ternlang-api.fly.dev/studio)** — Our work-in-progress SDK

→ **[Agent Albert](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/agent_albert_cli)** — terminal-native, model-agnostic AI agent built in pure Rust

---

## Performance Benchmarks

| Feature | Performance Gain | Industry Comparison |
|---------|------------------|---------------------|
| **Ternary Inference** | 2.3x (baseline) | Up to 122x at 99%+ Sparsity |
| **Data Density** | 1.25x improvement | 5-trit block packing (8-bit) |
| **Logic Consistency** | 100% Deterministic | Eliminates binary timeout/null-guessing |
| **Audit Fidelity** | Real-time / PDF / JSON | EU AI Act Audit-ready |

---

## Quick start

```bash
cargo install ternlang-cli
```

---

## Agent Albert — AI Intelligence Layer

[![crates.io](https://img.shields.io/crates/v/albert-cli.svg)](https://crates.io/crates/albert-cli)
[![version](https://img.shields.io/badge/version-v1.2.6-cyan)](https://crates.io/crates/albert-cli)

---
*Albert* is the sovereign, model-agnostic AI coding CLI and embedded intelligence layer of the Ternary Intelligence Stack. He runs entirely in your terminal, connects to any LLM provider, and never phones home.

### Quick Install — One Copy Gets It All

```bash
cargo install albert-cli
albert-cli
```

### What Albert Can Do
| Capability | Details |
|---|---|
| **Multi-provider routing** | Claude, GPT-4o, Gemini, Grok, Ollama, Bedrock, Azure — swap with `/model` |
| **Autonomous agent loop** | `/loop <mission>` — runs up to 10 tool-use turns to complete a goal |
| **Tool harness** | `read_file`, `write_file`, `edit_file`, `bash`, `glob_search`, `grep_search`, `web_fetch` |
| **Self-reflection memory** | Automatically scores each turn for importance |
| **Slash command library** | `/plan`, `/loop`, `/tdd`, `/bughunter`, `/code-review`, `/build-fix` |

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

## Ecosystem

| Crate | Role |
|---|---|
| [`albert-cli`](https://crates.io/crates/albert-cli) | The `albert` binary |
| [`albert-runtime`](https://crates.io/crates/albert-runtime) | Session engine, MCP, auth, bash |
| [`ternaudit-guard`](https://crates.io/crates/ternaudit-guard) | Sovereign Compliance SDK |
| [`pytern`](https://crates.io/crates/pytern) | Python-to-Ternary Transpiler |

---

## Live API

```bash
# Health check
curl https://ternlang.com/health
```

---

## StdLib Access

- **Tier 1 (free):** 293 open-core modules in [`ternlang-root/stdlib/`](ternlang-root/stdlib/)
- **Tier 2/3/4 (paid):** 28,495+ proprietary modules in the private `eriirfos-eng/ternang-premium` repo

---

## Licensing

Commercial licensing: [licensing@ternlang.com](mailto:licensing@ternlang.com)

---

## Team

The Ternary Intelligence Stack is built by a core team of four co-founders from **RFI-IRFOS**, Graz:

*   **Simeon Kepp**: Head of Research & Systems Architect.
*   **Nikoletta Csonka**: Head of Strategic Outreach & EU Relations.
*   **Zabih Karimi**: Principal Network & ML Engineer.
*   **Louis Paul Ehrig**: Head of Press & Public Affairs
---
→ **[Read our BIO and Mission in LEADERSHIP.md](LEADERSHIP.md)**

<div align="center">
  <img src="ternlang-root/ternlang-web/assets/ternlang_logo_notext.png" alt="Ternlang Logo" width="100">
</div>
