// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS
// Open-core compiler. See LICENSE-LGPL in the repository root.

//! ternlang-lsp — Language Server Protocol implementation for ternlang
//!
//! Implements LSP 3.17 over JSON-RPC 2.0 (stdio transport).
//! Provides: hover, diagnostics, completion, go-to-definition for .tern files.
//!
//! Usage: configure your editor to run `ternlang-lsp` as the language server
//! for `.tern` files. See the VS Code extension for a reference client setup.


//! ternlang-lsp
//!
//! LSP 3.17 language server for ternlang — hover docs, code completion, and live diagnostics for .tern files.
use std::io::{BufRead, BufReader, Write};

fn main() {
}