// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.

//! The atomic unit of episodic memory: a time-stamped, embedded event.

use serde::{Deserialize, Serialize};
use ternlang_ml::TritFloat;

/// Episode identifier — **the creation timestamp itself** (unix milliseconds),
/// bumped by 1ms on collision so it stays unique and strictly monotonic.
///
/// The id *is* the time: a larger id always happened later, and the difference
/// between two ids is the elapsed time between those memories. Anchored to the
/// fixed unix epoch (1970-01-01), so "yesterday" and "two weeks to the deadline"
/// fall straight out of arithmetic on the id.
pub type EpisodeId = u64;

/// A single autobiographical event held in episodic memory.
///
/// The embedding is stored as `Vec<TritFloat>` (T-FLOAT32). Each component
/// carries the episode's **salience** in its native confidence field, so recall
/// propagates an in-band epistemic certainty for free — no separate uncertainty
/// model. This is the structural difference from a plain RAG vector store.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Episode {
    /// Monotonic id; also the episodic ordering key.
    pub id: EpisodeId,
    /// Event time, unix milliseconds. Supplied by the caller (deterministic core).
    pub t_ms: i64,
    /// Human-readable content of the memory.
    pub content: String,
    /// Free-form tags for filtered recall and timelines.
    pub tags: Vec<String>,
    /// Who/what laid down the memory (agent name, sensor, user, …).
    pub source: String,
    /// Salience / vividness in `[0, 1]`. Drives the importance term in recall and
    /// seeds the TritFloat confidence field of the embedding.
    pub importance: f32,
    /// How many times this episode has been recalled (reinforcement signal).
    pub access_count: u32,
    /// Last time this episode was recalled, unix ms.
    pub last_access_ms: i64,
    /// Associative links to other episodes (consolidation, causal chains).
    pub links: Vec<EpisodeId>,
    /// Ternary-quantized embedding; confidence field encodes salience.
    pub embedding: Vec<TritFloat>,
    /// L2 norm of the original f32 embedding, kept for cosine normalization.
    pub norm: f32,
}

impl Episode {
    /// Quantize a (preferably L2-normalized) f32 embedding into TritFloats,
    /// stamping each component with `importance` as its confidence.
    pub(crate) fn quantize(embedding: &[f32], importance: f32) -> Vec<TritFloat> {
        embedding
            .iter()
            .map(|&x| TritFloat::from_f32_with_confidence(x, importance))
            .collect()
    }

    /// Number of non-zero (non-skippable) components — drives @sparseskip savings.
    pub fn nnz(&self) -> usize {
        self.embedding.iter().filter(|t| !t.is_zero()).count()
    }

    /// ISO 8601 UTC time of the event, e.g. `2026-06-25T05:43:11Z`.
    pub fn iso(&self) -> String {
        crate::time::unix_ms_to_iso(self.t_ms)
    }

    /// Canonical `Weekday/ISO8601Z/epoch` stamp (matches the `iso` tool).
    pub fn stamp(&self) -> String {
        crate::time::unix_ms_to_stamp(self.t_ms)
    }
}
