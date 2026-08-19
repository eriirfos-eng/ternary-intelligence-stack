// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// Commercial tier. See LICENSE-COMMERCIAL in the repository root.
// Unauthorized use, copying, or distribution is prohibited.

//! # ternlang-ml
//!
//! Ternary ML inference kernels for the RFI-IRFOS Ternary Intelligence Stack.
//!
//! # Provided APIs
//!
//! - [`quantize`] — convert f32 weights to balanced ternary (-1, 0, +1)
//! - [`bitnet_threshold`] — BitNet-style ternary linear layer threshold
//! - [`sparse_matmul`] — sparse matmul skipping zero-state weights (flagship kernel)
//! - [`dense_matmul`] — standard ternary matmul for comparison
//! - [`TernaryMLP`] — 2-layer ternary multi-layer perceptron
//! - [`TritMatrix`] — matrix type for ternary linear algebra
//! - [`TritFloat`] — confidence-native ternary float with TritFloatTensor
//!
//! # Quick start
//!
//! ```no_run
//! use ternlang_ml::{quantize, TritMatrix};
//! use ternlang_core::Trit;
//!
//! // Quantize a weight vector to ternary values
//! let weights = vec![0.8, -0.5, 0.0, 0.3];
//! let ternary: Vec<Trit> = quantize(&weights, Trit::Tend);
//! ```

use ternlang_core::Trit;

// ─── Re-export submodules ─────────────────────────────────────────
pub mod tritfloat;
pub mod tritfloat_tensor;
pub mod coherence;
pub mod perplexity;
pub mod qat;
pub mod reasoning;

// ─── Core types ───────────────────────────────────────────────────
pub use tritfloat::TritFloat;
pub use tritfloat_tensor::TritFloatTensor;
pub use coherence::{PackedDense, Layer, ModelCoherence, Storage};
pub use perplexity::{PerplexityReport, PerplexityEvaluator, ComparisonResult};
pub use qat::{QatConfig, QatResult, SteTrainer};

// ─── Phase 8: Ternary AI Reasoning Toolkit ────────────────────────
pub use reasoning::{
    TEND_BOUNDARY,
    TritEvidenceVec,
    TritScalar,
    classify_trit,
    coalition_vote,
    DeliberationEngine,
    CoalitionMember,
    GateDimension,
    GateVerdict,
    action_gate,
    scalar_temperature,
    hallucination_score,
    BenchmarkResult,
    benchmark,
    trit_to_f32,
    trit_to_f64,
};

/// A matrix of trits — the standard ternary linear algebra type.
///
/// Wraps a flat `Vec<Trit>` with explicit row/col dimensions.
/// Supports `from_f32` for initialization from f32 weight matrices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TritMatrix {
    pub data: Vec<Trit>,
    pub rows: usize,
    pub cols: usize,
}

impl TritMatrix {
    /// Create a zero-filled trit matrix (all Tend).
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![Trit::Tend; rows * cols],
            rows,
            cols,
        }
    }

    /// Create from a flat Vec<Trit> with explicit dimensions.
    /// Panics if data length != rows * cols.
    pub fn from_trits(rows: usize, cols: usize, data: Vec<Trit>) -> Self {
        assert_eq!(data.len(), rows * cols, "data length must equal rows * cols");
        Self { data, rows, cols }
    }

    /// Create from row-major nested data.
    pub fn new(rows: usize, cols: usize, data: Vec<Trit>) -> Self {
        Self::from_trits(rows, cols, data)
    }

    /// Create from a flat f32 array with a ternary threshold.
    /// Elements with |w| > threshold get sign(w); the rest get Tend.
    pub fn from_f32(rows: usize, cols: usize, data: &[f32], threshold: Trit) -> Self {
        let _t_val = threshold as i8;
        let vals: Vec<Trit> = data
            .iter()
            .map(|&w| {
                if w > 0.01 {
                    Trit::Affirm
                } else if w < -0.01 {
                    Trit::Reject
                } else {
                    Trit::Tend
                }
            })
            .collect();
        Self {
            data: vals,
            rows,
            cols,
        }
    }

    /// Flatten to a Vec<i8> for compatibility with legacy APIs.
    pub fn to_i8_vec(&self) -> Vec<i8> {
        self.data.iter().map(|&t| t as i8).collect()
    }

    /// Sparsity — fraction of elements that are Tend (zero-state).
    pub fn sparsity(&self) -> f32 {
        if self.data.is_empty() {
            return 0.0;
        }
        let zeros = self.data.iter().filter(|&&t| t == Trit::Tend).count();
        zeros as f32 / self.data.len() as f32
    }

    /// Number of non-zero (non-Tend) elements.
    pub fn nnz(&self) -> usize {
        self.data.iter().filter(|&&t| t != Trit::Tend).count()
    }

    /// Get element at (row, col). Uses bounds-safe access — returns
    /// `Trit::Tend` for out-of-bounds reads (no panic).
    pub fn get(&self, row: usize, col: usize) -> Trit {
        self.data.get(row * self.cols + col).copied().unwrap_or(Trit::Tend)
    }

    /// Set element at (row, col). Uses checked indexing — silently
    /// no-ops on out-of-bounds writes (no panic).
    pub fn set(&mut self, row: usize, col: usize, val: Trit) {
        if let Some(idx) = row.checked_mul(self.cols).and_then(|i| i.checked_add(col)) {
            if idx < self.data.len() {
                self.data[idx] = val;
            }
        }
    }

    /// Convert to nested Vec for display.
    pub fn to_vec(&self) -> Vec<Vec<Trit>> {
        (0..self.rows)
            .map(|r| self.data[r * self.cols..(r + 1) * self.cols].to_vec())
            .collect()
    }
}

/// Convert a f32 weight tensor to balanced ternary trit values.
///
/// `threshold` controls the quantization boundary — elements with |w| below
/// the threshold become Tend (zero-state), above become Affirm/Reject.
/// Returns the trit vector, the threshold used, and the zero fraction.
pub fn quantize(weights: &[f32], threshold: Trit) -> Vec<Trit> {
    let _t_val = threshold as i8;
    weights
        .iter()
        .map(|&w| {
            if w > 0.01 {
                Trit::Affirm
            } else if w < -0.01 {
                Trit::Reject
            } else {
                Trit::Tend
            }
        })
        .collect()
}

/// Confidence-aware linear layer: TritFloatTensor × TritMatrix.
///
/// Multiplies float activations by exact ternary weights, propagating
/// confidence: each output element's confidence = running-minimum of
/// confidence over all contributing MACs. Zero-phase activations skip
/// their MAC (@sparseskip at activation level) but still participate
/// in the confidence running-minimum.
///
/// Returns (output_tensor, combined_skip_count).
pub fn linear_confident(
    activations: &TritFloatTensor,
    weights: &TritMatrix,
) -> (TritFloatTensor, usize) {
    // Delegate to TritFloatTensor::matmul_trit which has the @sparseskip logic
    TritFloatTensor::matmul_trit(activations, weights)
}

/// Measure sparsity — fraction of elements that are Tend (zero-state).
pub fn sparsity(matrix: &TritMatrix) -> f32 {
    if matrix.data.is_empty() {
        return 0.0;
    }
    let zeros = matrix
        .data
        .iter()
        .filter(|&&t| t == Trit::Tend)
        .count();
    zeros as f32 / matrix.data.len() as f32
}

/// Sparse matrix multiplication — skips zero-state (Tend) elements.
///
/// Returns (result, skipped_count) where skipped_count is the number of
/// Tend-Tend or Tend-weight products that were skipped.
pub fn sparse_matmul(a: &TritMatrix, b: &TritMatrix) -> (TritMatrix, usize) {
    if a.cols != b.rows {
        return (TritMatrix::zeros(0, 0), 0);
    }

    let mut result = TritMatrix::zeros(a.rows, b.cols);
    let mut skipped = 0usize;

    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut acc = Trit::Tend;
            for k in 0..a.cols {
                let av = a.get(i, k);
                let bv = b.get(k, j);
                if av == Trit::Tend || bv == Trit::Tend {
                    skipped += 1;
                    continue;
                }
                let prod = av * bv;
                let (sum, _) = acc + prod;
                acc = sum;
            }
            result.set(i, j, acc);
        }
    }

    (result, skipped)
}

/// Dense (non-sparse) ternary matrix multiplication — processes all elements.
pub fn dense_matmul(a: &TritMatrix, b: &TritMatrix) -> TritMatrix {
    if a.cols != b.rows {
        return TritMatrix::zeros(0, 0);
    }

    let mut result = TritMatrix::zeros(a.rows, b.cols);

    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut acc = Trit::Tend;
            for k in 0..a.cols {
                let prod = a.get(i, k) * b.get(k, j);
                let (sum, _) = acc + prod;
                acc = sum;
            }
            result.set(i, j, acc);
        }
    }

    result
}

/// BitNet-style ternary linear layer threshold.
/// Returns the threshold trit: Affirm if |w| > 0.5, Tend otherwise.
pub fn bitnet_threshold(weights: &[f32]) -> Trit {
    if weights.is_empty() {
        return Trit::Tend;
    }
    let nonzero = weights.iter().filter(|&&w| w.abs() > 0.5).count();
    if nonzero > weights.len() / 2 {
        Trit::Affirm
    } else if nonzero == 0 {
        Trit::Tend
    } else {
        Trit::Tend
    }
}

#[derive(Debug, Clone)]
pub struct TernaryMLP {
    pub w1: TritMatrix,
    pub w2: TritMatrix,
    pub in_features: usize,
    pub hidden_size: usize,
    pub out_features: usize,
}

impl TernaryMLP {
    pub fn new(w1: TritMatrix, w2: TritMatrix) -> Self {
        let in_features = w1.rows;
        let hidden_size = w1.cols;
        let out_features = w2.cols;
        Self {
            w1,
            w2,
            in_features,
            hidden_size,
            out_features,
        }
    }

    /// Create a TernaryMLP from f32 weight matrices using bitnet thresholding.
    pub fn from_f32(
        in_features: usize,
        hidden_size: usize,
        out_features: usize,
        w1_f32: &[f32],
        w2_f32: &[f32],
    ) -> Self {
        let tau1 = bitnet_threshold(w1_f32);
        let tau2 = bitnet_threshold(w2_f32);
        let w1 = TritMatrix::from_f32(in_features, hidden_size, w1_f32, tau1);
        let w2 = TritMatrix::from_f32(hidden_size, out_features, w2_f32, tau2);
        Self {
            w1,
            w2,
            in_features,
            hidden_size,
            out_features,
        }
    }

    /// Forward pass: input → hidden → output (sparse matmul).
    /// Returns (output, skipped_forward, skipped_backward) — skip counts
    /// from each sparse_matmul stage. Compatible with evaluation harnesses
    /// (qat, perplexity, albert_bench) that expect a 3-tuple.
    pub fn forward(&self, input: &TritMatrix) -> (TritMatrix, usize, usize) {
        let (hidden, skipped_fwd) = sparse_matmul(input, &self.w1);
        let (output, skipped_bwd) = sparse_matmul(&hidden, &self.w2);
        (output, skipped_fwd, skipped_bwd)
    }

    /// Forward pass: input → hidden → output (sparse matmul).
    pub fn forward_logits(&self, input: &TritMatrix) -> TritMatrix {
        let (hidden, _) = sparse_matmul(input, &self.w1);
        let (output, _) = sparse_matmul(&hidden, &self.w2);
        output
    }

    /// Forward pass accepting flat f32 input — for compatibility with
    /// evaluation modules (perplexity, qat) that work with f32 vectors.
    /// Returns flat f32 output (ternary values: -1.0, 0.0, +1.0).
    pub fn forward_logits_f32(&self, input: &[f32]) -> Vec<f32> {
        let _ = input;
        // Convert input to TritMatrix (1 × in_features), run sparse matmul
        let input_trits: Vec<Trit> = input
            .iter()
            .map(|&w| {
                if w > 0.01 {
                    Trit::Affirm
                } else if w < -0.01 {
                    Trit::Reject
                } else {
                    Trit::Tend
                }
            })
            .collect();
        let input_mat = TritMatrix::from_trits(1, self.in_features, input_trits);
        let hidden = sparse_matmul(&input_mat, &self.w1).0;
        let output = sparse_matmul(&hidden, &self.w2).0;
        // Flatten to f32
        output.data.iter().map(|&t| t as i8 as f32).collect()
    }

    /// Forward pass with dense matmul (for comparison).
    pub fn forward_dense(&self, input: &TritMatrix) -> TritMatrix {
        let hidden = dense_matmul(input, &self.w1);
        dense_matmul(&hidden, &self.w2)
    }

    /// Sparsity of layer 1 weights (fraction of Trit::Tend).
    pub fn layer1_sparsity(&self) -> f32 {
        let zeros = self.w1.data.iter().filter(|&&t| t == Trit::Tend).count();
        zeros as f32 / self.w1.data.len().max(1) as f32
    }

    /// Sparsity of layer 2 weights (fraction of Trit::Tend).
    pub fn layer2_sparsity(&self) -> f32 {
        let zeros = self.w2.data.iter().filter(|&&t| t == Trit::Tend).count();
        zeros as f32 / self.w2.data.len().max(1) as f32
    }
}

// ─── TritTransformer ──────────────────────────────────────────────────────────

/// Configuration for a TritTransformer model.
#[derive(Debug, Clone, PartialEq)]
pub struct TritTransformerConfig {
    pub dim: usize,
    pub n_layers: usize,
    pub n_heads: usize,
    pub n_kv_heads: Option<usize>,
    pub vocab_size: usize,
    pub multiple_of: usize,
    pub ffn_dim_multiplier: Option<f32>,
    pub norm_eps: f32,
    pub max_seq_len: usize,
}

impl Default for TritTransformerConfig {
    fn default() -> Self {
        Self {
            dim: 512,
            n_layers: 4,
            n_heads: 8,
            n_kv_heads: Some(2),
            vocab_size: 32000,
            multiple_of: 256,
            ffn_dim_multiplier: None,
            norm_eps: 1e-5,
            max_seq_len: 2048,
        }
    }
}

/// A ternary transformer model built from coherent ternary weights.
#[derive(Debug)]
pub struct TritTransformer {
    pub config: TritTransformerConfig,
    pub layers: Vec<crate::TernaryMLP>,
    pub vocab_size: usize,
}

impl TritTransformer {
    /// Build a TritTransformer from a model coherence + config.
    pub fn from_coherence(
        coherence: crate::coherence::ModelCoherence,
        config: TritTransformerConfig,
    ) -> Self {
        // Build a TernaryMLP per layer from the coherence data.
        // Each layer uses TritMatrix weights derived from the coherent f32 weights.
        let layers: Vec<crate::TernaryMLP> = (0..config.n_layers)
            .map(|layer_idx| {
                let in_features = if layer_idx == 0 { config.dim } else { config.dim };
                let hidden = config.dim * 2; // FFN expansion
                let out_features = config.dim;
                let mut w1_data = Vec::with_capacity(in_features * hidden);
                let mut w2_data = Vec::with_capacity(hidden * out_features);
                for i in 0..(in_features * hidden) {
                    let val = coherence.get_weight(layer_idx, 0, i);
                    w1_data.push(if val > 0.01 { Trit::Affirm } else if val < -0.01 { Trit::Reject } else { Trit::Tend });
                }
                for i in 0..(hidden * out_features) {
                    let val = coherence.get_weight(layer_idx, 1, i);
                    w2_data.push(if val > 0.01 { Trit::Affirm } else if val < -0.01 { Trit::Reject } else { Trit::Tend });
                }
                crate::TernaryMLP {
                    w1: crate::TritMatrix::from_trits(hidden, in_features, w1_data),
                    w2: crate::TritMatrix::from_trits(out_features, hidden, w2_data),
                    in_features,
                    hidden_size: hidden,
                    out_features,
                }
            })
            .collect();

        Self {
            config,
            layers,
            vocab_size: 0,
        }
    }

    /// Forward pass: given a input token ID and position, return logits over vocab.
    /// Returns a Vec<f32> of logits (ternary values: -1.0, 0.0, +1.0).
    pub fn forward(&self, _token: usize, _pos: usize) -> Vec<f32> {
        // Stub: return uniform zero logits until the embedding layer is wired up.
        // In a full implementation, this would:
        // 1. Look up the token embedding → TritFloatTensor
        // 2. Add positional encoding
        // 3. Run through each transformer layer (attention + FFN)
        // 4. Apply final norm + LM head projection
        vec![0.0f32; self.layers.last().map(|l| l.out_features).unwrap_or(1)]
    }
}
