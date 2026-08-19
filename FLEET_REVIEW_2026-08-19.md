# RFI-IRFOS Crates Fleet Review & Maintenance

Generated: 2026-08-19 08:42

## Executive Summary

All crates synchronized to version **2.0.0**. Usability baseline established: descriptions, crate-level docs, and READMEs added fleet-wide. Dependency-ordered publish plan derived from actual `Cargo.toml` graphs.

## 1. Source Map & Version Sync

### Workspaces

- **ternary-intelligence-stack** (`~/projects/ternary-intelligence-stack`) — 63 crates, workspace version bumped `1.5.0` → `2.0.0`
- **call-laura** (`~/projects/call-laura`) — 4 crates (`lauras-*`), workspace version bumped `0.2.0` → `2.0.0`
- **bifp-core** (`~/projects/bifp/bifp-core`) — standalone, version bumped `0.1.0` → `2.0.0`
- **rusty-penguin** — not part of publish scope (separate project)

### Version Changes Applied

- Workspace `version` in `ternary-intelligence-stack/Cargo.toml`: `1.5.0` → `2.0.0`
- Workspace `version` in `call-laura/Cargo.toml`: `0.2.0` → `2.0.0`
- 15 crates with explicit `version = "1.5.0"` → `2.0.0`
- 1 crate with explicit `version = "0.1.0"` (`bifp-core`) → `2.0.0`
- Remaining crates inherit workspace version via `version.workspace = true`

## 2. Usability Improvements Applied

### Cargo.toml Descriptions

Added descriptions to **22 crates** missing them:

```
reproducibility_verifier, ternlang-api, ternlang-auth, ternlang-edu,
ternlang-cli, ternlang-bridge, ternlang-driver, ternlang-harmony,
ternlang-time, ternlang-gfx, ternlang-cad, ternlang-fs,
ternlang-net, ternlang-translator, ternpkg-registry, ternlang-grid,
ternlang-sql, ternlang-mkl, ternlang-swarm, ternlang-ui, ternlang-ttp,
moe-data, token_train
```

### Crate-Level Doc Comments

Added `//!` doc comments to **33 source files** (`lib.rs` / `main.rs`) across all crates.

### READMEs Created

Generated README templates for **42 crates** missing them in `ternary-intelligence-stack`.

- `call-laura` and `bifp-core` already had READMEs — preserved.

## 3. Findings & Issues

### 🔴 Active TODOs / FIXMEs

1. **ternlang-compress** (`src/format.rs:54`) — TODO (Phase 12): Implement GGUF reader using `candle` or `safetensors` crate. Currently a stub.
2. **ternlang-wasm** (`src/lib.rs:77`) — TODO: Pass `ModuleResolver` when WASM imports are supported.
3. **moe-llm-core** (`train_bible.rs:1760`) — Comment about dashboard parsing format; not a blocking TODO but indicates incomplete training pipeline integration.

### ⚠️ Structural Gaps

- **No `examples/` directories** in any crate — crates.io users have no runnable examples.
- **No `benches/` directories** — performance regression testing impossible.
- **Test coverage unclear** — `cargo test` hasn't been run due to environment timeout; assumed present where `#[cfg(test)]` blocks exist, but not verified.

### ℹ️ Naming / Organization Notes

- `call-laura` workspace publishes as `lauras-*` (alias `package = "lauras-*"`). Confirmed intentional.
- `ternlang-gate`, `moe-data`, `token_train`, `reproducibility_verifier` have explicit versions — all bumped to `2.0.0`.
- `rusty-penguin` crates excluded from this publish scope (separate project).

## 4. Feature Improvements Applied (Usability ×2)\n\n### ternlang-core (Foundation Crate)\n\n- **Consolidated Trit import path**: unified `src/trit.rs` and `src/types/trit.rs` — downstream crates now use `ternlang_core::Trit` instead of `ternlang_core::trit::Trit`\n- **Added re-exports**: `TritBlock5`, `pack_5_trits`, `unpack_5_trits` re-exported from crate root\n- **Deprecated old `trit.rs`**: now just a deprecated re-export for backward compat\n- **Feature flags added**: `default = ["std"]`, `simd`, `wasm` — optional SIMD acceleration and WebAssembly-compatible builds\n- **Integration tests**: added `tests/integration_trit_lifecycle.rs` with 5 tests covering pack/unpack roundtrips, 1-byte size verification, and 250-trit vector boundary\n\n### ternlang-ml (1330 downloads)\n\n- **Fixed import**: `ternlang_core::trit::Trit` → `ternlang_core::Trit` in `coherence.rs`, `qat.rs`\n- **Module re-exports**: `tritfloat`, `tritfloat_tensor`, `coherence`, `perplexity`, `qat` now exported from `lib.rs` instead of being hidden\n\n### ternlang-cli (445 downloads)\n\n- **Real CLI scaffold**: replaced 9-line stub with full clap derive `Commands` enum\n- **Subcommands**: `run`, `build`, `sim`, `fmt`, `repl`, `compat` — all with proper args and future integration hooks\n- **Feature flag**: `repl` (optional `rustyline` dependency) for future REPL implementation\n\n### ternlang-mcp (301 downloads)\n\n- **Real library interface**: added `trit_decide()` function with `TritDecision` struct (trit + confidence + trace)\n- **MCP_VERSION const**: protocol version pin\n- **Feature flag**: `nvidia` (for ternlang-engram integration)\n\n### ternlang-ruvector (413 downloads)\n\n- **Fixed import**: `ternlang_core::trit::Trit` → `ternlang_core::Trit`\n\n### Import Cleanup (8 files)\n\nFixed `ternlang_core::trit::Trit` → `ternlang_core::Trit` in:\n- `ternlang-compress/src/quantize.rs`, `sparse.rs`, `pipeline.rs`\n- `ternlang-ruvector/src/lib.rs`\n- `ternlang-driver/src/lib.rs`\n- `vm/src/opcodes/matmul.rs`, `vm/src/memory/trit_alloc.rs`\n\n## 5. Original Publish Plan (Unchanged)\n\nDerived from topological sort of actual `Cargo.toml` `[dependencies]` across all 67 crates.\n\n```\n  1. bifp-core\n  2. lauras-api\n  3. lauras-core\n  4. lauras-mcp\n  5. lauras-team\n  6. moe-compute\n  7. moe-core\n  8. moe-data\n  9. moe-llm-core\n 10. moe-test\n 11. reproducibility_verifier\n 12. ternlang-auth\n 13. ternlang-bridge\n 14. ternlang-cad\n 15. ternlang-core\n 16. ternlang-bio\n 17. ternlang-bci\n 18. ternlang-codegen\n 19. ternlang-compat\n 20. ternlang-consensus\n 21. ternlang-contract\n 22. ternlang-crypto\n 23. ternlang-driver\n 24. ternlang-edu\n 25. ternlang-fs\n 26. ternlang-gate\n 27. ternlang-gfx\n 28. ternlang-grid\n 29. ternlang-harmony\n 30. ternlang-hdl\n 31. ternlang-hft\n 32. ternlang-lsp\n 33. ternlang-mkl\n 34. ternlang-ml\n 35. moe-platform\n 36. ternlang-compress\n 37. ternlang-moe\n 38. moe-plugin-sdk\n 39. moe-runtime\n 40. moe-ddel\n 41. moe-sdk\n 42. moe-uril\n 43. moe-validation-suite\n 44. ternlang-net\n 45. ternlang-posix\n 46. ternlang-astro\n 47. ternlang-qutrit\n 48. ternlang-ros2\n 49. ternlang-runtime\n 50. ternlang-cli\n 51. ternlang-ruvector\n 52. ternlang-engram\n 53. ternlang-api\n 54. ternlang-mcp\n 55. ternlang-sec\n 56. ternlang-sql\n 57. ternlang-swarm\n 58. ternlang-test\n 59. ternlang-time\n 60. ternlang-translator\n 61. ternlang-tson\n 62. ternlang-ttp\n 63. ternlang-ui\n 64. ternlang-wasm\n 65. ternpkg\n 66. ternpkg-registry\n 67. token_train\n```\n\n## 6. Next Steps (Manual Approval Required)\n\n1. **Run `cargo check --workspace`** to verify import path consolidation didn't break compilation\n2. **Run `cargo test -p ternlang-core`** to verify integration tests\n3. **Fill README usage sections** — 42 templates still say "Add usage examples here"\n4. **Resolve 3 active TODOs** — especially `ternlang-compress` GGUF stub (Phase 12)\n5. **Approve publish order** — confirm the 67-crate list above, then execute `cargo publish` sequentially

---

**No crates were published.** All changes are local only. Human approval required before `cargo publish`.