# TritFloat — Technical Specification and IP Disclosure

**Author:** RFI-IRFOS  
**Date:** 2026-05-21  
**Status:** Confidential — IP Protection Record  
**Patent reference:** see A50296/2026 (@sparseskip, parent application)

---

## 1. Abstract

TritFloat is a floating-point numeric format built on balanced ternary trits rather than binary bits. The key structural innovation is a **native confidence field** embedded directly in the number representation. Confidence propagates automatically through arithmetic operations — multiplication uses a weakest-link rule, addition uses an averaging rule — without any external Bayesian layer or separate uncertainty tensor. This makes uncertainty a first-class citizen of every scalar value in a computation graph.

The format is designed for inference in ternary neural networks (BitNet b1.58 family and successors), where it extends the existing @sparseskip weight-level optimization to the activation level: zero-phase activations are skipped entirely with no multiply-accumulate, and the confidence field gates Mixture-of-Experts routing without a separate gating network.

---

## 2. Background and Prior Art Differentiation

### 2.1 IEEE 754 floating-point

IEEE 754 stores sign (1 bit), exponent (8 or 11 bits), and mantissa (23 or 52 bits) in binary. There is no uncertainty field. NaN and ±Inf are special encodings, not confidence gradations. Computations either succeed or produce an exception — there is no in-band signal about how much to trust the result.

**TritFloat differs:** the confidence field is a structural part of every number. It cannot be stripped; it propagates automatically.

### 2.2 Interval arithmetic

Interval arithmetic ([a, b]) tracks upper and lower bounds through operations. It is conservative: intervals grow monotonically through chains of operations and can become uselessly wide. It requires two f64 values per scalar (2× memory). There is no concept of "how confident are we in the center estimate" — only "what is the guaranteed range."

**TritFloat differs:** the confidence field is a single 2-trit (9-state) scalar that shrinks or averages through operations. It is lossy but compact and semantically meaningful for routing decisions. It does not guarantee bounds — it tracks epistemic confidence.

### 2.3 Stochastic arithmetic (CESTAC / CADNA)

Stochastic arithmetic perturbs operations with random rounding and runs N copies of the computation to estimate numerical stability. It requires N × memory and N × compute.

**TritFloat differs:** confidence is a single deterministic field derived from the values themselves, not from statistical sampling. There is no overhead beyond the confidence trit field itself.

### 2.4 Probabilistic/Bayesian numbers

Probabilistic arithmetic assigns a probability distribution to each number (Gaussian, mixture, etc.). This is theoretically precise but requires parameterized distributions per value, typically 2–4 extra floats per scalar.

**TritFloat differs:** confidence uses exactly 2 trits (9 states) — the minimum granularity needed to make routing and gating decisions. It does not attempt to represent full distributions.

### 2.5 Quantized formats (INT8, NF4, FP8, MXFP)

Quantized formats reduce bit-width for memory efficiency but carry no uncertainty signal. They are strictly value representations with no epistemic metadata.

**TritFloat differs:** the primary innovation is not quantization but the in-band confidence field.

### 2.6 BitNet b1.58 and ternary weight quantization

BitNet b1.58 (arxiv 2402.17764) quantizes weights to {-1, 0, +1} and applies a scalar beta = mean(|W|) to restore activation scale. The @sparseskip optimization (RFI-IRFOS, patent A50296/2026) skips zero-weight positions at the matrix-multiply level for linear time savings proportional to weight sparsity.

**TritFloat extends this:** @sparseskip is applied at the **activation level** (not only weight level) via the TritFloat phase field. A zero-phase activation skips its MAC regardless of the weight. Additionally, the confidence field gates MoE routing decisions, removing the need for a learned gating network.

---

## 3. Format Specification

### 3.1 Physical layout

```
[ phase: 1t ][ exponent: 5t ][ mantissa: 6t ][ confidence: 2t ]
  digit 0       digits 1–5     digits 6–11     digits 12–13
```

Total: **14 trits**, stored as a single `u32` in base-3 encoding.  
Each digit ∈ {0, 1, 2}. Raw value = Σ digit_i × 3^i.  
Maximum raw value: 3^14 − 1 = 4,782,968 (fits in u32 with room to spare).

### 3.2 Trit encoding convention

Balanced ternary: digit d encodes trit value t = d − 1, so:
- digit 0 → trit −1
- digit 1 → trit  0
- digit 2 → trit +1

Phase and exponent use this balanced encoding. Mantissa uses unbalanced {0,1,2} digits (pure magnitude).

### 3.3 Field semantics

**Phase (1 trit, position 0)**  
Encodes the sign of the value: trit −1 = negative, 0 = zero, +1 = positive.  
When phase = 0, the value is exactly zero regardless of exponent/mantissa.

**Exponent (5 trits, positions 1–5)**  
Balanced 5-trit integer, range [−121, +121].  
A 5-trit balanced ternary integer spans [−(3^5−1)/2, +(3^5−1)/2] = [−121, +121].  
Represents a power of 3: scale = 3^exponent.

**Mantissa (6 trits, positions 6–11)**  
Unbalanced base-3 integer in [0, 728] (= 3^6 − 1).  
Represents fractional part: mantissa_f = M / 364.5, range [0, 1.997).  
Combined with normalization: (1 + mantissa_f) ∈ [1, 3).

**Confidence (2 trits, positions 12–13)**  
Balanced 2-trit integer, range [−4, +4] (9 states).  
Shifted to [0, 8] and normalized: confidence = (c_balanced + 4) / 8 ∈ {0.000, 0.125, 0.250, 0.375, 0.500, 0.625, 0.750, 0.875, 1.000}.  
Semantics: 0.0 = completely unknown, 0.5 = neutral/unset, 1.0 = maximally certain.

### 3.4 Value formula

For a non-zero TritFloat:

```
value = phase × 3^exponent × (1 + M / 364.5)
```

where:
- phase ∈ {−1, +1}
- exponent ∈ [−121, +121]
- M ∈ [0, 728]

This gives representable range approximately [3^−121, 3^122 × (1 + 728/364.5)] ≈ [10^−57, 10^58].

Precision per step: 1/364.5 × 3^exponent ≈ 0.274% of the local scale value.  
This corresponds to ~8.8 bits of mantissa precision (log2(364.5) ≈ 8.51 bits).

### 3.5 Coverage of f32 range

f32 exponent range [−126, +127] in base-2 corresponds to roughly [−79, +80] in base-3.  
TritFloat exponent range [−121, +121] covers this comfortably.

---

## 4. Confidence Propagation Rules

These rules are the core algorithmic innovation. They define how certainty flows through arithmetic, turning TritFloat into a propagating epistemic signal rather than a static annotation.

### 4.1 Multiplication — weakest-link rule

```
conf(a × b) = min(conf(a), conf(b))
```

**Rationale:** a product is only as reliable as its least-certain factor. This mirrors Bayesian chain of custody: if one operand is uncertain, the result is no more certain than that operand. It is a conservative bound on output confidence.

**Consequence:** uncertainty is sticky through multiply chains. A single low-confidence activation in a dot product dominates the confidence of the result.

### 4.2 Addition — evidence averaging rule

```
conf(a + b) = (conf(a) + conf(b)) / 2
```

**Rationale:** addition combines independent evidence streams. If one signal is highly confident and another is uncertain, the result averages their epistemic states — reflecting that we have two partial pieces of evidence rather than one strong one. This is not a Bayesian posterior update; it is a coarse approximation designed to be cheap and in-band.

**Consequence:** averaging a confident signal with an uncertain one yields a medium-confidence result. In a dot product, the final confidence reflects the average certainty of all non-skipped terms (modulated by the min from multiplication).

### 4.3 Negation and absolute value

```
conf(−a) = conf(a)
conf(|a|) = conf(a)
```

These operations change value but carry no new information, so confidence is preserved unchanged.

### 4.4 Zero

Zero has its own confidence. `TritFloat::from_f32_with_confidence(0.0, c)` stores c in the confidence field even though the phase is 0. This enables the distinction between "definitely zero" (c=1.0) and "unknown — could be zero" (c=0.0).

---

## 5. @sparseskip at the Activation Level

The @sparseskip optimization (originally at the weight level in the TIS sparse matmul kernel) is extended here to the activation level via the TritFloat phase field.

### 5.1 Mechanism

In `TritFloat::dot(a, b)`:

```
for each pair (a_i, b_i):
    if a_i.is_zero() OR b_i.is_zero():
        update min_conf from this pair
        skip += 1
        continue          // no multiply, no accumulate
    acc += a_i.to_f32() * b_i.to_f32()
    update min_conf
```

A zero-phase activation contributes exactly 0 to the dot product and can be detected by testing the phase trit (a single 3-way digit comparison). The MAC is skipped entirely.

### 5.2 When activation sparsity matters

In trained BitNet b1.58 models, weight sparsity after quantization is typically 55–65%. Activation sparsity depends on the non-linearity and the input distribution. In ternary activation regimes (where activations are also quantized to {−1,0,+1}), activation sparsity of 30–50% is observed. Combined weight+activation sparsity compounds:

```
effective_skip_rate = 1 − (1 − weight_sparsity) × (1 − activation_sparsity)
```

At 60% weight sparsity and 40% activation sparsity: skip rate ≈ 76%.

### 5.3 Relation to parent @sparseskip patent (A50296/2026)

The parent application covers weight-level zero-skip in the TritMatrix sparse matmul kernel. The present contribution extends the mechanism:

1. **Activation-level**: checks phase of activation tensor elements, not weight matrix elements.
2. **Native field**: the phase trit is a structural part of the number, not a separate sparsity mask.
3. **Confidence accounting**: skipped terms still contribute to the confidence running minimum (they are not silently dropped from the uncertainty accounting).

---

## 6. Confidence-Gated Mixture-of-Experts Routing

### 6.1 Standard MoE routing

Standard MoE networks use a learned gating network (typically a small linear layer + softmax) to route each token to one or more experts. This adds parameters, memory, and compute — and the gate itself requires inference.

### 6.2 TritFloat routing primitive

```rust
pub fn should_route(self, threshold: f32) -> bool {
    !self.is_zero() && self.confidence() >= threshold
}
```

This gate has zero learned parameters. It uses the existing confidence field — which was computed for free as a side-effect of the arithmetic — to decide whether an activation is worth routing to an expensive expert layer.

**Semantics:**
- `is_zero()` = phase trit is 0. Zero-phase activations are definitionally uninformative and always skip.
- `confidence() >= threshold` = the activation carries enough certainty to justify the routing cost. Uncertain activations (noisy inputs, accumulation of low-confidence multiplications) stay in the cheap path.

### 6.3 Integration point

In a TritFloat-native MoE forward pass:

```
hidden = tritfloat_linear(input, weights)   // produces TritFloat activations
for each expert:
    routed = [h for h in hidden if h.should_route(threshold=0.4)]
    if len(routed) > min_batch:
        expert_out = expert.forward(routed)
```

The routing decision is O(n) with no matrix multiply. The threshold is a single hyperparameter set at initialization.

---

## 7. Precision and Dynamic Range

| Parameter | Value |
|-----------|-------|
| Exponent range | [−121, +121] in base 3 |
| Dynamic range | ≈ 10^−57 to 10^58 |
| f32 coverage | Full (f32 spans ≈ 10^−38 to 10^38) |
| Mantissa steps | 729 (6-trit field, values 0–728) |
| Precision per step | ≈ 0.274% of local scale |
| Effective mantissa bits | ≈ 8.5 bits |
| Confidence states | 9 (2-trit balanced) |
| Storage | u32 (4 bytes) |
| Information capacity | 14 trits ≈ 22.2 bits |

For comparison: f32 has 24 bits of mantissa (7.2 decimal digits). TritFloat has ~8.5 bits of mantissa (~2.6 decimal digits). This is deliberate: TritFloat is not a general-purpose float but a compact inference type for activations in already-quantized networks. The 2-trit confidence field uses storage that would otherwise go to mantissa precision.

---

## 8. Storage Format and Serialization

The u32 base-3 encoding is compact and portable. No normalization steps are needed during deserialization — `TritFloat::from_raw(u32)` reconstructs the exact value.

The 14-trit encoding never exceeds raw value 4,782,968, which fits in u32. The upper bits (positions 14+) of the u32 are always zero and can be used for application-level flags without affecting the TritFloat value.

For bulk storage (tensors of activations), a packed representation can encode two TritFloats per u32 since 4,782,968 < 2^23, leaving one bit per pair for alignment or metadata.

---

## 9. Claims of Novelty

The following aspects of TritFloat are believed to be novel as of the disclosure date 2026-05-21:

**Claim 1 — Confidence as a first-class numeric field in ternary floating-point**  
A floating-point number representation in balanced ternary where the bit-analog (trit) encoding includes a dedicated confidence sub-field that is part of the number representation itself, not a separate annotation tensor or wrapper type.

**Claim 2 — Confidence propagation through arithmetic without external infrastructure**  
A set of propagation rules (mul=min, add=average) defined on the confidence sub-field such that any sequence of arithmetic operations on TritFloats produces a result whose confidence reflects the certainty of the computation without any external Bayesian inference step.

**Claim 3 — @sparseskip at activation level via the phase trit**  
A method of skipping multiply-accumulate operations in neural network inference where the skip condition is evaluated on the phase trit of activation values rather than on a separate sparsity mask or weight-level zero check, enabling activation-level sparsity exploitation in a single integer comparison.

**Claim 4 — Confidence-gated Mixture-of-Experts routing without a learned gate**  
A routing mechanism for MoE neural networks where the routing decision is made by comparing the embedded confidence field of an activation value against a fixed threshold, eliminating the learned gating network and its associated parameters, memory, and compute.

**Claim 5 — Combination: ternary floating-point with in-band confidence + activation sparseskip + confidence-gated routing as a unified inference primitive**  
The combination of Claims 1–4 as a single numeric type enabling sparsity exploitation, uncertainty propagation, and MoE routing in one unified scalar type.

---

## 10. Implementation

Reference implementation: `ternlang-root/ternlang-ml/src/tritfloat.rs`  
Crate: `ternlang-ml` v1.3.6  
License: LicenseRef-Ternlang-Commercial (© 2026 RFI-IRFOS, all rights reserved)  
Test coverage: 19 unit tests, all passing as of 2026-05-21

Key types and methods:

```rust
TritFloat(u32)                                // newtype wrapping base-3 encoded u32
TritFloat::from_f32(x: f32) -> Self
TritFloat::from_f32_with_confidence(x, c) -> Self
TritFloat::to_f32(self) -> f32
TritFloat::confidence(self) -> f32            // [0.0, 1.0]
TritFloat::phase(self) -> i8                  // {-1, 0, +1}
TritFloat::is_zero(self) -> bool
TritFloat::should_route(self, threshold) -> bool
TritFloat::dot(a: &[Self], b: &[Self]) -> Self
TritFloat::dot_with_skips(a, b) -> (Self, usize)
TritFloat::add(self, rhs) -> Self
TritFloat::mul(self, rhs) -> Self
TritFloat::neg(self) -> Self
TritFloat::abs(self) -> Self
```

---

## 11. Relation to Broader TIS Architecture

TritFloat is the lowest-level numeric primitive in the Ternary Intelligence Stack. It sits below TritMatrix (the weight-level ternary matrix type) and below the MoE routing layer in albert. The hierarchy is:

```
albert. (MoE-13 model)
  └── MoE routing layer       ← confidence-gated via should_route()
       └── TritMatrix sparse matmul ← @sparseskip weight-level (A50296/2026)
            └── TritFloat activations ← @sparseskip activation-level + confidence propagation
                 └── TritFloat(u32) ← this document
```

The SPRIND pitch (deadline 2026-05-30) should reference TritFloat as the substrate that makes confidence-propagating inference possible. The claim: albert. is the only LLM whose every scalar activation carries a live epistemic certainty estimate that gates both compute and routing decisions.

---

*End of specification. All content herein constitutes a confidential IP disclosure record for RFI-IRFOS. Unauthorized reproduction or disclosure is prohibited.*
