// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
//
// TritFloat — a floating-point number built on balanced ternary trits.
//
// FORMAT (14 trits, stored as u32 in base-3):
//
//   [ phase: 1t ][ exponent: 5t ][ mantissa: 6t ][ confidence: 2t ]
//      {-,0,+}    bal. ternary    base-3 frac.     bal. ternary
//                 [-121, +121]    [0, 728]          [-4, +4] (9 states)
//
//   Total: 14 trits ≈ 22.2 bits of information capacity
//   Storage: u32, value = sum(digit_i * 3^i), digits in {0,1,2}
//
// VALUE SEMANTICS:
//   - Phase=0 (digit=1) → zero; exponent and mantissa are irrelevant
//   - value = phase × 3^exponent × (1 + mantissa/364.5)   [covers [1,3) normalized range]
//   - Exponent range ±121 covers f32 range comfortably (max f32 ≈ 3^80)
//
// CONFIDENCE FIELD:
//   The 2 confidence trits encode certainty about the value on a 9-state scale.
//   This is the key innovation: confidence is a first-class field in the number,
//   not a separate tensor. It propagates through arithmetic automatically.
//
//   c_digit = (c1_trit + 1) * 3 + (c0_trit + 1), range [0, 8]
//   Normalized to [0.0, 1.0] as c_digit / 8.0
//
//   0/8 = completely unknown    (both trits -1)
//   4/8 = neutral / unset       (both trits 0)
//   8/8 = maximally certain     (both trits +1)
//
// CONFIDENCE PROPAGATION RULES:
//   mul(a, b): c = min(conf_a, conf_b)     — chain weakest link
//   add(a, b): c = (conf_a + conf_b) / 2  — average the evidence

use serde::{Deserialize, Serialize};

// ─── Constants ────────────────────────────────────────────────────────────────

const TRIT_BASE: u32 = 3;

// Position offsets in the base-3 u32 encoding
const PHASE_POS: u32    = 0;   // trit 0
const EXP_POS: u32      = 1;   // trits 1-5
const MANT_POS: u32     = 6;   // trits 6-11
const CONF_POS: u32     = 12;  // trits 12-13

// Field widths (number of trits)
const EXP_TRITS: u32    = 5;
const MANT_TRITS: u32   = 6;
const CONF_TRITS: u32   = 2;

// Field maxima
const EXP_MAX: i32      = 121;   // (3^5 - 1) / 2
const MANT_MAX: u32     = 728;   // 3^6 - 1
const CONF_MAX: i32     = 4;     // (3^2 - 1) / 2

// Mantissa divisor: MANT_MAX/2 = 364.5 so (1 + M/MANT_DIV) ∈ [1, 3)
const MANT_DIV: f32     = 364.5;

// Total number of digits in the encoding
const TOTAL_TRITS: u32  = 14;

// The maximum u32 value representable: 3^14 - 1 = 4782968
const MAX_RAW: u32      = 4_782_968;

// ─── Core type ────────────────────────────────────────────────────────────────

/// A floating-point number encoded in balanced ternary with a native confidence field.
///
/// The confidence field propagates automatically through arithmetic, giving any
/// computation a live uncertainty estimate without a separate Bayesian layer.
///
/// Use `TritFloat::from_f32` to construct, `.to_f32()` to read the value,
/// and `.confidence()` to read the certainty in [0, 1].
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TritFloat(u32);

// ─── Internal helpers ─────────────────────────────────────────────────────────

/// Extract one base-3 digit at position `pos` from a u32 base-3 encoding.
#[inline]
fn get_digit(raw: u32, pos: u32) -> u32 {
    let divisor = TRIT_BASE.pow(pos);
    (raw / divisor) % TRIT_BASE
}

/// Set base-3 digit at position `pos` in a u32 base-3 encoding.
#[inline]
fn set_digit(raw: u32, pos: u32, digit: u32) -> u32 {
    debug_assert!(digit < 3, "digit must be in {{0,1,2}}");
    let place = TRIT_BASE.pow(pos);
    let cleared = raw - (raw / place % TRIT_BASE) * place;
    cleared + digit * place
}

/// Encode a balanced trit {-1, 0, +1} as a digit {0, 1, 2}.
#[inline]
fn balanced_to_digit(t: i8) -> u32 {
    (t + 1) as u32
}

/// Decode a digit {0, 1, 2} to a balanced trit {-1, 0, +1}.
#[inline]
fn digit_to_balanced(d: u32) -> i8 {
    d as i8 - 1
}

/// Decode a multi-trit balanced ternary integer from a packed u32 region.
/// `start_pos`: position of the least-significant trit in the packed u32.
/// `n_trits`: number of trits to read.
/// Returns the balanced integer value.
fn decode_balanced_int(raw: u32, start_pos: u32, n_trits: u32) -> i32 {
    let mut value = 0i32;
    let mut place = 1i32;
    for i in 0..n_trits {
        let digit = get_digit(raw, start_pos + i);
        let trit = digit_to_balanced(digit) as i32;
        value += trit * place;
        place *= 3;
    }
    value
}

/// Encode a balanced integer into n_trits starting at start_pos in a u32.
fn encode_balanced_int(mut raw: u32, start_pos: u32, n_trits: u32, mut value: i32) -> u32 {
    value = value.clamp(-((TRIT_BASE.pow(n_trits) as i32 - 1) / 2),
                         (TRIT_BASE.pow(n_trits) as i32 - 1) / 2);
    // Convert to balanced ternary digits (least significant first)
    let mut remaining = value;
    for i in 0..n_trits {
        // Find the trit that minimises |remaining|: try 0, then +1 or -1
        let low = (remaining % 3 + 3) % 3; // non-negative remainder mod 3
        let trit = if low <= 1 { low as i8 } else { (low as i8) - 3 }; // balanced: pick closest
        let digit = balanced_to_digit(trit);
        raw = set_digit(raw, start_pos + i, digit);
        remaining -= trit as i32;
        remaining /= 3;
    }
    raw
}

/// log base 3 of x, as integer floor. Returns 0 for x <= 0.
fn log3_floor(x: f32) -> i32 {
    if x <= 0.0 { return 0; }
    (x.ln() / 3f32.ln()).floor() as i32
}

// ─── TritFloat implementation ─────────────────────────────────────────────────

impl TritFloat {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// The canonical zero, with neutral confidence.
    pub fn zero() -> Self {
        // phase digit = 1 (balanced 0), all others = 1 (balanced 0), conf = neutral (digit 1,1)
        // This gives raw = 0 for all-zero balanced = digit 1 everywhere...
        // Actually: digit 1 = balanced 0 everywhere = all trits zero
        // raw = 1*3^0 + 1*3^1 + ... + 1*3^13 = (3^14 - 1)/2 = 2391484
        // But simpler: just build it procedurally.
        let mut raw = 0u32;
        for i in 0..TOTAL_TRITS {
            raw = set_digit(raw, i, 1); // digit 1 = balanced trit 0
        }
        // Set confidence to neutral (both trits 0 = digit 1 each)
        Self(raw)
    }

    /// Convert an `f32` to TritFloat with maximum confidence (certainty=1.0).
    pub fn from_f32(x: f32) -> Self {
        Self::from_f32_with_confidence(x, 1.0)
    }

    /// Convert an `f32` to TritFloat with a specified confidence in [0, 1].
    pub fn from_f32_with_confidence(x: f32, confidence: f32) -> Self {
        let mut raw = Self::zero().0;

        // ── Phase ─────────────────────────────────────────────────────────────
        if x == 0.0 || x.is_nan() {
            // phase = 0 (digit 1) — zero case; exponent/mantissa don't matter
            // confidence still applies to zero (we know it's zero)
            raw = set_digit(raw, PHASE_POS, 1);
            raw = Self::encode_confidence_into(raw, confidence);
            return Self(raw);
        }

        let phase: i8 = if x > 0.0 { 1 } else { -1 };
        raw = set_digit(raw, PHASE_POS, balanced_to_digit(phase));

        let x_abs = x.abs();

        // ── Exponent ──────────────────────────────────────────────────────────
        // E = floor(log3(x_abs)), clamped to [-121, +121]
        // After this, x_abs / 3^E ∈ [1, 3)
        let exp = log3_floor(x_abs).clamp(-EXP_MAX, EXP_MAX);
        raw = encode_balanced_int(raw, EXP_POS, EXP_TRITS, exp);

        // ── Mantissa ──────────────────────────────────────────────────────────
        // mantissa_f = x_abs / 3^exp - 1, in [0, 2)
        // M = round(mantissa_f * 729), clamped to [0, 728]
        let scale = (3f32).powi(exp);
        let mantissa_f = (x_abs / scale - 1.0).clamp(0.0, 1.9999);
        let m = (mantissa_f * MANT_DIV).round().clamp(0.0, MANT_MAX as f32) as u32;

        // Encode as 6 base-3 digits {0,1,2} (unbalanced mantissa — pure magnitude)
        let mut m_remaining = m;
        for i in 0..MANT_TRITS {
            let digit = m_remaining % 3;
            raw = set_digit(raw, MANT_POS + i, digit);
            m_remaining /= 3;
        }

        // ── Confidence ────────────────────────────────────────────────────────
        raw = Self::encode_confidence_into(raw, confidence);

        Self(raw)
    }

    /// Encode a [0,1] confidence float into the confidence trit field of a raw value.
    fn encode_confidence_into(raw: u32, confidence: f32) -> u32 {
        // Map [0,1] → [0, 8] → two balanced trits
        let c_int = (confidence.clamp(0.0, 1.0) * (CONF_MAX * 2) as f32).round() as i32;
        // c_int in [0, 8]: decode as c1*3 + c0 = c_int, balanced trits c0, c1 ∈ {-1,0,+1}
        let c_int_shifted = c_int - CONF_MAX; // shift to [-4, +4]
        encode_balanced_int(raw, CONF_POS, CONF_TRITS, c_int_shifted)
    }

    // ── Value extraction ──────────────────────────────────────────────────────

    /// Convert to f32. Confidence is discarded; use `.confidence()` separately.
    pub fn to_f32(self) -> f32 {
        let phase = digit_to_balanced(get_digit(self.0, PHASE_POS));
        if phase == 0 {
            return 0.0;
        }

        let exp = decode_balanced_int(self.0, EXP_POS, EXP_TRITS);

        // Decode mantissa (unbalanced base-3 digits {0,1,2})
        let mut m = 0u32;
        let mut place = 1u32;
        for i in 0..MANT_TRITS {
            m += get_digit(self.0, MANT_POS + i) * place;
            place *= 3;
        }
        let mantissa_f = m as f32 / MANT_DIV;

        let scale = (3f32).powi(exp);
        (phase as f32) * scale * (1.0 + mantissa_f)
    }

    /// The phase trit: -1 (negative), 0 (zero), or +1 (positive).
    pub fn phase(self) -> i8 {
        digit_to_balanced(get_digit(self.0, PHASE_POS))
    }

    /// The exponent as a signed integer in [-121, +121].
    pub fn exponent(self) -> i32 {
        decode_balanced_int(self.0, EXP_POS, EXP_TRITS)
    }

    /// The mantissa as a u32 in [0, 728]. Represents fractional part as M/729.
    pub fn mantissa(self) -> u32 {
        let mut m = 0u32;
        let mut place = 1u32;
        for i in 0..MANT_TRITS {
            m += get_digit(self.0, MANT_POS + i) * place;
            place *= 3;
        }
        m
    }

    /// Confidence as a float in [0.0, 1.0].
    ///
    /// 0.0 = completely unknown, 0.5 = neutral/unset, 1.0 = maximally certain.
    pub fn confidence(self) -> f32 {
        let c_balanced = decode_balanced_int(self.0, CONF_POS, CONF_TRITS);
        // c_balanced in [-4, +4] → shift to [0, 8] → divide by 8
        (c_balanced + CONF_MAX) as f32 / (CONF_MAX * 2) as f32
    }

    /// True if this value is zero (phase trit = 0).
    pub fn is_zero(self) -> bool {
        digit_to_balanced(get_digit(self.0, PHASE_POS)) == 0
    }

    /// True if confidence is below 0.5 (both confidence trits ≤ 0).
    pub fn is_uncertain(self) -> bool {
        self.confidence() < 0.5
    }

    /// The raw u32 backing value (for serialization and hardware interop).
    pub fn raw(self) -> u32 {
        self.0
    }

    /// Reconstruct from a raw u32 (as returned by `.raw()`).
    pub fn from_raw(raw: u32) -> Self {
        debug_assert!(raw <= MAX_RAW, "raw value exceeds 14-trit maximum");
        Self(raw.min(MAX_RAW))
    }

    // ── Confidence propagation ─────────────────────────────────────────────────

    /// Propagation rule for multiplication: weakest link.
    /// The result is only as confident as the less certain operand.
    pub fn mul_confidence(a: Self, b: Self) -> f32 {
        a.confidence().min(b.confidence())
    }

    /// Propagation rule for addition: average the evidence.
    pub fn add_confidence(a: Self, b: Self) -> f32 {
        (a.confidence() + b.confidence()) * 0.5
    }

    // ── Arithmetic ─────────────────────────────────────────────────────────────
    //
    // Software path: converts to f32, operates, converts back with propagated
    // confidence. Hardware-native trit arithmetic is a future optimization.

    /// Negate: flip phase, preserve all other fields including confidence.
    pub fn neg(self) -> Self {
        let new_phase = -self.phase();
        let new_digit = balanced_to_digit(new_phase);
        let raw = set_digit(self.0, PHASE_POS, new_digit);
        Self(raw)
    }

    /// Absolute value: force phase to +1 (or 0 if zero).
    pub fn abs(self) -> Self {
        if self.is_zero() { return self; }
        let raw = set_digit(self.0, PHASE_POS, balanced_to_digit(1));
        Self(raw)
    }

    /// Addition with confidence propagation (average rule).
    pub fn add(self, rhs: Self) -> Self {
        let value = self.to_f32() + rhs.to_f32();
        let conf = Self::add_confidence(self, rhs);
        Self::from_f32_with_confidence(value, conf)
    }

    /// Subtraction with confidence propagation (average rule).
    pub fn sub(self, rhs: Self) -> Self {
        self.add(rhs.neg())
    }

    /// Multiplication with confidence propagation (weakest-link rule).
    pub fn mul(self, rhs: Self) -> Self {
        // Short-circuit: if either operand is phase-zero, result is zero.
        // Confidence of zero = min(conf_a, conf_b) — we know it's zero, but
        // only as confidently as our least-certain input.
        if self.is_zero() || rhs.is_zero() {
            let conf = Self::mul_confidence(self, rhs);
            return Self::from_f32_with_confidence(0.0, conf);
        }
        let value = self.to_f32() * rhs.to_f32();
        let conf = Self::mul_confidence(self, rhs);
        Self::from_f32_with_confidence(value, conf)
    }

    /// Dot product of two slices of TritFloats.
    ///
    /// Confidence of the result = min confidence across all terms.
    /// Zero-phase terms are skipped entirely (@sparseskip at activation level).
    pub fn dot(a: &[Self], b: &[Self]) -> Self {
        assert_eq!(a.len(), b.len(), "dot product requires equal-length slices");

        let mut acc_value = 0.0f32;
        let mut min_conf = 1.0f32;
        let mut skipped = 0usize;

        for (&ai, &bi) in a.iter().zip(b.iter()) {
            // @sparseskip: neutral phase on either operand → contributes zero, skip MAC
            if ai.is_zero() || bi.is_zero() {
                // Still track the minimum confidence across skipped terms
                let term_conf = Self::mul_confidence(ai, bi);
                min_conf = min_conf.min(term_conf);
                skipped += 1;
                continue;
            }
            acc_value += ai.to_f32() * bi.to_f32();
            min_conf = min_conf.min(Self::mul_confidence(ai, bi));
        }

        let _ = skipped; // available for instrumentation if needed

        Self::from_f32_with_confidence(acc_value, min_conf)
    }

    /// Dot product returning (result, skip_count) for sparsity instrumentation.
    pub fn dot_with_skips(a: &[Self], b: &[Self]) -> (Self, usize) {
        assert_eq!(a.len(), b.len(), "dot product requires equal-length slices");

        let mut acc_value = 0.0f32;
        let mut min_conf = 1.0f32;
        let mut skipped = 0usize;

        for (&ai, &bi) in a.iter().zip(b.iter()) {
            if ai.is_zero() || bi.is_zero() {
                let term_conf = Self::mul_confidence(ai, bi);
                min_conf = min_conf.min(term_conf);
                skipped += 1;
                continue;
            }
            acc_value += ai.to_f32() * bi.to_f32();
            min_conf = min_conf.min(Self::mul_confidence(ai, bi));
        }

        (Self::from_f32_with_confidence(acc_value, min_conf), skipped)
    }

    // ── Routing hint ──────────────────────────────────────────────────────────

    /// Returns true if this activation should be routed to an expert.
    ///
    /// Uncertain activations (confidence < threshold) can skip expensive expert
    /// layers entirely — the confidence field directly gates MoE routing.
    ///
    /// `threshold` = minimum confidence to route (suggested: 0.3–0.5)
    pub fn should_route(self, threshold: f32) -> bool {
        !self.is_zero() && self.confidence() >= threshold
    }

    // ── Extended arithmetic ───────────────────────────────────────────────────

    /// Division with weakest-link confidence. Division by zero returns zero
    /// with 0 confidence — the caller can detect this via `is_uncertain`.
    pub fn div(self, rhs: Self) -> Self {
        if rhs.is_zero() {
            return Self::from_f32_with_confidence(0.0, 0.0);
        }
        let conf = Self::mul_confidence(self, rhs);
        Self::from_f32_with_confidence(self.to_f32() / rhs.to_f32(), conf)
    }

    /// Reciprocal: 1/x. Confidence preserved; zero input returns 0-confidence zero.
    pub fn recip(self) -> Self {
        if self.is_zero() {
            return Self::from_f32_with_confidence(0.0, 0.0);
        }
        Self::from_f32_with_confidence(1.0 / self.to_f32(), self.confidence())
    }

    /// Integer power. Confidence preserved — single-operand chain.
    pub fn powi(self, n: i32) -> Self {
        Self::from_f32_with_confidence(self.to_f32().powi(n), self.confidence())
    }

    /// Square root. Negative input returns 0-confidence zero (not a real number).
    pub fn sqrt(self) -> Self {
        if self.is_zero() { return self; }
        if self.phase() < 0 {
            return Self::from_f32_with_confidence(0.0, 0.0);
        }
        Self::from_f32_with_confidence(self.to_f32().sqrt(), self.confidence())
    }

    /// Clamp the value to [lo, hi]. Confidence is preserved unchanged.
    pub fn clamp(self, lo: f32, hi: f32) -> Self {
        Self::from_f32_with_confidence(self.to_f32().clamp(lo, hi), self.confidence())
    }

    /// Ternary comparison: returns +1 if self > rhs, −1 if self < rhs, 0 if equal.
    /// Confidence = min(conf_self, conf_rhs) — comparison is only as reliable as inputs.
    pub fn cmp_trit(self, rhs: Self) -> Self {
        let (va, vb) = (self.to_f32(), rhs.to_f32());
        let r = if va > vb { 1.0f32 } else if va < vb { -1.0 } else { 0.0 };
        Self::from_f32_with_confidence(r, Self::mul_confidence(self, rhs))
    }

    // ── Slice operations ──────────────────────────────────────────────────────

    /// Numerically stable softmax over a slice of TritFloats.
    ///
    /// Values are computed in f32; each output element carries the minimum
    /// confidence of all inputs (softmax mixes every element, so the whole
    /// slice's certainty bounds the result).
    pub fn softmax(slice: &[Self]) -> Vec<Self> {
        if slice.is_empty() { return vec![]; }
        let vals: Vec<f32> = slice.iter().map(|x| x.to_f32()).collect();
        let max_v = vals.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exps: Vec<f32> = vals.iter().map(|&v| (v - max_v).exp()).collect();
        let sum: f32 = exps.iter().sum::<f32>().max(f32::EPSILON);
        let min_c = slice.iter().map(|x| x.confidence()).fold(1.0f32, f32::min);
        exps.iter()
            .map(|&e| Self::from_f32_with_confidence(e / sum, min_c))
            .collect()
    }

    // ── Phase packing — SIMD-lite ─────────────────────────────────────────────

    /// Extract phase digits (0=neg, 1=zero, 2=pos) for a slice into a `Vec<u8>`.
    ///
    /// The pre-scan buffer: a single contiguous pass over raw u32 values (% 3)
    /// before the arithmetic loop. Separating phase-check from f32 math eliminates
    /// branch misprediction in the hot loop at high sparsity (≥50% zeros).
    #[inline]
    pub fn phase_digits(slice: &[Self]) -> Vec<u8> {
        slice.iter().map(|x| (x.0 % 3) as u8).collect()
    }

    /// Pack zero-phase flags for up to 64 TritFloats into a u64 bitmask.
    ///
    /// Bit i = 1 if slice[i].is_zero(), else 0. `mask.count_ones()` instantly
    /// gives the skip count for a 64-element chunk. `mask == 0` means all
    /// elements are active — no branch needed in the arithmetic loop.
    /// This is the preparation layer for AVX2 vectorization of the dot product.
    pub fn pack_phases_u64(slice: &[Self]) -> u64 {
        debug_assert!(slice.len() <= 64, "pack_phases_u64: slice too long (max 64)");
        let mut mask = 0u64;
        for (i, x) in slice.iter().take(64).enumerate() {
            if x.0 % 3 == 1 {
                mask |= 1u64 << i;
            }
        }
        mask
    }

    /// Dot product with two-pass pre-scan for reduced branch misprediction.
    ///
    /// Pass 1: extract all phase flags into u8 arrays (cache-hot, no branching).
    /// Pass 2: arithmetic only for active (non-zero-phase) pairs.
    ///
    /// Outperforms `dot_with_skips` at ≥50% sparsity where misprediction of the
    /// inline zero-check dominates. At low sparsity the extra allocation cost
    /// makes it slightly slower — profile before choosing.
    pub fn dot_prescan(a: &[Self], b: &[Self]) -> (Self, usize) {
        assert_eq!(a.len(), b.len(), "dot_prescan requires equal-length slices");
        let pa = Self::phase_digits(a);
        let pb = Self::phase_digits(b);

        let mut acc = 0.0f32;
        let mut min_conf = 1.0f32;
        let mut skipped = 0usize;

        for i in 0..a.len() {
            let c = Self::mul_confidence(a[i], b[i]);
            if c < min_conf { min_conf = c; }
            if pa[i] == 1 || pb[i] == 1 {
                skipped += 1;
            } else {
                acc += a[i].to_f32() * b[i].to_f32();
            }
        }

        (Self::from_f32_with_confidence(acc, min_conf), skipped)
    }
}

// ─── Display ─────────────────────────────────────────────────────────────────

impl std::fmt::Debug for TritFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TritFloat({:.6} conf={:.2} exp={} mant={})",
            self.to_f32(),
            self.confidence(),
            self.exponent(),
            self.mantissa(),
        )
    }
}

impl std::fmt::Display for TritFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.6}±{:.0}%", self.to_f32(), self.confidence() * 100.0)
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const TOL: f32 = 0.01; // ~1% relative tolerance for roundtrip

    fn approx(a: f32, b: f32, tol: f32) -> bool {
        if b == 0.0 { return a.abs() < tol; }
        ((a - b) / b).abs() < tol
    }

    #[test]
    fn zero_roundtrip() {
        let z = TritFloat::from_f32(0.0);
        assert!(z.is_zero());
        assert_eq!(z.to_f32(), 0.0);
        assert_eq!(z.phase(), 0);
    }

    #[test]
    fn positive_roundtrip() {
        for &x in &[0.001f32, 0.1, 0.5, 1.0, 3.0, 9.0, 100.0, 12345.678, 1e10, 1e-10] {
            let tf = TritFloat::from_f32(x);
            let back = tf.to_f32();
            assert!(approx(back, x, TOL),
                "roundtrip failed for x={}: got {} ({})", x, back, tf);
            assert_eq!(tf.phase(), 1);
        }
    }

    #[test]
    fn negative_roundtrip() {
        for &x in &[-0.5f32, -1.0, -3.14, -999.9] {
            let tf = TritFloat::from_f32(x);
            let back = tf.to_f32();
            assert!(approx(back.abs(), x.abs(), TOL),
                "negative roundtrip failed for x={}: got {}", x, back);
            assert_eq!(tf.phase(), -1);
        }
    }

    #[test]
    fn confidence_from_f32_is_max() {
        let tf = TritFloat::from_f32(1.0);
        assert!((tf.confidence() - 1.0).abs() < 0.15,
            "from_f32 should give near-max confidence, got {}", tf.confidence());
    }

    #[test]
    fn confidence_custom() {
        let tf = TritFloat::from_f32_with_confidence(1.0, 0.0);
        assert!(tf.confidence() < 0.2, "expected low confidence, got {}", tf.confidence());

        let tf = TritFloat::from_f32_with_confidence(1.0, 0.5);
        assert!((tf.confidence() - 0.5).abs() < 0.2, "expected mid confidence, got {}", tf.confidence());
    }

    #[test]
    fn zero_confidence_neutral() {
        let z = TritFloat::zero();
        assert!(z.is_zero());
        assert!((z.confidence() - 0.5).abs() < 0.2, "zero should have neutral confidence");
    }

    #[test]
    fn neg_flips_phase() {
        let pos = TritFloat::from_f32(2.5);
        let neg = pos.neg();
        assert_eq!(pos.phase(), 1);
        assert_eq!(neg.phase(), -1);
        assert!(approx(pos.to_f32(), -neg.to_f32(), TOL));
        // confidence is preserved
        assert!((pos.confidence() - neg.confidence()).abs() < 0.15);
    }

    #[test]
    fn abs_always_positive() {
        let neg = TritFloat::from_f32(-7.0);
        let a = neg.abs();
        assert_eq!(a.phase(), 1);
        assert!(a.to_f32() > 0.0);
    }

    #[test]
    fn mul_confidence_weakest_link() {
        let certain = TritFloat::from_f32_with_confidence(2.0, 1.0);
        let uncertain = TritFloat::from_f32_with_confidence(3.0, 0.0);
        let product = certain.mul(uncertain);
        assert!(product.confidence() < 0.2,
            "mul confidence should be dominated by uncertain operand");
    }

    #[test]
    fn mul_zero_propagates_uncertainty() {
        let zero = TritFloat::from_f32_with_confidence(0.0, 0.0);
        let certain = TritFloat::from_f32_with_confidence(5.0, 1.0);
        let product = certain.mul(zero);
        assert!(product.is_zero());
        // confidence = min(1.0, 0.0) = 0.0
        assert!(product.confidence() < 0.2);
    }

    #[test]
    fn add_confidence_averages() {
        let a = TritFloat::from_f32_with_confidence(1.0, 1.0);
        let b = TritFloat::from_f32_with_confidence(1.0, 0.0);
        let sum = a.add(b);
        assert!((sum.confidence() - 0.5).abs() < 0.2,
            "add confidence should average, got {}", sum.confidence());
    }

    #[test]
    fn add_value_correct() {
        let a = TritFloat::from_f32(1.5);
        let b = TritFloat::from_f32(2.5);
        let sum = a.add(b);
        assert!(approx(sum.to_f32(), 4.0, TOL), "1.5 + 2.5 should ≈ 4.0, got {}", sum.to_f32());
    }

    #[test]
    fn mul_value_correct() {
        let a = TritFloat::from_f32(3.0);
        let b = TritFloat::from_f32(4.0);
        let p = a.mul(b);
        assert!(approx(p.to_f32(), 12.0, 0.02), "3 × 4 should ≈ 12, got {}", p.to_f32());
    }

    #[test]
    fn dot_basic() {
        let a: Vec<TritFloat> = [1.0f32, 2.0, 3.0].iter().map(|&x| TritFloat::from_f32(x)).collect();
        let b: Vec<TritFloat> = [4.0f32, 5.0, 6.0].iter().map(|&x| TritFloat::from_f32(x)).collect();
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        let result = TritFloat::dot(&a, &b);
        assert!(approx(result.to_f32(), 32.0, 0.02),
            "dot([1,2,3],[4,5,6]) should ≈ 32, got {}", result.to_f32());
    }

    #[test]
    fn dot_skips_zeros() {
        // Two zeros in a = 2/3 skipped
        let a: Vec<TritFloat> = vec![
            TritFloat::from_f32(0.0),
            TritFloat::from_f32(2.0),
            TritFloat::from_f32(0.0),
        ];
        let b: Vec<TritFloat> = vec![
            TritFloat::from_f32(1.0),
            TritFloat::from_f32(3.0),
            TritFloat::from_f32(1.0),
        ];
        let (result, skips) = TritFloat::dot_with_skips(&a, &b);
        assert_eq!(skips, 2, "two zero phases should produce 2 skips");
        assert!(approx(result.to_f32(), 6.0, 0.02),
            "0*1 + 2*3 + 0*1 = 6, got {}", result.to_f32());
    }

    #[test]
    fn should_route_confidence_gate() {
        let certain = TritFloat::from_f32_with_confidence(1.0, 0.9);
        let uncertain = TritFloat::from_f32_with_confidence(1.0, 0.1);
        let zero = TritFloat::from_f32(0.0);

        assert!(certain.should_route(0.5),   "certain should route");
        assert!(!uncertain.should_route(0.5), "uncertain should not route");
        assert!(!zero.should_route(0.0),      "zero phase never routes");
    }

    #[test]
    fn raw_roundtrip() {
        let tf = TritFloat::from_f32(42.0);
        let raw = tf.raw();
        let restored = TritFloat::from_raw(raw);
        assert_eq!(tf, restored);
    }

    #[test]
    fn display_shows_confidence() {
        let tf = TritFloat::from_f32(3.14);
        let s = format!("{tf}");
        assert!(s.contains('%'), "display should show confidence %: got '{}'", s);
    }

    #[test]
    fn exponent_range_covered() {
        let large = TritFloat::from_f32(1e30f32);
        let small = TritFloat::from_f32(1e-30f32);
        assert!(large.exponent().abs() <= EXP_MAX as i32);
        assert!(small.exponent().abs() <= EXP_MAX as i32);
        assert!(approx(large.to_f32(), 1e30, 0.05));
        assert!(approx(small.to_f32(), 1e-30, 0.05));
    }

    // ── Extended arithmetic tests ─────────────────────────────────────────────

    #[test]
    fn div_basic() {
        let a = TritFloat::from_f32(6.0);
        let b = TritFloat::from_f32(2.0);
        let r = a.div(b);
        assert!(approx(r.to_f32(), 3.0, TOL), "6/2 should be 3, got {}", r.to_f32());
    }

    #[test]
    fn div_by_zero_returns_zero_confidence() {
        let a = TritFloat::from_f32(5.0);
        let z = TritFloat::from_f32(0.0);
        let r = a.div(z);
        assert!(r.is_zero());
        assert!(r.confidence() < 0.15, "div-by-zero should have 0 confidence");
    }

    #[test]
    fn recip_basic() {
        let r = TritFloat::from_f32(4.0).recip();
        assert!(approx(r.to_f32(), 0.25, TOL), "recip(4) should be 0.25, got {}", r.to_f32());
    }

    #[test]
    fn recip_zero_returns_zero_confidence() {
        let r = TritFloat::zero().recip();
        assert!(r.is_zero());
        assert!(r.confidence() < 0.15);
    }

    #[test]
    fn powi_basic() {
        let r = TritFloat::from_f32(2.0).powi(3);
        assert!(approx(r.to_f32(), 8.0, TOL), "2^3 should be 8, got {}", r.to_f32());
    }

    #[test]
    fn powi_confidence_preserved() {
        let a = TritFloat::from_f32_with_confidence(2.0, 0.75);
        let r = a.powi(2);
        assert!((r.confidence() - 0.75).abs() < 0.15);
    }

    #[test]
    fn sqrt_basic() {
        let r = TritFloat::from_f32(9.0).sqrt();
        assert!(approx(r.to_f32(), 3.0, TOL), "sqrt(9) should be 3, got {}", r.to_f32());
    }

    #[test]
    fn sqrt_negative_returns_zero_confidence() {
        let r = TritFloat::from_f32(-4.0).sqrt();
        assert!(r.is_zero());
        assert!(r.confidence() < 0.15, "sqrt of negative should have 0 confidence");
    }

    #[test]
    fn clamp_caps_value() {
        let hi = TritFloat::from_f32(5.0).clamp(0.0, 3.0);
        assert!(approx(hi.to_f32(), 3.0, TOL), "clamp(5, 0, 3) should be 3, got {}", hi.to_f32());
        let lo = TritFloat::from_f32(-2.0).clamp(0.0, 3.0);
        assert!(approx(lo.to_f32(), 0.0, 0.01), "clamp(-2, 0, 3) should be 0");
    }

    #[test]
    fn clamp_preserves_confidence() {
        let a = TritFloat::from_f32_with_confidence(10.0, 0.625);
        let r = a.clamp(0.0, 1.0);
        assert!((r.confidence() - 0.625).abs() < 0.15);
    }

    #[test]
    fn cmp_trit_ordering() {
        let big = TritFloat::from_f32(3.0);
        let small = TritFloat::from_f32(2.0);
        assert_eq!(big.cmp_trit(small).phase(),  1,  "3 > 2 should give +1");
        assert_eq!(small.cmp_trit(big).phase(), -1,  "2 < 3 should give -1");
        assert_eq!(big.cmp_trit(big).phase(),    0,  "x == x should give 0");
    }

    #[test]
    fn cmp_trit_confidence_is_min() {
        let a = TritFloat::from_f32_with_confidence(3.0, 1.0);
        let b = TritFloat::from_f32_with_confidence(2.0, 0.125);
        let r = a.cmp_trit(b);
        assert!(r.confidence() < 0.2, "cmp confidence should be min of inputs");
    }

    // ── Slice / SIMD-lite tests ───────────────────────────────────────────────

    #[test]
    fn softmax_sums_to_one() {
        let vals: Vec<TritFloat> = [1.0f32, 2.0, 3.0, 0.5]
            .iter().map(|&x| TritFloat::from_f32(x)).collect();
        let sm = TritFloat::softmax(&vals);
        let sum: f32 = sm.iter().map(|x| x.to_f32()).sum();
        assert!((sum - 1.0).abs() < 1e-4, "softmax should sum to 1.0, got {sum}");
    }

    #[test]
    fn softmax_confidence_is_min_of_inputs() {
        let vals = vec![
            TritFloat::from_f32_with_confidence(1.0, 1.0),
            TritFloat::from_f32_with_confidence(2.0, 0.125),
            TritFloat::from_f32_with_confidence(3.0, 1.0),
        ];
        let sm = TritFloat::softmax(&vals);
        for s in &sm {
            assert!(s.confidence() < 0.2,
                "softmax conf should be min of inputs (0.125), got {}", s.confidence());
        }
    }

    #[test]
    fn softmax_empty_slice() {
        assert_eq!(TritFloat::softmax(&[]).len(), 0);
    }

    #[test]
    fn pack_phases_u64_correctness() {
        let vals: Vec<TritFloat> = [1.0f32, 0.0, -1.0, 0.0, 2.0]
            .iter().map(|&x| TritFloat::from_f32(x)).collect();
        let mask = TritFloat::pack_phases_u64(&vals);
        // bits 1 and 3 should be set (zero-phase elements at indices 1 and 3)
        assert_eq!(mask & 1,  0, "index 0 (1.0) should not be zero-phase");
        assert_eq!(mask & 2,  2, "index 1 (0.0) should be zero-phase");
        assert_eq!(mask & 4,  0, "index 2 (-1.0) should not be zero-phase");
        assert_eq!(mask & 8,  8, "index 3 (0.0) should be zero-phase");
        assert_eq!(mask & 16, 0, "index 4 (2.0) should not be zero-phase");
        assert_eq!(mask.count_ones(), 2);
    }

    #[test]
    fn dot_prescan_matches_dot_with_skips() {
        let a: Vec<TritFloat> = [1.0f32, 0.0, 2.0, 0.0, 3.0]
            .iter().map(|&x| TritFloat::from_f32(x)).collect();
        let b: Vec<TritFloat> = [4.0f32, 5.0, 0.0, 6.0, 7.0]
            .iter().map(|&x| TritFloat::from_f32(x)).collect();

        let (r1, s1) = TritFloat::dot_with_skips(&a, &b);
        let (r2, s2) = TritFloat::dot_prescan(&a, &b);

        assert!(approx(r1.to_f32(), r2.to_f32(), 0.001),
            "prescan and dot_with_skips should match: {} vs {}", r1.to_f32(), r2.to_f32());
        assert_eq!(s1, s2, "skip counts should match: {s1} vs {s2}");
    }

    #[test]
    fn phase_digits_correct() {
        let vals: Vec<TritFloat> = [-1.0f32, 0.0, 1.0]
            .iter().map(|&x| TritFloat::from_f32(x)).collect();
        let pd = TritFloat::phase_digits(&vals);
        assert_eq!(pd[0], 0, "neg phase → digit 0");
        assert_eq!(pd[1], 1, "zero phase → digit 1");
        assert_eq!(pd[2], 2, "pos phase → digit 2");
    }
}
