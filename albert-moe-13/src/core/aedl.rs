//! # Adaptive Expert Drift Layer (AEDL)
//! 
//! Implements controlled, long-term adaptation of expert routing priors
//! using performance-based reinforcement signals, maintaining strict
//! architectural bounds on drift and emergence.

pub struct AEDL {
    pub routing_prior_bias: Vec<f32>,
    pub expert_success: Vec<f32>,
    pub alpha: f32, // EMA smoothing factor
    pub entropy_floor: f32,
}

impl AEDL {
    pub fn new(num_experts: usize) -> Self {
        Self {
            routing_prior_bias: vec![0.0; num_experts],
            expert_success: vec![0.5; num_experts],
            alpha: 0.01,
            entropy_floor: 0.2,
        }
    }

    /// Update routing biases based on observed performance reward (0.0 - 1.0).
    pub fn update(&mut self, expert_id: usize, reward: f32) {
        // Update success signal with EMA
        self.expert_success[expert_id] = (1.0 - self.alpha) * self.expert_success[expert_id] + self.alpha * reward;
        
        // Update prior bias with clipping
        let delta = self.alpha * (reward - 0.5);
        self.routing_prior_bias[expert_id] = (self.routing_prior_bias[expert_id] + delta).clamp(-1.0, 1.0);
    }

    pub fn get_bias(&self, expert_id: usize) -> f32 {
        self.routing_prior_bias[expert_id]
    }
}
