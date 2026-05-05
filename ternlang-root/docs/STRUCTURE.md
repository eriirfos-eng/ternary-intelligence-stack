# ternlang-root — Directory Structure Guide

The root contains 40+ Rust crates, a standard library, examples, specifications, and research material. This guide groups them logically.

---

## Rust Crates

### Open Core (LGPL-3.0)

| Directory | What it is |
|-----------|-----------|
| `ternlang-core/` | Lexer, parser, AST, BET VM — the language foundation |
| `ternlang-cli/` | `ternlang` binary: run, build, sim, fmt, repl, audit, translate |
| `ternlang-lsp/` | LSP 3.17 server — hover, completion, diagnostics |
| `ternlang-compat/` | 9-trit RISC assembler (Brandon Smith bridge), Owlet S-expr parser |
| `ternpkg/` | Package manager with GitHub-backed registry |
| `ternpkg-registry/` | Registry index and metadata |

### AI & ML (BSL-1.1)

| Directory | What it is |
|-----------|-----------|
| `ternlang-ml/` | Sparse matmul, BitNet QAT, STE trainer, coalition vote, action gate |
| `ternlang-moe/` | MoE-13 orchestrator — dual-key routing, triad synthesis, 3-tier memory |
| `ternlang-mkl/` | cuTern: Math Kernel Library with native `@sparseskip` bypass |

### Runtime & Serving (BSL-1.1)

| Directory | What it is |
|-----------|-----------|
| `ternlang-api/` | REST + SSE API, multi-tenant key management |
| `ternlang-mcp/` | MCP server — 30 tools, stdio + HTTP transport |
| `ternlang-runtime/` | Distributed TCP actor runtime |
| `ternlang-studio/` | TernStudio flow canvas UI |
| `ternlang-wasm/` | WebAssembly compilation target |
| `ternlang-web/` | Web platform integration |

### Protocol & Data (BSL-1.1)

| Directory | What it is |
|-----------|-----------|
| `ternlang-tson/` | TSON: Ternary Standard Object Notation (30% denser than JSON) |
| `ternlang-ttp/` | TTP: Triadic Transfer Protocol (Status 000: Deliberating) |
| `ternlang-net/` | Triadic networking stack (Introspective Handshake) |
| `ternlang-sql/` | Native ternary graph database driver |
| `ternlang-fs/` | Triadic file system (deliberative hold transactional pend) |
| `ternlang-time/` | T-NTP: Triadic Network Time Protocol |
| `ternlang-auth/` | T-DID: Triadic Decentralized Identity |
| `ternlang-crypto/` | High-entropy trit-based cryptographic primitives |
| `ternlang-consensus/` | Triadic Byzantine Fault Tolerance (TBFT) |
| `ternlang-contract/` | T-Contract: Triadic Smart Contracts |

### Systems & Hardware (BSL-1.1)

| Directory | What it is |
|-----------|-----------|
| `ternlang-hdl/` | Verilog-2001 codegen, BET processor, FPGA simulation |
| `ternlang-posix/` | T-POSIX: Triadic OS interface |
| `ternlang-driver/` | Hardware driver bindings |
| `ternlang-gate/` | Safety gate logic |
| `ternlang-compress/` | Trit-pack compression (5 trits/byte) |
| `ternlang-grid/` | Power grid / distributed compute |
| `ternlang-bridge/` | Binary-to-ternary transpiler |

### Domain Extensions (BSL-1.1)

| Directory | What it is |
|-----------|-----------|
| `ternlang-astro/` | Interplanetary Delay-Tolerant Networking (DTN) |
| `ternlang-bci/` | Brain-Computer Interface (native inhibitory decoding) |
| `ternlang-bio/` | Bioinformatics ternary primitives |
| `ternlang-cad/` | Computer-aided design integration |
| `ternlang-edu/` | Educational / tutorial runtime |
| `ternlang-gfx/` | T-GPU: Triadic Graphics Pipeline (Depth-as-a-Trit) |
| `ternlang-harmony/` | Multi-agent harmony protocols |
| `ternlang-hft/` | High-frequency trading primitives |
| `ternlang-qutrit/` | Quantum-Classical Bridge (Qutrit native superposition) |
| `ternlang-ros2/` | ROS2 robotics integration |
| `ternlang-ruvector/` | Ternary vector database |
| `ternlang-sec/` | Security audit primitives |
| `ternlang-swarm/` | Triadic Kinematics (biological hesitation for robotics) |
| `ternlang-translator/` | Natural-language to ternary translation |
| `ternlang-ui/` | Triadic State Management & DOM Rendering |
| `ternlang-test/` | Test harness and assertion framework |

---

## Content Directories (non-crate)

### Language & Ecosystem

| Directory | What it is |
|-----------|-----------|
| `stdlib/` | 28,500+ open-core `.tern` modules (Tier 1, LGPL) |
| `examples/` | 2,090+ example programs across all domains |
| `spec/` | EBNF grammar, language reference, 30+ T-* protocol specs (`spec/standards/`) |
| `tests/` | `.tern` VM test programs and regression cases |
| `compiler/` | Compiler internals notes, legacy shim |

### Documentation

| Directory | What it is |
|-----------|-----------|
| `docs/` | Architecture, roadmap, session log, ecosystem maps, strategy |
| `whitepaper/` | IEEE two-column LaTeX whitepaper (DOI: 10.17605/OSF.IO/TZ7DC) |
| `wiki/` | Extended wiki content |

### Research & Domain Material

| Directory | What it is |
|-----------|-----------|
| `research/` | Academic and R&D papers |
| `aerospace/` | Aerospace-domain specifications and examples |
| `agent/` | Agent architecture research |
| `analytics/` | Analytics pipeline documentation |
| `apps/` | Application-layer examples |
| `architecture/` | System architecture deep-dives |
| `edge/` | Edge computing deployment docs |
| `hardware/` | Hardware platform research |
| `hdl/` | HDL design notes (see also `ternlang-hdl/`) |
| `integrations/` | Third-party integration guides |
| `kernel/` | OS kernel interface research |
| `linguist/` | Linguistics and natural language research |
| `moe/` | MoE architecture research (see also `ternlang-moe/`) |
| `negotiator/` | Multi-agent negotiation protocols |
| `network/` | Network protocol research |
| `playground/` | Interactive demos |
| `security/` | Security research and threat models |
| `stealth/` | Stealth-mode product strategy |
| `storage/` | Storage backend research |
| `torch-ternary/` | PyTorch ternary integration experiments |
| `vm/` | BET-VM design notes |
| `web/` | Web platform research |

### Business & Legal

| Directory | What it is |
|-----------|-----------|
| `legal/` | Licenses, patent filings, compliance |
| `market/` | Market analysis and positioning |
| `marketing/` | Marketing materials |
| `enterprise/` | Enterprise deployment guides |
| `tier2/`, `tier3/` | Premium-tier content stubs |

### Operational

| Directory | What it is |
|-----------|-----------|
| `scripts/` | Build, deploy, and CI scripts |
| `tools/` | Developer tooling |
| `Buglist/` | Known bug tracking |

---

## Key Files at Root

| File | What it is |
|------|-----------|
| `README.md` | Main project README |
| `Cargo.toml` | Workspace manifest (all crates) |
| `Cargo.lock` | Pinned dependency versions |
| `BENCHMARKS.md` | Measured sparse inference benchmarks |
| `CHANGELOG.md` | Version history |
| `TRAINING.md` | Model training guide |
| `ternlang.toml` | Ternlang package configuration |
| `fly.toml` | Fly.io deployment config (ternlang-api) |
| `Dockerfile` | Container build |
| `LICENSE` / `LICENSE-*` | LGPL, BSL-1.1, commercial license texts |
