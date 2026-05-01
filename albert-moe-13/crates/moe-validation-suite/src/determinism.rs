use anyhow::{Result, bail};
use moe_sdk::Platform;

pub struct DeterminismHarness {
    iterations: usize,
}

impl DeterminismHarness {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    pub async fn run_test(&self, input: &str, platform: &Platform) -> Result<Vec<String>> {
        let mut results = Vec::new();

        for i in 0..self.iterations {
            let output = platform.run_inference((), input)?;
            results.push(output);
            
            if i > 0 && results[i] != results[0] {
                bail!("Determinism violation at iteration {}: output does not match first run.", i);
            }
        }
        
        Ok(results)
    }
}
