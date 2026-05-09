# TIS vs BitNet: A Technical Comparison

**RFI-IRFOS | Albert MoE-13 | Patent Pending A50296/2026**

---

## Summary

Albert MoE-13 uses BitNet 1.58-bit ternary weights ({-1, 0, 1}) — the same quantization format as Microsoft's BitNet b1.58. The difference between our approach and BitNet is not in the weight format. It is in what we do with the weights at execution time.

BitNet is a quantization technique applied to existing dense transformer architectures running on standard binary hardware and frameworks (PyTorch/CUDA). The Ternary Intelligence Stack (TIS) is a complete execution stack — language, VM, training, inference runtime — built to exploit ternary sparsity at the instruction level.

---

## Where We Agree with BitNet

- Ternary weights ({-1, 0, 1}) are sufficient for competitive language modeling
- 1.58-bit quantization dramatically reduces memory bandwidth vs FP16/BF16
- Ternary arithmetic simplifies multiply-accumulate to add/subtract/noop
- The format is hardware-friendly and inference-efficient

---

## What We Add: @sparseskip

BitNet quantizes weights. It does not change which computations are executed — it changes how those computations are done (cheaper multiply). A dense BitNet transformer still runs every weight, every layer, every forward pass.

TIS adds a second dimension of sparsity: **execution sparsity** via `@sparseskip`.

In Albert's Mixture-of-Experts architecture:
- Each token is routed to 3 of 12 experts (Top-3 routing)
- The remaining 9 experts receive zero combined routing weight
- `@sparseskip` detects this via `combined_weight.max_all() == 0.0` and skips the entire MLP — not masked, not zeroed, **not executed**

This is not a masking trick. The MLP forward pass for non-routed experts does not run. At single-token inference (the typical decode step), 75% of the expert network is not executing.

**Measured result (2026-05-07, CPU only, ThinkPad-class laptop):**
- 83 tokens/second sustained decode
- 9/12 experts skipped per decode token
- No GPU required

The `@sparseskip` primitive is also expressed as the `TSPARSE_MATMUL` opcode in the BET VM — a native sparse matrix multiply at the instruction level that skips multiply operations on zero weights. This is the mechanism behind Patent A50296/2026.

---

## Comparison Table

| Dimension | BitNet b1.58 | Albert MoE-13 (TIS) |
|---|---|---|
| Weight format | {-1, 0, 1} ternary | {-1, 0, 1} ternary (same) |
| Framework | PyTorch / standard | Custom BET VM + Rust runtime |
| Architecture | Dense transformer | Sparse MoE (Top-3 / 12 experts) |
| Execution sparsity | None (all weights computed) | @sparseskip: 75% experts skipped at decode |
| Training framework | PyTorch-based | Candle (Rust) — full-stack ownership |
| Inference target | GPU (typical) | CPU-first (83 tok/s measured) |
| Stack ownership | Technique on existing stack | Language + VM + model + tooling |
| Patent | — | A50296/2026 (@sparseskip / TSPARSE_MATMUL) |
| MCP tooling | — | 30 free tools on Smithery |
| Open source | Research code | Full stack (34 crates on crates.io) |

---

## What We Do Not Claim

- We do not claim our model outperforms larger BitNet models on benchmark tasks. Albert MoE-13 is a small research model (256 hidden, 4 layers, 8k vocab) — not a production-scale LLM.
- We do not claim ternary weights eliminate hallucination. No current architecture does.
- The theoretical 122× speedup figure in the whitepaper refers to native ternary ASIC hardware at near-100% weight sparsity — not the x86 CPU numbers above. The measured x86 speedup from @sparseskip at 75% expert sparsity is in the 2–5× range (see `sparseskip_throughput` benchmark).

---

## The Actual Differentiator

BitNet is a quantization method you apply to a model. TIS is a stack you build on.

The value of TIS is not that it outperforms BitNet at any specific benchmark today — it's that the entire inference chain, from weight encoding through VM opcode through routing decision, is owned, understood, and optimizable by the team building on it. When the next sparsity primitive matters, we implement it directly in the ISA. We don't wait for a framework update.

That is the architectural bet.

---

**Reproduce the @sparseskip benchmark:**
```
cargo run --release --bin sparseskip_throughput -p moe-llm-core
```

**Repository:** https://github.com/eriirfos-eng/ternary-intelligence-stack
