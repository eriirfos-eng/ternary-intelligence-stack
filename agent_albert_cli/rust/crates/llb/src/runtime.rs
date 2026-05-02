//! LLB Runtime — orchestrates the full atomic transaction.
//!
//! Flow:
//!   Gate 1 → [Snapshot] → [Capture dir state] → Execute → Gate 2 → Commit OR Rollback

use crate::{
    error::LlbError,
    gate1, gate2, snapshot,
    types::{LlbDecision, MutationRequest},
};
use std::path::Path;

/// The top-level entry point for all AI-agent filesystem mutations.
///
/// `action` receives the canonicalized, Gate-1-approved path and performs
/// the actual filesystem operation. It must be the only place that touches
/// the disk for this mutation.
///
/// # Returns
/// - `Ok(LlbDecision::Allow)` — transaction committed.
/// - `Ok(LlbDecision::Warn(msg))` — soft rejection (Gate 1 warning path).
/// - `Err(LlbError::HumanVeto)` — operator cancelled at HITL prompt.
/// - `Err(LlbError::IoccViolation{..})` — Gate 2 failed; rollback triggered.
/// - `Err(LlbError::RollbackFailed{..})` — PERMANENT PANIC: rollback failed.
pub fn execute<F>(req: MutationRequest, action: F) -> Result<LlbDecision, LlbError>
where
    F: FnOnce(&Path) -> Result<(), LlbError>,
{
    // ── Gate 1: Preflight ─────────────────────────────────────────────────
    let ticket = gate1::run(&req)?;

    // ── Capture dir state BEFORE snapshot (IOCC baseline) ────────────────
    let dir = ticket.resolved_path.parent().unwrap_or(Path::new("."));
    let dir_before = gate2::capture_dir_state(dir)?;

    // ── Snapshot (Tier 2+) — counted against dir_before by snap_delta ────
    let snap = if ticket.tier.requires_snapshot() && ticket.resolved_path.exists() {
        Some(snapshot::create(&ticket.resolved_path)?)
    } else {
        None
    };

    // ── Execute ───────────────────────────────────────────────────────────
    let exec_result = action(&ticket.resolved_path);

    match exec_result {
        Ok(()) => {
            // ── Gate 2: Postflight IOCC ───────────────────────────────────
            if ticket.tier.requires_gate2() {
                if let Err(iocc_err) = gate2::run(&ticket, &dir_before) {
                    // IOCC failure → rollback.
                    if let Some(ref s) = snap {
                        snapshot::restore(s).map_err(|re| LlbError::RollbackFailed {
                            reason: format!("IOCC failed ({iocc_err}) AND rollback failed: {re}"),
                        })?;
                    }
                    return Err(iocc_err);
                }
            }
            // Snapshot is discarded via Drop.
            Ok(LlbDecision::Allow)
        }
        Err(exec_err) => {
            // Execution failed → attempt rollback.
            if let Some(ref s) = snap {
                snapshot::restore(s).map_err(|re| LlbError::RollbackFailed {
                    reason: format!(
                        "execution failed ({exec_err}) AND rollback failed: {re}"
                    ),
                })?;
            }
            Err(exec_err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{MutationRequest, Operation, StructuredIntent};

    fn intent(goal: &str) -> StructuredIntent {
        StructuredIntent {
            goal: goal.into(),
            justification: "runtime test".into(),
            fallback_if_rejected: "skip".into(),
        }
    }

    fn isolated_dir(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("llb_rt_{tag}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn create_commit_allow() {
        let dir = isolated_dir("create");
        let path = dir.join("output.txt");

        let req = MutationRequest {
            path: path.to_str().unwrap().into(),
            operation: Operation::Create,
            intent: intent("create a test file"),
        };

        let decision = execute(req, |p| {
            std::fs::write(p, b"hello llb").map_err(LlbError::Io)
        });

        let _ = std::fs::remove_dir_all(&dir);
        assert!(matches!(decision, Ok(LlbDecision::Allow)), "got: {decision:?}");
    }

    #[test]
    fn overwrite_with_snapshot_and_commit() {
        let dir = isolated_dir("overwrite");
        let path = dir.join("target.txt");
        std::fs::write(&path, b"before").unwrap();

        let req = MutationRequest {
            path: path.to_str().unwrap().into(),
            operation: Operation::Overwrite,
            intent: intent("overwrite test file"),
        };

        let decision = execute(req, |p| {
            std::fs::write(p, b"after").map_err(LlbError::Io)
        });

        let content = std::fs::read(&path).unwrap_or_default();
        let _ = std::fs::remove_dir_all(&dir);
        assert!(matches!(decision, Ok(LlbDecision::Allow)), "got: {decision:?}");
        assert_eq!(content, b"after");
    }

    #[test]
    fn failed_action_triggers_rollback() {
        let dir = isolated_dir("rollback");
        let path = dir.join("precious.txt");
        std::fs::write(&path, b"original").unwrap();

        let req = MutationRequest {
            path: path.to_str().unwrap().into(),
            operation: Operation::Overwrite,
            intent: intent("overwrite then fail"),
        };

        let decision = execute(req, |p| {
            std::fs::write(p, b"corrupted").map_err(LlbError::Io)?;
            Err(LlbError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "simulated failure",
            )))
        });

        let content = std::fs::read(&path).unwrap_or_default();
        let _ = std::fs::remove_dir_all(&dir);
        assert!(decision.is_err());
        assert_eq!(content, b"original", "rollback should restore original content");
    }
}
