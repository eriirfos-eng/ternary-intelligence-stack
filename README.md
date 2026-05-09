# Ternary Intelligence Stack (TIS)

[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![version](https://img.shields.io/badge/version-v1.3.6-blue)](#architecture)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-138%20CI%20%7C%205%20crates-yellow)](#architecture)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![EU AI Act](https://img.shields.io/badge/EU%20AI%20Act-Article%2013,14+15%20Compliant%20Design-003399?logo=european-union)](https://ternlang.com/compliance)
[![MCP](https://img.shields.io/badge/MCP-34_tools-orange)](#mcp-server--v040-34-tools)
[![smithery badge](https://smithery.ai/badge/rfi-irfos/ternlang)](https://smithery.ai/servers/rfi-irfos/ternlang)
[![examples](https://img.shields.io/badge/examples+2k%2B_.tern_programs-blueviolet)](#architecture)
[![stdlib](https://img.shields.io/badge/stdlib-28k+_open_%26_2.5k+_premium-blue)](ternlang-root/stdlib/PREMIUM.md)
[![DOI](https://img.shields.io/badge/DOI-10.17605%2FOSF.IO%2FTZ7DC-informational)](https://doi.org/10.17605/OSF.IO/TZ7DC)
[![moe-core](https://img.shields.io/crates/v/moe-core.svg?label=moe-core)](https://crates.io/crates/moe-core)
[![moe-platform](https://img.shields.io/crates/v/moe-platform.svg?label=moe-platform)](https://crates.io/crates/moe-platform)
[![moe-runtime](https://img.shields.io/crates/v/moe-runtime.svg?label=moe-runtime)](https://crates.io/crates/moe-runtime)


Built by [RFI-IRFOS](https://ternlang.com) · Graz, Austria · Whitepaper [https://osf.io/cyn28]

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
# One line — installs Rust (if needed) + ternlang-cli, ready immediately
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source "$HOME/.cargo/env" && cargo install ternlang-cli
```
> **Note:** Do not use `sudo apt install cargo` — Ubuntu's packaged version is too old (1.75). The line above installs the current toolchain via rustup.

```bash
ternlang                       # launch interactive REPL immediately
ternlang my_program.tern       # run a .tern file directly
ternlang run my_program.tern   # explicit form
```



## 2. What is Albert-MoE-13? (The Intelligence Layer)

Albert is a ternary Mixture-of-Experts language model trained from scratch — not quantized from a float model. Every weight is in `{-γ, 0, +γ}` throughout training via Straight-Through Estimator (STE). The architecture expands itself autonomously via Net2Net surgery when it plateaus.

**Current state (2026-05-07):** 3L · 256H · 12E · Top-3 routing · 128CTX · 8000 vocab · ~10M params · training on CPU · growing autonomously via Net2Net surgery (target: 12L · ~58M params).

### What makes it different

| Feature | Albert MoE-13 | Standard LLM |
|--------|--------------|--------------|
| Weight precision | Ternary `{-γ, 0, +γ}` from scratch | Float32 / post-hoc INT4 |
| **@sparseskip** | Skips 56% of matmul ops at element level | Dense matmul always |
| Architecture growth | Autonomous Net2Net surgery (3L→12L, live on CPU) | Fixed at init |
| Inference speed | **83–125 tok/s on laptop CPU** | Requires GPU at this quality |
| Routing | 9/12 experts skipped per decode step | All experts active |
| Patent | A50296/2026 (@sparseskip primitive) | — |

### Try it (API — no install needed)
```bash
# Ternary decision: affirm / hold / reject
curl -s https://ternlang-api.fly.dev/api/trit_decide \
  -H "Content-Type: application/json" \
  -d '{"statement": "This architecture is worth funding"}' | jq .

# Sparse MoE reasoning over 13 expert domains
curl -s https://ternlang-api.fly.dev/api/moe/orchestrate \
  -H "Content-Type: application/json" \
  -H "X-Ternlang-Key: YOUR_KEY" \
  -d '{"query": "What is ternary logic?", "evidence": [0.9, 0.1]}' | jq .
```

*See [`BENCHMARKS.md`](ternlang-root/BENCHMARKS.md) for full sparsity speedup data and [`albert-moe-13/`](albert-moe-13/) for training code.*

### Known Limitations (honest)
- Albert at 7L is a **research prototype**, not a production LLM. He generates statistically coherent text but does not yet answer questions — that capability is targeted at 9L+ with instruction fine-tuning.
- Training runs on a single CPU (HP ZBook, no GPU). A proper GPU cluster would run 10-50× faster.
- Held-out perplexity vs float32 baseline is not yet published (in progress — `scripts/eval_perplexity.py`).
- The CUDA backend (`cuda_matmul.rs`) is a design sketch at TRL 3, not yet a running kernel.

---

## 3. Agent Albert — The Sovereign AI Assistant

**Albert** is also the terminal-native interface for the TIS. He runs entirely in your terminal, connects to local models (Ollama) or commercial bridges, and never “phones home.”

### Quick Start
```bash
# One line — installs Rust (if needed) + albert-cli, ready immediately
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source "$HOME/.cargo/env" && cargo install albert-cli
```
> **Note:** Do not use `sudo apt install cargo` — Ubuntu's packaged version is too old (1.75). The line above installs the current toolchain via rustup.

```bash
albert-cli     # launch immediately
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

## 4. Repository Architecture

This repository is split into three primary domains, each serving a distinct purpose in the TIS ecosystem:

| Path | Purpose |
|-----------|----------------|
| [`ternlang-root/`](ternlang-root/) | **The Orchestration Layer:** Compiler, BET VM, and the MoE-13 Orchestrator MCP server. This layer handles logical routing and ternary decision-making. |
| [`albert-moe-13/`](albert-moe-13/) | **Model Development Framework:** The native research framework for training scaling. Houses the crates responsible for ternary manifold adaptation, STE-based training, and model architecture. |
| [`agent_albert_cli/`](agent_albert_cli/) | **Sovereign Agent Layer:** The terminal-native, model-agnostic AI agent (Albert) built in pure Rust for autonomous coding and orchestration. |

### Note on Training Infrastructure
The full training pipeline — including the Straight-Through Estimator (STE) backward pass, EvolutionManager, and cosine LR schedule — is implemented in `albert-moe-13/moe-llm-core/src/bin/train_bible.rs`. The STE quantization primitive is at `albert-moe-13/moe-llm-core/src/model/ste.rs`. Empirical loss convergence data (25 epochs, descending from random baseline 8.987 to 6.95) is in `albert-moe-13/docs/convergence_log.md`. Massive-scale distributed training on GPU clusters is on the roadmap (Phase 23).

---

## Licensing

| Tier | Price | Details |
|------|-------|---------|
| Community (LGPL-3.0) | Free | Compiler, VM, CLI, LSP, 28,500+ open-core modules + 34 MCP tools |
| Pro Standard (BSL-1.1) | €99/month | REST API, server-side memory, Tier 2 'Masterwork' modules |
| Industrial (BSL-1.1) | €349/month | QNN, SEC, T-HAL, TernAudit + Tier 3 'Masterwork' modules |
| Enterprise (Proprietary) | From €2,500/month | On-premise, FPGA, custom SLA + Tier 4 'Masterwork' modules |

---

## Team

The Ternary Intelligence Stack is built by a core team of five co-founders from **RFI-IRFOS**, Graz:

*   **Simeon Kepp**: Head of Research & Systems Architect.
*   **Nikoletta Csonka**: Head of Strategic Outreach & EU Relations.
*   **Zabih Karimi**: Principal Network & ML Engineer.
*   **Lisa Scharler**: Head of Social Technology & Ecocentric Systems.
*   **Louis Ehrig**: Corporate Secretary and Press & Media Relations.

→ **[Read our BIO and Mission in LEADERSHIP.md](LEADERSHIP.md)**

---

<div align="center">
  <img src="ternlang-root/ternlang-web/assets/ternlang_logo_notext.png" alt="Ternlang Logo" width="100">
</div>
