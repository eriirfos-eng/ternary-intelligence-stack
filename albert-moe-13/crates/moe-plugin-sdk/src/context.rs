use crate::types::Capability;

pub struct PluginContext {
    pub runtime_version: String,
    pub cmir_schema_version: u32,
    pub allowed_capabilities: Vec<Capability>,
}

impl PluginContext {
    pub fn has_capability(&self, cap: Capability) -> bool {
        self.allowed_capabilities.iter().any(|c| std::mem::discriminant(c) == std::mem::discriminant(&cap))
    }
}
