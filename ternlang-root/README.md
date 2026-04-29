# Ternlang — Balanced Ternary Intelligence Stack

[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![version](https://img.shields.io/badge/version-v0.3.3-black)](#architecture)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-88%2B%20passing-brightgreen)](#architecture)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![EU AI Act](https://img.shields.io/badge/EU%20AI%20Act-Article%2013,14+15%20Compliant%20Design-003399?logo=european-union)](https://ternlang.com/compliance)
[![speedup](https://img.shields.io/badge/@sparseskip-up_to_122x-success)](#sparse-ternary-inference)
[![MCP](https://img.shields.io/badge/MCP-30_tools-orange)](#mcp-server--v030-19-tools)
[![smithery badge](https://smithery.ai/badge/rfi-irfos/ternlang)](https://smithery.ai/servers/rfi-irfos/ternlang)
[![examples](https://img.shields.io/badge/examples+2k%2B_.tern_programs-blueviolet)](#example-library)
[![stdlib](https://img.shields.io/badge/stdlib-293_open%20%2B%2028k%2B_premium-blue)](stdlib/PREMIUM.md)
[![DOI](https://img.shields.io/badge/DOI-10.17605%2FOSF.IO%2FTZ7DC-informational)](https://doi.org/10.17605/OSF.IO/TZ7DC)


Ternlang is a systems programming language and ML inference runtime built on balanced ternary logic. It is the foundational layer of a post-binary technology ecosystem designed to secure European technological sovereignty.

Built by [RFI-IRFOS](https://ternlang.com) · Graz, Austria · Whitepaper [https://osf.io/cyn28/files/8hzux]

---

## Our Mission: Post-Binary Sovereignty

We are a core team of three co-founders overseeing a international Team of 10 Members from our base in Graz, Austria, working at the intersection of computer science, machine learning, and international relations. We believe that Europe's path to technological sovereignty lies in leapfrogging the binary limitations of current AI hyperscalers.

**Ternlang adds the third state.** By representing uncertainty as a first-class citizen, we enable AI systems that reason with causal transparency and ontological integrity.

**[Meet the Team in LEADERSHIP.md](../LEADERSHIP.md)**

Every AI system today is forced to answer yes or no — even when the evidence is contradictory, incomplete, or genuinely uncertain. Binary logic has no formal representation for *"I don't know yet."* Systems either make a confident inference or return null.

Ternlang adds the third state.

| Trit | Name | What it means |
|------|------|---------------|
| `−1` | **reject** | Clear negative signal. Do not proceed. |
| ` 0` | **tend** | Insufficient data. Gather more before acting. |
| `+1` | **affirm** | Clear positive signal. Proceed. |

## The `tend` state is not indecision. It is a **first-class routing instruction** — a computational directive to remain in deliberation until evidence crosses a threshold. This makes ternlang the natural foundation for AI agents that must reason honestly under uncertainty.
---
→ **[ROADMAP.md](ROADMAP.md)** — Phases 1–18, session log, priority matrix

→ **[Ternlang Studio IDE Preview](https://ternlang-api.fly.dev/studio)** — Our work-in-progress developer dash


## What's in This Repository

| Layer | What it does |
|-------|-------------|
| [Language & VM](#language--vm) | Compile and run `.tern` programs on the Balanced Ternary Execution VM |
| [Sparse Inference](#sparse-ternary-inference) | @sparseskip: 2.3× measured baseline, scales to 122× at extreme sparsity |
| [MoE-13 Orchestrator](#moe-13-ternary-orchestrator) | Mixture-of-Experts reasoning engine with safety hard gate |
| [Protocol Specifications](#rfi-irfos-protocol-specifications) | BET-ISA, TSON, TTP, and T-POSIX — RFI-IRFOS open proposals |
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

**Standard Library:** 293 open-core modules in this repo · 28,495+ proprietary modules across Tier 2/3/4 in the [private premium repo](stdlib/PREMIUM.md) — including `nn::*`, `nlp::*`, `vision::*`, `rl::*`, `stats::*`, `bio::*`, `crypto::*`, `finance::*`, and research-grade `qnn::*`.

**Compiler Features:** First-class `affirm/tend/reject` keywords · Binary `if/while` fallbacks · Tensor indexing `obj[r,c]` · Built-in `use` resolver with zero runtime I/O.

**Quick start — install the CLI:**

```bash
cargo install ternlang-cli
```

That installs the `ternlang` binary. Then:

```bash
ternlang                        # launch interactive REPL immediately
ternlang my_program.tern        # run a .tern file directly — no subcommand needed
ternlang run my_program.tern    # same as above (explicit form)
```

**All commands:**

```bash
ternlang                                          # → interactive REPL
ternlang <file.tern>                              # → run file directly
ternlang run <file.tern>                          # → run file
ternlang build <file.tern> [--output file.bet]   # → compile to bytecode
ternlang repl                                     # → interactive REPL
ternlang fmt <file.tern> [--write]               # → format source
ternlang test [path]                              # → run test suite
ternlang audit decisions.json [--html]           # → EU AI Act audit report
ternlang translate my_logic.py [--language python] [--output result.tern]
```

**Or build from source:**

```bash
git clone https://github.com/eriirfos-eng/ternary-intelligence-stack
cd ternary-intelligence-stack/ternlang-root
cargo build --release
./target/release/ternlang my_program.tern   # or: ./target/release/ternlang run ...
```

---

## Jupyter Kernel

[![PyPI](https://img.shields.io/pypi/v/ternlang-jupyter?color=blue&logo=python&logoColor=white)](https://pypi.org/project/ternlang-jupyter/)

Run `.tern` programs directly in Jupyter notebooks — JupyterLab, Jupyter Notebook, VS Code notebooks, or any environment that speaks the Jupyter protocol.

```bash
pip install ternlang-jupyter
ternlang-jupyter-install
```

Then select **Ternlang (BET VM)** from the kernel menu. Each cell is a complete `.tern` program:

```ternlang
fn main() -> trit {
    print("uncertainty quantification, natively");
    let confidence: trit = hold;   // 0 — insufficient evidence
    return confidence;
}
```

Output streams `print()` calls live and renders a color-coded result block:
- **AFFIRM +1** — green
- **HOLD 0** — amber
- **REJECT −1** — red

Includes tab completion, hover docs for all builtins, and `%version` / `%help` magic commands.

---

## VS Code Extension

[![Open VSX](https://img.shields.io/badge/Open%20VSX-rfi--irfos.ternlang%20v0.4.0-blue?logo=visualstudiocode)](https://open-vsx.org/extension/rfi-irfos/ternlang)

Install from the [Open VSX Registry](https://open-vsx.org/extension/rfi-irfos/ternlang) (works in VS Code, VSCodium, Gitpod, and any Open VSX-compatible editor):

```
ext install rfi-irfos.ternlang
```

Or install the VSIX directly:

```bash
code --install-extension ternlang-vscode/ternlang-0.4.0.vsix
```

**What you get:**

| Feature | Details |
|---------|---------|
| Syntax highlighting | All keywords, types, trit literals (`affirm`/`tend`/`reject`), operators (`&&` `\|\|` `<=` `>=`), `@sparseskip` directive |
| `.tern` file association | Opens as Ternlang automatically |
| LSP diagnostics | Hover, completions, and error underlining via `ternlang-lsp` |
| Inline trit value hints | Ghost decorations on `let` bindings show current trit state while editing |
| Language configuration | Auto-close brackets/braces, comment toggle (`Ctrl+/`) |

**LSP setup** — the language server binary is bundled inside the extension and auto-downloaded on install. No manual build step is required. Diagnostics activate automatically when you open any `.tern` file.

> **Published on Open VSX** — multiple downloads live. Works in VS Code, VSCodium, Cursor, and any Open VSX-compatible editor.

---

## Agent Albert — AI Intelligence Layer

[![crates.io](https://img.shields.io/crates/v/albert-cli.svg)](https://crates.io/crates/albert-cli)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](../agent_albert_cli/rust/LICENSE)

Albert is the sovereign, model-agnostic AI coding CLI built as the intelligence layer of the Ternary Intelligence Stack. He runs as a standalone terminal agent or embedded directly inside TernStudio — wired into the flow canvas to generate, debug, and explain ternary workflows.

### Two modes

**Mode 1 — Standalone coding CLI**

```bash
cargo install albert-cli    # installs the `albert` binary
albert                      # interactive REPL
albert "refactor this"      # one-shot prompt
```

Albert boots with a one-time interview (name, role, cognitive archetype) and auto-generates an `ALBERT.md` in your workspace with baked-in project context. Every subsequent session starts with full memory of who you are and what your codebase does.

**Mode 2 — Embedded in TernStudio** *(in development)*

Press `F6` anywhere on the TernStudio flow canvas to summon Albert. From there you can:

- Describe a workflow in plain language → Albert generates the full node graph
- Select a node or wire → ask Albert to explain the signal path or debug the logic
- Type `/plan` → Albert proposes an execution strategy before touching the canvas

This makes Albert the intelligence behind every workflow — not just a chatbot bolted on the side.

---

### Model-agnostic — bring your own LLM

Albert dispatches to whichever provider you configure. No default billing, no vendor lock-in.

| Provider | Environment variable | Notes |
|----------|---------------------|-------|
| Google Gemini | `GEMINI_API_KEY` | Default model: `gemini-2.0-flash` |
| Anthropic Claude | `ANTHROPIC_API_KEY` | Any Claude model |
| OpenAI / GPT | `OPENAI_API_KEY` | GPT-4o and others |
| XAI / Grok | `XAI_API_KEY` | Grok-2 and Grok-3 |
| Ollama (local) | *(none needed)* | `ollama serve` — fully air-gapped |
| HuggingFace | `HF_API_KEY` | Any HF inference endpoint |
| Azure OpenAI | `AZURE_OPENAI_API_KEY` | Enterprise deployments |

Keys are stored in `~/.config/albert/secrets.json` — never sent anywhere except directly to your chosen provider.

---

### Slash command library

| Command | What it does |
|---------|-------------|
| `/plan` | Decompose a task into a structured execution plan before coding |
| `/tdd` | Red-Green-Refactor loop — write the failing test first |
| `/verify` | Run tests and validate correctness, report failures only |
| `/code-review` | Full review pass: correctness, safety, idioms, edge cases |
| `/build-fix` | Compile, read errors, fix, repeat until clean |
| `/refactor` | Targeted cleanup without changing behaviour |
| `/docs` | Generate documentation for the current scope |
| `/loop` | Recursive mission loop — runs up to 10 iterations until `MISSION COMPLETE` |
| `/compress` | Aggressive context compaction — keeps Albert's memory sharp and cheap |

---

### Architecture (5 published crates)

```
albert-cli  (binary: albert)
  ├── albert-api       — multi-provider LLM client, SSE streaming, retry logic
  ├── albert-commands  — slash command library
  ├── albert-compat    — upstream manifest extraction and path resolution
  ├── albert-runtime   — session management, MCP client, OAuth, bash execution,
  │                      file ops, compaction, token usage tracking
  └── albert-tools     — tool dispatch: read, write, edit, bash, glob/grep, MCP
```

All five crates are published on [crates.io](https://crates.io/crates/albert-cli). The workspace lives in [`agent_albert_cli/rust/`](../agent_albert_cli/rust/).

---

### RTK integration — 60–90% token savings

Albert ships with [RTK (Rust Token Killer)](https://www.rtk-ai.app) integrated as a context filter. Every command output is compressed before it reaches the LLM context window — git logs, cargo output, test results, file trees — keeping sessions fast and cheap regardless of which provider you use.

---

### TernStudio integration (roadmap)

Albert will be deployed as a sidecar service alongside the TernStudio API on Fly.io (`ternlang-api.fly.dev`). The Studio front-end will route `F6` prompts to Albert's `/api/albert` endpoint, with the full canvas state serialised as context. The integration surface:

| Studio action | Albert behaviour |
|--------------|-----------------|
| `F6` → free prompt | Generate workflow nodes + wires from description |
| `F6` with node selected | Explain node logic, suggest improvements |
| `F6` with wire selected | Trace signal path, debug confidence values |
| `/plan` in Albert panel | Propose execution strategy for current graph |
| Run simulation → error | Albert auto-diagnoses and suggests fix |

---

### Source

→ [`agent_albert_cli/`](../agent_albert_cli/) — top-level source  
→ [`agent_albert_cli/rust/`](../agent_albert_cli/rust/) — Rust workspace  
→ [crates.io/crates/albert-cli](https://crates.io/crates/albert-cli)

---

## Sparse Ternary Inference

The core performance claim of TIS rests on a single hardware primitive: `@sparseskip` — an opcode that skips computation on zero-state (`tend`) weights entirely.

**Measured baseline (v0.3.0, `ternlang-ml` on x86):**

| Scenario | Sparsity | Speedup over dense float32 |
|----------|----------|-----------------------------|
| Typical BitNet-style distribution | ~50–70% | **2–4×** |
| Highly sparse ternary model | ~90% | **~10×** |
| Extreme sparsity (upper bound) | ~99% | **up to 122×** |

The 2.3× figure is the baseline measured result from the first `@sparseskip` benchmark ([commit `60f7ef6`](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/commit/60f7ef659)). Speedup scales proportionally with sparsity — the 122× figure is a mathematical upper bound at 99%+ sparsity, not a general-case claim.

**5-trit block packing** encodes 5 trits into 8 bits (vs 10 bits for naive 2-bit emulation) — a 1.25× storage density improvement.

```bash
cd benchmarks && make bench-all
```

---

## MoE-13: Explainable Mixture-of-Experts


MoE-13 an **ecocentric deliberation architecture** designed for high-stakes decision systems where safety, ethics, causality, and contextual memory must participate as first-class reasoning agents.

Instead of routing tokens to computational experts, MoE-13 routes a decision query through **13 specialist epistemic agents**, each representing a critical dimension of trustworthy reasoning.

### The 13 Deliberation Axes

| Axis | Role |
|---|---|
| Safety | Immediate risk / harm detection |
| MetaSafety | Safety-on-safety audit and veto authority |
| Logic | Formal consistency |
| Ethics | Normative and moral constraints |
| FactCheck | Claim verification |
| Causal | Cause-effect integrity |
| Context | Situational awareness |
| History | Prior decision memory |
| Ambiguity | Uncertainty detection |
| Math | Quantitative verification |
| ToolUse | External action risk |
| Persona | Human alignment layer |
| Efficiency | Resource and environmental cost |

Each expert returns a ternary vote:

- `-1` → reject
- ` 0` → hold / insufficient evidence
- `+1` → affirm

Votes are weighted by EMA convergence confidence and combined into a network-wide verdict.

### Safety-first ecological veto

MoE-13 is explicitly **ecocentric**.

No majority can override a hard safety veto.

If either `Safety` or `MetaSafety` returns `-1` with confidence > 0.90, the entire decision chain terminates immediately before any tool execution or external action.

This mirrors ecological systems where boundary constraints dominate local optimization.

### Network telemetry

Every axis emits:

- live vote state
- confidence score
- convergence momentum
- trace logs
- veto rationale

This creates a fully auditable reasoning path for EU AI Act Article 13 / 14 compliance.

`ternlang-moe` implements the MoE-13 architecture ([DOI: 10.17605/OSF.IO/TZ7DC](https://doi.org/10.17605/OSF.IO/TZ7DC)) — a **Deterministic Mixture-of-Experts** system that routes queries through 13 domain-specific agents to achieve **Emergent Reasoning** with a mandatory safety hard gate.

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
2. **Emergent triad synthesis** — weighted field `Ek = synergy × (vi + vj) / 2`. Two orthogonal experts produce a composite signal whose confidence exceeds either input independently.
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
| `POST /api/stream/moe_orchestrate` | MoE-13 pass streamed round-by-round via SSE |
| `POST /api/audit` | TernAudit — binary habituation + EU AI Act Art.13/14 heuristic (Tier 2+) |
| `POST /api/translate` | TernTranslator — Python/SQL/JSON rules → .tern with tend arms (Tier 2+) |

---
### MCP Server — v1.1.0 (30 tools — all free)

The MCP server runs at `https://ternlang.com/mcp` — compatible with Claude Desktop, Smithery, Cursor, Gemini CLI, and any HTTP MCP client.

**All 30 tools are free — no API key required.** An optional API key (`X-Ternlang-Key`) upgrades memory to server-side persistent storage (instead of stateless blob mode) and unlocks the REST API with rate-limited quota.

**Core trit primitives:** `trit_decide` · `trit_vector` · `trit_consensus` · `trit_eval` · `trit_action_gate`

**BET VM:** `ternlang_run` · `trit_translate`

**MoE-13 orchestration:** `moe_orchestrate` · `moe_deliberate` · `moe_full` · `trit_debate` · `trit_calibrate` · `trit_uncertainty_map`

**3-layer memory:** `trit_mem_write` · `trit_mem_read` · `trit_mem_consolidate` · `trit_mem_stats` · `trit_mem_compress`

**Context & planning:** `trit_compress` · `trit_triage` · `trit_plan` · `trit_factcheck`

**EcoCore + Audit:** `trit_eco_check` · `trit_audit` · `audit_ternary_logic`

**ML primitives:** `quantize_weights` · `sparse_benchmark` · `tsql_join` · `get_industrial_standards` · `trit_upgrade`

**New in v0.3.0 — Phase 11A & Phase 13 tools:**

| Tool | What it does |
|------|-------------|
| `trit_debate` | Routes two claims through MoE-13, compares trits, returns tension score and synthesis |
| `trit_uncertainty_map` | Scans text sentence-by-sentence, annotates each claim as affirm/tend/reject |
| `trit_calibrate` | Analyzes a decision log for binary habituation — surfaces over-confident yes/no patterns |
| `trit_translate` | Converts Python `if/elif/else`, SQL `CASE WHEN`, or JSON rules into `.tern` with `tend` arms |
| `trit_eco_check` | Compares human decision trit against environmental signal — returns tension and synthesis |
| `trit_audit` | Full TernAudit: binary habituation ratio + EU AI Act Art.13/14 compliance heuristic |

#### Three-Layer AI Memory (v0.3.0 flagship)

The `trit_mem_*` tools implement a server-side three-layer memory system modelled on human memory consolidation:

| Layer | TTL | Capacity | Write behaviour | Consolidation |
|-------|-----|----------|-----------------|---------------|
| `working` | 1h | LRU-256 | Raw write | Affirm → session (compressed) |
| `session` | 24h | LRU-128 | Ternary-compressed | Affirm at half-life → MoE-13 → core |
| `core` | Never | Unlimited | Compressed + MoE-resolved | Identity anchors, vetoes |

**Ternary attention on read:** `score = key_overlap×0.35 + value_overlap×0.55 + trit_bias×0.10`

**Without API key (stateless blob mode):** `trit_mem_write` returns a `state` blob — pass it back in subsequent calls. All computation still works; data just lives client-side.

**With API key (server-side mode):** Memory is stored server-side keyed to your API key — no state blob to pass between calls. Persistent across sessions.

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

**30,000+ `.tern` programs** (2,090 examples + 27,800+ stdlib) across every domain — establishing TIS as the definitive standard for balanced ternary logic.

| Category | Examples |
|----------|---------|
| [Aerospace & Safety](examples/03_rocket_launch.tern) | Rocket launch, aircraft deicing, runway incursion, satellite collision |
| [Medicine](examples/05_medical_triage.tern) | ER triage, ICU ventilator, sepsis warning, organ transplant, APGAR |
| [Finance](examples/42_trading_signal.tern) | Algorithmic trading, AML filter, options expiry, loan underwriting |
| [Infrastructure](examples/14_circuit_breaker.tern) | Circuit breaker, nuclear reactor SCRAM, bridge health, power grid |
| [AI Agents](examples/08_evidence_collector.tern) | Evidence density, confidence escalation, MoE routing, deliberation |
| [Civic Systems](examples/12_vote_aggregator.tern) | Vote aggregation, bail decision, treaty negotiation, refugee status |
| [Computer Science](examples/09_risc_fetch_decode.tern) | CPU pipeline, cache invalidation, API rate limiting, deployment gate |
| [Tutorials](stdlib/tutorials/) | Step-by-step tutorials — hello ternary → full ML pipeline |
| [QNN / Qutrit](stdlib/qnn/) | Qutrit Neural Networks — Kepp 2026 reference implementations |
| [Standard Library](stdlib/) | Agents, reasoning, ML layers, optimizers, std, benchmarks |

→ [**Browse all 2,090 examples**](examples/INDEX.md)

---

## Architecture

| Crate | Tier | Description |
|-------|------|-------------|
| [`ternlang-core`](ternlang-core/) | Open (LGPL) | Lexer, parser, AST, BET VM — 53 opcodes, unbounded register file |
| [`ternlang-cli`](ternlang-cli/) | Open (LGPL) | `run` · `build` · `sim` · `fmt` · `repl` · `compat` · `audit` · `translate` |
| [`ternlang-lsp`](ternlang-lsp/) | Open (LGPL) | LSP 3.17 — hover, completion, diagnostics |
| [`ternlang-compat`](ternlang-compat/) | Open (LGPL) | 9-trit RISC assembler (Brandon Smith bridge), Owlet S-expr parser |
| [`ternpkg`](ternpkg/) | Open (LGPL) | Package manager, GitHub-backed registry |
| [`ternlang-ml`](ternlang-ml/) | BSL-1.1 | Sparse matmul, BitNet quantization, TernaryMLP, deliberation engine, coalition vote, action gate |
| [`ternlang-moe`](ternlang-moe/) | BSL-1.1 | MoE-13 orchestrator — dual-key routing, triad synthesis, 3-tier memory, AgentHarness |
| [`ternlang-api`](ternlang-api/) | BSL-1.1 | REST + SSE API, multi-tenant key management, GitHub repo invite flow |
| [`ternlang-mcp`](ternlang-mcp/) | BSL-1.1 | MCP server — 19 tools (10 free + 9 premium), stdio + HTTP transport, server-side 3-layer memory, TernAudit |
| [`ternlang-mkl`](ternlang-mkl/) | BSL-1.1 | **cuTern**: Math Kernel Library with native sparsity bypass |
| [`ternlang-sql`](ternlang-sql/) | BSL-1.1 | Native Ternary Graph Database driver (50% speedup) |
| [`ternlang-bridge`](ternlang-bridge/) | BSL-1.1 | Binary-to-Ternary Transpiler (The Seamless Migration Layer) |
| [`ternlang-net`](ternlang-net/) | BSL-1.1 | Triadic Networking Stack (Introspective Handshake) |
| [`ternlang-crypto`](ternlang-crypto/) | BSL-1.1 | High-entropy Trit-based Cryptographic primitives |
| [`ternlang-fs`](ternlang-fs/) | BSL-1.1 | Triadic File System (deliberative hold Transactional Pend) |
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

**88+ probe tests passing · v1.2.1**

---

## RFI-IRFOS Protocol Specifications

> These are open technical proposals authored by RFI-IRFOS. They are not yet ratified by IEEE, ISO, or any external standards body. They define intended behaviour for the Ternlang ecosystem and are published to establish prior art and invite community review.

### Core Architecture & Logic
- [**BET-ISA v1.0**](spec/standards/BET-ISA-v1.0.md): The definitive 9-Trit RISC Instruction Set Architecture.
- [**T-UNCERTAINTY**](spec/standards/ISO-CERTIFIED-UNCERTAINTY.md): Specification for deliberative hold abstention in safety-critical decision systems.
- [**TSON v1.0**](spec/standards/TSON-v1.0.md): Optimized data serialization for triadic systems.
- [**T-POSIX v1.0**](spec/standards/T-POSIX-v1.0.md): Operating system interface redefining process state logic.

### AI, Memory & Intelligence
- [**T-TOKEN v1.0**](spec/standards/T-TOKEN-v1.0.md): Trit-Pair Encoding (TPE) — Compressing semantic entropy by 33%.
- [**T-KV-CACHE v1.0**](spec/standards/T-KV-CACHE-v1.0.md): The Memory Moat — Eliminating 60% of zero-signal KV allocations.
- [**T-WEIGHT v1.0**](spec/standards/T-WEIGHT-v1.0.md): Triadic Weight Exchange — Mandatory safety headers for LLM weights.
- [**T-EXPLAIN v1.0**](spec/standards/T-EXPLAIN-v1.0.md): Triadic Traceability — Standardizing MoE-13 deliberation logs.
- [**T-HALO v1.0**](spec/standards/T-HALO-v1.0.md): Triadic Alignment & Governance — Hardware-locked safety holds.
- [**TUANN v1.0**](stdlib/ml/tuann.tern): Triadic Uncertainty-Aware Neural Networks — Native deliberative hold for hallucination rejection.

### Hardware, Physical & Frontier
- [**T-HAL v1.0**](spec/standards/T-HAL-v1.0.md): Universal Hardware Abstraction — The bridge for Huawei/NVIDIA/FPGA.
- [**T-SENSE v1.0**](spec/standards/T-SENSE-v1.0.md): Ternary Sensor Fusion — Triadic Delta fields for IoT efficiency.
- [**T-THERMAL v1.0**](spec/standards/T-THERMAL-v1.0.md): Power-Aware Compute — Dynamic clock scaling via sparsity.
- [**T-BIO v1.0**](spec/standards/T-BIO-v1.0.md): Triadic Neural Encoding — 1:1 parity for Brain-Computer Interfaces.
- [**T-QUT v1.0**](spec/standards/T-QUT-v1.0.md): Qutrit Bridge Standard — Positioning TIS as the OS for Quantum.

---

## RFI-IRFOS Protocol Specifications

> These are open technical proposals authored by RFI-IRFOS. They are not yet ratified by IEEE, ISO, or any external standards body. They define intended behaviour for the Ternlang ecosystem and are published to establish prior art and invite community review.

### Core Architecture & Logic
- [**BET-ISA v1.0**](spec/standards/BET-ISA-v1.0.md): The definitive 9-Trit RISC Instruction Set Architecture.
- [**T-UNCERTAINTY**](spec/standards/ISO-CERTIFIED-UNCERTAINTY.md): Specification for deliberative hold abstention in safety-critical decision systems.
- [**TSON v1.0**](spec/standards/TSON-v1.0.md): Optimized data serialization for triadic systems.
- [**T-POSIX v1.0**](spec/standards/T-POSIX-v1.0.md): Operating system interface redefining process state logic.

### AI, Memory & Intelligence
- [**T-TOKEN v1.0**](spec/standards/T-TOKEN-v1.0.md): Trit-Pair Encoding (TPE) — Compressing semantic entropy by 33%.
- [**T-KV-CACHE v1.0**](spec/standards/T-KV-CACHE-v1.0.md): The Memory Moat — Eliminating 60% of zero-signal KV allocations.
- [**T-WEIGHT v1.0**](spec/standards/T-WEIGHT-v1.0.md): Triadic Weight Exchange — Mandatory safety headers for LLM weights.
- [**T-EXPLAIN v1.0**](spec/standards/T-EXPLAIN-v1.0.md): Triadic Traceability — Standardizing MoE-13 deliberation logs.
- [**T-HALO v1.0**](spec/standards/T-HALO-v1.0.md): Triadic Alignment & Governance — Hardware-locked safety holds.
- [**TUANN v1.0**](stdlib/ml/tuann.tern): Triadic Uncertainty-Aware Neural Networks — Native deliberative hold for hallucination rejection.

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
- [**T-ROUTING v1.0**](spec/standards/T-ROUTING-v1.0.md): Sparse Packet Switching — Skipping deliberative hold network congestion.
- [**T-SQL v1.0**](spec/standards/T-SQL-v1.0.md): Triadic Query Language — Redefining search via T-Trees.
- [**T-ARCHIVE v1.0**](spec/standards/T-ARCHIVE-v1.0.md): Triadic Cold Storage — Neutral State structural stability.
- [**T-Fi v1.0**](spec/standards/T-Fi-v1.0.md): Triadic Compute Currency — Standardizing the TaaS cryptographic toll.
- [**T-TAX v1.0**](spec/standards/T-TAX-v1.0.md): Automatic Compute Dividends — Decentralized royalty routing.


---

## Standard Library Access

The Ternlang stdlib is split across two repositories to protect proprietary IP:

| | Repo | Files | Access |
|--|------|-------|--------|
| **Tier 1 — Open Core** | [`ternary-intelligence-stack/stdlib/`](stdlib/) | 293 `.tern` modules | Free — clone this repo |
| **Tier 2/3/4 — Premium** | [`eriirfos-eng/ternlang-premium`](https://github.com/eriirfos-eng/ternlang-premium) *(private)* | 28,495+ `.tern` modules | Paid license required |

The open-core stdlib includes: `core/`, `ternary/`, `std/`, `showcase/`, `bughunt/`, `testing/`, `bench/`, `benchmarks/`, `classical/`, `errors/`, `tutorials/`, `lib/`

The premium repo includes all paid-tier directories: `agents/`, `ml/`, `nn/`, `nlp/`, `finance/`, `bio/`, `crypto/`, `security/`, `math/`, `logic/`, `vision/`, `stats/`, `physics/`, `qnn/`, and 40+ more.


---

## Licensing Tiers

### Tier 2 — Pro Standard
For developers and startups building AI agents. **All 30 MCP tools are free** for every tier. Tier 2 adds: REST API (10,000 calls/month), server-side persistent 3-layer memory, SSE streaming, and production SLA. **€99/month** · [Subscribe](https://buy.stripe.com/5kQ28t7SM4rB0DH6jm7N608)

### Tier 3 — Industrial
Production-grade deployment for teams requiring EU AI Act-compliant safety gating, audit trails, and high-volume inference. Includes 50,000 API calls/month, QNN & SEC modules, T-HAL silicon bindings, and TernAudit log access. **€349/month** · [Subscribe](https://buy.stripe.com/eVq7sNfle0bl86937a7N609)

### Tier 4 — Enterprise
On-premise BET-VM clusters, custom FPGA integration via `ternlang-hdl`, unlimited throughput, dedicated SLA, and direct BSL-1.1 source access for air-gapped or regulated environments. Contact [licensing@ternlang.com](mailto:licensing@ternlang.com) — **from €2,500/month.**

```
┌─────────────────────────────────────────────────────────────────┐
│  TIER 1 — Open Core (LGPL-3.0)                     Free         │
│  ternlang-core · ternlang-cli · ternlang-lsp · ternlang-compat  │
│  ternpkg · spec/                                                │
│  Free to use, modify, and distribute under LGPL.                │
├─────────────────────────────────────────────────────────────────┤
│  TIER 2 — Pro Standard (BSL-1.1)                  €99/mo        │
│  ★ 30 MCP tools (all free) + REST API (10,000 calls/month)      │
│  ✓ Server-side persistent 3-layer memory                        │
│  ✓ SSE streaming · MoE-13 consolidation · production SLA        │
├─────────────────────────────────────────────────────────────────┤
│  TIER 3 — Industrial (BSL-1.1)                  €349/mo         │
│  ✓ 50,000 API calls/month · QNN & SEC modules                   │
│  ✓ T-HAL silicon bindings · TernAudit · Full premium stdlib     │
├─────────────────────────────────────────────────────────────────┤
│  TIER 4 — Enterprise (Proprietary)      from €2,500/mo          │
│  On-premise BET-VM clusters · Custom FPGA · Enterprise SLA      │
│  Unlimited throughput · Dedicated support · Air-gap ready       │
│  Contact: licensing@ternlang.com                                │
└─────────────────────────────────────────────────────────────────┘
```

**After purchasing a license:** Visit **[ternlang.com/activate](https://ternlang.com/activate)**, enter your API key and GitHub username, and you will receive a collaborator invite to the private repo automatically.

→ [See full tier table in stdlib/PREMIUM.md](stdlib/PREMIUM.md)
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

---

## EU AI Act Compliance

> Ternlang is designed from first principles to support EU AI Act compliance for downstream AI systems. The following applies to TIS v0.3.0 and later.

| EU AI Act Article | How Ternlang addresses it |
|-------------------|--------------------------|
| **Art. 9 — Risk Management** | MoE-13 Safety hard gate: Axis-6 veto fires unconditionally when Safety confidence > 0.90. Every veto is permanently logged to AxisMemory. |
| **Art. 11 — Technical Documentation** | BET-ISA-SPEC.md, ternlang-whitepaper.tex (DOI: 10.17605/OSF.IO/TZ7DC), RFI-IRFOS protocol specifications (spec/standards/). |
| **Art. 12 — Record-Keeping** | TernAudit: every trit=0 hold and trit=−1 veto is timestamped, logged, and queryable. Persistent AxisMemory. CLI: `ternlang audit decisions.json [--html]`. MCP: `trit_audit` tool. |
| **Art. 13 — Transparency** | `trit` type is an explicit uncertainty carrier. Every decision includes a confidence score and a `hint` field explaining the outcome in plain text. |
| **Art. 14 — Human Oversight** | `trit = 0` (hold) is a first-class routing signal to human review queues. The system cannot be forced to commit — it blocks until evidence threshold is met or a human resolves the hold. |
| **Art. 15 — Accuracy & Robustness** | Compile-time exhaustive 3-way match enforcement. Non-exhaustive matches are a compile error. EMA convergence loop prevents single-round snap decisions. |
| **GDPR — Data Residency** | API deployed on Fly.io Frankfurt region (EU). Server-side memory is keyed to API key and not shared across tenants. |

**EU AI Act risk classification:** Ternlang is a **general-purpose AI tool** (not a listed high-risk system per Annex III). When deployed in high-risk contexts (medical, financial, judicial), downstream operators bear classification responsibility. Ternlang's architecture is designed to make those deployments auditable and verifiable.

---

## Contact & Licensing

| | |
|---|---|
| **Website** | [ternlang.com](https://ternlang.com) |
| **Commercial licensing** | [licensing@ternlang.com](mailto:licensing@ternlang.com) |
| **Academic collaboration** | Open — cite the whitepaper |
| **API access** | [ternlang.com/#licensing](https://ternlang.com/#licensing) |
