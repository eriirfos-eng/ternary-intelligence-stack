// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// Commercial tier. See LICENSE-COMMERCIAL in the repository root.
// Unauthorized use, copying, or distribution is prohibited.

//! ternlang-runtime — Distributed actor runtime for ternlang
//!
//! Phase 5.1: synchronous TCP transport for remote agent communication.
//!
//! Protocol: newline-delimited JSON over TCP.
//! Each message is a single JSON object followed by '\n'.
//!
//! Message types:
//!   {"type":"send",  "agent_id": 0, "trit": 1}     → send trit to local agent
//!   {"type":"await", "agent_id": 0}                 → run agent handler, return result
//!   {"type":"reply", "trit": 1}                     → response to await
//!   {"type":"error", "msg": "..."}                  → error response
//!
//! Usage:
//!   let node = TernNode::new("127.0.0.1:7373");
//!   node.listen();                  // spawns listener thread
//!   node.connect("127.0.0.1:7374"); // connect to peer
//!   node.remote_send("127.0.0.1:7374", 0, 1);  // send +1 to remote agent 0
//!   let result = node.remote_await("127.0.0.1:7374", 0); // get reply


//! Distributed actor runtime for ternlang — TCP-based TernNode with remote spawn/send/await over newline-JSON protocol.
use std::collections::HashMap;