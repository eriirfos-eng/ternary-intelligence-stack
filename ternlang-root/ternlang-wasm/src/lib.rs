//! ternlang-wasm — WebAssembly bindings for the BET VM
//!
//! Exposes `run_tern(src: &str) -> String` and `check_tern(src: &str) -> String`.
//! Drop the wasm-pack output into any web page — no server, no install.
//!
//! © RFI-IRFOS · LGPL-3.0-or-later · Patent A50296/2026

use wasm_bindgen::prelude::*;
use serde::Serialize;
use ternlang_core::{Parser, SemanticAnalyzer, BytecodeEmitter, BetVm};
use ternlang_core::vm::{Value, VmError};

#[derive(Serialize)]
struct RunResult {
    ok:        bool,
    output:    Vec<String>,
    trit:      i8,       // -1 / 0 / +1
    label:     String,   // "affirm" / "hold" / "reject"
    registers: Vec<String>, // debug state
    error:     Option<String>,
    cycles:    usize,    // bytecode bytes (proxy for complexity)
}

/// Run a complete .tern source program through the real BET VM.
///
/// Returns JSON:
/// ```json
/// { "ok": true, "output": ["hello"], "trit": 1, "label": "affirm",
///   "registers": ["Trit(Affirm)", "Int(42)"],
///   "error": null, "cycles": 42 }
/// ```
/// On any error `ok` is false and `error` carries the message.
#[wasm_bindgen]
pub fn run_tern(src: &str) -> String {
    let r = match execute(src) {
        Ok(r)  => r,
        Err(e) => RunResult {
            ok: false, output: vec![], trit: 0,
            label: "hold".into(), registers: vec![], error: Some(e), cycles: 0,
        },
    };
    serde_json::to_string(&r).unwrap_or_else(|e| format!(r#"{{"ok":false,"error":"{e}"}}"#))
}

/// Parse + semantic-check without executing.
/// Returns JSON `{ "ok": bool, "error": string | null }`.
#[wasm_bindgen]
pub fn check_tern(src: &str) -> String {
    match parse_and_check(src) {
        Ok(())  => r#"{"ok":true,"error":null}"#.into(),
        Err(e)  => {
            let escaped = e.replace('\\', r"\\").replace('"', r#"\""#);
            format!(r#"{{"ok":false,"error":"{escaped}"}}"#)
        }
    }
}

// ─── internals ────────────────────────────────────────────────────────────────

fn parse_and_check(src: &str) -> Result<(), String> {
    let program = Parser::new(src)
        .parse_program()
        .map_err(|e| format!("parse error: {e:?}"))?;
    let mut sa = SemanticAnalyzer::new();
    sa.check_program(&program)
        .map_err(|e| format!("type error: {e:?}"))
}

fn execute(src: &str) -> Result<RunResult, String> {
    // 1-3. Parse & Compile (Mirrors CLI Run logic)
    let mut emitter = BytecodeEmitter::new();
    let header_patch = emitter.emit_header_jump();

    let mut parser = Parser::new(src);
    match parser.parse_program() {
        Ok(prog) => {
            // TODO: In the future, pass a ModuleResolver if we support imports in WASM
            emitter.emit_program(&prog);
            emitter.patch_header_jump(header_patch);
            emitter.emit_entry_call("main");
        }
        Err(e) => {
            let error_str = format!("{:?}", e);
            {
                // Fallback for scripts/snippets: parse stmt-by-stmt
                let mut parser = Parser::new(src);
                emitter.patch_header_jump(header_patch);
                let mut found_functions = false;
                loop {
                    match parser.parse_stmt() {
                        Ok(stmt) => emitter.emit_stmt(&stmt),
                        Err(e) => {
                            let e_str = format!("{:?}", e);
                            if e_str.contains("EOF") { break; }
                            if e_str.contains("UnexpectedToken(\"Fn\")") {
                                if let Ok(func) = parser.parse_function() {
                                    emitter.emit_function(&func);
                                    found_functions = true;
                                    continue;
                                }
                            }
                            if e_str.contains("UnexpectedToken(\"Agent\")") {
                                if let Ok(agent) = parser.parse_agent_def() {
                                    emitter.emit_agent_def(&agent);
                                    found_functions = true;
                                    continue;
                                }
                            }
                            return Err(format!("parse error: {e:?}"));
                        }
                    }
                }
                if found_functions { emitter.emit_entry_call("main"); }
            }
        }
    }

    let bytecode = emitter.finalize();
    let cycles = bytecode.len();

    // 4. Run on BET VM
    let mut vm = BetVm::new(bytecode);
    emitter.register_agents(&mut vm);
    
    vm.run().map_err(|e| match e {
        VmError::AgentTypeNotRegistered(id) => format!(
            "Agent type {} is not defined in this program. \
             Define it with: agent Type{} {{ fn run(...) -> trit {{ ... }} }}",
            id, id
        ),
        other => format!("runtime error: {other:?}"),
    })?;

    // 5. Collect output + result trit
    let output = vm.take_output();
    let registers = vm.get_registers().into_iter().map(|v| format!("{:?}", v)).collect();
    let trit_val: i8 = match vm.peek_stack() {
        Some(Value::Trit(t))  => t as i8,
        Some(Value::Int(n))   => n.clamp(-1, 1) as i8,
        Some(Value::Float(f)) => if f > 0.0 { 1 } else if f < 0.0 { -1 } else { 0 },
        _ => 0,
    };
    let label = match trit_val { 1 => "affirm", -1 => "reject", _ => "hold" }.into();

    Ok(RunResult { ok: true, output, trit: trit_val, label, registers, error: None, cycles })
}
