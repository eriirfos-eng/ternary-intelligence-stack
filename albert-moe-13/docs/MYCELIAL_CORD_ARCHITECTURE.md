# Mycelial Cord Architecture — Design Specification

**Project:** Albert-MoE-13 / Ternary Intelligence Stack  
**Prepared by:** RFI-IRFOS  
**Date:** 2026-05-14  
**Status:** Pre-implementation design — no code exists yet  
**Patent basis:** Extension of A50296/2026 (@sparseskip method); cord architecture is a distinct novel contribution

---

## Motivation

Albert. grows by depth: net2net surgery adds layers when the current architecture has exhausted what it can learn from the existing corpus. This works until hidden dimension becomes the binding constraint. At 256H, the representational bandwidth per layer is finite — adding more layers eventually stops returning improvement because each layer has insufficient width to meaningfully transform what it receives.

The conventional engineering response is to expand hidden dimension: 256H → 512H. This requires reshaping every tensor in every layer of the model — a destructive, high-risk operation with no clean net2net analogue for ternary weights.

**The biological observation:** mycelium does not widen its hyphae when it hits a substrate density limit. It bundles. Multiple threads of the same diameter aggregate into a mycelial cord, increasing transport bandwidth through parallelism rather than increased fiber width. Where two independent threads meet and recognise chemical compatibility, they fuse — anastomosis — creating a shortcut between distant regions of the network.

This document specifies an architecture that follows the same logic: two independent ternary streams of identical width running in parallel, fusing at sparse biologically-motivated intervals, producing a model with doubled representational capacity that requires no reshaping of any existing weight.

---

## Architecture Overview

### Current (single-stream)

```
Input → [Block_0] → [Block_1] → ... → [Block_16] → LM Head → Loss
         256H         256H               256H
```

Each block: RMSNorm → Attention (RoPE) → MoE gate → Top-3 experts → residual

### After cord surgery (dual-stream)

```
Input ──┬─→ [Block_0_A] → [Block_1_A] ──[ANAST_2]──→ [Block_2_A] → ... ──→ Merge → LM Head → Loss
        │        256H          256H      ↕ F32 gate       256H
        └─→ [Block_0_B] → [Block_1_B] ──[ANAST_2]──→ [Block_2_B] → ... ──→
                 256H          256H                       256H
```

Anastomosis layers (A): Fibonacci positions in the layer sequence — [2, 3, 5, 8, 13] for a 17L model.  
Final merge: additive mean of stream A and stream B hidden states before LM head.

---

## Key Design Decisions

### 1. Stream initialisation — net2net cord surgery

Stream A: all existing weights copied unchanged. The current training state, learning history, and expert specialisations are fully preserved. Stream A at t=0 is identical to the pre-surgery model.

Stream B: all weights copied from Stream A, then Mandelbrot perturbation applied to every tensor across all layers. The perturbation uses the same `MandelbrotSurgery` implementation as depth surgery, with a distinct `c_im` derived from `stream_index` rather than `layer_index` — placing Stream B at a different latitude in Mandelbrot parameter space than any existing layer.

At t=0 post-surgery: both streams compute identical outputs (Stream B ≈ Stream A + small perturbation). Loss is approximately unchanged. Gradients immediately begin differentiating the streams because the perturbation breaks symmetry deterministically.

### 2. Experts — shared weights, independent routing

Each stream uses the **same 12 expert FFN weights** but has its **own routing gate** (the F32 gate linear that produces per-token expert logits). Stream A and Stream B each independently decide which 3 of the 12 experts to activate for each token, based on their own hidden states at that layer.

This means:
- Expert weights receive the full training gradient (not halved, as would happen with independent expert sets)
- The streams differentiate through routing behaviour, not through separate expert knowledge
- An expert that Stream A rarely uses is available for Stream B to specialise
- Biologically accurate: the shared experts are the substrate resources; the two streams compete and cooperate to access them according to their own internal state

Independent expert sets (24 total) are a possible future upgrade — not in v1.

### 3. Anastomosis — sparse Fibonacci gate

Anastomosis layers do not appear at every block. They appear at Fibonacci-indexed positions within the layer sequence:

For a 17L model: fusion at layers [2, 3, 5, 8, 13] — 5 fusion points out of 17 layers.

At each anastomosis layer, after both streams have computed their block output, a thin F32 gate computes bidirectional influence:

```
h_combined = concat([h_a, h_b], dim=-1)     # [B, S, 2×hidden]
g = sigmoid(Linear_F32(h_combined))          # [B, S, 2]
h_a' = h_a + g[:,:,0:1] × h_b              # Stream A absorbs from B
h_b' = h_b + g[:,:,1:2] × h_a              # Stream B absorbs from A
```

The gate linear (`[2×hidden → 2]`, F32) is initialised near zero — at t=0, `g ≈ 0.5` and cross-influence is minimal. The model learns to open the gate selectively as the streams develop different specialisations that are worth sharing.

The Fibonacci spacing is deliberate: sparse, self-similar, not uniform. Dense fusion at every layer would couple the streams too tightly — they would fail to specialise independently. Sparse fusion at Fibonacci intervals gives each stream long enough uninterrupted runs to develop its own routing patterns before the next junction.

### 4. Final merge

After the last block, both streams produce hidden states `h_a` and `h_b` of shape `[B, S, 256]`. These are merged before the LM head:

```
h_merged = (h_a + h_b) / 2
logits = LM_Head(h_merged)
```

Additive mean at t=0 is equivalent to the original single-stream output (since both streams start identical). Over training, `h_a` and `h_b` diverge and the merge naturally ensembles their predictions.

A learned merge gate (F32, same pattern as anastomosis) is a possible upgrade for v2.

### 5. Tensor naming convention

Current checkpoint format:
```
blocks.{layer}.{module}.{param}
embed.weight
lm_head.weight
```

Post-cord-surgery format:
```
blocks.{layer}.stream_a.{module}.{param}
blocks.{layer}.stream_b.{module}.{param}
blocks.{layer}.experts.{n}.{module}.{param}   ← shared, no stream prefix
anastomosis.{fusion_idx}.gate.weight
anastomosis.{fusion_idx}.gate.bias
embed.weight                                   ← shared, single embedding
lm_head.weight                                 ← shared, single LM head
```

The shared embedding and LM head mean the model's token representation and output projection are unified — the streams process the same input and compete to produce the best representation for the same output vocabulary. This is correct: mycelial threads share access to the same substrate and produce the same fruiting body.

### 6. Config changes

New fields in `albert_v3.0.config.json`:

```json
{
  "num_streams": 2,
  "fusion_layers": [2, 3, 5, 8, 13],
  "fusion_type": "bidirectional_gate"
}
```

`num_streams: 1` (or absent) = current single-stream behaviour. The architecture is fully backwards-compatible: a single-stream checkpoint loads and runs as before. Cord surgery is a one-way migration of the checkpoint format.

---

## Surgery Implementation Plan

`perform_cord_surgery()` in `train_bible.rs`:

1. Load best checkpoint tensors
2. Rename all `blocks.{l}.{module}.{param}` → `blocks.{l}.stream_a.{module}.{param}`, except expert tensors which become `blocks.{l}.experts.{n}.{module}.{param}` (shared)
3. Copy all stream_a tensors → stream_b equivalents
4. Apply `MandelbrotSurgery::perturb()` to every stream_b tensor (using `stream_index=1` as the `c_im` seed key)
5. Initialise anastomosis gate tensors (F32, near-zero, shape `[2*hidden_size → 2]` per fusion layer)
6. Update config: `num_streams = 2`, `fusion_layers = fibonacci_up_to(num_layers)`
7. Archive pre-surgery best checkpoint
8. Save new checkpoint

The surgery fires once, manually triggered — it is not part of the EvolutionManager's autonomous plateau gate. It is a conscious architectural decision made by the team when the depth-growth plateau is diagnosed. The EvolutionManager continues to govern depth surgery (new layers added to both streams simultaneously) after the cord is established.

---

## Training dynamics — expected behaviour

### Immediately post-surgery (ep 0–5)
- Loss approximately unchanged (net2net function preservation)
- Stream B Mandelbrot perturbation produces small transient (~0.01–0.02 nats), recovery within 3 epochs
- Anastomosis gates near zero — streams effectively independent

### Early specialisation (ep 5–50)
- Both streams receive the same tokens but route to different experts
- Stream A maintains existing expert specialisations (SEM/LNG/ABS patterns observed at 17L)
- Stream B begins exploring alternative routing — initially random, then coherent
- Gate activation at anastomosis layers gradually increases where cross-stream information is useful

### Sustained learning (ep 50+)
- Two distinct routing regimes emerge — measurable as KL divergence between stream A and B expert selection distributions per layer
- Combined output outperforms either stream alone on held-out loss
- Descent rate expected: faster than corpus-free depth surgery (more capacity AND new routing paths), comparable to a corpus-unlock surgery

---

## Metrics to track post-surgery

| Metric | Implementation | Purpose |
|--------|---------------|---------|
| Stream routing divergence | KL(routing_A \|\| routing_B) per layer | Confirm streams are specialising |
| Anastomosis gate activation | mean(sigmoid(gate)) per fusion layer | Confirm information exchange is occurring |
| Per-stream loss | separate forward passes, no merge | Track relative stream quality |
| Stream cosine similarity | cos(h_a, h_b) at each layer | Detect premature convergence |

These metrics will be added to the dashboard training.log output and visualised in a new "Cord" panel alongside TTL.

---

## Biological correspondence

| Biological mechanism | Architecture equivalent |
|---------------------|------------------------|
| Individual hypha | Single 256H ternary stream |
| Mycelial cord | Dual-stream bundle |
| Anastomosis pore | F32 gate at fusion layer |
| Fibonacci hyphal junction spacing | Fibonacci fusion layer positions |
| Shared nutrient substrate | Shared expert FFN weights |
| Independent chemical sensing | Independent routing gates per stream |
| Fruiting body | Shared LM head (unified output) |

The correspondence is not metaphorical window dressing. The engineering decisions — sparse rather than dense fusion, shared resources with independent access strategies, net2net initialisation that preserves existing learning — each follow directly from asking how the biological system actually handles the same constraint.

---

## Novelty statement

No published work describes:
1. Dual-stream ternary weight networks with sparse anastomosis fusion
2. Net2net-compatible stream-level surgery (as distinct from depth or width surgery)
3. Fibonacci-gated cross-stream information exchange
4. Mandelbrot perturbation applied at stream level to break inter-stream symmetry

The closest related work is mixture-of-experts (shared experts, independent gates) and dual-encoder architectures (two streams merged at output). Neither uses ternary weights, net2net surgery, or biologically-motivated fusion scheduling. The combination is original.

This architecture extends patent A50296/2026 and will be documented as a continuation or distinct filing pending legal review.

---

## Implementation sequence

1. `perform_cord_surgery()` in `train_bible.rs` — tensor migration and gate init
2. Dual-stream forward pass in `moe_llm_core` — parallel block execution + anastomosis
3. Config loader — parse `num_streams`, `fusion_layers`
4. Training loop instrumentation — stream divergence metrics to log
5. Dashboard — Cord panel with per-stream routing heatmap and gate activation strip
6. `moe-test` inference binary — dual-stream forward pass for benchmarking

This document is the authoritative design specification. No implementation begins before this document is reviewed and the design decisions above are confirmed.

---

*Designed by RFI-IRFOS · 2026-05-14 · Graz, Austria*  
*Contact: contact@ternlang.com · Patent: A50296/2026*
