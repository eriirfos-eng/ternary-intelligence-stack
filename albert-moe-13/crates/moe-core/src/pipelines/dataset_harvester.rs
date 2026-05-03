//! # Dataset Harvester
//! 
//! Automated pipeline for fetching and preprocessing open-source
//! training corpora (Project Gutenberg, Wikipedia, etc.).

use anyhow::Result;
use std::fs;
use std::path::Path;
use reqwest;

pub struct DatasetHarvester {
    pub download_dir: String,
}

impl DatasetHarvester {
    pub fn new(download_dir: &str) -> Self {
        fs::create_dir_all(download_dir).unwrap();
        Self { download_dir: download_dir.to_string() }
    }

    /// Fetches a dataset shard from a public source using reqwest.
    pub async fn fetch_shard(&self, source_url: &str, shard_name: &str) -> Result<String> {
        let dest = format!("{}/{}", self.download_dir, shard_name);
        println!("Fetching dataset shard from {}...", source_url);
        
        // Fetch the raw text corpus
        let response = reqwest::get(source_url).await?.text().await?;
        
        fs::write(&dest, response)?;
        Ok(dest)
    }
}
