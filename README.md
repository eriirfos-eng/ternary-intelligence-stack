# Ternary Intelligence Stack (TIS)

**Ternlang is a systems programming language, compiler, and ML inference runtime built on balanced ternary logic.**

[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](ternlang-root/LICENSE)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![MCP](https://img.shields.io/badge/MCP-+20%20tools%20v0.3.0-purple)](https://ternlang.com/mcp)
[![EU AI Act](https://img.shields.io/badge/EU%20AI%20Act-Article%2013%20Compliant%20Design-003399)](https://ternlang.com/compliance)
[![Data Residency](https://img.shields.io/badge/Data%20Residency-EU%20Frankfurt-003399)](https://ternlang.com/compliance)

Built by [RFI-IRFOS](https://ternlang.com) · [ternlang.com](https://ternlang.com) · [Whitepaper (DOI)](https://doi.org/10.17605/OSF.IO/TZ7DC)

---

The core type is `trit`: three values — `−1` (reject), `0` (hold), `+1` (affirm).
The zero state is not an error. It is a first-class routing instruction: *"insufficient confidence — do not act yet."*

This makes Ternlang the natural foundation for AI systems that must reason honestly under uncertainty — with a machine-readable path to human escalation instead of a forced binary guess.

---

## Full documentation

→ **[ternlang-root/README.md](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/README.md)**

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
