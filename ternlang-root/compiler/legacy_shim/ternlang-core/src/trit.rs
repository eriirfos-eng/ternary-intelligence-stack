// SPDX-License-Identifier: LGPL-3.0-or-later
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS
// Open-core compiler. See LICENSE-LGPL in the repository root.
//
// ┌─────────────────────────────────────────────────────────────────────────────┐
// │  DEPRECATED — use `types/trit.rs` instead.                                   │
// │  This module retained for backward compatibility with crates that import      │
// │  `ternlang_core::Trit`. The canonical Trit now lives at                          │
// │  `ternlang_core::types::trit` and is re-exported as `ternlang_core::Trit`.     │
// └─────────────────────────────────────────────────────────────────────────────┘

#[deprecated(note = "use `ternlang_core::types::trit::Trit` instead")]
pub use crate::types::trit::Trit;
