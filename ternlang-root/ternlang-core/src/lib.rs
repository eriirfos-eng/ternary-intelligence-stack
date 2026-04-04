// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS
// Open-core compiler. See LICENSE-LGPL in the repository root.

pub mod trit;
pub mod vm;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod semantic;
pub mod codegen;
pub mod stdlib;

pub use trit::Trit;
pub use vm::bet::{pack_trits, unpack_trits, BetFault};
pub use lexer::Token;
pub use ast::*;
pub use parser::Parser;
pub use semantic::SemanticAnalyzer;
pub use codegen::betbc::BytecodeEmitter;
pub use vm::BetVm;
pub use stdlib::{StdlibLoader, ModuleResolver};
