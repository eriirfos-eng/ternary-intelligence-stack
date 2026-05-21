// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
//
// TritFloatTensor — N-dimensional tensor of TritFloats.
//
// Every element carries its own confidence field. The confidence map of the
// tensor is an emergent property of the computation that produced it — there
// is no separate uncertainty bookkeeping layer.
//
// Architecture integration point:
//   TritMatrix (weights, exact {-1,0,+1}, conf=1.0)
//     ×
//   TritFloatTensor (activations, float magnitudes, conf propagated)
//     →
//   TritFloatTensor (outputs, conf reflects the least-certain activation path)

use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::tritfloat::TritFloat;
use crate::{Trit, TritMatrix};

// ─── Core type ────────────────────────────────────────────────────────────────

/// An N-dimensional tensor of TritFloats laid out in row-major order.
///
/// Each element carries its own confidence field. The tensor's overall
/// confidence is determined by `min_confidence()` or `mean_confidence()`.
#[derive(Clone, Debug)]
pub struct TritFloatTensor {
    pub data: Vec<TritFloat>,
    pub shape: Vec<usize>,
}

// ─── Constructors ─────────────────────────────────────────────────────────────

impl TritFloatTensor {
    /// All-zero tensor with neutral confidence (0.5) at every element.
    pub fn zeros(shape: &[usize]) -> Self {
        let numel = shape.iter().product();
        Self { data: vec![TritFloat::zero(); numel], shape: shape.to_vec() }
    }

    /// All-ones tensor with maximum confidence at every element.
    pub fn ones(shape: &[usize]) -> Self {
        let numel = shape.iter().product::<usize>();
        Self {
            data: vec![TritFloat::from_f32(1.0); numel],
            shape: shape.to_vec(),
        }
    }

    /// Build from a flat f32 slice, all elements get confidence=1.0.
    pub fn from_f32_slice(data: &[f32], shape: &[usize]) -> Self {
        assert_eq!(data.len(), shape.iter().product::<usize>(),
            "data length must equal product of shape dimensions");
        Self {
            data: data.iter().map(|&x| TritFloat::from_f32(x)).collect(),
            shape: shape.to_vec(),
        }
    }

    /// Build from f32 values with per-element confidence.
    pub fn from_f32_with_confidence(vals: &[f32], conf: &[f32], shape: &[usize]) -> Self {
        assert_eq!(vals.len(), shape.iter().product::<usize>());
        assert_eq!(vals.len(), conf.len());
        Self {
            data: vals.iter().zip(conf.iter())
                .map(|(&v, &c)| TritFloat::from_f32_with_confidence(v, c))
                .collect(),
            shape: shape.to_vec(),
        }
    }

    /// Convert a TritMatrix to a 2D TritFloatTensor.
    ///
    /// Weights are exactly {-1, 0, +1} — they carry maximum confidence (1.0).
    pub fn from_tritmatrix(m: &TritMatrix) -> Self {
        let data = m.data.iter().map(|&t| {
            let v = match t {
                Trit::Affirm =>  1.0f32,
                Trit::Reject => -1.0,
                Trit::Tend   =>  0.0,
            };
            TritFloat::from_f32_with_confidence(v, 1.0)
        }).collect();
        Self { data, shape: vec![m.rows, m.cols] }
    }

    // ── Shape and access ──────────────────────────────────────────────────────

    pub fn shape(&self) -> &[usize] { &self.shape }
    pub fn ndim(&self)  -> usize { self.shape.len() }
    pub fn numel(&self) -> usize { self.data.len() }

    /// Row-major flat index from a multi-dimensional index.
    fn flat_idx(&self, idx: &[usize]) -> usize {
        assert_eq!(idx.len(), self.ndim(), "index rank must match tensor rank");
        let mut flat = 0usize;
        let mut stride = 1usize;
        for i in (0..self.ndim()).rev() {
            flat   += idx[i] * stride;
            stride *= self.shape[i];
        }
        flat
    }

    pub fn get(&self, idx: &[usize]) -> TritFloat {
        self.data[self.flat_idx(idx)]
    }

    pub fn set(&mut self, idx: &[usize], val: TritFloat) {
        let flat = self.flat_idx(idx);
        self.data[flat] = val;
    }

    // ── Matmul — TritFloatTensor × TritFloatTensor ────────────────────────────

    /// 2D matrix multiply: [m, k] × [k, n] → [m, n].
    ///
    /// Each output element's confidence = min confidence over all contributing
    /// (a_i × b_i) multiplications. Zero-phase activations skip their MAC
    /// (@sparseskip at activation level) but still participate in the
    /// confidence running-minimum so uncertain zeros don't disappear.
    /// Rows are computed in parallel via Rayon.
    pub fn matmul(a: &Self, b: &Self) -> Self {
        assert_eq!(a.ndim(), 2, "matmul requires 2D tensors");
        assert_eq!(b.ndim(), 2, "matmul requires 2D tensors");
        let (m, k) = (a.shape[0], a.shape[1]);
        let (k2, n) = (b.shape[0], b.shape[1]);
        assert_eq!(k, k2, "matmul: a.cols ({k}) must equal b.rows ({k2})");

        let mut out_data = vec![TritFloat::zero(); m * n];

        out_data.par_chunks_mut(n).enumerate().for_each(|(row, out_row)| {
            for col in 0..n {
                let mut acc      = 0.0f32;
                let mut min_conf = 1.0f32;
                for i in 0..k {
                    let ai = a.data[row * k + i];
                    let bi = b.data[i * n + col];
                    let c  = TritFloat::mul_confidence(ai, bi);
                    if c < min_conf { min_conf = c; }
                    if !ai.is_zero() && !bi.is_zero() {
                        acc += ai.to_f32() * bi.to_f32();
                    }
                }
                out_row[col] = TritFloat::from_f32_with_confidence(acc, min_conf);
            }
        });

        Self { data: out_data, shape: vec![m, n] }
    }

    /// Matmul returning (result, total_macs_skipped) for sparsity instrumentation.
    pub fn matmul_sparse(a: &Self, b: &Self) -> (Self, usize) {
        assert_eq!(a.ndim(), 2);
        assert_eq!(b.ndim(), 2);
        let (m, k) = (a.shape[0], a.shape[1]);
        let (k2, n) = (b.shape[0], b.shape[1]);
        assert_eq!(k, k2);

        let mut out_data    = vec![TritFloat::zero(); m * n];
        let total_skipped   = AtomicUsize::new(0);

        out_data.par_chunks_mut(n).enumerate().for_each(|(row, out_row)| {
            let mut row_skipped = 0usize;
            for col in 0..n {
                let mut acc      = 0.0f32;
                let mut min_conf = 1.0f32;
                for i in 0..k {
                    let ai = a.data[row * k + i];
                    let bi = b.data[i * n + col];
                    let c  = TritFloat::mul_confidence(ai, bi);
                    if c < min_conf { min_conf = c; }
                    if ai.is_zero() || bi.is_zero() {
                        row_skipped += 1;
                    } else {
                        acc += ai.to_f32() * bi.to_f32();
                    }
                }
                out_row[col] = TritFloat::from_f32_with_confidence(acc, min_conf);
            }
            total_skipped.fetch_add(row_skipped, Ordering::Relaxed);
        });

        let skipped = total_skipped.load(Ordering::Relaxed);
        (Self { data: out_data, shape: vec![m, n] }, skipped)
    }

    // ── Matmul — TritFloatTensor × TritMatrix (inference hot path) ────────────

    /// Multiply float activations by exact ternary weights (TritMatrix).
    ///
    /// This is the inference-time hot path: activations carry live confidence,
    /// weights are exact {-1,0,+1} with confidence=1.0, so output confidence
    /// = min activation confidence over each dot product.
    ///
    /// @sparseskip fires on BOTH activation zeros (is_zero()) AND weight zeros
    /// (w == 0), giving the full combined sparsity savings.
    ///
    /// Returns (output_tensor, total_macs_skipped).
    pub fn matmul_trit(activations: &Self, weights: &TritMatrix) -> (Self, usize) {
        assert_eq!(activations.ndim(), 2,
            "matmul_trit requires 2D activation tensor");
        let (m, k) = (activations.shape[0], activations.shape[1]);
        assert_eq!(k, weights.rows,
            "activation cols ({k}) must match weight rows ({})", weights.rows);
        let n = weights.cols;

        let w_i8 = weights.to_i8_vec();
        let mut out_data  = vec![TritFloat::zero(); m * n];
        let total_skipped = AtomicUsize::new(0);

        out_data.par_chunks_mut(n).enumerate().for_each(|(row, out_row)| {
            let mut row_skipped = 0usize;
            let act_row = &activations.data[row * k..(row + 1) * k];

            for col in 0..n {
                let mut acc      = 0.0f32;
                let mut min_conf = 1.0f32;
                for i in 0..k {
                    let ai = act_row[i];
                    let wi = w_i8[i * n + col];
                    // Weight conf is 1.0, so min = activation conf
                    let c = ai.confidence();
                    if c < min_conf { min_conf = c; }
                    if ai.is_zero() || wi == 0 {
                        row_skipped += 1;
                    } else {
                        acc += ai.to_f32() * (wi as f32);
                    }
                }
                out_row[col] = TritFloat::from_f32_with_confidence(acc, min_conf);
            }
            total_skipped.fetch_add(row_skipped, Ordering::Relaxed);
        });

        (Self { data: out_data, shape: vec![m, n] },
         total_skipped.load(Ordering::Relaxed))
    }

    // ── Elementwise operations ────────────────────────────────────────────────

    pub fn add_elementwise(a: &Self, b: &Self) -> Self {
        assert_eq!(a.shape, b.shape, "elementwise add requires equal shapes");
        Self {
            data:  a.data.iter().zip(b.data.iter()).map(|(&ai, &bi)| ai.add(bi)).collect(),
            shape: a.shape.clone(),
        }
    }

    pub fn mul_elementwise(a: &Self, b: &Self) -> Self {
        assert_eq!(a.shape, b.shape, "elementwise mul requires equal shapes");
        Self {
            data:  a.data.iter().zip(b.data.iter()).map(|(&ai, &bi)| ai.mul(bi)).collect(),
            shape: a.shape.clone(),
        }
    }

    /// Apply a function to every element in parallel.
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(TritFloat) -> TritFloat + Sync + Send,
    {
        Self {
            data:  self.data.par_iter().map(|&x| f(x)).collect(),
            shape: self.shape.clone(),
        }
    }

    // ── Reductions ────────────────────────────────────────────────────────────

    pub fn sum_all(&self) -> TritFloat {
        self.data.iter().fold(TritFloat::zero(), |acc, &x| acc.add(x))
    }

    pub fn mean_all(&self) -> TritFloat {
        if self.data.is_empty() { return TritFloat::zero(); }
        let s = self.sum_all();
        TritFloat::from_f32_with_confidence(
            s.to_f32() / self.data.len() as f32,
            s.confidence(),
        )
    }

    /// Minimum confidence across all elements: how certain is the least-certain value?
    pub fn min_confidence(&self) -> f32 {
        self.data.iter().map(|x| x.confidence()).fold(1.0f32, f32::min)
    }

    /// Mean confidence: average epistemic certainty across the tensor.
    pub fn mean_confidence(&self) -> f32 {
        if self.data.is_empty() { return 0.0; }
        self.data.iter().map(|x| x.confidence()).sum::<f32>() / self.data.len() as f32
    }

    /// Histogram of confidence states across all elements.
    /// Index i = count of elements with confidence ≈ i/8.
    /// The 9 bins correspond to the 9 discrete states of the 2-trit confidence field.
    pub fn confidence_histogram(&self) -> [usize; 9] {
        let mut hist = [0usize; 9];
        for x in &self.data {
            let idx = (x.confidence() * 8.0).round() as usize;
            hist[idx.min(8)] += 1;
        }
        hist
    }

    // ── Sparsity ──────────────────────────────────────────────────────────────

    /// Fraction of elements with zero phase (exactly 0.0 value).
    pub fn sparsity(&self) -> f64 {
        let zeros = self.data.iter().filter(|x| x.is_zero()).count();
        zeros as f64 / self.data.len().max(1) as f64
    }

    // ── Conversions ───────────────────────────────────────────────────────────

    /// Extract f32 values, discarding confidence information.
    pub fn to_f32_vec(&self) -> Vec<f32> {
        self.data.iter().map(|x| x.to_f32()).collect()
    }

    /// Quantize to TritMatrix: positive phase → Affirm, negative → Reject, zero → Tend.
    pub fn to_tritmatrix(&self) -> TritMatrix {
        assert_eq!(self.ndim(), 2, "to_tritmatrix requires a 2D tensor");
        let data = self.data.iter().map(|x| match x.phase() {
            1  => Trit::Affirm,
            -1 => Trit::Reject,
            _  => Trit::Tend,
        }).collect();
        TritMatrix { rows: self.shape[0], cols: self.shape[1], data }
    }

    /// Apply softmax along each row of a 2D tensor.
    pub fn softmax_rows(&self) -> Self {
        assert_eq!(self.ndim(), 2, "softmax_rows requires a 2D tensor");
        let (m, n) = (self.shape[0], self.shape[1]);
        let mut out = Self::zeros(&[m, n]);
        for row in 0..m {
            let slice = &self.data[row * n..(row + 1) * n];
            let sm    = TritFloat::softmax(slice);
            out.data[row * n..(row + 1) * n].copy_from_slice(&sm);
        }
        out
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f32, b: f32, tol: f32) -> bool {
        if b == 0.0 { return a.abs() < tol; }
        ((a - b) / b).abs() < tol
    }

    #[test]
    fn zeros_shape_and_values() {
        let t = TritFloatTensor::zeros(&[3, 4]);
        assert_eq!(t.shape(), &[3, 4]);
        assert_eq!(t.numel(), 12);
        assert!(t.data.iter().all(|x| x.is_zero()));
    }

    #[test]
    fn ones_values() {
        let t = TritFloatTensor::ones(&[2, 3]);
        for x in &t.data {
            assert!(approx(x.to_f32(), 1.0, 0.01));
            assert_eq!(x.phase(), 1);
        }
    }

    #[test]
    fn from_f32_slice_roundtrip() {
        let vals = vec![1.0f32, -2.0, 0.0, 3.14];
        let t = TritFloatTensor::from_f32_slice(&vals, &[2, 2]);
        assert_eq!(t.shape(), &[2, 2]);
        let back = t.to_f32_vec();
        for (a, b) in vals.iter().zip(back.iter()) {
            assert!(approx(*b, *a, 0.01), "{a} → {b}");
        }
    }

    #[test]
    fn from_tritmatrix_correct_values_and_confidence() {
        use crate::TritMatrix;
        use crate::Trit;
        let m = TritMatrix::from_trits(2, 2, vec![
            Trit::Affirm, Trit::Tend, Trit::Reject, Trit::Affirm,
        ]);
        let t = TritFloatTensor::from_tritmatrix(&m);
        assert_eq!(t.shape(), &[2, 2]);
        assert!(approx(t.get(&[0, 0]).to_f32(),  1.0, 0.01));
        assert!(t.get(&[0, 1]).is_zero());
        assert!(approx(t.get(&[1, 0]).to_f32(), -1.0, 0.01));
        // All weights are exactly known → confidence 1.0
        assert!(t.data.iter().all(|x| (x.confidence() - 1.0).abs() < 0.15));
    }

    #[test]
    fn matmul_identity() {
        // I × A = A
        let identity = TritFloatTensor::from_f32_slice(
            &[1.0f32, 0.0, 0.0, 1.0], &[2, 2]
        );
        let a = TritFloatTensor::from_f32_slice(
            &[3.0f32, 4.0, 5.0, 6.0], &[2, 2]
        );
        let r = TritFloatTensor::matmul(&identity, &a);
        let vals = r.to_f32_vec();
        assert!(approx(vals[0], 3.0, 0.02));
        assert!(approx(vals[1], 4.0, 0.02));
        assert!(approx(vals[2], 5.0, 0.02));
        assert!(approx(vals[3], 6.0, 0.02));
    }

    #[test]
    fn matmul_2x3_x_3x2() {
        // [[1,2,3],[4,5,6]] × [[7,8],[9,10],[11,12]] = [[58,64],[139,154]]
        let a = TritFloatTensor::from_f32_slice(
            &[1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]
        );
        let b = TritFloatTensor::from_f32_slice(
            &[7.0f32, 8.0, 9.0, 10.0, 11.0, 12.0], &[3, 2]
        );
        let r = TritFloatTensor::matmul(&a, &b);
        assert_eq!(r.shape(), &[2, 2]);
        let v = r.to_f32_vec();
        assert!(approx(v[0],  58.0, 0.02), "got {}", v[0]);
        assert!(approx(v[1],  64.0, 0.02), "got {}", v[1]);
        assert!(approx(v[2], 139.0, 0.02), "got {}", v[2]);
        assert!(approx(v[3], 154.0, 0.02), "got {}", v[3]);
    }

    #[test]
    fn matmul_confidence_propagates() {
        // Certain weights × uncertain activations → uncertain outputs
        let acts = TritFloatTensor::from_f32_with_confidence(
            &[1.0f32, 1.0], &[0.125f32, 0.125], &[1, 2]
        );
        let weights = TritFloatTensor::from_f32_slice(&[1.0f32, 0.0, 0.0, 1.0], &[2, 2]);
        let r = TritFloatTensor::matmul(&acts, &weights);
        assert!(r.min_confidence() < 0.3, "low-conf inputs → low-conf output");
    }

    #[test]
    fn matmul_sparse_skip_count() {
        // Half zeros in activations → roughly half skips
        let acts = TritFloatTensor::from_f32_slice(
            &[1.0f32, 0.0, 1.0, 0.0], &[1, 4]
        );
        let w = TritFloatTensor::from_f32_slice(
            &[1.0f32; 8], &[4, 2]
        );
        let (_, skips) = TritFloatTensor::matmul_sparse(&acts, &w);
        assert!(skips > 0, "zero activations should produce skips");
    }

    #[test]
    fn matmul_trit_matches_dense() {
        // [1, -1] × [[1, 0], [-1, 1]] = [1*1 + (-1)*(-1), 1*0 + (-1)*1] = [2, -1]
        use crate::TritMatrix;
        let acts = TritFloatTensor::from_f32_slice(&[1.0f32, -1.0], &[1, 2]);
        let mut w = TritMatrix::new(2, 2);
        w.set(0, 0, Trit::Affirm);   // (0,0) = +1
        w.set(0, 1, Trit::Tend);     // (0,1) = 0
        w.set(1, 0, Trit::Reject);   // (1,0) = -1
        w.set(1, 1, Trit::Affirm);   // (1,1) = +1

        let (r, _) = TritFloatTensor::matmul_trit(&acts, &w);
        assert_eq!(r.shape(), &[1, 2]);
        let v = r.to_f32_vec();
        // 1*1 + (-1)*(-1) = 1 + 1 = 2
        assert!(approx(v[0], 2.0, 0.02), "col0: expected 2, got {}", v[0]);
        // 1*0 + (-1)*1 = -1
        assert!(approx(v[1], -1.0, 0.02), "col1: expected -1, got {}", v[1]);
    }

    #[test]
    fn elementwise_add_and_mul() {
        let a = TritFloatTensor::from_f32_slice(&[1.0f32, 2.0, 3.0], &[3]);
        let b = TritFloatTensor::from_f32_slice(&[4.0f32, 5.0, 6.0], &[3]);
        let s = TritFloatTensor::add_elementwise(&a, &b);
        let p = TritFloatTensor::mul_elementwise(&a, &b);
        let sv = s.to_f32_vec();
        let pv = p.to_f32_vec();
        assert!(approx(sv[0], 5.0, 0.02));
        assert!(approx(sv[2], 9.0, 0.02));
        assert!(approx(pv[0], 4.0, 0.02));
        assert!(approx(pv[2], 18.0, 0.02));
    }

    #[test]
    fn map_applies_function() {
        let t = TritFloatTensor::from_f32_slice(&[1.0f32, 4.0, 9.0], &[3]);
        let r = t.map(|x| x.sqrt());
        let v = r.to_f32_vec();
        assert!(approx(v[0], 1.0, 0.02));
        assert!(approx(v[1], 2.0, 0.02));
        assert!(approx(v[2], 3.0, 0.02));
    }

    #[test]
    fn sparsity_correct() {
        // 2 zeros out of 4 = 50%
        let t = TritFloatTensor::from_f32_slice(&[1.0f32, 0.0, -1.0, 0.0], &[2, 2]);
        assert!((t.sparsity() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn confidence_histogram_bins() {
        let t = TritFloatTensor::from_f32_with_confidence(
            &[1.0f32, 1.0, 1.0],
            &[0.0f32, 0.5, 1.0],
            &[3],
        );
        let hist = t.confidence_histogram();
        assert_eq!(hist[0], 1, "one element at conf=0");
        assert_eq!(hist[4], 1, "one element at conf=0.5");
        assert_eq!(hist[8], 1, "one element at conf=1.0");
    }

    #[test]
    fn min_and_mean_confidence() {
        let t = TritFloatTensor::from_f32_with_confidence(
            &[1.0f32, 1.0],
            &[0.125f32, 1.0],
            &[2],
        );
        assert!((t.min_confidence() - 0.125).abs() < 0.15);
        let mean = t.mean_confidence();
        assert!(mean > 0.125 && mean < 1.0, "mean should be between min and max");
    }

    #[test]
    fn to_tritmatrix_roundtrip() {
        let t = TritFloatTensor::from_f32_slice(&[1.0f32, -1.0, 0.0, 0.5], &[2, 2]);
        let m = t.to_tritmatrix();
        assert_eq!(m.get(0, 0), Trit::Affirm);
        assert_eq!(m.get(0, 1), Trit::Reject);
        assert_eq!(m.get(1, 0), Trit::Tend);
        assert_eq!(m.get(1, 1), Trit::Affirm);
    }

    #[test]
    fn softmax_rows_sums_to_one() {
        let t = TritFloatTensor::from_f32_slice(
            &[1.0f32, 2.0, 3.0, 0.1, 0.2, 0.3], &[2, 3]
        );
        let sm = t.softmax_rows();
        for row in 0..2 {
            let row_sum: f32 = sm.data[row * 3..(row + 1) * 3]
                .iter().map(|x| x.to_f32()).sum();
            // TritFloat roundtrip precision ~0.3% per element; 3 elements → up to ~1% accumulated
            assert!((row_sum - 1.0).abs() < 0.005, "row {row} sum = {row_sum}");
        }
    }

    #[test]
    fn matmul_sparse_matches_matmul() {
        let a = TritFloatTensor::from_f32_slice(
            &[1.0f32, 0.0, 2.0, 0.0, 1.0, 3.0], &[2, 3]
        );
        let b = TritFloatTensor::from_f32_slice(
            &[1.0f32, 2.0, 0.0, 3.0, 4.0, 1.0], &[3, 2]
        );
        let r1 = TritFloatTensor::matmul(&a, &b);
        let (r2, _) = TritFloatTensor::matmul_sparse(&a, &b);
        for (x, y) in r1.to_f32_vec().iter().zip(r2.to_f32_vec().iter()) {
            assert!(approx(*x, *y, 0.001), "sparse and dense matmul disagree: {x} vs {y}");
        }
    }
}
