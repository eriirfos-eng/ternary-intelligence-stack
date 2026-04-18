//! ternlang-wasm — WebAssembly bindings for the BET VM
//!
//! Exposes `run_tern(src: &str) -> String` and `check_tern(src: &str) -> String`.
//! Drop the wasm-pack output into any web page — no server, no install.
//!
//! © RFI-IRFOS · LGPL-3.0-or-later · Patent A50296/2026

use wasm_bindgen::prelude::*;
use serde::Serialize;
use ternlang_core::{Parser, SemanticAnalyzer, BytecodeEmitter, BetVm};
use ternlang_core::vm::Value;

#[derive(Serialize)]
struct RunResult {
    ok:     bool,
    output: Vec<String>,
    trit:   i8,       // -1 / 0 / +1
    label:  String,   // "affirm" / "hold" / "reject"
    error:  Option<String>,
    cycles: usize,    // bytecode bytes (proxy for complexity)
}

/// Run a complete .tern source program through the real BET VM.
///
/// Returns JSON:
/// ```json
/// { "ok": true, "output": ["hello"], "trit": 1, "label": "affirm",
///   "error": null, "cycles": 42 }
/// ```
/// On any error `ok` is false and `error` carries the message.
#[wasm_bindgen]
pub fn run_tern(src: &str) -> String {
    let r = match execute(src) {
        Ok(r)  => r,
        Err(e) => RunResult {
            ok: false, output: vec![], trit: 0,
            label: "hold".into(), error: Some(e), cycles: 0,
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
    // 1. Parse
    let program = Parser::new(src)
        .parse_program()
        .map_err(|e| format!("parse error: {e:?}"))?;

    // 2. Semantic analysis
    let mut sa = SemanticAnalyzer::new();
    sa.check_program(&program)
        .map_err(|e| format!("type error: {e:?}"))?;

    // 3. Compile to bytecode (mirrors CLI run pipeline)
    let mut emitter = BytecodeEmitter::new();
    let header_patch = emitter.emit_header_jump();
    emitter.emit_program(&program);
    emitter.patch_header_jump(header_patch);
    emitter.emit_entry_call("main");
    let bytecode = emitter.finalize();
    let cycles = bytecode.len();

    // 4. Run on BET VM
    let mut vm = BetVm::new(bytecode);
    vm.run().map_err(|e| format!("runtime error: {e:?}"))?;

    // 5. Collect output + result trit
    let output = vm.take_output();
    let trit_val: i8 = match vm.peek_stack() {
        Some(Value::Trit(t))  => t as i8,
        Some(Value::Int(n))   => n.clamp(-1, 1) as i8,
        Some(Value::Float(f)) => if f > 0.0 { 1 } else if f < 0.0 { -1 } else { 0 },
        _ => 0,
    };
    let label = match trit_val { 1 => "affirm", -1 => "reject", _ => "hold" }.into();

    Ok(RunResult { ok: true, output, trit: trit_val, label, error: None, cycles })
}
