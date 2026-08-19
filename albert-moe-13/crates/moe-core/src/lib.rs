//! Internal core engine for MoE-13 ternary inference and routing. Part of the Ternary Intelligence Stack.
pub mod core;
pub mod experts;
pub mod training;
pub mod pipelines;
pub mod cli;
pub mod model_adapter;
pub mod inference;
pub mod benchmarks;

pub use crate::core::ternary_mapper::TernaryMapper;
pub use crate::training::ternarization::TernarizationPipeline;
pub use crate::model_adapter::*;

pub fn run_mock_ternarization() {
    TernarizationPipeline::run_mock_ternarization();
}
