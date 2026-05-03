//! # Data Pipeline V2
//! 
//! High-performance streaming ingestion and preprocessing for MoE-13.
//! Designed for petabyte-scale training clusters using async ring-buffer prefetching.

use anyhow::Result;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::sync::mpsc;
use std::path::{Path, PathBuf};

pub struct DataPipeline {
    pub buffer_size: usize,
    pub prefetch_depth: usize,
}

impl DataPipeline {
    pub fn new(buffer_size: usize) -> Self {
        Self { buffer_size, prefetch_depth: 4 }
    }

    /// Asynchronously streams shards from disk using zero-copy buffers.
    pub async fn ingest_stream(&self, path: &Path) -> Result<Vec<u8>> {
        let file = File::open(path).await?;
        let mut reader = BufReader::with_capacity(self.buffer_size, file);
        let mut buffer = Vec::with_capacity(self.buffer_size);
        reader.read_to_end(&mut buffer).await?;
        Ok(buffer)
    }

    /// Launches a background prefetcher that eagerly loads shards into a ring buffer.
    /// This ensures the TrainingHarness never waits for disk I/O.
    pub fn start_prefetch_loop(&self, shard_paths: Vec<PathBuf>) -> mpsc::Receiver<Result<Vec<u8>>> {
        let (tx, rx) = mpsc::channel(self.prefetch_depth);
        let buffer_size = self.buffer_size;

        tokio::spawn(async move {
            for path in shard_paths {
                let file = match File::open(&path).await {
                    Ok(f) => f,
                    Err(e) => {
                        let _ = tx.send(Err(e.into())).await;
                        break;
                    }
                };
                let mut reader = BufReader::with_capacity(buffer_size, file);
                let mut buffer = Vec::with_capacity(buffer_size);
                
                if let Err(e) = reader.read_to_end(&mut buffer).await {
                    let _ = tx.send(Err(e.into())).await;
                    break;
                }
                
                if tx.send(Ok(buffer)).await.is_err() {
                    break; // Receiver dropped, stop prefetching
                }
            }
        });

        rx
    }
}
