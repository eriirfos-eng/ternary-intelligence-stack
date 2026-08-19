// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS
// Open-core compiler. See LICENSE-LGPL in the repository root.

//! ternlang-codegen — AST → C transpiler backend.
//!
//! Converts a ternlang `Program` (produced by `ternlang-core::Parser`) into
//! a valid, self-contained C source file that can be compiled with any C11
//! compiler.
//!
//! # Ternary representation
//! Trits are represented as `int8_t` with values `-1`, `0`, `+1`.
//! The generated file includes a small header of inline trit primitives so it
//! has no external dependencies beyond `<stdint.h>` and `<stdio.h>`.
//!
//! # Usage
//! ```rust
//! use ternlang_codegen::CTranspiler;
//! use ternlang_core::{Parser, StdlibLoader};
//!
//! let src = r#"fn main() -> trit { return consensus(1, -1); }"#;
//! let mut parser = ternlang_core::Parser::new(src);
//! let mut prog = parser.parse_program().unwrap();
//! StdlibLoader::resolve(&mut prog);
//!
//! let c_src = CTranspiler::new().emit(&prog);
//! println!("{c_src}");
//! ```


//! C transpiler backend for the Ternlang compiler — emits C source from the Ternlang AST for native cross-compilation targets.
use ternlang_core::ast::*;