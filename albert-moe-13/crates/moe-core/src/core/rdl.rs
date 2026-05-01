//! # Representation Divergence Layer (RDL)
//! 
//! Applies expert-specific deformations to float-level activations before they 
//! undergo ternary compression, enabling distinct manifold pathways for experts.

use rand::prelude::*;

pub struct RepresentationDivergenceLayer {
    pub basis_vectors: Vec<Vec<f32>>, // 13 experts, each with a basis vector
    pub divergence_strengths: Vec<f32>,
}

impl RepresentationDivergenceLayer {
    pub fn new(input_dim: usize) -> Self {
        let mut rng = thread_rng();
        let basis_vectors = (0..13)
            .map(|_| (0..input_dim).map(|_| rng.gen_range(-0.1..0.1)).collect())
            .collect();
        let divergence_strengths = vec![0.5; 13];
        
        Self {
            basis_vectors,
            divergence_strengths,
        }
    }

    /// Deforms raw float activations based on expert index and task embedding.
    pub fn diverge(&self, expert_idx: usize, input: &[f32], task_embedding: &[f32]) -> Vec<f32> {
        let basis = &self.basis_vectors[expert_idx];
        let strength = self.divergence_strengths[expert_idx];
        
        // Simple projection of task embedding to scale the divergence
        let task_scale: f32 = task_embedding.iter().sum::<f32>() / task_embedding.len() as f32;
        
        input.iter().enumerate().map(|(i, &x)| {
            x + (basis[i] * strength * task_scale)
        }).collect()
    }
}
