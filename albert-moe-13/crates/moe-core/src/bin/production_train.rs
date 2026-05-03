//! # Production Training Execution
//! 
//! Launch script for large-scale Albert-1B experiments.

use moe_core::core::model_adapter::albert_1b::Albert1B;
use moe_core::training::controller::TrainingController;
use moe_core::training::distributed::DistributedOrchestrator;
use moe_core::training::checkpoint::CheckpointManager;
use moe_core::pipelines::data_pipeline::DataPipeline;
use moe_core::training::config::TrainingConfig;
use moe_core::training::harness::TrainingHarness;
use anyhow::Result;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    println!("--- Initializing Production-Scale Training Harness ---");
    
    // 1. Setup Components
    let orch = DistributedOrchestrator::new(0, 1);
    let pipe = DataPipeline::new(1024 * 1024 * 128); // 128MB buffers
    let cm = CheckpointManager::new("./albert-moe-13/models/registry/albert-v1");
    let controller = TrainingController::new(orch, pipe, cm);
    
    let model = Albert1B::new();
    let config = TrainingConfig::default();
    
    // 2. Initialize Harness
    let harness = TrainingHarness::new(model, controller, config);
    
    // 3. Execution
    let dataset = Path::new("training_data.shard");
    println!("Starting multi-epoch training sweep...");
    
    // In a real run, this would stream through petabytes.
    // For this experiment, we execute the production harness.
    harness.execute_run(dataset).await?;
    
    println!("--- Training Run Successfully Finished ---");
    Ok(())
}
