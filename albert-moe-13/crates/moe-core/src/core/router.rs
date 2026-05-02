//! # Differentiable Router
//! 
//! Learned gating mechanism for MoE-13 dynamic expert specialization.

use crate::training::ternarization::TernarizationPipeline;

pub struct DifferentiableRouter {
    pub gate_weights: Vec<f32>, // Learnable gate weights (float)
    pub num_experts: usize,
    pub threshold: f32,
}

impl DifferentiableRouter {
    pub fn new(input_dim: usize, num_experts: usize, threshold: f32) -> Self {
        Self {
            gate_weights: vec![0.0; input_dim * num_experts],
            num_experts,
            threshold,
        }
    }

    /// Computes the gated routing scores for each expert.
    /// Uses STE-mapped weights for forward/backward consistency.
    pub fn route(&self, input: &[f32]) -> Vec<f32> {
        let input_dim = input.len();
        let mut scores = vec![0.0f32; self.num_experts];

        for e in 0..self.num_experts {
            let mut score = 0.0;
            for i in 0..input_dim {
                let w = self.gate_weights[e * input_dim + i];
                // Apply STE-ternarization to weight
                let tw = TernarizationPipeline::forward_ste(w, self.threshold) as f32;
                score += input[i] * tw;
            }
            scores[e] = score;
        }

        // Softmax gating
        let max_score = scores.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exps: Vec<f32> = scores.iter().map(|s| (s - max_score).exp()).collect();
        let sum: f32 = exps.iter().sum();
        exps.into_iter().map(|e| e / sum).collect()
    }
}
