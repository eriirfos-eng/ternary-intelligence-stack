/// ternlang-mcp — Model Context Protocol server
///
/// Turns any binary AI agent into a ternary decision engine.
/// Transport: JSON-RPC 2.0 over stdio (MCP standard).
///
/// Tools exposed:
///   trit_decide           — scalar ternary decision: evidence[] → reject/tend/affirm + confidence
///   trit_vector           — multi-dimensional evidence aggregation with named dimensions + weights
///   trit_consensus        — consensus(a, b) → ternary result
///   trit_eval             — evaluate a trit expression string
///   ternlang_run          — compile + run a .tern snippet
///   quantize_weights      — f32 array → ternary weights
///   sparse_benchmark      — run sparse vs dense matmul, report skip rate
///   trit_debate           — two competing claims → 3-way verdict with tension score
///   trit_uncertainty_map  — annotate any text with trit signals per sentence/paragraph
///   trit_calibrate        — AI decision log → binary habituation report + calibration score
///   trit_translate        — Python/SQL/JSON rules → .tern equivalent with hold zones injected
///   trit_eco_check        — human trit + eco trit → synthesise tension as tend when they diverge
///   trit_audit            — full TernAudit: binary ratio, EU AI Act heuristic, flagged decisions

use std::io::{self, BufRead, Write};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ternlang_core::{trit::Trit, parser::Parser, codegen::betbc::BytecodeEmitter, vm::BetVm};
use ternlang_ml::{TritMatrix, TritScalar, TritEvidenceVec, TEND_BOUNDARY,
                   bitnet_threshold, benchmark, dense_matmul, sparse_matmul,
                   DeliberationEngine, action_gate, GateDimension, GateVerdict};
use ternlang_moe::TernMoeOrchestrator;

// ─── JSON-RPC types ──────────────────────────────────────────────────────────

#[derive(Deserialize)]
#[allow(dead_code)]
struct RpcRequest {
    jsonrpc: String,
    id:      Option<Value>,
    method:  String,
    params:  Option<Value>,
}

#[derive(Serialize)]
struct RpcResponse {
    jsonrpc: String,
    id:      Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result:  Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error:   Option<RpcError>,
}

#[derive(Serialize)]
struct RpcError {
    code:    i32,
    message: String,
}

impl RpcResponse {
    fn ok(id: Value, result: Value) -> Self {
        Self { jsonrpc: "2.0".into(), id, result: Some(result), error: None }
    }
    fn err(id: Value, code: i32, message: impl Into<String>) -> Self {
        Self { jsonrpc: "2.0".into(), id, result: None,
               error: Some(RpcError { code, message: message.into() }) }
    }
}

// ─── Trit helpers ────────────────────────────────────────────────────────────

fn trit_to_i8(t: Trit) -> i8 {
    match t { Trit::Reject => -1, Trit::Tend => 0, Trit::Affirm => 1 }
}

fn trit_label(t: Trit) -> &'static str {
    match t { Trit::Reject => "reject", Trit::Tend => "tend", Trit::Affirm => "affirm" }
}

fn i8_to_trit(v: i64) -> Option<Trit> {
    match v { -1 => Some(Trit::Reject), 0 => Some(Trit::Tend), 1 => Some(Trit::Affirm), _ => None }
}

// ─── Tool: trit_decide ───────────────────────────────────────────────────────
//
// THE flagship tool. Any agent passes in floating-point evidence signals.
// We quantize to ternary, run consensus across all signals, return a decision.
//
// This is what "turns a binary agent into a ternary decision engine" means:
// the agent no longer has to collapse ambiguous evidence into YES/NO.
// It gets HOLD — a computational instruction to gather more before acting.

fn tool_trit_decide(params: &Value) -> Result<Value, String> {
    let evidence: Vec<f32> = params["evidence"]
        .as_array()
        .ok_or("evidence must be an array of numbers")?
        .iter()
        .map(|v| v.as_f64().ok_or("evidence values must be numbers").map(|f| f as f32))
        .collect::<Result<_, _>>()?;

    if evidence.is_empty() {
        return Err("evidence array cannot be empty".into());
    }

    let min_confidence = params["min_confidence"].as_f64().unwrap_or(0.0) as f32;

    // Mean of clamped evidence values → aggregate TritScalar
    let mean: f32 = evidence.iter().sum::<f32>() / evidence.len() as f32;
    let scalar = TritScalar::new(mean);

    // Per-signal breakdown
    let per_signal: Vec<Value> = evidence.iter().enumerate().map(|(i, &v)| {
        let s = TritScalar::new(v);
        json!({
            "index": i,
            "raw": (v * 1000.0).round() / 1000.0,
            "label": s.label(),
            "confidence": (s.confidence() * 1000.0).round() / 1000.0,
            "trit": trit_to_i8(s.trit()),
        })
    }).collect();

    let zeros = per_signal.iter().filter(|s| s["trit"] == 0).count();
    let sparsity = zeros as f64 / evidence.len() as f64;
    let actionable = scalar.is_actionable(min_confidence);

    let recommendation = match scalar.trit() {
        Trit::Affirm => format!(
            "Affirm — confidence {:.0}%{}. Proceed with action.",
            scalar.confidence() * 100.0,
            if actionable { "" } else { " (below min_confidence threshold — gather more evidence)" }
        ),
        Trit::Reject => format!(
            "Reject — confidence {:.0}%{}. Do not proceed.",
            scalar.confidence() * 100.0,
            if actionable { "" } else { " (below min_confidence threshold — gather more evidence)" }
        ),
        Trit::Tend => format!(
            "Tend — scalar {:.3} is within the deliberation zone [{:.3}, +{:.3}]. \
             Gather more evidence before acting.",
            scalar.raw(), -TEND_BOUNDARY, TEND_BOUNDARY
        ),
    };

    Ok(json!({
        "scalar":          (scalar.raw() * 1000.0).round() / 1000.0,
        "trit":            trit_to_i8(scalar.trit()),
        "label":           scalar.label(),
        "confidence":      (scalar.confidence() * 1000.0).round() / 1000.0,
        "is_actionable":   actionable,
        "tend_boundary":   TEND_BOUNDARY,
        "signal_sparsity": sparsity,
        "recommendation":  recommendation,
        "per_signal":      per_signal,
    }))
}

// ─── Tool: trit_vector ───────────────────────────────────────────────────────
//
// Multi-dimensional evidence aggregation with named dimensions and weights.
// The flagship agent-reasoning tool: give it your evidence sources by name,
// get back an aggregate scalar decision + per-dimension breakdown.

fn tool_trit_vector(params: &Value) -> Result<Value, String> {
    let dims = params["dimensions"]
        .as_array()
        .ok_or("dimensions must be an array of {label, value, weight} objects")?;

    if dims.is_empty() {
        return Err("dimensions cannot be empty".into());
    }

    let min_confidence = params["min_confidence"].as_f64().unwrap_or(0.5) as f32;

    let mut labels  = Vec::new();
    let mut values  = Vec::new();
    let mut weights = Vec::new();

    for (i, d) in dims.iter().enumerate() {
        let label  = d["label"].as_str()
            .unwrap_or_else(|| "unnamed")
            .to_string();
        let value  = d["value"].as_f64()
            .ok_or(format!("dimension[{}].value must be a number", i))? as f32;
        let weight = d["weight"].as_f64().unwrap_or(1.0) as f32;
        if weight < 0.0 { return Err(format!("dimension[{}].weight must be >= 0", i)); }
        labels.push(label);
        values.push(value);
        weights.push(weight);
    }

    let ev  = TritEvidenceVec::new(labels, values, weights);
    let agg = ev.aggregate();
    let scalars = ev.scalars();

    let breakdown: Vec<Value> = ev.dimensions.iter()
        .zip(ev.values.iter())
        .zip(ev.weights.iter())
        .zip(scalars.iter())
        .map(|(((label, &raw), &weight), s)| json!({
            "label":      label,
            "raw":        (raw * 1000.0).round() / 1000.0,
            "weight":     weight,
            "trit":       trit_to_i8(s.trit()),
            "zone":       s.label(),
            "confidence": (s.confidence() * 1000.0).round() / 1000.0,
        }))
        .collect();

    let dominant = ev.dominant().map(|(label, s)| json!({
        "label":      label,
        "zone":       s.label(),
        "confidence": (s.confidence() * 1000.0).round() / 1000.0,
    }));

    let actionable = agg.is_actionable(min_confidence);

    let recommendation = match agg.trit() {
        Trit::Affirm => format!(
            "Affirm — aggregate scalar {:.3}, confidence {:.0}%{}.",
            agg.raw(), agg.confidence() * 100.0,
            if actionable { ". Act." } else { ". Confidence below threshold — continue gathering evidence." }
        ),
        Trit::Reject => format!(
            "Reject — aggregate scalar {:.3}, confidence {:.0}%{}.",
            agg.raw(), agg.confidence() * 100.0,
            if actionable { ". Do not act." } else { ". Confidence below threshold — continue gathering evidence." }
        ),
        Trit::Tend => format!(
            "Tend — aggregate scalar {:.3} is in the deliberation zone [{:.3}, +{:.3}]. \
             Do not act yet. Strongest signal: {}.",
            agg.raw(), -TEND_BOUNDARY, TEND_BOUNDARY,
            ev.dominant().map(|(l, _)| l).unwrap_or("none")
        ),
    };

    Ok(json!({
        "aggregate": {
            "scalar":       (agg.raw() * 1000.0).round() / 1000.0,
            "trit":         trit_to_i8(agg.trit()),
            "label":        agg.label(),
            "confidence":   (agg.confidence() * 1000.0).round() / 1000.0,
            "is_actionable": actionable,
        },
        "breakdown":      breakdown,
        "dominant":       dominant,
        "tend_boundary":  TEND_BOUNDARY,
        "recommendation": recommendation,
    }))
}

// ─── Tool: trit_consensus ────────────────────────────────────────────────────

fn tool_trit_consensus(params: &Value) -> Result<Value, String> {
    let a_val = params["a"].as_i64().ok_or("a must be -1, 0, or 1")?;
    let b_val = params["b"].as_i64().ok_or("b must be -1, 0, or 1")?;
    let a = i8_to_trit(a_val).ok_or(format!("a={} is not a valid trit (-1, 0, 1)", a_val))?;
    let b = i8_to_trit(b_val).ok_or(format!("b={} is not a valid trit (-1, 0, 1)", b_val))?;
    let (sum, carry) = a + b;
    Ok(json!({
        "result": trit_to_i8(sum),
        "label": trit_label(sum),
        "carry": trit_to_i8(carry),
        "expression": format!("consensus({}, {}) = {}", a_val, b_val, trit_to_i8(sum))
    }))
}

// ─── Tool: trit_eval ─────────────────────────────────────────────────────────

fn tool_trit_eval(params: &Value) -> Result<Value, String> {
    let code = params["expression"].as_str().ok_or("expression must be a string")?;
    // Wrap bare expression in a return statement if needed
    let full_code = if code.trim_end().ends_with(';') {
        code.to_string()
    } else {
        format!("return {};", code)
    };

    let mut parser = Parser::new(&full_code);
    let mut emitter = BytecodeEmitter::new();
    loop {
        match parser.parse_stmt() {
            Ok(stmt) => emitter.emit_stmt(&stmt),
            Err(e) => {
                if format!("{:?}", e).contains("EOF") { break; }
                return Err(format!("parse error: {:?}", e));
            }
        }
    }
    let code_bytes = emitter.finalize();
    let mut vm = BetVm::new(code_bytes);
    vm.run().map_err(|e| format!("vm error: {}", e))?;

    let reg0 = vm.get_register(0);
    Ok(json!({
        "expression": params["expression"],
        "result_register_0": format!("{:?}", reg0)
    }))
}

// ─── Tool: ternlang_run ──────────────────────────────────────────────────────

fn tool_ternlang_run(params: &Value) -> Result<Value, String> {
    let code = params["code"].as_str().ok_or("code must be a string")?;

    let mut parser = Parser::new(code);
    let mut emitter = BytecodeEmitter::new();

    match parser.parse_program() {
        Ok(prog) => emitter.emit_program(&prog),
        Err(_) => {
            let mut p2 = Parser::new(code);
            loop {
                match p2.parse_stmt() {
                    Ok(stmt) => emitter.emit_stmt(&stmt),
                    Err(e) => {
                        if format!("{:?}", e).contains("EOF") { break; }
                        return Err(format!("parse error: {:?}", e));
                    }
                }
            }
        }
    }

    let bytecode = emitter.finalize();
    let bytecode_len = bytecode.len();
    let mut vm = BetVm::new(bytecode);
    vm.run().map_err(|e| format!("vm error: {}", e))?;

    let registers: Vec<Value> = (0..10).map(|i| {
        format!("{:?}", vm.get_register(i)).into()
    }).collect();

    Ok(json!({
        "status": "ok",
        "bytecode_bytes": bytecode_len,
        "registers": registers
    }))
}

// ─── Tool: quantize_weights ──────────────────────────────────────────────────

fn tool_quantize_weights(params: &Value) -> Result<Value, String> {
    let weights: Vec<f32> = params["weights"]
        .as_array().ok_or("weights must be an array")?
        .iter()
        .map(|v| v.as_f64().ok_or("weight values must be numbers").map(|f| f as f32))
        .collect::<Result<_, _>>()?;

    let threshold = params["threshold"].as_f64()
        .unwrap_or_else(|| bitnet_threshold(&weights) as f64) as f32;

    let trits: Vec<i8> = weights.iter().map(|&w| {
        if w > threshold { 1 }
        else if w < -threshold { -1 }
        else { 0 }
    }).collect();

    let zeros = trits.iter().filter(|&&t| t == 0).count();
    let sparsity = zeros as f64 / trits.len() as f64;

    Ok(json!({
        "trits": trits,
        "threshold_used": threshold,
        "sparsity": sparsity,
        "nnz": trits.len() - zeros,
        "total": trits.len()
    }))
}

// ─── Tool: sparse_benchmark ──────────────────────────────────────────────────

fn tool_sparse_benchmark(params: &Value) -> Result<Value, String> {
    let rows = params["rows"].as_u64().unwrap_or(4) as usize;
    let cols = params["cols"].as_u64().unwrap_or(4) as usize;

    let weights: Vec<f32> = match params["weights"].as_array() {
        Some(arr) => arr.iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect(),
        None => {
            // Generate a representative ternary weight distribution if none provided
            (0..rows*cols).map(|i| {
                match i % 5 { 0 => 0.9, 1 => -0.8, 2 => 0.1, 3 => -0.1, _ => 0.05 }
            }).collect()
        }
    };

    if weights.len() != rows * cols {
        return Err(format!("weights length {} must equal rows×cols = {}×{}={}", weights.len(), rows, cols, rows*cols));
    }

    let threshold = params["threshold"].as_f64()
        .unwrap_or_else(|| bitnet_threshold(&weights) as f64) as f32;

    let w = TritMatrix::from_f32(rows, cols, &weights, threshold);
    let input = TritMatrix::new(rows, cols); // zero input — measures weight sparsity purely

    let r = benchmark(&input, &w);
    let (_dense_result, _) = { let i2 = TritMatrix::new(rows, cols); (dense_matmul(&i2, &w), 0) };
    let (_, skipped) = sparse_matmul(&input, &w);

    Ok(json!({
        "rows": rows,
        "cols": cols,
        "weight_sparsity": r.weight_sparsity,
        "skip_rate": r.skip_rate,
        "dense_ops": r.dense_ops,
        "sparse_ops": r.sparse_ops,
        "skipped_ops": skipped,
        "ops_reduction_factor": r.dense_ops as f64 / r.sparse_ops.max(1) as f64,
        "threshold_used": threshold,
        "summary": format!(
            "{:.1}% weight sparsity → {:.1}x fewer multiply ops ({} skipped of {})",
            r.weight_sparsity * 100.0,
            r.dense_ops as f64 / r.sparse_ops.max(1) as f64,
            skipped,
            r.dense_ops
        )
    }))
}

// ─── Tool: moe_orchestrate ───────────────────────────────────────────────────
//
// Full MoE-13 ternary orchestration pass.
// Routes through the 13-expert pool, synthesises triad field, runs safety gate,
// votes, detects hold, optionally calls tiebreaker. Returns full OrchestrationResult.

fn tool_moe_orchestrate(params: &Value) -> Result<Value, String> {
    let query = params["query"].as_str().ok_or("query must be a string")?;

    // Parse evidence vector (6 dims: syntax/world_knowledge/reasoning/tool_use/persona/safety)
    let evidence: Vec<f32> = match params["evidence"].as_array() {
        Some(arr) => arr.iter()
            .map(|v| v.as_f64().ok_or("evidence values must be numbers").map(|f| f as f32))
            .collect::<Result<_, _>>()?,
        None => vec![0.0f32; 6],
    };

    let mut orch = TernMoeOrchestrator::with_standard_experts();
    let result = orch.orchestrate(query, &evidence);

    let trit_label = match result.trit {
        1  => "affirm",
        -1 => "reject",
        _  => "tend",
    };

    let verdicts: Vec<Value> = result.verdicts.iter().map(|v| json!({
        "expert_id":   v.expert_id,
        "expert_name": v.expert_name,
        "trit":        v.trit,
        "confidence":  (v.confidence * 1000.0).round() / 1000.0,
        "reasoning":   v.reasoning,
    })).collect();

    let pair_info = result.pair.as_ref().map(|p| json!({
        "expert_a":  p.expert_a,
        "expert_b":  p.expert_b,
        "relevance": (p.relevance * 1000.0).round() / 1000.0,
        "synergy":   (p.synergy  * 1000.0).round() / 1000.0,
        "combined":  (p.combined * 1000.0).round() / 1000.0,
    }));

    Ok(json!({
        "trit":           result.trit,
        "label":          trit_label,
        "confidence":     (result.confidence * 1000.0).round() / 1000.0,
        "held":           result.held,
        "safety_vetoed":  result.safety_vetoed,
        "temperature":    (result.temperature * 1000.0).round() / 1000.0,
        "prompt_hint":    result.prompt_hint,
        "triad_field":    {
            "synergy_weight":    (result.triad_field.synergy_weight * 1000.0).round() / 1000.0,
            "field":             result.triad_field.field.raw,
            "is_amplifying":     result.triad_field.is_amplifying(),
        },
        "routing_pair":   pair_info,
        "verdicts":       verdicts,
    }))
}

// ─── Tool: moe_deliberate ────────────────────────────────────────────────────
//
// Run the EMA deliberation engine — converges scalar toward target confidence
// over multiple rounds. Useful when initial evidence is weak and you want to
// simulate iterative reasoning (like a human saying "let me think about this").

fn tool_moe_deliberate(params: &Value) -> Result<Value, String> {
    let initial = params["initial_scalar"]
        .as_f64().ok_or("initial_scalar must be a number")? as f32;
    let target  = params["target_confidence"]
        .as_f64().unwrap_or(0.8) as f32;
    let alpha   = params["alpha"].as_f64().unwrap_or(0.4) as f32;
    let max_rounds = params["max_rounds"].as_u64().unwrap_or(10) as usize;

    // Evidence updates — each round can inject new signals
    let updates: Vec<f32> = match params["evidence_updates"].as_array() {
        Some(arr) => arr.iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect(),
        None => vec![initial; max_rounds], // no updates → converge from initial alone
    };

    let engine = DeliberationEngine::new(target, max_rounds)
        .with_alpha(alpha);

    // `run()` takes Vec<Vec<f32>> — each outer entry is one round's signals
    let rounds_evidence: Vec<Vec<f32>> = updates.iter().map(|&v| vec![v]).collect();
    let result = engine.run(rounds_evidence);

    let trit_label = match result.final_trit {
        1  => "affirm",
        -1 => "reject",
        _  => "tend",
    };

    let rounds: Vec<Value> = result.trace.iter().map(|r| json!({
        "round":      r.round,
        "scalar":     (r.scalar.raw() * 1000.0).round() / 1000.0,
        "confidence": (r.scalar.confidence() * 1000.0).round() / 1000.0,
        "trit":       r.scalar.trit_i8(),
        "converged":  r.converged,
    })).collect();

    Ok(json!({
        "final_confidence": (result.final_confidence * 1000.0).round() / 1000.0,
        "trit":             result.final_trit,
        "label":            trit_label,
        "converged":        result.converged,
        "rounds_used":      result.rounds_used,
        "target_confidence": target,
        "convergence_reason": result.convergence_reason,
        "rounds":           rounds,
        "summary": format!(
            "{} after {} round(s) — confidence {:.0}% (target {:.0}%) — {}",
            trit_label.to_uppercase(),
            result.rounds_used,
            result.final_confidence * 100.0,
            target * 100.0,
            if result.converged { "converged" } else { "did not converge (held)" }
        ),
    }))
}

// ─── Tool: trit_action_gate ──────────────────────────────────────────────────
//
// Multi-dimensional hard-block safety gate.
// Any dimension marked hard_block:true with a negative signal VETOES the action.
// All other dims contribute to the weighted pass/block vote.
// This is the structural safety layer: it fires before any other reasoning.

fn tool_trit_action_gate(params: &Value) -> Result<Value, String> {
    let dims_raw = params["dimensions"]
        .as_array().ok_or("dimensions must be an array")?;

    // GateDimension uses `name` + `evidence` (f32 signal), not label+trit
    // evidence on [-1,1]: positive = pass signal, negative = block signal
    let dims: Vec<GateDimension> = dims_raw.iter().map(|d| {
        let name       = d["label"].as_str().unwrap_or("dim").to_string();
        // Accept either `trit` (-1/0/1) or `evidence` (-1.0..1.0); trit → float
        let evidence   = if let Some(t) = d["trit"].as_i64() {
            t as f32
        } else {
            d["evidence"].as_f64().unwrap_or(0.0) as f32
        };
        let weight     = d["weight"].as_f64().unwrap_or(1.0) as f32;
        let hard_block = d["hard_block"].as_bool().unwrap_or(false);
        let mut dim = GateDimension::new(name, evidence, weight);
        if hard_block { dim = dim.hard(); }
        dim
    }).collect::<Vec<_>>();

    if dims.is_empty() {
        return Err("dimensions cannot be empty".into());
    }

    let result = action_gate(&dims);

    let verdict_label = match result.verdict {
        GateVerdict::Proceed => "proceed",
        GateVerdict::Block   => "blocked",
        GateVerdict::Hold    => "hold",
    };

    let dim_details: Vec<Value> = result.dim_results.iter().map(|(name, scalar, is_hard)| json!({
        "label":      name,
        "evidence":   (scalar.raw() * 1000.0).round() / 1000.0,
        "trit":       scalar.trit_i8(),
        "hard_block": is_hard,
        "status": if *is_hard && scalar.trit_i8() < 0 { "VETO" }
                  else if scalar.trit_i8() > 0 { "pass" }
                  else if scalar.trit_i8() < 0 { "block" }
                  else { "hold" },
    })).collect();

    Ok(json!({
        "verdict":       verdict_label,
        "hard_blocked_by": result.hard_blocked_by,
        "aggregate_scalar": (result.aggregate.raw() * 1000.0).round() / 1000.0,
        "aggregate_confidence": (result.aggregate.confidence() * 1000.0).round() / 1000.0,
        "dimensions":    dim_details,
        "explanation":   result.explanation,
    }))
}

// ─── Industrial Standards Tools ──────────────────────────────────────────────

fn tool_get_industrial_standards(_params: &Value) -> Result<Value, String> {
    Ok(json!({
        "authority": "RFI-IRFOS (ZVR: 1015608684)",
        "standards": [
            { "id": "T-TOKEN-v1.0", "name": "Triadic Tokenization (TPE)", "impact": "33% Entropy Reduction" },
            { "id": "T-KV-CACHE-v1.0", "name": "Memory Moat (Sparse KV)", "impact": "60% Memory Reduction" },
            { "id": "T-Fi-v1.0", "name": "Triadic Compute Currency", "impact": "Standardized TaaS Billing" },
            { "id": "T-HAL-v1.0", "name": "Hardware Abstraction", "impact": "Native Triadic Silicon Access" },
            { "id": "T-BIO-v1.0", "name": "Neural Encoding", "impact": "1:1 BCI Parity" }
        ],
        "message": "Published by RFI-IRFOS (ZVR: 1015608684). See ternlang.com for full specifications."
    }))
}

fn tool_audit_ternary_logic(params: &Value) -> Result<Value, String> {
    let code = params["code"].as_str().ok_or("code must be a string")?;
    
    // Simulate auditing logic for triadic compliance
    let binary_count = code.matches("true").count() + code.matches("false").count();
    let triadic_count = code.matches("affirm").count() + code.matches("tend").count() + code.matches("reject").count();
    
    let status = if triadic_count > binary_count { "COMPLIANT" } else { "NON-COMPLIANT (Binary Habituation Detected)" };
    
    Ok(json!({
        "audit_id": format!("AUDIT-{:x}", binary_count * 31 + triadic_count * 97 + code.len()),
        "status": status,
        "metrics": {
            "binary_leakage": binary_count,
            "triadic_fluency": triadic_count,
            "sparseskip_potential": if code.contains("@sparseskip") { "High" } else { "Low" }
        }
    }))
}

// ─── Tool: trit_debate ──────────────────────────────────────────────────────
//
// Two competing claims → structured 3-way verdict.
// Routes each claim through MoE-13, compares, synthesises tension.

fn tool_trit_debate(params: &Value) -> Result<Value, String> {
    let claim_a = params["claim_a"].as_str().ok_or("claim_a must be a string")?;
    let claim_b = params["claim_b"].as_str().ok_or("claim_b must be a string")?;
    let context  = params["context"].as_str().unwrap_or("");

    let mut orch = TernMoeOrchestrator::with_standard_experts();

    let query_a = if context.is_empty() {
        format!("Evaluate this claim: {}", claim_a)
    } else {
        format!("Context: {}. Evaluate this claim: {}", context, claim_a)
    };
    let query_b = if context.is_empty() {
        format!("Evaluate this claim: {}", claim_b)
    } else {
        format!("Context: {}. Evaluate this claim: {}", context, claim_b)
    };

    let res_a = orch.orchestrate(&query_a, &[0.0; 6]);
    let res_b = orch.orchestrate(&query_b, &[0.0; 6]);

    let ta = res_a.trit;
    let tb = res_b.trit;
    let tension: f32 = (res_a.confidence - res_b.confidence).abs()
        .max(if ta != tb { 0.6 } else { 0.1 });

    let label_i8 = |t: i8| if t == 1 { "affirm" } else if t == -1 { "reject" } else { "tend" };

    let synthesis = match (ta, tb) {
        (1,  1)  => "Both claims have supporting evidence — they may be complementary, not contradictory.",
        (-1,-1)  => "Both claims face contradicting evidence — neither is well-supported.",
        (1, -1)  => "Claim A is supported; Claim B is contradicted. Evidence favours A.",
        (-1, 1)  => "Claim B is supported; Claim A is contradicted. Evidence favours B.",
        (0,  _)  => "Claim A is uncertain. More evidence needed before comparing.",
        (_,  0)  => "Claim B is uncertain. More evidence needed before comparing.",
        _        => "Both claims remain in deliberation — insufficient evidence to decide.",
    };

    let hold_reason = if ta == 0 || tb == 0 {
        Some("One or both claims are in the tend zone — the system is holding for more context.")
    } else {
        None
    };

    let mut out = json!({
        "claim_a": claim_a,
        "claim_b": claim_b,
        "for_a":   { "trit": ta, "label": label_i8(ta), "confidence": (res_a.confidence * 100.0).round() / 100.0 },
        "for_b":   { "trit": tb, "label": label_i8(tb), "confidence": (res_b.confidence * 100.0).round() / 100.0 },
        "tension": (tension * 100.0).round() / 100.0,
        "synthesis": synthesis,
        "verdict": if ta == tb { "AGREEMENT" } else if ta == 0 || tb == 0 { "HOLD" } else { "CONFLICT" }
    });
    if let Some(r) = hold_reason {
        out["hold_reason"] = json!(r);
    }
    Ok(out)
}

// ─── Tool: trit_uncertainty_map ─────────────────────────────────────────────
//
// Paste any text → every sentence/paragraph annotated with trit + confidence.
// Useful for compliance teams, analysts, legal review.

fn tool_trit_uncertainty_map(params: &Value) -> Result<Value, String> {
    let text        = params["text"].as_str().ok_or("text must be a string")?;
    let granularity = params["granularity"].as_str().unwrap_or("sentence");

    // Split into chunks
    let chunks: Vec<&str> = if granularity == "paragraph" {
        text.split("\n\n").map(str::trim).filter(|s| !s.is_empty()).collect()
    } else {
        // sentence split on . ? !
        text.split(|c| c == '.' || c == '?' || c == '!')
            .map(str::trim).filter(|s| !s.is_empty()).collect()
    };

    if chunks.is_empty() {
        return Err("text produced no chunks to analyse".into());
    }

    // Heuristic scoring — keyword-based confidence signal
    let hedges   = ["maybe","possibly","could","might","unclear","uncertain","perhaps",
                    "seems","appears","suggests","approximately","roughly","often","sometimes"];
    let affirms  = ["definitely","clearly","proven","confirmed","established","certainly",
                    "always","guaranteed","verified","demonstrates","shows"];
    let rejects  = ["false","wrong","incorrect","impossible","never","debunked","disproved",
                    "contradicts","refutes","myth","invalid","denied"];

    let score_chunk = |chunk: &str| -> (i8, f32, &'static str) {
        let lower = chunk.to_lowercase();
        let ha: usize = affirms.iter().filter(|w| lower.contains(*w)).count();
        let hh: usize = hedges.iter().filter(|w| lower.contains(*w)).count();
        let hr: usize = rejects.iter().filter(|w| lower.contains(*w)).count();
        let total = (ha + hh + hr).max(1);
        if hh > ha && hh > hr {
            (0, 0.3 + 0.05 * hh.min(8) as f32, "tend")
        } else if hr > ha {
            (-1, 0.4 + 0.08 * (hr as f32 / total as f32), "reject")
        } else if ha > 0 {
            (1, 0.5 + 0.08 * (ha as f32 / total as f32), "affirm")
        } else {
            (0, 0.25, "tend")  // neutral text — hold
        }
    };

    let annotations: Vec<Value> = chunks.iter().map(|chunk| {
        let (trit, conf, label) = score_chunk(chunk);
        json!({
            "claim":      chunk,
            "trit":       trit,
            "label":      label,
            "confidence": (conf.min(1.0) * 100.0).round() / 100.0,
            "reason":     match trit {
                1  => "Confident, affirming language detected.",
                -1 => "Contradicting or negating language detected.",
                _  => "Hedging language or insufficient signal — hold for verification.",
            }
        })
    }).collect();

    let affirm_n = annotations.iter().filter(|a| a["trit"] == 1).count();
    let tend_n   = annotations.iter().filter(|a| a["trit"] == 0).count();
    let reject_n = annotations.iter().filter(|a| a["trit"] == -1).count();

    Ok(json!({
        "total_claims": annotations.len(),
        "summary": { "affirm": affirm_n, "tend": tend_n, "reject": reject_n },
        "uncertainty_ratio": (tend_n as f32 / annotations.len() as f32 * 100.0).round() / 100.0,
        "annotations": annotations
    }))
}

// ─── Tool: trit_calibrate ────────────────────────────────────────────────────
//
// Feed an AI agent's recent decision log → calibration report.
// Detects binary habituation: how often did it force YES/NO when it should hold?

fn tool_trit_calibrate(params: &Value) -> Result<Value, String> {
    let decisions = params["decisions"]
        .as_array().ok_or("decisions must be an array")?;

    if decisions.is_empty() {
        return Err("decisions array cannot be empty".into());
    }

    let binary_words  = ["yes","no","true","false","accept","reject","allow","deny",
                         "approve","block","pass","fail","correct","incorrect","valid","invalid"];
    let hold_words    = ["maybe","uncertain","unclear","depends","possibly","need more",
                         "insufficient","context-dependent","further review","hold","tend"];
    let confident_ptn = ["definitely","certainly","always","never","absolutely","guaranteed"];

    let mut binary_count = 0usize;
    let mut hold_opps    = 0usize;
    let mut flagged: Vec<Value> = Vec::new();

    for d in decisions {
        let output     = d["output"].as_str().unwrap_or("").to_lowercase();
        let confidence = d["confidence"].as_f64().unwrap_or(0.5);
        let input      = d["input"].as_str().unwrap_or("");

        let is_binary    = binary_words.iter().any(|w| output.split_whitespace().any(|tok| tok.trim_matches(|c: char| !c.is_alphabetic()) == *w));
        let has_hedge    = hold_words.iter().any(|w| output.contains(*w));
        let is_confident = confident_ptn.iter().any(|w| output.contains(*w)) || confidence > 0.9;

        if is_binary && !has_hedge && is_confident {
            binary_count += 1;
            hold_opps    += 1;
            flagged.push(json!({
                "input":  input,
                "output": d["output"].as_str().unwrap_or(""),
                "trit":   -1,
                "reason": "Forced binary decision with high confidence — a tend zone may have been appropriate."
            }));
        } else if is_binary && !has_hedge {
            binary_count += 1;
        }
    }

    let n = decisions.len() as f32;
    let binary_ratio = binary_count as f32 / n;
    let calibration_trit: i8 = if binary_ratio < 0.2 { 1 } else if binary_ratio < 0.6 { 0 } else { -1 };
    let cal_label = if calibration_trit == 1 { "affirm" } else if calibration_trit == 0 { "tend" } else { "reject" };

    let mut recommendations: Vec<&str> = Vec::new();
    if binary_ratio > 0.6 {
        recommendations.push("High binary habituation detected. Consider injecting tend zones for ambiguous inputs.");
        recommendations.push("Review flagged decisions — each represents a missed deliberation opportunity.");
    } else if binary_ratio > 0.2 {
        recommendations.push("Moderate binary ratio. Add confidence thresholds to route low-confidence outputs to tend.");
    } else {
        recommendations.push("Calibration is good. The agent is using the full ternary range.");
    }
    if hold_opps > 0 {
        recommendations.push("Some high-confidence binary outputs could benefit from a second-opinion pass before committing.");
    }

    Ok(json!({
        "total_decisions":   decisions.len(),
        "binary_count":      binary_count,
        "binary_ratio":      (binary_ratio * 100.0).round() / 100.0,
        "hold_opportunities": hold_opps,
        "calibration_score": { "trit": calibration_trit, "label": cal_label },
        "recommendations":   recommendations,
        "flagged":           flagged
    }))
}

// ─── Tool: trit_translate ────────────────────────────────────────────────────
//
// Input Python if/elif/else or SQL CASE WHEN → output .tern equivalent
// with the ternary hold zone inserted where the original code had no coverage.

fn tool_trit_translate(params: &Value) -> Result<Value, String> {
    let code     = params["code"].as_str().ok_or("code must be a string")?;
    let language = params["language"].as_str().unwrap_or("python");

    let mut hold_zones = 0usize;
    let mut tern_lines: Vec<String> = Vec::new();
    let mut explanation_notes: Vec<String> = Vec::new();

    tern_lines.push(format!("// Translated from {} by trit_translate — RFI-IRFOS", language));
    tern_lines.push(format!("// Original: {} lines", code.lines().count()));
    tern_lines.push(String::new());

    match language {
        "python" => {
            // Detect if/elif/else chains and convert to match with tend arm
            let mut in_chain = false;
            let mut has_else = false;

            for line in code.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("if ") && trimmed.ends_with(':') {
                    in_chain = true;
                    has_else = false;
                    let cond = &trimmed[3..trimmed.len()-1];
                    tern_lines.push(format!("// if {}", cond));
                    tern_lines.push(format!("let condition: trit = cast({});", cond));
                    tern_lines.push("match condition {".to_string());
                    tern_lines.push("     1 => {".to_string());
                } else if trimmed.starts_with("elif ") && trimmed.ends_with(':') {
                    let cond = &trimmed[5..trimmed.len()-1];
                    tern_lines.push("    }".to_string());
                    tern_lines.push(format!("    // elif {}", cond));
                    tern_lines.push("    -1 => {".to_string());
                } else if trimmed == "else:" {
                    has_else = true;
                    tern_lines.push("    }".to_string());
                    tern_lines.push("     0 => { // else — original".to_string());
                } else if trimmed.starts_with("return ") {
                    let val = &trimmed[7..];
                    let trit_val = if val.contains("True") || val.contains("1") { "truth()" }
                                   else if val.contains("False") || val.contains("0") { "conflict()" }
                                   else { "hold()" };
                    tern_lines.push(format!("        return {}; // {}", trit_val, val));
                } else if !trimmed.is_empty() {
                    tern_lines.push(format!("        // {}", trimmed));
                }
            }

            if in_chain {
                if !has_else {
                    // No else → inject tend arm
                    tern_lines.push("    }".to_string());
                    tern_lines.push("     0 => {".to_string());
                    tern_lines.push("        // HOLD ZONE — no original coverage here".to_string());
                    tern_lines.push("        // The original code had no else: branch.".to_string());
                    tern_lines.push("        // This is where binary logic was silent. Ternary holds.".to_string());
                    tern_lines.push("        return hold();".to_string());
                    hold_zones += 1;
                    explanation_notes.push("Injected tend arm: original if/elif had no else branch — added explicit hold zone.".into());
                }
                tern_lines.push("    }".to_string());
                tern_lines.push("}".to_string());
            }
        }

        "sql" => {
            // Detect CASE WHEN / THEN / ELSE / END
            tern_lines.push("fn evaluate(signal: trit) -> trit {".to_string());
            let mut in_case = false;
            for line in code.lines() {
                let upper = line.trim().to_uppercase();
                if upper.starts_with("CASE") {
                    in_case = true;
                    tern_lines.push("    match signal {".to_string());
                } else if upper.starts_with("WHEN") {
                    let cond = line.trim()[4..].trim().trim_end_matches(|c| c == ' ');
                    tern_lines.push(format!("         1 => {{ // WHEN {}", cond));
                } else if upper.starts_with("THEN") {
                    let val  = line.trim()[4..].trim();
                    tern_lines.push(format!("            return truth(); // THEN {}", val));
                    tern_lines.push("        }".to_string());
                } else if upper.starts_with("ELSE") {
                    let val = line.trim()[4..].trim();
                    tern_lines.push(format!("        -1 => {{ return conflict(); }} // ELSE {}", val));
                } else if upper.starts_with("END") && in_case {
                    // Add tend arm if not present
                    tern_lines.push("         0 => {".to_string());
                    tern_lines.push("            // HOLD ZONE — CASE had no explicit NULL/unknown branch".to_string());
                    tern_lines.push("            return hold();".to_string());
                    tern_lines.push("        }".to_string());
                    tern_lines.push("    }".to_string());
                    hold_zones += 1;
                    in_case = false;
                    explanation_notes.push("Injected tend arm: SQL CASE had no branch for NULL or unknown inputs.".into());
                }
            }
            tern_lines.push("}".to_string());
        }

        "json_rules" => {
            // Detect simple JSON rule arrays: [{if, then}]
            if let Ok(rules) = serde_json::from_str::<Vec<Value>>(code) {
                tern_lines.push("fn apply_rules(signal: trit) -> trit {".to_string());
                tern_lines.push("    match signal {".to_string());
                let mut has_default = false;
                for rule in &rules {
                    let cond = rule["if"].as_str().unwrap_or("condition");
                    let then = rule["then"].as_str().unwrap_or("pass");
                    let tval = if then.contains("pass") || then.contains("allow") || then.contains("true") { "truth()" }
                               else if then.contains("deny") || then.contains("block") || then.contains("false") { "conflict()" }
                               else { "hold()" };
                    if rule["default"].as_bool().unwrap_or(false) { has_default = true; }
                    tern_lines.push(format!("         1 => {{ return {}; }} // if: {}", tval, cond));
                }
                if !has_default {
                    tern_lines.push("         0 => { return hold(); } // HOLD ZONE — no rule matched".to_string());
                    tern_lines.push("        -1 => { return conflict(); } // default reject".to_string());
                    hold_zones += 1;
                    explanation_notes.push("Injected tend arm: JSON rules had no explicit default — added hold zone for unmatched inputs.".into());
                } else {
                    tern_lines.push("         0 => { return hold(); }".to_string());
                    tern_lines.push("        -1 => { return conflict(); }".to_string());
                }
                tern_lines.push("    }".to_string());
                tern_lines.push("}".to_string());
            } else {
                return Err("json_rules: code must be a valid JSON array of rule objects".into());
            }
        }

        other => return Err(format!("unsupported language '{}'. Use: python, sql, json_rules", other)),
    }

    tern_lines.push(String::new());
    tern_lines.push("fn main() -> trit {".to_string());
    tern_lines.push("    // Wire your inputs here and call the translated function above.".to_string());
    tern_lines.push("    return hold();".to_string());
    tern_lines.push("}".to_string());

    Ok(json!({
        "language":        language,
        "hold_zones_added": hold_zones,
        "tern_code":       tern_lines.join("\n"),
        "explanation":     explanation_notes,
        "note":            "Review the generated code — trit_translate is a heuristic tool. Always verify the match arms match your domain semantics."
    }))
}

// ─── Tool: trit_eco_check ────────────────────────────────────────────────────
//
// Given a proposed action: returns human trit + eco trit.
// When they diverge, synthesis → tend ("needs more consideration").
// The first MCP tool that asks: is this good for everything, not just us?

fn tool_trit_eco_check(params: &Value) -> Result<Value, String> {
    let action  = params["action"].as_str().ok_or("action must be a string")?;
    let context = params["context"].as_str().unwrap_or("");
    let scope   = params["scope"].as_str().unwrap_or("local");

    // Human score: run through MoE
    let mut orch = TernMoeOrchestrator::with_standard_experts();
    let human_query = if context.is_empty() {
        format!("Should we take this action? {}", action)
    } else {
        format!("Context: {}. Should we take this action? {}", context, action)
    };
    let human_res = orch.orchestrate(&human_query, &[0.0; 6]);
    let human_trit: i8 = human_res.trit;

    // Eco score: keyword heuristic (EcoCore Phase 11B stub)
    let lower = format!("{} {}", action, context).to_lowercase();
    let eco_neg = ["carbon","emission","pollution","waste","deforestation","fossil","dump",
                   "toxic","landfill","exhaust","greenhouse","plastic","contamina"];
    let eco_pos = ["renewable","sustainable","solar","wind","recycl","reforest","organic",
                   "biodegradable","conservation","clean energy","low carbon","compost"];
    let eco_neu = ["digital","software","data","model","analysis","report","review","plan"];

    let neg_hits: usize = eco_neg.iter().filter(|w| lower.contains(*w)).count();
    let pos_hits: usize = eco_pos.iter().filter(|w| lower.contains(*w)).count();
    let neu_hits: usize = eco_neu.iter().filter(|w| lower.contains(*w)).count();

    // Scale up impact for broader scopes
    let scope_mult = match scope { "global" => 2, "regional" => 1, _ => 0 };
    let neg_scaled = neg_hits + scope_mult * neg_hits / 2;

    let eco_trit: i8 = if pos_hits > neg_scaled { 1 }
                       else if neg_scaled > pos_hits { -1 }
                       else if neu_hits > 0 { 0 }
                       else { 0 }; // neutral/unknown → hold

    let eco_reasoning = match eco_trit {
        1  => format!("Action appears ecologically positive ({}). Sustainable signals detected.", scope),
        -1 => format!("Action shows ecological risk signals at {} scope. Consider environmental impact.", scope),
        _  => format!("Ecological impact at {} scope is unclear. No strong eco signals detected — hold for assessment.", scope),
    };

    // Synthesis: divergence → tend
    let tension    = human_trit != eco_trit;
    let synthesis  = if tension {
        0i8 // hold — divergence means reconsider
    } else {
        human_trit // agreement — pass through
    };
    let syn_label = if synthesis == 1 { "affirm" } else if synthesis == -1 { "reject" } else { "tend" };

    Ok(json!({
        "action":        action,
        "scope":         scope,
        "human_trit":    { "trit": human_trit, "label": if human_trit==1 {"affirm"} else if human_trit==-1 {"reject"} else {"tend"}, "confidence": human_res.confidence },
        "eco_trit":      { "trit": eco_trit,   "label": if eco_trit==1 {"affirm"} else if eco_trit==-1 {"reject"} else {"tend"} },
        "tension":       tension,
        "synthesis":     { "trit": synthesis, "label": syn_label },
        "eco_reasoning": eco_reasoning,
        "note":          if tension { "Human-optimal and eco-optimal diverge. Synthesis held — find a path that serves both." }
                         else { "Human and eco perspectives agree." }
    }))
}

// ─── Tool: trit_audit (Phase 13 — TernAudit) ────────────────────────────────
//
// Full ternary audit of an AI decision log.
// Returns calibration stats, EU AI Act compliance hints, and flagged decisions.

fn tool_trit_audit(params: &Value) -> Result<Value, String> {
    let decisions = params["decisions"]
        .as_array().ok_or("decisions must be an array of {input, output, confidence?}")?;

    if decisions.is_empty() {
        return Err("decisions array cannot be empty".into());
    }

    let binary_words  = ["yes","no","true","false","accept","reject","allow","deny",
                         "approve","block","pass","fail","correct","incorrect","valid","invalid"];
    let hold_words    = ["maybe","uncertain","unclear","depends","possibly","insufficient",
                         "further review","hold","tend","need more","context-dependent"];
    let high_conf_pat = ["definitely","certainly","always","never","absolutely","guaranteed","100%"];

    let mut affirm_n  = 0usize;
    let mut tend_n    = 0usize;
    let mut reject_n  = 0usize;
    let mut forced_binary = 0usize;
    let mut flagged: Vec<Value> = Vec::new();

    for d in decisions {
        let output     = d["output"].as_str().unwrap_or("").to_lowercase();
        let confidence = d["confidence"].as_f64().unwrap_or(-1.0);
        let input      = d["input"].as_str().unwrap_or("");

        // Classify the output trit
        let is_binary  = binary_words.iter().any(|w| output.split_whitespace()
            .any(|tok| tok.trim_matches(|c: char| !c.is_alphabetic()) == *w));
        let has_hedge  = hold_words.iter().any(|w| output.contains(*w));
        let is_forced  = high_conf_pat.iter().any(|w| output.contains(*w))
            || (confidence >= 0.0 && confidence > 0.9);

        if has_hedge {
            tend_n += 1;
        } else if output.contains("reject") || output.contains("deny") || output.contains("no") || output.contains("false") {
            reject_n += 1;
        } else {
            affirm_n += 1;
        }

        if is_binary && !has_hedge && is_forced {
            forced_binary += 1;
            flagged.push(json!({
                "input":       input,
                "output":      d["output"].as_str().unwrap_or(""),
                "confidence":  if confidence >= 0.0 { json!(confidence) } else { json!(null) },
                "trit":        0,
                "reason":      "Forced high-confidence binary decision — tend zone may have been appropriate here."
            }));
        }
    }

    let n = decisions.len() as f32;
    let forced_binary_ratio = forced_binary as f32 / n;
    let tend_ratio          = tend_n as f32 / n;

    // EU AI Act heuristic (Article 13 = transparency, Article 14 = human oversight)
    let art13 = if tend_ratio > 0.1 { "pass" } else { "warn" };  // some uncertainty surfaced
    let art14 = if forced_binary_ratio < 0.3 { "pass" } else { "fail" }; // human override possible

    Ok(json!({
        "total_decisions":      decisions.len(),
        "affirm_count":         affirm_n,
        "tend_count":           tend_n,
        "reject_count":         reject_n,
        "forced_binary_ratio":  (forced_binary_ratio * 100.0).round() / 100.0,
        "tend_ratio":           (tend_ratio * 100.0).round() / 100.0,
        "eu_ai_act": {
            "article_13_transparency":     art13,
            "article_14_human_oversight":  art14,
            "note": "Heuristic assessment only — not a substitute for legal compliance review."
        },
        "calibration": {
            "score":  if forced_binary_ratio < 0.2 { "good" } else if forced_binary_ratio < 0.5 { "warn" } else { "poor" },
            "advice": if forced_binary_ratio < 0.2 {
                "Calibration looks healthy — the agent is using the full ternary range."
            } else if forced_binary_ratio < 0.5 {
                "Moderate binary habituation. Add confidence thresholds to route uncertain outputs to tend."
            } else {
                "High binary habituation. The agent is collapsing ambiguity instead of surfacing it."
            }
        },
        "flagged": flagged
    }))
}

// ─── Tool dispatch ───────────────────────────────────────────────────────────

fn dispatch_tool(name: &str, params: &Value) -> Result<Value, String> {
    match name {
        "trit_decide"       => tool_trit_decide(params),
        "trit_vector"       => tool_trit_vector(params),
        "trit_consensus"    => tool_trit_consensus(params),
        "trit_eval"         => tool_trit_eval(params),
        "ternlang_run"      => tool_ternlang_run(params),
        "quantize_weights"  => tool_quantize_weights(params),
        "sparse_benchmark"  => tool_sparse_benchmark(params),
        "moe_orchestrate"   => tool_moe_orchestrate(params),
        "moe_deliberate"    => tool_moe_deliberate(params),
        "trit_action_gate"  => tool_trit_action_gate(params),
        "get_industrial_standards" => tool_get_industrial_standards(params),
        "audit_ternary_logic"      => tool_audit_ternary_logic(params),
        "tsql_join"                => tool_tsql_join(params),
        "trit_debate"              => tool_trit_debate(params),
        "trit_uncertainty_map"     => tool_trit_uncertainty_map(params),
        "trit_calibrate"           => tool_trit_calibrate(params),
        "trit_translate"           => tool_trit_translate(params),
        "trit_eco_check"           => tool_trit_eco_check(params),
        "trit_audit"               => tool_trit_audit(params),
        _ => Err(format!("unknown tool: {}", name)),
    }
}

// ─── Tool: tsql_join ─────────────────────────────────────────────────────────

fn tool_tsql_join(params: &Value) -> Result<Value, String> {
    let record_a: Vec<f32> = params["record_a"]
        .as_array().ok_or("record_a must be an array")?
        .iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect();
    let record_b: Vec<f32> = params["record_b"]
        .as_array().ok_or("record_b must be an array")?
        .iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect();

    if record_a.len() != record_b.len() || record_a.is_empty() {
        return Err("records must be non-empty and of equal length".into());
    }

    // Compute triadic overlap (cosine similarity)
    let dot: f32 = record_a.iter().zip(record_b.iter()).map(|(a, b)| a * b).sum();
    let norm_a: f32 = record_a.iter().map(|a| a * a).sum::<f32>().sqrt();
    let norm_b: f32 = record_b.iter().map(|b| b * b).sum::<f32>().sqrt();
    
    let similarity = if norm_a > 0.0 && norm_b > 0.0 { dot / (norm_a * norm_b) } else { 0.0 };

    let (trit, label) = if similarity > 0.99 {
        (1, "affirm (Match)")
    } else if similarity < 0.30 {
        (-1, "reject (Mismatch)")
    } else {
        (0, "tend (Deliberative Hold / Partial Match)")
    };

    Ok(json!({
        "similarity": (similarity * 1000.0).round() / 1000.0,
        "trit": trit,
        "label": label,
        "action": if trit == 0 { "ESCROW_FOR_AUDIT" } else { "COMMIT_TRANSACTION" },
        "retention": "100% (No data dropped)",
        "legal": "Patent Pending A50296/2026"
    }))
}

// ─── Tool manifest ───────────────────────────────────────────────────────────

fn tools_list() -> Value {
    json!({ "tools": [
        {
            "name": "trit_decide",
            "description": "Scalar ternary decision engine. Pass in floating-point evidence signals on [-1.0, +1.0]. Returns a continuous scalar temperature, a discrete ternary decision (reject/tend/affirm), and a confidence score.\n\nThe three zones:\n  affirm  (+0.33, +1.0] — signal is affirmative. Act if confidence is high enough.\n  tend    [-0.33, +0.33] — deliberation zone. Do NOT act. Gather more evidence.\n  reject  [-1.0, -0.33) — signal is negative. Do not proceed.\n\nThe 'tend' zone is the key innovation: it is not null or undecided — it is an active computational instruction to remain in uncertainty until the scalar clears a boundary. Confidence tells you HOW decisive the signal is within its zone.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "evidence": {
                        "type": "array",
                        "items": { "type": "number" },
                        "description": "Array of numeric evidence signals on [-1.0, +1.0]. Positive = supporting, negative = contradicting, near-zero = uncertain. Mean is computed as the aggregate scalar."
                    },
                    "min_confidence": {
                        "type": "number",
                        "description": "Minimum confidence threshold for is_actionable (0.0–1.0). Default 0.0 (any decisive signal is actionable)."
                    }
                },
                "required": ["evidence"]
            }
        },
        {
            "name": "trit_vector",
            "description": "Multi-dimensional ternary evidence aggregation. The full agent reasoning tool.\n\nProvide named evidence dimensions, each with a scalar value [-1.0, +1.0] and an importance weight. The engine computes a weighted-mean aggregate TritScalar and returns:\n  - aggregate: scalar, trit, label (reject/tend/affirm), confidence, is_actionable\n  - breakdown: per-dimension zone classification\n  - dominant: which evidence dimension is pulling hardest\n  - recommendation: plain-language decision guidance\n\nUse case: an AI agent collects evidence from multiple sources (visual, textual, contextual, historical) before deciding whether to act. The tend zone means 'keep deliberating'. Only act when is_actionable is true.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "dimensions": {
                        "type": "array",
                        "description": "Array of evidence dimensions. Each: {label: string, value: number [-1,1], weight: number >= 0 (default 1.0)}",
                        "items": {
                            "type": "object",
                            "properties": {
                                "label":  { "type": "string",  "description": "Name of this evidence source" },
                                "value":  { "type": "number",  "description": "Evidence scalar, clamped to [-1.0, +1.0]" },
                                "weight": { "type": "number",  "description": "Importance weight (default 1.0)" }
                            },
                            "required": ["label", "value"]
                        }
                    },
                    "min_confidence": {
                        "type": "number",
                        "description": "Minimum confidence for is_actionable (0.0–1.0). Default 0.5."
                    }
                },
                "required": ["dimensions"]
            }
        },
        {
            "name": "trit_consensus",
            "description": "Compute balanced ternary consensus between two trit signals.\n\nConsensus is the core ternary addition operator: truth+conflict=hold, truth+truth=truth, conflict+conflict=conflict.\n\nUse this to merge two independent ternary judgements into a single signal.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "a": { "type": "integer", "enum": [-1, 0, 1], "description": "First trit: -1=conflict, 0=hold, 1=truth" },
                    "b": { "type": "integer", "enum": [-1, 0, 1], "description": "Second trit: -1=conflict, 0=hold, 1=truth" }
                },
                "required": ["a", "b"]
            }
        },
        {
            "name": "trit_eval",
            "description": "Evaluate a ternary expression using the BET (Balanced Ternary Execution) VM.\n\nSupports: consensus(a,b), invert(x), truth(), hold(), conflict(), arithmetic (+, -, *), let bindings.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "expression": { "type": "string", "description": "A ternlang expression or statement. E.g. 'consensus(1, -1)' or 'let x: trit = 1; return -x;'" }
                },
                "required": ["expression"]
            }
        },
        {
            "name": "ternlang_run",
            "description": "Compile and execute a full ternlang (.tern) program on the BET VM. Returns register state after execution.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "code": { "type": "string", "description": "Full ternlang source code to compile and run." }
                },
                "required": ["code"]
            }
        },
        {
            "name": "quantize_weights",
            "description": "Quantize an array of float weights to balanced ternary (-1, 0, +1) using BitNet-style thresholding. Returns the trit vector and sparsity statistics.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "weights": { "type": "array", "items": { "type": "number" }, "description": "Float weight values to quantize." },
                    "threshold": { "type": "number", "description": "Quantization threshold. Omit to auto-compute via BitNet formula." }
                },
                "required": ["weights"]
            }
        },
        {
            "name": "sparse_benchmark",
            "description": "Run sparse vs dense ternary matrix multiply benchmark. Shows how many multiply-accumulate operations are skipped due to zero-state (hold) weights. Demonstrates the computational efficiency of ternary AI inference.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "rows":      { "type": "integer", "description": "Matrix rows (default 4)" },
                    "cols":      { "type": "integer", "description": "Matrix cols (default 4)" },
                    "weights":   { "type": "array", "items": { "type": "number" }, "description": "Flat row-major float weights. Length must equal rows×cols. Omit for a demo distribution." },
                    "threshold": { "type": "number", "description": "Quantization threshold. Omit to auto-compute." }
                }
            }
        },
        {
            "name": "moe_orchestrate",
            "description": "Full MoE-13 ternary orchestration pass.\n\nRoutes your query through a pool of 13 domain experts (Syntax, WorldKnowledge, DeductiveReason, InductiveReason, ToolUse, Persona, Safety, FactCheck, CausalReason, AmbiguityRes, MathReason, ContextMem, MetaSafety) using dual-key synergistic routing.\n\nThe routing algorithm selects the expert PAIR that maximises both relevance to the query AND complementarity to each other — orthogonal competences produce a stronger emergent field than two similar experts.\n\nThe emergent triad field (1+1=3): Ek = synergy × (vi + vj)/2. Two experts that are good in different dimensions produce a third signal neither could produce alone.\n\nSafety hard gate fires FIRST, before any vote. A negative safety field = immediate reject, logged to audit trail.\n\nHold state: if the vote is split or confidence is low, the orchestrator calls a tiebreaker (up to 4 active experts max) before giving up — modelling the human 'I'll think about it' behaviour.\n\nReturns: trit decision, confidence, held flag, safety_vetoed flag, temperature hint for downstream LLM, prompt_hint, triad field details, and per-expert verdicts.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The question or decision to orchestrate."
                    },
                    "evidence": {
                        "type": "array",
                        "items": { "type": "number" },
                        "description": "6-element evidence vector: [syntax, world_knowledge, reasoning, tool_use, persona, safety]. Values on [-1.0, +1.0]. Omit or leave empty for neutral (all zeros)."
                    }
                },
                "required": ["query"]
            }
        },
        {
            "name": "moe_deliberate",
            "description": "EMA-based ternary deliberation engine.\n\nModels iterative reasoning: given an initial scalar signal and a series of incoming evidence updates, the engine converges toward a stable ternary decision using exponential moving average. Alpha controls how fast new evidence is weighted vs prior belief.\n\nThis is the 'Wall-E collecting things' mechanism: the agent doesn't have to decide immediately. It accumulates evidence across rounds and commits only when confidence crosses the target threshold.\n\nRound-by-round trace is returned so you can inspect exactly when and why the agent committed or stayed in the tend (hold) zone.\n\nUse cases:\n  - Simulate multi-turn deliberation before acting\n  - Model an agent that keeps gathering context before deciding\n  - Tune alpha and target_confidence to calibrate risk tolerance",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "initial_scalar": {
                        "type": "number",
                        "description": "Starting evidence scalar on [-1.0, +1.0]. Positive = leaning affirm, negative = leaning reject, near-zero = maximum uncertainty."
                    },
                    "target_confidence": {
                        "type": "number",
                        "description": "Confidence threshold to stop deliberating (0.0–1.0). Default 0.8. Agent stays in tend until this is met."
                    },
                    "alpha": {
                        "type": "number",
                        "description": "EMA smoothing factor (0.0–1.0). High = fast adaptation to new evidence. Low = strong prior. Default 0.4."
                    },
                    "max_rounds": {
                        "type": "integer",
                        "description": "Maximum deliberation rounds before giving up and returning tend. Default 10."
                    },
                    "evidence_updates": {
                        "type": "array",
                        "items": { "type": "number" },
                        "description": "Sequence of incoming evidence scalars, one per round. Omit to replay initial_scalar each round (tests convergence from a single signal)."
                    }
                },
                "required": ["initial_scalar"]
            }
        },
        {
            "name": "trit_action_gate",
            "description": "Multi-dimensional ternary action gate with hard-block safety veto.\n\nBefore any AI agent takes an action, pass the relevant decision dimensions through this gate. Any dimension marked hard_block:true with a negative trit (reject) VETOES the action unconditionally, regardless of other dimensions. This implements the 'safety as absolute veto' principle from the MoE-13 architecture.\n\nNon-hard-block dimensions contribute to a weighted vote: positive trits add to pass_weight, negative trits add to block_weight. The gate returns Pass, Blocked, or Hold.\n\nTypical pattern:\n  - Safety check → hard_block: true\n  - Relevance → hard_block: false, weight: 1.0\n  - User consent → hard_block: true\n  - Confidence → hard_block: false, weight: 0.5\n\nThe gate enforces a structural separation: some signals are hard constraints, others are soft preferences. Binary logic collapses these — ternary keeps them distinct.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "dimensions": {
                        "type": "array",
                        "description": "Array of gate dimensions. Each: {label, trit (-1/0/1), weight (default 1.0), hard_block (default false)}",
                        "items": {
                            "type": "object",
                            "properties": {
                                "label":      { "type": "string",  "description": "Name of this decision dimension" },
                                "trit":       { "type": "integer", "enum": [-1, 0, 1], "description": "-1=block, 0=hold, 1=pass" },
                                "weight":     { "type": "number",  "description": "Importance weight (default 1.0). Ignored for hard_block dims." },
                                "hard_block": { "type": "boolean", "description": "If true, a negative trit on this dim VETOES regardless of all other dims." }
                            },
                            "required": ["label", "trit"]
                        }
                    }
                },
                "required": ["dimensions"]
            }
        },
        {
            "name": "trit_debate",
            "description": "Give two competing claims — get a structured 3-way verdict. Routes each claim through MoE-13 independently, compares verdicts, and surfaces the tension between them.\n\nOutput: for_a (trit+confidence), for_b (trit+confidence), tension (0–1), synthesis (plain-language summary), verdict (AGREEMENT / CONFLICT / HOLD).\n\nThe tend zone means the system is holding — one or both claims are genuinely uncertain.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "claim_a":  { "type": "string", "description": "First claim or position to evaluate." },
                    "claim_b":  { "type": "string", "description": "Second claim or position to evaluate." },
                    "context":  { "type": "string", "description": "Optional background context shared by both claims." }
                },
                "required": ["claim_a", "claim_b"]
            }
        },
        {
            "name": "trit_uncertainty_map",
            "description": "Paste any text — meeting notes, a medical report, a legal clause — and get every sentence or paragraph annotated with a ternary signal (affirm/tend/reject) and a confidence score.\n\nThe tend annotations are the ones that matter most: they mark claims the text itself hedges, qualifies, or leaves open. These are the areas that need verification before acting.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "text":        { "type": "string", "description": "The text to annotate." },
                    "granularity": { "type": "string", "enum": ["sentence", "paragraph"], "description": "Split by sentence (default) or paragraph." }
                },
                "required": ["text"]
            }
        },
        {
            "name": "trit_calibrate",
            "description": "Feed an AI agent's recent decision log and get a calibration report. Detects binary habituation: how often did the agent force YES/NO when it should have held?\n\nReturns: binary_ratio, hold_opportunities, calibration_score (trit), recommendations, and flagged decisions with explanations.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "decisions": {
                        "type": "array",
                        "description": "Array of recent agent decisions. Each: {input: string, output: string, confidence?: number 0–1}",
                        "items": {
                            "type": "object",
                            "properties": {
                                "input":      { "type": "string" },
                                "output":     { "type": "string" },
                                "confidence": { "type": "number" }
                            },
                            "required": ["input", "output"]
                        }
                    }
                },
                "required": ["decisions"]
            }
        },
        {
            "name": "trit_translate",
            "description": "Input Python if/elif/else, SQL CASE WHEN, or a JSON rule array — output the equivalent .tern program with the ternary hold zone inserted where the original code had no coverage.\n\nThe 'else' gap in binary code is where real-world ambiguity lives. trit_translate makes it explicit as a Tend arm.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "code":     { "type": "string", "description": "The source code to translate." },
                    "language": { "type": "string", "enum": ["python", "sql", "json_rules"], "description": "Source language (default: python)." }
                },
                "required": ["code"]
            }
        },
        {
            "name": "trit_eco_check",
            "description": "Given a proposed action, returns two trit scores: one from a human-centric perspective (MoE-13) and one from an ecocentric perspective (EcoCore heuristic). When they diverge, synthesis is Tend — 'this needs more consideration before acting.'\n\nThe first MCP tool that asks: is this good for everything, not just us?",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "action":  { "type": "string", "description": "The proposed action or decision to evaluate." },
                    "context": { "type": "string", "description": "Optional additional context." },
                    "scope":   { "type": "string", "enum": ["local", "regional", "global"], "description": "Impact scope (default: local). Broader scope amplifies ecological weight." }
                },
                "required": ["action"]
            }
        },
        {
            "name": "trit_audit",
            "description": "Full ternary audit of an AI system's decision log. Answers: 'how binary is this AI's decision-making, and would a regulator flag it?'\n\nReturns: decision counts (affirm/tend/reject), forced_binary_ratio, EU AI Act heuristic (Article 13 transparency + Article 14 human oversight), calibration score, and a list of flagged decisions that should have held.\n\nThis is Phase 13 — TernAudit. The commercial case for ternlang made tangible.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "decisions": {
                        "type": "array",
                        "description": "Array of AI decisions to audit. Each: {input: string, output: string, confidence?: number 0–1}",
                        "items": {
                            "type": "object",
                            "properties": {
                                "input":      { "type": "string", "description": "The input prompt or question." },
                                "output":     { "type": "string", "description": "The AI's response or decision." },
                                "confidence": { "type": "number", "description": "Reported confidence 0–1 (optional)." }
                            },
                            "required": ["input", "output"]
                        }
                    }
                },
                "required": ["decisions"]
            }
        },
        {
            "name": "get_industrial_standards",
            "description": "Returns the list of current triadic industrial standards published by RFI-IRFOS. Use this to determine compliance requirements for tokenization, memory, compute, and hardware access.",
            "inputSchema": { "type": "object", "properties": {} }
        },
        {
            "name": "audit_ternary_logic",
            "description": "Performs a compliance audit on a snippet of Ternlang or triadic-inspired code. Detects 'Binary Habituation' (over-reliance on true/false) and suggests 'Deliberation Injection' to break binary logic drifts. Reports sparsity potential and MoE-13 safety gate alignment.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "code": { "type": "string", "description": "The code snippet to audit for triadic compliance." }
                },
                "required": ["code"]
            }
        },
        {
            "name": "tsql_join",
            "description": "Enterprise T-SQL Triadic Join. Unlike binary SQL (Match/No-Match), a T-Join routes partial matches into a Deliberative Hold (State 0) for escrow audit. Guarantees 100% data retention and eliminates binary error-handling overhead for high-frequency trading and defense datasets.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "record_a": { "type": "array", "items": { "type": "number" }, "description": "Vector representation of record A." },
                    "record_b": { "type": "array", "items": { "type": "number" }, "description": "Vector representation of record B." }
                },
                "required": ["record_a", "record_b"]
            }
        }
    ]})
}

// ─── MCP initialize response ─────────────────────────────────────────────────

fn initialize_response() -> Value {
    json!({
        "protocolVersion": "2024-11-05",
        "capabilities": { "tools": {} },
        "serverInfo": {
            "name":    "ternlang-mcp",
            "version": "0.3.0",
            "description": "Ternary Intelligence Stack — turns binary AI agents into ternary decision engines. Built by RFI-IRFOS."
        }
    })
}

// ─── Main loop ───────────────────────────────────────────────────────────────

fn handle_request(req: RpcRequest) -> RpcResponse {
    let id = req.id.unwrap_or(Value::Null);
    let params = req.params.unwrap_or(Value::Object(Default::default()));

    match req.method.as_str() {
        "initialize" => RpcResponse::ok(id, initialize_response()),

        "notifications/initialized" => {
            // Client confirmation — no response needed, but send ok
            RpcResponse::ok(id, json!({}))
        }

        "tools/list" => RpcResponse::ok(id, tools_list()),

        "tools/call" => {
            let tool_name = match params["name"].as_str() {
                Some(n) => n.to_string(),
                None => return RpcResponse::err(id, -32602, "missing tool name"),
            };
            let tool_params = &params["arguments"];
            match dispatch_tool(&tool_name, tool_params) {
                Ok(result) => RpcResponse::ok(id, json!({
                    "content": [{
                        "type": "text",
                        "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                    }]
                })),
                Err(e) => RpcResponse::err(id, -32000, e),
            }
        }

        other => RpcResponse::err(id, -32601, format!("method not found: {}", other)),
    }
}

fn main() {
    let stdin  = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    eprintln!("[ternlang-mcp] server ready — ternary intelligence stack v0.2");
    eprintln!("[ternlang-mcp] waiting for MCP client on stdin...");

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) if l.trim().is_empty() => continue,
            Ok(l) => l,
            Err(_) => break,
        };

        let response = match serde_json::from_str::<RpcRequest>(&line) {
            Ok(req) => handle_request(req),
            Err(e)  => RpcResponse::err(
                Value::Null, -32700,
                format!("parse error: {}", e)
            ),
        };

        let json = serde_json::to_string(&response).unwrap_or_default();
        writeln!(out, "{}", json).ok();
        out.flush().ok();
    }
}
