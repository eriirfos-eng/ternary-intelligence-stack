use crate::core::routing::ExpertBank13;
use ndarray::ArrayD;

pub struct UnifiedWeightTensor {
    pub data: ArrayD<f32>,
    pub name: String,
}

pub struct CMIR {
    pub expert_bank: ExpertBank13,
    pub ternary_weights: Vec<i8>,
    pub routing_map: serde_json::Value,
    pub metadata: CMIRMetadata,
}

pub struct CMIRMetadata {
    pub provenance: String,
    pub provider_trace: String,
    pub version: String,
}
