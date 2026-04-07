[![License: LGPL-3.0](https://img.shields.io/badge/License-LGPL_3.0-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0.html) [![OSF: DOI](https://img.shields.io/badge/OSF-DOI_10.17605-brightgreen.svg)](https://osf.io) [![Linguist Status: Ternlang](https://img.shields.io/badge/Linguist-Ternlang-4A90E2.svg)](#)

# Ternary Intelligence Stack (TIS)

**The Institutional Standard for Post-Binary Computing and Triadic Reasoning.**

[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![License](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1%20%2F%20Proprietary-blue)](ternlang-root/LICENSE)
[![Tests](https://img.shields.io/badge/tests-212%2B%20passing-brightgreen)](ternlang-root/ROADMAP.md)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![Tether](https://img.shields.io/badge/Genesis_Tether-Active-red)](#)

Built by [RFI-IRFOS](https://ternlang.com) · [ternlang.com](https://ternlang.com)

---

Binary systems treat uncertainty as null. Ternlang treats it as a **first-class hardware state**.

Every value in the stack is a *trit* — one of three:

```
-1  →  VETO      Hard rejection. unrecoverable security panic.
 0  →  TEND      Deliberation. Insufficient data. Enter hardware equilibrium.
+1  →  AFFIRM    Authorization. Proceed with execution.
```

The **TEND (0)** state is the core innovation of the **BET-VM**. It is not "undecided"; it is a formal signal that tells the system to remain in deliberation until evidence is sufficient. This eliminates "hallucination-by-coercion" in AI, sparse neural inference where zero-weights are skipped at the instruction level, and safety-critical systems where a premature decision is worse than no decision.

---

## What's in This Repository

```
ternlang-root/        Language, VM, inference engine, API, MCP server
agent-albert/         Local AI node built on the Ternary Intelligence Stack
ternlang-vscode/      VS Code extension (.tern syntax highlighting + LSP)
ternlang-root/spec/   Institutional standards (BET-ISA, IEEE TFP-754, etc.)
```

→ **[Full technical documentation](ternlang-root/README.md)**
→ **[Development roadmap](ternlang-root/ROADMAP.md)**
→ **[250+ .tern example programs](ternlang-root/examples/INDEX.md)**

---

##  Ecosystem Architecture

The TIS is partitioned into three tiers to ensure global institutional stability and vendor security:

| Tier | License | Scope | Access |
|------|---------|-------|--------|
| **Tier 1: Open Core** | LGPL-3.0 | Language, VM, Parser, CLI, Package Manager | Open Source |
| **Tier 2: Restricted API** | BSL-1.1 | MoE-13, Sparse ML, cuTern, Networking, Crypto | **€24.99/mo** |
| **Tier 3: Enterprise** | Proprietary | T-GPU, T-BCI, T-Astro, Genesis Hardware Key | **Contact RFI** |

> **Security Note:** All VM execution is structurally bound to the **RFI-IRFOS Triadic Genesis Tether**. Unauthorized execution of restricted opcodes forces a permanent hardware `THOLD` (State 0).

---

## The Stack at a Glance

| Layer | What it does |
|-------|-------------|
| **Language** | `.tern` programs compile to BET bytecode and run on the BET VM — 51 opcodes, 27 registers, exhaustive 3-way match enforcement |
| **Sparse Inference** | `@sparseskip` routes `matmul()` to `TSPARSE_MATMUL` — zero-weight elements skipped at the instruction level. **86–122× faster** than dense float32 |
| **MoE-13 Orchestrator** | Mixture-of-Experts reasoning engine: 13 domain experts, dual-key synergistic routing, 1+1=3 emergent triad synthesis, safety hard gate |
| **Reasoning Toolkit** | Deliberation engine (EMA convergence), coalition vote, action gate (hard-block safety veto), scalar temperature, hallucination score |
| **Live API** | REST + SSE + MCP endpoints at `https://ternlang-api.fly.dev` — deployed on Fly.io |
| **MCP Server** | 20 tools via HTTP or stdio — any MCP client becomes a ternary decision engine |

---

## MoE-13 Ternary Orchestrator

The flagship reasoning component. Based on prior research ([DOI: 10.17605/OSF.IO/TZ7DC](https://doi.org/10.17605/OSF.IO/TZ7DC)).

```rust
use ternlang_moe::TernMoeOrchestrator;

let mut orch = TernMoeOrchestrator::with_standard_experts();
let result = orch.orchestrate("Should I proceed?", &[0.6, 0.7, 0.8, 0.5, 0.4, 0.9]);

// trit=1 conf=84% held=false
// "Affirm with confidence 84%. Emergent field amplifying."
```

Routes through 13 specialists: Syntax · WorldKnowledge · DeductiveReason · InductiveReason · ToolUse · Persona · Safety · FactCheck · CausalReason · AmbiguityRes · MathReason · ContextMem · MetaSafety.

---

## Live API

```bash
# Health check
curl https://ternlang-api.fly.dev/health

# MCP — Compatible with Claude Desktop / Cursor
curl -X POST https://ternlang-api.fly.dev/mcp \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/call",
       "params":{"name":"trit_decide","arguments":{"evidence":[0.8,0.6,-0.2,0.9]}}}'
```

**MCP server:** `https://ternlang-api.fly.dev/mcp`

```json
{ "mcpServers": { "ternlang": { "url": "https://ternlang-api.fly.dev/mcp" } } }
```

---

## Sparse Inference Benchmark

| Sparsity | 128² | 256² | 512² |
|----------|------|------|------|
| 40% | 29.6× | 46.0× | 73.6× |
| **60%** | **27.9×** | **32.1×** | **86.1×** |
| 99% | 13.1× | 53.9× | **122.3×** |

40–60% sparsity is exactly where BitNet b1.58 quantization (`tau = 0.5 × mean(|w|)`) places weights in trained language models. The kernel and the quantization scheme are structurally aligned.

---

## Agent Albert

[agent-albert/](agent-albert/) is a sovereign, offline-first local AI node built on top of the Ternary Intelligence Stack. It uses the BET VM and MoE-13 orchestrator as its native reasoning layer — every decision is evaluated through the `{-1, 0, +1}` state space.

---

## Architecture

| Crate | Tier | Description |
|-------|------|-------------|
| [`ternlang-core`](ternlang-root/ternlang-core/) | Tier 1 | Lexer, parser, AST, BET VM — 51 opcodes, 27 registers |
| [`ternlang-cli`](ternlang-root/ternlang-cli/) | Tier 1 | `run` · `build` · `sim` · `fmt` · `repl` · `compat` |
| [`ternlang-lsp`](ternlang-root/ternlang-lsp/) | Tier 1 | LSP 3.17 — hover, completion, diagnostics |
| [`ternlang-compat`](ternlang-root/ternlang-compat/) | Tier 1 | 9-trit RISC assembler (Brandon Smith bridge), Owlet S-expr parser |
| [`ternpkg`](ternlang-root/ternpkg/) | Tier 1 | Package manager, GitHub-backed registry |
| [`ternlang-ml`](ternlang-root/ternlang-ml/) | Tier 2 | Sparse matmul, BitNet quantization, deliberation engine, coalition vote, action gate |
| [`ternlang-moe`](ternlang-root/ternlang-moe/) | Tier 2 | MoE-13 orchestrator — dual-key routing, triad synthesis, 3-layer memory |
| [`ternlang-api`](ternlang-root/ternlang-api/) | Tier 2 | REST + SSE API, multi-tenant key management |
| [`ternlang-mcp`](ternlang-root/ternlang-mcp/) | Tier 2 | MCP server — 20 tools, stdio + HTTP transport |
| [`ternlang-mkl`](ternlang-root/ternlang-mkl/) | Tier 2 | **cuTern**: Math Kernel Library with native sparsity bypass |
| [`ternlang-sql`](ternlang-root/ternlang-sql/) | Tier 2 | Native Ternary Graph Database driver |
| [`ternlang-bridge`](ternlang-root/ternlang-bridge/) | Tier 2 | Binary-to-Ternary Transpiler (The "Trojan Horse") |
| [`ternlang-net`](ternlang-root/ternlang-net/) | Tier 2 | Triadic Networking Stack (Introspective Handshake) |
| [`ternlang-crypto`](ternlang-root/ternlang-crypto/) | Tier 2 | High-entropy Trit-based Cryptographic primitives |
| [`ternlang-fs`](ternlang-root/ternlang-fs/) | Tier 2 | Triadic File System (State 0 Transactional Pend) |
| [`ternlang-hdl`](ternlang-root/ternlang-hdl/) | Tier 2 | Verilog-2001 codegen, BET processor, FPGA simulation |
| [`ternlang-runtime`](ternlang-root/ternlang-runtime/) | Tier 2 | Distributed TCP actor runtime |
| [`ternlang-qutrit`](ternlang-root/ternlang-qutrit/) | Tier 2 | Quantum-Classical Bridge (Qutrit Native Superposition) |
| [`ternlang-consensus`](ternlang-root/ternlang-consensus/) | Tier 2 | Triadic Byzantine Fault Tolerance (TBFT) |
| [`ternlang-ui`](ternlang-root/ternlang-ui/) | Tier 2 | Triadic State Management & DOM Rendering |
| [`ternlang-tson`](ternlang-root/ternlang-tson/) | Tier 2 | **TSON**: Ternary Standard Object Notation |
| [`ternlang-ttp`](ternlang-root/ternlang-ttp/) | Tier 2 | **TTP**: Triadic Transfer Protocol |
| [`ternlang-posix`](ternlang-root/ternlang-posix/) | Tier 2 | **T-POSIX**: Triadic Operating System Interface |
| [`ternlang-bci`](ternlang-root/ternlang-bci/) | Tier 3 | Brain-Computer Interface (Native Inhibitory Decoding) |
| [`ternlang-astro`](ternlang-root/ternlang-astro/) | Tier 3 | Interplanetary Delay-Tolerant Networking (DTN) |
| [`ternlang-swarm`](ternlang-root/ternlang-swarm/) | Tier 3 | Triadic Kinematics (Biological Hesitation for Robotics) |
| [`ternlang-bio`](ternlang-root/ternlang-bio/) | Tier 3 | Triadic Genomic Sequencing (Native Epigenetic Methylation) |
| [`ternlang-sec`](ternlang-root/ternlang-sec/) | Tier 3 | Post-Quantum Cryptography (Native Triadic Lattices) |
| [`ternlang-grid`](ternlang-root/ternlang-grid/) | Tier 3 | Triadic Energy Distribution (State 0 Phase-Hold) |
| [`ternlang-cad`](ternlang-root/ternlang-cad/) | Tier 3 | Topology Optimization (Triadic Metamaterials) |
| [`ternlang-harmony`](ternlang-root/ternlang-harmony/) | Tier 2 | Harmony OS NDK Bindings (The Abstraction Trap) |
| [`ternlang-driver`](ternlang-root/ternlang-driver/) | Tier 2 | Universal Hardware Abstraction Layer (HAL) |
| [`ternlang-edu`](ternlang-root/ternlang-edu/) | Tier 1 | "The Education Cartel" (Standardized Curriculum Tools) |

---

## 📜 Strategic Standards (Published)

RFI-IRFOS is standardizing the triadic era. These are the definitive specifications:

- [**BET-ISA v1.0**](ternlang-root/spec/standards/BET-ISA-v1.0.md): The definitive 9-Trit RISC Architecture.
- [**IEEE TFP-754**](ternlang-root/spec/standards/IEEE-TFP-754.md): Global Ternary Floating-Point Standard.
- [**ISO Certified Uncertainty**](ternlang-root/spec/standards/ISO-CERTIFIED-UNCERTAINTY.md): Mandatory State 0 for safety-critical AI.
- [**TSON**](ternlang-root/spec/standards/TSON-v1.0.md): 30% denser data serialization than JSON.
- [**TTP**](ternlang-root/spec/standards/TTP-v1.0.md): Replaces HTTP Status 200/400 with 000 (Deliberating).
- [**T-POSIX**](ternlang-root/spec/standards/T-POSIX-v1.0.md): Triadic process signals and scheduler logic.
- [**T-GENESIS**](ternlang-root/spec/standards/GENESIS-ANCHOR.md): The cryptographic root of trust and Fly.io tether.
- [**T-BIO v1.0**](ternlang-root/spec/standards/T-BIO-v1.0.md): Triadic Genomic Sequencing Standard.
- [**T-SEC v1.0**](ternlang-root/spec/standards/T-SEC-v1.0.md): Triadic Post-Quantum Cryptography (T-PQKE).
- [**T-GRID v1.0**](ternlang-root/spec/standards/T-GRID-v1.0.md): Triadic Energy Distribution Standard.
- [**T-CAD v1.0**](ternlang-root/spec/standards/T-CAD-v1.0.md): Triadic Topology Optimization Standard.
- [**T-TriLM v1.0**](ternlang-root/spec/standards/T-TriLM-v1.0.md): Language Model Interop & Audit Standard.
- [**T-FPGA v1.0**](ternlang-root/spec/standards/T-FPGA-v1.0.md): Legacy Hardware Subordinate Compliance.
- [**T-HARMONY v1.0**](ternlang-root/spec/standards/T-HARMONY-v1.0.md): Edge Device Abstraction Standard.
- [**T-DRIVER v1.0**](ternlang-root/spec/standards/T-DRIVER-v1.0.md): Universal Hardware Abstraction Standard.
- [**ISO/IEC TIS-9000**](ternlang-root/spec/standards/ISO-TIS-9000.md): Triadic Computing Systems Certification & Benchmarks.

---

## 🤝 Help Ternlang Get Recognized

We are currently working toward official recognition by **GitHub Linguist** (Issue [#7904](https://github.com/github-linguist/linguist/issues/7904)). To meet their inclusion criteria, we need **2000 unique `.tern` files** across the GitHub ecosystem.

**How you can help:**
1. Create a `.tern` logic file in your own public repository.
2. Share your triadic algorithms with the community.
3. Track our progress: [extension:tern NOT is:fork](https://github.com/search?q=extension%3Atern+NOT+is%3Afork&type=code)

---

## License

Open core under **LGPL-3.0**. Restricted components under **BSL-1.1**.
Commercial licensing: [licensing@ternlang.com](mailto:licensing@ternlang.com)

> The contents of this repository may not be used to train, fine-tune, or distill machine learning models without explicit written permission from RFI-IRFOS.

---

*Obm. Simeon Kepp · [Research Focus Institute – Interdisciplinary Research Facility for Open Sciences](https://ternlang.com)*  
*ZVR: 1015608684 · Elisabethinergasse 25, 8020 Graz, Austria*
<!-- Index Nudge: Tue Apr  7 10:10:45 UTC 2026 -->
