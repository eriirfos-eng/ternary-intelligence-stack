use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};

// ── Operation ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    Create,
    Overwrite,
    Delete,
    Chmod,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create => write!(f, "CREATE"),
            Self::Overwrite => write!(f, "OVERWRITE"),
            Self::Delete => write!(f, "DELETE"),
            Self::Chmod => write!(f, "CHMOD"),
        }
    }
}

// ── StructuredIntent ──────────────────────────────────────────────────────────

/// Mandatory structured justification that the agent must supply with every
/// mutation request. Deliberately forces the model to articulate its plan
/// before the physical gate is evaluated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredIntent {
    pub goal: String,
    pub justification: String,
    pub fallback_if_rejected: String,
}

// ── MutationRequest ───────────────────────────────────────────────────────────

/// The only sanctioned entry point for an AI agent to request a filesystem
/// mutation. Corresponds to the MCP tool `request_filesystem_mutation`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRequest {
    pub path: String,
    pub operation: Operation,
    pub intent: StructuredIntent,
}

// ── LlbDecision ──────────────────────────────────────────────────────────────

/// Ternary gate verdict — maps to TIS trit values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlbDecision {
    /// +1 — Allowed and committed.
    Allow,
    /// 0  — Soft block. Agent receives a rethink prompt.
    Warn(String),
    /// -1 — Hard veto. Rollback triggered; agent thread terminated.
    HardVeto,
}

impl LlbDecision {
    #[must_use]
    pub fn trit_value(&self) -> i8 {
        match self {
            Self::Allow => 1,
            Self::Warn(_) => 0,
            Self::HardVeto => -1,
        }
    }
}

// ── ExecutionTicket ───────────────────────────────────────────────────────────

const TICKET_TTL: Duration = Duration::from_secs(30);

/// Ephemeral cryptographic token issued by Gate 1.
/// Every mutation syscall must present a valid, unexpired ticket.
#[derive(Debug, Clone)]
pub struct ExecutionTicket {
    pub id: String,
    pub resolved_path: PathBuf,
    pub operation: Operation,
    pub tier: crate::tier::SafetyTier,
    issued_at: Instant,
}

impl ExecutionTicket {
    pub(crate) fn issue(
        resolved_path: PathBuf,
        operation: Operation,
        tier: crate::tier::SafetyTier,
        nonce: u64,
    ) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(resolved_path.to_string_lossy().as_bytes());
        hasher.update(operation.to_string().as_bytes());
        hasher.update(nonce.to_string().as_bytes());
        hasher.update(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
                .to_string()
                .as_bytes(),
        );
        let id = hex::encode(hasher.finalize());
        Self { id, resolved_path, operation, tier, issued_at: Instant::now() }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issued_at.elapsed() < TICKET_TTL
    }
}

// ── SnapshotHandle ────────────────────────────────────────────────────────────

const SNAPSHOT_SIZE_LIMIT_MB: u64 = 50;

/// Backup of a file created before a Tier 2/3 mutation.
/// Enables atomic rollback if Gate 2 fails or execution panics.
pub struct SnapshotHandle {
    pub original_path: PathBuf,
    pub snap_path: PathBuf,
    pub is_content_backed: bool,
    pub size_bytes: u64,
}

impl SnapshotHandle {
    #[must_use]
    pub fn size_limit_mb() -> u64 {
        SNAPSHOT_SIZE_LIMIT_MB
    }
}

impl Drop for SnapshotHandle {
    fn drop(&mut self) {
        if self.snap_path.exists() {
            let _ = std::fs::remove_file(&self.snap_path);
        }
    }
}

// ── DirState ──────────────────────────────────────────────────────────────────

/// Lightweight snapshot of a directory for spillover detection (O(1)).
#[derive(Debug, Clone)]
pub struct DirState {
    pub path: PathBuf,
    pub entry_count: usize,
    pub modified: SystemTime,
}
