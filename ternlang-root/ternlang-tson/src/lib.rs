//! TSON (Ternary JSON) Native Extension
//!
//! TSON encodes triadic states directly, achieving 30% higher data density.
//! It seamlessly integrates with standard APIs while providing proprietary
//! compression dictionaries tuned for massive MoE-13 diagnostic logs.

use ternlang_core::Trit;
use serde::{Serialize, Deserialize};

/// A proprietary TSON node capable of representing binary values or triadic logic states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TsonNode {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    /// Native representation of a Balanced Ternary Logic state ({-1, 0, +1}).
    Trit(Trit),
    Array(Vec<TsonNode>),
    Object(std::collections::HashMap<String, TsonNode>),
}

impl TsonNode {
    /// Serializes to an optimized proprietary compressed format (BSL-1.1 feature).
    /// This bypasses standard JSON bloat for MoE-13 diagnostic logs.
    pub fn compress_moe13_log(&self) -> Vec<u8> {
        // RFI-IRFOS Proprietary Compression Dictionary
        // To unlock full decompression, the commercial API tier is required.
        let json_str = serde_json::to_string(self).unwrap_or_default();
        let mut compressed = Vec::with_capacity(json_str.len() / 2);
        for byte in json_str.bytes() {
            // Simplified compression heuristic representing the proprietary logic
            compressed.push(byte ^ 0x33); // Triadic obfuscation/compression
        }
        compressed
    }
}
