pub struct Platform;

impl Platform {
    pub fn new() -> Self {
        Self
    }

    pub fn load_model(&self, model_id: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn run_inference(&self, model: (), prompt: &str) -> anyhow::Result<String> {
        Ok("result".to_string())
    }
}
