# Ternary Intelligence Stack (TIS)

[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![version](https://img.shields.io/badge/version-v0.3.0-black)](#architecture)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-212%2B%20passing-brightgreen)](#architecture)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![EU AI Act](https://img.shields.io/badge/EU%20AI%20Act-Article%2013%20Compliant%20Design-003399?logo=european-union)](https://ternlang.com/compliance)
[![speedup](https://img.shields.io/badge/@sparseskip-up_to_122x-success)](#sparse-ternary-inference)
[![MCP](https://img.shields.io/badge/MCP-19_tools-orange)](#mcp-server--v030-19-tools)
[![examples](https://img.shields.io/badge/examples-2,090%2B_.tern_programs-blueviolet)](#example-library)
[![stdlib](https://img.shields.io/badge/stdlib-27,000%2B_modules-blue)](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/stdlib)
[![DOI](https://img.shields.io/badge/DOI-10.17605%2FOSF.IO%2FTZ7DC-informational)](https://doi.org/10.17605/OSF.IO/TZ7DC)



Ternlang is a systems programming language, compiler, and high-performance inference runtime built on balanced ternary logic. We provide a fundamental architectural shift for **Explainable AI (XAI)** and European technological sovereignty.

Built by [RFI-IRFOS](https://ternlang.com) · Graz, Austria · Whitepaper [https://osf.io/cyn28/files/8hzux]

---

## Technical Pillars

- **Deterministic Uncertainty**: Ternlang's `trit` (affirm/tend/reject) provides a first-class routing mechanism for **Uncertainty-Aware AI**, eliminating "hallucinated confidence."
- **Sparsity-Aware Inference Engine**: Native `@sparseskip` optimization achieves up to 122x throughput gains by bypassing zero-signal (`tend`) weights at the hardware primitive level.
- **Explainable AI (XAI) by Design**: Every decision is auditable and traceable, fulfilling **EU AI Act Articles 13, 14, and 15** mandates for algorithmic transparency and human oversight.
- **Post-Binary Systems Architecture**: A full-stack ecosystem including a custom **Instruction Set Architecture (ISA)**, triadic networking, and memory-efficient ternary encoding.

The core type is `trit`: three values — `−1` (reject), `0` (hold), `+1` (affirm).
The zero state is a first-class routing instruction: *"insufficient confidence — do not act yet."*

Ternlang provides a machine-readable path to human escalation instead of a forced binary guess. This is the foundation for **Post-Binary Intelligence**.

---
## Full Technical Documentation

→ **[ternlang-root/README.md](ternlang-root/README.md)** (Architecture, Benchmarks, and Spec)

---

## Team

The Ternary Intelligence Stack is built by a core team of three co-founders from **RFI-IRFOS**, Graz:

*   **Simeon Kepp**: Head of Research & Systems Architect.
*   **Nikoletta Csonka**: Head of Strategic Outreach & EU Relations.
*   **Zabih Karimi**: Principal Network & ML Engineer.

**[Read our BIOS and Mission in LEADERSHIP.md](LEADERSHIP.md)**

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
git clone https://github.com/eriirfos-eng/ternary-intelligence-stack
cd ternary-intelligence-stack/ternlang-root
cargo build --release
./target/release/ternlang run examples/03_rocket_launch.tern
```

Or install the CLI directly:

```bash
cargo install ternlang-cli
```

---

## Repository layout

| Directory | Contents |
|-----------|----------|
| [`ternlang-root/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root) | All Rust crates — compiler, VM, API, MCP server, ML stack |
| [`ternlang-root/stdlib/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/stdlib) | 300+ `.tern` programs across every domain |
| [`ternlang-root/examples/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/examples) | Runnable `.tern` examples (medical, finance, aerospace, etc.) |
| [`ternlang-root/spec/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/spec) | BET-ISA spec, language reference, grammar, protocol specs |
| [`ternlang-root/ternlang-web/`](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/ternlang-root/ternlang-web) | ternlang.com frontend (GitHub Pages) |

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

## Licensing

| Tier | Price | Details |
|------|-------|---------|
| Community (LGPL-3.0) | Free | Compiler, VM, CLI, LSP — full open source |
| Pro Standard (BSL-1.1) | €99/month | 10,000 API calls, MoE-13, 3-layer memory, MCP |
| Industrial (BSL-1.1) | €349/month | 50,000 API calls, QNN, SEC, T-HAL, TernAudit |
| Enterprise (Proprietary) | From €2,500/month | On-premise, FPGA, custom SLA |

BSL-1.1 crates auto-convert to Apache-2.0 on 2030-04-03.

Commercial licensing: [licensing@ternlang.com](mailto:licensing@ternlang.com)

---

*RFI-IRFOS (ZVR: 1015608684) · Graz, Austria · EU AI Act Article 13/14 compliant design*
