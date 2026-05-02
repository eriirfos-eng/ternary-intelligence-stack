//! Snapshot engine — creates an undo point before Tier 2/3 mutations.

use crate::{error::LlbError, types::SnapshotHandle};
use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

const SNAP_SUFFIX: &str = ".llb_snap";

/// Create a snapshot of `path` before mutation.
///
/// For files ≤ 50 MB: full binary copy.
/// For files > 50 MB: metadata only — prints a mandatory warning.
pub fn create(path: &Path) -> Result<SnapshotHandle, LlbError> {
    let size_bytes = path.metadata().map(|m| m.len()).unwrap_or(0);
    let limit_bytes = SnapshotHandle::size_limit_mb() * 1024 * 1024;
    let snap_path = snap_path_for(path);

    if size_bytes > limit_bytes {
        let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
        eprintln!(
            "\n\x1b[33;1m[LLB WARNING]\x1b[0m CAUTION: This operation is NOT fully reversible.\n\
             File '{path_display}' is {size_mb:.1} MB — exceeds the {limit_mb} MB snapshot threshold.\n\
             Content backup SKIPPED. Only metadata is recorded.\n",
            path_display = path.display(),
            limit_mb = SnapshotHandle::size_limit_mb()
        );
        return Ok(SnapshotHandle {
            original_path: path.to_path_buf(),
            snap_path,
            is_content_backed: false,
            size_bytes,
        });
    }

    std::fs::copy(path, &snap_path).map_err(|e| LlbError::SnapshotFailed {
        path: path.display().to_string(),
        reason: e.to_string(),
    })?;

    Ok(SnapshotHandle {
        original_path: path.to_path_buf(),
        snap_path,
        is_content_backed: true,
        size_bytes,
    })
}

/// Restore the original file from the snapshot.
/// Called when Gate 2 fails or execution panics.
pub fn restore(handle: &SnapshotHandle) -> Result<(), LlbError> {
    if !handle.is_content_backed {
        return Err(LlbError::RollbackFailed {
            reason: format!(
                "no content snapshot for '{}' (exceeded {} MB limit)",
                handle.original_path.display(),
                SnapshotHandle::size_limit_mb()
            ),
        });
    }

    if !handle.snap_path.exists() {
        return Err(LlbError::RollbackFailed {
            reason: format!("snapshot file '{}' has vanished", handle.snap_path.display()),
        });
    }

    std::fs::copy(&handle.snap_path, &handle.original_path)
        .map_err(|e| LlbError::RollbackFailed { reason: e.to_string() })?;

    Ok(())
}

fn snap_path_for(original: &Path) -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let name = format!(
        "{}{}_{ts}",
        original.file_name().unwrap_or_default().to_string_lossy(),
        SNAP_SUFFIX
    );
    original.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_and_restore_roundtrip() {
        let dir = std::env::temp_dir();
        let path = dir.join("llb_snap_test.txt");
        std::fs::write(&path, b"original content").unwrap();

        let snap = create(&path).unwrap();
        assert!(snap.is_content_backed);

        std::fs::write(&path, b"corrupted").unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"corrupted");

        restore(&snap).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"original content");

        let _ = std::fs::remove_file(&path);
    }
}
