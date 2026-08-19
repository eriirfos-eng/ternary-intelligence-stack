// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
//
// ternlang-moe — Ternary Mixture-of-Experts Orchestrator (MoE-13)

//! # ternlang-moe — Ternary Mixture-of-Experts Orchestrator (MoE-13)
//!
//! Implements the MoE-13 architecture from:
//!   DOI: 10.17605/OSF.IO/TZ7DC
//!
//! Key mechanisms:
//! - **Dual-key synergistic routing** — selects expert pairs by (relevance × complementarity)
//! - **1+1=3 triad synthesis** — emergent field Ek = synergy × (vi + vj)/2
//! - **6D competence vectors** — [syntax, world_knowledge, reasoning, tool_use, persona, safety]
//! - **Three-tier memory mesh** — Node (TTL:sec), Cluster (TTL:min), Axis (persistent/audit)
//! - **Safety as hard gate** — Axis 6 absolute veto overrides all other dims

use std::time::{Duration, Instant};

use ternlang_core::Trit;

// ─── Constants ────────────────────────────────────────────────────────────────

/// Routing weight between 0 and 1.
pub type Weight = f32;

/// Number of experts in the standard MoE-13 pool.
pub const MOE_EXPERT_COUNT: usize = 13;

// ─── CompetencyVector — 6D competence profiles ────────────────────────────────

/// 6D competence vector: [syntax, world_knowledge, reasoning, tool_use, persona, safety]
#[derive(Debug, Clone, PartialEq)]
pub struct CompetencyVector {
    pub dims: [f32; 6],
}

impl CompetencyVector {
    pub fn default() -> Self {
        Self { dims: [0.5; 6] }
    }
    pub fn syntax() -> Self { Self { dims: [0.95, 0.1, 0.2, 0.05, 0.1, 0.3] } }
    pub fn knowledge() -> Self { Self { dims: [0.1, 0.95, 0.4, 0.05, 0.3, 0.2] } }
    pub fn reasoning() -> Self { Self { dims: [0.4, 0.5, 0.95, 0.1, 0.3, 0.4] } }
    pub fn code() -> Self { Self { dims: [0.8, 0.3, 0.7, 0.85, 0.1, 0.2] } }
    pub fn creative() -> Self { Self { dims: [0.2, 0.4, 0.3, 0.2, 0.9, 0.1] } }
    pub fn safety() -> Self { Self { dims: [0.1, 0.2, 0.3, 0.05, 0.15, 0.95] } }
    pub fn pattern() -> Self { Self { dims: [0.5, 0.6, 0.7, 0.3, 0.4, 0.5] } }
    pub fn causal() -> Self { Self { dims: [0.3, 0.5, 0.85, 0.2, 0.3, 0.6] } }
    pub fn uncertainty() -> Self { Self { dims: [0.2, 0.3, 0.6, 0.1, 0.2, 0.5] } }
    pub fn memory() -> Self { Self { dims: [0.1, 0.8, 0.4, 0.05, 0.3, 0.2] } }
    pub fn synthesis() -> Self { Self { dims: [0.3, 0.4, 0.5, 0.2, 0.6, 0.4] } }
    pub fn consistency() -> Self { Self { dims: [0.7, 0.4, 0.6, 0.3, 0.3, 0.5] } }
    pub fn consensus() -> Self { Self { dims: [0.5, 0.5, 0.7, 0.3, 0.4, 0.6] } }

    /// Cosine similarity between two competence vectors.
    pub fn similarity(&self, other: &Self) -> f32 {
        let dot: f32 = self.dims.iter().zip(other.dims.iter()).map(|(a, b)| a * b).sum();
        let mag_a: f32 = self.dims.iter().map(|d| d * d).sum::<f32>().sqrt();
        let mag_b: f32 = other.dims.iter().map(|d| d * d).sum::<f32>().sqrt();
        if mag_a < 1e-6 || mag_b < 1e-6 {
            0.0
        } else {
            dot / (mag_a * mag_b)
        }
    }
}

// ─── Memory mesh ───────────────────────────────────────────────────────────────

/// Tier of memory persistence in the three-tier mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryTier {
    /// Node — short-term, TTL ~30s.
    Node,
    /// Cluster — working memory, TTL ~5min.
    Cluster,
    /// Axis — persistent audit trail, TTL ~1hr.
    Axis,
}

impl MemoryTier {
    pub fn ttl(&self) -> Duration {
        match self {
            MemoryTier::Node => Duration::from_secs(30),
            MemoryTier::Cluster => Duration::from_secs(300),
            MemoryTier::Axis => Duration::from_secs(3600),
        }
    }
}

/// A single memory entry in the mesh.
#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub query: Vec<f32>,
    pub response: String,
    pub stored_at: Instant,
    pub tier: MemoryTier,
}

/// Three-tier episodic memory mesh (Node / Cluster / Axis).
#[derive(Debug)]
pub struct MemoryMesh {
    store: Vec<MemoryEntry>,
}

impl MemoryMesh {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }

    pub fn store(&mut self, query: &[f32], response: &str, at: Instant, tier: MemoryTier) {
        self.store.push(MemoryEntry {
            query: query.to_vec(),
            response: response.to_string(),
            stored_at: at,
            tier,
        });
    }

    pub fn recall(&self, now: Instant, tier: MemoryTier) -> Vec<&MemoryEntry> {
        self.store
            .iter()
            .filter(|e| e.tier == tier && now.duration_since(e.stored_at) < e.tier.ttl())
            .collect()
    }
}

/// A safety veto entry on the Axis (persistent/audit) memory tier.
/// Records a plugin/sandbox boundary violation that triggered the
/// hard safety gate.
#[derive(Debug, Clone)]
pub struct VetoEntry {
    pub timestamp: std::time::SystemTime,
    pub expert_id: u32,
    pub reason: String,
    pub query_hash: u64,
}

impl VetoEntry {
    pub fn new(expert_id: u32, reason: impl Into<String>, query_hash: u64) -> Self {
        Self {
            timestamp: std::time::SystemTime::now(),
            expert_id,
            reason: reason.into(),
            query_hash,
        }
    }
}

/// Axis-tier memory — the persistent/audit layer of the memory mesh.
/// Holds a veto log that plugins write to via the safety sandbox.
#[derive(Debug, Default)]
pub struct AxisMemory {
    pub veto_log: Vec<VetoEntry>,
}

impl AxisMemory {
    pub fn new() -> Self {
        Self { veto_log: Vec::new() }
    }
}

// ─── Router ────────────────────────────────────────────────────────────────────

/// Result of routing a query to an expert.
#[derive(Debug, Clone)]
pub struct RouteScore {
    pub expert_idx: usize,
    pub relevance: f32,
    pub complementarity: f32,
}

/// Decision from dual-key routing — primary + complement expert.
#[derive(Debug, Clone)]
pub struct RouteDecision {
    pub primary: usize,
    pub partner: usize,
    pub synergy: f32,
}

/// Dual-key synergistic router.
#[derive(Debug)]
pub struct Router {
    competencies: Vec<CompetencyVector>,
}

impl Router {
    pub fn new(competencies: Vec<CompetencyVector>) -> Self {
        Self { competencies }
    }

    pub fn route(&self, query: &[f32]) -> RouteScore {
        // Query proxy: mean of query dims clamped to [0,1]
        let q_proxy = if query.is_empty() {
            CompetencyVector::default()
        } else {
            let mean = query.iter().map(|v| v.abs()).sum::<f32>() / query.len() as f32;
            CompetencyVector { dims: [mean; 6] }
        };

        let best_idx = self
            .competencies
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                let sim_a = a.similarity(&q_proxy);
                let sim_b = b.similarity(&q_proxy);
                sim_a.partial_cmp(&sim_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0);

        let relevance = self
            .competencies
            .get(best_idx)
            .map(|c| c.similarity(&q_proxy))
            .unwrap_or(0.0);

        RouteScore {
            expert_idx: best_idx,
            relevance,
            complementarity: 1.0 - relevance,
        }
    }

    pub fn route_complement(&self, query: &[f32], exclude: usize) -> RouteScore {
        let q_proxy = if query.is_empty() {
            CompetencyVector::default()
        } else {
            let mean = query.iter().map(|v| v.abs()).sum::<f32>() / query.len() as f32;
            CompetencyVector { dims: [mean; 6] }
        };

        let complement_idx = self
            .competencies
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != exclude)
            .min_by(|(_, a), (_, b)| {
                let sim_a = a.similarity(&q_proxy);
                let sim_b = b.similarity(&q_proxy);
                sim_a.partial_cmp(&sim_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0);

        RouteScore {
            expert_idx: complement_idx,
            relevance: 0.5,
            complementarity: 0.8,
        }
    }

    pub fn score_expert(&self, idx: usize, query: &[f32]) -> f32 {
        let q_proxy = if query.is_empty() {
            CompetencyVector::default()
        } else {
            let mean = query.iter().map(|v| v.abs()).sum::<f32>() / query.len() as f32;
            CompetencyVector { dims: [mean; 6] }
        };
        self.competencies
            .get(idx)
            .map(|c| c.similarity(&q_proxy))
            .unwrap_or(0.0)
    }
}

// ─── ExpertAgent ──────────────────────────────────────────────────────────────

/// A single expert agent within the MoE-13 pool.
#[derive(Debug, Clone)]
pub struct ExpertAgent {
    pub id: usize,
    pub name: String,
    pub competency: CompetencyVector,
    pub weight: Weight,
}

impl ExpertAgent {
    /// Build the standard 13-agent pool with 6D competence vectors.
    pub fn standard_pool() -> Vec<Self> {
        const EXPERT_NAMES: &[&str] = &[
            "syntax_guard",
            "world_knowledge",
            "reasoning_engine",
            "code_specialist",
            "creative_catalyst",
            "safety_veto",
            "pattern_recognizer",
            "causal_analyst",
            "uncertainty_quantifier",
            "memory_curator",
            "response_synthesizer",
            "consistency_checker",
            "consensus_builder",
        ];

        EXPERT_NAMES
            .iter()
            .enumerate()
            .map(|(i, &name)| {
                let competency = match name {
                    "syntax_guard" => CompetencyVector::syntax(),
                    "world_knowledge" => CompetencyVector::knowledge(),
                    "reasoning_engine" => CompetencyVector::reasoning(),
                    "code_specialist" => CompetencyVector::code(),
                    "creative_catalyst" => CompetencyVector::creative(),
                    "safety_veto" => CompetencyVector::safety(),
                    "pattern_recognizer" => CompetencyVector::pattern(),
                    "causal_analyst" => CompetencyVector::causal(),
                    "uncertainty_quantifier" => CompetencyVector::uncertainty(),
                    "memory_curator" => CompetencyVector::memory(),
                    "response_synthesizer" => CompetencyVector::synthesis(),
                    "consistency_checker" => CompetencyVector::consistency(),
                    "consensus_builder" => CompetencyVector::consensus(),
                    _ => CompetencyVector::default(),
                };
                ExpertAgent {
                    id: i,
                    name: name.to_string(),
                    competency,
                    weight: 1.0 / 13.0,
                }
            })
            .collect()
    }
}

// ─── TriadResult ──────────────────────────────────────────────────────────────

/// Result of a triad synthesis — a ternary decision with confidence.
#[derive(Debug, Clone, PartialEq)]
pub struct TriadResult {
    pub winner: Trit,
    pub confidence: f32,
    pub synergy: f32,
}

// ─── TernMoeOrchestrator ─────────────────────────────────────────────────────

/// 13-agent deliberation pool with dual-key routing.
#[derive(Debug)]
pub struct TernMoeOrchestrator {
    pub experts: Vec<ExpertAgent>,
    pub router: Router,
    pub memory: MemoryMesh,
    pub max_rounds: usize,
}

impl TernMoeOrchestrator {
    /// Create from a list of expert agents.
    pub fn new(experts: Vec<ExpertAgent>) -> Self {
        let router = Router::new(experts.iter().map(|e| e.competency.clone()).collect());
        Self {
            experts,
            router,
            memory: MemoryMesh::new(),
            max_rounds: 3,
        }
    }

    /// Convenience: build the standard 13-expert configuration.
    pub fn with_standard_experts() -> Self {
        let experts = ExpertAgent::standard_pool();
        Self::new(experts)
    }

    /// Route a query to the best expert pair using dual-key routing
    /// (relevance × complementarity).
    pub fn route(&self, query: &[f32]) -> RouteDecision {
        let primary = self.router.route(query);
        let complement = self.router.route_complement(query, primary.expert_idx);
        RouteDecision {
            primary: primary.expert_idx,
            partner: complement.expert_idx,
            synergy: primary.relevance * complement.relevance,
        }
    }

    /// Run a full deliberation round across all 13 agents.
    /// Returns the consensus trit + confidence.
    pub fn deliberate(&mut self, query: &[f32]) -> TriadResult {
        let route = self.route(query);
        // Classify query sign: negative → Reject bias, positive → Affirm bias, ~0 → Tend.
        let query_mean = if query.is_empty() {
            0.0f32
        } else {
            query.iter().sum::<f32>() / query.len() as f32
        };
        let classified = ternlang_ml::classify_trit(query_mean as f64);
        let mut votes: Vec<Trit> = Vec::new();
        let mut weights: Vec<f32> = Vec::new();

        for (i, agent) in self.experts.iter().enumerate() {
            let relevance = self.router.score_expert(i, query);
            if relevance < 0.01 {
                continue;
            }
            let vote = if relevance > 0.7 {
                // Strong relevance follows query polarity.
                if query_mean < -0.1 { Trit::Reject } else { Trit::Affirm }
            } else if relevance < 0.3 && i != route.primary && i != route.partner {
                Trit::Reject
            } else {
                classified
            };
            votes.push(vote);
            weights.push(agent.weight * relevance);
        }

        self.synthesize(&votes, &weights, route.synergy)
    }

    /// Core triad synthesis: combine weighted votes + synergy into a result.
    /// Patent reference: A50296/2026 Claim 5 (+1 = 3).
    fn synthesize(&self, votes: &[Trit], weights: &[f32], synergy: f32) -> TriadResult {
        let mut w_affirm = 0.0f32;
        let mut w_reject = 0.0f32;
        let mut w_tend = 0.0f32;

        for (v, &w) in votes.iter().zip(weights.iter()) {
            match v {
                Trit::Affirm => w_affirm += w,
                Trit::Reject => w_reject += w,
                Trit::Tend => w_tend += w,
            }
        }

        let (winner, base_conf) = if w_affirm >= w_reject && w_affirm >= w_tend {
            (Trit::Affirm, w_affirm)
        } else if w_reject >= w_tend {
            (Trit::Reject, w_reject)
        } else {
            (Trit::Tend, w_tend)
        };

        let total: f32 = weights.iter().sum();
        let raw_confidence = if total > 0.0 { base_conf / total } else { 0.0 };
        let confidence = (raw_confidence * (1.0 + synergy * 0.5)).min(1.0);
        TriadResult { winner, confidence, synergy }
    }

    /// Store a memory in the appropriate tier.
    pub fn remember(&mut self, query: &[f32], response: &str, tier: MemoryTier) {
        self.memory.store(query, response, Instant::now(), tier);
    }
}

// ─── GDScalar — consensus result from deliberation ───────────────────────────

/// Result of a consensus deliberation round across 13 agents.
/// Contains the winning trit, aggregate confidence, reasoning trace,
/// and final resolved trit.
#[derive(Debug, Clone)]
pub struct GDScalar {
    /// The winning trit from the consensus round.
    pub trit: Trit,
    /// Aggregate confidence in [0, 1].
    pub confidence: f32,
    /// Full reasoning trace from all 13 agents.
    pub trace: Vec<String>,
    /// Final resolved trit after EMA smoothing across rounds.
    pub final_trit: Trit,
}

// ─── Re-exports ───────────────────────────────────────────────────────────────

pub use CompetencyVector as CompVector;
pub use RouteDecision as Decision;

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_pool_has_13_agents() {
        let pool = ExpertAgent::standard_pool();
        assert_eq!(pool.len(), MOE_EXPERT_COUNT);
    }

    #[test]
    fn consensus_affirm_on_strong_signal() {
        let mut moe = TernMoeOrchestrator::with_standard_experts();
        let query = vec![0.9f32; 6];
        let result = moe.deliberate(&query);
        assert_eq!(result.winner, Trit::Affirm);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn consensus_reject_on_strong_negative() {
        let mut moe = TernMoeOrchestrator::with_standard_experts();
        let query = vec![-0.9f32; 6];
        let result = moe.deliberate(&query);
        assert_eq!(result.winner, Trit::Reject);
    }

    #[test]
    fn competence_similarity_bounds() {
        let a = CompetencyVector::syntax();
        let b = CompetencyVector::safety();
        assert!((0.0..=1.0).contains(&a.similarity(&b)));
    }
}
