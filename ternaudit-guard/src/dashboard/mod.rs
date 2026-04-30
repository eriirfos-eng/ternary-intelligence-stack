use serde::{Serialize, Deserialize};
use std::sync::Arc;

/// Represents a real-time audit event from the MCP layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp: String,
    pub agent_id: String,
    pub decision_path: Vec<String>,
    pub outcome: String,
    pub compliance_score: f32,
}

/// Interface for streaming audit events to a visualization front-end.
pub trait AuditStreamer {
    fn stream_event(&self, event: AuditEvent) -> Result<(), String>;
}

/// Placeholder for the WebSocket/Dashboard integration engine.
pub struct DashboardManager {
    pub connection_pool: Arc<std::sync::Mutex<Vec<String>>>,
}

impl DashboardManager {
    pub fn new() -> Self {
        Self {
            connection_pool: Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
}
