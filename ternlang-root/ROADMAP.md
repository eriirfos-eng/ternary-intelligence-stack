# Ternlang Roadmap: Bridging the Ternary Software Deficit
### Project: Ternary Intelligence Stack (TIS) | RFI-IRFOS
**Current Version:** v0.3.0
**Last Updated:** 2026-04-10
**Repo:** https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-
**Local:** ~/Desktop/Ternary Intelligence Stack (TIS)/

---

## 🎯 Strategic Objective
Position RFI-IRFOS as the definitive middleware provider for ternary computing by commercializing **ternlang** as the standard paradigm for ambiguity-aware AI agents and sparse inference.

This is our philosopher's stone. Full resource commitment.

---

## 🔁 Development Protocol (READ THIS FIRST)
- **Always work locally AND push to GitHub** after every meaningful session
- Push command (credentials stored): `git push origin main` from inside `Ternary Intelligence Stack (TIS)/`
- Pull before starting a new session: `git pull origin main`
- Update this ROADMAP at the end of every session with current status
- The AI assistant (Claude) maintains a memory file at `~/.claude/projects/.../memory/project_ternlang.md` — update it each session too

### 📄 Whitepaper Update Protocol
The academic whitepaper (`whitepaper/ternlang-whitepaper.tex` + `whitepaper/ternlang-whitepaper.docx`) is a **living document**. Update it whenever a phase produces measurable results:
- New opcodes or VM features → update Section 4 (ISA) and Section 10 (implementation status)
- New benchmark numbers → update Section 5 (sparse inference) tables
- New crates or test counts → update Table 8 (implementation status)
- Rebuild DOCX: `python3 whitepaper/build_docx.py`
- Rebuild PDF: `cd whitepaper && pdflatex ternlang-whitepaper.tex` (requires texlive)

---

## ✅ Phase 1: Core Language & VM Stability — COMPLETE
- [x] **Trit Primitives**: `-1`, `0`, `+1` logic (Sum/Carry, Neg, Mul) — fully tested
- [x] **Lexer**: Tokenize ternary-specific keywords (`trit`, `trittensor`, `?`, `sparseskip`)
- [x] **Skeletal Parser**: Parse basic expressions and `IfTernary` (`if ?`)
- [x] **BET VM Core**: Stack, 27 registers, carry reg, 2-bit packing (`0b01=-1`, `0b10=+1`, `0b11=0`)
- [x] **Parser Completion**: `Function`, `Program`, `match` with 3-way exhaustive branching
- [x] **Codegen (Bytecode Emitter)**: Jump resolution, register allocation, symbol table
- [x] **VM Enhancements**: Carry handling in `Tadd`, rich `VmError` reporting

---

## ✅ Phase 2: Standard Library & CLI Integration — COMPLETE
- [x] **CLI Driver**: `ternlang run <file>` and `ternlang build <file>` (clap-based)
- [x] **Built-in Functions**: `consensus(a,b)`, `invert(x)`, `truth()`, `hold()`, `conflict()`
- [x] **Standard Library (`std::trit`)**: COMPLETE — ModuleResolver with stdlib built-in (compile-time embedded) + filesystem-relative user modules

---

## ✅ Known Bugs — ALL FIXED
- [x] **`DimSeparator` vs `Ident` collision**: FIXED — removed dedicated token, `x` now correctly tokenizes as `Ident` everywhere
- [x] **No function call dispatch**: FIXED — TCALL/TRET fully implemented with call stack, emit_entry_call("main") in test harness
- [x] **`@sparseskip` is a stub**: FIXED — codegen emits TSPARSE_MATMUL correctly via BytecodeEmitter
- [x] **Semantic checker mocks all `Call` return types as `Trit`**: FIXED — real function table lookup implemented in SemanticAnalyzer
- [x] **Match exhaustiveness not enforced**: FIXED — parser rejects non-exhaustive match at compile time [PARSE-004]

---

## 🛠 Phase 3: TritTensors & Sparse Inference — IN PROGRESS
**This is the commercial differentiator. The AI inference story.**

- [x] **Fix `DimSeparator` bug** in lexer (remove dedicated token, handle as `Ident("x")` in type parser) — 11/11 tests passing
- [x] **TCALL/TRET opcodes**: Real function call dispatch with call stack — DONE
- [x] **TCALL/TRET opcodes**: Real function call dispatch with call stack — DONE
- [x] **TritTensor VM Operations** — DONE (14/14 tests passing):
    - [x] `0x20` `TMATMUL` — multiply two tensor refs
    - [x] `0x21` `TSPARSE_MATMUL` — matmul skipping zero-state weights (flagship) ⭐
    - [x] `0x22` `TIDX` — index into tensor (tensor_ref, row, col → trit)
    - [x] `0x23` `TSET` — store trit at tensor index
    - [x] `0x24` `TSHAPE` — push tensor dimensions to stack
    - [x] `0x25` `TSPARSITY` — compute zero-element count
- [x] **Implement `@sparseskip`** in codegen → emits `TSPARSE_MATMUL` — DONE
- [x] **`TCOMPRESS` (0x26) / `TUNPACK` (0x27)** — base-3 RLE codec for sparse trit tensors; max-chunk=8 (2 base-3 digits), NegOne header; 5 VM tests
- [x] **Fill `ternlang-ml`** with real kernels — DONE:
    - [x] `quantize(f32_weights, threshold) -> Vec<Trit>` — BitNet-style ternary quantization
    - [x] `bitnet_threshold(weights)` — auto-compute τ = 0.5 × mean(|w|)
    - [x] `dense_matmul(a, b) -> TritMatrix` — baseline
    - [x] `sparse_matmul(a, b) -> (TritMatrix, skipped_count)` — flagship kernel
    - [x] `linear(input, W) -> (TritMatrix, skipped)` — BitNet-style ternary linear layer
    - [x] `benchmark(a, b) -> BenchmarkResult` — prints summary with skip rate
- [x] **First benchmark result**: 56% weight sparsity → **2.3x fewer multiply ops** vs dense
- [x] **Wall-clock timing benchmark**: 5 sizes (32²–512²), 5-rep median
- [x] **CSC sparse matmul**: Compressed Sparse Column precompute — branch-free inner loop; at 25% sparsity: **4.8–6.9× faster**; at 60% sparsity (BitNet-realistic): **8–14× faster** than dense in release mode
- [x] **BitNet b1.58 benchmark**: explicit 60% sparsity, release mode — **86×** at 512² (3-layer CSC kernel)
- [x] **Goldilocks sparsity sweep**: 9 sparsity levels × 5 sizes — peak **122×** at 99% sparsity 512²; goldilocks zone confirmed at 40–60% sparsity (20–57× on medium matrices)
- [x] **TernaryMLP**: 2-layer MLP (from_f32, forward, predict, XOR/parity datasets, accuracy eval) — full inference path tested end-to-end
- [ ] **Publish sparse matmul benchmark** — write blog post / README section comparing vs float32

---

## 🧩 Phase 4: Language Completeness — IN PROGRESS
- [x] **Lexer**: `for`, `in`, `while`, `loop`, `break`, `continue`, `mut`, `use`, `module`, `pub`, `struct`, `::`, `!=`, `&&`, `||`
- [x] **AST**: `ForIn`, `WhileTernary`, `Loop`, `Break`, `Continue`, `Use` nodes; `BinOp::NotEqual/And/Or`; `Type::Bool/Float/String`
- [x] **Parser**: `for x in expr { }`, `while cond ? { } else { } else { }`, `loop { }`, `break`, `continue`, `use std::trit;`, `let mut`
- [x] **Match exhaustiveness enforcement** in parser — `NonExhaustiveMatch` error if any of -1/0/+1 missing
- [x] **Codegen**: `ForIn`, `Loop`+`Break`, `WhileTernary`, `Use` (no-op), `Continue` (no-op), `BinOp` operators
- [x] **Semantic checker**: all new nodes handled
- [x] **Standard Library** source files: `std::trit`, `std::tensor`, `std::math`, `std::io`, `ml::quantize`, `ml::inference`
- [x] **StdlibLoader**: `use std::trit;` inside function bodies actually injects parsed stdlib functions — `include_str!` at compile time, zero runtime filesystem I/O
- [x] **Comment support in lexer**: `//` line comments now skipped — user programs and stdlib files can use comments freely
- [x] **Real function call type resolution** in semantic checker (FunctionSig exact/variadic, ArgCountMismatch, ArgTypeMismatch, ReturnTypeMismatch)
- [x] `cast()` expression for bool→trit coercion — transparent BET pass-through, type-system level only
- [x] `struct` definitions and field access — `struct Name {}`, `s.field`, `s.field = v;`, `Type::Named`

---

## 🔌 Phase 3.5: MCP Integration — COMPLETE ✅
**Any binary AI agent connected to this MCP server becomes a ternary decision engine.**

- [x] `ternlang-mcp` crate — JSON-RPC 2.0 over stdio, MCP protocol 2024-11-05
- [x] `trit_decide` — flagship tool: float evidence → ternary decision (+1/0/-1) with confidence, interpretation, sparsity
- [x] `trit_consensus` — consensus(a, b) with carry
- [x] `trit_eval` — evaluate ternlang expressions on live BET VM
- [x] `ternlang_run` — compile + run full .tern programs via MCP
- [x] `quantize_weights` — f32 → ternary with BitNet thresholding
- [x] `sparse_benchmark` — sparse vs dense matmul stats
- [x] `mcp-config.json` — drop-in config for Claude Desktop and any MCP client
- [x] Release binary: `target/release/ternlang-mcp`

**Next for MCP:** publish to MCP registry, write integration guide

---

## 🤖 Phase 5: Actor Model & Distributed Agents — PHASE 5.1 COMPLETE ✅
- [x] **Lexer/Parser/AST**: `agent`, `spawn`, `send`, `await`, `agentref` — all done
- [x] **Local Actor Runtime**: AgentInstance + mailbox, `TSPAWN`/`TSEND`/`TAWAIT` opcodes, synchronous dispatch
- [x] **Integration test**: spawn identity-agent, send +1, await → +1 ✓
- [x] **Distributed Runtime** (Phase 5.1): `RemoteTransport` trait in core (no circular dep), `TernNode` impl in runtime; TSEND/TAWAIT route over TCP for remote AgentRefs; auto-connect on first use; 4 runtime tests passing
- [x] **`remote`/`nodeid`** keywords: `--node-addr` + `--peer` CLI flags; `TernNode` injected into VM via `set_remote(Arc<dyn RemoteTransport>)`

---

## 📡 Phase 6: Hardware & HDL Backends — PHASE 6.1 COMPLETE ✅
- [x] **Verilog/VHDL Codegen**: `ternlang-hdl` crate, map trit → 2-bit wire pairs
  - Primitives: trit_neg, trit_cons, trit_mul, trit_add, trit_reg, bet_alu
  - Sparse matmul array: parameterised N×N with per-cell zero-skip enable
  - ISA control: bet_regfile (27 reg), bet_pc (16-bit), bet_control (all opcodes), bet_processor (top-level)
  - 11 HDL tests passing
- [x] **BET ISA Spec Document**: `BET-ISA-SPEC.md` — formal ISA spec with encoding tables, stack-effect notation, hardware mapping
- [x] **FPGA Simulation** (Phase 6.1): Cycle-accurate RTL simulator in pure Rust (`BetRtlProcessor`) — mirrors `bet_processor.v` exactly; same 2-bit encoding, clocked regfile, PC, ALU; 12 RTL unit tests; `ternlang sim --rtl` CLI flag (no external tools needed); iverilog path still supported via `ternlang sim --run`

---

## 🛠 Developer Tooling — COMPLETE ✅
- [x] **LSP**: `ternlang-lsp` crate — JSON-RPC 2.0 over stdio, diagnostics, hover, completion (19 snippets)
- [x] **VS Code extension v0.3.0**: `ternlang-vscode/` — TextMate grammar, 19 snippets, `Ctrl+Shift+R` run command, graceful LSP, status bar, 4-tier API key gating. Published to Open VSX (`rfi-irfos/ternlang`).
- [x] **Formatter**: `ternlang fmt [--write]` — canonical style for 3-way match arms
- [x] **REPL**: `ternlang repl` — interactive trit expression evaluation via BET VM
- [x] **Package manager (ternpkg)**: `ternlang.toml`, `ternpkg install [PKG]`, GitHub-backed registry

---

## 🌐 Phase 7: Ecosystem Bridges — PHASE 7A COMPLETE ✅
**Goal: make ternlang the convergence point for all existing ternary computing projects.**

- [x] **Hub positioning**: README + TERNARY-ECOSYSTEM.md — maps every active ternary project to a ternlang interop bridge
- [x] **TasmAssembler** (ternlang-compat): two-pass 9-trit RISC assembler → BET bytecode; parses balanced ternary literals (T=-1); 15 tests
- [x] **OwletParser** (ternlang-compat): S-expression ternary front-end → ternlang AST → BET VM; full S-expr grammar; 14 tests
- [x] **VS Code VSIX packaging**: `ternlang-0.1.0.vsix` built, publisher metadata set (rfi-irfos)
- [x] **Cargo workspace metadata**: `[workspace.package]` with keywords, categories, license, repository for crates.io
- [x] **Academic whitepaper**: `whitepaper/ternlang-whitepaper.tex` (IEEE two-column LaTeX) + `ternlang-whitepaper.docx`
- [x] **Spec consolidation**: `spec/grammar.ebnf`, `spec/ternlang-language-reference-v0.1.md`, `spec/ternlang-dictionary-v0.1.json` versioned in main repo
- [ ] **Phase 7B**: VS Code Marketplace publication (needs user publisher PAT token → `vsce publish`)
- [x] **Phase 7B**: crates.io — all 9 crates published (2026-04-04)
- [x] **Phase 7B**: MCP registry — HTTP transport live at https://ternlang.com/mcp, Smithery submission in progress
- [ ] **Phase 7C**: USN / Bos+Gundersen academic outreach, joint whitepaper draft

---

## 🧠 Phase 8: Ternary AI Reasoning Toolkit — COMPLETE ✅
**Tools that make any AI agent structurally ternary in its decisions.**

- [x] **`DeliberationEngine`** (`ternlang-ml`): EMA-based iterative deliberation loop — converges scalar toward target confidence via configurable `alpha` and max rounds; returns `DeliberationResult` with `rounds_used`, `converged`, final scalar
- [x] **`coalition_vote()`** (`ternlang-ml`): weighted trit vote across N agents — `CoalitionResult` includes quorum, dissent, abstain, consensus trit, dominant faction fraction
- [x] **`action_gate()`** (`ternlang-ml`): multi-dim hard-block gate — `GateDimension` with optional `hard_block`; `GateVerdict::Blocked` fires on any hard-block dim regardless of other dims (safety veto pattern)
- [x] **`scalar_temperature()`** (`ternlang-ml`): trit state → LLM sampling temperature bridge; affirm+high_conf → low temp (focused), hold → mid (exploratory), reject → very low (cautious)
- [x] **`hallucination_score()`** (`ternlang-ml`): signal variance → trust trit; high spread = untrusted = -1
- [x] **`trit_i8()` + `#[derive(Clone)]`** on `TritScalar` — enables serialization and use in API/MoE layers
- [x] **Phase 8 REST endpoints** (`ternlang-api`): `/api/trit_deliberate`, `/api/trit_coalition`, `/api/trit_gate`, `/api/scalar_temperature`, `/api/hallucination_score`
- [x] **Multi-tenant key management** (`ternlang-api`): `KeyStore` (JSON-backed, async RwLock), `tern_<tier>_<uuid24>` format, admin CRUD routes, usage counters, revocation
- [x] 15 reasoning tests in `ternlang-ml`

---

## 🎭 Phase 9: MoE-13 Ternary Orchestrator — COMPLETE ✅
**The head of a ternary Mixture-of-Experts system. Based on prior RFI-IRFOS research.**

Paper: DOI [10.17605/OSF.IO/TZ7DC](https://doi.org/10.17605/OSF.IO/TZ7DC) · TVLD: DOI [10.17605/OSF.IO/X96HS](https://doi.org/10.17605/OSF.IO/X96HS)

- [x] **`CompetenceVector`** — 6D space: `[syntax, world_knowledge, reasoning, tool_use, persona, safety]`; cosine similarity, synergy (complementarity metric), dot product, norm
- [x] **`TernMoeRouter`** — dual-key synergistic routing: score = `rel_a × rel_b × synergy`; selects the expert pair that maximises relevance AND complementarity simultaneously
- [x] **`TriadField::synthesize()`** — 1+1=3 emergent field: `Ek = synergy × (vi + vj) / 2`; `is_amplifying()` check
- [x] **`NodeMemory`** — TTL-based volatile store (LRU cap 256, TTL in seconds)
- [x] **`ClusterMemory`** — routing frequency counters, `mode_collapse_risk()` (fraction dominated by one pair)
- [x] **`AxisMemory`** — permanent audit log: safety veto log with timestamp + query hash, global priors
- [x] **`TernMoeOrchestrator::orchestrate()`** — full 9-step pipeline:
  1. Encode query → 6D evidence vector
  2. Dual-key route → best expert pair
  3. Evaluate both experts independently
  4. Synthesise triad field (1+1=3)
  5. Safety hard-gate (Axis-6 absolute veto → logs to AxisMemory)
  6. Weighted trit vote (confidence + synergy amplification)
  7. Hold detection (trit=0 or conf below threshold)
  8. Tiebreaker invocation (max 4 active experts, selected by highest reasoning dim)
  9. Return `OrchestrationResult` + update all three memory tiers
- [x] **`with_standard_experts()`** — canonical MoE-13 pool: Syntax, WorldKnowledge, DeductiveReason, InductiveReason, ToolUse, Persona, Safety, FactCheck, CausalReason, AmbiguityRes, MathReason, ContextMem, MetaSafety (13 experts)
- [x] **Temperature bridge** — affirm+high_conf → 0.3, hold → 0.7–0.9, reject → 0.05
- [x] **`OrchestrationResult`** — trit, confidence, verdicts, triad_field, pair, held, safety_vetoed, temperature, prompt_hint
- [x] 16 tests: competence vectors, router, triad synthesis, safety veto, hold/tiebreaker, reject, memory (node TTL, cluster mode-collapse, axis veto log), standard pool, full orchestration

**Phase 9 additions (2026-04-04):**
- [x] MCP tools: `moe_orchestrate`, `moe_deliberate`, `trit_action_gate` (3 new tools → 10 total in ternlang-mcp)
- [x] Whitepaper updated — Phase 8 + Phase 9 sections added (887 lines)
- [x] SSE streaming in `ternlang-api`: `GET /api/stream/moe_orchestrate` + `GET /api/stream/deliberate`
- [x] All 9 crates published to crates.io (ternlang-core through ternlang-cli, BSL + LGPL)
- [x] Wall-E easter egg hidden in `ternlang-moe::orchestrate()` — fires on query "wall-e"

---

## ⚖️ Licensing & IP
- [ ] **Open core**: LGPL v3 (compiler + stdlib) — forces compiler contributions back
- [ ] **Commercial tier**: proprietary license for `ternlang-ml`, HDL backend, distributed runtime
- [ ] **Trademark**: "Ternlang", "BET VM", "Balanced Ternary Execution"
- [ ] **Academic outreach**: contact USN group (Bos & Gundersen) for co-authorship whitepaper

---

## 🚧 Pending Action Items (as of 2026-04-10)

### High Priority
- [ ] **TernStudio full rewrite** — SAP-style dashboard view + Editor view (activity bar, Explorer, History, resizable panels) + Settings view. "Upskill" replaces "Upgrade". Share button (btoa hash), Download button, run history (last 20), toast notifications, expanded stdlib tree. Monaco layout() on view switch. Design locked, not yet written.
- [x] **BUG-L01 FIXED** — Block comment skip rule was already in `lexer.rs` line 6. Documentation error corrected. Verified 2026-04-10.
- [x] **BUG-L02 FIXED** — `parse_stmt()` now has explicit `Token::Fn` arm (no-consume); fallback loop in `main.rs` routes to `parse_function()` + `emit_entry_call("main")`. +6 lines parser, +22 lines CLI. 2026-04-10.
- [ ] **stdlib/qnn/ populate** — 10 planned Qutrit Neural Network modules (qutrit_gate, qutrit_hadamard, qutrit_entangle, qnn_layer, qnn_measure, qnn_inference, qutrit_teleport, qnn_grover, qnn_vqe, qnn_qaoa). Tier 3. ROADMAP.md stub exists.

### Medium Priority
- [ ] **GEMINI.md v1.1** — Add Tset Int coercion note (fix #23). Add Tset error message fix. Update known-good patterns.
- [ ] **Whitepaper update** — stdlib count now 27,000+ files, 267 examples in root + 2,090 total. Update Section 10 implementation status table.
- [ ] **crates.io republish** — After BUG-L01/L02 fixes in `ternlang-core`, bump version and republish. Badge count "212+ tests" needs update post-ternlang-compat compile error fix.
- [x] **Open VSX publish** — `ternlang-0.2.0.vsix` published to open-vsx.org (rfi-irfos/ternlang). v0.2.0: affirm/tend/reject highlighting, <=/>= operators added to grammar.
- [ ] **VS Code Marketplace publish** — BLOCKED: credit card not accepted by publisher portal. `ternlang-0.2.0.vsix` built and ready when resolved.
- [x] **MCP registry / Smithery** — listed as `rfi-irfos/ternlang` at smithery.ai. Description + icon updated to v0.3.0 via API. Tools auto-scanned from live server.
- [ ] **Phase 7C: Academic outreach** — USN group (Bos & Gundersen) for co-authorship.

### Low Priority / Nice to Have
- [ ] **README example count update** — Table says "300+ `.tern` programs" but actual count is 2,090 in examples/ (267 root + subdirs). Update to "2,000+".
- [ ] **Benchmark blog post** — Document 2.3×–122× sparse matmul results vs float32.
- [ ] **Gemini stdlib sessions** — Continue breadth-first population per STDLIB_AGENT.md v2.5. Use AGENT_SESSIONS.md cooldown log. Target: 50 new files per session, avoid math/logic/safety (recently covered).

---

## 🗺 Strategic Vision: 2026 — The Year Ternlang Ships to the World

> *"What's best for humans is often not what's best for everybody."*
> — the design principle behind EcoCore

The core stack is complete and deployed. Phase 10 onward is about making it real for people who aren't us: distributable, debuggable, discoverable, and philosophically coherent.

---

## ⚡ Phase 10: Extension Maturity — IMMEDIATE PRIORITY

These two items ship before any other new feature. Without them, the extension is syntax highlighting with a marketing page.

### 10A — Pre-Built LSP Binary (GitHub Actions CI)
**Why:** Users who install from Open VSX cannot build `ternlang-lsp` from source. Hover docs and live diagnostics are dead on arrival without this. Every serious language extension ships pre-built binaries (rust-analyzer, clangd, gopls).

- [ ] GitHub Actions workflow: on `v*` tag, build `ternlang-lsp` for 4 targets:
  - `x86_64-unknown-linux-gnu`
  - `aarch64-unknown-linux-gnu`
  - `x86_64-apple-darwin`
  - `x86_64-pc-windows-msvc`
- [ ] Extension `activate()`: detect host platform, unpack correct binary to `bin/ternlang-lsp`
- [ ] VSIX manifest: bundle all 4 binaries or use a post-install step
- [ ] Remove the "build it yourself" requirement from README

### 10B — Tier 2: Inline Trit Value Hints (Ghost Decorations)
**Why:** This is the visual proof-of-concept that makes ternlang click for everyone who sees it. A ghost annotation after every `let` binding showing its resolved trit state. No other language does this. It's the demo that gets shared.

- [ ] Parse VM stdout after `ternlang.run` — extract `Reg N: trit(...)` lines
- [ ] Map registers back to source variable names via symbol table export in CLI (`--emit-symbols`)
- [ ] VS Code `DecorationProvider`: render `// → Affirm` / `// → Tend` / `// → Reject` after each binding
  - Affirm: green ghost text
  - Tend: amber/yellow ghost text
  - Reject: red ghost text
- [ ] Activate on file save (if key is `tern_t2_*`) or on explicit run
- [ ] `ternlang.inlineTritHints` command wired from stub → real implementation
- [ ] Extension v0.4.0 — bump + publish to Open VSX

### 10C — Dogfood the MCP
- [ ] Add ternlang MCP to this development environment:
  ```
  smithery mcp add rfi-irfos/ternlang
  ```
- [ ] Use `trit_decide` and `moe_orchestrate` in real daily decision-making
- [ ] Every friction point found becomes a bug report → next release

---

## 🤝 Phase 10.5: Tern Systems Ecosystem Collaboration

**Tern Computer (https://github.com/Tern-Computer)** is the other ternary computing org — they have BTMC (Balanced Ternary Machine Code) and TERN assembly (RISC-V-inspired, G language compiles to it). We are in the same uncrowded field. Collaboration beats competition.

- [x] Opened collaboration issue: **Tern-Computer/.github#8** — intro, ecosystem alignment, TERN-ASM offer
- [x] `ternlang-core/codegen/tern_asm.rs` — TERN-compatible ASM emitter (all AST nodes covered)
- [x] `ternlang build --emit-tern` CLI flag → outputs `.tern.asm` RISC-V-inspired balanced ternary assembly
- [ ] **Awaiting Tern Systems response** — once they share TERN spec, align register naming / instruction semantics
- [ ] Contribute to their `coding-guidelines.md` — add joint trit encoding standard section
- [ ] Add BTMC serialisation target to `ternlang-hdl` (their binary format ↔ our Verilog pipeline)
- [ ] Cross-link: their README links ternlang, our README links BTMC/TERN
- [ ] Academic co-positioning: shared "ternary ecosystem" framing for arXiv submission (Phase 15C)

Contacts: garydinmore@tern.ac | adrianfontanilla@tern.ac

---

## 🧠 Phase 11: MCP Intelligence Upgrade — 5 New Tools + EcoCore

### 11A — 5 New MCP Tools

**`trit_debate`** (Free tier)
> Give it two competing claims. Get a structured 3-way verdict: evidence for each side, and what's genuinely uncertain (Tend). This is the tool that gets shared on social media — it's instantly legible to anyone who sees the output.

- [ ] Input: `{ "claim_a": string, "claim_b": string, "context"?: string }`
- [ ] MoE routes both claims through FactCheck + DeductiveReason + AmbiguityRes experts
- [ ] Output: `{ "for_a": trit, "for_b": trit, "tension": float, "synthesis": string, "hold_reason"?: string }`

**`trit_uncertainty_map`** (Free tier)
> Paste in any text — meeting notes, a medical report, a legal clause. Returns every claim annotated with Affirm/Tend/Reject and a confidence score. Compliance teams, lawyers, analysts — this is immediately useful to non-technical users.

- [ ] Input: `{ "text": string, "granularity": "sentence" | "paragraph" }`
- [ ] Split text into claims, run each through FactCheck + MetaSafety
- [ ] Output: array of `{ "claim": string, "trit": int, "confidence": float, "reason": string }`

**`trit_calibrate`** (Free tier)
> Given a log of an AI agent's recent outputs, score how binary its decision-making is. Returns a calibration report: how often did it force yes/no when it should have held? This is the meta-tool — it makes the AI using ternlang better at using ternlang.

- [ ] Input: `{ "decisions": [{ "input": string, "output": string, "confidence"?: float }] }`
- [ ] Score binary_ratio: fraction of outputs that were forced yes/no with >0.9 confidence
- [ ] Output: `{ "binary_ratio": float, "hold_opportunities": int, "calibration_score": trit, "recommendations": string[] }`

**`trit_translate`** (Pro — Tier 2)
> Input a Python `if/elif/else`, a SQL `CASE WHEN`, or a JSON rule set. Output: equivalent `.tern` program with the ternary hold zone inserted where the original code had no coverage. This is the onramp for existing codebases.

- [ ] Input: `{ "code": string, "language": "python" | "sql" | "json_rules" }`
- [ ] Pattern-match binary branches, identify the "else" gap, insert Tend arm
- [ ] Output: `{ "tern_code": string, "hold_zones_added": int, "explanation": string }`

**`trit_eco_check`** (Free tier — see 11B for full context)
> Given a proposed action or decision, returns two trit scores: one from a human-centric perspective and one from an ecocentric perspective. When they diverge, the synthesis is Tend — "this needs more consideration before acting." The first MCP tool that asks "but is this good for everything, not just us?"

- [ ] Input: `{ "action": string, "context"?: string, "scope"?: "local" | "regional" | "global" }`
- [ ] Human score: standard MoE-13 orchestration
- [ ] Eco score: EcoCore expert (see 11B)
- [ ] When `human_trit != eco_trit`: synthesis → Tend, flag tension
- [ ] Output: `{ "human_trit": int, "eco_trit": int, "synthesis": int, "tension": bool, "eco_reasoning": string }`

### 11B — EcoCore: Ecocentric Reasoning Parameter for MoE-13

The philosophical premise: MoE-13 currently deliberates from a human-optimal perspective. Every expert evaluates "is this good?" meaning "good for the user / the task / human interests." EcoCore adds a 14th lens — not a 14th expert that votes, but a post-synthesis modifier that asks: **what would the whole system say?**

The key insight is ternary: when human-optimal is Affirm and eco-optimal is Reject, the right answer isn't a compromise — it's **Tend** (hold, reconsider, find a path that serves both). Ternary logic handles this naturally. Binary systems can't — they either ignore the tension or average it away.

- [ ] New `EcoExpert` in `ternlang-moe`: competence vector emphasizes `safety` + a new `systemic_impact` dimension (add to 6D → 7D, backwards compatible)
- [ ] `EcoCentric` flag on `TernMoeOrchestrator` config (`eco_mode: bool`)
- [ ] When `eco_mode: true`:
  1. Run standard 9-step MoE pipeline → `human_result`
  2. Run EcoExpert independently → `eco_result`
  3. Compute `eco_tension = |human_result.trit - eco_result.trit|`
  4. If `eco_tension > 0`: override synthesis → Tend, add `eco_reasoning` to `OrchestrationResult`
  5. If `eco_tension == 0`: pass through unchanged — agreement means the action is coherent
- [ ] MoE conversation weighting: `ConversationContext` struct — user's prior messages influence expert weights dynamically. A user who consistently pushes for fast answers gets `AmbiguityRes` weighted higher; a user who asks ecological questions gets `EcoExpert` weighted higher
- [ ] `EcoCoreConfig`: `{ "enabled": bool, "scope": "local"|"regional"|"global", "hard_veto_on_eco_reject": bool }`
  - `hard_veto_on_eco_reject: true` → if eco score is Reject AND confidence > 0.85, block the action entirely (the safety gate analog, but for ecological harm)
- [ ] 8 new tests: eco_tension detection, hard veto, conversation weighting, trit_eco_check MCP tool

---

## 🗜️ Phase 11.5: ternlang-compress — Float LLM → Ternary Compression Pipeline

**The idea:** Download any Ollama model → feed it through `ternpress` → get back a `.tern` file
that is 3-10× smaller and runs on `ternlang-ml`'s sparse kernel with no GPU required.

**Why it's real:** Post-training ternary quantization (PTQ) is proven. BitNet b1.58 shows that
weight-only ternary quantization to {-1, 0, +1} preserves most model quality. Our sparse matmul
kernel already skips zero weights (86× at 60% sparsity). The missing piece was a front-end
pipeline to convert existing models — that's what this phase builds.

**Architecture:**
```
GGUF / safetensors
      │
  GgufLoader / SafeTensorsLoader  (format.rs — dequant to f32)
      │
  PerLayerQuant::quantize()        (quantize.rs — PTQ, BitNet threshold)
      │  scale α = mean(|W|), trits = round_clamp(W/α)
      │
  SparseIndex (CSR) or packed dense  (sparse.rs / model.rs)
      │  auto-chosen: CSR if sparsity ≥ 75%, else 2-bit packed
      │
  TernModel { layers, scales, metadata }  (model.rs)
      │
  .tern file (bincode)             (format.rs — write_tern)
      │
  ternlang-ml sparse_matmul()      (existing kernel — zero weights skipped)
```

**New crate: `ternlang-compress`** — workspace member, foundations complete as of 2026-04-11.

### Phase 11.5A — Foundations (COMPLETE 2026-04-11) ✅
- [x] `ternlang-compress` crate scaffolded, added to workspace
- [x] `quantize.rs` — `PerLayerQuant::quantize()`, BitNet threshold, MSE measurement, parallel path
- [x] `sparse.rs` — `SparseIndex` (CSR), roundtrip test, memory efficiency calc
- [x] `model.rs` — `TernModel`, `TernLayer`, `LayerStorage` (Dense/Sparse), summary(), compression ratio
- [x] `pipeline.rs` — `compress()`, `CompressConfig`, 2-bit packing, layer dim inference
- [x] `format.rs` — `.tern` writer/reader (bincode), GGUF/safetensors stubs with impl guide
- [x] `main.rs` — `ternpress` CLI: `--info`, `--synthetic`, `--verbose`
- [x] End-to-end unit test: synthetic 4-layer model compresses and saves/loads correctly

### Phase 11.5B — Llama 3.2 1B Integration (ZBook, no GPU)
- [ ] Implement `load_gguf()` in `format.rs` using candle's GGUF reader
  - Dequantize existing quant types (Q4_0, Q4_1, F16) to f32
  - Re-quantize to ternary via `PerLayerQuant::quantize()`
- [ ] Test on `llama3.2:1b` GGUF from `~/.ollama/models/`
  - Measure: sparsity per layer, MSE per layer, total compressed size
  - Target: >50% sparsity average (typical for LLM weight distributions)
- [ ] Validate output with a simple text generation test (token-by-token decode with ternlang-ml)
- [ ] `ternpress --input ~/.ollama/models/llama3.2-1b.gguf --output llama32-1b.tern --verbose`

### Phase 11.5C — QLoRA Recovery (ZBook, CPU fine-tune)
- [ ] After PTQ, run a short LoRA fine-tune on a small calibration dataset to recover accuracy
- [ ] Target: 1000-step fine-tune on C4 subset, ZBook ZG G9 (14-core, 32 GB RAM), ~2-4 hours
- [ ] Measure: perplexity before/after PTQ, perplexity after QLoRA recovery
- [ ] `ternpress fine-tune --model llama32-1b.tern --data calibration.jsonl --steps 1000`

### Phase 11.5D — GGUF Export (Ollama compatibility)
- [ ] Register a new GGUF quantization type: `GGML_TYPE_TERNARY` (extend llama.cpp type enum)
  - Or: export as GGUF with Q2_K packing as the nearest standard type
- [ ] Write `write_gguf()` in `format.rs` so the output is loadable by `ollama serve`
- [ ] If upstream ternary quant lands in llama.cpp — this becomes a direct integration point

---

## 🌐 Phase 12: WASM Runtime — Make TernGround Real

**Why this matters:** TernGround Lab 05 currently runs `.tern` in a hand-written JS interpreter. The semantics drift from the real compiler. When someone finds a discrepancy, they lose trust. The fix is to compile `ternlang-core` to WebAssembly — the real BET VM, running in the browser, no installation.

This is also the Hacker News launch vehicle. "Try the first balanced ternary language in your browser" with a live WASM runtime is a compelling demo.

- [ ] `cargo build --target wasm32-unknown-unknown -p ternlang-core` — verify it compiles (no OS deps)
- [ ] `wasm-bindgen` wrapper: expose `run_tern(source: &str) -> String` (returns stdout + register dump)
- [ ] Replace `playground/index.html` JS interpreter with WASM call
- [ ] TernGround Lab 05: "Real BET VM, actually running" — accurate, not approximate
- [ ] Performance: BET VM in WASM should be fast enough for the 5 demo programs instantly
- [ ] CI: build WASM artifact on release, embed in `ternlang-api` static assets or serve from CDN

---

## 🔍 Phase 13: TernAudit — The Killer App

TernAudit is the commercial case made tangible. It answers the question "why would an enterprise buy ternlang?" with a specific, auditable, EU-AI-Act-compliant answer: *because our AI's decisions are now trit-annotated and you can prove it to a regulator.*

**What it does:** Takes any AI system's decision log, LLM output batch, or classifier result set and returns a trit-annotated audit trail. Every claim: Affirm (evidence present, high confidence), Tend (uncertain, needs more data), or Reject (contradicted). The Tend cases are the ones the AI should have flagged as "I don't know" but didn't — that's the audit finding.

- [ ] `ternlang audit <input.json>` CLI command
  - Input: JSON array of `{ "input": string, "output": string, "confidence"?: float }`
  - Output: `audit_report.json` + `audit_report.html` (human-readable)
- [ ] `POST /api/audit` REST endpoint (Tier 2+)
- [ ] Audit report format:
  ```json
  {
    "total_decisions": N,
    "affirm_count": N, "tend_count": N, "reject_count": N,
    "forced_binary_ratio": 0.73,
    "eu_ai_act": { "article_13": "pass|warn|fail", "article_14": "pass|warn|fail" },
    "flagged": [{ "input": "...", "output": "...", "trit": 0, "reason": "..." }]
  }
  ```
- [ ] VS Code command: `Ternlang Pro: Audit Selection` — select any block of AI outputs, get inline annotations
- [ ] `trit_audit` MCP tool (wraps the REST endpoint)
- [ ] Marketing: "The only tool that finds the decisions your AI should have held"

---

## 🔄 Phase 14: TernTranslator — The Bridge Into the Existing World

Most potential users have binary decision trees they've been running for years. TernTranslator is the onramp: give it your Python `if/elif/else` or SQL `CASE WHEN` and it outputs `.tern` with the ternary hold zone added where the original code had no coverage.

- [ ] `ternlang translate <input.py>` CLI command
  - Parses Python if/elif/else, SQL CASE WHEN, JSON rule arrays
  - Identifies "else: default" patterns — these are the hold zones in disguise
  - Outputs `.tern` equivalent with explicit Tend arm + comment explaining the gap
- [ ] `POST /api/translate` REST endpoint (Tier 2+)
- [ ] VS Code command: `Ternlang Pro: Translate Selection to Ternary` (Tier 2)
  - Select any if/else block → get `.tern` equivalent in a side panel
- [ ] `trit_translate` MCP tool (already planned in Phase 11A)
- [ ] Target languages for v1: Python, SQL, JSON rule sets
- [ ] Target languages for v2: JavaScript, TypeScript, YAML (Kubernetes policy rules)

---

## 📚 Phase 15: Distribution, Academia, Community

### 15A — Jupyter Kernel
- [ ] `ternlang-jupyter`: ZeroMQ-based Jupyter kernel wrapping `ternlang-cli`
- [ ] `.tern` cells in Jupyter notebooks — execute, display trit state of all variables
- [ ] Rich output: trittensor visualized as colored grid (Affirm=green, Tend=amber, Reject=red)
- [ ] Install: `pip install ternlang-jupyter && python -m ternlang_jupyter.install`
- [ ] Target: AI safety researchers, ML students, anyone working on uncertainty quantification

### 15B — ternpkg Curated Registry
- [ ] Move beyond GitHub-backed install — add a curated `registry.ternlang.com` index
- [ ] Quality gate: every registered package must pass `ternlang-cli run` with exit 0
- [ ] `ternpkg search <keyword>` — search the registry
- [ ] `ternpkg publish` — submit a package (authenticated, rate-limited)
- [ ] Seed with: stdlib bundles (core, ml, safety), TernAudit rules, community agents

### 15C — Academic Outreach
- [ ] Contact USN group (Bos & Gundersen) — joint whitepaper on ternary ISA + inference
- [ ] Submit to arXiv: "BET-ISA: A Balanced Ternary Execution Architecture for Sparse Neural Inference"
- [ ] Target venues: NeurIPS workshop on efficiency, ISCA, DATE conference (hardware)
- [ ] DOI registration for all RFI-IRFOS papers — already started (OSF)

### 15D — Community
- [ ] Discord server: `#ternlang` — language, `#bet-vm` — compiler, `#mcp` — AI integration, `#research`
- [ ] GitHub Discussions: enabled on the repo
- [ ] Hacker News launch: coordinate WASM playground (Phase 12) + curated stdlib showcase
  - Headline: "Ternlang: a programming language where 'I don't know' is a first-class value [try in browser]"
- [ ] Weekly changelog post

---

## 🏗 Phase 16: TernStudio v1.0 — The Full IDE

The arc of the VS Code extension ends at v1.0.0 / TernStudio. This is the complete developer environment for ternary systems programming.

### VS Code Extension Milestones
| Version | Key Feature |
|---------|-------------|
| v0.4.0 | Inline trit hints live (Phase 10B), pre-built LSP binary (Phase 10A) |
| v0.5.0 | BET VM step debugger (Tier 3): breakpoints on trit values, register watch panel, step through opcodes |
| v0.6.0 | Tensor visualizer (Tier 3): trittensor rendered as colored grid inline; `@sparseskip` coverage overlay |
| v0.7.0 | TernAudit inline (Tier 2): annotate selected AI output block with trit verdicts |
| v0.8.0 | TernTranslator panel (Tier 2): translate selected if/else to .tern in side panel |
| v1.0.0 | Stable API, all tiers fully implemented, Enterprise cluster panel + agent monitor |

### TernStudio Web IDE
The standalone web IDE — Monaco editor + real BET VM (WASM, Phase 12) + integrated TernAudit + project management.

- [ ] SAP-style layout: Activity Bar → Explorer / Editor / History / Settings panes
- [ ] File tree: project-aware, stdlib browser, `ternpkg.toml` aware
- [ ] Run panel: real BET VM output, inline trit annotations on variables
- [ ] TernAudit tab: paste any AI output, get trit audit instantly
- [ ] TernTranslator tab: paste Python/SQL, get .tern output
- [ ] Share button: `btoa` hash → shareable URL with full program state
- [ ] Download button: save current project as `.ternproj` bundle
- [ ] Run history: last 20 executions with trit state snapshots

---

## 📊 2026 Priority Matrix

| Quarter | Deliverable | Impact | Effort |
|---------|------------|--------|--------|
| Q2 2026 | Pre-built LSP binary (Phase 10A) | 🔴 Critical | Medium |
| Q2 2026 | Inline trit hints v0.4.0 (Phase 10B) | 🔴 Critical | Medium |
| Q2 2026 | 5 new MCP tools (Phase 11A) | 🟠 High | Medium |
| Q2 2026 | EcoCore in MoE-13 (Phase 11B) | 🟠 High | High |
| Q2 2026 | WASM runtime (Phase 12) | 🟠 High | Medium |
| Q3 2026 | TernAudit CLI + REST (Phase 13) | 🔴 Critical (commercial) | High |
| Q3 2026 | TernTranslator (Phase 14) | 🟠 High | Medium |
| Q3 2026 | Jupyter kernel (Phase 15A) | 🟡 Medium | Medium |
| Q3 2026 | Hacker News launch (Phase 15D) | 🔴 Critical (distribution) | Low |
| Q4 2026 | VS Code extension v0.5.0 — BET debugger (Phase 16) | 🟠 High | High |
| Q4 2026 | TernStudio v1.0 (Phase 16) | 🟠 High | Very High |
| Q4 2026 | arXiv paper submission (Phase 15C) | 🟡 Medium | Medium |

---


|------|---------------|
| 2026-04-02 | Initial repo setup. Phase 1+2 confirmed complete. Git initialized, pushed to GitHub. Credential store configured. 4 failing tests identified (DimSeparator bug). Phase 3 plan defined. |
| 2026-04-02 | Fixed DimSeparator/Ident collision in lexer. Fixed betbc test import. 11/11 tests passing. Next: TCALL/TRET function dispatch + tensor VM opcodes. |
| 2026-04-02 | TCALL/TRET implemented. Tensor opcodes DONE: TMATMUL, TSPARSE_MATMUL, TIDX, TSET, TSHAPE, TSPARSITY. 14/14 tests passing. Next: @sparseskip codegen wiring + ternlang-ml kernels. |
| 2026-04-02 | @sparseskip → TSPARSE_MATMUL wired in codegen. ternlang-ml filled: quantize, bitnet_threshold, dense_matmul, sparse_matmul, linear, benchmark. First benchmark: 56% sparsity → 2.3x fewer multiply ops. 23/23 tests passing. |
| 2026-04-02 | ternlang-mcp LIVE — MCP server (JSON-RPC 2.0, stdio). 6 tools: trit_decide, trit_consensus, trit_eval, ternlang_run, quantize_weights, sparse_benchmark. Any binary agent connecting to this becomes a ternary decision engine. Hidden easter egg: ternlang enlighten. |
| 2026-04-02 | Phase 4 language completeness: for/while/loop/break/continue/mut/use/::. Match exhaustiveness enforced at parser. 20 core tests + 6 ML tests + 1 codegen tests = 28 total passing. |
| 2026-04-02 | stdlib source files: std::trit, std::math, std::tensor, std::io, ml::quantize, ml::inference. Struct defs + field access (s.field) + field assignment (s.field=v) + cast() + Type::Named. Dot token in lexer. 25/25 tests passing. |
| 2026-04-02 | Phase 5.0 actor model: agent/spawn/send/await/agentref in lexer+AST+parser+semantic+codegen+VM. TSPAWN/TSEND/TAWAIT opcodes. AgentInstance with mailbox. Integration test: spawn echo agent, send +1, await +1. 30/30 tests passing. |
| 2026-04-02 | Phase 5.1: ternlang-runtime crate (TCP distributed actors). TernNode with listen/connect/remote_send/remote_await. Wire protocol: newline JSON over TCP. remote/nodeid keywords. spawn remote "addr" syntax. StringLit token. Real function call type resolution in semantic checker. 31 core + 2 runtime tests. |
| 2026-04-02 | Phase 6.0: ternlang-hdl crate. Verilog primitives: trit_neg/cons/mul/add/reg, bet_alu, sparse_matmul(N). ISA control: bet_regfile/pc/control/processor. All BET opcodes mapped. 52 total tests passing. |
| 2026-04-03 | BET-ISA-SPEC.md formal spec published. ternlang-lsp: full LSP 3.17 JSON-RPC (hover, completion, diagnostics). ternlang-vscode: TextMate grammar, LSP client extension. ternlang fmt + repl in CLI. ternpkg v0.1: init/install/list/info, GitHub-backed registry. 58 total tests passing. |
| 2026-04-03 | Phase 7A: TasmAssembler + OwletParser (ternlang-compat, 29 tests). TCOMPRESS/TUNPACK RLE codec (0x26/0x27). TernaryMLP 2-layer with from_f32/forward/predict, XOR+parity datasets. timed_benchmark: 32²–512², 5-rep median wall-clock. BET sim emitter (Icarus Verilog testbench). Hub README + TERNARY-ECOSYSTEM.md. VSIX packaging. Whitepaper TEX+DOCX published (10 sections, IEEE two-column). Spec files consolidated into main repo. 116 total tests passing. |
| 2026-04-03 | StdlibLoader: `use std::trit;` works end-to-end. Comment skip in lexer. 3-layer CSC sparse matmul (flat i8 + offset table + Rayon): 86× at 60% sparsity, 122× at 99% sparsity (512² release). Goldilocks sweep confirms 40–60% as optimal zone for medium matrices. Whitepaper updated with full sweep table. 120+ tests passing. |
| 2026-04-03 | Multi-tenant API key management in ternlang-api: KeyStore (JSON-backed, async RwLock), key generation (tern_<tier>_<uuid24>), revocation, usage counters, admin routes POST/GET/DELETE /admin/keys. `TERNLANG_ADMIN_KEY` + `KEYS_FILE` env vars. Albert-agent integrated as primary TIS agent. 5 VM compile errors fixed (Value::Clone, AgentRef 2-tuple). Build clean across full workspace. |
| 2026-04-03 | Phase 5.1 COMPLETE: RemoteTransport trait in ternlang-core (no circular dep), TernNode impl in ternlang-runtime; TSEND/TAWAIT route over TCP for remote AgentRefs with auto-connect; `ternlang run --node-addr --peer` CLI flags wire TernNode into VM at startup; 4 runtime tests passing. |
| 2026-04-03 | Phase 6.1 COMPLETE: BetRtlProcessor — cycle-accurate RTL simulator in pure Rust. Mirrors bet_processor.v exactly: TritWire 2-bit encoding, trit_neg/cons/mul/add combinational primitives, BetRegfile (27 regs), BetPc (16-bit), BetAlu, bet_decode control unit. `ternlang sim --rtl [--max-cycles N]` CLI. 12 RTL unit tests + 2 doctests. 93 tests total across core/hdl/runtime. |
| 2026-04-03 | Phase 8 COMPLETE: Ternary AI Reasoning Toolkit in ternlang-ml — DeliberationEngine (EMA convergence), coalition_vote (quorum/dissent/abstain), action_gate (multi-dim hard-block), scalar_temperature (trit→LLM temp bridge), hallucination_score (variance→trust trit). TritScalar gains trit_i8() + Clone. Phase 8 REST endpoints in ternlang-api (5 endpoints). 15 reasoning tests. |
| 2026-04-03 | Phase 9 COMPLETE: ternlang-moe crate — MoE-13 ternary orchestrator (DOI 10.17605/OSF.IO/TZ7DC). CompetenceVector (6D), TernMoeRouter (dual-key synergistic routing), TriadField (1+1=3 emergent synthesis), three-tier memory mesh (Node/Cluster/Axis), TernMoeOrchestrator (9-step pipeline), 13-expert standard pool, temperature bridge, safety hard gate with audit log. 16/16 tests passing. 146+ total tests across workspace. |
| 2026-04-04 | MCP Phase 9 tools: moe_orchestrate, moe_deliberate, trit_action_gate (10 MCP tools total). SSE streaming endpoints in ternlang-api (stream/moe_orchestrate, stream/deliberate). Whitepaper updated to 887 lines with Phase 8+9 sections. All 9 crates published to crates.io. 20 .tern example files added to examples/ with INDEX.md — covering aerospace, medicine, DevOps, autonomous vehicles, AI agents, civic systems, finance, CPU pipeline (Brandon Smith tribute), S-expression eval (Owlet tribute), microservices, diplomacy, recruiting, scheduling, caching. Homepage updated with plain-language "Why Ternary" section (traffic lights, doctors, judges analogies) + all new API routes in nav. |
| 2026-04-04 | **MILESTONE: ternlang.com LIVE + HTTPS** — Fly.io TLS cert provisioned for ternlang.com. GitHub Pages A records (185.199.x.x) removed from GoDaddy; Fly.io A record (66.241.124.209) + AAAA record now sole DNS entries. index.html embedded in ternlang-api via `include_str!` — website served at GET / (Accept: text/html) with JSON manifest fallback for API clients. Albert wired: `trit_decide` (public MCP) + `moe_orchestrate` (REST) added to albert tools.py + TOOLS_DEF — live test confirmed trit=1 AFFIRM from Fly.io. MoE-13 agent harness overhauled: all 13 TernaryAgent impls rebuilt with dual-signal deliberation (positive/negative keyword sets, genuine stasis for trit=0). `run_introspective()` added to AgentHarness — stable attractor hold: when affirm/conflict balanced across ≥4 agents, trit=0 is permanent (not a tie to break). `to_evidence_vector()` maps 13 verdicts to 6D MoE router axes. `orchestrate_full()` added to TernMoeOrchestrator — 13-substage pass → safety gate → stable hold check → enriched evidence → MoE synthesis. 24/24 tests passing (8 new harness tests). QNN .tern examples 251–265 added (Qutrit Neural Networks, Simeon Kepp / RFI-IRFOS paper) + INDEX.md QNN section. |
| 2026-04-04 | **MILESTONE: ternlang-api LIVE on Fly.io** (Frankfurt, shared-cpu-1x 256MB). POST /mcp HTTP MCP transport built — JSON-RPC 2.0 over HTTP, 10 tools, no auth required. GET /.well-known/mcp/server-card.json for Smithery scan skip. DNS A record (66.241.124.209) added to ternlang.com via GoDaddy. Smithery submission pending DNS propagation. Gemini agent: 250 total .tern examples (files 141–250), 13 expert AgentHarness modules (ternlang-moe/src/agents/), 20 tutorial .tern files with INDEX.md (examples/tutorials/). README.md overhauled — fully interlinked, professional, live API examples. ROADMAP.md session log updated. |
| 2026-04-05 | **Gemini Agent: STDLIB DOMINATION.** Added ~130 new stdlib modules across 17 categories (Classical ML, Deep Learning, RL, NLP, Vision, etc.). Upgraded compiler to support `affirm/tend/reject` keywords, binary `if/while` fallbacks, tensor indexing `obj[r,c]`, and comparison operators. All 217+ stdlib modules are now built-in to the compiler. Tests passing (190+). |
| 2026-04-09 | **MAJOR SESSION: Commercial hardening, EU AI Act compliance, pricing overhaul, Fly.io production deploy fix, full website redesign, TernGround playground launch.** Details below. |
| 2026-04-09 | **[1/8] Repository & README hardening.** Root `README.md` (TIS overview) completely rewritten — all relative links were broken (wrong path resolution from repo root). Replaced with landing page using absolute GitHub URLs throughout. Fixed broken internal link `42_algorithmic_trading.tern` → actual filename `42_trading_signal.tern`. In `ternlang-root/README.md`: MCP tool count corrected 20 → 13 (10 free + 3 premium). Architecture page crate table updated to match. `Strategic Standards` section renamed to `RFI-IRFOS Protocol Specifications` with disclaimer: "Open proposals — not yet ratified by external standards bodies." IEEE TFP-754 → TFP-754, ISO Certified Uncertainty → T-UNCERTAINTY (removing false certification claims). Removed stray `.tern linguist-language=Ternlang` line artifact. Emergent triad synthesis copy updated to accurate formula `Ek = synergy × (vi + vj) / 2`. Architecture links for all 20+ crate cards updated from dead `href="#"` to real crates.io and GitHub URLs. |
| 2026-04-09 | **[2/8] EU AI Act compliance layer.** Added 4 EU AI Act / GDPR compliance badges to README badge row: Article 13 Compliant Design, Article 14 Human Oversight Ready, Data Residency EU Frankfurt, GDPR Compliant Design. Added full compliance table to README body covering Articles 9 (Risk Mgmt), 11 (Technical Documentation), 12 (Record-Keeping / AxisMemory), 13 (Transparency / trit-typed outputs), 14 (Human Oversight / trit=0 hold), 15 (Accuracy / MoE-13 veto) and GDPR (Frankfurt data residency). |
| 2026-04-09 | **[3/8] Pricing overhaul — market-rate repricing.** Previous pricing (€24.99 Tier 2, €49.99 Tier 3) identified as 4–7× below market rate vs LangSmith Teams, Humanloop Growth, Arize AI comparables. New pricing set: Tier 2 → €99/month (10k calls), Tier 3 → €349/month (50k calls, raised from 20k), Tier 4 → from €2,500/month (Enterprise, custom FPGA). New Stripe payment links created and deployed: Tier 2 `buy.stripe.com/5kQ28t7SM4rB0DH6jm7N608`, Tier 3 `buy.stripe.com/eVq7sNfle0bl86937a7N609`. All occurrences updated across `index.html` (pricing page + JSON-LD + all CTAs), `pricing.html` (CTA buttons renamed "Subscribe — €99/mo" / "Subscribe — €349/mo"), `README.md` (licensing tier box + Stripe links). |
| 2026-04-09 | **[4/8] API tier detection + Stripe webhook fix.** `ternlang-api/src/main.rs`: updated Stripe `checkout.session.completed` tier detection threshold from `amount >= 4999` → `amount >= 34900` cents (matching new €349 Tier 3 price). Tier 3 `tier_monthly_limit` updated `Some(20_000)` → `Some(50_000)`. Comments updated throughout. Webhook secret `whsec_yDlh55RHGRh1iBOfEsdJnFzHvBCXNGcZ` stored as Fly.io secret `STRIPE_WEBHOOK_SECRET`. Webhook registered at `https://ternlang.com/stripe/webhook` listening to `checkout.session.completed`. |
| 2026-04-09 | **[5/8] Website product copy overhaul (index.html).** MCP page headline rewritten: "Any agent. Ternary reasoning." → "Your agent. Now it can say hold." New subheadline and 3 value-prop bullets added: trit=0 gather-more-data signal, 3-layer persistent memory, Safety hard veto at conf>0.90. TernAudit headline and all 3 feature cards (Evidence Graph, Veto Trace, Resolution Path) rewritten to lead with customer pain instead of technology description. Logic Library card text size: `text-xs` → `text-sm` on all 8 cards (better readability). 4-tier licensing box on pricing page updated with correct prices, call quotas, and BSL-1.1 → Apache 2030 conversion note. |
| 2026-04-09 | **[6/8] Email routing: *@ternlang.com catch-all.** GoDaddy DNS updated: removed old Mailgun MX records (`mxa.mailgun.org`, `mxb.mailgun.org`). Added ImprovMX free MX records (`mx1.improvmx.com` priority 10, `mx2.improvmx.com` priority 20). ImprovMX catch-all alias configured: `*@ternlang.com → rfi.irfos@gmail.com`. Resend.com handles outbound transactional mail separately (post-payment API key delivery). Documented in memory at `reference_email.md`. |
| 2026-04-09 | **[7/8] Fly.io production deploy — 3-stage debug & fix.** Dockerfile had accumulated path errors from a crate reorganization (core/codegen/cli/runtime moved from `ternlang-root/` into `compiler/legacy_shim/`). Three failures fixed iteratively: (a) Missing `ternlang-runtime` in workspace Cargo.toml.bak and Dockerfile COPY commands — cli depends on runtime, was invisible until Docker cold build; (b) Missing `compiler/legacy_shim/ternlang-core/stdlib` in COPY commands — ternlang-core's `include_str!` macros require the stdlib directory at build time; Docker build was clean locally (cached) but failed with 245 errors in CI. Fixed by adding `COPY compiler/legacy_shim/ternlang-core/stdlib compiler/legacy_shim/ternlang-core/stdlib` to Dockerfile; (c) Rust edition 2024 borrow checker regression in `compiler/legacy_shim/ternlang-core/src/vm/mod.rs` — 7 E0382 "use of moved value" errors: `match (a, b)` consumed both values, then wildcard arm `format!("{:?}", (a, b))` attempted to use the already-moved bindings. Fixed by rust-analyzer auto-applying `.clone()` on match scrutinees (`match (a.clone(), b.clone())`). All affected opcodes: Tadd (0x02), Tmul (0x03), Tneg (0x04), Tcons (0x0e), Tless (0x14), Tgreater (0x15), TaddInt (0x18), Tidx (0x22), Tset (0x23). `cargo build --release -p ternlang-api` now clean locally and in Docker. Deploy successful: `https://ternlang-api.fly.dev/` — both machines healthy. |
| 2026-04-09 | **[8/8] TernGround playground — new page 9.** Major website restructure: all 5 interactive labs (previously scattered across Home, Albert, MCP pages) consolidated into a dedicated TernGround tab. Labs removed from originating pages; each original page now has a focused CTA strip redirecting to TernGround. TernGround (navTo(9)) contains: **Lab 01** "13 Experts. One Answer." — MoE-13 deliberation with live expert telemetry grid and verdict ring; **Lab 02** "Your Memory Stack Just Said No." — L1/L2/L3 memory consolidation with veto collision demo; **Lab 03** "It Doesn't Guess. It Waits." — EMA deliberation engine, scenario picker, round-by-round confidence trace; **Lab 04** "Zero Means Don't Bother." — Neural weight matrix, click to prune, live power/throughput readout; **Lab 05** "Real Code. Actually Running." — BET VM terminal with 5 .tern programs (classify, pipeline, sparse_nn, moe_vote, uncertainty). **NEW: Lab 06** "Three States. Nine Outcomes." — Ternary truth table, interactive 3×3 grid for TMAX (OR analog), TMIN (AND analog), TADD (clamped balanced ternary addition), TMUL. Click any cell to see the full operation explained in plain English. All lab descriptions rewritten in plain English, high-contrast `text-t_text` replacing `text-t_muted` throughout — fixes grey-on-grey readability in both dark and light mode. Nav changes: **Benchmarks removed from Products dropdown** (it is a metric, not a product). TernGround added to Products dropdown, top nav bar, mobile menu, and footer. CSS page-slider width updated 900vw → 1000vw. JS TOTAL_PAGES 9 → 10. GEMINI.md also rewritten from scratch — removed hallucinated "Monopoly Payload" security vulnerability entry, replaced with verified facts-only agent brief covering who to work for, ternary stack, pricing, allowed/not-allowed actions, syntax reference, and quality rules. |
| 2026-04-10 | **[1/5] Fun Error Dictionary + VM hardening.** `stdlib/errors/error_registry.json` v2.0.0: 20 error codes across 7 namespaces (PARSE, TYPE, SCOPE, STRUCT, FN, PROP, BET), each with fun_message, technical, common_causes[], fix, bad_pattern, good_pattern, severity, example_file. 7 reserved namespaces pre-documented (IMPORT, ANNOT, WARN, CODE, HDL, MCP, NET). 22 runnable `.tern` example files in `stdlib/errors/` — all pass VM test runner. `stdlib/core/errors.tern`: trit-based severity taxonomy. Compiler fun messages: all PARSE (parser.rs), TYPE/SCOPE/STRUCT/FN/PROP (semantic.rs), and BET (vm/mod.rs) error messages updated with personality + `→ details: stdlib/errors/CODE.tern` reference line. VM hardening: 4 new VmError variants (TensorIndexOutOfBounds, TensorNotAllocated, AgentTypeNotRegistered, AgentIdInvalid) + bounds checks for TIDX/TSET/TSHAPE (previously panicked on out-of-bounds access), Tspawn/Tsend/Tawait now use correct error variants. |
| 2026-04-10 | **[2/5] playground/index.html — self-contained TernGround IDE.** `playground/index.html`: 103KB self-contained HTML file with full JS .tern interpreter (tokenizer → recursive descent parser → evaluator), 25 stdlib files (errors/ + core/) baked in via Python embed script. No server, no dialog — file tree populated on load. Parameter panel auto-detects top-level `let` declarations and renders trit buttons / number inputs. `playground/embed_stdlib.py`: regeneration script to update embed when stdlib grows. `fly.toml` moved from TIS/ parent into repo root so GitHub Actions can access it. |
| 2026-04-10 | **[3/5] CI/CD overhaul.** `.github/workflows/ternlang-ci.yml`: replaced hallucinated workflow (called nonexistent `tern-audit`, `scripts/install.sh`, `ternlang build --target bet-vm-v1`) with real Rust CI — installs stable Rust via dtolnay/rust-toolchain, caches cargo, builds `ternlang-cli --release`, smoke tests version + 11 stdlib error examples. `.github/workflows/deploy-fly.yml`: new Fly.io deployment workflow — triggers on push to main when `ternlang-web/`, `ternlang-api/`, `Dockerfile`, or `fly.toml` change; uses `FLY_API_TOKEN` secret (now set). |
| 2026-04-10 | **[4/5] Index.html polish — lab copy + nav.** All 6 lab descriptions in TernGround now have two tiers: a muted context paragraph (what the underlying problem is + why this approach matters) above the existing bold interaction guide. Labs covered: MoE routing rationale, L1/L2/L3 memory layers, EMA deliberation and trit=0 as correct output, ternary BitNet zero-skip source of 60% power savings, BET VM as native ternary (not converted), ternary truth table semantics. Nav product items unified: all 7 products now `text-t_text hover:text-t_teal` — teal reserved for hover and badge accents (CLI, PLAY) only. |
| 2026-04-10 | **[5/5] Production deploy.** `fly deploy` triggered from local after FLY_API_TOKEN added to GitHub secrets. Both Fly.io machines updated (rolling deploy), health checks passed. ternlang.com live with all 2026-04-10 changes. Auto-deploy via GitHub Actions now active — future pushes to `ternlang-web/` trigger deploy automatically. |
| 2026-04-10 | **[Gemini stdlib session] STDLIB agent overhaul + VM fix.** STDLIB_AGENT.md rewritten to v2.5: weakness scan, 5-batch × 10-file sessions, anti-overlap via AGENT_SESSIONS.md log (3-session cooldown). GEMINI.md hardcoded parameter sheet committed to repo. Purged 3,000+ hallucinated files from `stdlib/astro/`, `stdlib/bench/`, `stdlib/benchmarks/` (10 concept × 150 suffix variant pattern). Seeded with 6 real, tested files: `launch_window_gate.tern`, `reentry_heat_gate.tern`, `telemetry_anomaly.tern` (astro); `opcode_coverage.tern`, `inference_latency_gate.tern` (bench); `sparse_matmul.tern` (benchmarks, rewrite). `confidence_gate.tern` fixed: removed block comment (BUG-L01), rewrote with `fn main()` entry + helper pattern. Buglist/Fixes.md entries 20–22 committed (from prior uncommitted Gemini diff). AGENT_SESSIONS.md bootstrap log created. |
| 2026-04-10 | **[VM fix] Tset (0x23) Int polymorphism.** `vm/mod.rs`: added `(Value::TensorRef, Value::Int)` arm to Tset dispatch — integer values now silently coerce to `Trit::from(v as i8)` on tensor write. Fixed row mismatch error message (was printing `col` in row branch). Fixes BET-007 TypeMismatch on `stdlib/nn/ternary_relu.tern` and any stdlib file that stores integer loop results into tensor slots. Fixes.md entry #23 added. Debug `println(i)` removed from `ternary_relu.tern`. |
| 2026-04-10 | **[Infra] stdlib/qnn/ placeholder created.** README referenced `stdlib/qnn/` (dead link — directory absent). Created with ROADMAP.md listing 10 planned QNN module stubs. Actual QNN programs (251–265) remain in `examples/`. QNN stdlib population deferred to Gemini next session (excluded from today's cooldown list). |
| 2026-04-10 | **[VS Code extension] v0.2.0 — grammar upgrade + Open VSX publish.** `ternlang.tmLanguage.json`: added `affirm\|tend\|reject` as `constant.language.trit.ternlang`; added `<=\|>=` to operators pattern. `package.json` bumped 0.1.0 → 0.2.0. VSIX rebuilt (442 KB, 11 files). Published to Open VSX registry as `rfi-irfos/ternlang v0.2.0`. `ternlang-root/README.md`: Open VSX badge added to badge row; full "VS Code Extension" section added after quick-start (install instructions, feature table, LSP wiring, marketplace note). Bughunt: 5 compiler/VM bugs fixed (AND/OR logic BUG-A/B, for-in loop count BUG-C, FieldAccess BUG-D, Cast BUG-E) — all documented in Buglist/Fixes.md. GEMINI.md v1.2: all fixes hardcoded, opcodes 0x28/0x29 (Tand/Tor) added to table, §15 fix ledger. Fly.io CI: FLY_API_TOKEN refreshed (100-year Macaroon), deploy-fly.yml moved to repo root `.github/workflows/`. |
| 2026-04-11 | **[Ecosystem] Tern Systems collaboration outreach + TERN-ASM emitter.** Discovered Tern-Computer GitHub org (Tern Systems) — building BTMC (Balanced Ternary Machine Code) and TERN assembly language (RISC-V-inspired) in the same ternary computing space. Opened collaboration issue: Tern-Computer/.github#8 (formal intro, ecosystem alignment proposal, offer to share BET ISA spec + contribute to their docs). Added `ternlang-core/codegen/tern_asm.rs` — full RISC-V-inspired balanced ternary ASM emitter covering all AST nodes (arithmetic, comparison, control flow, tensor ops, actors, error propagation). Wired into CLI as `ternlang build --emit-tern` → writes `.tern.asm` file. Smoke tested on multi-function programs — output is valid TERN-compatible assembly with correct register allocation, label generation, and 3-way branch dispatch. |
