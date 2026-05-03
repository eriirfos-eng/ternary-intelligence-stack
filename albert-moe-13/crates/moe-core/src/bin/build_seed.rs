//! # Build Seed Artifact
//! 
//! Generates the actual .tern.bin artifact for the Albert-Seed-250k model.

use moe_core::core::model_adapter::albert_seed::AlbertSeed250k;
use moe_core::training::checkpoint::CheckpointManager;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let model = AlbertSeed250k::new();
    let weights = model.initialize();
    
    let cm = CheckpointManager::new("./albert-moe-13/models/registry/albert-seed-250k");
    std::fs::create_dir_all("./albert-moe-13/models/registry/albert-seed-250k")?;
    
    cm.save_checkpoint("seed_weights", &weights).await?;
    println!("Artifact generated: albert-seed-250k/seed_weights.tern.bin");
    Ok(())
}
