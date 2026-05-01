//! # Expert Mapper
//! 
//! Maps external model weights to internal ExpertBank13 slots.

use crate::core::model_adapter::ternary_converter::TernaryShard;

pub struct ExpertMappingPlan {
    pub mappings: Vec<usize>,
}

pub fn map_weights_to_experts(raw_weights: Vec<Vec<f32>>, expert_count: usize) -> Vec<TernaryShard> {
    // If MoE, map directly. If dense, use partitioning strategy (e.g., variance).
    raw_weights.into_iter().map(|w| {
        crate::core::model_adapter::ternary_converter::convert_to_ternary(&w, 0.1)
    }).collect()
}
