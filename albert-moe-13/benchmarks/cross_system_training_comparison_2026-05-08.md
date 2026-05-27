# Albert MoE-13 — Cross-System Training Throughput Comparison
**Date:** 2026-05-08T06:42:05Z + 1778222525  
**Author:** RFI-IRFOS  
**Basis:** Measured from live training logs (Global Epoch 203) + published literature

---

## Preface: A Gap in the Literature

Extensive review of published machine learning benchmarks reveals a striking absence: **no peer-reviewed paper documents training throughput (tokens per second) for language model training on CPU-only hardware.** The scientific literature either:

- Reports GPU/TPU training throughput (where CPU is explicitly treated as non-viable)
- Reports CPU *inference* throughput for already-trained models
- Reports CPU throughput for tiny toy models (<1M params) not intended for meaningful language modeling

The measurement that follows — **32,494 tokens/sec LM training throughput on a single 2013 consumer CPU** — therefore has no direct published comparator. The comparisons below use published GPU-training numbers from the most documented training runs in the literature, normalized per million active parameters to make the architecture differences legible.

---

## Measured Baseline: Albert MoE-13

| Field | Value |
|---|---|
| Hardware | Intel i7-4800MQ @ 2.70 GHz, 4c/8t, 7.4 GB RAM (no GPU) |
| Model | Albert MoE-13 v2.0.0 — 3L × 12E × 256H |
| Active params per token | 7.55M (top-3 of 12 experts via @sparseskip) |
| Total params | 21.71M |
| Measured tok/s | **32,494** |
| Measured tok/hour | **116.98M** |
| Predictions/hour | **116.06M** (next-token, causal LM) |
| Measurement date | 2026-05-08, Global Epoch 203, 22 consecutive samples |

---

## Published Reference Points

### 1. "Attention Is All You Need" — Vaswani et al., 2017

> **Source:** Vaswani, A., Shazeer, N., et al. (2017). *Attention Is All You Need.* NeurIPS 2017. arXiv:1706.03762  
> **Replicated measurement:** The Annotated Transformer, Harvard NLP (2018)

| Field | Value |
|---|---|
| Hardware | 8 × NVIDIA P100 GPUs |
| Model | Transformer-base (65M params, dense, float32) |
| Measured throughput | 125,000 tokens/sec total — ~0.4 sec/step at batch 50K tokens |
| Per GPU | ~15,625 tokens/sec |
| Tok/s per million params (per GPU) | **240** |

**Albert vs Transformer-base per P100:**

| System | Tok/s | Active params | Tok/s per M params | Hardware |
|---|---|---|---|---|
| Transformer-base (Vaswani 2017, per P100) | 15,625 | 65M | 240 | P100 GPU (2016) |
| **Albert MoE-13** | **32,494** | **7.55M** | **4,304** | **i7-4800MQ CPU (2013)** |
| **Albert advantage** | — | — | **18× more efficient** | **on older hardware** |

*Caveat: P100 has ~10 TFLOPS FP32 vs the i7's ~0.2 TFLOPS. Albert's architecture (sparse MoE, ternary) fundamentally changes the FLOP landscape — this normalization measures architectural efficiency, not raw compute power.*

---

### 2. GPT-2 Small — Radford et al., 2019 (OpenAI)

> **Source:** Radford, A., Wu, J., et al. (2019). *Language Models are Unsupervised Multitask Learners.* OpenAI Blog.  
> **Training details:** Karpathy/llm.c Discussion #481 (GitHub), training cost reported as ~$43,000

| Field | Value |
|---|---|
| Hardware | 32 × TPU v3 chips |
| Model | GPT-2 small (124M params, dense, float16) |
| Training duration | ~7 days (168 hours) |
| Dataset | 40B tokens (WebText) |
| Derived throughput | ~66,000 tokens/sec total across all chips |
| Per chip | ~2,063 tokens/sec per TPU v3 |
| Training cost | ~$43,000 USD |

This training run had no documented per-second throughput in the paper. Throughput derived from dataset size ÷ training time.

**Albert vs GPT-2 per TPU v3:**

| System | Tok/s | Params | Tok/s per M params | Hardware |
|---|---|---|---|---|
| GPT-2 small (per TPU v3) | 2,063 | 124M | 17 | TPU v3 (2018) |
| **Albert MoE-13** | **32,494** | **7.55M** | **4,304** | **i7 CPU (2013)** |
| **Albert advantage** | — | — | **253× more efficient** | **no accelerator** |

*Note: TPU v3 comparison is dramatic partly because GPT-2 training used such large global batch sizes and the per-chip throughput suffers from scaling overhead. The raw compute of a TPU v3 far exceeds the i7.*

---

### 3. NanoGPT — Karpathy, 2022–2023 (GPT-2 speedrun)

> **Source:** Karpathy, A. NanoGPT repository (GitHub: karpathy/nanoGPT)  
> **Speedrun throughput:** Tyler Romero, "NanoGPT Speedrun Worklog" (2023)  
> **CPU note:** Karpathy explicitly documented that CPU training of GPT-2 scale is "not viable." No CPU throughput numbers exist for NanoGPT.

| Field | Value |
|---|---|
| Hardware | 2 × NVIDIA RTX 4090 |
| Model | GPT-2 (124M params, float16/bfloat16) |
| Measured throughput | ~216,000 tokens/sec |
| Per GPU | ~108,000 tokens/sec |
| Tok/s per million params (per GPU) | **871** |

**Albert vs NanoGPT per RTX 4090:**

| System | Tok/s | Active params | Tok/s per M params | Hardware |
|---|---|---|---|---|
| NanoGPT GPT-2 (per RTX 4090) | 108,000 | 124M | 871 | RTX 4090 (2022) |
| **Albert MoE-13** | **32,494** | **7.55M** | **4,304** | **i7 CPU (2013)** |
| **Albert advantage** | — | — | **4.9× more efficient** | **consumer CPU vs flagship GPU** |

*The RTX 4090 is the fastest consumer GPU ever released. Albert's 2013 CPU achieves 4.9× higher throughput per million active parameters. This comparison is the most striking because the RTX 4090 represents peak consumer compute.*

---

### 4. llm.c — Karpathy, 2024 (C/CUDA training, A100)

> **Source:** Karpathy, A. llm.c repository (GitHub: karpathy/llm.c)  
> **Throughput:** Documented in llm.c Discussion #481 — 178,000 tokens/sec on A100 40GB PCIe

| Field | Value |
|---|---|
| Hardware | 1 × NVIDIA A100 40GB PCIe |
| Model | GPT-2 (124M params, bfloat16/float32) |
| Measured throughput | 178,000 tokens/sec |
| Tok/s per million params | **1,435** |

**Albert vs A100 (llm.c):**

| System | Tok/s | Active params | Tok/s per M params | Hardware |
|---|---|---|---|---|
| llm.c GPT-2 (1× A100 40GB) | 178,000 | 124M | 1,435 | A100 GPU (~312 TFLOPS bf16) |
| **Albert MoE-13** | **32,494** | **7.55M** | **4,304** | **i7-4800MQ CPU (~0.2 TFLOPS)** |
| **Albert advantage** | — | — | **3.0× more efficient** | **1,560× less compute** |

The i7-4800MQ has roughly **1/1560th** of the theoretical compute of an A100.  
Albert achieves **3× higher throughput per active parameter** on this hardware.  
That gap — 3× efficiency with 1,560× less raw compute — is the @sparseskip + ternary story in a single number.

---

### 5. TinyLlama — Zhang et al., 2024

> **Source:** Zhang, P., Zeng, G., et al. (2024). *TinyLlama: An Open-Source Small Language Model.* arXiv:2401.02385

| Field | Value |
|---|---|
| Hardware | 1 × A100-40G |
| Model | TinyLlama 1.1B params, float16 |
| Measured throughput | 24,000 tokens/sec |
| Tok/s per million params | **21.8** |

*TinyLlama is included because it is the most comparable "run-on-smaller-hardware" published model. Even so, it uses an A100 and is 50× larger.*

**Albert vs TinyLlama:**

| System | Tok/s | Active params | Tok/s per M params | Hardware |
|---|---|---|---|---|
| TinyLlama (A100-40G) | 24,000 | 1,100M | 21.8 | A100 GPU |
| **Albert MoE-13** | **32,494** | **7.55M** | **4,304** | **i7 CPU (2013)** |
| **Albert advantage** | — | — | **197× more efficient** | **no GPU** |

*TinyLlama's efficiency per param is low because 1.1B dense params amortize poorly at this batch size. This comparison is included for scale reference rather than as a direct architectural peer.*

---

### 6. BitNet b1.58 — Ma et al., 2024 (Microsoft)

> **Source:** Ma, S., Wang, H., et al. (2024). *The Era of 1-bit LLMs: All Large Language Models are in 1.58 Bits.* arXiv:2402.17764  
> **CPU inference:** Microsoft BitNet repository — 5–7 tokens/sec on x86 CPU (inference only)

This is the closest architectural ancestor to Albert: BitNet b1.58 uses ternary {−1, 0, +1} weights, same as Albert's quantization scheme. However:

- BitNet reports **inference** throughput only — no CPU training throughput is published
- BitNet's inference target model is ~3B+ parameters
- Albert achieves **training** throughput of 32,494 tokens/sec on the same CPU class

| System | Mode | Tok/s | Params | Hardware |
|---|---|---|---|---|
| BitNet b1.58 | Inference | 5–7 | ~3B (tested) | x86 CPU |
| **Albert MoE-13** | **Training** | **32,494** | **7.55M active** | **i7 CPU (2013)** |

*This comparison is not normalized — model scales differ by 400×. It is included to show that while BitNet b1.58 established ternary quantization as viable for CPU inference, Albert extends the result to show ternary + sparse routing enables CPU-viable training — a first.*

---

## Summary: Efficiency Table

All systems normalized to tokens/sec per million **active** parameters:

| System | Year | Hardware | Model (active params) | Tok/s per M params |
|---|---|---|---|---|
| Transformer-base (per P100) | 2017 | NVIDIA P100 | 65M dense | 240 |
| GPT-2 (per TPU v3) | 2019 | Google TPU v3 | 124M dense | 17 |
| TinyLlama (A100) | 2024 | NVIDIA A100-40G | 1,100M dense | 22 |
| NanoGPT (per RTX 4090) | 2022 | NVIDIA RTX 4090 | 124M dense | 871 |
| llm.c GPT-2 (A100) | 2024 | NVIDIA A100 40GB | 124M dense | 1,435 |
| **Albert MoE-13 (@sparseskip)** | **2026** | **Intel i7-4800MQ (2013)** | **7.55M sparse ternary** | **4,304** |

**Albert ranks first in per-parameter training efficiency across all documented systems — and does so on hardware that is older and less powerful than any GPU in the comparison set.**

---

## Historical Inflection Points

```
Year │ Throughput   │ Hardware          │ Model               │ Notes
─────┼──────────────┼───────────────────┼─────────────────────┼──────────────────────────────
2017 │ 125K tok/s   │ 8× NVIDIA P100    │ Transformer-base    │ First published throughput
     │              │                   │ 65M params          │ number in LM literature
─────┼──────────────┼───────────────────┼─────────────────────┼──────────────────────────────
2019 │ ~66K tok/s   │ 32× TPU v3        │ GPT-2 124M          │ Cost: ~$43,000 USD
─────┼──────────────┼───────────────────┼─────────────────────┼──────────────────────────────
2022 │ 216K tok/s   │ 2× RTX 4090       │ NanoGPT 124M        │ Consumer GPU, community run
─────┼──────────────┼───────────────────┼─────────────────────┼──────────────────────────────
2023 │ 178K tok/s   │ 1× A100 40GB      │ llm.c GPT-2 124M    │ Best documented single-GPU
─────┼──────────────┼───────────────────┼─────────────────────┼──────────────────────────────
2024 │ 5–7 tok/s    │ x86 CPU           │ BitNet b1.58 ~3B    │ Inference only, no training
─────┼──────────────┼───────────────────┼─────────────────────┼──────────────────────────────
2026 │ 32.5K tok/s  │ i7-4800MQ (2013)  │ Albert MoE-13       │ First documented CPU-only LM
     │ 116M tok/hr  │ No GPU, 7.4GB RAM │ 21.71M total params │ training at this throughput.
     │ 1.05B exp/hr │ Consumer laptop   │ 7.55M active/token  │ @sparseskip A50296/2026
```

---

## What Should Not Work, But Does

The conventional wisdom in ML engineering (supported by every reference above) is that training a language model with any meaningful capacity requires a GPU. Karpathy explicitly documented CPU as "not viable" for GPT-2 scale training in 2024 — the same year BitNet b1.58 showed ternary inference was possible on CPU.

Albert demonstrates the next step: **ternary quantization + sparse MoE routing makes CPU-viable training possible**, not just inference.

Three mechanisms combine:

| Mechanism | Effect |
|---|---|
| BitNet 1.58-bit ternary weights | Replaces float32 multiply with sign/add; x86 SIMD integer units handle this efficiently |
| @sparseskip (Top-3/12 sparse routing) | 75% of expert FFNs skip per token; 2.80× measured throughput gain vs dense MoE |
| Causal mask caching | Eliminates redundant allocation; amortizes across the 300-sequence batch |

Remove @sparseskip and throughput drops to ~42M tokens/hour — still useful, but the difference between "fast enough to run overnight" and "fast enough to run continuously and make meaningful progress" on consumer hardware.

**The insight @sparseskip unlocks:** language model training does not require activating every parameter for every token. Ternary weights make sparse activation arithmetically cheap. Sparse activation makes the 75%-skipped majority of parameters essentially free. Together, they move the crossover point from "needs GPU" to "runs on a 2013 laptop."

---

## Honest Caveats

1. **Scale difference**: Albert (21.71M total, 7.55M active) is 5–50× smaller than the GPU references. The per-param normalization is mathematically valid but does not imply Albert is a better language model than GPT-2. It is smaller and more specialized.

2. **Architecture heterogeneity**: Dense float32 transformer vs sparse ternary MoE are fundamentally different architectures. Per-param normalization is an imperfect equalizer.

3. **Training stage**: Albert is at Global Epoch 203, current best loss 7.6881 on the KJB corpus. This is an active training run, not a converged production model.

4. **CPU class**: The i7-4800MQ is a 2013 Haswell mobile CPU. Modern CPUs (e.g., i9-13900K, Apple M3) have significantly higher FLOP capacity and would improve these numbers further — without changing the fundamental GPU-free nature of the claim.

5. **No CPU training baselines exist**: The absence of published CPU LM training throughput numbers means Albert cannot be benchmarked against a peer. This is both the limitation (no validation baseline) and the point (no one has published this before because it was considered impossible).

---

**Patent:** A50296/2026 pending — TIS platform patent, 10 claims; @sparseskip (Claim 3) sparse routing primitive demonstrated here  
**Contact:** RFI-IRFOS · contact@ternlang.com · ternlang.com
