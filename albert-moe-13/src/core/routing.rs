// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
//! # Statistical Expert Routing
//! 
//! Routes neural layers to specialized compression strategies based on 
//! their statistical properties.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExpertType {
    HighSparsity, // Aggressive compression (threshold ~0.5)
    Balanced,     // Mid compression (threshold ~0.3)
    Precision,    // Low error priority (threshold ~0.15)
}

#[derive(Debug, Clone, Copy)]
pub struct LayerStats {
    pub variance: f32,
    pub mean: f32,
    pub sparsity_estimate: f32,
}

pub struct DataRouter;

impl DataRouter {
    /// Analyzes the statistical distribution of weights.
    pub fn analyze_layer(weights: &[f32]) -> LayerStats {
        if weights.is_empty() {
            return LayerStats { variance: 0.0, mean: 0.0, sparsity_estimate: 0.0 };
        }

        let mean = weights.iter().sum::<f32>() / weights.len() as f32;
        let variance = weights.iter().map(|&w| (w - mean).powi(2)).sum::<f32>() / weights.len() as f32;
        
        // Estimate potential sparsity by counting near-zero weights (heuristic)
        let near_zero_count = weights.iter().filter(|&&w| w.abs() < 0.1).count();
        let sparsity_estimate = near_zero_count as f32 / weights.len() as f32;

        LayerStats { variance, mean, sparsity_estimate }
    }

    /// Routes a layer to an expert based on its statistics.
    pub fn route_layer(stats: &LayerStats) -> ExpertType {
        // Heuristic 1: High variance requires precision to maintain signal fidelity.
        if stats.variance > 0.5 {
            ExpertType::Precision
        } 
        // Heuristic 2: Many near-zero weights suggest high compressibility.
        else if stats.sparsity_estimate > 0.4 {
            ExpertType::HighSparsity
        }
        // Fallback: Balanced strategy.
        else {
            ExpertType::Balanced
        }
    }

    /// Maps expert type to its recommended threshold.
    pub fn get_threshold(expert: ExpertType) -> f32 {
        match expert {
            ExpertType::HighSparsity => 0.5,
            ExpertType::Balanced => 0.3,
            ExpertType::Precision => 0.15,
        }
    }
}
