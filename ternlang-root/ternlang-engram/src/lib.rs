// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.

//! # ternlang-engram — Ternary Episodic Memory
//!
//! Persistent, autobiographical memory for AI agents, built on the Ternary
//! Intelligence Stack. Where a plain vector store answers *"what is similar?"*,
//! an episodic store answers *"what happened, when, and how much did it matter?"*.
//!
//! ## What makes it episodic (vs. a RAG vector DB)
//! - **Temporal cognition.** Every memory is anchored to a unix-epoch timestamp
//!   and rendered as a `Weekday/ISO8601Z/epoch` stamp (matching the `iso` tool);
//!   recall blends relevance with **recency** and reports a human age
//!   (`"3d 4h ago"`), and [`EngramStore::timeline`] slices history. See [`time`].
//! - **Composite recall.** Ranking follows human episodic retrieval —
//!   `relevance × recency × salience × frequency` — and recall **reinforces**
//!   the episodes it returns.
//! - **Native confidence.** Embeddings are stored as
//!   [`TritFloat`](ternlang_ml::TritFloat) (T-FLOAT32). Each component carries the
//!   episode's salience in its confidence field, so every [`Recall`] arrives with
//!   an in-band, *propagated* epistemic certainty — no separate uncertainty model.
//! - **@sparseskip recall.** Similarity runs over ternary vectors with
//!   zero-phase skipping, so cost scales with non-zero density, not dimension.
//! - **Forgetting is a feature.** [`EngramStore::consolidate`] decays and evicts;
//!   [`EngramStore::forget`] is a GDPR right-to-erasure primitive.
//! - **Append-only & auditable.** State is a replayable JSONL journal.
//!
//! ## Quick start
//! ```
//! use ternlang_engram::{EngramStore, HashEmbedder};
//!
//! // 256-dim store with the built-in offline embedder.
//! let mut mem = EngramStore::new(256)
//!     .with_embedder(Box::new(HashEmbedder::new(256)));
//!
//! let t0 = 1_750_000_000_000; // caller supplies wall-clock (deterministic core)
//! mem.remember("met Ana in Mendoza about wellbeing research", 0.9,
//!              vec!["people".into(), "research".into()], "albert", t0).unwrap();
//! mem.remember("fixed the LayerNorm gradient wall in albert", 0.8,
//!              vec!["albert".into(), "bug".into()], "albert", t0 + 3_600_000).unwrap();
//!
//! let hits = mem.recall("who works on wellbeing?", 3, t0 + 7_200_000).unwrap();
//! assert!(!hits.is_empty());
//! println!("top memory: {} (conf {:.2})", hits[0].content, hits[0].confidence);
//! ```
//!
//! Bring your own embeddings with [`EngramStore::remember_vec`] /
//! [`EngramStore::recall_vec`]; persist with [`EngramStore::open`].

mod embed;
mod episode;
mod store;
pub mod time;

pub use embed::{Embedder, HashEmbedder};
#[cfg(feature = "nvidia")]
pub use embed::NvidiaEmbedder;
pub use episode::{Episode, EpisodeId};
pub use store::{ConsolidateReport, EngramStore, Recall, RecallConfig, Stats};
pub use time::{
    humanize_age_ms, iso_to_unix_ms, now_ms, unix_ms_to_iso, unix_ms_to_stamp, weekday,
};

#[cfg(test)]
mod tests {
    use super::*;

    const T0: i64 = 1_750_000_000_000;
    const HOUR: i64 = 3_600_000;

    fn seeded() -> EngramStore {
        let mut m = EngramStore::new(384).with_embedder(Box::new(HashEmbedder::new(384)));
        m.remember("ternary neural network inference on the edge", 0.9,
                   vec!["ml".into()], "albert", T0).unwrap();
        m.remember("the weather in vienna is sunny and warm", 0.4,
                   vec!["misc".into()], "albert", T0 + HOUR).unwrap();
        m.remember("quantization of weights to balanced ternary", 0.8,
                   vec!["ml".into()], "albert", T0 + 2 * HOUR).unwrap();
        m
    }

    #[test]
    fn recall_ranks_relevant_first() {
        let mut m = seeded();
        let hits = m.recall("ternary quantization neural networks", 3, T0 + 3 * HOUR).unwrap();
        assert_eq!(hits.len(), 3);
        // The weather memory must not top a ternary-ML query.
        assert!(!hits[0].content.contains("weather"));
        assert!(hits[0].confidence >= 0.0 && hits[0].confidence <= 1.0);
    }

    #[test]
    fn recall_reinforces() {
        let mut m = seeded();
        let id = m.recall("ternary", 1, T0 + 3 * HOUR).unwrap()[0].id;
        assert_eq!(m.get(id).unwrap().access_count, 1);
        m.recall("ternary", 1, T0 + 4 * HOUR).unwrap();
        assert_eq!(m.get(id).unwrap().access_count, 2);
    }

    #[test]
    fn timeline_is_chronological_and_bounded() {
        let m = seeded();
        let slice = m.timeline(T0 + HOUR, T0 + 2 * HOUR);
        assert_eq!(slice.len(), 2);
        assert!(slice[0].t_ms <= slice[1].t_ms);
    }

    #[test]
    fn forget_removes() {
        let mut m = seeded();
        let id = m.recall("ternary", 1, T0).unwrap()[0].id;
        assert!(m.forget(id).unwrap());
        assert!(m.get(id).is_none());
    }

    #[test]
    fn consolidate_keeps_vivid_evicts_trivial() {
        let mut m = seeded(); // salience: ternary 0.9, weather 0.4, quantization 0.8
        let ternary_id = m.recall("ternary neural networks", 1, T0).unwrap()[0].id;
        let before = m.len();
        // One year on: trivial low-salience memory fades under the floor; the
        // vivid, recalled one must survive.
        let far = T0 + 365 * 24 * HOUR;
        let report = m.consolidate(far, 0.15, HOUR).unwrap();
        assert!(report.evicted >= 1, "a stale low-salience memory should be evicted");
        assert!(m.len() < before);
        assert!(m.get(ternary_id).is_some(), "a vivid, recalled memory must persist a year");
    }

    #[test]
    fn persistence_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("engram.jsonl");
        {
            let mut m = EngramStore::open(&path, 384).unwrap()
                .with_embedder(Box::new(HashEmbedder::new(384)));
            m.remember("persisted memory about ternary logic", 0.7,
                       vec!["t".into()], "albert", T0).unwrap();
            m.compact().unwrap();
        }
        let m2 = EngramStore::open(&path, 384).unwrap();
        assert_eq!(m2.len(), 1);
        assert_eq!(m2.timeline(0, i64::MAX)[0].content, "persisted memory about ternary logic");
    }

    #[test]
    fn ruvector_bridge() {
        let m = seeded();
        let db = m.to_ruvector().unwrap().expect("non-empty");
        let q = HashEmbedder::new(384).embed("ternary quantization");
        let results = db.search(&q, 2);
        assert_eq!(results.len(), 2);
    }
}
