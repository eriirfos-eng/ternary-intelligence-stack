# /scripts — RFI-IRFOS Ecosystem Tooling

## ⚠️ IMPORTANT: These scripts are NOT the Albert training pipeline

`transmute_llama.py` and `transmute_full_llama.py` are **Phase 12 POC bridge tools**
designed for ecosystem interoperability testing — specifically, validating that the
ternary quantization math produces identical results whether applied to externally-sourced
dense models (LLaMA) or natively trained checkpoints.

**Albert MoE-13 trains natively from scratch.** It does not use, depend on, or require
any externally trained model weights. The Albert training pipeline is:

```
albert-moe-13/moe-llm-core/src/bin/train_bible.rs   ← training loop + EvolutionManager
albert-moe-13/moe-llm-core/src/bin/quantize_model.rs ← F32 → .trit export
albert-moe-13/moe-llm-core/src/ste.rs               ← Straight-Through Estimator math
albert-moe-13/moe-llm-core/src/bin/inspect.rs        ← weight inspection
```

## What these scripts actually do

| Script | Purpose |
|---|---|
| `transmute_llama.py` | POC: reads a LLaMA GGUF blob, applies BitNet-style threshold quantization to a single MLP layer, demonstrates {-1, 0, +1} output. Used to validate that the Python quantization math matches the Rust `pack_tensor` implementation. |
| `transmute_full_llama.py` | POC: full-model variant of above, packs all layers into a `.tern.json` artifact for cross-language verification. |

These scripts exist because the ternary ecosystem must be able to **import** weights from
the broader open-source landscape — not because Albert relies on that landscape.
An ecosystem that can only consume natively trained models has no interoperability story.
These scripts prove the interoperability layer works.

## The native Albert training claim

Albert trains on a multilingual corpus (Wikipedia CC BY-SA, Europarl, Gutenberg, EU AI Act —
EN/DE/FR/ES/PT/IT/NL/PL) using a from-scratch Rust training pipeline built on
[candle](https://github.com/huggingface/candle). The active checkpoint (`albert_v3.0.safetensors`)
is the direct output of this native pipeline — not transmuted from any external source.
Historical checkpoints: `bible_ternary_v2.0.0.safetensors` (v2.0.0, archived at Global Epoch 477+).

The EvolutionManager (in `train_bible.rs`) orchestrates automatic layer growth from a
minimal 3-layer inception state, expanding the architecture as the model achieves mastery
thresholds, without human intervention.
