use crate::core::model_adapter::umil::cmir::UnifiedWeightTensor;
use anyhow::Result;

pub trait ModelStream {
    fn stream_weights(&self) -> Result<Box<dyn Iterator<Item = UnifiedWeightTensor>>>;
    fn stream_metadata(&self) -> Result<serde_json::Value>;
    fn stream_topology(&self) -> Result<serde_json::Value>;
}

pub trait ModelProviderPlugin: Send + Sync {
    fn initialize(&self) -> Result<()>;
    fn fetch_stream(&self, model_id: &str) -> Result<Box<dyn ModelStream>>;
    fn validate_format(&self, path: &str) -> Result<bool>;
}
