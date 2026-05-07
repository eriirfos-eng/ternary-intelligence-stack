# Changelog
All notable changes to the Ternary Intelligence Stack. Follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

---

## [Unreleased]

### Added
- Staged corpus injection: `load_corpus(num_layers)` unlocks richer data automatically on each Net2Net surgery (stage_3 → stage_13)
- `scripts/eval_perplexity.py`: held-out perplexity harness with unigram baseline for SPRIND audit trail
- `docs/SPRIND_PITCH_CHECKLIST.md`: honest done/pending tracker for frontier pitch credibility
- Phase 25 (tern-forge) and Phase 24 (instruction following) added to ROADMAP.md
- PR/Issue templates, CONTRIBUTING.md, CODE_OF_CONDUCT.md at repo root

### Fixed
- SQL injection: `data_query` endpoint now uses allowlisted query keys — raw SQL from callers rejected
- `.env` removed from version control; `.env.example` committed as template
- LICENSE upgraded from LGPL-2.1 to LGPL-3.0 to match Cargo.toml declaration

---

## [2.0.0] — 2026-05-07 (Albert MoE-13 v2.0.0)

### Added
- **@sparseskip element-level** (Patent Pending A50296/2026): `SparseCache` with `pos_indices[i]`/`neg_indices[i]` per output neuron; `forward_sparse()` skips ~56% of multiplications
- **KV-cache**: `Attention.kv_cache` — prefill stores full K/V; decode is O(1) per step. Result: **83–125 tok/s** from ~5 tok/s baseline
- **Pre-ternarized weight cache**: `prepare_inference()` computes ternary weights once at model load
- **F32 MoE gate**: routing gate changed from TernaryLinear to `candle_nn::Linear` — fixes routing collapse (0.333/0.333/0.333) caused by ternary resolution too coarse at 256→12 scale
- **Collapse→surgery escalation**: when best checkpoint doesn't exist or is also above collapse threshold, rollback skipped and surgery fires directly
- **Whitepaper §N.N annotations** across all core algorithm files
- **GPU backend sketch**: `cuda_matmul.rs` — INT2-packed weights, dp4a kernel outline, candle CustomOp1 skeleton (TRL 3)
- **Dashboard live TELE**: every 30 batches (~60s) instead of epoch-only
- **Q&A instruction corpus**: `scripts/generate_qa_corpus.py` → 2196 User:/Albert: pairs from Wikipedia, Bible, Gutenberg

### Changed
- Collapse streak limit: 3 → 2 (surgery fires one epoch sooner at 6L+)
- Per-layer grad norm display: 4dp → 6dp for sub-millinorm visibility
- Scraper scripts moved from repo root to `scripts/`
- `training.log` and `training.pid` gitignored

### Architecture (Auto-Evolutionary)
- Albert expanded autonomously: 3L → 4L → 5L → 6L via Net2Net surgery
- Current: 6L · 256H · 12E · Top-3 routing · 128CTX · 8000 vocab · ~35M params

---

## [1.3.6] — 2026-05-06

### Added
- All 38 publishable crates synced to v1.3.6 on crates.io
- Joint Alice + Bible 8000-token WordLevel tokenizer
- Expert Routing Heatmap + Per-Layer Gradient Norm panels in dashboard
- SPRIND deliverables: EVOLUTION_EVIDENCE.md, EU AI Act mapping, public telemetry endpoint
- Three telemetry bugs fixed (GRAD, ROUTE, TELE)
- ratatui TUI REPL for ternlang-cli
- HITL redesign: below-input panel with ↑↓ navigation
- KPI workflow fixed end-to-end via HTTP upload endpoint

### Fixed
- Gradient clipping: AdamW called once per batch (was 16×, causing 50× slowdown)
- Routing collapse: threshold 0.05 → 0.0 (was zeroing all gate logits post-surgery)
- TELE sparsity check: uses `config.layer_threshold(li)` (was hardcoded 0.1)

---

## [1.3.5] — 2026-05-03 (Great Release)

### Added
- 28,500+ open-core stdlib modules
- ExaTern SIMD: AVX2 trit packing + zero-copy array views
- Scientific hardware sparsity benchmarks: 1.63× ternary throughput, 2.84× at 90% sparsity
- LLB protocol: deterministic filesystem safety gate
- Full v1.3.5 technical report and SPRIND-grade audit documentation

---

## [1.0.0] — 2026-04-17 (First Stable Release)

### Added
- ternlang compiler, BET VM, MCP server declared stable
- VS Code extension `rfi-irfos.ternlang` published
- 30 MCP tools (all free)
- TSPARSE_MATMUL: @sparseskip origin — 56% sparsity → 2.3× fewer multiply ops
