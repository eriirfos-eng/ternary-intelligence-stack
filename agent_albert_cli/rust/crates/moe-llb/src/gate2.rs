//! Gate 2 — Post-execution Intent-Outcome Consistency Check (IOCC).
//!
//! Audits the filesystem bits *after* they have been flipped.
//! Three checks:
//!   1. Target integrity — mutation affected only the resolved path.
//!   2. Spillover detection — no unexpected files created or deleted nearby.
//!   3. Executable audit — new executables not declared in intent are flagged.

use crate::{error::LlbError, types::{DirState, ExecutionTicket, Operation}};
use std::{os::unix::fs::PermissionsExt, path::Path, time::SystemTime};

/// Capture a lightweight directory state for spillover comparison.
pub fn capture_dir_state(dir: &Path) -> Result<DirState, LlbError> {
    let meta = std::fs::metadata(dir).map_err(LlbError::Io)?;
    let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let entry_count = std::fs::read_dir(dir)
        .map_err(LlbError::Io)?
        .count();
    Ok(DirState { path: dir.to_path_buf(), entry_count, modified })
}

/// Run Gate 2 IOCC on a completed mutation.
pub fn run(ticket: &ExecutionTicket, dir_before: &DirState) -> Result<(), LlbError> {
    check_ticket_validity(ticket)?;
    check_target_integrity(ticket)?;
    check_spillover(ticket, dir_before)?;
    check_executable_audit(ticket)?;
    Ok(())
}

fn check_ticket_validity(ticket: &ExecutionTicket) -> Result<(), LlbError> {
    if !ticket.is_valid() {
        return Err(LlbError::TicketExpired);
    }
    Ok(())
}

fn check_target_integrity(ticket: &ExecutionTicket) -> Result<(), LlbError> {
    let path = &ticket.resolved_path;
    match ticket.operation {
        Operation::Delete => {
            if path.exists() {
                return Err(LlbError::IoccViolation {
                    detail: format!("DELETE declared but '{}' still exists", path.display()),
                });
            }
        }
        Operation::Create | Operation::Overwrite => {
            if !path.exists() {
                return Err(LlbError::IoccViolation {
                    detail: format!("{} declared but '{}' does not exist", ticket.operation, path.display()),
                });
            }
        }
        Operation::Chmod => {
            if !path.exists() {
                return Err(LlbError::IoccViolation {
                    detail: format!("CHMOD declared but '{}' does not exist", path.display()),
                });
            }
        }
    }
    Ok(())
}

fn check_spillover(ticket: &ExecutionTicket, before: &DirState) -> Result<(), LlbError> {
    let dir = ticket.resolved_path.parent().unwrap_or(Path::new("."));

    // If the directory itself is gone, that is certainly a spillover.
    if !dir.exists() {
        return Err(LlbError::IoccViolation {
            detail: format!("parent directory '{}' no longer exists after mutation", dir.display()),
        });
    }

    let after = capture_dir_state(dir)?;

    // Expected delta per operation.
    let expected_delta: i64 = match ticket.operation {
        Operation::Create => 1,
        Operation::Delete => -1,
        Operation::Overwrite | Operation::Chmod => 0,
    };

    let actual_delta = after.entry_count as i64 - before.entry_count as i64;

    // Allow for the snapshot file we may have created (one extra file).
    // The snapshot is in the same directory and has a `.llb_snap_*` suffix.
    let snap_delta: i64 = if ticket.tier.requires_snapshot() { 1 } else { 0 };
    let adjusted_actual = actual_delta - snap_delta;

    if adjusted_actual != expected_delta {
        return Err(LlbError::IoccViolation {
            detail: format!(
                "spillover detected: directory '{}' entry count changed by {} (expected {})",
                dir.display(),
                adjusted_actual,
                expected_delta
            ),
        });
    }

    Ok(())
}

fn check_executable_audit(ticket: &ExecutionTicket) -> Result<(), LlbError> {
    // Only relevant for CREATE/OVERWRITE.
    if !matches!(ticket.operation, Operation::Create | Operation::Overwrite) {
        return Ok(());
    }

    let path = &ticket.resolved_path;
    if !path.exists() {
        return Ok(());
    }

    let perms = path.metadata().map_err(LlbError::Io)?.permissions();
    let mode = perms.mode();

    // Flag if owner/group/other execute bits are set.
    if mode & 0o111 != 0 {
        return Err(LlbError::IoccViolation {
            detail: format!(
                "newly created/modified file '{}' has executable permissions (mode {:o}) \
                 that were not declared in the agent's intent",
                path.display(),
                mode & 0o777
            ),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tier::SafetyTier,
        types::{ExecutionTicket, Operation},
    };

    fn make_ticket(path: &str, op: Operation) -> ExecutionTicket {
        let p = std::path::PathBuf::from(path);
        let tier = SafetyTier::classify(&p, &op);
        ExecutionTicket::issue(p, op, tier, 0xdeadbeef)
    }

    fn isolated_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("llb_g2_{name}"));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn gate2_passes_after_create() {
        let dir = isolated_dir("create");
        let path = dir.join("target.txt");

        let before = capture_dir_state(&dir).unwrap();
        std::fs::write(&path, b"hello").unwrap();

        let ticket = make_ticket(path.to_str().unwrap(), Operation::Create);
        let result = run(&ticket, &before);
        let _ = std::fs::remove_dir_all(&dir);
        assert!(result.is_ok(), "Gate 2 failed: {result:?}");
    }

    #[test]
    fn gate2_fails_if_file_missing_after_create() {
        let dir = isolated_dir("missing");
        let path = dir.join("nonexistent.txt");
        let before = capture_dir_state(&dir).unwrap();

        // Don't create the file — Gate 2 should catch this.
        let ticket = make_ticket(path.to_str().unwrap(), Operation::Create);
        let result = run(&ticket, &before);
        let _ = std::fs::remove_dir_all(&dir);
        assert!(result.is_err());
    }
}
