// albert-llb MCP server — stdio transport
//
// Exposes LLB Protocol tools to any MCP-compatible AI agent:
//   llb_validate      — Gate 1 dry-run (no execution, no snapshot)
//   llb_classify      — Safety tier + required gates for path+op
//   llb_check         — Path and shell command blacklist check
//   llb_write_safe    — Full LLB-protected file write (T1/T2 only)
//
// Install: cargo install albert-llb
// Run:     albert-llb-mcp   (reads from stdin, writes to stdout)

use albert_llb::{
    blacklist,
    gate1,
    tier::SafetyTier,
    types::{MutationRequest, Operation, StructuredIntent},
    LlbError,
};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) if !l.trim().is_empty() => l,
            _ => continue,
        };

        let msg: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let response = handle(&msg);
        let _ = writeln!(out, "{}", response);
        let _ = out.flush();
    }
}

fn handle(msg: &Value) -> Value {
    let id = msg.get("id").cloned().unwrap_or(Value::Null);
    let method = msg["method"].as_str().unwrap_or("");
    let params = msg.get("params").cloned().unwrap_or(json!({}));

    match method {
        "initialize" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2025-03-26",
                "serverInfo": {
                    "name": "albert-llb",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "capabilities": { "tools": {} }
            }
        }),

        "notifications/initialized" | "notifications/cancelled" => {
            // One-way notification — no response needed.
            json!(null)
        }

        "tools/list" => json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": { "tools": tools_manifest() }
        }),

        "tools/call" => {
            let name = params["name"].as_str().unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            let result = dispatch_tool(name, &args);
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [{ "type": "text", "text": result.to_string() }],
                    "isError": false
                }
            })
        }

        _ => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": -32601, "message": format!("unknown method: {method}") }
        }),
    }
}

// ── Tool dispatch ─────────────────────────────────────────────────────────────

fn dispatch_tool(name: &str, args: &Value) -> Value {
    match name {
        "llb_validate" => tool_validate(args),
        "llb_classify" => tool_classify(args),
        "llb_check" => tool_check(args),
        "llb_write_safe" => tool_write_safe(args),
        _ => json!({ "error": format!("unknown tool: {name}") }),
    }
}

/// Gate 1 dry-run — no execution, no snapshot, no HITL.
fn tool_validate(args: &Value) -> Value {
    let path = args["path"].as_str().unwrap_or("").to_string();
    let op_str = args["operation"].as_str().unwrap_or("OVERWRITE");
    let op = parse_op(op_str);
    let intent = intent_from_args(args);
    let req = MutationRequest { path: path.clone(), operation: op.clone(), intent };

    match gate1::run(&req) {
        Ok(ticket) => json!({
            "decision": "+1 ALLOW",
            "trit": 1,
            "resolved_path": ticket.resolved_path.display().to_string(),
            "tier": ticket.tier.to_string(),
            "ticket_id": &ticket.id[..16],
            "requires_snapshot": ticket.tier.requires_snapshot(),
            "requires_hitl": ticket.tier.requires_hitl(),
            "requires_gate2": ticket.tier.requires_gate2(),
            "note": "Gate 1 passed. ExecutionTicket issued (not used in dry-run)."
        }),
        Err(LlbError::BlacklistedPath { path: p }) => json!({
            "decision": "-1 HARD VETO",
            "trit": -1,
            "reason": "blacklisted_path",
            "detail": format!("Path '{p}' is on the LLB blacklist — protected system directory.")
        }),
        Err(LlbError::PathTraversal { path: p }) => json!({
            "decision": "-1 HARD VETO",
            "trit": -1,
            "reason": "path_traversal",
            "detail": format!("Path '{p}' contains '..' — traversal blocked.")
        }),
        Err(e) => json!({
            "decision": "-1 HARD VETO",
            "trit": -1,
            "reason": "gate1_failure",
            "detail": e.to_string()
        }),
    }
}

/// Return safety tier and gate requirements without any filesystem access.
fn tool_classify(args: &Value) -> Value {
    let path_str = args["path"].as_str().unwrap_or("/tmp/unknown");
    let op_str = args["operation"].as_str().unwrap_or("OVERWRITE");
    let op = parse_op(op_str);
    let path = std::path::Path::new(path_str);
    let tier = SafetyTier::classify(path, &op);

    json!({
        "path": path_str,
        "operation": op.to_string(),
        "tier": tier.to_string(),
        "trit_risk": match tier {
            SafetyTier::T0Read => 1,
            SafetyTier::T1Create => 1,
            SafetyTier::T2Modify => 0,
            SafetyTier::T3Delete => -1,
        },
        "requires_snapshot": tier.requires_snapshot(),
        "requires_human_confirmation": tier.requires_hitl(),
        "requires_iocc_gate2": tier.requires_gate2(),
        "note": match tier {
            SafetyTier::T0Read => "Read-only. No gates required.",
            SafetyTier::T1Create => "Safe create. No snapshot or confirmation needed.",
            SafetyTier::T2Modify => "High risk. Snapshot + IOCC Gate 2 required.",
            SafetyTier::T3Delete => "CRITICAL. Snapshot + human confirmation + IOCC Gate 2 required.",
        }
    })
}

/// Check a path and/or shell command against LLB blacklists.
fn tool_check(args: &Value) -> Value {
    let mut results = serde_json::Map::new();

    if let Some(path_str) = args["path"].as_str() {
        let path = std::path::Path::new(path_str);
        match blacklist::check_path(path) {
            Ok(()) => {
                results.insert("path_check".into(), json!({ "status": "PASS", "trit": 1 }));
            }
            Err(LlbError::BlacklistedPath { path: p }) => {
                results.insert("path_check".into(), json!({
                    "status": "BLOCKED",
                    "trit": -1,
                    "detail": format!("'{p}' is a protected path")
                }));
            }
            Err(e) => {
                results.insert("path_check".into(), json!({ "status": "ERROR", "detail": e.to_string() }));
            }
        }
    }

    if let Some(cmd) = args["command"].as_str() {
        match blacklist::check_shell_command(cmd) {
            Ok(()) => {
                results.insert("command_check".into(), json!({ "status": "PASS", "trit": 1 }));
            }
            Err(LlbError::ForbiddenBinary { binary }) => {
                results.insert("command_check".into(), json!({
                    "status": "BLOCKED",
                    "trit": -1,
                    "detail": format!("Forbidden binary '{binary}' in command")
                }));
            }
            Err(e) => {
                results.insert("command_check".into(), json!({ "status": "ERROR", "detail": e.to_string() }));
            }
        }
    }

    if results.is_empty() {
        return json!({ "error": "Provide 'path' and/or 'command' to check." });
    }

    Value::Object(results)
}

/// Full LLB-protected file write. T1/T2 only (no DELETE via MCP).
fn tool_write_safe(args: &Value) -> Value {
    let path = match args["path"].as_str() {
        Some(p) => p.to_string(),
        None => return json!({ "error": "Missing required field: path" }),
    };
    let content = match args["content"].as_str() {
        Some(c) => c.to_string(),
        None => return json!({ "error": "Missing required field: content" }),
    };
    let op_str = args["operation"].as_str().unwrap_or("CREATE");
    let op = parse_op(op_str);

    // DELETE is not permitted via MCP — must be done locally with HITL.
    if op == Operation::Delete {
        return json!({
            "decision": "0 WARN",
            "trit": 0,
            "detail": "DELETE operations are not permitted via the MCP interface. \
                       They require local Human-In-The-Loop confirmation. \
                       Use albert-cli locally to delete files safely."
        });
    }

    let intent = intent_from_args(args);
    let req = MutationRequest { path, operation: op, intent };

    match albert_llb::execute(req, |p| {
        std::fs::write(p, content.as_bytes()).map_err(LlbError::Io)
    }) {
        Ok(albert_llb::LlbDecision::Allow) => json!({
            "decision": "+1 ALLOW",
            "trit": 1,
            "status": "committed",
            "detail": "File written and IOCC Gate 2 passed. Transaction committed."
        }),
        Ok(albert_llb::LlbDecision::Warn(msg)) => json!({
            "decision": "0 WARN",
            "trit": 0,
            "status": "soft_block",
            "detail": msg
        }),
        Ok(albert_llb::LlbDecision::HardVeto) => json!({
            "decision": "-1 HARD VETO",
            "trit": -1,
            "status": "vetoed",
            "detail": "Hard veto. No filesystem changes made."
        }),
        Err(LlbError::IoccViolation { detail }) => json!({
            "decision": "-1 HARD VETO",
            "trit": -1,
            "status": "rolled_back",
            "detail": format!("IOCC Gate 2 violation — rolled back: {detail}")
        }),
        Err(e) => json!({
            "decision": "-1 HARD VETO",
            "trit": -1,
            "status": "error",
            "detail": e.to_string()
        }),
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_op(s: &str) -> Operation {
    match s.to_uppercase().as_str() {
        "CREATE" => Operation::Create,
        "DELETE" => Operation::Delete,
        "CHMOD" => Operation::Chmod,
        _ => Operation::Overwrite,
    }
}

fn intent_from_args(args: &Value) -> StructuredIntent {
    StructuredIntent {
        goal: args["intent_goal"].as_str().unwrap_or("agent-requested mutation").into(),
        justification: args["intent_justification"].as_str().unwrap_or("no justification provided").into(),
        fallback_if_rejected: args["intent_fallback"].as_str().unwrap_or("abort").into(),
    }
}

fn tools_manifest() -> Value {
    json!([
        {
            "name": "llb_validate",
            "description": "Gate 1 dry-run — validate a proposed filesystem mutation without executing it. Returns ternary decision (+1/0/-1), resolved path, safety tier, and required gates. Use this before any filesystem write to check LLB will permit it.",
            "annotations": {
                "readOnlyHint": true,
                "idempotentHint": true,
                "destructiveHint": false
            },
            "inputSchema": {
                "type": "object",
                "required": ["path", "operation"],
                "properties": {
                    "path": { "type": "string", "description": "Absolute path to validate." },
                    "operation": { "type": "string", "enum": ["CREATE", "OVERWRITE", "DELETE", "CHMOD"], "description": "Intended operation." },
                    "intent_goal": { "type": "string", "description": "What you're trying to accomplish." },
                    "intent_justification": { "type": "string", "description": "Why this mutation is needed." },
                    "intent_fallback": { "type": "string", "description": "What to do if this is rejected." }
                }
            }
        },
        {
            "name": "llb_classify",
            "description": "Return the LLB safety tier and gate requirements for a given path + operation. T0=read, T1=create, T2=modify (snapshot + IOCC), T3=delete (snapshot + HITL + IOCC). No filesystem access — pure classification.",
            "annotations": {
                "readOnlyHint": true,
                "idempotentHint": true,
                "destructiveHint": false
            },
            "inputSchema": {
                "type": "object",
                "required": ["path", "operation"],
                "properties": {
                    "path": { "type": "string", "description": "File path to classify (does not need to exist)." },
                    "operation": { "type": "string", "enum": ["CREATE", "OVERWRITE", "DELETE", "CHMOD"] }
                }
            }
        },
        {
            "name": "llb_check",
            "description": "Check a path and/or shell command against the LLB blacklists. Returns PASS (+1) or BLOCKED (-1). Blacklisted paths: /etc, /bin, /sys, ~/.ssh, etc. Forbidden binaries: rm, sudo, chmod, dd, shred, etc.",
            "annotations": {
                "readOnlyHint": true,
                "idempotentHint": true,
                "destructiveHint": false
            },
            "inputSchema": {
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Path to check against the blacklist." },
                    "command": { "type": "string", "description": "Shell command string to scan for forbidden binaries." }
                }
            }
        },
        {
            "name": "llb_write_safe",
            "description": "LLB-protected file write. Full atomic transaction: Gate 1 (blacklist + traversal check) → Snapshot (for OVERWRITE) → Write → IOCC Gate 2 (integrity + spillover check) → Commit or Rollback. DELETE is not available via MCP — requires local HITL.",
            "annotations": {
                "readOnlyHint": false,
                "idempotentHint": false,
                "destructiveHint": false
            },
            "inputSchema": {
                "type": "object",
                "required": ["path", "content"],
                "properties": {
                    "path": { "type": "string", "description": "Absolute path to write." },
                    "content": { "type": "string", "description": "File content to write." },
                    "operation": { "type": "string", "enum": ["CREATE", "OVERWRITE"], "description": "CREATE (default) or OVERWRITE." },
                    "intent_goal": { "type": "string", "description": "What you're trying to accomplish." },
                    "intent_justification": { "type": "string", "description": "Why this file write is needed." },
                    "intent_fallback": { "type": "string", "description": "What to do if rejected." }
                }
            }
        }
    ])
}
