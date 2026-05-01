//! # MoE-13 Expert System
//!
//! Control and routing layer for Mixture-of-Experts architecture on top of the ternary inference substrate.

use crate::core::mock_layer::{TernaryLayer, ternary_matmul_kernel};
use crate::core::aedl::AEDL;
use crate::core::policy::{HybridRouterPolicy, MoEMode};
use rand::{prelude::*, Rng};

#[derive(Clone, Copy, Debug)]
pub enum ExpertType { FastSparse, Balanced, HighPrecision, MemoryHeavy }

/// Indexed bank of 13 independent experts.
pub struct ExpertBank13 {
    pub experts: Vec<TernaryLayer>,
}

impl ExpertBank13 {
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        let mut rng = thread_rng();
        let mut experts = Vec::with_capacity(13);
        for _ in 0..13 {
            experts.push(TernaryLayer {
                weights: (0..input_dim * output_dim).map(|_| rng.gen_range(-1..2) as i8).collect(),
                alpha: 1.0,
                bias: vec![0.0; output_dim],
                input_dim,
                output_dim,
            });
        }
        Self { experts }
    }

    pub fn execute_expert(&self, expert_idx: usize, input: &[f32]) -> Vec<f32> {
        let expert = &self.experts[expert_idx];
        let mut output = vec![0.0; expert.output_dim];
        ternary_matmul_kernel(&expert.weights, input, &mut output, expert.alpha, expert.input_dim, expert.output_dim);
        output
    }
}

/// Input-dependent routing layer.
pub struct MoERouter13 {
    pub gate_weights: Vec<f32>,
    pub aedl: AEDL,
    pub policy: HybridRouterPolicy,
}

impl MoERouter13 {
    pub fn new(input_dim: usize) -> Self {
        Self {
            gate_weights: vec![0.1; input_dim * 13],
            aedl: AEDL::new(13),
            policy: HybridRouterPolicy::default(),
        }
    }

    /// Selects top-k experts based on input conditioning and adaptive bias.
    pub fn route(&self, input: &[f32], top_k: usize) -> Vec<(usize, f32)> {
        let mut scores = vec![0.0; 13];
        for i in 0..13 {
            let mut score = 0.0;
            for j in 0..input.len() {
                score += input[j] * self.gate_weights[i * input.len() + j];
            }
            
            // Apply Hybrid Routing: base_score + (λ * AEDL_bias)
            if self.policy.mode == MoEMode::HYBRID {
                scores[i] = score + (self.policy.lambda * self.aedl.get_bias(i));
            } else {
                scores[i] = score;
            }
        }

        let mut indexed_scores: Vec<(usize, f32)> = scores.into_iter().enumerate().collect();
        indexed_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        indexed_scores.into_iter().take(top_k).collect()
    }
}
