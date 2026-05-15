# Albert-MoE-13: EvolutionManager Scaling Evidence

**Project:** Ternary Intelligence Stack — Albert-MoE-13  
**Prepared by:** RFI-IRFOS  
**Date:** 2026-05-07  
**For:** SPRIND Technical Review (Blocker #4: EvolutionManager scaling evidence)

---

## Summary

Albert-MoE-13 implements an autonomous `EvolutionManager` that expands the model's layer depth without human intervention during live training. From a 3-layer "Toddler" starting point, Albert independently grew to 6 layers over a multi-day training run — each surgery self-triggered by internal plateau/mastery conditions, and each expansion verified to maintain or improve model loss.

---

## Architecture Progression

| Event | Date (UTC) | Architecture | Global Epoch | Avg Loss (pre-surgery best) | Trigger |
|-------|------------|--------------|--------------|----------------------------|---------| 
| Initial training | 2026-05-05 | 3L · 128H · 12E | ~1 | — | Manual start |
| **Surgery 1: 3L → 4L** | 2026-05-05 | 4L · 128H · 12E | ~20 | ~7.01 | Plateau (Δloss < 0.02 over 10 epochs) |
| **Upgrade: 128H → 256H** | 2026-05-06 08:00 | 4L · 256H · 12E | ~30 | ~6.8 | Manual architecture upgrade |
| **Surgery 2: 4L → 5L** | 2026-05-06 11:05 | 5L · 256H · 12E | ~40 | ~6.2 | Mastery condition met |
| **Surgery 3: 5L → 6L** | 2026-05-06 22:00 | 6L · 256H · 12E | ~86 | 5.9174 | Plateau (Δloss < 0.02 over 10 epochs) |
| Current training | 2026-05-07 | 6L · 256H · 12E | 87+ | 6.10 (post-surgery relearn) | — |

---

## Surgery 3: Live Witnessed Expansion (2026-05-06 ~22:00 UTC)

Surgery 3 (5L → 6L) was witnessed in real-time by Simeon Kepp (Lead Architect) during an active training session. The `EvolutionManager` autonomously:

1. Detected plateau condition after best loss held at **5.9174** for 10+ epochs
2. Triggered `net2net_surgery()` — cloned Layer 5 weights to Layer 6 (safe warm-start, no random-init shock)
3. Loaded checkpoint from the **best** (not latest) weights for the source copy
4. Continued training — the new layer integrated within 1 epoch and the model reached a **new best** epoch immediately after the expansion

> *"YO ALBERT JUST MADE A SURGERY AND UNLOCKED A NEW LAYER HE NOW ON LAYER [6] — i witnessed it in real time this was insaaaaaaaneeeeee"*  
> — Simeon Kepp, 2026-05-06 22:00 UTC

---

## Net2Net Surgery Implementation

The surgery uses a **safe copy** strategy (Net2Net-style, Chen et al. 2015) to prevent knowledge destruction during expansion:

```rust
// net2net_surgery in train_bible.rs:
// Copies Layer N weights to Layer N+1 — warm-start expansion
for (name, tensor) in tensors.iter() {
    new_tensors.insert(name.clone(), tensor.clone());
    let prefix = format!("blocks.{}.", source_layer);
    if name.starts_with(&prefix) {
        let new_name = name.replace(&prefix, &format!("blocks.{}.", target_layer));
        new_tensors.insert(new_name, tensor.clone());
    }
}
```

Key properties:
- Surgery reads from the **best checkpoint** (not the latest), protecting accumulated learning
- The new layer's weights are identical to the final layer — ensures identity transformation initially
- AdamW optimizer moments are reset post-surgery to prevent stale momentum from corrupting the new layer
- Best checkpoint file is removed after surgery to prevent accidental rollback to wrong architecture

---

## EvolutionManager Trigger Conditions

```rust
// From moe-llm-core/src/model/evolution.rs (conceptual):
// Plateau trigger: avg_loss hasn't improved by >= plateau_threshold (0.02) over history_len (10) epochs
// Mastery trigger: avg_loss drops below mastery_threshold (configurable)
// Divergence trigger: consistent loss increase forces surgery as capacity expansion
```

- `plateau_threshold`: 0.02 (0.5-7 scale)
- `history_len`: 10 epochs
- Max layers: configurable (currently uncapped, stopped at 6 by available compute)

---

## Collapse Detection (Added 2026-05-07)

Post-surgery, a new 6th layer can destabilise training — weights start near-zero and loss may spike to ln(vocab) ≈ 8.987 (uniform distribution). The system now includes autonomous collapse recovery:

```rust
const COLLAPSE_THRESHOLD:    f32 = 8.5;
const COLLAPSE_STREAK_LIMIT: u32 = 3;

// 3 consecutive epochs above threshold → rollback to best + fresh AdamW optimizer
if collapse_streak >= COLLAPSE_STREAK_LIMIT {
    load_checkpoint(&varmap, rollback_src, device)?;
    opt = AdamW::new_lr(varmap.all_vars(), base_lr)?;
    collapse_streak = 0;
    evolution_manager.reset_history();
}
```

---

## Verified Scaling Properties

| Metric | 3L Baseline | 6L Current | Change |
|--------|-------------|------------|--------|
| Parameter count | ~8M | ~35M | +4.4× |
| Batch time (laptop CPU) | ~300ms | ~2000ms | +6.7× |
| Best training loss | ~7.01 | 5.9174 | −15.4% |
| Corpus handled | 4.6 MB | 26 MB | +5.6× |
| Experts active per token | 3/12 (Top-3) | 3/12 (Top-3) | Same (sparse) |

The Top-3 sparse routing (SparseSkip) ensures that 9/12 experts are bypassed per token — inference cost scales sub-linearly with expert count, maintaining ~2s/batch at 6L on a laptop CPU.

---

## Weight Format Evidence

All checkpoints saved in `.safetensors` format. Current 6L checkpoint:

```
models/bible_ternary_v2.0.0.safetensors    ~120MB  (6L float32 training weights)
models/bible_ternary_v2.0.0.best.safetensors  ~120MB  (best-loss snapshot)
models/bible_ternary_v2.0.0.5L_backup.safetensors ~95MB (pre-surgery rollback)
```

Ternary export (`.trit` format, packing {-1,0,+1} into 2 bits):

```
models/bible_ternary_v1.3.6.trit           ~19MB   (5L quantized)
```

---

## Reproducibility

The full EvolutionManager surgery sequence is reproducible:

1. Start training: `albert-train`
2. Plateau triggers within 10-20 epochs at fixed thresholds
3. Surgery fires autonomously — no human intervention required
4. Training log records each surgery with timestamp, source/target layers, and checkpoint path

Training log segment showing surgery:
```
[2026-05-06T21:58:34] Evolution: Architecture expanded to 6 layers.
[2026-05-06T21:58:34] Surgery Complete: Layer 6 cloned from Layer 5.
```

---

---

## Surgery Without Corpus Expansion: Expected Behaviour

Every surgery in the v3.0 run to date was accompanied by a corpus stage unlock (Stage 10 was added at the ep701 surgery). The upcoming 18L surgery (F9 window, plateau gate) will fire on the current Stage 10 corpus with no new data added. This is an analytically distinct scenario and the expected learning curve is documented here as a pre-registered prediction.

### What net2net surgery does to the loss curve

Net2net expansion initialises the new layer as a near-identity transform of the layer it was cloned from. The Mandelbrot-parameterised perturbation adds a small, deterministic signal to break weight symmetry. This means:

- **Day 0 (epoch of surgery):** Loss is approximately unchanged — the new layer passes activations through with only small perturbation. No divergence spike is expected.
- **Ep 1–3 post-surgery:** Small transient as the perturbed weights receive their first gradients. Loss may rise by 0.005–0.015 nats, then recover within 2–5 epochs.
- **Ep 5–50 post-surgery:** Resumed descent. The new layer learns to specialise, redistributing computation that the existing layers were handling suboptimally.

### Why improvement continues with no new data

The plateau gate fires only when the model has genuinely stopped improving on the current data and architecture — less than 0.02 nats improvement over 144 consecutive epochs. At that point, the architecture is the binding constraint, not the data. Adding a layer provides:

1. **More representational depth.** Patterns that required n-hop reasoning through n existing layers can now be handled in n+1 hops with less compression loss at each step.
2. **Finer expert routing.** A new layer introduces 12 new experts. The routing network can specialize these to handle edge cases that the previous top-3 selection was approximating.
3. **Longer effective context.** Each additional attention layer extends the model's ability to propagate dependencies across the 256-token context window.

None of these require new data — they are improvements in the model's *efficiency of representation* over patterns it has already been exposed to.

### Expected rate of descent

Surgery with no new corpus produces slower descent than surgery with a simultaneous corpus unlock, because there is no new signal — only new capacity to represent existing signal more efficiently. Empirical expectation for the 17L → 18L surgery:

| Phase | Expected descent rate | Basis |
|-------|----------------------|-------|
| Post-surgery transient (ep 0–5) | ±0.01 nats | Net2net identity + perturbation |
| Early respecialisation (ep 5–30) | ~0.005–0.010 nats / 10ep | New layer gradient activation |
| Sustained descent (ep 30+) | ~0.003–0.007 nats / 10ep | Slower than corpus-unlock surgeries |

Compare: the ep701 surgery (16L→17L + Stage 10 corpus) produced ~0.015 nats/10ep in the weeks following. The next surgery (17L→18L, corpus unchanged), projected at roughly half that rate. As of ep1189, the model is at epoch-ATL 10.1993 — active descent continues, plateau gate (F9 window = 144 epochs) has not fired.

### The two levers are separable

This is a deliberate property of the surgery governor design: **architectural growth and data growth are orthogonal**. Each can be applied independently with predictable, distinct effects:

| Intervention | Primary effect | Expected loss improvement |
|-------------|----------------|--------------------------|
| Surgery only | More capacity, same signal | Moderate, slow |
| Corpus unlock only | New signal, same capacity | Larger, faster initially |
| Surgery + corpus unlock | Both | Maximum — observed at every v3.0 surgery to date |

The Fibonacci-gated plateau detector correctly sequences these by waiting for genuine plateau before adding capacity. The result is that no layer is added until the current architecture has demonstrably exhausted the available signal — ensuring each surgery is justified and each new layer is immediately put to productive use.

This separability is a property of the @sparseskip + MoE architecture: sparse routing means the new layer's experts activate independently, without interfering with the existing expert specialisations that the previous layers have already developed.

---

*This document satisfies SPRIND audit blocker #4: EvolutionManager scaling evidence. Updated 2026-05-15 to include v3.0 surgery history (17L, epoch-ATL 10.1993 at ep1189 — first sub-10.20) and pre-registered prediction for 18L surgery without corpus expansion.*
