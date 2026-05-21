# Changelog

All notable changes to the Ternary Intelligence Stack (TIS) will be documented in this file.

## [1.5.0] — 2026-05-21

### ternlang-ml: TritFloat + TritFloatTensor
- `TritFloat` — 14-trit balanced ternary floating-point format (phase 1t + exponent 5t + mantissa 6t + confidence 2t) stored as u32. Confidence is a first-class field in the number; it propagates automatically: mul uses weakest-link, add uses averaging. 19 unit tests.
- `TritFloatTensor` — N-dimensional tensor of TritFloats with confidence-propagating matmul (Rayon-parallel), @sparseskip at the activation level, elementwise ops, softmax_rows, and bidirectional TritMatrix conversion.
- `linear_confident()` — top-level inference hot path: TritFloat activations × TritMatrix weights. @sparseskip fires on both activation zeros and weight zeros for combined sparsity savings. Returns (TritFloatTensor, skips).
- Extended TritFloat arithmetic: `div`, `recip`, `powi`, `sqrt`, `clamp`, `cmp_trit`, `softmax`, `pack_phases_u64`, `dot_prescan`.
- `TRITFLOAT_SPEC.md` — IP disclosure and format specification (confidential, local only).

## [Unreleased] — 2026-05-17

### albert. training
- ep1584: new all-time-low batch loss 9.9925 and epoch-average loss 10.0915
- ep1550: accidental AdamW buffer reset on Modal restart → 4x swing amplitude, steeper descent — net positive
- Surgery history complete: 12L→13L (ep511), →14L (ep547), →15L (ep611), →16L (ep645), →17L (ep701)
- Next surgery gate: 17L→21L at loss ≤ 9.8, gap ~0.29 nats from current

### albert. colony (SPORE protocol)
- `albert-spore` command live: auto-publishes checkpoints to private `albert-spores` repo via Git LFS
- First real spore: zabih-sudo / ep900 / loss 10.0922 — below GPU all-time-low 10.1117
- Colony fitness gate: loss < main_best + 1.0
- Zabih fully onboarded; Lisa onboarding in progress

### Research findings
- **Cross-lingual semantic broadcasting**: albert. outputs domain-correct tokens across all 8 corpus languages simultaneously before learning language selection — documented with full evidence table and 6-phase training arc
- **Surgery unlocks named entities**: ep511 (12L→13L) is the exact epoch where Joseph, David, Maria, Roman first appear in biblical prompts — each surgery unlocks a qualitatively new semantic capability class

### Infrastructure
- Training platform migrated from GCP VM (T4 exhausted 2026-05-12) to Modal.com T4
- Gate reset footgun fixed: kaiming-uniform + expert noise on restart now gated behind `--break-symmetry` + EvolutionManager entropy auto-detection; estimated ~500 wasted epochs before fix
- `bench_translate.py`: automatic English translation companion for all bench outputs
- 28 historical bench files retroactively translated, landing in `albert-moe-13/benchmarks/`

### SPRIND
- Full application submitted 2026-05-16 (Stage 1: 10 teams / €3M)
- `@sparseskip` patent pending A50296/2026 referenced throughout submission
- EU AI Act compliance mapping complete: `docs/compliance/eu_ai_act_mapping.md`

### Codebase
- Ecosystem-wide sync to v1.3.7
- `albert-moe-13/docs/` major additions: SPORE_PROTOCOL, SPARSESKIP_METHODOLOGY, EVOLUTION_EVIDENCE, MYCELIAL_CORD_ARCHITECTURE, EPIS_FRAMEWORK, CORPUS_CURRICULUM
- Dashboard: POLL_MS=500 for T4 latency, burst animation, ARCH reset, GRAD/DIV suppressed from terminal

---

## [0.3.2] - 2026-04-14

### Added
- **ModelCoherence binary format**: New packed binary format for 1B+ parameter models, reducing 1.2GB JSON to 240MB (5x reduction).
- **TritTransformer**: Llama-3 style architecture implementation in `ternlang-ml` using strictly ternary weights.
- **Inference Runner**: New binary `inference` in `ternlang-ml` for verifying full-scale model forward passes.
- **Kernels**: Optimized RMSNorm, RoPE, and SwiGLU kernels for ternary-based LLMs.

### Fixed
- Fixed several Rust compilation errors in `ternlang-ml` related to type mismatch, partial moves, and visibility.
- Resolved memory heap issues when processing large JSON models by introducing the binary transmutation pipeline.

### Changed
- Refactored `ternlang-ml` to separate serialization logic into `coherence.rs`.
- Updated `GEMINI.md` and `ROADMAP.md` to reflect Phase 12A milestones.
