use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlbError {
    #[error("path traversal detected in: {path}")]
    PathTraversal { path: String },

    #[error("path is on the blacklist: {path}")]
    BlacklistedPath { path: String },

    #[error("forbidden binary in command: {binary}")]
    ForbiddenBinary { binary: String },

    #[error("execution ticket expired or invalid")]
    TicketExpired,

    #[error("ticket path mismatch — ticket issued for {ticket_path}, called with {called_path}")]
    TicketMismatch {
        ticket_path: String,
        called_path: String,
    },

    #[error("snapshot creation failed for {path}: {reason}")]
    SnapshotFailed { path: String, reason: String },

    #[error("ROLLBACK FAILED — SYSTEM IN PERMANENT PANIC: {reason}")]
    RollbackFailed { reason: String },

    #[error("IOCC Gate 2 violation: {detail}")]
    IoccViolation { detail: String },

    #[error("human operator vetoed this operation")]
    HumanVeto,

    #[error(
        "file exceeds snapshot threshold ({size_mb:.1} MB > {limit_mb} MB limit): {path}"
    )]
    FileTooLargeForSnapshot {
        path: String,
        size_mb: f64,
        limit_mb: u64,
    },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
