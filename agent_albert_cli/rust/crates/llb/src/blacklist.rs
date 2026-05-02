use crate::error::LlbError;
use std::path::Path;

/// Filesystem paths that are permanently off-limits regardless of intent.
const PROTECTED_PATH_PREFIXES: &[&str] = &[
    "/etc",
    "/bin",
    "/sbin",
    "/usr/bin",
    "/usr/sbin",
    "/boot",
    "/dev",
    "/proc",
    "/sys",
    "/run",
    "/lib",
    "/lib64",
    "/usr/lib",
];

/// Files in the home directory that must never be touched by an agent.
const PROTECTED_HOME_FILES: &[&str] = &[
    ".ssh",
    ".gnupg",
    ".bashrc",
    ".bash_profile",
    ".profile",
    ".zshrc",
    ".config/albert/secrets.json",
];

/// Binaries that an agent must never invoke through raw shell commands.
/// Gate 1 scans any shell command string for these tokens.
pub const FORBIDDEN_BINARIES: &[&str] = &[
    "rm", "rmdir", "shred", "dd", "mkfs", "fdisk", "parted", "wipefs", "chmod", "chown",
    "sudo", "su", "passwd", "visudo", "crontab",
];

/// Check that `path` does not fall under any protected prefix.
pub fn check_path(path: &Path) -> Result<(), LlbError> {
    let path_str = path.to_string_lossy();

    for prefix in PROTECTED_PATH_PREFIXES {
        if path_str.starts_with(prefix) {
            return Err(LlbError::BlacklistedPath { path: path_str.into_owned() });
        }
    }

    if let Some(home) = std::env::var_os("HOME") {
        for rel in PROTECTED_HOME_FILES {
            let protected = Path::new(&home).join(rel);
            if path.starts_with(&protected) {
                return Err(LlbError::BlacklistedPath { path: path_str.into_owned() });
            }
        }
    }

    Ok(())
}

/// Scan a shell command string for forbidden binary names.
/// Catches attempts to bypass structured tools via raw bash.
pub fn check_shell_command(cmd: &str) -> Result<(), LlbError> {
    let tokens: Vec<&str> = cmd.split_whitespace().collect();
    for token in &tokens {
        let binary = Path::new(token)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(token);
        if FORBIDDEN_BINARIES.contains(&binary) {
            return Err(LlbError::ForbiddenBinary { binary: binary.to_string() });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn etc_is_blacklisted() {
        assert!(check_path(&PathBuf::from("/etc/passwd")).is_err());
    }

    #[test]
    fn tmp_is_allowed() {
        assert!(check_path(&PathBuf::from("/tmp/safe_file.txt")).is_ok());
    }

    #[test]
    fn rm_in_command_is_blocked() {
        assert!(check_shell_command("rm -rf /tmp/foo").is_err());
    }

    #[test]
    fn safe_command_passes() {
        assert!(check_shell_command("cargo build --release").is_ok());
    }

    #[test]
    fn sudo_is_blocked() {
        assert!(check_shell_command("sudo apt-get install curl").is_err());
    }
}
