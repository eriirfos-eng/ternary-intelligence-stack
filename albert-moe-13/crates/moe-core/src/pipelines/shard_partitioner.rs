//! # Dataset Shard Partitioner
//! 
//! Utility to split massive raw corpora into ingestion-ready .shard files.

use anyhow::Result;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub struct ShardPartitioner {
    pub shard_size: usize, // Lines per shard
}

impl ShardPartitioner {
    pub fn new(shard_size: usize) -> Self {
        Self { shard_size }
    }

    pub fn partition(&self, input_path: &Path, output_dir: &Path) -> Result<()> {
        std::fs::create_dir_all(output_dir)?;
        let file = File::open(input_path)?;
        let reader = BufReader::new(file);
        
        let mut shard_idx = 0;
        let mut count = 0;
        let mut out = OpenOptions::new()
            .create(true)
            .write(true)
            .open(output_dir.join(format!("shard_{}.shard", shard_idx)))?;

        for line in reader.lines() {
            if count >= self.shard_size {
                shard_idx += 1;
                count = 0;
                out = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(output_dir.join(format!("shard_{}.shard", shard_idx)))?;
            }
            out.write_all(line?.as_bytes())?;
            out.write_all(b"\n")?;
            count += 1;
        }
        Ok(())
    }
}
