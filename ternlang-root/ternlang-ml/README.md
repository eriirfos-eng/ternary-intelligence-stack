# ternlang-ml

Ternary ML inference kernels for the [Ternlang](https://ternlang.com) ecosystem.

## What's in this crate

| Component | Description |
|-----------|-------------|
| `quantize()` | Convert f32 weights → balanced ternary using BitNet-style threshold |
| `sparse_matmul()` | Matmul that skips zero-state (`tend`) weights — speedup scales with sparsity |
| `dense_matmul()` | Standard ternary matmul for comparison |
| `TritMatrix` | Core ternary matrix type |
| `TritScalar` | Scalar ternary decision with confidence and zone classification |
| `TritEvidenceVec` | Multi-dimensional weighted evidence aggregation |
| `DeliberationEngine` | EMA-based convergence loop — iterates evidence until a stable trit is reached |
| `action_gate()` | Multi-dimensional hard-block gate for safety-critical decisions |
| `MLP` | 2-layer ternary multi-layer perceptron |
| `benchmark()` | Wall-clock timing across matrix sizes with sparsity reporting |

## Performance

Speedup from `sparse_matmul` over dense float32 scales proportionally with zero-weight fraction:

| Sparsity | Speedup (vs dense f32) |
|----------|------------------------|
| ~50–70% (typical BitNet) | 2–4× |
| ~90% | ~10× |
| ~99% (theoretical bound) | up to 122× |

Baseline measured result: **2.3×** at typical distributions ([commit 60f7ef6](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/commit/60f7ef659)).

## Usage

```rust
use ternlang_ml::{quantize, bitnet_threshold, sparse_matmul, TritMatrix};

let weights = vec![0.8, -0.3, 0.05, -0.9, 0.1, 0.7];
let tau     = bitnet_threshold(&weights);          // BitNet b1.58 threshold
let trits   = quantize(&weights, tau);

let a = TritMatrix::from_f32(2, 3, &weights[..6], tau);
let b = TritMatrix::from_f32(3, 2, &weights[..6], tau);
let c = sparse_matmul(&a, &b);
println!("sparsity: {:.1}%", ternlang_ml::sparsity(&trits) * 100.0);
```

## License

BSL-1.1 (converts to Apache-2.0 on 2030-04-03). See [LICENSE](https://github.com/eriirfos-eng/ternary-intelligence-stack--tis-/blob/main/ternlang-root/LICENSE).
