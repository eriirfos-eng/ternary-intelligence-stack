use anyhow::Result;
use crate::context::PluginContext;
use crate::types::PluginType;
use moe_core::model_adapter::umil::cmir::CMIR;
use moe_core::model_adapter::umil::traits::ModelStream;

pub trait MoePlugin: Send + Sync {
    fn id(&self) -> &str;
    fn version(&self) -> semver::Version;
    fn plugin_type(&self) -> PluginType;

    fn initialize(&mut self, context: &PluginContext) -> Result<()>;

    fn ingest(&self, stream: Box<dyn ModelStream>) -> Result<CMIR>;

    fn shutdown(&self);
}
