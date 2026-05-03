//! # Differentiable Router
//! 
//! Learned gating mechanism for MoE-13 dynamic expert specialization.
//! Telemetry-enabled to track expert load balancing in real-time.

use crate::training::ternarization::TernarizationPipeline;
use std::sync::atomic::{AtomicUsize, Ordering};

// Global telemetry registry for expert routing
static EXPERT_LOAD: [AtomicUsize; 13] = [
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0),
    AtomicUsize::new(0),
];

pub struct DifferentiableRouter {
    pub gate_weights: Vec<f32>,
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

    /// Computes the gated routing scores for each expert and updates telemetry.
    pub fn route(&self, input: &[f32]) -> Vec<f32> {
        let input_dim = input.len();
        let mut scores = vec![0.0f32; self.num_experts];

        for e in 0..self.num_experts {
            let mut score = 0.0;
            for i in 0..input_dim {
                let w = self.gate_weights[e * input_dim + i];
                let tw = TernarizationPipeline::forward_ste(w, self.threshold) as f32;
                score += input[i] * tw;
            }
            scores[e] = score;
        }

        let max_score = scores.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exps: Vec<f32> = scores.iter().map(|s| (s - max_score).exp()).collect();
        let sum: f32 = exps.iter().sum();
        let probs: Vec<f32> = exps.into_iter().map(|e| e / sum).collect();

        // Update routing telemetry
        if let Some(best_expert) = probs.iter().enumerate().max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()) {
            if best_expert.0 < 13 {
                EXPERT_LOAD[best_expert.0].fetch_add(1, Ordering::Relaxed);
            }
        }

        probs
    }

    /// Returns current distribution of expert usage.
    pub fn get_telemetry() -> Vec<usize> {
        EXPERT_LOAD.iter().map(|c| c.load(Ordering::Relaxed)).collect()
    }
}
