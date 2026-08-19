// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.

//! # Ternary AI Reasoning Toolkit (Phase 8)
//!
//! Core reasoning primitives that map f32 evidence → balanced-ternary decisions
//! and back. All types live here so `ternlang-api` and downstream MCP servers
//! can import them without depending on a specific VM backend.
//!
//! ## TritScalar — evidence → scalar trit
//! `TritScalar::from_raw(value, confidence)` takes a confidence-weighted f32 and maps it to a
//!   `Trit` (Affirm / Tend / Reject) via the `TEND_BOUNDARY` threshold.
//!
//! ## TritEvidenceVec — evidence bundle
//! A collection of labeled TritScalars with weighted dimensions.
//! `aggregate()` computes the consensus trit; `scalars()` exposes individual entries.
//!
//! ## DeliberationEngine — multi-round evidence convergence
//! Feeds evidence through N rounds of EMA smoothing until convergence.
//!
//! ## Coalition voting — multi-agent consensus
//! N coalition members vote; the trit with highest weighted confidence wins.
//!
//! ## Action gating — safety hard gate
//! An array of `GateDimension`s is scored; if **any** dimension is Reject
//! the gate is `Block`; if all are Tend it's `Hold`; otherwise `Proceed`.

use std::collections::HashMap;

use ternlang_core::Trit;

// ─── Constants ────────────────────────────────────────────────────────────────

/// Threshold below which evidence is classed as Tend (neutral).
/// Values with |v| > TEND_BOUNDARY map to Affirm/Reject.
pub const TEND_BOUNDARY: f64 = 0.15;

// ─── TritScalar ───────────────────────────────────────────────────────────────

/// A confidence-weighted ternary scalar — the output of reasoning over f32 evidence.
///
/// # API (ternlang-api compatible)
/// - `TritScalar::new(value)` / `TritScalar::from_raw(value, confidence)`
/// - `.trit()` — classified Trit (Affirm/Tend/Reject)
/// - `.confidence()` — confidence in [0.0, 1.0]
/// - `.raw()` — the raw evidence value
/// - `.is_actionable(min_confidence)` — true if confidence ≥ threshold AND trit != Tend
/// - `.trit_i8()` — trit as i8 (-1, 0, +1)
/// - `.label()` — human-readable label
#[derive(Debug, Clone, PartialEq)]
pub struct TritScalar {
    /// Raw evidence value (f32).
    raw_value: f32,
    /// Classified trit.
    trit_value: Trit,
    /// Confidence in [0, 1].
    conf: f32,
}

impl TritScalar {
    /// Classify `value` into a `Trit` using `TEND_BOUNDARY`.
    /// `confidence` is |value| clamped to [0, 1].
    pub fn new(value: f32) -> Self {
        let trit = classify_trit(value as f64);
        let confidence = ((value as f64).abs()).min(1.0) as f32;
        Self {
            raw_value: value,
            trit_value: trit,
            conf: confidence,
        }
    }

    /// Explicit constructor with raw value + confidence.
    pub fn from_raw(value: f32, confidence: f32) -> Self {
        let trit = classify_trit(value as f64);
        Self {
            raw_value: value,
            trit_value: trit,
            conf: confidence.min(1.0).max(0.0),
        }
    }

    /// The classified `Trit`.
    pub fn trit(&self) -> Trit {
        self.trit_value
    }

    /// Confidence in [0.0, 1.0].
    pub fn confidence(&self) -> f32 {
        self.conf
    }

    /// The raw evidence value.
    pub fn raw(&self) -> f32 {
        self.raw_value
    }

    /// Trit as i8 (-1, 0, +1).
    pub fn trit_i8(&self) -> i8 {
        self.trit_value as i8
    }

    /// Human-readable label: "affirm" / "tend" / "reject".
    pub fn label(&self) -> &'static str {
        match self.trit_value {
            Trit::Affirm => "affirm",
            Trit::Tend => "tend",
            Trit::Reject => "reject",
        }
    }

    /// True if confidence ≥ min_confidence AND trit is not Tend (i.e. a definitive
    /// affirm or reject, not undecided).
    pub fn is_actionable(&self, min_confidence: f32) -> bool {
        self.conf >= min_confidence && self.trit_value != Trit::Tend
    }
}

/// Classify an f64 evidence value into a `Trit` using `TEND_BOUNDARY`.
pub fn classify_trit(v: f64) -> Trit {
    if v > TEND_BOUNDARY {
        Trit::Affirm
    } else if v < -TEND_BOUNDARY {
        Trit::Reject
    } else {
        Trit::Tend
    }
}

impl From<TritScalar> for f64 {
    fn from(s: TritScalar) -> Self {
        s.trit_value as i8 as f64
    }
}

/// Convert a `Trit` to its f32 representation (-1.0, 0.0, +1.0).
pub fn trit_to_f32(t: Trit) -> f32 {
    t as i8 as f32
}

/// Convert a `Trit` to its f64 representation (-1.0, 0.0, +1.0).
pub fn trit_to_f64(t: Trit) -> f64 {
    t as i8 as f64
}

// ─── TritEvidenceVec ──────────────────────────────────────────────────────────

/// A vector of classified TritScalar evidence with weighted gate dimensions.
#[derive(Debug, Clone)]
pub struct TritEvidenceVec {
    /// Individual scalar evidence entries.
    pub scalars: Vec<TritScalar>,
    /// Gate dimensions derived from evidence.
    pub dimensions: Vec<GateDimension>,
    /// Weights for each evidence entry.
    pub weights: Vec<f32>,
}

impl TritEvidenceVec {
    /// Create from labels, raw values, and weights.
    /// Each value is classified into a TritScalar; weights determine
    /// per-dimension influence in the aggregate.
    pub fn new(labels: Vec<String>, values: Vec<f32>, weights: Vec<f32>) -> Self {
        assert_eq!(
            labels.len(),
            values.len(),
            "labels and values must have equal length"
        );
        assert_eq!(
            labels.len(),
            weights.len(),
            "labels and weights must have equal length"
        );

        let scalars: Vec<TritScalar> = values
            .iter()
            .zip(weights.iter())
            .map(|(&v, &w)| TritScalar::from_raw(v, w.min(1.0).max(0.0)))
            .collect();

        let dimensions: Vec<GateDimension> = labels
            .iter()
            .zip(values.iter())
            .zip(weights.iter())
            .map(|((label, &v), &w)| GateDimension::new(label.clone(), v, w))
            .collect();

        Self {
            scalars,
            dimensions,
            weights,
        }
    }

    /// Aggregate all evidence into a single consensus TritScalar.
    ///
    /// The aggregate trit is the weighted majority trit across all entries;
    /// the aggregate confidence is the weighted mean confidence.
    pub fn aggregate(&self) -> TritScalar {
        if self.scalars.is_empty() {
            return TritScalar::new(0.0);
        }

        // Compute weighted aggregate
        let total_weight: f32 = self.weights.iter().sum();
        let weighted_value: f32 = self
            .scalars
            .iter()
            .zip(self.weights.iter())
            .map(|(s, w)| s.raw() * w)
            .sum::<f32>()
            / total_weight.max(0.001);

        let weighted_conf: f32 = self
            .scalars
            .iter()
            .zip(self.weights.iter())
            .map(|(s, w)| s.confidence() * w)
            .sum::<f32>()
            / total_weight.max(0.001);

        TritScalar::from_raw(weighted_value, weighted_conf)
    }

    /// Exposes the individual scalar evidence entries.
    pub fn scalars(&self) -> &[TritScalar] {
        &self.scalars
    }

    /// Weighted consensus trit — the trit with the highest cumulative weight.
    pub fn consensus(&self) -> Trit {
        let mut scores: HashMap<Trit, f32> = HashMap::new();
        for (s, &w) in self.scalars.iter().zip(&self.weights) {
            *scores.entry(s.trit()).or_insert(0.0) += w;
        }
        let affirm = scores.get(&Trit::Affirm).copied().unwrap_or(0.0);
        let tend = scores.get(&Trit::Tend).copied().unwrap_or(0.0);
        let reject = scores.get(&Trit::Reject).copied().unwrap_or(0.0);
        if reject >= affirm && reject >= tend && reject > 0.0 {
            Trit::Reject
        } else if affirm >= tend && affirm >= reject && affirm > 0.0 {
            Trit::Affirm
        } else {
            Trit::Tend
        }
    }
}

// ─── DeliberationEngine ───────────────────────────────────────────────────────

/// Multi-round evidence convergence engine using EMA smoothing.
///
/// ```
/// use ternlang_ml::{DeliberationEngine, TritScalar};
/// let mut engine = DeliberationEngine::new(0.7, 5);
/// let result = engine.run(&[TritScalar::new(0.2), TritScalar::new(0.6)]);
/// ```
#[derive(Debug, Clone)]
pub struct DeliberationEngine {
    target_confidence: f32,
    max_rounds: usize,
    alpha: f32,
}

impl DeliberationEngine {
    /// `target_confidence` — convergence threshold for the scalar's confidence.
    /// `max_rounds` — maximum number of EMA smoothing rounds.
    pub fn new(target_confidence: f32, max_rounds: usize) -> Self {
        Self {
            target_confidence,
            max_rounds,
            alpha: 0.3,
        }
    }

    /// Set the EMA smoothing factor (0 < alpha ≤ 1).
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Run convergence over evidence — returns the final `TritScalar`.
    pub fn run(&self, evidence: &[TritScalar]) -> TritScalar {
        if evidence.is_empty() {
            return TritScalar::new(0.0);
        }

        // Weighted mean of input values
        let total_weight: f32 = evidence.iter().map(|e| e.confidence().max(0.01)).sum();
        let mut current: f32 = evidence
            .iter()
            .map(|e| e.raw() * e.confidence().max(0.01))
            .sum::<f32>()
            / total_weight.max(0.001);

        for round in 0..self.max_rounds {
            let scalar = TritScalar::new(current);
            if scalar.confidence() >= self.target_confidence {
                return scalar;
            }
            current = self.alpha * current;
            if round == self.max_rounds - 1 {
                return TritScalar::new(current);
            }
        }
        TritScalar::new(current)
    }

    /// Streaming variant — one round at a time for SSE.
    pub fn step(&self, current: f32) -> (f32, usize, bool) {
        let scalar = TritScalar::new(current);
        let converged = scalar.confidence() >= self.target_confidence;
        (self.alpha * current, 1, converged)
    }
}

// ─── Coalition voting ─────────────────────────────────────────────────────────

/// A coalition member — one agent in the deliberation.
#[derive(Debug, Clone)]
pub struct CoalitionMember {
    pub label: String,
    pub trit: Trit,
    pub confidence: f32,
    pub weight: f32,
}

impl CoalitionMember {
    pub fn new(label: impl Into<String>, trit: Trit, confidence: f32, weight: f32) -> Self {
        Self {
            label: label.into(),
            trit,
            confidence: confidence.min(1.0).max(0.0),
            weight: weight.max(0.0),
        }
    }
}

/// Weighted consensus across coalition members.
/// Returns the trit with the highest weighted confidence.
pub fn coalition_vote(members: &[CoalitionMember]) -> Trit {
    let mut scores: HashMap<Trit, f32> = HashMap::new();
    for m in members {
        *scores.entry(m.trit).or_insert(0.0) += m.confidence * m.weight;
    }
    let affirm = scores.get(&Trit::Affirm).copied().unwrap_or(0.0);
    let tend = scores.get(&Trit::Tend).copied().unwrap_or(0.0);
    let reject = scores.get(&Trit::Reject).copied().unwrap_or(0.0);
    if reject >= affirm && reject >= tend && reject > 0.0 {
        Trit::Reject
    } else if affirm >= tend && affirm >= reject && affirm > 0.0 {
        Trit::Affirm
    } else {
        Trit::Tend
    }
}

// ─── Action gating ────────────────────────────────────────────────────────────

/// A dimension of evidence for the action gate.
#[derive(Debug, Clone)]
pub struct GateDimension {
    pub name: String,
    pub evidence: f32,
    pub weight: f32,
}

impl GateDimension {
    pub fn new(name: impl Into<String>, evidence: f32, weight: f32) -> Self {
        Self {
            name: name.into(),
            evidence,
            weight: weight.max(0.0),
        }
    }
}

/// Verdict from `action_gate` — whether a proposed action is safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateVerdict {
    /// All dimensions clear — proceed.
    Proceed,
    /// One or more dimensions blocked — halt.
    Block,
    /// All tend — insufficient evidence, hold for more info.
    Hold,
}

/// Evaluate whether an action should proceed based on gate dimensions.
///
/// - If **any** dimension is `Reject` → `Block` (hard veto)
/// - If **all** dimensions are `Tend` → `Hold` (insufficient evidence)
/// - Otherwise → `Proceed`
pub fn action_gate(dimensions: &[GateDimension]) -> GateVerdict {
    if dimensions.is_empty() {
        return GateVerdict::Hold;
    }

    let mut has_affirm = false;
    for dim in dimensions {
        let t = classify_trit(dim.evidence as f64);
        match t {
            Trit::Reject => return GateVerdict::Block,
            Trit::Affirm => has_affirm = true,
            Trit::Tend => {}
        }
    }

    if has_affirm {
        GateVerdict::Proceed
    } else {
        GateVerdict::Hold
    }
}

// ─── Scalar temperature & hallucination ───────────────────────────────────────

/// Convert a `TritScalar` into an LLM sampling temperature.
/// Maps trit → temperature: Affirm → 0.15, Tend → 0.75, Reject → 1.2.
pub fn scalar_temperature(scalar: &TritScalar) -> f32 {
    match scalar.trit() {
        Trit::Affirm => 0.15, // low temp — confident, conservative
        Trit::Tend => 0.75,   // medium temp — exploratory
        Trit::Reject => 1.2,  // high temp — re-evaluate / re-prompt
    }
}

/// Compute a hallucination score from a set of signal variances.
/// Returns 0.0 (no hallucination) to 1.0 (high suspicion).
///
/// High variance across signal values → higher hallucination score.
/// Values within `TEND_BOUNDARY` of each other → low score.
pub fn hallucination_score(signals: &[f32]) -> f32 {
    if signals.len() < 2 {
        return 0.0;
    }
    let mean: f32 = signals.iter().sum::<f32>() / signals.len() as f32;
    let variance: f32 =
        signals.iter().map(|&s| (s - mean).powi(2)).sum::<f32>() / signals.len() as f32;
    let std_dev = variance.sqrt();
    (std_dev / 0.5).min(1.0)
}

// ─── Benchmark ────────────────────────────────────────────────────────────────

/// Sparse vs dense matmul benchmark results.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub sparse_time_ms: f32,
    pub dense_time_ms: f32,
    pub speedup: f32,
    pub sparsity: f32,
}

/// Run a micro-benchmark comparing sparse vs dense ternary matmul.
pub fn benchmark(rows: usize, cols: usize, sparsity_target: f32) -> BenchmarkResult {
    use std::time::Instant;

    let input = vec![Trit::Affirm; cols];
    let weight_trits: Vec<Trit> = (0..rows * cols)
        .map(|i| {
            if (i as f32 / (rows * cols) as f32) < sparsity_target {
                Trit::Tend
            } else if (i % 3) == 0 {
                Trit::Reject
            } else {
                Trit::Affirm
            }
        })
        .collect();

    let input_mat = super::TritMatrix::new(1, cols, input.clone());
    let weight_mat = super::TritMatrix::new(rows, cols, weight_trits.clone());

    let start = Instant::now();
    for _ in 0..100 {
        let _ = super::dense_matmul(&input_mat, &weight_mat);
    }
    let dense_time = start.elapsed().as_secs_f32() / 100.0 * 1000.0;

    let start = Instant::now();
    for _ in 0..100 {
        let _ = super::sparse_matmul(&input_mat, &weight_mat);
    }
    let sparse_time = start.elapsed().as_secs_f32() / 100.0 * 1000.0;

    let nnz = weight_trits
        .iter()
        .filter(|&&t| t != Trit::Tend)
        .count() as f32;
    let total = weight_trits.len() as f32;

    BenchmarkResult {
        sparse_time_ms: sparse_time,
        dense_time_ms: dense_time,
        speedup: dense_time / sparse_time.max(0.001),
        sparsity: 1.0 - (nnz / total).max(0.0),
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tritscalar_classification() {
        assert_eq!(TritScalar::new(0.5).trit(), Trit::Affirm);
        assert_eq!(TritScalar::new(-0.5).trit(), Trit::Reject);
        assert_eq!(TritScalar::new(0.1).trit(), Trit::Tend);
    }

    #[test]
    fn tritscalar_methods() {
        let s = TritScalar::new(0.5);
        assert!(s.is_actionable(0.5)); // conf is 0.5, exactly at threshold → actionable (>=)
        assert!(!s.is_actionable(0.5 + f32::EPSILON)); // just above → not actionable
        assert_eq!(s.trit_i8(), 1);
        assert!((s.raw() - 0.5).abs() < 0.001);
    }

    #[test]
    fn coalition_vote_picks_winner() {
        let members = vec![
            CoalitionMember::new("a", Trit::Affirm, 0.9, 1.0),
            CoalitionMember::new("b", Trit::Reject, 0.3, 0.5),
            CoalitionMember::new("c", Trit::Tend, 0.5, 1.0),
        ];
        assert_eq!(coalition_vote(&members), Trit::Affirm);
    }

    #[test]
    fn action_gate_honors_reject_veto() {
        let dims = vec![
            GateDimension::new("safety", 0.8, 1.0),
            GateDimension::new("spot", -0.9, 1.0),
        ];
        assert_eq!(action_gate(&dims), GateVerdict::Block);
    }

    #[test]
    fn action_gate_proceeds_after_tend() {
        let dims = vec![GateDimension::new("safety", 0.0, 1.0)];
        assert_eq!(action_gate(&dims), GateVerdict::Hold);
    }

    #[test]
    fn evidence_vec_aggregate() {
        let ev = TritEvidenceVec::new(
            vec!["a".into(), "b".into()],
            vec![0.8, 0.2],
            vec![1.0, 1.0],
        );
        let agg = ev.aggregate();
        assert_eq!(agg.trit(), Trit::Affirm);
    }

    #[test]
    fn hallucination_score_increases_with_variance() {
        let low = hallucination_score(&[1.0, 1.01, 0.99, 1.0]);
        let high = hallucination_score(&[0.0, 2.0, -1.0, 1.5]);
        assert!(high > low);
    }
}
