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

//! Ternary episodic memory — time-stamped, TritFloat-quantized autobiographical recall scored by relevance × recency × salience × frequency.

pub mod embed;
pub mod store;
pub mod time;
pub mod episode;

// ─── Re-exports ───────────────────────────────────────────────────────────────
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

    #[test]
    fn store_basic_lifecycle() {
        let mut mem = EngramStore::new(128)
            .with_embedder(Box::new(HashEmbedder::new(128)));
        let t0: i64 = 1_750_000_000_000;
        mem.remember("test memory", 0.9, vec!["test".into()], "agent", t0).unwrap();
        let hits = mem.recall("test", 3, t0 + 1000).unwrap();
        assert!(!hits.is_empty());
    }

    #[test]
    fn timestamp_round_trip() {
        let ts: i64 = 1_750_000_000_000;
        let iso = unix_ms_to_iso(ts);
        let back = iso_to_unix_ms(&iso).unwrap();
        assert_eq!(ts, back);
    }
}
