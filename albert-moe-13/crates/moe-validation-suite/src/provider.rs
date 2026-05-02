use anyhow::{Result, bail};
use moe_sdk::Platform;

pub struct ProviderAdapter {
    name: String,
}

impl ProviderAdapter {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    pub async fn validate_compatibility(&self, platform: &Platform) -> Result<()> {
        let test_input = "System check: validation_sequence_001";
        let model = platform.load_model(&self.name)?;
        let result = platform.run_inference(&model, test_input)?;

        if result.output_vec.is_empty() {
            bail!("Provider {} returned empty output for compatibility check.", self.name);
        }

        println!("Provider {} passed compatibility check: {}", self.name, result.routing_summary);
        Ok(())
    }
}
