//! # Ternary Checkpoint Manager
//! 
//! Handles persistence of sparse ternary MoE parameters.
//! Implements bit-packed storage (5 trits per byte) to minimize I/O for 1T+ models.

use anyhow::{Result, Context};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct CheckpointManager {
    pub base_path: String,
}

impl CheckpointManager {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
        }
    }

    /// Saves the ternary state (weights/experts) to a bit-packed binary file.
    pub async fn save_checkpoint(&self, model_id: &str, weights: &[i8]) -> Result<()> {
        let path = format!("{}/{}.tern.bin", self.base_path, model_id);
        let mut file = File::create(&path).await.context("Failed to create checkpoint file")?;
        
        // Packing logic: Map {-1, 0, 1} to {0, 1, 2} then pack 5 trits into 1 byte (3^5 = 243 < 256)
        let mut packed = Vec::with_capacity(weights.len() / 5 + 1);
        for chunk in weights.chunks(5) {
            let mut byte: u8 = 0;
            for (i, &trit) in chunk.iter().enumerate() {
                let val = (trit + 1) as u8; // Map {-1, 0, 1} -> {0, 1, 2}
                byte += val * 3u8.pow(i as u32);
            }
            packed.push(byte);
        }
        
        file.write_all(&packed).await.context("Failed to write checkpoint data")?;
        Ok(())
    }

    /// Loads a ternary checkpoint from storage.
    pub async fn load_checkpoint(&self, model_id: &str) -> Result<Vec<i8>> {
        let path = format!("{}/{}.tern.bin", self.base_path, model_id);
        let mut file = File::open(&path).await.context("Failed to open checkpoint file")?;
        let mut packed = Vec::new();
        file.read_to_end(&mut packed).await?;
        
        let mut weights = Vec::new();
        for byte in packed {
            let mut val = byte;
            for _ in 0..5 {
                let trit = (val % 3) as i8 - 1;
                weights.push(trit);
                val /= 3;
            }
        }
        Ok(weights)
    }
}
