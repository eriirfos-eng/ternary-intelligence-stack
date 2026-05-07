# Reproducibility

**Project:** Ternary Intelligence Stack (TIS) — Albert-MoE-13  
**Version:** v1.3.6 | Architecture: 6L · 256H · 12E · 128CTX · 8000V  
**Last verified:** 2026-05-07

---

## What is and isn't deterministic

| Component | Deterministic? | Notes |
|-----------|---------------|-------|
| Tokenization (WordLevel) | **Yes** | Fixed `vocab.json`, byte-identical output |
| Corpus loading | **Yes** | `load_corpus()` reads files in sorted name order |
| Batch sampling order | **No** | Uses OS-seeded `rand::random::<usize>()` |
| Weight initialization | **No** | candle uses its own internal RNG |
| Training loss trajectory | **No** | Depends on init + batch order |
| Checkpoint format (`.safetensors`) | **Yes** | Header structure stable across versions |
| Parameter count for a given config | **Yes** | Config → shape map is fully deterministic |
| Gradient signal (non-zero backprop) | **Yes** | STE connected end-to-end, verified by repro_check |

Individual training runs are not bit-identical. The training *pipeline* is reproducible: the same config always produces the same architecture, gradient signal is always present, and checkpoints are portable and verifiable.

---

## Smoke test

The `repro_check` binary verifies three independently checkable properties in under 5 seconds on any CPU:

```bash
cd albert-moe-13
bash scripts/verify_reproducibility.sh
```

Or directly:

```bash
cargo run --release -p moe-llm-core --bin repro_check
```

### What it checks

**Check 1 — Architecture**: Parameter count (827,008 for 2L·64H·4E smoke config) and all required tensor keys match exactly.

**Check 2 — Gradient signal**: 5 forward + backward passes on seeded synthetic tokens. Loss must be within ±50% of `ln(vocab_size)` (uniform-distribution baseline). Gradient norms must be non-zero — confirms STE backprop is connected end-to-end.

**Check 3 — Checkpoint round-trip**: Saves a checkpoint, reloads it into a fresh model, verifies every tensor shape matches the original.

### Expected output

```
RESULT: PASS (17 checks, 0 failures)
Albert-MoE-13 training pipeline is functionally correct and reproducible.
```

---

## Production checkpoint verification

The `reproducibility_verifier` binary parses a `.safetensors` header (no full weight load) and checks every tensor shape against the declared config:

```bash
cargo run --release -p reproducibility_verifier -- \
    --checkpoint albert-moe-13/models/bible_ternary_v2.0.0.safetensors \
    --config     albert-moe-13/models/bible_ternary_v2.0.0.config.json
```

**Verified output — 2026-05-07, production checkpoint:**

```
File size:  166.2 MB | Tensors: 347 | Parameters: 43,567,616 (43.6M)

RESULT: PASS (24 checks, 0 failures)
Checkpoint valid for 6L · 256H · 12E architecture.
```

---

## Dependency pinning

```bash
cargo build --locked --release   # fails if any dep differs from Cargo.lock
```

All dependencies are pinned via `Cargo.lock` committed to the repository.

---

## Known non-determinism sources

1. **Weight initialization**: candle's `Init::Uniform` seeds from OS entropy. Two fresh runs produce different initial weights → different loss trajectories. Expected and normal.

2. **Net2Net surgery timing**: `EvolutionManager` triggers on training loss, which varies between runs. Surgery mechanism is reproducible; timing is not.

3. **Float ordering**: Not applicable — training is single-threaded on CPU. Rayon is used only for corpus loading (order-stable: sorted file names).

---

*SPRIND audit note: run `bash albert-moe-13/scripts/verify_reproducibility.sh` from the repo root for the full reproducibility test suite.*
