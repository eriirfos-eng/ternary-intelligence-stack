use serde::{Deserialize, Serialize};
use crate::types::{PluginType, Capability};

#[derive(Serialize, Deserialize)]
pub struct PluginHeader {
    pub name: String,
    pub version: semver::Version,
    pub plugin_type: PluginType,
    pub target_runtime: String,
    pub cmir_version: u32,
}

#[derive(Serialize, Deserialize)]
pub struct PluginManifest {
    pub required_capabilities: Vec<Capability>,
    pub forbidden_capabilities: Vec<Capability>,
    pub deterministic: bool,
    pub offline_only: bool,
    pub execution_timeout_ms: u64,
}

pub struct SPF13 {
    pub header: PluginHeader,
    pub manifest: PluginManifest,
    pub payload: Vec<u8>,
    pub signature: [u8; 64], // ed25519 signature
    pub checksum: [u8; 32],  // sha256 hash
}
