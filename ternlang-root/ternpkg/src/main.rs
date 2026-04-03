//! ternpkg — Ternlang Package Manager
//!
//! Manages ternlang libraries via `ternlang.toml` manifest files.
//! Uses GitHub as the package registry (owner/repo#tag format).
//!
//! Commands:
//!   ternpkg init            — create a new ternlang.toml in the current directory
//!   ternpkg install         — install all packages from ternlang.toml
//!   ternpkg install PKG     — add package to ternlang.toml and install it
//!   ternpkg list            — show installed packages
//!   ternpkg info PKG        — show package metadata
//!
//! Package format (GitHub-backed):
//!   PKG = "owner/repo" or "owner/repo@tag"
//!
//! ternlang.toml example:
//!   [package]
//!   name = "my-ternlang-app"
//!   version = "0.1.0"
//!
//!   [dependencies]
//!   "eriirfos-eng/ternary-intelligence-stack--tis-" = "latest"
//!   "some-user/some-lib" = "v0.2.0"

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

// ─────────────────────────────────────────────────────────────────────────────
// Manifest types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Default)]
struct PackageInfo {
    name: String,
    version: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    license: String,
    #[serde(default)]
    authors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Manifest {
    package: PackageInfo,
    #[serde(default)]
    dependencies: HashMap<String, String>,
}

impl Manifest {
    fn load(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path)
            .map_err(|e| format!("Cannot read ternlang.toml: {}", e))?;
        // Simple TOML-like parsing (key = "value" format, no full TOML library dependency)
        parse_manifest(&text).ok_or_else(|| "Failed to parse ternlang.toml".to_string())
    }

    fn save(&self, path: &Path) -> Result<(), String> {
        let mut out = String::new();
        out.push_str("[package]\n");
        out.push_str(&format!("name = \"{}\"\n", self.package.name));
        out.push_str(&format!("version = \"{}\"\n", self.package.version));
        if !self.package.description.is_empty() {
            out.push_str(&format!("description = \"{}\"\n", self.package.description));
        }
        if !self.package.license.is_empty() {
            out.push_str(&format!("license = \"{}\"\n", self.package.license));
        }
        if !self.dependencies.is_empty() {
            out.push_str("\n[dependencies]\n");
            for (dep, ver) in &self.dependencies {
                out.push_str(&format!("\"{}\" = \"{}\"\n", dep, ver));
            }
        }
        fs::write(path, out).map_err(|e| format!("Cannot write ternlang.toml: {}", e))
    }
}

/// Minimal TOML-subset parser for ternlang.toml.
/// Handles [sections], key = "value", and quoted keys.
fn parse_manifest(text: &str) -> Option<Manifest> {
    let mut manifest = Manifest::default();
    let mut section = "";

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }

        if line.starts_with('[') && line.ends_with(']') {
            section = &line[1..line.len() - 1];
            continue;
        }

        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim().trim_matches('"');
            let val = val.trim().trim_matches('"');
            match section {
                "package" => match key {
                    "name"        => manifest.package.name = val.to_string(),
                    "version"     => manifest.package.version = val.to_string(),
                    "description" => manifest.package.description = val.to_string(),
                    "license"     => manifest.package.license = val.to_string(),
                    _ => {}
                },
                "dependencies" => {
                    manifest.dependencies.insert(key.to_string(), val.to_string());
                }
                _ => {}
            }
        }
    }
    Some(manifest)
}

// ─────────────────────────────────────────────────────────────────────────────
// Package operations
// ─────────────────────────────────────────────────────────────────────────────

/// Resolve a package specifier to (owner, repo, tag).
fn parse_pkg_spec(spec: &str) -> Option<(&str, &str, &str)> {
    let (repo_part, tag) = if let Some(at) = spec.find('@') {
        (&spec[..at], &spec[at + 1..])
    } else {
        (spec, "latest")
    };
    let mut parts = repo_part.splitn(2, '/');
    let owner = parts.next()?;
    let repo  = parts.next()?;
    Some((owner, repo, tag))
}

/// Return the GitHub archive URL for a package.
fn github_archive_url(owner: &str, repo: &str, tag: &str) -> String {
    if tag == "latest" {
        format!("https://github.com/{}/{}/archive/refs/heads/main.tar.gz", owner, repo)
    } else {
        format!("https://github.com/{}/{}/archive/refs/tags/{}.tar.gz", owner, repo, tag)
    }
}

/// Simulate package installation (prints what would happen; real download needs curl/reqwest).
fn install_package(spec: &str, tern_modules: &Path) -> Result<(), String> {
    let (owner, repo, tag) = parse_pkg_spec(spec)
        .ok_or_else(|| format!("Invalid package spec: {}", spec))?;
    let url = github_archive_url(owner, repo, tag);
    let install_dir = tern_modules.join(format!("{}__{}", owner, repo));

    if install_dir.exists() {
        println!("  [cached] {} ({})", spec, tag);
        return Ok(());
    }

    println!("  [install] {} @ {}", spec, tag);
    println!("    source: {}", url);
    println!("    target: {}", install_dir.display());

    // In v0.1: print the curl command the user would run.
    // Production: use a HTTP client (reqwest) to download and unpack.
    println!("    run:    curl -L '{}' | tar xz -C '{}'",
             url, tern_modules.display());
    println!("    (automatic download planned for ternpkg v0.2)");

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// CLI
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("help");

    match cmd {
        "init" => cmd_init(),
        "install" => {
            let pkg = args.get(2).map(String::as_str);
            cmd_install(pkg);
        }
        "list"    => cmd_list(),
        "info"    => {
            let pkg = args.get(2).map(String::as_str).unwrap_or("");
            cmd_info(pkg);
        }
        "help" | "--help" | "-h" => print_help(),
        _ => {
            eprintln!("Unknown command: {}. Run `ternpkg help`.", cmd);
            std::process::exit(1);
        }
    }
}

fn cmd_init() {
    let toml_path = PathBuf::from("ternlang.toml");
    if toml_path.exists() {
        println!("ternlang.toml already exists.");
        return;
    }
    let cwd = std::env::current_dir().unwrap();
    let name = cwd.file_name().and_then(|n| n.to_str()).unwrap_or("my-tern-project");
    let manifest = Manifest {
        package: PackageInfo {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: String::new(),
            license: "LGPL-3.0".to_string(),
            authors: Vec::new(),
        },
        dependencies: HashMap::new(),
    };
    match manifest.save(&toml_path) {
        Ok(_)  => println!("Created ternlang.toml"),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_install(pkg: Option<&str>) {
    let toml_path = PathBuf::from("ternlang.toml");
    let mut manifest = match Manifest::load(&toml_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}. Run `ternpkg init` first.", e);
            return;
        }
    };

    let tern_modules = PathBuf::from(".tern_modules");
    if !tern_modules.exists() {
        fs::create_dir_all(&tern_modules).expect("Cannot create .tern_modules");
    }

    if let Some(spec) = pkg {
        // Add and install a single package
        let (_, _, tag) = parse_pkg_spec(spec).unwrap_or(("", spec, "latest"));
        manifest.dependencies.insert(spec.to_string(), tag.to_string());
        match manifest.save(&toml_path) {
            Ok(_) => {}
            Err(e) => { eprintln!("Error saving manifest: {}", e); return; }
        }
        match install_package(spec, &tern_modules) {
            Ok(_)  => println!("Done."),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        // Install all dependencies from manifest
        if manifest.dependencies.is_empty() {
            println!("No dependencies in ternlang.toml");
            return;
        }
        let deps: Vec<(String, String)> = manifest.dependencies.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        for (dep, _) in &deps {
            match install_package(dep, &tern_modules) {
                Ok(_)  => {}
                Err(e) => eprintln!("Error installing {}: {}", dep, e),
            }
        }
        println!("Done. {} package(s) processed.", deps.len());
    }
}

fn cmd_list() {
    let toml_path = PathBuf::from("ternlang.toml");
    match Manifest::load(&toml_path) {
        Err(e) => eprintln!("Error: {}", e),
        Ok(m) => {
            println!("Package: {} v{}", m.package.name, m.package.version);
            if m.dependencies.is_empty() {
                println!("No dependencies.");
            } else {
                println!("Dependencies:");
                for (dep, ver) in &m.dependencies {
                    println!("  {} = \"{}\"", dep, ver);
                }
            }
        }
    }
}

fn cmd_info(pkg: &str) {
    if let Some((owner, repo, tag)) = parse_pkg_spec(pkg) {
        println!("Package: {}/{}", owner, repo);
        println!("Tag:     {}", tag);
        println!("Source:  {}", github_archive_url(owner, repo, tag));
        println!("Registry: https://github.com/{}/{}", owner, repo);
    } else {
        eprintln!("Invalid package spec. Use: owner/repo or owner/repo@tag");
    }
}

fn print_help() {
    println!("ternpkg — Ternlang Package Manager v0.1");
    println!();
    println!("USAGE:");
    println!("  ternpkg init              Create ternlang.toml in current directory");
    println!("  ternpkg install           Install all packages from ternlang.toml");
    println!("  ternpkg install PKG       Add PKG to ternlang.toml and install");
    println!("  ternpkg list              List packages in ternlang.toml");
    println!("  ternpkg info PKG          Show package info and source URL");
    println!();
    println!("PACKAGE FORMAT:");
    println!("  owner/repo                Latest main branch");
    println!("  owner/repo@v0.1.0         Specific tag");
    println!();
    println!("EXAMPLE:");
    println!("  ternpkg init");
    println!("  ternpkg install eriirfos-eng/ternary-intelligence-stack--tis-@latest");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pkg_spec_simple() {
        let (owner, repo, tag) = parse_pkg_spec("owner/repo").unwrap();
        assert_eq!(owner, "owner");
        assert_eq!(repo, "repo");
        assert_eq!(tag, "latest");
    }

    #[test]
    fn test_parse_pkg_spec_tagged() {
        let (owner, repo, tag) = parse_pkg_spec("owner/repo@v0.2.0").unwrap();
        assert_eq!(owner, "owner");
        assert_eq!(repo, "repo");
        assert_eq!(tag, "v0.2.0");
    }

    #[test]
    fn test_github_archive_url_latest() {
        let url = github_archive_url("alice", "tern-lib", "latest");
        assert!(url.contains("alice/tern-lib"));
        assert!(url.contains("main.tar.gz"));
    }

    #[test]
    fn test_github_archive_url_tagged() {
        let url = github_archive_url("alice", "tern-lib", "v1.0.0");
        assert!(url.contains("v1.0.0"));
        assert!(url.contains("tags"));
    }

    #[test]
    fn test_parse_manifest() {
        let toml = r#"
[package]
name = "my-app"
version = "0.1.0"
license = "LGPL-3.0"

[dependencies]
"alice/tern-lib" = "latest"
"#;
        let m = parse_manifest(toml).unwrap();
        assert_eq!(m.package.name, "my-app");
        assert_eq!(m.package.version, "0.1.0");
        assert_eq!(m.dependencies.get("alice/tern-lib").map(String::as_str), Some("latest"));
    }
}
