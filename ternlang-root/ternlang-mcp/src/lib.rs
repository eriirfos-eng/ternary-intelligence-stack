// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// Commercial tier. See LICENSE-COMMERCIAL in the repository root.
// Unauthorized use, copying, or distribution is prohibited.

//! # ternlang-mcp
//!
//! MCP (Model Context Protocol) server for ternlang — connects any AI agent
//! to balanced ternary decision logic via `trit_decide` and friends.
//!
//! # Tools
//!
//! - `trit_decide` — evaluate a ternary decision tree
//! - `trit_vector` — vectorize text to ternary embeddings
//! - `ternary_eval` — evaluate a .tern program snippet
//!
//! # Quick start
//!
//! ```no_run
//! // Run the MCP server
//! // ternlang-mcp --transport stdio
//! ```

/// Re-export of trit primitives for external MCP clients.
pub use ternlang_core::{pack_trits, unpack_trits, Trit};

/// Version string for MCP protocol compatibility.
pub const MCP_VERSION: &str = "2.0.0";

/// Trit-decision result with confidence annotation.
#[derive(Debug, Clone, PartialEq)]
pub struct TritDecision {
    pub trit: Trit,
    pub confidence: f32,
    pub trace: Vec<String>,
}

/// Evaluate a ternary decision tree from packed trits.
pub fn trit_decide(trits: &[u8], count: usize) -> Result<TritDecision, String> {
    let unpacked =
        ternlang_core::unpack_trits(trits, count).map_err(|e| format!("BET fault: {:?}", e))?;

    if unpacked.is_empty() {
        return Err("no trits provided".into());
    }

    // Consensus rule: last trit wins, Tend propagates as "undecided"
    let mut result = Trit::Tend;
    let mut trace = Vec::new();

    for (i, t) in unpacked.iter().enumerate() {
        trace.push(format!("step {}: {:?}", i, t));
        if *t != Trit::Tend {
            result = *t;
        }
    }

    let confidence = if unpacked.iter().any(|&t| t != Trit::Tend) {
        1.0
    } else {
        0.0
    };

    Ok(TritDecision {
        trit: result,
        confidence,
        trace,
    })
}
