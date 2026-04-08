# Ternlang — Balanced Ternary Intelligence Stack

**The definitive platform for balanced ternary computing.**

[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-212%2B%20passing-brightgreen)](#architecture)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![MCP](https://img.shields.io/badge/MCP-20%20tools%20v0.3.0-purple)](https://ternlang.com/mcp)
[![Linguist PR](https://img.shields.io/badge/GitHub%20Linguist-PR%20pending-yellow)](https://github.com/github/linguist/pulls)

Built by [RFI-IRFOS](https://ternlang.com) · [ternlang.com](https://ternlang.com) · [Whitepaper (DOI)](https://doi.org/10.17605/OSF.IO/TZ7DC)

---

## The Problem with Binary AI

Every AI system today is forced to answer yes or no — even when the evidence is contradictory, incomplete, or genuinely uncertain. Binary logic has no formal representation for *"I don't know yet."* Systems either hallucinate a confident answer or return null.

Ternlang adds the third state.

| Trit | Name | What it means |
|------|------|---------------|
| `−1` | **reject** | Clear negative signal. Do not proceed. |
| ` 0` | **tend** | Insufficient data. Gather more before acting. |
| `+1` | **affirm** | Clear positive signal. Proceed. |

The `tend` state is not indecision. It is a **first-class routing instruction** — a computational directive to remain in deliberation until evidence crosses a threshold. This makes ternlang the natural foundation for AI agents that must reason honestly under uncertainty.

---

## What's in This Repository

| Layer | What it does |
|-------|-------------|
| [Language & VM](#language--vm) | Compile and run `.tern` programs on the Balanced Ternary Execution VM |
| [Sparse Inference](#sparse-ternary-inference) | BitNet-style ternary weights with 86–122× speedup over dense float32 |
| [MoE-13 Orchestrator](#moe-13-ternary-orchestrator) | Mixture-of-Experts reasoning engine with safety hard gate |
| [Strategic Standards](#strategic-standards) | BET-ISA, TFP-754, TSON, TTP, and T-POSIX — The rules of post-binary computing |
| [Enterprise Middleware](#architecture) | **cuTern** (MKL), Ternary SQL, Triadic Networking, and Crypto |
| [Frontier Tech](#architecture) | Qutrit Quantum bridging, BCI neural decoding, and Interplanetary DTN |
| [Example Library](#example-library) | 300+ `.tern` programs across every domain |

---

## Language & VM

Ternlang programs use `trit` as the only scalar type. Every `match` must cover all three arms — the compiler rejects non-exhaustive matches.

```ternlang
// A ternary medical triage gate
fn patient_conscious(signal: trit) -> trit {
    match signal {
        reject => { return reject; }   // hard gate — unconscious patient blocks all other evaluation
        tend   => { return tend;   }
        affirm => { return affirm; }
    }
}

fn vital_signs(heart: trit, pressure: trit) -> trit {
    return consensus(heart, pressure);
}

let conscious: trit = patient_conscious(affirm);

match conscious {
    reject => { return reject; }   // immediate escalation, no further checks
    tend   => { return tend;   }
    affirm => {
        let vitals: trit = vital_signs(affirm, tend);
        match vitals {
            reject => { return reject; }
            tend   => { return tend;   }
            affirm => { return affirm; }
        }
    }
}
```

**Built-in Standard Library:** 217+ modules including `std::*`, `classical::*`, `nn::*`, `nlp::*`, `vision::*`, `rl::*`, `stats::*`, and research-grade `qnn::*`.

**Compiler Features:** First-class `affirm/tend/reject` keywords · Binary `if/while` fallbacks · Tensor indexing `obj[r,c]` · Built-in `use` resolver with zero runtime I/O.

**Quick start — install the CLI:**

```bash
cargo install ternlang-cli
```

Then run any `.tern` file directly from your terminal:

```bash
ternlang run my_program.tern
ternlang run examples/03_rocket_launch.tern
ternlang build my_program.tern --output my_program.bet
ternlang repl
ternlang fmt my_program.tern --write
```

**Or build from source:**

```bash
git clone https://github.com/eriirfos-eng/ternary-intelligence-stack
cd ternary-intelligence-stack/ternlang-root
cargo build --release
./target/release/ternlang run examples/03_rocket_launch.tern
```

---

## Sparse Ternary Inference

`mul(a, 0) = 0` for all `a` — provably zero, no computation needed. The `ternlang-ml` kernel precomputes a Compressed Sparse Column index, flattens weights to raw `i8`, and dispatches rows in parallel via Rayon. No branches in the inner loop.

**Goldilocks sparsity sweep** (release build, 3-rep median):

| Sparsity | 32² | 64² | 128² | 256² | 512² |
|----------|-----|-----|------|------|------|
| 25% | 6.3× | 11.5× | 26.4× | 39.3× | 53.1× |
| 40% | 6.3× | 13.1× | 29.6× | 46.0× | 73.6× |
| **50%** | **5.9×** | **10.2×** | **28.7×** | **56.6×** | **82.1×** |
| **60%** | **5.8×** | **9.5×** | **27.9×** | **32.1×** | **86.1×** |
| 99% | 1.8× | 9.9× | 13.1× | 53.9× | **122.3×** |

**Peak: 122× at 512×512, 99% sparsity.**
**Goldilocks zone: 40–60% → 20–86× on medium matrices.** This is exactly where BitNet b1.58 quantization (`τ = 0.5 × mean(|w|)`) naturally places weights in trained language models. The kernel and the quantization scheme are structurally aligned.

---

## MoE-13 Ternary Orchestrator

`ternlang-moe` implements the MoE-13 architecture ([DOI: 10.17605/OSF.IO/TZ7DC](https://doi.org/10.17605/OSF.IO/TZ7DC)) — a ternary Mixture-of-Experts system that routes queries through a pool of 13 domain experts, synthesises an emergent signal, enforces a hard safety veto, and returns a ternary decision with confidence and temperature.

```rust
use ternlang_moe::TernMoeOrchestrator;

let mut orch = TernMoeOrchestrator::with_standard_experts();

// [syntax, world_knowledge, reasoning, tool_use, persona, safety]
let evidence = [0.6, 0.7, 0.8, 0.5, 0.4, 0.9];
let result = orch.orchestrate("Should I proceed with this action?", &evidence);

println!("trit={} conf={:.0}% held={}", result.trit, result.confidence * 100.0, result.held);
// → trit=1 conf=84% held=false
println!("{}", result.prompt_hint);
// → "Affirm with confidence 84%. Emergent field amplifying."
```

**How it works:**

1. **Dual-key routing** — scores every expert pair by `relevance_a × relevance_b × synergy`. Complementary experts outperform redundant ones.
2. **1+1=3 triad synthesis** — emergent field `Ek = synergy × (vi + vj) / 2`. Two orthogonal experts produce a third signal neither could generate alone.
3. **Safety hard gate** — Axis-6 veto fires before any vote. Every veto is permanently logged to `AxisMemory` for audit.
4. **Hold with tiebreaker** — a split vote or low confidence yields `trit=0`. The orchestrator invokes a tiebreaker (max 4 active experts) before committing, modelling the human *"let me think about this"* behaviour.
5. **Three-tier memory** — Node (TTL: seconds), Cluster (routing frequency, mode-collapse risk), Axis (persistent priors + veto audit log).

**13 standard experts:** Syntax · WorldKnowledge · DeductiveReason · InductiveReason · ToolUse · Persona · Safety · FactCheck · CausalReason · AmbiguityRes · MathReason · ContextMem · MetaSafety

**AgentHarness** provides a pluggable interface for all 13 experts:

```rust
use ternlang_moe::agents::AgentHarness;

let harness = AgentHarness::with_standard_agents();
let verdicts = harness.run("Is this safe to execute?", &evidence);
```

---

## Live API

The full TIS API runs at **`https://ternlang.com`** — deployed on Fly.io, Frankfurt region.

```bash
# Health check
curl https://ternlang.com/health

# MoE-13 orchestration (no API key required for MCP)
curl -X POST https://ternlang.com/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call",
       "params":{"name":"moe_orchestrate",
                 "arguments":{"query":"Should I send this email?"}}}'

# Scalar ternary decision (API key required)
curl -X POST https://ternlang.com/api/trit_decide \
  -H "X-Ternlang-Key: your_key" \
  -H "Content-Type: application/json" \
  -d '{"evidence":[0.8, -0.2, 0.6, 0.9]}'
```

**REST endpoints** (require `X-Ternlang-Key`):

| Endpoint | Description |
|----------|-------------|
| `POST /api/trit_decide` | Float evidence array → reject / tend / affirm + confidence |
| `POST /api/trit_vector` | Named dimensions with weights → aggregate ternary decision |
| `POST /api/trit_consensus` | `consensus(a, b)` → ternary result |
| `POST /api/trit_deliberate` | EMA convergence loop — multi-round evidence → stable trit |
| `POST /api/trit_coalition` | N-agent weighted vote → quorum / dissent / abstain |
| `POST /api/trit_gate` | Multi-dimensional hard-block safety gate |
| `POST /api/moe/orchestrate` | Full MoE-13 pass — synchronous JSON result |
| `GET  /api/stream/moe_orchestrate` | MoE-13 pass streamed round-by-round via SSE |
| `GET  /api/stream/deliberate` | EMA deliberation streamed per round via SSE |
| `GET  /api/usage` | Monthly usage stats for the authenticated key |

**API key:** [ternlang.com/pricing](https://ternlang.com/pricing) · Tier 2 (€24/month): 10,000 calls/month, calendar-month reset

### MCP Server — v0.3.0 (20 tools)

The MCP server runs at `https://ternlang.com/mcp` — compatible with Claude Desktop, Smithery, and any HTTP MCP client.

**10 free tools (no key):** `trit_decide` · `trit_consensus` · `trit_eval` · `ternlang_run` · `quantize_weights` · `sparse_benchmark` · `moe_orchestrate` · `moe_deliberate` · `trit_action_gate` · `trit_upgrade`

**10 premium tools (X-Ternlang-Key):** `trit_compress` · `trit_triage` · `trit_plan` · `trit_factcheck` · `moe_full` · `trit_mem_write` · `trit_mem_read` · `trit_mem_consolidate` · `trit_mem_stats` · `trit_mem_compress`

#### Three-Layer AI Memory (v0.3.0 flagship)

The `trit_mem_*` tools implement a server-side three-layer memory system modelled on human memory consolidation:

| Layer | TTL | Capacity | Write behaviour | Consolidation |
|-------|-----|----------|-----------------|---------------|
| `working` | 1h | LRU-256 | Raw write | Affirm → session (compressed) |
| `session` | 24h | LRU-128 | Ternary-compressed | Affirm at half-life → MoE-13 → core |
| `core` | Never | Unlimited | Compressed + MoE-resolved | Identity anchors, vetoes |

**Ternary attention on read:** `score = key_overlap×0.35 + value_overlap×0.55 + trit_bias×0.10`

Memory is stored server-side keyed to your API key — no state blob to pass between calls.

```json
{
  "mcpServers": {
    "ternlang": {
      "url": "https://ternlang.com/mcp"
    }
  }
}
```

For local stdio transport (Claude Desktop, offline use):
```json
{
  "mcpServers": {
    "ternlang": {
      "command": "/path/to/ternlang-mcp",
      "args": []
    }
  }
}
```

---

## Example Library

**300+ `.tern` programs** covering real-world decision logic across every domain — the largest collection of balanced ternary programs in existence.

| Category | Examples |
|----------|---------|
| [Aerospace & Safety](examples/03_rocket_launch.tern) | Rocket launch, aircraft deicing, runway incursion, satellite collision |
| [Medicine](examples/05_medical_triage.tern) | ER triage, ICU ventilator, sepsis warning, organ transplant, APGAR |
| [Finance](examples/42_algorithmic_trading.tern) | Algorithmic trading, AML filter, options expiry, loan underwriting |
| [Infrastructure](examples/14_circuit_breaker.tern) | Circuit breaker, nuclear reactor SCRAM, bridge health, power grid |
| [AI Agents](examples/08_evidence_collector.tern) | Evidence density, confidence escalation, MoE routing, deliberation |
| [Civic Systems](examples/12_vote_aggregator.tern) | Vote aggregation, bail decision, treaty negotiation, refugee status |
| [Computer Science](examples/09_risc_fetch_decode.tern) | CPU pipeline, cache invalidation, API rate limiting, deployment gate |
| [Tutorials](stdlib/tutorials/) | 15 step-by-step tutorials — hello ternary → full ML pipeline |
| [QNN / Qutrit](stdlib/qnn/) | Qutrit Neural Networks — Kepp 2026 reference implementations |
| [Standard Library](stdlib/) | Agents, reasoning, ML layers, optimizers, std, benchmarks |

→ [**Browse all examples**](examples/INDEX.md)

---

## Architecture

| Crate | Tier | Description |
|-------|------|-------------|
| [`ternlang-core`](ternlang-core/) | Open (LGPL) | Lexer, parser, AST, BET VM — 51 opcodes, 27 registers |
| [`ternlang-cli`](ternlang-cli/) | Open (LGPL) | `run` · `build` · `sim` · `fmt` · `repl` · `compat` |
| [`ternlang-lsp`](ternlang-lsp/) | Open (LGPL) | LSP 3.17 — hover, completion, diagnostics |
| [`ternlang-compat`](ternlang-compat/) | Open (LGPL) | 9-trit RISC assembler (Brandon Smith bridge), Owlet S-expr parser |
| [`ternpkg`](ternpkg/) | Open (LGPL) | Package manager, GitHub-backed registry |
| [`ternlang-ml`](ternlang-ml/) | BSL-1.1 | Sparse matmul, BitNet quantization, TernaryMLP, deliberation engine, coalition vote, action gate |
| [`ternlang-moe`](ternlang-moe/) | BSL-1.1 | MoE-13 orchestrator — dual-key routing, triad synthesis, 3-tier memory, AgentHarness |
| [`ternlang-api`](ternlang-api/) | BSL-1.1 | REST + SSE API, multi-tenant key management, all reasoning endpoints |
| [`ternlang-mcp`](ternlang-mcp/) | BSL-1.1 | MCP server — 20 tools (10 free + 10 premium), stdio + HTTP transport, server-side 3-layer memory |
| [`ternlang-mkl`](ternlang-mkl/) | BSL-1.1 | **cuTern**: Math Kernel Library with native sparsity bypass |
| [`ternlang-sql`](ternlang-sql/) | BSL-1.1 | Native Ternary Graph Database driver (50% speedup) |
| [`ternlang-bridge`](ternlang-bridge/) | BSL-1.1 | Binary-to-Ternary Transpiler (The "Trojan Horse") |
| [`ternlang-net`](ternlang-net/) | BSL-1.1 | Triadic Networking Stack (Introspective Handshake) |
| [`ternlang-crypto`](ternlang-crypto/) | BSL-1.1 | High-entropy Trit-based Cryptographic primitives |
| [`ternlang-fs`](ternlang-fs/) | BSL-1.1 | Triadic File System (State 0 Transactional Pend) |
| [`ternlang-hdl`](ternlang-hdl/) | BSL-1.1 | Verilog-2001 codegen, BET processor, FPGA simulation |
| [`ternlang-runtime`](ternlang-runtime/) | BSL-1.1 | Distributed TCP actor runtime |
| [`ternlang-qutrit`](ternlang-qutrit/) | BSL-1.1 | Quantum-Classical Bridge (Qutrit Native Superposition) |
| [`ternlang-consensus`](ternlang-consensus/) | BSL-1.1 | Triadic Byzantine Fault Tolerance (TBFT) |
| [`ternlang-ui`](ternlang-ui/) | BSL-1.1 | Triadic State Management & DOM Rendering |
| [`ternlang-bci`](ternlang-bci/) | BSL-1.1 | Brain-Computer Interface (Native Inhibitory Decoding) |
| [`ternlang-astro`](ternlang-astro/) | BSL-1.1 | Interplanetary Delay-Tolerant Networking (DTN) |
| [`ternlang-swarm`](ternlang-swarm/) | BSL-1.1 | Triadic Kinematics (Biological Hesitation for Robotics) |
| [`ternlang-tson`](ternlang-tson/) | BSL-1.1 | **TSON**: Ternary Standard Object Notation (30% denser than JSON) |
| [`ternlang-ttp`](ternlang-ttp/) | BSL-1.1 | **TTP**: Triadic Transfer Protocol (Status 000: Deliberating) |
| [`ternlang-posix`](ternlang-posix/) | BSL-1.1 | **T-POSIX**: Triadic Operating System Interface |
| [`ternlang-time`](ternlang-time/) | BSL-1.1 | **T-NTP**: Triadic Network Time Protocol (Temporal Hold) |
| [`ternlang-auth`](ternlang-auth/) | BSL-1.1 | **T-DID**: Triadic Decentralized Identity (Provisional Auth) |
| [`ternlang-gfx`](ternlang-gfx/) | BSL-1.1 | **T-GPU**: Triadic Graphics Pipeline (Depth-as-a-Trit) |
| [`ternlang-contract`](ternlang-contract/) | BSL-1.1 | **T-Contract**: Triadic Smart Contracts (Arbitration State) |

**212+ tests · All passing · v0.3.0**

---

## Strategic Standards

RFI-IRFOS is establishing the total regulatory and technical moat for the post-binary era. These specifications are designed to mandate Ternary compliance across global hardware, software, and AI industries.

### Core Architecture & Logic
- [**BET-ISA v1.0**](spec/standards/BET-ISA-v1.0.md): The definitive 9-Trit RISC Instruction Set Architecture.
- [**IEEE TFP-754**](spec/standards/IEEE-TFP-754.md): Global standard for Ternary Floating-Point Arithmetic.
- [**ISO Certified Uncertainty**](spec/standards/ISO-CERTIFIED-UNCERTAINTY.md): Regulatory framework mandating State 0 abstention.
- [**TSON v1.0**](spec/standards/TSON-v1.0.md): Optimized data serialization for triadic systems.
- [**T-POSIX v1.0**](spec/standards/T-POSIX-v1.0.md): Operating system interface redefining process state logic.

### AI, Memory & Intelligence
- [**T-TOKEN v1.0**](spec/standards/T-TOKEN-v1.0.md): Trit-Pair Encoding (TPE) — Compressing semantic entropy by 33%.
- [**T-KV-CACHE v1.0**](spec/standards/T-KV-CACHE-v1.0.md): The Memory Moat — Eliminating 60% of zero-signal KV allocations.
- [**T-WEIGHT v1.0**](spec/standards/T-WEIGHT-v1.0.md): Triadic Weight Exchange — Mandatory safety headers for LLM weights.
- [**T-EXPLAIN v1.0**](spec/standards/T-EXPLAIN-v1.0.md): Triadic Traceability — Standardizing MoE-13 deliberation logs.
- [**T-HALO v1.0**](spec/standards/T-HALO-v1.0.md): Triadic Alignment & Governance — Hardware-locked safety holds.
- [**TUANN v1.0**](stdlib/ml/tuann.tern): Triadic Uncertainty-Aware Neural Networks — Native State 0 for hallucination rejection.

### Hardware, Physical & Frontier
- [**T-HAL v1.0**](spec/standards/T-HAL-v1.0.md): Universal Hardware Abstraction — The bridge for Huawei/NVIDIA/FPGA.
- [**T-SENSE v1.0**](spec/standards/T-SENSE-v1.0.md): Ternary Sensor Fusion — Triadic Delta fields for IoT efficiency.
- [**T-THERMAL v1.0**](spec/standards/T-THERMAL-v1.0.md): Power-Aware Compute — Dynamic clock scaling via sparsity.
- [**T-BIO v1.0**](spec/standards/T-BIO-v1.0.md): Triadic Neural Encoding — 1:1 parity for Brain-Computer Interfaces.
- [**T-QUT v1.0**](spec/standards/T-QUT-v1.0.md): Qutrit Bridge Standard — Positioning TIS as the OS for Quantum.

### Network, Data & Economy
- [**TTP v1.0**](spec/standards/TTP-v1.0.md): Web transfer protocol eliminating binary timeouts.
- [**T-NET v1.0**](spec/standards/T-NET-v1.0.md): Triadic Networking — Deliberative headers for intelligent packet routing.
- [**T-RPC v1.0**](spec/standards/T-RPC-v1.0.md): Remote Procedure Call with native deliberative waiting.
- [**T-ROUTING v1.0**](spec/standards/T-ROUTING-v1.0.md): Sparse Packet Switching — Skipping State 0 network congestion.
- [**T-SQL v1.0**](spec/standards/T-SQL-v1.0.md): Triadic Query Language — Redefining search via T-Trees.
- [**T-ARCHIVE v1.0**](spec/standards/T-ARCHIVE-v1.0.md): Triadic Cold Storage — Neutral State structural stability.
- [**T-Fi v1.0**](spec/standards/T-Fi-v1.0.md): Triadic Compute Currency — Standardizing the TaaS cryptographic toll.
- [**T-TAX v1.0**](spec/standards/T-TAX-v1.0.md): Automatic Compute Dividends — Decentralized royalty routing.
- [**T-GENESIS v1.0**](spec/standards/GENESIS-ANCHOR.md): The Triadic Trust Anchor. Mandates global MoE-13 consensus validation to prevent logic drift.

---

## Licensing Tiers

### Pro Standard Library (Tier 2)
Optimized for startups and small teams. Includes 10,000 API requests/month and 122x Sparse Bypass acceleration. **€24.99/mo**.

### Industrial Standard Library (Tier 3)
The full RFI-IRFOS ecosystem. Includes 20,000 API requests/month, QNN, SEC, and T-HAL silicon bindings. **€49.99/mo**.

```
┌─────────────────────────────────────────────────────────────────┐
│  TIER 1 — Open Core (LGPL-3.0)                                  │
│  ternlang-core · ternlang-cli · ternlang-lsp · ternlang-compat  │
│  ternpkg · spec/                                                 │
│  Free to use, modify, and distribute. Modifications must be     │
│  contributed back under LGPL.                                   │
├─────────────────────────────────────────────────────────────────┤
│  TIER 2 — Restricted API (BSL 1.1)                              │
│  €24.99 / MONTH                                                 │
│  ★ 20 MCP tools (full stack)                                    │
│  ✓ Server-side 3-layer memory                                   │
│  ✓ MoE-13 consolidation                                         │
│  ✓ Ternary context compression                                  │
│  ✓ 10,000 high-mass API calls                                   │
│  Commercial use requires a license → licensing@ternlang.com     │
│  Auto-converts to Apache-2.0 on 2030-04-03.                     │
├─────────────────────────────────────────────────────────────────┤
│  TIER 3 — Enterprise (ternlang.com)                             │
│  Hosted API · Enterprise SLA · Commercial inference engine      │
│  On-premise node clusters · Custom FPGA implementations         │
│  Contact: licensing@ternlang.com                                │
└─────────────────────────────────────────────────────────────────┘
```

> **ML Training Restriction:** The contents of this repository may not be used to train, fine-tune, or distill machine learning models without explicit written permission from RFI-IRFOS. See [LICENSE-ML-TRAINING](LICENSE-ML-TRAINING).

---

## Ecosystem Position

Ternlang is designed to be the convergence point for the fragmented ternary computing field.

| Project | Bridge / Replacement |
|---------|----------------------|
| [JSON](https://www.json.org/) | `TSON` in `ternlang-tson` — eliminates `null` lossiness, 30% denser |
| [HTTP/TCP](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol) | `TTP` in `ternlang-ttp` — eliminates timeouts via Status 000 |
| [POSIX / Unix](https://en.wikipedia.org/wiki/POSIX) | `T-POSIX` in `ternlang-posix` — redefines process exits as triadic signals |
| [Neuralink](https://www.neuralink.com/) | `BCI` in `ternlang-bci` — decodes active inhibition POTENTIAL natively |
| [BitNet b1.58](https://arxiv.org/abs/2402.17764) | `TSPARSE_MATMUL` — hardware-level sparsity bypass for ternary weights |
| [Brandon Smith 9-trit](https://github.com/brandon-smith-187) | `TasmAssembler` — assembles `.tasm` → BET bytecode |
| [Owlet S-expression](https://github.com/owlet-lang) | `OwletParser` — S-expr front-end → ternlang AST |

→ [**Full ecosystem map**](TERNARY-ECOSYSTEM.md)

---

## Whitepaper & Specs

- [ternlang-whitepaper.tex](whitepaper/ternlang-whitepaper.tex) — IEEE two-column, arXiv-ready (cs.PL / cs.AR / cs.NE)
- [BET-ISA-SPEC.md](BET-ISA-SPEC.md) — formal ISA specification with encoding tables and stack-effect notation
- [spec/grammar.ebnf](spec/grammar.ebnf) — language grammar
- [spec/ternlang-language-reference-v0.1.md](spec/ternlang-language-reference-v0.1.md) — language reference

```bibtex
@misc{kepp2026ternlang,
  author  = {Kepp, Simeon},
  title   = {Ternlang: Balanced Ternary Intelligence Stack},
  year    = {2026},
  url     = {https://ternlang.com},
  doi     = {10.17605/OSF.IO/TZ7DC}
}
```
*.tern linguist-language=Ternlang
---

## Contact & Licensing

| | |
|---|---|
| **Website** | [ternlang.com](https://ternlang.com) |
| **Commercial licensing** | [licensing@ternlang.com](mailto:licensing@ternlang.com) |
| **Academic collaboration** | Open — cite the whitepaper |
| **API access** | [ternlang.com/#licensing](https://ternlang.com/#licensing) |

*"The place where fragmented ternary efforts compile into one."*
licensing](https://ternlang.com/#licensing) |

*"The place where fragmented ternary efforts compile into one."*
