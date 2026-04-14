# Changelog

All notable changes to the Ternary Intelligence Stack (TIS) will be documented in this file.

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
