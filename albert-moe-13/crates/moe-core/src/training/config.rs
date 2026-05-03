//! # Training Configuration
//! 
//! Defines the training sweep parameters for Albert-1B.

pub struct TrainingConfig {
    pub learning_rate: f32,
    pub threshold: f32,
    pub batch_size: usize,
}

impl TrainingConfig {
    pub fn default() -> Self {
        Self {
            learning_rate: 1e-4,
            threshold: 0.5,
            batch_size: 32,
        }
    }
}
