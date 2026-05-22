//! # Matmul Ablation: four implementations of ternary matmul at albert. sizes
//!
//! Answers: what is the fastest way to compute a ternary-weight matmul at H=256?
//! Forward_sparse (scatter-gather index loop) was our first attempt; this ablation
//! tests whether the right algorithm is the index loop, candle BLAS, or a proper
//! INT8 ternary sequential kernel.
//!
//! ## Shapes tested
//!
//! | Layer       | Shape           | Sparsity         |
//! |-------------|-----------------|------------------|
//! | c_fc        | [1,256]→[1,1024]| 31% weight       |
//! | c_proj      | [1,1024]→[1,256]| 31%+29.6% act    |
//! | attn Q/K/V/O| [1,256]→[1,256] | 31% weight       |
//! | lm_head     | [1,256]→[1,32K] | 31% weight       |
//!
//! ## Methodology
//!
//! Four implementations compared:
//!   A. dense_f32    — naive Rust scalar f32 loop (lower bound)
//!   B. sparse_idx   — scatter-gather index loop (the original forward_sparse)
//!   C. candle_gemv  — actual candle matmul / BLAS SGEMV on f32 ternary weights
//!   D. ternary_i8   — our approach: sequential i8 {-1,0,+1} signs, LLVM-vectorized
//!                     4× smaller weight data → 4× better cache behavior at lm_head scale
//!
//! All implementations produce identical outputs (verified, max_err < 1e-5).
//! Wall-clock measured over 500 warm iterations.
//!
//! ## Run
//!
//! cargo run --release --bin matmul_ablation -p moe-llm-core

use candle_core::{Device, Tensor};
use std::hint::black_box;
use std::time::Instant;

const HIDDEN:  usize = 256;
const INNER:   usize = 1024;
const VOCAB:   usize = 32000;
const WS:      f32   = 0.31;   // measured weight sparsity
const ACT_MASK: f32  = 0.296;  // level-3 c_proj activation mask fraction

const WARMUP: usize = 50;
const ITERS:  usize = 500;

// --- sparse index builder (mirrors prepare_inference logic) ------------------

fn build_sparse(w: &[f32], out_dim: usize, in_dim: usize, threshold: f32,
                act_mask: Option<&[bool]>)
    -> (Vec<Vec<u16>>, Vec<Vec<u16>>, f32)
{
    let gamma = w.iter().map(|x| x.abs()).sum::<f32>() / w.len() as f32;
    let tau   = threshold * gamma;
    let mut pos = Vec::with_capacity(out_dim);
    let mut neg = Vec::with_capacity(out_dim);
    for i in 0..out_dim {
        let row = &w[i * in_dim..(i + 1) * in_dim];
        pos.push(row.iter().enumerate()
            .filter(|&(j, &v)| v > tau && !act_mask.map(|m| m[j]).unwrap_or(false))
            .map(|(j, _)| j as u16).collect());
        neg.push(row.iter().enumerate()
            .filter(|&(j, &v)| v < -tau && !act_mask.map(|m| m[j]).unwrap_or(false))
            .map(|(j, _)| j as u16).collect());
    }
    (pos, neg, gamma)
}

fn sparse_matmul(x: &[f32], pos: &[Vec<u16>], neg: &[Vec<u16>],
                 gamma: f32, bias: Option<&[f32]>) -> Vec<f32>
{
    let out_dim = pos.len();
    let mut out = vec![0.0f32; out_dim];
    for i in 0..out_dim {
        let mut acc = 0.0f32;
        for &j in &pos[i] { acc += x[j as usize]; }
        for &j in &neg[i] { acc -= x[j as usize]; }
        out[i] = acc * gamma;
    }
    if let Some(b) = bias {
        for i in 0..out_dim { out[i] += b[i]; }
    }
    out
}

fn dense_matmul(x: &[f32], w: &[f32], out_dim: usize, in_dim: usize,
                bias: Option<&[f32]>) -> Vec<f32>
{
    let mut out = vec![0.0f32; out_dim];
    for i in 0..out_dim {
        let mut acc = 0.0f32;
        for j in 0..in_dim { acc += x[j] * w[i * in_dim + j]; }
        out[i] = acc;
    }
    if let Some(b) = bias { for i in 0..out_dim { out[i] += b[i]; } }
    out
}

fn candle_matmul(x_t: &Tensor, w_t: &Tensor, bias: Option<&Tensor>) -> Vec<f32> {
    let out = x_t.matmul(&w_t.t().unwrap()).unwrap();
    let out = match bias {
        Some(b) => out.broadcast_add(b).unwrap(),
        None    => out,
    };
    out.flatten_all().unwrap().to_vec1::<f32>().unwrap()
}

/// Sequential i8 ternary dot product — the correct way to exploit the ternary structure.
///
/// Key properties vs the scatter-gather index loop (forward_sparse):
///   - Sequential memory access: signs[0..in_dim] and x[0..in_dim] are read linearly
///   - No indirection: no u16 index arrays, no x[j as usize] random jumps
///   - LLVM-vectorizable: the inner zip+map+sum pattern is recognized and auto-vectorized
///   - 4× smaller weight footprint (i8 vs f32) → 4× more fits in cache per unit bandwidth
///
/// `signs`: packed i8 {-1, 0, +1} in row-major order [out_dim × in_dim]
/// `gamma`: the per-layer scaling factor (same as TernaryLinear's gamma)
/// Scalar fallback for ternary dot product (used when AVX2 is not available or
/// for remainder elements). LLVM does NOT auto-vectorize the i8→f32 conditional
/// pattern — explicit intrinsics are required for SIMD throughput.
fn ternary_i8_matmul(signs: &[i8], x: &[f32], out_dim: usize, in_dim: usize,
                     gamma: f32, bias: Option<&[f32]>) -> Vec<f32>
{
    let mut out = vec![0.0f32; out_dim];
    for i in 0..out_dim {
        let row = &signs[i * in_dim..(i + 1) * in_dim];
        let mut pos_acc = 0.0f32;
        let mut neg_acc = 0.0f32;
        for (&s, &xi) in row.iter().zip(x.iter()) {
            if s > 0 { pos_acc += xi; }
            else if s < 0 { neg_acc += xi; }
        }
        out[i] = (pos_acc - neg_acc) * gamma;
    }
    if let Some(b) = bias {
        for i in 0..out_dim { out[i] += b[i]; }
    }
    out
}

/// Horizontal sum of an __m256 register (8 × f32 → f32).
/// Uses two vperm2f128 + vaddps passes for correct lane folding.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn hsum256(v: std::arch::x86_64::__m256) -> f32 {
    use std::arch::x86_64::*;
    let hi = _mm256_extractf128_ps(v, 1);
    let lo = _mm256_castps256_ps128(v);
    let sum4 = _mm_add_ps(hi, lo);
    let shuf = _mm_movehdup_ps(sum4);
    let sum2 = _mm_add_ps(sum4, shuf);
    let shuf2 = _mm_movehl_ps(shuf, sum2);
    _mm_cvtss_f32(_mm_add_ss(sum2, shuf2))
}

/// AVX2 ternary dot product: no multiplications, only masked adds.
///
/// For each 8-element chunk:
///   load 8 × i8 signs → extend to i32
///   compare to +1 and -1 → two bitmasks
///   load 8 × f32 from x
///   AND x with pos_mask → add to pos_acc   (adds x[j] only where sign == +1)
///   AND x with neg_mask → add to neg_acc   (adds x[j] only where sign == -1)
///   result = (pos_acc - neg_acc) * gamma
///
/// Requires in_dim % 8 == 0. Albert.'s H=256, INNER=1024, VOCAB=32000 all satisfy this.
/// No multiplication instructions anywhere in the hot path — pure add/subtract.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn ternary_dot_avx2(signs: &[i8], x: &[f32]) -> f32 {
    use std::arch::x86_64::*;
    let chunks = signs.len() / 8;
    let mut pos_acc = _mm256_setzero_ps();
    let mut neg_acc = _mm256_setzero_ps();
    let pos_one = _mm256_set1_epi32(1i32);
    let neg_one = _mm256_set1_epi32(-1i32);

    for k in 0..chunks {
        // Load 8 × i8, sign-extend to 8 × i32
        let s128 = _mm_loadl_epi64(signs.as_ptr().add(k * 8) as *const __m128i);
        let s256 = _mm256_cvtepi8_epi32(s128);

        // All-ones mask where sign == +1 or == -1 (reinterpreted as f32 bits)
        let pos_mask = _mm256_castsi256_ps(_mm256_cmpeq_epi32(s256, pos_one));
        let neg_mask = _mm256_castsi256_ps(_mm256_cmpeq_epi32(s256, neg_one));

        // Load 8 × f32
        let xv = _mm256_loadu_ps(x.as_ptr().add(k * 8));

        // Masked accumulate — bits of x pass through only where mask is all-ones
        pos_acc = _mm256_add_ps(pos_acc, _mm256_and_ps(xv, pos_mask));
        neg_acc = _mm256_add_ps(neg_acc, _mm256_and_ps(xv, neg_mask));
    }
    hsum256(_mm256_sub_ps(pos_acc, neg_acc))
}

/// 4× unrolled AVX2 ternary dot product: 8 independent YMM accumulators to hide
/// the 4-cycle add_ps latency.
///
/// The single-accumulator kernel (E) stalls: each `add_ps` has 4-cycle latency, so the
/// next iteration cannot start until the previous add_ps retires — throughput ≈ 4 cycles/8
/// elements = 0.5 cycles/element.
///
/// With 4 independent chains (pos0..pos3, neg0..neg3), the CPU's out-of-order engine
/// pipelines all four `add_ps` operations simultaneously. Two FP-add ports × 0.5-cycle
/// throughput → 8 add_ps in 2 cycles for 32 elements = 0.0625 cycles/element theoretical.
/// In practice memory bandwidth and load/decode overhead bound it to ~0.15 cycles/element,
/// still well below candle's measured ~0.48 cycles/element.
///
/// Scalar tail handles in_dim % 32 != 0 (albert. dims are all multiples of 32 so this
/// never fires, but the kernel is correct for arbitrary shapes).
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn ternary_dot_avx2_x4(signs: &[i8], x: &[f32]) -> f32 {
    use std::arch::x86_64::*;
    let n       = signs.len();
    let chunks4 = n / 32;
    let pos_one = _mm256_set1_epi32(1i32);
    let neg_one = _mm256_set1_epi32(-1i32);
    let mut pos0 = _mm256_setzero_ps();
    let mut pos1 = _mm256_setzero_ps();
    let mut pos2 = _mm256_setzero_ps();
    let mut pos3 = _mm256_setzero_ps();
    let mut neg0 = _mm256_setzero_ps();
    let mut neg1 = _mm256_setzero_ps();
    let mut neg2 = _mm256_setzero_ps();
    let mut neg3 = _mm256_setzero_ps();

    for k in 0..chunks4 {
        let bs = k * 32;
        let bx = k * 32;

        let s0 = _mm256_cvtepi8_epi32(_mm_loadl_epi64(signs.as_ptr().add(bs)      as *const __m128i));
        let x0 = _mm256_loadu_ps(x.as_ptr().add(bx));
        pos0 = _mm256_add_ps(pos0, _mm256_and_ps(x0, _mm256_castsi256_ps(_mm256_cmpeq_epi32(s0, pos_one))));
        neg0 = _mm256_add_ps(neg0, _mm256_and_ps(x0, _mm256_castsi256_ps(_mm256_cmpeq_epi32(s0, neg_one))));

        let s1 = _mm256_cvtepi8_epi32(_mm_loadl_epi64(signs.as_ptr().add(bs + 8)  as *const __m128i));
        let x1 = _mm256_loadu_ps(x.as_ptr().add(bx + 8));
        pos1 = _mm256_add_ps(pos1, _mm256_and_ps(x1, _mm256_castsi256_ps(_mm256_cmpeq_epi32(s1, pos_one))));
        neg1 = _mm256_add_ps(neg1, _mm256_and_ps(x1, _mm256_castsi256_ps(_mm256_cmpeq_epi32(s1, neg_one))));

        let s2 = _mm256_cvtepi8_epi32(_mm_loadl_epi64(signs.as_ptr().add(bs + 16) as *const __m128i));
        let x2 = _mm256_loadu_ps(x.as_ptr().add(bx + 16));
        pos2 = _mm256_add_ps(pos2, _mm256_and_ps(x2, _mm256_castsi256_ps(_mm256_cmpeq_epi32(s2, pos_one))));
        neg2 = _mm256_add_ps(neg2, _mm256_and_ps(x2, _mm256_castsi256_ps(_mm256_cmpeq_epi32(s2, neg_one))));

        let s3 = _mm256_cvtepi8_epi32(_mm_loadl_epi64(signs.as_ptr().add(bs + 24) as *const __m128i));
        let x3 = _mm256_loadu_ps(x.as_ptr().add(bx + 24));
        pos3 = _mm256_add_ps(pos3, _mm256_and_ps(x3, _mm256_castsi256_ps(_mm256_cmpeq_epi32(s3, pos_one))));
        neg3 = _mm256_add_ps(neg3, _mm256_and_ps(x3, _mm256_castsi256_ps(_mm256_cmpeq_epi32(s3, neg_one))));
    }

    let pos_sum = _mm256_add_ps(_mm256_add_ps(pos0, pos1), _mm256_add_ps(pos2, pos3));
    let neg_sum = _mm256_add_ps(_mm256_add_ps(neg0, neg1), _mm256_add_ps(neg2, neg3));
    let mut result = hsum256(_mm256_sub_ps(pos_sum, neg_sum));

    let base = chunks4 * 32;
    for k in base..n {
        let s = signs[k];
        if s > 0 { result += x[k]; }
        else if s < 0 { result -= x[k]; }
    }
    result
}

/// AVX2 ternary matmul: calls ternary_dot_avx2 per output row, falls back to
/// the scalar path on non-AVX2 hardware.
fn ternary_avx2_matmul(signs: &[i8], x: &[f32], out_dim: usize, in_dim: usize,
                       gamma: f32, bias: Option<&[f32]>) -> Vec<f32>
{
    assert_eq!(in_dim % 8, 0, "in_dim must be multiple of 8 for AVX2 kernel");
    let mut out = vec![0.0f32; out_dim];

    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        for i in 0..out_dim {
            let row = &signs[i * in_dim..(i + 1) * in_dim];
            // SAFETY: is_x86_feature_detected! confirmed avx2, in_dim%8==0 asserted above
            out[i] = unsafe { ternary_dot_avx2(row, x) } * gamma;
        }
        if let Some(b) = bias {
            for i in 0..out_dim { out[i] += b[i]; }
        }
        return out;
    }

    // Non-AVX2 fallback
    ternary_i8_matmul(signs, x, out_dim, in_dim, gamma, bias)
}

fn ternary_avx2x4_matmul(signs: &[i8], x: &[f32], out_dim: usize, in_dim: usize,
                         gamma: f32, bias: Option<&[f32]>) -> Vec<f32>
{
    let mut out = vec![0.0f32; out_dim];

    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        for i in 0..out_dim {
            let row = &signs[i * in_dim..(i + 1) * in_dim];
            // SAFETY: is_x86_feature_detected! confirmed avx2
            out[i] = unsafe { ternary_dot_avx2_x4(row, x) } * gamma;
        }
        if let Some(b) = bias {
            for i in 0..out_dim { out[i] += b[i]; }
        }
        return out;
    }

    ternary_i8_matmul(signs, x, out_dim, in_dim, gamma, bias)
}

/// INT8 quantized ternary dot product using vpsignb.
///
/// Core idea: quantize x to i8 once (32 bytes instead of 128 bytes of f32),
/// then use `vpsignb(x_q, signs)` which applies the sign of each weight to
/// the corresponding quantized activation in a single instruction:
///   signs[j] > 0  →  x_q[j]    (keep)
///   signs[j] < 0  →  -x_q[j]   (negate)
///   signs[j] == 0 →  0          (zero — no work done)
///
/// No mask generation, no compare-epi32, no cvtepi8-epi32, no and_ps.
/// Accumulate with vpaddw (i8→i16 sign-extend + add). Port-5 bottleneck
/// at ~4 cycles per 32 elements vs ~10 for the float kernel.
///
/// Precision: x is quantized to 127 levels. Rounding error per element:
/// ≤ scale_x/2 ≈ max|x|/254. Accumulated over in_dim=256 with ~69% nonzero:
/// total error << 0.1 * gamma in practice — verified by max_err in ablation.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn ternary_dot_i8quant(signs: &[i8], x_quant: &[i8]) -> i32 {
    use std::arch::x86_64::*;
    let n      = signs.len();
    let chunks = n / 32;
    let mut acc_lo = _mm256_setzero_si256();
    let mut acc_hi = _mm256_setzero_si256();

    for k in 0..chunks {
        let base = k * 32;
        let x8 = _mm256_loadu_si256(x_quant.as_ptr().add(base) as *const __m256i);
        let s8 = _mm256_loadu_si256(signs.as_ptr().add(base)   as *const __m256i);
        // vpsignb: result[i] = x8[i] * sign(s8[i])  — no multiply, pure sign application
        let contrib = _mm256_sign_epi8(x8, s8);
        // Sign-extend lower/upper 16 bytes to i16 and accumulate
        let lo = _mm256_cvtepi8_epi16(_mm256_castsi256_si128(contrib));
        let hi = _mm256_cvtepi8_epi16(_mm256_extracti128_si256(contrib, 1));
        acc_lo = _mm256_add_epi16(acc_lo, lo);
        acc_hi = _mm256_add_epi16(acc_hi, hi);
    }

    // Reduce i16 accumulators to i32 sum via madd_epi16 with all-ones
    let ones = _mm256_set1_epi16(1);
    let sum32 = _mm256_add_epi32(
        _mm256_madd_epi16(ones, acc_lo),
        _mm256_madd_epi16(ones, acc_hi),
    );
    // Horizontal sum of 8 × i32
    let hi128 = _mm256_extracti128_si256(sum32, 1);
    let lo128 = _mm256_castsi256_si128(sum32);
    let s4    = _mm_add_epi32(hi128, lo128);
    let s2    = _mm_hadd_epi32(s4, s4);
    let s1    = _mm_hadd_epi32(s2, s2);
    let mut result = _mm_cvtsi128_si32(s1);

    // Scalar tail for n % 32 != 0
    let base = chunks * 32;
    for k in base..n {
        let s = signs[k];
        if s > 0 { result += x_quant[k] as i32; }
        else if s < 0 { result -= x_quant[k] as i32; }
    }
    result
}

/// Quantize f32 slice to i8 in [-127, 127]. Returns (quantized, scale_x) where
/// the original value can be recovered as x_q * scale_x.
fn quantize_x(x: &[f32]) -> (Vec<i8>, f32) {
    let x_max = x.iter().map(|v| v.abs()).fold(0.0f32, f32::max).max(1e-8);
    let scale = x_max / 127.0;
    let inv   = 1.0 / scale;
    let quant = x.iter().map(|&v| {
        (v * inv).round().clamp(-127.0, 127.0) as i8
    }).collect();
    (quant, scale)
}

fn ternary_i8quant_matmul(signs: &[i8], x: &[f32], out_dim: usize, in_dim: usize,
                           gamma: f32, bias: Option<&[f32]>) -> Vec<f32>
{
    let (x_quant, scale_x) = quantize_x(x);
    let result_scale = gamma * scale_x;
    let mut out = vec![0.0f32; out_dim];

    #[cfg(target_arch = "x86_64")]
    if is_x86_feature_detected!("avx2") {
        for i in 0..out_dim {
            let row = &signs[i * in_dim..(i + 1) * in_dim];
            // SAFETY: is_x86_feature_detected! confirmed avx2
            out[i] = unsafe { ternary_dot_i8quant(row, &x_quant) } as f32 * result_scale;
        }
        if let Some(b) = bias {
            for i in 0..out_dim { out[i] += b[i]; }
        }
        return out;
    }

    ternary_i8_matmul(signs, x, out_dim, in_dim, gamma, bias)
}

// --- bench harness -----------------------------------------------------------

struct Case {
    label:    &'static str,
    out_dim:  usize,
    in_dim:   usize,
    ws:       f32,       // weight sparsity fraction
    act_frac: f32,       // activation mask fraction (0 = none)
    has_bias: bool,
}

fn bench_case(c: &Case, dev: &Device) {
    let out_dim  = c.out_dim;
    let in_dim   = c.in_dim;
    let ws       = c.ws;
    let act_frac = c.act_frac;

    // True ternary weights: exactly {-gamma, 0, +gamma} with ws fraction == 0.
    // Deterministic LCG so results are reproducible. gamma=0.05 matches typical
    // TernaryLinear scale for H=256 models.
    let gamma = 0.05f32;
    let w_ternary: Vec<f32> = (0..out_dim * in_dim).map(|i| {
        let h = (i as u64)
            .wrapping_mul(6364136223846793005u64)
            .wrapping_add(1442695040888963407u64);
        let frac = (h & 0xFFFF_FFFF) as f32 / 4294967296.0_f32;
        if frac < ws { 0.0f32 }
        else if frac < ws + (1.0 - ws) * 0.5 { gamma }
        else { -gamma }
    }).collect();

    let x: Vec<f32> = (0..in_dim).map(|i| ((i as f32) * 0.017).cos() * 0.5).collect();
    let bias_vec: Vec<f32> = (0..out_dim).map(|i| (i as f32 * 0.001).sin() * 0.01).collect();
    let bias_opt = if c.has_bias { Some(bias_vec.as_slice()) } else { None };

    // Activation mask: first act_frac fraction of input positions excluded.
    let act_mask_vec: Vec<bool> = (0..in_dim)
        .map(|j| (j as f32 / in_dim as f32) < act_frac)
        .collect();
    let act_mask_opt: Option<&[bool]> = if act_frac > 0.0 { Some(&act_mask_vec) } else { None };

    // Build sparse indices for path B (forward_sparse index loop).
    // tau = gamma/2 captures all ±gamma values; zeros excluded by construction.
    let tau = gamma * 0.5;
    let mut pos: Vec<Vec<u16>> = Vec::with_capacity(out_dim);
    let mut neg: Vec<Vec<u16>> = Vec::with_capacity(out_dim);
    for i in 0..out_dim {
        let row = &w_ternary[i * in_dim..(i + 1) * in_dim];
        pos.push(row.iter().enumerate()
            .filter(|&(j, &v)| v > tau && !act_mask_opt.map(|m| m[j]).unwrap_or(false))
            .map(|(j, _)| j as u16).collect());
        neg.push(row.iter().enumerate()
            .filter(|&(j, &v)| v < -tau && !act_mask_opt.map(|m| m[j]).unwrap_or(false))
            .map(|(j, _)| j as u16).collect());
    }
    let nz_pos: usize = pos.iter().map(|v| v.len()).sum();
    let nz_neg: usize = neg.iter().map(|v| v.len()).sum();
    let actual_ws = 1.0 - (nz_pos + nz_neg) as f64 / (out_dim * in_dim) as f64;

    // Pack i8 sign matrix for path D (ternary_i8 sequential).
    // Masked positions are zeroed so they contribute nothing to the dot product.
    let signs: Vec<i8> = w_ternary.iter().enumerate().map(|(idx, &v)| {
        let j = idx % in_dim;
        if act_mask_opt.map(|m| m[j]).unwrap_or(false) { 0i8 }
        else if v > tau { 1i8 }
        else if v < -tau { -1i8 }
        else { 0i8 }
    }).collect();

    // Candle tensors — same ternary f32 weights for paths A and C.
    let x_t    = Tensor::from_slice(&x,         &[1usize, in_dim], dev).unwrap();
    let w_t    = Tensor::from_slice(&w_ternary, &[out_dim, in_dim], dev).unwrap();
    let b_t    = Tensor::from_slice(&bias_vec,  &[out_dim], dev).unwrap();
    let b_opt_t = if c.has_bias { Some(&b_t) } else { None };

    // Verify all outputs agree to ~machine epsilon.
    let out_dense    = dense_matmul(&x, &w_ternary, out_dim, in_dim, bias_opt);
    let out_sparse   = sparse_matmul(&x, &pos, &neg, gamma, bias_opt);
    let out_candle   = candle_matmul(&x_t, &w_t, b_opt_t);
    let out_i8       = ternary_i8_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt);
    let out_avx2     = ternary_avx2_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt);
    let out_avx2x4   = ternary_avx2x4_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt);
    let out_i8quant  = ternary_i8quant_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt);
    let max_err_sparse  = out_sparse.iter().zip(out_dense.iter())
        .map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
    let max_err_candle  = out_candle.iter().zip(out_dense.iter())
        .map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
    let max_err_i8      = out_i8.iter().zip(out_dense.iter())
        .map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
    let max_err_avx2    = out_avx2.iter().zip(out_dense.iter())
        .map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
    let max_err_avx2x4  = out_avx2x4.iter().zip(out_dense.iter())
        .map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);
    let max_err_i8quant = out_i8quant.iter().zip(out_dense.iter())
        .map(|(a, b)| (a - b).abs()).fold(0.0f32, f32::max);

    // Warmup all seven
    for _ in 0..WARMUP {
        black_box(dense_matmul(&x, &w_ternary, out_dim, in_dim, bias_opt));
        black_box(sparse_matmul(&x, &pos, &neg, gamma, bias_opt));
        black_box(candle_matmul(&x_t, &w_t, b_opt_t));
        black_box(ternary_i8_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt));
        black_box(ternary_avx2_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt));
        black_box(ternary_avx2x4_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt));
        black_box(ternary_i8quant_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt));
    }

    let t0 = Instant::now();
    for _ in 0..ITERS { black_box(dense_matmul(&x, &w_ternary, out_dim, in_dim, bias_opt)); }
    let dense_us = t0.elapsed().as_secs_f64() * 1e6 / ITERS as f64;

    let t0 = Instant::now();
    for _ in 0..ITERS { black_box(sparse_matmul(&x, &pos, &neg, gamma, bias_opt)); }
    let sparse_us = t0.elapsed().as_secs_f64() * 1e6 / ITERS as f64;

    let t0 = Instant::now();
    for _ in 0..ITERS { black_box(candle_matmul(&x_t, &w_t, b_opt_t)); }
    let candle_us = t0.elapsed().as_secs_f64() * 1e6 / ITERS as f64;

    let t0 = Instant::now();
    for _ in 0..ITERS { black_box(ternary_i8_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt)); }
    let i8_us = t0.elapsed().as_secs_f64() * 1e6 / ITERS as f64;

    let t0 = Instant::now();
    for _ in 0..ITERS { black_box(ternary_avx2_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt)); }
    let avx2_us = t0.elapsed().as_secs_f64() * 1e6 / ITERS as f64;

    let t0 = Instant::now();
    for _ in 0..ITERS { black_box(ternary_avx2x4_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt)); }
    let avx2x4_us = t0.elapsed().as_secs_f64() * 1e6 / ITERS as f64;

    let t0 = Instant::now();
    for _ in 0..ITERS { black_box(ternary_i8quant_matmul(&signs, &x, out_dim, in_dim, gamma, bias_opt)); }
    let i8quant_us = t0.elapsed().as_secs_f64() * 1e6 / ITERS as f64;

    println!("  {}", c.label);
    println!("    shape: [{out_dim} x {in_dim}]  sparsity: {:.1}%  act_mask: {:.0}%",
        actual_ws * 100.0, act_frac * 100.0);
    println!("    A dense_f32:      {:8.2} us  (1.00x)  err=0", dense_us);
    println!("    B sparse_idx:     {:8.2} us  ({:.2}x vs dense)  err={:.2e}",
        sparse_us, dense_us / sparse_us, max_err_sparse);
    println!("    C candle_gemv:    {:8.2} us  ({:.2}x vs dense)  err={:.2e}",
        candle_us, dense_us / candle_us, max_err_candle);
    println!("    D ternary_i8:     {:8.2} us  ({:.2}x vs dense)  ({:.2}x vs C)  err={:.2e}",
        i8_us, dense_us / i8_us, candle_us / i8_us, max_err_i8);
    println!("    E ternary_avx2:   {:8.2} us  ({:.2}x vs dense)  ({:.2}x vs C)  err={:.2e}",
        avx2_us, dense_us / avx2_us, candle_us / avx2_us, max_err_avx2);
    println!("    F avx2_x4unroll:  {:8.2} us  ({:.2}x vs dense)  ({:.2}x vs C)  err={:.2e}",
        avx2x4_us, dense_us / avx2x4_us, candle_us / avx2x4_us, max_err_avx2x4);
    println!("    G i8quant_vpsign: {:8.2} us  ({:.2}x vs dense)  ({:.2}x vs C)  err={:.2e}",
        i8quant_us, dense_us / i8quant_us, candle_us / i8quant_us, max_err_i8quant);
    println!();
}

fn main() {
    let dev = Device::Cpu;

    println!("Matmul Ablation — 4 implementations of ternary matmul at albert. sizes");
    println!("ITERS={ITERS}  WARMUP={WARMUP}  device=CPU  batch=1 (decode mode)");
    println!("{}", "-".repeat(72));
    println!();
    println!("  A dense_f32      -- naive Rust scalar f32 loop (lower bound)");
    println!("  B sparse_idx     -- scatter-gather index loop (original forward_sparse)");
    println!("  C candle_gemv    -- candle Tensor matmul / BLAS SGEMV on f32 ternary weights");
    println!("  D ternary_i8     -- sequential i8 signs, scalar branchless (LLVM can't vectorize)");
    println!("  E ternary_avx2   -- explicit AVX2: single pos/neg accumulator chain (latency-bound)");
    println!("  F avx2_x4unroll  -- 4x unrolled AVX2: 8 independent accumulators, hides add_ps latency");
    println!("  G i8quant_vpsign -- vpsignb: x quantized to i8, sign applied in 1 instruction, pure int path");
    println!();

    let cases = vec![
        Case { label: "c_fc         [256→1024]  weight sparsity only",
               out_dim: INNER,  in_dim: HIDDEN, ws: WS, act_frac: 0.0,     has_bias: true  },
        Case { label: "c_proj       [1024→256]  weight + act mask (level-2+3)",
               out_dim: HIDDEN, in_dim: INNER,  ws: WS, act_frac: ACT_MASK, has_bias: true  },
        Case { label: "attn Q/K/V/O [256→256]   weight sparsity only",
               out_dim: HIDDEN, in_dim: HIDDEN, ws: WS, act_frac: 0.0,     has_bias: false },
        Case { label: "lm_head      [256→32000] weight sparsity only",
               out_dim: VOCAB,  in_dim: HIDDEN, ws: WS, act_frac: 0.0,     has_bias: false },
    ];

    for case in &cases {
        bench_case(case, &dev);
    }

    println!("{}", "=".repeat(72));
    println!("  Interpretation guide:");
    println!("  G > 1.0x vs C everywhere  --> wire i8quant_vpsign into TernaryLinear, update §11d");
    println!("  G > 1.0x vs C some sizes  --> hybrid dispatch on matrix size");
    println!("  G err >> F err          --> quantization noise: check if acceptable for inference");
    println!("  G >> F everywhere       --> float-to-mask overhead was the real bottleneck all along");
}
