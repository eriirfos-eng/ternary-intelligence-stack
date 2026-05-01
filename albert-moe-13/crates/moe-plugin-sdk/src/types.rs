use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PluginType {
    ModelProvider,
    RoutingExtension,
    PreProcessor,
    PostProcessor,
    Diagnostic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Capability {
    ReadModelStream,
    EmitCMIR,
    RegisterMetrics,
    AccessMetadataOnly,
}
