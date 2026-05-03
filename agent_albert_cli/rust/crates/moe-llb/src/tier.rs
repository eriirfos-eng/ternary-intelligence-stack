use crate::types::Operation;
use std::path::Path;

/// Safety tier determines which gates, snapshots, and confirmations apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetyTier {
    /// T0 — READ: Lowest risk. Canonicalization only.
    T0Read = 0,
    /// T1 — CREATE: Moderate risk. Escalates to T2 for executable files.
    T1Create = 1,
    /// T2 — MODIFY/OVERWRITE: High risk. Mandatory snapshot + Gate 2.
    T2Modify = 2,
    /// T3 — DELETE: Critical. Mandatory snapshot + HITL + Gate 2.
    T3Delete = 3,
}

impl std::fmt::Display for SafetyTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::T0Read => write!(f, "T0/READ"),
            Self::T1Create => write!(f, "T1/CREATE"),
            Self::T2Modify => write!(f, "T2/MODIFY"),
            Self::T3Delete => write!(f, "T3/DELETE — CRITICAL"),
        }
    }
}

/// Extensions that escalate a CREATE to T2 (executable-like artifacts).
const EXEC_EXTENSIONS: &[&str] = &[
    "sh", "bash", "zsh", "fish", "py", "rb", "pl", "js", "ts", "bin", "exe", "elf", "so",
    "dylib", "dll",
];

impl SafetyTier {
    #[must_use]
    pub fn classify(path: &Path, op: &Operation) -> Self {
        match op {
            Operation::Delete => Self::T3Delete,
            Operation::Overwrite | Operation::Chmod => Self::T2Modify,
            Operation::Create => {
                if is_executable_extension(path) {
                    Self::T2Modify
                } else {
                    Self::T1Create
                }
            }
        }
    }

    #[must_use]
    pub fn requires_snapshot(self) -> bool {
        self >= Self::T2Modify
    }

    #[must_use]
    pub fn requires_hitl(self) -> bool {
        self >= Self::T3Delete
    }

    #[must_use]
    pub fn requires_gate2(self) -> bool {
        self >= Self::T2Modify
    }
}

fn is_executable_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|ext| EXEC_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn delete_is_always_t3() {
        assert_eq!(
            SafetyTier::classify(&PathBuf::from("/tmp/x.txt"), &Operation::Delete),
            SafetyTier::T3Delete
        );
    }

    #[test]
    fn create_script_escalates_to_t2() {
        assert_eq!(
            SafetyTier::classify(&PathBuf::from("/tmp/setup.sh"), &Operation::Create),
            SafetyTier::T2Modify
        );
    }

    #[test]
    fn create_plain_file_is_t1() {
        assert_eq!(
            SafetyTier::classify(&PathBuf::from("/tmp/notes.md"), &Operation::Create),
            SafetyTier::T1Create
        );
    }

    #[test]
    fn ordering_is_correct() {
        assert!(SafetyTier::T3Delete > SafetyTier::T2Modify);
        assert!(SafetyTier::T2Modify > SafetyTier::T1Create);
        assert!(SafetyTier::T1Create > SafetyTier::T0Read);
    }
}
