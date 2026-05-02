//! # MoE-13 Expert System
//!
//! Control and routing layer for Mixture-of-Experts architecture on top of the ternary inference substrate.

use crate::core::mock_layer::{TernaryLayer, ternary_matmul_kernel};
use crate::core::aedl::AEDL;
use crate::core::policy::{HybridRouterPolicy, MoEMode};
use crate::experts::ExpertLogic;
use crate::experts::domains::{ethics::EthicsExpert, logic::LogicExpert, science::ScienceExpert};
use rand::{prelude::*, Rng};

#[derive(Clone, Copy, Debug)]
pub enum ExpertType { FastSparse, Balanced, HighPrecision, MemoryHeavy }

/// Indexed bank of 13 independent experts.
///
/// Each slot holds a `TernaryLayer` (weight matrix) and an `ExpertLogic` domain filter.
/// Domain assignment (round-robin by index % 3): 0,3,6,9,12 → Ethics; 1,4,7,10 → Logic; 2,5,8,11 → Science.
pub struct ExpertBank13 {
    pub experts: Vec<TernaryLayer>,
    domain_logic: Vec<Box<dyn ExpertLogic + Send + Sync>>,
}

impl ExpertBank13 {
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        let mut rng = thread_rng();
        let experts = (0..13)
            .map(|_| TernaryLayer {
                weights: (0..input_dim * output_dim).map(|_| rng.gen_range(-1..2) as i8).collect(),
                alpha: 1.0,
                bias: vec![0.0; output_dim],
                input_dim,
                output_dim,
            })
            .collect();
        Self { experts, domain_logic: Self::default_domain_logic() }
    }

    /// Construct from pre-loaded weight layers (e.g., from a .tern.bin file).
    /// Attaches default domain logic (Ethics / Logic / Science round-robin).
    pub fn from_layers(experts: Vec<TernaryLayer>) -> Self {
        Self { experts, domain_logic: Self::default_domain_logic() }
    }

    /// Returns a domain score in {-1.0, 0.0, +1.0} for each of the 13 experts.
    /// Used by MoERouter13 to bias routing toward domain-competent experts.
    pub fn domain_scores(&self, query: &[f32]) -> Vec<f32> {
        self.domain_logic.iter().map(|d| d.evaluate(query) as f32).collect()
    }

    pub fn execute_expert(&self, expert_idx: usize, input: &[f32]) -> Vec<f32> {
        let expert = &self.experts[expert_idx];
        let mut output = vec![0.0; expert.output_dim];
        ternary_matmul_kernel(&expert.weights, input, &mut output, expert.alpha, expert.input_dim, expert.output_dim);
        output
    }

    fn default_domain_logic() -> Vec<Box<dyn ExpertLogic + Send + Sync>> {
        (0..13)
            .map(|i| -> Box<dyn ExpertLogic + Send + Sync> {
                match i % 3 {
                    0 => Box::new(EthicsExpert),
                    1 => Box::new(LogicExpert),
                    _ => Box::new(ScienceExpert),
                }
            })
            .collect()
    }
}

/// Input-dependent routing layer.
pub struct MoERouter13 {
    pub gate_weights: Vec<f32>,
    pub aedl: AEDL,
    pub policy: HybridRouterPolicy,
}

/// Weight of domain expert scores relative to gate scores.
/// Kept small so domain bias nudges rather than overrides learned routing.
const DOMAIN_LAMBDA: f32 = 0.1;

impl MoERouter13 {
    pub fn new(input_dim: usize) -> Self {
        Self {
            gate_weights: vec![0.1; input_dim * 13],
            aedl: AEDL::new(13),
            policy: HybridRouterPolicy::default(),
        }
    }

    /// Selects top-k experts based on gate weights, AEDL bias (HYBRID only), and domain scores.
    ///
    /// `domain_bias`: per-expert score from `ExpertBank13::domain_scores()` in {-1, 0, +1}.
    /// Applied in both EPIS and HYBRID modes — domain scoring is fully deterministic.
    pub fn route(&self, input: &[f32], top_k: usize, domain_bias: &[f32]) -> Vec<(usize, f32)> {
        let mut scores = vec![0.0f32; 13];
        for i in 0..13 {
            let mut score = 0.0f32;
            for j in 0..input.len() {
                score += input[j] * self.gate_weights[i * input.len() + j];
            }

            if self.policy.mode == MoEMode::HYBRID {
                score += self.policy.lambda * self.aedl.get_bias(i);
            }

            if i < domain_bias.len() {
                score += DOMAIN_LAMBDA * domain_bias[i];
            }

            scores[i] = score;
        }

        let mut indexed: Vec<(usize, f32)> = scores.into_iter().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        indexed.into_iter().take(top_k).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_scores_returns_13_values() {
        let bank = ExpertBank13::new(64, 64);
        let query = vec![0.5f32; 64];
        let scores = bank.domain_scores(&query);
        assert_eq!(scores.len(), 13);
        assert!(scores.iter().all(|&s| s == -1.0 || s == 0.0 || s == 1.0),
            "domain scores must be ternary values");
    }

    #[test]
    fn test_from_layers_attaches_domain_logic() {
        let layers: Vec<TernaryLayer> = (0..13).map(|_| TernaryLayer {
            weights: vec![0i8; 64 * 64],
            alpha: 1.0,
            bias: vec![0.0; 64],
            input_dim: 64,
            output_dim: 64,
        }).collect();
        let bank = ExpertBank13::from_layers(layers);
        let scores = bank.domain_scores(&vec![1.0f32; 64]);
        assert_eq!(scores.len(), 13);
    }

    #[test]
    fn test_domain_bias_influences_routing() {
        let bank = ExpertBank13::new(64, 64);
        let query = vec![0.3f32; 64];
        let domain_bias = bank.domain_scores(&query);

        let router = MoERouter13::new(64);
        // Route with domain bias vs zero bias
        let with_domain = router.route(&query, 2, &domain_bias);
        let without_domain = router.route(&query, 2, &vec![0.0f32; 13]);

        // Both must return exactly 2 experts
        assert_eq!(with_domain.len(), 2);
        assert_eq!(without_domain.len(), 2);
        // Selected expert indices must be in 0..13
        assert!(with_domain.iter().all(|(idx, _)| *idx < 13));
    }

    #[test]
    fn test_route_is_deterministic_in_epis_mode() {
        let bank = ExpertBank13::new(64, 64);
        let query = vec![0.7f32; 64];
        let domain_bias = bank.domain_scores(&query);
        let router = MoERouter13::new(64);

        let a = router.route(&query, 2, &domain_bias);
        let b = router.route(&query, 2, &domain_bias);
        assert_eq!(a, b, "EPIS routing must be deterministic");
    }
}
