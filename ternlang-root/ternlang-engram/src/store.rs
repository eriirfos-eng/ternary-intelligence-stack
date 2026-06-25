// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.

//! The episodic store: remember, recall, timeline, consolidate, forget.

use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use ternlang_ml::TritFloat;
use ternlang_ruvector::RuVectorDB;

use crate::embed::Embedder;
use crate::episode::{Episode, EpisodeId};

/// Weights and decay constants for composite recall scoring.
///
/// Recall ranks each episode by a blend modeled on human episodic retrieval
/// (relevance, recency, salience, frequency — cf. Park et al. 2023):
///
/// ```text
/// score = w_relevance * relevance      // ternary cosine via @sparseskip
///       + w_recency   * recency        // exponential time decay
///       + w_importance* importance     // stored salience
///       + w_frequency * frequency      // reinforcement from prior recalls
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecallConfig {
    pub w_relevance: f32,
    pub w_recency: f32,
    pub w_importance: f32,
    pub w_frequency: f32,
    /// Recency half-life in milliseconds for *recall scoring* (default 7 days).
    pub recency_half_life_ms: i64,
    /// Base half-life in milliseconds for *consolidation decay* (default 90 days).
    /// Effective decay is this scaled by each episode's stability, so vivid and
    /// frequently-recalled memories fade far slower (Ebbinghaus-style).
    pub decay_half_life_ms: i64,
}

impl Default for RecallConfig {
    fn default() -> Self {
        Self {
            w_relevance: 1.0,
            w_recency: 1.0,
            w_importance: 1.0,
            w_frequency: 0.5,
            recency_half_life_ms: 7 * 24 * 60 * 60 * 1000,
            decay_half_life_ms: 90 * 24 * 60 * 60 * 1000,
        }
    }
}

/// One scored recall hit. `confidence` is the **native** TritFloat certainty
/// propagated through the similarity dot product — not a post-hoc estimate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recall {
    pub id: EpisodeId,
    pub score: f32,
    pub relevance: f32,
    pub recency: f32,
    pub importance: f32,
    pub frequency: f32,
    /// In-band epistemic certainty of the match, in `[0, 1]`.
    pub confidence: f32,
    /// Event time, unix milliseconds (the epoch anchor).
    pub t_ms: i64,
    /// Canonical `Weekday/ISO8601Z/epoch` stamp of the event (matches the `iso` tool).
    pub stamp: String,
    /// Human-legible age at recall time, e.g. `"3d 4h ago"`.
    pub age: String,
    pub content: String,
    pub tags: Vec<String>,
}

/// Summary returned by [`EngramStore::consolidate`].
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConsolidateReport {
    pub decayed: usize,
    pub evicted: usize,
    pub linked: usize,
}

/// Health snapshot of the store.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    pub episodes: usize,
    pub dim: usize,
    pub avg_importance: f32,
    pub avg_confidence: f32,
    pub avg_sparsity: f32,
    pub oldest_ms: i64,
    pub newest_ms: i64,
    pub total_access: u64,
}

/// Append-only journal entry. The on-disk log is a JSONL replay stream.
#[derive(Serialize, Deserialize)]
#[serde(tag = "op")]
enum LogEntry {
    Remember(Episode),
    Forget { id: EpisodeId },
    ForgetBefore { t_ms: i64 },
    /// Written by [`EngramStore::compact`]: a full-state checkpoint that
    /// supersedes everything before it (also persists recall reinforcement).
    Snapshot { last_id: EpisodeId, episodes: Vec<Episode> },
}

/// Ternary episodic memory store.
pub struct EngramStore {
    dim: usize,
    episodes: Vec<Episode>,
    last_id: EpisodeId,
    cfg: RecallConfig,
    embedder: Option<Box<dyn Embedder>>,
    log_path: Option<PathBuf>,
}

impl EngramStore {
    /// In-memory store of the given embedding dimension. No persistence, no embedder.
    pub fn new(dim: usize) -> Self {
        assert!(dim > 0, "dim must be > 0");
        Self {
            dim,
            episodes: Vec::new(),
            last_id: 0,
            cfg: RecallConfig::default(),
            embedder: None,
            log_path: None,
        }
    }

    /// Attach a text embedder, enabling the `*_text` convenience methods.
    pub fn with_embedder(mut self, embedder: Box<dyn Embedder>) -> Self {
        assert_eq!(
            embedder.dim(),
            self.dim,
            "embedder dim must match store dim"
        );
        self.embedder = Some(embedder);
        self
    }

    /// Override the default recall scoring weights.
    pub fn with_config(mut self, cfg: RecallConfig) -> Self {
        self.cfg = cfg;
        self
    }

    /// Open (or create) a persistent store backed by an append-only JSONL log,
    /// replaying it to reconstruct state. `dim` must match the original store.
    pub fn open(path: impl AsRef<Path>, dim: usize) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let mut store = Self::new(dim);

        if path.exists() {
            let file = File::open(&path)
                .with_context(|| format!("opening engram log {}", path.display()))?;
            for (lineno, line) in BufReader::new(file).lines().enumerate() {
                let line = line?;
                if line.trim().is_empty() {
                    continue;
                }
                let entry: LogEntry = serde_json::from_str(&line)
                    .with_context(|| format!("corrupt engram log at line {}", lineno + 1))?;
                store.apply_replay(entry)?;
            }
        }
        store.log_path = Some(path);
        Ok(store)
    }

    fn apply_replay(&mut self, entry: LogEntry) -> Result<()> {
        match entry {
            LogEntry::Remember(ep) => {
                self.last_id = self.last_id.max(ep.id);
                self.episodes.push(ep);
            }
            LogEntry::Forget { id } => {
                self.episodes.retain(|e| e.id != id);
            }
            LogEntry::ForgetBefore { t_ms } => {
                self.episodes.retain(|e| e.t_ms >= t_ms);
            }
            LogEntry::Snapshot { last_id, episodes } => {
                self.episodes = episodes;
                self.last_id = last_id;
            }
        }
        Ok(())
    }

    fn append(&self, entry: &LogEntry) -> Result<()> {
        if let Some(path) = &self.log_path {
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .with_context(|| format!("appending to engram log {}", path.display()))?;
            let line = serde_json::to_string(entry)?;
            writeln!(f, "{line}")?;
        }
        Ok(())
    }

    // ── Writing memories ──────────────────────────────────────────────────────

    /// Store an episode from a caller-supplied f32 embedding (BYO vectors).
    ///
    /// `importance` is the salience in `[0, 1]`; it seeds the TritFloat confidence
    /// field, so low-salience memories yield low-confidence recalls. Returns the
    /// new episode id.
    #[allow(clippy::too_many_arguments)]
    pub fn remember_vec(
        &mut self,
        embedding: &[f32],
        content: impl Into<String>,
        importance: f32,
        tags: Vec<String>,
        source: impl Into<String>,
        now_ms: i64,
    ) -> Result<EpisodeId> {
        if embedding.len() != self.dim {
            return Err(anyhow!(
                "embedding dim {} != store dim {}",
                embedding.len(),
                self.dim
            ));
        }
        let importance = importance.clamp(0.0, 1.0);
        let norm = l2_norm(embedding);
        // Normalize so the recall dot product is a cosine similarity.
        let normalized: Vec<f32> = if norm > 1e-9 {
            embedding.iter().map(|x| x / norm).collect()
        } else {
            embedding.to_vec()
        };

        // The id IS the timestamp (unix ms), bumped by 1ms on collision so it
        // stays unique and strictly monotonic. The id therefore encodes *when*.
        let id = (now_ms.max(0) as u64).max(self.last_id + 1);
        self.last_id = id;
        let ep = Episode {
            id,
            t_ms: now_ms,
            content: content.into(),
            tags,
            source: source.into(),
            importance,
            access_count: 0,
            last_access_ms: now_ms,
            links: Vec::new(),
            embedding: Episode::quantize(&normalized, importance),
            norm: 1.0, // already normalized
        };
        self.append(&LogEntry::Remember(ep.clone()))?;
        self.episodes.push(ep);
        Ok(id)
    }

    /// Store an episode from text, embedding it with the attached [`Embedder`].
    pub fn remember(
        &mut self,
        text: impl AsRef<str>,
        importance: f32,
        tags: Vec<String>,
        source: impl Into<String>,
        now_ms: i64,
    ) -> Result<EpisodeId> {
        let text = text.as_ref();
        let emb = self
            .embedder
            .as_ref()
            .ok_or_else(|| anyhow!("no embedder attached; use remember_vec or with_embedder"))?
            .embed(text);
        self.remember_vec(&emb, text, importance, tags, source, now_ms)
    }

    // ── Recall ────────────────────────────────────────────────────────────────

    /// Recall the top-`k` episodes for a caller-supplied query vector at time
    /// `now_ms`. Recalled episodes are reinforced (access_count += 1).
    pub fn recall_vec(&mut self, query: &[f32], k: usize, now_ms: i64) -> Result<Vec<Recall>> {
        if query.len() != self.dim {
            return Err(anyhow!(
                "query dim {} != store dim {}",
                query.len(),
                self.dim
            ));
        }
        if self.episodes.is_empty() || k == 0 {
            return Ok(Vec::new());
        }

        let qnorm = l2_norm(query);
        let qnorm = if qnorm > 1e-9 { qnorm } else { 1.0 };
        // Query is certain: confidence 1.0. The propagated confidence of a match
        // therefore collapses to the episode's own salience.
        let q_tf: Vec<TritFloat> = query
            .iter()
            .map(|x| TritFloat::from_f32_with_confidence(x / qnorm, 1.0))
            .collect();

        let cfg = &self.cfg;
        // Score every episode in parallel (@sparseskip inside dot_with_skips).
        let mut scored: Vec<Recall> = self
            .episodes
            .par_iter()
            .map(|ep| {
                let (dot, _skipped) = TritFloat::dot_with_skips(&q_tf, &ep.embedding);
                let cos = (dot.to_f32() / (ep.norm + 1e-9)).clamp(-1.0, 1.0);
                let relevance = (cos + 1.0) * 0.5; // map [-1,1] → [0,1]

                let age = (now_ms - ep.t_ms).max(0) as f64;
                let hl = cfg.recency_half_life_ms.max(1) as f64;
                let recency = 0.5f64.powf(age / hl) as f32;

                let frequency = 1.0 - 1.0 / (1.0 + ep.access_count as f32);

                let score = cfg.w_relevance * relevance
                    + cfg.w_recency * recency
                    + cfg.w_importance * ep.importance
                    + cfg.w_frequency * frequency;

                Recall {
                    id: ep.id,
                    score,
                    relevance,
                    recency,
                    importance: ep.importance,
                    frequency,
                    confidence: dot.confidence(),
                    t_ms: ep.t_ms,
                    stamp: crate::time::unix_ms_to_stamp(ep.t_ms),
                    age: crate::time::humanize_age_ms(now_ms - ep.t_ms),
                    content: ep.content.clone(),
                    tags: ep.tags.clone(),
                }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);

        // Reinforce recalled episodes.
        for r in &scored {
            if let Some(ep) = self.episodes.iter_mut().find(|e| e.id == r.id) {
                ep.access_count += 1;
                ep.last_access_ms = now_ms;
            }
        }
        Ok(scored)
    }

    /// Recall by text query (requires an attached [`Embedder`]).
    pub fn recall(&mut self, text: impl AsRef<str>, k: usize, now_ms: i64) -> Result<Vec<Recall>> {
        let emb = self
            .embedder
            .as_ref()
            .ok_or_else(|| anyhow!("no embedder attached; use recall_vec or with_embedder"))?
            .embed_query(text.as_ref());
        self.recall_vec(&emb, k, now_ms)
    }

    // ── Temporal access ───────────────────────────────────────────────────────

    /// Episodes whose timestamp falls in `[from_ms, to_ms]`, in chronological order.
    /// This is the episodic-specific query a pure vector store cannot answer.
    pub fn timeline(&self, from_ms: i64, to_ms: i64) -> Vec<&Episode> {
        let mut v: Vec<&Episode> = self
            .episodes
            .iter()
            .filter(|e| e.t_ms >= from_ms && e.t_ms <= to_ms)
            .collect();
        v.sort_by_key(|e| (e.t_ms, e.id));
        v
    }

    /// Fetch one episode by id.
    pub fn get(&self, id: EpisodeId) -> Option<&Episode> {
        self.episodes.iter().find(|e| e.id == id)
    }

    // ── Consolidation & forgetting ────────────────────────────────────────────

    /// Run one consolidation pass at `now_ms`:
    /// 1. Decay each episode's importance along a forgetting curve modulated by
    ///    recency of access (frequently recalled memories decay slower).
    /// 2. Evict episodes whose importance falls below `floor` *and* are older than
    ///    `min_age_ms` (never evict fresh memories).
    ///
    /// Returns counts. Persisted to the log as a [`compact`](Self::compact)-style
    /// checkpoint so reinforcement and decay survive a restart.
    pub fn consolidate(&mut self, now_ms: i64, floor: f32, min_age_ms: i64) -> Result<ConsolidateReport> {
        let base_hl = self.cfg.decay_half_life_ms.max(1) as f64;
        let mut report = ConsolidateReport::default();

        for ep in &mut self.episodes {
            // Stability grows with original salience and reinforcement, so vivid,
            // often-recalled memories decay much slower than trivial ones.
            let stability = 1.0 + ep.access_count as f64 + 4.0 * ep.importance as f64;
            // Decay relative to time since last access; access resets the clock.
            let idle = (now_ms - ep.last_access_ms).max(0) as f64;
            let decay = 0.5f64.powf(idle / (base_hl * stability)) as f32;
            let before = ep.importance;
            ep.importance = (ep.importance * decay).clamp(0.0, 1.0);
            if (before - ep.importance).abs() > f32::EPSILON {
                report.decayed += 1;
            }
        }

        let before_len = self.episodes.len();
        self.episodes.retain(|e| {
            let old_enough = (now_ms - e.t_ms) > min_age_ms;
            !(old_enough && e.importance < floor)
        });
        report.evicted = before_len - self.episodes.len();

        // Persist the new state as a compacting checkpoint.
        self.compact()?;
        Ok(report)
    }

    /// Explicitly delete one episode (GDPR right-to-erasure primitive).
    pub fn forget(&mut self, id: EpisodeId) -> Result<bool> {
        let before = self.episodes.len();
        self.episodes.retain(|e| e.id != id);
        let removed = self.episodes.len() != before;
        if removed {
            self.append(&LogEntry::Forget { id })?;
        }
        Ok(removed)
    }

    /// Delete every episode older than `t_ms` (retention-policy / erasure).
    pub fn forget_before(&mut self, t_ms: i64) -> Result<usize> {
        let before = self.episodes.len();
        self.episodes.retain(|e| e.t_ms >= t_ms);
        let removed = before - self.episodes.len();
        if removed > 0 {
            self.append(&LogEntry::ForgetBefore { t_ms })?;
        }
        Ok(removed)
    }

    // ── Maintenance & introspection ───────────────────────────────────────────

    /// Rewrite the log as a single full-state checkpoint, collapsing all prior
    /// entries and persisting in-memory reinforcement/decay. Safe no-op for
    /// in-memory stores.
    pub fn compact(&self) -> Result<()> {
        if let Some(path) = &self.log_path {
            let tmp = path.with_extension("jsonl.tmp");
            {
                let mut f = File::create(&tmp)
                    .with_context(|| format!("writing checkpoint {}", tmp.display()))?;
                let entry = LogEntry::Snapshot {
                    last_id: self.last_id,
                    episodes: self.episodes.clone(),
                };
                writeln!(f, "{}", serde_json::to_string(&entry)?)?;
            }
            std::fs::rename(&tmp, path)?;
        }
        Ok(())
    }

    /// Health snapshot.
    pub fn stats(&self) -> Stats {
        let n = self.episodes.len();
        if n == 0 {
            return Stats { dim: self.dim, ..Default::default() };
        }
        let mut imp = 0.0f32;
        let mut conf = 0.0f32;
        let mut sparsity = 0.0f32;
        let mut oldest = i64::MAX;
        let mut newest = i64::MIN;
        let mut access = 0u64;
        for e in &self.episodes {
            imp += e.importance;
            let nz = e.nnz();
            conf += if !e.embedding.is_empty() {
                e.embedding.iter().map(|t| t.confidence()).sum::<f32>() / e.embedding.len() as f32
            } else {
                0.0
            };
            sparsity += 1.0 - (nz as f32 / self.dim as f32);
            oldest = oldest.min(e.t_ms);
            newest = newest.max(e.t_ms);
            access += e.access_count as u64;
        }
        let nf = n as f32;
        Stats {
            episodes: n,
            dim: self.dim,
            avg_importance: imp / nf,
            avg_confidence: conf / nf,
            avg_sparsity: sparsity / nf,
            oldest_ms: oldest,
            newest_ms: newest,
            total_access: access,
        }
    }

    /// Number of stored episodes.
    pub fn len(&self) -> usize {
        self.episodes.len()
    }

    /// True if no episodes are stored.
    pub fn is_empty(&self) -> bool {
        self.episodes.is_empty()
    }

    // ── RuVector bridge ───────────────────────────────────────────────────────

    /// Export the whole episodic corpus into a [`RuVectorDB`] for batch RAG /
    /// offline semantic search over memories. Metadata = episode content.
    ///
    /// The episodic hot path stays append-friendly; RuVector is the static,
    /// dense-corpus companion. Returns `None` if empty.
    pub fn to_ruvector(&self) -> Result<Option<RuVectorDB>> {
        if self.episodes.is_empty() {
            return Ok(None);
        }
        let embeddings: Vec<Vec<f32>> = self
            .episodes
            .iter()
            .map(|e| e.embedding.iter().map(|t| t.to_f32()).collect())
            .collect();
        let metadata: Vec<String> = self.episodes.iter().map(|e| e.content.clone()).collect();
        let db = RuVectorDB::from_f32(&embeddings, metadata)
            .map_err(|e| anyhow!("ruvector bridge failed: {e}"))?;
        Ok(Some(db))
    }
}

fn l2_norm(v: &[f32]) -> f32 {
    v.iter().map(|x| x * x).sum::<f32>().sqrt()
}
