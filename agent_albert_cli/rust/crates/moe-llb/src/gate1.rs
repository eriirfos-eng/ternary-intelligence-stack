//! Gate 1 — Preflight validation.
//!
//! Resolves the path, enforces blacklists, classifies the safety tier,
//! optionally triggers HITL for Tier 3, and issues the ExecutionTicket.

use crate::{
    blacklist, error::LlbError, hitl, tier::SafetyTier, types::{ExecutionTicket, MutationRequest},
};
use std::path::Path;

/// Run Gate 1 preflight on a `MutationRequest`.
/// Returns an `ExecutionTicket` if the request is authorized.
pub fn run(req: &MutationRequest) -> Result<ExecutionTicket, LlbError> {
    // 1. Canonicalize — resolves symlinks, strips `..`, makes path absolute.
    //    If the file doesn't exist yet (CREATE), we canonicalize the parent.
    let resolved = resolve_path(&req.path, &req.operation)?;

    // 2. Check path blacklist.
    blacklist::check_path(&resolved)?;

    // 3. Classify safety tier.
    let tier = SafetyTier::classify(&resolved, &req.operation);

    // 4. HITL confirmation for T3/DELETE.
    if tier.requires_hitl() {
        hitl::confirm(req, &resolved, tier)?;
    }

    // 5. Issue cryptographic ticket.
    let nonce = pseudo_nonce();
    Ok(ExecutionTicket::issue(resolved, req.operation.clone(), tier, nonce))
}

fn resolve_path(raw: &str, op: &crate::types::Operation) -> Result<std::path::PathBuf, LlbError> {
    // Reject obviously suspicious patterns before hitting the filesystem.
    if raw.contains("..") {
        return Err(LlbError::PathTraversal { path: raw.to_string() });
    }

    let path = Path::new(raw);

    if path.exists() {
        return std::fs::canonicalize(path).map_err(LlbError::Io);
    }

    // For CREATE operations on non-existent files, canonicalize the parent.
    if *op == crate::types::Operation::Create {
        let parent = path.parent().unwrap_or(Path::new("."));
        let canon_parent = if parent.exists() {
            std::fs::canonicalize(parent).map_err(LlbError::Io)?
        } else {
            parent.to_path_buf()
        };
        let file_name = path.file_name().ok_or_else(|| LlbError::PathTraversal {
            path: raw.to_string(),
        })?;
        return Ok(canon_parent.join(file_name));
    }

    // File does not exist and op is not CREATE — reject.
    Err(LlbError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("target path does not exist: {raw}"),
    )))
}

fn pseudo_nonce() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64 ^ (d.as_secs() << 32))
        .unwrap_or(0xdeadbeef)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{MutationRequest, Operation, StructuredIntent};

    fn intent() -> StructuredIntent {
        StructuredIntent {
            goal: "test".into(),
            justification: "unit test".into(),
            fallback_if_rejected: "abort".into(),
        }
    }

    #[test]
    fn traversal_is_rejected() {
        let req = MutationRequest {
            path: "/tmp/../etc/passwd".into(),
            operation: Operation::Overwrite,
            intent: intent(),
        };
        assert!(run(&req).is_err());
    }

    #[test]
    fn blacklisted_path_is_rejected() {
        let req = MutationRequest {
            path: "/etc/hosts".into(),
            operation: Operation::Overwrite,
            intent: intent(),
        };
        assert!(run(&req).is_err());
    }

    #[test]
    fn valid_create_in_tmp_issues_ticket() {
        let req = MutationRequest {
            path: "/tmp/llb_test_gate1.txt".into(),
            operation: Operation::Create,
            intent: intent(),
        };
        let result = run(&req);
        assert!(result.is_ok(), "expected ticket, got: {result:?}");
    }
}
