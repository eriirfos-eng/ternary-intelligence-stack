# SPRIND Pitch & Global Community Credibility Checklist

**SUBMITTED: 2026-05-16** — Application submitted to SPRIND. This checklist is now archived as a pre-submission record.
One item was still open at submission: `unsafe` SIMD `// SAFETY:` comments in `moe-core/src/core/model_adapter/simd_kernels.rs` — tracked separately as a post-submission code quality task.

---

Last updated: 2026-05-07 | Maintained by: Lead Architect (Simeon Kepp)

This document tracks what is needed to be taken seriously by the global ML research community and SPRIND evaluators. It is honest about what is done and what is not. Items are never marked done without a verifiable artifact.

---

## Security (Blocker)

| Item | Status | Artifact |
|------|--------|----------|
| SQL injection via `data_query` endpoint | ✅ Fixed 2026-05-07 | Allowlist-only query keys, raw SQL rejected |
| `.env` file removed from repo | ✅ Fixed 2026-05-07 | `.env.example` committed, `.env` gitignored |
| Auth middleware applied globally | ✅ Confirmed | `require_api_key` layer on all routes |
| `unsafe` SIMD blocks have `// SAFETY:` comments | ⬜ Pending | `moe-core/src/core/model_adapter/simd_kernels.rs` |

---

## ML Research Credibility

| Item | Status | Artifact |
|------|--------|----------|
| Held-out test set perplexity | ⬜ Pending | `scripts/eval_perplexity.py` (harness ready, needs albert-test --eval mode) |
| Float32 baseline on same corpus | ⬜ Pending | Needed for "ternary vs float" comparison |
| Training curve evidence (loss vs epoch) | ✅ Live | Dashboard + `dashboard/training.log` |
| Sparsity speedup benchmarks | ✅ Published | `BENCHMARKS.md` — 2.3× at 56%, 2.84× at 90% |
| Benchmark reproducibility (Docker) | ⬜ Pending | No published container or pinned environment yet |
| Comparison to Pythia/BitNet on same corpus | ⬜ Pending | Key SPRIND differentiator |
| Auto-evolution evidence (3L→6L surgery log) | ✅ Published | `albert-moe-13/docs/EVOLUTION_EVIDENCE.md` |
| Patent pending reference | ✅ Filed | A50296/2026 in whitepaper + source headers |
| SPRIND whitepaper | ✅ Submitted | `docs/tis-sprind-submission-2026.tex` (deadline May 15) |
| EU AI Act compliance mapping | ✅ Done | `docs/compliance/eu_ai_act_mapping.md` |
| Reproducibility spec | ✅ Done | `docs/REPRODUCIBILITY.md` + `repro_check.rs` |

---

## Inference Credibility

| Item | Status | Artifact |
|------|--------|----------|
| 83-125 tok/s CPU demo | ✅ Live | albert-test with KV-cache + @sparseskip |
| @sparseskip element-level implementation | ✅ Shipped | `ternary_linear.rs` `forward_sparse()` |
| KV-cache implementation | ✅ Shipped | `attention.rs` `forward_decode()` |
| Expert-level sparse routing | ✅ Shipped | `moe.rs` — 9/12 experts skipped per decode |
| GPU backend | ⬜ Roadmap | `cuda_matmul.rs` sketch — TRL 3 |
| CUDA dp4a kernel (INT2-packed) | ⬜ Roadmap | ~5-6 engineer-weeks estimate |

---

## Code Quality

| Item | Status | Artifact |
|------|--------|----------|
| CI covers core crates | ✅ Partial | 5/30+ crates in `rust.yml` |
| CI covers ternlang-api | ⬜ Missing | Largest production binary, 0 CI tests |
| Auth flow integration tests | ⬜ Missing | Key security boundary, untested in CI |
| API modularization | ⬜ Backlog | `ternlang-api/src/main.rs` is 5363 lines |
| CHANGELOG.md at workspace root | ⬜ Missing | Needed for semver credibility |

---

## Immediate Next Actions (ordered by impact)

1. **Add `--eval` mode to albert-test** → enables `eval_perplexity.py` to produce real held-out perplexity
2. **Run perplexity eval and publish** → `eval_results.json` becomes the benchmark artifact for SPRIND
3. **Add `// SAFETY:` comments to unsafe SIMD blocks** → 30-minute task, removes a reviewer red flag
4. **Add ternlang-api to CI** → even one smoke test is better than zero
5. **Publish benchmark Docker image** → `docker run rfi-irfos/tis-bench` should reproduce the sparsity speedup table

---

## What We Have That Nobody Else Has

- Auto-evolutionary ternary MoE training from scratch (3L→6L autonomous surgery, live)
- Patent pending A50296/2026 (TIS platform, 10 claims) — @sparseskip element-level sparse matmul primitive = Claim 3
- 83-125 tok/s on laptop CPU with no GPU — proven live, not projected
- Full Rust implementation (candle) with ternary STE training
- EU AI Act compliance mapped article by article
- Open-source from day one, all 30+ crates published to crates.io
