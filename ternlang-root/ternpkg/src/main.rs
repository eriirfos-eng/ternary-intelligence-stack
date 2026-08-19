// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS
// Open-core compiler. See LICENSE-LGPL in the repository root.

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


//! ternpkg
//!
//! Package manager for the ternlang ecosystem — ternlang.toml manifest, GitHub-backed registry, install/list/info commands.
use std::collections::{HashMap, HashSet};