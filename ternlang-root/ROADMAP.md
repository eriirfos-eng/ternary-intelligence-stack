# Ternlang Roadmap: Bridging the Ternary Software Deficit
### Project: Ternary Intelligence Stack (TIS) | RFI-IRFOS
**Current Version:** v1.3.0 (dev) / v1.2.1 (published)
**Last Updated:** 2026-04-29
**Repo:** https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-
**Local:** ~/Desktop/Ternary Intelligence Stack (TIS)/

---

## Development Protocol (READ THIS FIRST)
- **Always work locally AND push to GitHub** after every meaningful session
- Push command (credentials stored): `git push origin main` from inside `Ternary Intelligence Stack (TIS)/`
- Pull before starting a new session: `git pull origin main`
- Update this ROADMAP at the end of every session with current status
- The AI engineering agent (Gemini CLI) maintains context via the TIS-specific brief in `GEMINI.md` — ensure all foundational mandates are followed each session.

###  Whitepaper Update Protocol
The academic whitepaper (`whitepaper/ternlang-whitepaper.tex` + `whitepaper/ternlang-whitepaper.docx`) is a **living document**. Update it whenever a phase produces measurable results:
- New opcodes or VM features → update Section 4 (ISA) and Section 10 (implementation status)
- New benchmark numbers → update Section 5 (sparse inference) tables
- New crates or test counts → update Table 8 (implementation status)
- Rebuild DOCX: `python3 whitepaper/build_docx.py`
- Rebuild PDF: `cd whitepaper && pdflatex ternlang-whitepaper.tex` (requires texlive)

---

═══════════════════════════════════════════════════
# PART I — FOUNDATION (Phases 1–9) — ALL COMPLETE
═══════════════════════════════════════════════════

## ✅ Phase 1: Core Language & VM Stability — COMPLETE
- [x] **Trit Primitives**: `-1`, `0`, `+1` logic (Sum/Carry, Neg, Mul) — fully tested
- [x] **Lexer**: Tokenize ternary-specific keywords (`trit`, `trittensor`, `?`, `sparseskip`)
- [x] **Skeletal Parser**: Parse basic expressions and `IfTernary` (`if ?`)
- [x] **BET VM Core**: Stack, unbounded register file, carry reg, 2-bit packing (`0b01=-1`, `0b10=+1`, `0b11=0`)
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

## ✅ Phase 3: TritTensors & Sparse Inference — COMPLETE
**This is the commercial differentiator. The AI inference story.**

- [x] **Fix `DimSeparator` bug** in lexer (remove dedicated token, handle as `Ident("x")` in type parser) — 11/11 tests passing
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
- [x] **First benchmark result**: 56% weight sparsity → **2.3× fewer multiply ops** vs dense (measured in Rust release mode)
- [x] **Wall-clock timing benchmark**: 5 sizes (32²–512²), 5-rep median
- [x] **CSC sparse matmul**: Compressed Sparse Column precompute — branch-free inner loop; at 25% sparsity: **4.8–6.9× faster**; at 60% sparsity (BitNet-realistic): **8–14× faster** than dense in release mode
- [x] **BitNet b1.58 benchmark**: explicit 60% sparsity, release mode — **86×** at 512² (3-layer CSC kernel)
- [x] **Goldilocks sparsity sweep**: 9 sparsity levels × 5 sizes — peak **122×** at 99% sparsity 512²; goldilocks zone confirmed at 40–60% sparsity (20–57× on medium matrices)
- [x] **TernaryMLP**: 2-layer MLP (from_f32, forward, predict, XOR/parity datasets, accuracy eval) — full inference path tested end-to-end
- [ ] **Publish sparse matmul benchmark** — write blog post / README section comparing vs float32

> ⚠️ Benchmark note: all figures measured in Rust release mode on x86 simulation. Not validated on real ternary hardware (per Dr. Bos / USN feedback, 2026-04-13).

---

## ✅ Phase 3.5: MCP Integration — COMPLETE
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

---

## ✅ Phase 4: Language Completeness — COMPLETE
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

## ✅ Phase 5: Actor Model & Distributed Agents — COMPLETE
- [x] **Lexer/Parser/AST**: `agent`, `spawn`, `send`, `await`, `agentref` — all done
- [x] **Local Actor Runtime**: AgentInstance + mailbox, `TSPAWN`/`TSEND`/`TAWAIT` opcodes, synchronous dispatch
- [x] **Integration test**: spawn identity-agent, send +1, await → +1 ✓
- [x] **Distributed Runtime** (Phase 5.1): `RemoteTransport` trait in core (no circular dep), `TernNode` impl in runtime; TSEND/TAWAIT route over TCP for remote AgentRefs; auto-connect on first use; 4 runtime tests passing
- [x] **`remote`/`nodeid`** keywords: `--node-addr` + `--peer` CLI flags; `TernNode` injected into VM via `set_remote(Arc<dyn RemoteTransport>)`

---

## ✅ Phase 6: Hardware & HDL Backends — COMPLETE
- [x] **Verilog/VHDL Codegen**: `ternlang-hdl` crate, map trit → 2-bit wire pairs
  - Primitives: trit_neg, trit_cons, trit_mul, trit_add, trit_reg, bet_alu
  - Sparse matmul array: parameterised N×N with per-cell zero-skip enable
  - ISA control: bet_regfile, bet_pc (16-bit), bet_control (all opcodes), bet_processor (top-level)
  - 11 HDL tests passing
- [x] **BET ISA Spec Document**: `BET-ISA-SPEC.md` — formal ISA spec with encoding tables, stack-effect notation, hardware mapping
- [x] **FPGA Simulation** (Phase 6.1): Cycle-accurate RTL simulator in pure Rust (`BetRtlProcessor`) — mirrors `bet_processor.v` exactly; same 2-bit encoding, clocked regfile, PC, ALU; 12 RTL unit tests; `ternlang sim --rtl` CLI flag (no external tools needed); iverilog path still supported via `ternlang sim --run`

---

## ✅ Developer Tooling — COMPLETE
- [x] **LSP**: `ternlang-lsp` crate — JSON-RPC 2.0 over stdio, diagnostics, hover, completion (19 snippets)
- [x] **VS Code extension v0.3.3**: `ternlang-vscode/` — Full extension rebuilt from scratch. TextMate grammar (keywords/types/trit constants/functions/@attributes/operators/strings/comments), 8 snippets (main, fn, exhaustive 3-way match, let, @sparseskip, consensus, invert, tensor), 5 commands (`ternlang.run`, `ternlang.runDebug`, `ternlang.build`, `ternlang.check`, `ternlang.repl`), check-on-save diagnostics, `ternlang.executablePath` + `ternlang.checkOnSave` settings. Published to **Open VSX only** (`rfi-irfos/ternlang`). Never publish to Microsoft VS Marketplace.
- [x] **Formatter**: `ternlang fmt [--write]` — canonical style for 3-way match arms
- [x] **REPL**: `ternlang repl` — interactive trit expression evaluation via BET VM
- [x] **Package manager (ternpkg)**: `ternlang.toml`, `ternpkg install [PKG]`, GitHub-backed registry

---

## ✅ Phase 7: Ecosystem Bridges — COMPLETE
**Goal: make ternlang the convergence point for all existing ternary computing projects.**

- [x] **Hub positioning**: README + TERNARY-ECOSYSTEM.md — maps every active ternary project to a ternlang interop bridge
- [x] **TasmAssembler** (ternlang-compat): two-pass 9-trit RISC assembler → BET bytecode; parses balanced ternary literals (T=-1); 15 tests
- [x] **OwletParser** (ternlang-compat): S-expression ternary front-end → ternlang AST → BET VM; full S-expr grammar; 14 tests
- [x] **Cargo workspace metadata**: `[workspace.package]` with keywords, categories, license, repository for crates.io
- [x] **Academic whitepaper**: `whitepaper/ternlang-whitepaper.tex` (IEEE two-column LaTeX) + `ternlang-whitepaper.docx`
- [x] **Spec consolidation**: `spec/grammar.ebnf`, `spec/ternlang-language-reference-v0.1.md`, `spec/ternlang-dictionary-v0.1.json` versioned in main repo
- [x] **7B — crates.io**: all 9 crates published (2026-04-04); v0.3.3 published 2026-04-16
- [x] **7B — MCP registry**: HTTP transport live at https://ternlang.com/mcp, listed on Smithery (`rfi-irfos/ternlang`)
- [x] **7B — VS Code**: Open VSX only (`rfi-irfos/ternlang`). Microsoft VS Marketplace is NOT a target.
- [x] **7C — USN academic outreach**: Dr. Steven Bos replied 2026-04-13. Key feedback: hardware switching is the real bottleneck (not software); 122× claim needs hardware validation; use their open ISAs/compiler pipelines. Simeon opened 2 silent PRs fixing bugs in their codebase. Relationship initiated correctly.

---

## ✅ Phase 8: Ternary AI Reasoning Toolkit — COMPLETE
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

## ✅ Phase 9: MoE-13 Ternary Orchestrator — COMPLETE
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
- [x] MCP tools: `moe_orchestrate`, `moe_deliberate`, `trit_action_gate` (3 new tools → 10 total in ternlang-mcp)
- [x] SSE streaming in `ternlang-api`: `GET /api/stream/moe_orchestrate` + `GET /api/stream/deliberate`
- [x] Wall-E easter egg hidden in `ternlang-moe::orchestrate()` — fires on query "wall-e"

---

 ═══════════════════════════════════════════════════
# PART II — ACTIVE DEVELOPMENT (Phases 10–13)
 ═══════════════════════════════════════════════════

## ✅ Phase 10: Extension Maturity — COMPLETE

### 10A — Pre-Built LSP Binary (GitHub Actions CI) ✅
- [x] `.github/workflows/lsp-release.yml`: build matrix (ubuntu/macos/windows + cross for ARM64); triggered by `vscode-v*` tags; publishes to GitHub Releases
- [x] Extension `activate()`: platform-aware binary resolution (linux-x64/arm64, darwin-x64, win32-x64); resolution order: bundled → globalStoragePath cache → GitHub Releases download with progress notification; graceful failure with suppress option

### 10B — Tier 2: Inline Trit Value Hints (Ghost Decorations) ✅
- [x] `--emit-symbols` flag on CLI Run command; emits `TERN_SYMBOLS:var=reg,...` to stderr using main() scope snapshot
- [x] VS Code `DecorationProvider`: ghost annotations after every `let` binding (→ Affirm teal / → Tend amber / → Reject red); parses TERN_SYMBOLS from stderr + `Reg N: trit(...)` from stdout; cleared on file switch
- [x] Activated only when apiKey starts with `tern_t2_`
- [x] Extension v0.4.0 published to Open VSX as rfi-irfos/ternlang

### 10C — Dogfood the MCP ✅
- [x] ternlang-mcp added to Claude Code local config via `claude mcp add` — 19 tools live in this session
- [ ] Log friction points found during use → next release

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

## ✅ Phase 11: MCP Intelligence Upgrade — COMPLETE

### 11A — 5 New MCP Tools ✅ COMPLETE (2026-04-11)

**`trit_debate`** (Free tier) ✅
> Give it two competing claims. Get a structured 3-way verdict: evidence for each side, and what's genuinely uncertain (Tend).
- [x] Input: `{ "claim_a": string, "claim_b": string, "context"?: string }`
- [x] MoE routes both claims through FactCheck + DeductiveReason + AmbiguityRes experts
- [x] Output: `{ "for_a": trit, "for_b": trit, "tension": float, "synthesis": string, "hold_reason"?: string }`

**`trit_uncertainty_map`** (Free tier) ✅
> Paste in any text. Returns every claim annotated with Affirm/Tend/Reject and a confidence score.
- [x] Input: `{ "text": string, "granularity": "sentence" | "paragraph" }`
- [x] Output: array of `{ "claim": string, "trit": int, "confidence": float, "reason": string }`

**`trit_calibrate`** (Free tier) ✅
> Given a log of an AI agent's recent outputs, score how binary its decision-making is.
- [x] Input: `{ "decisions": [{ "input": string, "output": string, "confidence"?: float }] }`
- [x] Output: `{ "binary_ratio": float, "hold_opportunities": int, "calibration_score": trit, "recommendations": string[] }`

**`trit_translate`** (Pro — Tier 2) ✅
> Input Python `if/elif/else` or SQL `CASE WHEN`. Output: `.tern` with hold zone inserted where the original had no coverage.
- [x] Input: `{ "code": string, "language": "python" | "sql" | "json_rules" }`
- [x] Output: `{ "tern_code": string, "hold_zones_added": int, "explanation": string }`

**`trit_eco_check`** (Free tier) ✅
> Returns two trit scores: human-centric and ecocentric. When they diverge, synthesis → Tend.
- [x] Input: `{ "action": string, "context"?: string, "scope"?: "local" | "regional" | "global" }`
- [x] Output: `{ "human_trit": int, "eco_trit": int, "synthesis": int, "tension": bool, "eco_reasoning": string }`

### 11B — EcoCore: Ecocentric Reasoning Parameter for MoE-13 ✅ COMPLETE (2026-04-20)

The key insight: when human-optimal is Affirm and eco-optimal is Reject, the right answer isn't a compromise — it's **Tend** (hold, reconsider). Ternary logic handles this natively.

- [x] New `EcoExpert` in `ternlang-moe`: competence vector emphasizes `safety` + new `systemic_impact` dimension (6D → 7D, backwards compatible)
- [x] `EcoCentric` flag on `TernMoeOrchestrator` config (`eco_mode: bool`)
- [x] When `eco_mode: true`: run standard MoE pipeline → `human_result`; run EcoExpert independently → `eco_result`; if tension > 0: override → Tend
- [x] `EcoCoreConfig`: `{ "enabled": bool, "scope": "local"|"regional"|"global", "hard_veto_on_eco_reject": bool }`
- [x] 8 new tests: eco_tension detection, hard veto, conversation weighting, trit_eco_check MCP tool

---

## ✅ Phase 11.5: ternlang-compress — Float LLM → Ternary Compression Pipeline — COMPLETE
- [x] **Phase 11.5A**: Foundations — `compress()`, `SparseIndex`, 2-bit packing
- [x] **Phase 11.5B**: Llama 3.2 1B Integration — full model transmutation (Llama 3.2 1B → `llama32-1b.tern.json`)
- [x] **Sparsity Verified**: 30.63% mean sparsity across 147 layers

---

## ✅ Phase 11.6: ExaTern High-Performance Foundation — COMPLETE
**Goal: Shift from software emulation to hardware-aligned vectorized execution.**

- [x] **"Tridiac" SIMD Core**:
    - [x] 5-trit-per-byte packing (99.06% storage efficiency).
    - [x] Vectorized opcodes: `0x52` (TV_ADD), `0x53` (TV_NEG), `0x54` (TV_CON).
    - [x] $243 \times 243$ pre-computed arithmetic lookup tables for O(1) packed ops.
- [x] **Zero-Copy View Architecture**:
    - [x] `Value::TensorView` with absolute offset/stride resolution.
    - [x] `0x55` (TVIEW) opcode for nested slicing without memory allocation/copying.
- [x] **Register Binding (TBIND)**:
    - [x] `0x42` (TBIND) to link VM registers directly to memory views.
    - [x] High-speed in-place updates to underlying tensor data.
- [x] **Language Integration**:
    - [x] `packed trit[N]` type support in parser and codegen.
    - [x] `tensor[start..end; stride]` slicing syntax.
- [x] **Verification**: Standard library performance test suite passed.

---

## 🛠 Phase 12: Ternary Model Coherence & Retraining — IN PROGRESS
**Goal: Make the transmuted Llama model coherent using QAT/STE.**

- [x] **Phase 12A**: Coherence Testing — Rust-based forward pass for `llama32-1b.tern.json`. Verified signal coherence (97.06% signal ratio) using `sparse_matmul` on Llama 3.2 1B weights.
- [x] **Phase 12A.1**: Model Reduction — Created `ModelCoherence` binary format; reduced 1.2GB JSON weights to 240MB packed binary.
- [x] **Phase 12A.2**: Architecture Implementation — Implemented `TritTransformer` (Llama-3 style) in `ternlang-ml` with RMSNorm, RoPE, and SwiGLU kernels. Verified with a full 1.2B parameter forward pass.
- [ ] **Phase 12B**: Quantization-Aware Training (QAT) — Implement Straight-Through Estimator (STE) fine-tuning loop in `ternlang-ml`.
- [ ] **Phase 12C**: Accuracy Validation — Compare perplexity before/after retraining.

---

## ✅ Phase 13: Repository Professionalization — COMPLETE
- [x] **SEO & Keyword Injection**: XAI, Sparsity-Aware, Deterministic terms in all READMEs and Cargo metadata.
- [x] **Community Files**: Rewrote CONTRIBUTING.md and SECURITY.md with RFI-IRFOS "Styrian Rebel" branding.
- [x] **Governance**: Established .github/CODEOWNERS for the core leadership team.
- [x] **CI Badges**: Added Rust CI and Sparsity-Performance badges to README.

---

═══════════════════════════════════════════════════
# PART III — FUTURE PHASES (14–18)
═══════════════════════════════════════════════════

## 🔄 Phase 14: TernTranslator — The Bridge Into the Existing World — PARTIAL
Most potential users have binary decision trees they've been running for years. TernTranslator is the onramp: give it your Python `if/elif/else` or SQL `CASE WHEN` and it outputs `.tern` with the ternary hold zone added where the original code had no coverage.

- [x] `ternlang translate <input.py>` CLI command — Python, SQL, JSON rules → .tern with tend arms (2026-04-12)
- [x] `POST /api/translate` REST endpoint — live at `https://ternlang.com/api/translate` (Tier 2+, deployed 2026-04-12)
- [x] `trit_translate` MCP tool — done in Phase 11A (2026-04-11)
- [x] Target languages v1: Python, SQL, JSON rule sets — all done
- [ ] VS Code command: `Ternlang Pro: Translate Selection to Ternary` (Tier 2) — select if/else block → `.tern` in side panel
- [ ] Target languages v2: JavaScript, TypeScript, YAML (Kubernetes policy rules)

---

## 📚 Phase 15: Distribution, Academia, Community

### 15A — Jupyter Kernel ✅ COMPLETE (2026-04-18)
- [x] `ternlang-jupyter`: ipykernel-based Jupyter kernel wrapping `ternlang-cli`
- [x] `.tern` cells in Jupyter notebooks — execute, stream print() output, display trit result
- [x] Rich output: color-coded AFFIRM/HOLD/REJECT blocks with byte count
- [x] Install: `pip install ternlang-jupyter && ternlang-jupyter-install`
- [x] Tab completion, hover docs, %version/%help magic commands
- [x] Handles all three exit codes correctly (reject RC=1 is valid result, not error)

### 15B — ternpkg Curated Registry
- [x] Move beyond GitHub-backed install — add a curated `registry.ternlang.com` index
- [x] Quality gate: every registered package must pass `ternlang-cli run` with exit 0
- [x] `ternpkg search <keyword>` — search the registry
- [x] `ternpkg publish` — submit a package (authenticated, rate-limited)
- [x] Seed with: stdlib bundles (core, ml, safety), TernAudit rules, community agents

### 15C — Academic Outreach
- [x] Contact USN group (Bos & Gundersen) — done 2026-04-13. Reply from Dr. Steven Bos received.
  - Key feedback: hardware switching devices are the real bottleneck; they have EDA tooling, RTL, C→ternary compilers already; 122× claim needs sim vs real hardware distinction; AI-generated prose was off-putting.
  - Simeon opened 2 silent PRs fixing bugs in their code — correct path to credibility.
- [ ] Follow-up after PRs merged — propose aligning BET-ISA encoding with their BCT standard
- [ ] Submit to arXiv: "BET-ISA: A Balanced Ternary Execution Architecture for Sparse Neural Inference" — must include honest benchmark methodology (measured in Rust release mode, not on ternary hardware)
- [ ] Target venues: DATE conference (hardware EDA — relevant to their tapeout work), NeurIPS efficiency workshop
- [ ] DOI registration for all RFI-IRFOS papers — already started (OSF)

### 15D — Community
- [ ] Discord server: `#ternlang` — language, `#bet-vm` — compiler, `#mcp` — AI integration, `#research`
- [ ] GitHub Discussions: enabled on the repo
- [ ] Hacker News launch: coordinate WASM playground (Phase 17) + curated stdlib showcase
  - Headline: "Ternlang: a programming language where 'I don't know' is a first-class value [try in browser]"
- [ ] Weekly changelog post

---

## 🏗 Phase 16: TernStudio v1.0 — The Full IDE
[x] The arc of the VS Code extension ends at v1.0.0 / TernStudio. This is the complete developer environment for ternary systems programming. Ternstudio SDK Beta is released, 21.04.2026 @ https://ternlang-api.fly.dev/studio

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
The standalone web IDE — Monaco editor + real BET VM (WASM, Phase 17) + integrated TernAudit + project management.

- [x] SAP-style layout: Activity Bar → Explorer / Editor / History / Settings panes
- [x] File tree: project-aware, stdlib browser, `ternpkg.toml` aware
- [x] Run panel: real BET VM output, inline trit annotations on variables
- [x] TernAudit tab: paste any AI output, get trit audit instantly
- [x] TernTranslator tab: paste Python/SQL, get .tern output
- [x] Share button: `btoa` hash → shareable URL with full program state
- [x] Download button: save current project as `.ternproj` bundle
- [x] Run history: last 20 executions with trit state snapshots

---

## 🌐 Phase 17: WASM Runtime — Make TernGround Real
*(Previously listed as "Phase 12: WASM Runtime" — renumbered to resolve collision with Phase 12: Model Coherence)*

**Why this matters:** TernGround Lab 05 currently runs `.tern` in a hand-written JS interpreter. The semantics drift from the real compiler. The fix is to compile `ternlang-core` to WebAssembly — the real BET VM, running in the browser, no installation. This is also the Hacker News launch vehicle.

- [x] `cargo build --target wasm32-unknown-unknown -p ternlang-core` — **VERIFIED** compiles with zero errors (2026-04-12).
- [x] `wasm-bindgen` wrapper: `run_tern(src) -> String` and `check_tern(src) -> String` exposed (2026-04-18)
- [x] Replace `playground/index.html` JS interpreter with WASM call — real BET VM in browser (2026-04-18)
- [x] TernGround playground: "BET VM (WASM) · real compiler · v1.0.0" — accurate, not approximate
- [x] Performance: real compiler instantaneous for all demo programs
- [x] WASM artifact embedded in `ternlang-api` via `include_bytes!`, served at `/playground/pkg/`
- [x] Auth middleware whitelisted for `/playground` and `/playground/pkg/*` — publicly accessible

---

## 🔍 Phase 18: TernAudit — The Killer App — PARTIAL
*(Previously listed as "Phase 13: TernAudit" — renumbered to resolve collision with Phase 13: Repository Professionalization)*

TernAudit answers the question "why would an enterprise buy ternlang?" with a specific, auditable, EU-AI-Act-compliant answer: *because our AI's decisions are now trit-annotated and you can prove it to a regulator.*

- [x] `ternlang audit <input.json>` CLI command — reads JSON, prints coloured summary, writes `audit_report.json` + optional `audit_report.html` (`--html` flag)
- [x] `POST /api/audit` REST endpoint — live at `https://ternlang.com/api/audit` (Tier 2+, deployed 2026-04-12)
- [x] Audit report format: `total_decisions`, `affirm/tend/reject_count`, `forced_binary_ratio`, `eu_ai_act.article_13/14`, `flagged[]`
- [x] `trit_audit` MCP tool — 26th tool in ternlang-api v0.3.1 (10 free + 16 premium)
- [x] Marketing: "The only tool that finds the decisions your AI should have held"
- [ ] VS Code command: `Ternlang Pro: Audit Selection` — select any block of AI outputs, get inline annotations

---

═══════════════════════════════════════════════════
# LICENSING & IP
═══════════════════════════════════════════════════

- [x] **Open core**: LGPL v3 (compiler + stdlib) — forces compiler contributions back
- [x] **Commercial tier**: proprietary license for `ternlang-ml`, HDL backend, distributed runtime
- [ ] **Trademark**: "Ternlang", "BET VM", "Balanced Ternary Execution"
- [x] **Academic outreach**: USN / Bos & Gundersen contacted — see Phase 7C and Phase 15C

---

═══════════════════════════════════════════════════
# PENDING ACTION ITEMS (as of 2026-04-17)
═══════════════════════════════════════════════════

### High Priority — Completed This Session (2026-04-17)
- [x] **Smithery uptime fix** — premium tool errors changed from JSON-RPC protocol errors → MCP tool-level `isError:true`. Deployed to Fly.io. 2026-04-17.
- [x] **KPI dashboard** — `~/Desktop/ternlang_kpi_fetch.py` (1054 lines): live crates.io/OpenVSX/GitHub/Smithery data, 5-min auto-refresh, light/dark, XLSX export, milestone alerts, Firefox launch. 2026-04-17.
- [x] **Day-0 baseline locked** — `~/Desktop/ternlang_baseline.json` (crates 749, openvsx 554, gh_stars 4). Permanent reference point. 2026-04-17.
- [x] **Permanent GitHub traffic log** — `~/Desktop/ternlang_gh_traffic.json` — 14-day window merges on every run, data never expires. 2026-04-17.
- [x] **ternlang-mcp v1.0.0** — README.md created, package.json hardened (keywords/homepage/repo/license), smithery.yaml systemPrompt added, published to Smithery (release `a02625c1`, 30 tools). 2026-04-17.
- [x] **STDLIB_AGENT.md targeted scan protocol** — replaced global weakness scan with per-directory targeted check (10 ls commands max, hard prohibition on ls -R / find across full tree). 2026-04-17.

### High Priority — Completed Previous Session (2026-04-16)
- [x] **v0.3.3 crates.io republish** — all 8 crates (core→cli) republished at v0.3.3. 2026-04-16.
- [x] **`ternlang check`** — new subcommand: parse-only validation, walks dirs, per-file ok/error + fn count. 2026-04-16.
- [x] **`ternlang exec <file.tbc>`** — run pre-compiled bytecode. 2026-04-16.
- [x] **`ternlang run --debug`** — verbose register dump with variable name labels. 2026-04-16.
- [x] **`ternlang build` fixed** — emits header-jump + `emit_entry_call("main")`, .tbc files are now self-contained. 2026-04-16.
- [x] **VS Code extension v0.3.3** — full rebuild from stub. Published to Open VSX. 2026-04-16.

### Medium Priority — Remaining
- [x] **Smithery score check** — verify if 100/100 achieved after release `a02625c1` rescan. Address any remaining gaps.
- [ ] **Whitepaper update** — stdlib count now 27,000+ files, 267 examples in root + 2,090 total. Update Section 10 implementation status table.
- [ ] **Phase 7C follow-up** — after PRs merged, propose BET-ISA alignment with USN BCT standard.
- [x] **Phase 11A/11B build** — 5 new MCP tools + EcoCore in ternlang-moe.
- [x] **Phase 17 (WASM)** — wasm-bindgen wrapper, replace JS interpreter in TernGround.

### Low Priority / Nice to Have
- [x] **README example count update** — actual count is 2,090 in examples/. Update table to "2,000+".
- [ ] **Benchmark blog post** — Document 2.3×–122× sparse matmul results vs float32 (with methodology note).
- [x] **Gemini stdlib sessions** — Continue breadth-first population per STDLIB_AGENT.md (now with targeted scan protocol). Target: 50 new files per session.

---

 ═══════════════════════════════════════════════════
# 2026 PRIORITY MATRIX
 ═══════════════════════════════════════════════════

| Quarter | Deliverable | Impact | Effort |
|---------|------------|--------|--------|
| ~~Q2 2026~~ **DONE** | 5 new MCP tools (Phase 11A) ✅ | 🟠 High | — |
| ~~Q2 2026~~ **DONE** | EcoCore in MoE-13 (Phase 11B) ✅ | 🟠 High | — |
| ~~Q2 2026~~ **DONE** | WASM runtime (Phase 17) ✅ | 🟢 Complete | — |
| Q3 2026 | TernAudit VS Code inline (Phase 18) | 🔴 Critical (commercial) | Medium |
| Q3 2026 | TernTranslator VS Code panel (Phase 14) | 🟠 High | Medium |
| ~~Q3 2026~~ **DONE** | Jupyter kernel (Phase 15A) ✅ | 🟢 Complete | — |
| ~~Q3 2026~~ **DONE** | Hacker News launch (Phase 15D) ✅ | 🔴 Critical (distribution) | Low |
| Q4 2026 | VS Code extension v0.5.0 — BET debugger (Phase 16) | 🟠 High | High |
| >>Q4 2026>> **DONE** | TernStudio v1.0 (Phase 16) | 🟠 High | Very High |
| Q4 2026 | arXiv paper submission (Phase 15C) | 🟡 Medium | Medium |

---

═══════════════════════════════════════════════════
# APPENDIX A — HISTORICAL SESSION LOG
═══════════════════════════════════════════════════

| Date | Session Update |
|------|----------------|
| 2026-04-02 | Initial repo setup. Phase 1+2 confirmed complete. Git initialized, pushed to GitHub. Credential store configured. 4 failing tests identified (DimSeparator bug). Phase 3 plan defined. |
| 2026-04-02 | Fixed DimSeparator/Ident collision in lexer. Fixed betbc test import. 11/11 tests passing. Next: TCALL/TRET function dispatch + tensor VM opcodes. |
| 2026-04-02 | TCALL/TRET implemented. Tensor opcodes DONE: TMATMUL, TSPARSE_MATMUL, TIDX, TSET, TSHAPE, TSPARSITY. 14/14 tests passing. |
| 2026-04-02 | @sparseskip → TSPARSE_MATMUL wired in codegen. ternlang-ml filled: quantize, bitnet_threshold, dense_matmul, sparse_matmul, linear, benchmark. First benchmark: 56% sparsity → 2.3× fewer multiply ops. 23/23 tests passing. |
| 2026-04-02 | ternlang-mcp LIVE — MCP server (JSON-RPC 2.0, stdio). 6 tools: trit_decide, trit_consensus, trit_eval, ternlang_run, quantize_weights, sparse_benchmark. Hidden easter egg: ternlang enlighten. |
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
| 2026-04-03 | Phase 6.1 COMPLETE: BetRtlProcessor — cycle-accurate RTL simulator in pure Rust. Mirrors bet_processor.v exactly: TritWire 2-bit encoding, trit_neg/cons/mul/add combinational primitives, BetRegfile, BetPc (16-bit), BetAlu, bet_decode control unit. `ternlang sim --rtl [--max-cycles N]` CLI. 12 RTL unit tests + 2 doctests. 93 tests total across core/hdl/runtime. |
| 2026-04-03 | Phase 8 COMPLETE: Ternary AI Reasoning Toolkit in ternlang-ml — DeliberationEngine (EMA convergence), coalition_vote (quorum/dissent/abstain), action_gate (multi-dim hard-block), scalar_temperature (trit→LLM temp bridge), hallucination_score (variance→trust trit). TritScalar gains trit_i8() + Clone. Phase 8 REST endpoints in ternlang-api (5 endpoints). 15 reasoning tests. |
| 2026-04-03 | Phase 9 COMPLETE: ternlang-moe crate — MoE-13 ternary orchestrator (DOI 10.17605/OSF.IO/TZ7DC). CompetenceVector (6D), TernMoeRouter (dual-key synergistic routing), TriadField (1+1=3 emergent synthesis), three-tier memory mesh (Node/Cluster/Axis), TernMoeOrchestrator (9-step pipeline), 13-expert standard pool, temperature bridge, safety hard gate with audit log. 16/16 tests passing. 146+ total tests across workspace. |
| 2026-04-04 | MCP Phase 9 tools: moe_orchestrate, moe_deliberate, trit_action_gate (10 MCP tools total). SSE streaming endpoints in ternlang-api (stream/moe_orchestrate, stream/deliberate). Whitepaper updated to 887 lines with Phase 8+9 sections. All 9 crates published to crates.io. 20 .tern example files added to examples/ with INDEX.md — covering aerospace, medicine, DevOps, autonomous vehicles, AI agents, civic systems, finance, CPU pipeline (Brandon Smith tribute), S-expression eval (Owlet tribute), microservices, diplomacy, recruiting, scheduling, caching. Homepage updated with plain-language "Why Ternary" section. |
| 2026-04-04 | **MILESTONE: ternlang.com LIVE + HTTPS** — Fly.io TLS cert provisioned for ternlang.com. GitHub Pages A records removed from GoDaddy; Fly.io A record (66.241.124.209) + AAAA record now sole DNS entries. index.html embedded in ternlang-api via `include_str!`. Albert wired: `trit_decide` (public MCP) + `moe_orchestrate` (REST) added to albert tools.py — live test confirmed trit=1 AFFIRM from Fly.io. MoE-13 agent harness overhauled: 13 TernaryAgent impls rebuilt with dual-signal deliberation. `run_introspective()` added to AgentHarness — stable attractor hold. `orchestrate_full()` added. 24/24 tests passing. QNN .tern examples 251–265 added. |
| 2026-04-04 | **MILESTONE: ternlang-api LIVE on Fly.io** (Frankfurt, shared-cpu-1x 256MB). POST /mcp HTTP MCP transport built — JSON-RPC 2.0 over HTTP, 10 tools, no auth required. DNS A record added to ternlang.com via GoDaddy. Smithery submission pending DNS propagation. Gemini agent: 250 total .tern examples, 13 expert AgentHarness modules, 20 tutorial .tern files. README.md overhauled. |
| 2026-04-05 | **Gemini Agent: STDLIB DOMINATION.** Added ~130 new stdlib modules across 17 categories (Classical ML, Deep Learning, RL, NLP, Vision, etc.). Upgraded compiler to support `affirm/tend/reject` keywords, binary `if/while` fallbacks, tensor indexing `obj[r,c]`, and comparison operators. All 217+ stdlib modules now built-in to the compiler. Tests passing (190+). |
| 2026-04-09 | **MAJOR SESSION: Commercial hardening, EU AI Act compliance, pricing overhaul, Fly.io production deploy fix, full website redesign, TernGround playground launch.** |
| 2026-04-09 | **[1/8] README hardening.** Root README completely rewritten — absolute URLs throughout, broken links fixed, MCP tool count corrected 20 → 13, IEEE/ISO standard claims removed, emergent triad formula corrected, architecture card links updated to real crates.io/GitHub URLs. |
| 2026-04-09 | **[2/8] EU AI Act compliance layer.** 4 EU AI Act / GDPR compliance badges added. Full compliance table added covering Articles 9, 11, 12, 13, 14, 15 and GDPR (Frankfurt data residency). |
| 2026-04-09 | **[3/8] Pricing overhaul.** Tier 2 → €99/month (10k calls), Tier 3 → €349/month (50k calls), Tier 4 → from €2,500/month. New Stripe payment links created and deployed across index.html, pricing.html, README.md. |
| 2026-04-09 | **[4/8] API tier detection + Stripe webhook.** Tier detection threshold updated to match new €349 Tier 3 price. Tier 3 monthly limit updated from 20k → 50k. Webhook secret stored as Fly.io secret `STRIPE_WEBHOOK_SECRET` (value never in source control). |
| 2026-04-09 | **[5/8] Website product copy overhaul (index.html).** MCP headline rewritten. TernAudit feature cards rewritten to lead with customer pain. Logic Library card text size improved. 4-tier licensing box updated. |
| 2026-04-09 | **[6/8] Email routing: *@ternlang.com catch-all.** GoDaddy DNS updated to ImprovMX (MX1/MX2). Catch-all: `*@ternlang.com → rfi.irfos@gmail.com`. Resend handles outbound transactional mail. |
| 2026-04-09 | **[7/8] Fly.io production deploy fix.** Three Dockerfile path errors fixed: (a) missing ternlang-runtime in COPY commands; (b) missing stdlib directory for include_str! macros; (c) Rust 2024 borrow checker — 7 E0382 errors in vm/mod.rs fixed with .clone() on match scrutinees. Deploy successful, all machines healthy. |
| 2026-04-09 | **[8/8] TernGround playground — new page.** 5 interactive labs consolidated into TernGround tab. New Lab 06 added: ternary truth table (TMAX, TMIN, TADD, TMUL). All lab descriptions rewritten in plain English. Nav restructured. GEMINI.md rewritten from scratch (hallucinated security vulnerability removed). |
| 2026-04-10 | **[1/5] Fun Error Dictionary + VM hardening.** `stdlib/errors/error_registry.json` v2.0.0: 20 error codes across 7 namespaces, each with fun_message, technical, common_causes[], fix, bad/good patterns. 22 runnable .tern example files. Compiler fun messages updated across parser.rs, semantic.rs, vm/mod.rs. VM hardening: 4 new VmError variants + bounds checks for TIDX/TSET/TSHAPE (previously panicked). |
| 2026-04-10 | **[2/5] playground/index.html — self-contained TernGround IDE.** 103KB self-contained HTML with full JS .tern interpreter (tokenizer → recursive descent parser → evaluator). 25 stdlib files baked in via Python embed script. Parameter panel auto-detects `let` declarations. `playground/embed_stdlib.py` regeneration script created. `fly.toml` moved to repo root. |
| 2026-04-10 | **[3/5] CI/CD overhaul.** `ternlang-ci.yml`: replaced hallucinated workflow with real Rust CI — installs stable Rust, caches cargo, builds ternlang-cli --release, smoke tests version + 11 stdlib error examples. `deploy-fly.yml`: new Fly.io deployment workflow — triggers on push to main when ternlang-web/, ternlang-api/, Dockerfile, or fly.toml change. |
| 2026-04-10 | **[4/5] Index.html polish.** All 6 lab descriptions in TernGround now have two tiers: context paragraph + interaction guide. Nav product items unified: all 7 products `text-t_text hover:text-t_teal`. |
| 2026-04-10 | **[5/5] Production deploy.** `fly deploy` triggered. Both Fly.io machines updated, health checks passed. ternlang.com live. Auto-deploy via GitHub Actions now active. |
| 2026-04-10 | **[Gemini stdlib session]** STDLIB_AGENT.md rewritten to v2.5: weakness scan, 5-batch × 10-file sessions, AGENT_SESSIONS.md cooldown log. Purged 3,000+ hallucinated files from stdlib/astro/, stdlib/bench/, stdlib/benchmarks/. Seeded with 6 real tested files. GEMINI.md v1.2: all fixes hardcoded, opcodes 0x28/0x29 added. |
| 2026-04-10 | **[VM fix] Tset (0x23) Int polymorphism.** Added `(Value::TensorRef, Value::Int)` arm to Tset dispatch — integer values now coerce to `Trit::from(v as i8)` on tensor write. Fixes BET-007 TypeMismatch on `stdlib/nn/ternary_relu.tern`. Fixes.md entry #23 added. |
| 2026-04-10 | **[VS Code extension] v0.2.0.** `ternlang.tmLanguage.json`: added `affirm\|tend\|reject` as trit constants; added `<=\|>=` to operators. `package.json` bumped 0.1.0 → 0.2.0. VSIX rebuilt (442 KB). Published to Open VSX registry as `rfi-irfos/ternlang v0.2.0`. 5 compiler/VM bugs fixed (AND/OR logic BUG-A/B, for-in loop count BUG-C, FieldAccess BUG-D, Cast BUG-E). FLY_API_TOKEN refreshed (100-year Macaroon). |
| 2026-04-11 | **[Ecosystem] Tern Systems collaboration outreach + TERN-ASM emitter.** Discovered Tern-Computer GitHub org (BTMC + TERN assembly). Opened collaboration issue: Tern-Computer/.github#8. Added `ternlang-core/codegen/tern_asm.rs` — full RISC-V-inspired balanced ternary ASM emitter. Wired into CLI as `ternlang build --emit-tern` → writes `.tern.asm` file. |
| 2026-04-12 | Open VSX publish — `ternlang-0.2.0.vsix` published to open-vsx.org (rfi-irfos/ternlang). v0.2.0: affirm/tend/reject highlighting, <=/>= operators added to grammar. Multiple downloads live. |
| 2026-04-12 | MCP registry / Smithery — listed as `rfi-irfos/ternlang` at smithery.ai. Description + icon updated to v0.3.0 via API. Tools auto-scanned from live server. |
| 2026-04-14 | **MILESTONE: 1.2B Parameter Binary Model + TritTransformer.** Created `ModelCoherence` binary format for `ternlang-ml`, reducing 1.2GB JSON to 240MB packed binary. Implemented `TritTransformer` (Llama-3 architecture) in Rust with RMSNorm, RoPE, and SwiGLU kernels. Verified model loading and performed first full 1.2B parameter ternary forward pass in `ternlang-ml/src/bin/inference.rs`. |
| 2026-04-16 | **[CLI v0.3.3 + VS Code v0.3.3]** `ternlang check`: parse-only validator, walks directories, per-file ok/error + fn count. `ternlang exec <file.tbc>`: run pre-compiled bytecode — completes build→exec pipeline. `ternlang run --debug`: verbose register dump with variable name labels. `ternlang run`: clean coloured result box by default. `ternlang build` fixed: header-jump + entry_call("main") now emitted — .tbc files self-contained. `ternlang <file>` shortcut. All 8 crates bumped to v0.3.3 and published to crates.io. VS Code extension v0.3.3 rebuilt from scratch — full grammar, 8 snippets, 5 commands, check-on-save diagnostics, language-config. Published to Open VSX only. GitHub release v0.3.3 tagged. Stripe webhook secret rotated. ROADMAP structurally refactored: duplicate Phase 12/13 renumbered to 17/18, broken session log table header fixed, section dividers added, orphaned rows moved to appendix. |
| 2026-04-17 | **[Smithery uptime + MCP v1.0.0 + KPI dashboard + STDLIB_AGENT targeted scan]** Smithery 0% uptime fixed (protocol errors → MCP isError:true). Deployed to Fly.io (release 412ff381). KPI dashboard built: ternlang_kpi_fetch.py (1054 lines), Day-0 baseline locked, permanent traffic log, 5-min live chart, light/dark, XLSX export, Firefox launch. ternlang-mcp bumped to v1.0.0: README.md created, package.json hardened, smithery.yaml systemPrompt added. Republished to Smithery (release a02625c1, 30 tools confirmed). STDLIB_AGENT.md: global weakness scan replaced with targeted 10-dir check protocol (wc -l per dir, single-file existence check before writing, hard ban on ls -R / find). |
| 2026-04-25 | **[Albert TUI + Voice + Multi-provider]** Major Albert CLI overhaul. Full ratatui TUI rewrite with tool dots and ⎿ output connectors. Voice input (Whisper STT with API fallback), clipboard paste, and /plan+/loop inline commands. Multi-provider expansion for Albert CLI + Studio. Markdown rendering and interactive popup menus. |
| 2026-04-26 | **[KPI Spine + Stdlib Expansion]** Automated KPI dashboard hosted via GitHub Pages with interactive charts. Containerized KPI spine and background service integration. Massive expansion of `stdlib` (over 40 new modules) including `ternary_set_theory`, `market_sentiment_sim`, `pid_thermal_control`, and advanced algorithms (Fenwick, Segment trees). Bumped Albert crates to v1.1.3. |
| 2026-04-29 | **[COMP-TENSOR-001 + VM-STRUCT-001 + v1.2.1 Sync]** Catastrophic technical debt resolved in BET-VM. Lifted 16-bit tensor dimension cap to 32-bit (u32 immediates), allowing dimensions up to 4.29B. Implemented native Struct return ABI (Value::Struct) resolving stack underflow limitations. Synchronized entire Ternlang crate ecosystem to v1.2.1 and published all 13 crates to crates.io. |
| 2026-04-29 | **[ExaTern SIMD + Zero-Copy]** Implemented ExaTern High-Performance Foundation. 5-trit-per-byte packing (99.06% efficiency) with pre-computed lookup tables for O(1) vectorized ops. Implemented Zero-Copy View architecture (TVIEW) and Register Binding (TBIND) for in-place tensor updates. Added `packed trit[N]` and `tensor[start..end]` syntax to parser/compiler. |
