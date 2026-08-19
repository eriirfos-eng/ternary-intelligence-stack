// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS
// Open-core compiler. See LICENSE-LGPL in the repository root.

//! # ternlang-core
//!
//! Compiler and VM for Ternlang — balanced ternary language with
//! affirm/tend/reject trit semantics, `@sparseskip` codegen, and BET
//! (Balanced Encoded Ternary) bytecode execution.
//!
//! ## Quick start
//!
//! ```no_run
//! use ternlang_core::{Trit, pack_trits, unpack_trits};
//!
//! let trits = vec![Trit::Affirm, Trit::Tend, Trit::Reject];
//! let packed = pack_trits(&trits);
//! let round_tripped = unpack_trits(&packed, trits.len()).unwrap();
//! assert_eq!(trits, round_tripped);
//! ```

pub mod trit;
pub mod types;
pub mod vm;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod semantic;
pub mod codegen;
pub mod stdlib;
pub mod wasm_simd;

// ─── Consolidated re-exports ───────────────────────────────────────
// Trit, TritBlock5, and packed arithmetic helpers all come from the
// types module so downstream crates (ternlang-ml, ternlang-mcp, etc.)
// can simply do `use ternlang_core::{Trit, TritBlock5, packed_add}`.
pub use types::trit::{Trit, TritBlock5, pack_5_trits, unpack_5_trits};
pub use trit::Trit as LegacyTrit; // deprecated alias — do not use in new code
pub use vm::bet::{pack_trits, unpack_trits, BetFault};
pub use vm::{BetVm, Value as VmValue};
pub use lexer::Token;
pub use ast::*;
pub use parser::Parser;
pub use semantic::SemanticAnalyzer;
pub use codegen::betbc::BytecodeEmitter;
pub use stdlib::{StdlibLoader, ModuleResolver};
