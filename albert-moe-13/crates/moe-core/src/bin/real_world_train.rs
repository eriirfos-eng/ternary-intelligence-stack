//! # Real-World Training Experiment
//! 
//! Orchestrates the end-to-end flow: Fetch dataset -> Ingest -> Train -> Checkpoint.

use moe_core::core::model_adapter::albert_1b::Albert1B;
use moe_core::training::controller::TrainingController;
use moe_core::training::distributed::DistributedOrchestrator;
use moe_core::training::checkpoint::CheckpointManager;
use moe_core::pipelines::data_pipeline::DataPipeline;
use moe_core::pipelines::dataset_harvester::DatasetHarvester;
use moe_core::training::config::TrainingConfig;
use moe_core::training::harness::TrainingHarness;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("--- Initializing Frontier-Scale Training Loop ---");

    // 1. Data Ingestion (Project Gutenberg Example)
    let harvester = DatasetHarvester::new("./data/corpus");
    let dataset_url = "https://www.gutenberg.org/files/11/11-0.txt"; // Alice's Adventures in Wonderland
    let dataset_path = harvester.fetch_shard(dataset_url, "alice.txt").await?;
    println!("Corpus acquired: {}", dataset_path);

    // 2. Training Infrastructure
    let orch = DistributedOrchestrator::new(0, 1);
    let pipe = DataPipeline::new(1024 * 1024 * 64);
    let cm = CheckpointManager::new("./albert-moe-13/models/registry/albert-v1");
    let controller = TrainingController::new(orch, pipe, cm);
    
    let model = Albert1B::new();
    let config = TrainingConfig::default();
    
    // 3. Harness
    let harness = TrainingHarness::new(model, controller, config);
    
    // 4. Execute Training
    println!("Starting ternary manifold optimization...");
    harness.execute_run(std::path::Path::new(&dataset_path)).await?;
    
    println!("--- Training Run Successfully Finished ---");
    Ok(())
}
