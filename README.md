# Ternary Intelligence Stack (TIS)

[![version](https://img.shields.io/badge/version-v1.2.9-blue)](#architecture)
[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-88%2B%20passing-brightgreen)](#architecture)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![EU AI Act](https://img.shields.io/badge/EU%20AI%20Act-Article%2013,14+15%20Compliant%20-003399?logo=european-union)](https://ternlang.com/compliance)
[![MCP](https://img.shields.io/badge/MCP-30_tools_free-orange)](#live-api)
[![smithery badge](https://smithery.ai/badge/rfi-irfos/ternlang)](https://smithery.ai/servers/rfi-irfos/ternlang)
[![examples](https://img.shields.io/badge/examples-2,090%2B_.tern_programs-blueviolet)](#example-library)
[![stdlib](https://img.shields.io/badge/stdlib-293_open%20%2B%2028k%2B_premium-blue)](ternlang-root/stdlib/PREMIUM.md)
[![DOI](https://img.shields.io/badge/DOI-10.17605%2FOSF.IO%2FTZ7DC-informational)](https://doi.org/10.17605/OSF.IO/TZ7DC)

The Ternary Intelligence Stack (TIS) provides a fundamental architectural shift for **Explainable AI (XAI)** and European technological sovereignty by moving beyond the binary limitations of current systems.

Built by [RFI-IRFOS](https://ternlang.com) · Graz, Austria · Whitepaper [https://osf.io/cyn28/files/8hzux]

---

### Full Documentation
  
→ **[ternlang-root/README.md](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/README.md)** (Full explanation, technical details, and compiler specs)

→ **[ROADMAP.md](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/docs/ROADMAP.md)** (Phases 1–20, MoE-13 structure, priority matrix)

→ **[session_log.md](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/docs/session_log.md)** (Details to the Fixes we adressed during production)

→ **[Ternlang Studio Preview](https://ternlang-api.fly.dev/studio)** — Our work-in-progress SDK

→ **[Agent Albert](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/agent_albert_cli)** — terminal-native, model-agnostic AI agent built in pure Rust

---

## Ternlang (The Infrastructure Layer)

Ternlang is a systems programming language, compiler, and high-performance inference runtime built on balanced ternary logic.

The core type is `trit`: three values — `−1` (reject), `0` (hold), `+1` (affirm). This allows for **Deterministic Uncertainty**, where the zero state is a first-class routing instruction: *"insufficient confidence — do not act yet."*

## Performance Benchmarks

| Feature | Performance Gain | Industry Comparison |
|---------|------------------|---------------------|
| **Data Density** | 1.25x improvement | 5-trit block packing (8-bit) |
| **Logic Consistency** | 100% Deterministic | Eliminates binary timeout/null-guessing |
| **Safety Latency** | < 1ms hard-veto | Axis-6 Veto Hard Gate |

---

## Quick start (Compiler)

```bash
cargo install ternlang-cli
```

The `ternlang` binary provides the compiler, REPL, and test runner:

```bash
ternlang my_program.tern        # → run a .tern file directly
ternlang build my_program.tern  # → compile to .bet bytecode
```
---

## Albert-MoE-13 — The Intelligence Layer

We are actively designing, training, and deploying our own **ternary-native Mixture-of-Experts model**.

**Albert** is our in-progress **MoE-13 (Mixture-of-Experts) ternary LLM**.  
Unlike traditional binary LLMs, Albert uses **13 domain-specific experts** to deliberate on every decision.

---

### Core Architecture

- **13 Deliberation Axes**  
  Each expert represents a distinct reasoning dimension (e.g. Safety, Ethics, Logic, FactCheck, etc.),  
  voting using **ternary signals**: `{ -1, 0, +1 }`.

- **Safety Hard Gate (Axis-6)**  
  A mandatory veto layer capable of **terminating any execution path** before action is taken.

- **Local-First Sovereignty**  
  Designed to run fully offline on **local infrastructure** via Ollama or custom ternary hardware.

---

### Technical Stack

- **Base Model**  
  Open-source MoE in the **20B–30B parameter class**

- **Transformation**  
  Full **ternarization into `{ -1, 0, +1 }` weight space**

- **Objective**  
  - Reduce memory footprint  
  - Enable **sparsity-aware execution**  
  - Improve deterministic interpretability  

- **Architecture**  
  Consolidation into **13 domain-specific expert routers (MoE-13)**

---

### Why This Matters

Albert is designed to:

- Operate fully **offline** on sovereign infrastructure  
- Enforce **deterministic uncertainty** via the `HOLD (0)` state  
- Integrate natively with **Ternlang** and the **BET-VM runtime**  
- Provide a **verifiable decision layer**, not just probabilistic outputs  
- Enable **traceable, inspectable reasoning paths**

---

## 3. Agent Albert — The Sovereign AI Assistant

[![crates.io](https://img.shields.io/crates/v/albert-cli.svg)](https://crates.io/crates/albert-cli)
[![version](https://img.shields.io/badge/version-v1.2.9-cyan)](https://crates.io/crates/albert-cli)

**Albert** is also the terminal-native interface for the TIS. He runs entirely in your terminal, connects to local models (Ollama) or commercial bridges, and never “phones home.”

### Quick Install

```bash
# Install Albert (brings the full agent engine with it)
cargo install albert-cli

# Launch
albert-cli
```

### Key Capabilities

| Capability | Details |
|---|---|
| **Autonomous agent loop** | `/loop <mission>` — runs up to 10 tool-use turns to complete a goal |
| **Local-First** | Native Ollama support for completely offline and free operation |
| **Model-agnostic** | Connect to Gemini, Claude, GPT-4o, or Grok as "capability bridges" |
| **Self-reflection memory** | Commits key facts to local storage to maintain long-term context |


---

## Repository layout

| Directory | Contents |
|-----------|----------|
| [`ternlang-root/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root) | All Rust crates — compiler, VM, API, MCP server, ML stack |
| [`agent_albert_cli/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/agent_albert_cli) | Agent Albert — terminal-native CLI + TernStudio intelligence layer |
| [`ternlang-root/stdlib/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/stdlib) | 293 open-core `.tern` modules (Tier 1 — free) |
| `eriirfos-eng/ternlang-premium` *(private)* | 28,495+ proprietary `.tern` modules — Tier 2 / 3 / 4 |

---

## Licensing

| Tier | Price | Details |
|------|-------|---------|
| Community (LGPL-3.0) | Free | Compiler, VM, CLI, LSP, 293 stdlib modules + 30 MCP tools |
| Pro Standard (BSL-1.1) | €99/month | REST API, server-side memory, Tier 2 stdlib |
| Industrial (BSL-1.1) | €349/month | QNN, SEC, T-HAL, TernAudit + Tier 3 stdlib |
| Enterprise (Proprietary) | From €2,500/month | On-premise, FPGA, custom SLA + Tier 4 stdlib |

---

## Team

The Ternary Intelligence Stack is built by a core team of five co-founders from **RFI-IRFOS**, Graz:

*   **Simeon Kepp**: Head of Research & Systems Architect.
*   **Nikoletta Csonka**: Head of Strategic Outreach & EU Relations.
*   **Zabih Karimi**: Principal Network & ML Engineer.
*   **Lisa Scharler**: Head of Social Technology & Ecocentric Systems.
*   **Louis Ehrig**: Louis Paul Ehrig Corporate Secretary and Press & Media Relations.

→ **[Read our BIO and Mission in LEADERSHIP.md](LEADERSHIP.md)**

---

<div align="center">
  <img src="ternlang-root/ternlang-web/assets/ternlang_logo_notext.png" alt="Ternlang Logo" width="100">
</div>
