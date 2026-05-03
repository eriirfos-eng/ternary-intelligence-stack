//! # Dataset Ingestion Utility
//! 
//! Simple binary to run the harvester and partitioner in sequence.

use moe_core::pipelines::dataset_harvester::DatasetHarvester;
use moe_core::pipelines::shard_partitioner::ShardPartitioner;
use anyhow::Result;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let harvester = DatasetHarvester::new("./data/corpora/raw");
    let partitioner = ShardPartitioner::new(1000); // 1000 lines per shard

    println!("Fetching corpus...");
    let raw_path = harvester.fetch_shard("https://www.gutenberg.org/files/10/10-0.txt", "bible.txt").await?;
    
    println!("Partitioning corpus...");
    partitioner.partition(Path::new(&raw_path), Path::new("./data/corpora/shards"))?;

    println!("Dataset ready in ./data/corpora/shards");
    Ok(())
}
