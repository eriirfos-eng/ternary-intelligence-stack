//! # Representation Entropy Injection Mechanism
//!
//! Controlled stochastic and structural perturbations to prevent optimization collapse.

use rand::{distributions::Uniform, prelude::*, Rng};

pub struct EntropyInjector {
    pub noise_scale: f32,
    pub activation_dropout: f32,
    pub ternary_flip_probability: f32,
}

impl EntropyInjector {
    pub fn new(noise_scale: f32, activation_dropout: f32, ternary_flip_probability: f32) -> Self {
        Self {
            noise_scale,
            activation_dropout,
            ternary_flip_probability,
        }
    }

    /// Applies entropy injection: activation jitter and dropout.
    pub fn inject_entropy(&self, activations: &mut [f32]) {
        let mut rng = thread_rng();
        let uniform = Uniform::new(-self.noise_scale, self.noise_scale);
        for val in activations.iter_mut() {
            // Activation Jitter
            if self.noise_scale > 0.0 {
                let jitter: f32 = rng.sample(uniform);
                *val += jitter;
            }
            // Dropout
            if self.activation_dropout > 0.0 && rng.gen_bool(self.activation_dropout as f64) {
                *val = 0.0;
            }
        }
    }

    /// Probabilistic ternary flip at boundary.
    pub fn flip_ternary_boundary(&self, ternary_values: &mut [i8]) {
        if self.ternary_flip_probability <= 0.0 {
            return;
        }
        let mut rng = thread_rng();
        for val in ternary_values.iter_mut() {
            if rng.gen_bool(self.ternary_flip_probability as f64) {
                // Flip ternary state: -1 -> 1, 0 -> 1, 1 -> -1
                *val = match *val {
                    -1 => 1,
                    0 => 1,
                    1 => -1,
                    _ => *val,
                };
            }
        }
    }
}
