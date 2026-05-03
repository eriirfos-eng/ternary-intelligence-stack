# Ternary Intelligence Stack (TIS)

[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![version](https://img.shields.io/badge/version-v1.2.9-blue)](#architecture)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-88%2B%20passing-brightgreen)](#architecture)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![EU AI Act](https://img.shields.io/badge/EU%20AI%20Act-Article%2013,14+15%20Compliant%20Design-003399?logo=european-union)](https://ternlang.com/compliance)
[![MCP](https://img.shields.io/badge/MCP-30_tools-orange)](#mcp-server--v030-19-tools)
[![smithery badge](https://smithery.ai/badge/rfi-irfos/ternlang)](https://smithery.ai/servers/rfi-irfos/ternlang)
[![examples](https://img.shields.io/badge/examples+2k%2B_.tern_programs-blueviolet)](#example-library)
[![stdlib](https://img.shields.io/badge/stdlib-1k+_open_%26_28k+_premium-blue)](stdlib/PREMIUM.md)
[![DOI](https://img.shields.io/badge/DOI-10.17605%2FOSF.IO%2FTZ7DC-informational)](https://doi.org/10.17605/OSF.IO/TZ7DC)
[![moe-core](https://img.shields.io/crates/v/moe-core.svg?label=moe-core)](https://crates.io/crates/moe-core)
[![moe-platform](https://img.shields.io/crates/v/moe-platform.svg?label=moe-platform)](https://crates.io/crates/moe-platform)
[![moe-runtime](https://img.shields.io/crates/v/moe-runtime.svg?label=moe-runtime)](https://crates.io/crates/moe-runtime)


Built by [RFI-IRFOS](https://ternlang.com) · Graz, Austria · Whitepaper [https://osf.io/cyn28/files/8hzux]

---

### Full Documentation

- **[README.md](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/README.md)** — Full explanation, technical details, and compiler specifications
- **[Albert-MoE-13: Ternary Scaling Research](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/albert-moe-13)** — Research into natively trained ternary scaling.
- **[Agent Albert CLI](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/agent_albert_cli)** — Terminal-native, model-agnostic AI agent built in pure Rust
- **[Ternlang Studio (Preview)](https://ternlang-api.fly.dev/studio)** — Work-in-progress developer dashboard and SDK
- **[Session Log](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/docs/session_log.md)** — Production fixes and refinements addressed during deployment
- **[Roadmap](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/docs/ROADMAP.md)** — Phases 1–20 and priority matrix 

---

## 1. What is Ternlang? (The Infrastructure Layer)

Ternlang is a systems programming language, compiler, and high-performance inference runtime built on balanced ternary logic. 
The Stack provides a fundamental architectural shift for **Explainable AI (XAI)** and European technological sovereignty by moving beyond the binary limitations of current systems.


The core type is `trit`: three values — `−1` (reject), `0` (hold), `+1` (affirm). This allows for **Deterministic Uncertainty**, where the zero state is a first-class routing instruction: *"insufficient confidence — do not act yet."*

### Quick Start
```bash
# Ensure Rust is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the Ternlang CLI
cargo install ternlang-cli

# Start developing
ternlang                       # launch interactive REPL immediately
ternlang my_program.tern       # run a .tern file directly
ternlang run my_program.tern   # explicit form
```



## 2. What is Albert-MoE-13? (The Intelligence Layer)

We are researching the fundamental scaling laws of natively trained ternary neural networks.

**Albert** is our experimental framework for ternary Mixture-of-Experts (MoE). Unlike binary transformer architectures, Albert explores how scaling parameter counts ($N$) in a ternary manifold ($\{-1, 0, +1\}$) leads to stable loss convergence without the need for high-precision float training.

- **Scaling Dimension**: Investigating how ternary-native representations scale predictably to 1T+ parameters.
- **Manifold Stability**: Empirically measuring loss convergence and gradient flow through discrete ternary thresholds (STE).
- **Native Ternary Training**: Training from scratch on the ternary manifold to optimize representation efficiency.
- **Sparse Geometric Scaling**: Leveraging the sparse geometry of ternary weights for sub-linear memory requirements.

Base Architecture: Scalable Ternary MoE (Multi-Domain Experts)
Objective: Demonstrate empirical scaling laws and representation efficiency of native ternary manifolds.
Why This Matters: Move AI scaling beyond the energy-intensive binary-float paradigm.

### Empirical Scaling Metrics

We measure architectural success through ternary convergence and scaling efficiency.

| Metric | Scientific Focus | Empirical Baseline |
|--------|------------------|--------------------|
| **Loss Convergence ($N$)** | Power-law scaling | Stable scaling up to 1T params |
| **Manifold Sparsity** | Sparse geometric efficiency | ~32% stable sparsity |
| **Ternary Manifold ($\alpha$)** | Signal amplitude stability | $\alpha \approx 0.55$ invariant |

*See [`BENCHMARKS.md`](ternlang-root/BENCHMARKS.md) for full experimental data.*

---

## 3. Agent Albert — The Sovereign AI Assistant

**Albert** is also the terminal-native interface for the TIS. He runs entirely in your terminal, connects to local models (Ollama) or commercial bridges, and never “phones home.”

### Quick Start
```bash
# Ensure Rust is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Agent Albert
cargo install albert-cli

# Launch the agent
albert-cli
```
---

### Key Capabilities

| Capability | Details |
|---|---|
| **Autonomous agent loop** | `/loop <mission>` — runs up to 10 tool-use turns to complete a goal |
| **Local-First** | Native Ollama support for completely offline and free operation |
| **Model-agnostic** | Connect to Gemini, Claude, GPT-4o, or Grok as "capability bridges" |
| **Self-reflection memory** | Commits key facts to local storage to maintain long-term context |

---

## Repository Architecture

This repository is split into two primary domains, each serving a distinct purpose in our research.

| Path | Purpose |
|-----------|----------------|
| `ternlang-root/` | **The Orchestration Layer:** Compiler, BET VM, and the MoE-13 Orchestrator MCP server. This layer handles logical routing and ternary decision-making. |
| `albert-moe-13/` | **Model Development Framework:** The native research framework for training scaling. Houses the crates responsible for ternary manifold adaptation, STE-based training, and model architecture. |

### Note on Training Infrastructure
As of current development, this repository houses the foundational model architecture, ternary math, and scaling research framework (see `albert-moe-13/crates/`). Massive-scale distributed training infrastructure is currently managed in a separate, secured workflow or remains in pre-cluster, experimental development stages. Experimental benchmarks and logic can be found in `albert-moe-13/crates/moe-core/src/training/`.

---

## Licensing

| Tier | Price | Details |
|------|-------|---------|
| Community (LGPL-3.0) | Free | Compiler, VM, CLI, LSP, 1k+ open-core modules + 30 MCP tools |
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
*   **Louis Ehrig**: Corporate Secretary and Press & Media Relations.

→ **[Read our BIO and Mission in LEADERSHIP.md](docs/LEADERSHIP.md)**

---

<div align="center">
  <img src="ternlang-root/ternlang-web/assets/ternlang_logo_notext.png" alt="Ternlang Logo" width="100">
</div>
