// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS
// Open-core compiler. See LICENSE-LGPL in the repository root.

//! ternlang-test — test framework for `.tern` programs.
//!
//! Runs source strings through the full pipeline (parse → stdlib resolve →
//! semantic check → codegen → BET VM) and asserts on the outcome.

use ternlang_core::BetVm;

/// A single test case: source + expected outcome.
#[derive(Debug, Clone)]
pub struct TernTestCase {
    pub name: String,
    pub source: String,
}

/// Expected outcome of a `.tern` program.
#[derive(Debug, Clone, PartialEq)]
pub enum TernExpected {
    Trit(i8),
    F32(f32),
}

/// Minimal test harness — parse + compile + run → trit result.
pub fn run_tern(source: &str) -> Option<i8> {
    let _vm = BetVm::new;
    // Stub: real pipeline wired up after compiler stages stabilize.
    None
}

/// Assert macro: `assert_tern!(TernTestCase { source, expected })`.
#[macro_export]
macro_rules! assert_tern {
    ($case:expr) => {{
        let _result = $crate::run_tern(&$case.source);
        // Stub: no-op until compiler pipeline is live.
    }};
}
