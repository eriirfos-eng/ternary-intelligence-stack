use candle_core::{Device, DType, Result, Tensor};
use candle_nn::{Optimizer, VarBuilder, loss, VarMap};
use moe_llm_core::model::{Transformer, TransformerConfig, clear_routing_capture, take_routing_capture, clear_entropy_capture, take_entropy_capture, clear_lb_capture, take_lb_capture, clear_tlight_capture, take_tlight_capture, clear_div_capture, take_div_capture, take_div_log_capture, take_div_f32_log_capture, set_div_enabled, set_gate_diversity_scale};
use moe_llm_core::tokenizer::BpeTokenizer;
use moe_llm_core::evolution::EvolutionManager;
use moe_llm_core::mycelium::MyceliumModule;
use moe_llm_core::wald::{WaldModule, format_wald_line};
use moe_llm_core::mandelbrot::{MandelbrotSurgery, format_mandelbrot_line};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use rayon::ThreadPoolBuilder;
use serde_json::{Value, json};
use std::collections::HashMap;

struct TrainFlags {
    /// Load-balancing loss weight. 0.0 = disabled entirely.
    lb_weight: f64,
    /// Epochs over which LB ramps from 0.1× to 1.0× when re-enabled from disabled.
    lb_ramp_epochs: usize,
    /// Seed F32 shadow expert weights with orthogonal Householder rotations on startup.
    seed_experts: bool,
    /// Override the scheduled DIV loss weight with a fixed value for this run.
    div_weight_override: Option<f64>,
    /// Fixed per-expert gate logit bias spread (0.0 = disabled). Expert E-1 gets +scale,
    /// expert 0 gets -scale. Breaks Nash routing symmetry without learning.
    gate_diversity: f32,
    /// Root directory for all data/model/log paths. Default: "." (run from project root).
    /// On a rental machine: --root=/path/to/albert-moe-13
    root: String,
}

fn parse_args() -> TrainFlags {
    let args: Vec<String> = std::env::args().collect();
    let mut flags = TrainFlags {
        lb_weight:           0.03,
        lb_ramp_epochs:      5,
        seed_experts:        false,
        div_weight_override: None,
        gate_diversity:      0.0,
        root:                ".".to_string(),
    };
    for arg in &args[1..] {
        match arg.as_str() {
            "--lb-disable"   => flags.lb_weight = 0.0,
            "--seed-experts" => flags.seed_experts = true,
            _ => {
                if let Some(v) = arg.strip_prefix("--lb-weight=") {
                    if let Ok(w) = v.parse::<f64>() { flags.lb_weight = w; }
                } else if let Some(v) = arg.strip_prefix("--lb-ramp=") {
                    if let Ok(e) = v.parse::<usize>() { flags.lb_ramp_epochs = e; }
                } else if let Some(v) = arg.strip_prefix("--div-weight=") {
                    if let Ok(w) = v.parse::<f64>() { flags.div_weight_override = Some(w); }
                } else if let Some(v) = arg.strip_prefix("--gate-diversity=") {
                    if let Ok(w) = v.parse::<f32>() { flags.gate_diversity = w; }
                } else if let Some(v) = arg.strip_prefix("--root=") {
                    flags.root = v.trim_end_matches('/').to_string();
                }
            }
        }
    }
    flags
}

// Gradient clipping + collapse detection — whitepaper §11.4 (Ternary Training Innovations)

// Gradient clipping threshold — prevents weight explosions.
// Healthy grad norms for this model are typically 0.1–2.0.
const MAX_GRAD_NORM: f32 = 1.0;

// If a single batch loss exceeds this, the weights have already exploded.
// ln(vocab=32000) ≈ 10.373 — anything above 11.0 is catastrophic.
const LOSS_EXPLOSION_THRESHOLD: f32 = 11.0;

// If the epoch-average loss sits at or above this for COLLAPSE_STREAK_LIMIT
// consecutive epochs the model has collapsed to uniform distribution.
// 32k vocab random baseline: ln(32000) ≈ 10.373. Threshold set at 11.0 —
// well above the vocabulary-transfer plateau band (~10.35) so normal
// post-vocab-expansion epochs don't trigger false collapse detection.
// Old value (10.2) was calibrated for 8k vocab; recalibrated for 32k here.
const COLLAPSE_THRESHOLD:    f32 = 11.0;
const COLLAPSE_STREAK_LIMIT: u32 = 2;

// Gradient accumulation: accumulate this many micro-batch losses before
// calling backward() + opt.step(). Equivalent to batch_size=N at no
// extra memory cost — each forward graph is summed into accum_loss,
// then one backward pass covers all N sequences. Mirrors the ternary
// hold state: withhold the weight update until enough evidence accumulates.
const GRAD_ACCUM_STEPS: usize = 4;
const BATCH_SIZE: usize = 4;   // CTX=256: attention is O(seq²), T4 16GB needs batch ≤ 4
// Direct LR boost applied after Adam's step for cold layers (norm < THRESHOLD).
// Adam normalizes away gradient amplification via its second-moment denominator,
// so we bypass it entirely: a sign-normalized SGD step with lr = current_lr * cold_boost.
// Self-terminating: once a layer's norm exceeds THRESHOLD the condition is false.
// cold_boost is driven by wald_amplify_scale (updated each epoch from WALD severity).
// At severity=0.983 → scale=47×. Floor of 8× prevents under-reinforcement at low severity.

// TTL burst detection — freeze logit modifiers when per-layer grad norm spikes.
// GRAD_NORM_EMA_ALPHA: ~1/α ≈ 50-step window for the per-layer baseline EMA.
// BURST_RATIO_THRESHOLD: fire when current_norm / baseline > this value.
// TTL_FREEZE_GRAD_STEPS: freeze duration in optimizer steps; multiply by GRAD_ACCUM_STEPS
//   for the update() call count passed to TrafficLight::freeze().
// MAX_BURSTS_PER_EPOCH: safety circuit — disable further freezes after this many per epoch per layer.
const GRAD_NORM_EMA_ALPHA: f32 = 0.02;
const BURST_RATIO_THRESHOLD: f32 = 5.0;
const TTL_FREEZE_GRAD_STEPS: usize = 50;
const MAX_BURSTS_PER_EPOCH: usize = 5;

// Expert output divergence loss — breaks Universal Nash by pushing expert outputs apart.
// Continuous restoring force in output space; operates independent of routing impulses.
// Linear decay from START to END over DIV_DECAY_START..DIV_DECAY_END epochs, then off.
const DIV_LOSS_WEIGHT_START: f64 = 5e-2;
const DIV_LOSS_WEIGHT_END:   f64 = 1e-4;
const DIV_DECAY_START_EPOCH: usize = 105; // re-enabled from ep105 with seed biases live
const DIV_DECAY_END_EPOCH:   usize = 125; // exclusive: epoch 125+ weight = 0.0

fn div_loss_weight(epoch: usize) -> f64 {
    if epoch < DIV_DECAY_START_EPOCH {
        DIV_LOSS_WEIGHT_START
    } else if epoch >= DIV_DECAY_END_EPOCH {
        0.0
    } else {
        let t = (epoch - DIV_DECAY_START_EPOCH) as f64
              / (DIV_DECAY_END_EPOCH - DIV_DECAY_START_EPOCH - 1) as f64;
        DIV_LOSS_WEIGHT_START * (1.0 - t) + DIV_LOSS_WEIGHT_END * t
    }
}

fn timestamp() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

// Cosine annealing with warm restart — whitepaper §11.4
fn cosine_lr(base_lr: f64, min_lr: f64, step: usize, total_steps: usize) -> f64 {
    let t = step as f64 / total_steps.max(1) as f64;
    min_lr + 0.5 * (base_lr - min_lr) * (1.0 + (std::f64::consts::PI * t).cos())
}

/// Save the current varmap weights to a file.
fn save_checkpoint(varmap: &VarMap, path: &str) -> Result<()> {
    let all_vars = varmap.data().lock().unwrap();
    let mut tensor_map = HashMap::new();
    for (name, var) in all_vars.iter() {
        tensor_map.insert(name.clone(), var.as_tensor().clone());
    }
    candle_core::safetensors::save(&tensor_map, path)?;
    Ok(())
}

/// Load checkpoint weights into an existing varmap (shape-guarded).
fn load_checkpoint(varmap: &VarMap, path: &str, device: &Device) -> Result<usize> {
    let checkpoint_data = candle_core::safetensors::load(path, device)?;
    let all_vars = varmap.data().lock().unwrap();
    let mut loaded = 0usize;
    for (name, var) in all_vars.iter() {
        if let Some(tensor) = checkpoint_data.get(name) {
            if tensor.shape() == var.shape() {
                var.set(tensor)?;
                loaded += 1;
            }
        }
    }
    Ok(loaded)
}

/// Compute the global L2 gradient norm across all variables.
/// Returns 0.0 if no gradients are present.
fn global_grad_norm(varmap: &VarMap, grads: &candle_core::backprop::GradStore) -> f32 {
    let mut sq_sum = 0.0_f32;
    let all_vars = varmap.all_vars();
    for var in &all_vars {
        if let Some(g) = grads.get(var.as_tensor()) {
            if let Ok(sq) = g.sqr().and_then(|t| t.sum_all()).and_then(|t| t.to_scalar::<f32>()) {
                if sq.is_finite() {
                    sq_sum += sq;
                }
            }
        }
    }
    sq_sum.sqrt()
}

/// Compute L2 gradient norm per transformer layer.
/// Returns a Vec of length num_layers; each entry is sqrt(sum of squared grads
/// for all tensors whose name starts with "blocks.N.").
fn per_layer_grad_norm(varmap: &VarMap, grads: &candle_core::backprop::GradStore, num_layers: usize) -> Vec<f32> {
    let mut sq: Vec<f32> = vec![0.0; num_layers];
    let all_vars = varmap.data().lock().unwrap();
    for (name, var) in all_vars.iter() {
        let li = name.strip_prefix("blocks.")
            .and_then(|s| s.split('.').next())
            .and_then(|s| s.parse::<usize>().ok());
        if let Some(i) = li {
            if i < num_layers {
                if let Some(g) = grads.get(var.as_tensor()) {
                    if let Ok(s) = g.sqr().and_then(|t| t.sum_all()).and_then(|t| t.to_scalar::<f32>()) {
                        if s.is_finite() { sq[i] += s; }
                    }
                }
            }
        }
    }
    sq.iter().map(|&s| s.sqrt()).collect()
}

/// Per-layer variance of expert weight gradient norms — the definitive H3 diagnostic.
/// Uses naming convention blocks.{L}.moe.experts.{E}.{c_fc|c_proj}.weight.
/// If variance is non-zero, DIV gradient is differentially flowing to experts.
/// If all experts have identical grad norms, gradient is uniform (Nash collapse in grad space).
fn expert_grad_variance(
    varmap: &VarMap,
    grads: &candle_core::backprop::GradStore,
    num_layers: usize,
    num_experts: usize,
) -> Vec<f32> {
    let all_vars = varmap.data().lock().unwrap();
    let mut layer_expert_sq: Vec<Vec<f32>> = vec![vec![0.0; num_experts]; num_layers];
    for (name, var) in all_vars.iter() {
        // Match: blocks.{L}.moe.experts.{E}.c_fc.weight or c_proj.weight
        let parts: Vec<&str> = name.split('.').collect();
        if parts.len() >= 6
            && parts[0] == "blocks"
            && parts[2] == "moe"
            && parts[3] == "experts"
            && (parts[5] == "c_fc" || parts[5] == "c_proj")
            && parts.last() == Some(&"weight")
        {
            if let (Ok(li), Ok(ei)) = (parts[1].parse::<usize>(), parts[4].parse::<usize>()) {
                if li < num_layers && ei < num_experts {
                    if let Some(g) = grads.get(var.as_tensor()) {
                        if let Ok(s) = g.sqr().and_then(|t| t.sum_all()).and_then(|t| t.to_scalar::<f32>()) {
                            if s.is_finite() { layer_expert_sq[li][ei] += s; }
                        }
                    }
                }
            }
        }
    }
    layer_expert_sq.iter().map(|expert_sq| {
        let norms: Vec<f32> = expert_sq.iter().map(|&s| s.sqrt()).collect();
        let mean = norms.iter().sum::<f32>() / norms.len() as f32;
        norms.iter().map(|&n| (n - mean).powi(2)).sum::<f32>() / norms.len() as f32
    }).collect()
}

/// Per-layer ratio: mean DIV-derived expert gradient norm vs expected weight-decay pull.
/// Returns (grad_mean, wd_equiv_mean, ratio) per layer.
/// Ratio < 1.0 means weight decay dominates → F32-direct cannot grow DIVF32.
/// Uses the same naming convention as expert_grad_variance.
fn expert_wd_ratio(
    varmap: &VarMap,
    grads: &candle_core::backprop::GradStore,
    num_layers: usize,
    num_experts: usize,
    wd: f64,
) -> Vec<(f32, f32, f32)> {
    let all_vars = varmap.data().lock().unwrap();
    let mut layer_expert_grad_sq:   Vec<Vec<f32>> = vec![vec![0.0; num_experts]; num_layers];
    let mut layer_expert_weight_sq: Vec<Vec<f32>> = vec![vec![0.0; num_experts]; num_layers];
    for (name, var) in all_vars.iter() {
        let parts: Vec<&str> = name.split('.').collect();
        if parts.len() >= 6
            && parts[0] == "blocks"
            && parts[2] == "moe"
            && parts[3] == "experts"
            && (parts[5] == "c_fc" || parts[5] == "c_proj")
            && parts.last() == Some(&"weight")
        {
            if let (Ok(li), Ok(ei)) = (parts[1].parse::<usize>(), parts[4].parse::<usize>()) {
                if li < num_layers && ei < num_experts {
                    // Gradient norm contribution
                    if let Some(g) = grads.get(var.as_tensor()) {
                        if let Ok(s) = g.sqr().and_then(|t| t.sum_all()).and_then(|t| t.to_scalar::<f32>()) {
                            if s.is_finite() { layer_expert_grad_sq[li][ei] += s; }
                        }
                    }
                    // Weight norm contribution (for wd equivalent)
                    if let Ok(s) = var.as_tensor().sqr().and_then(|t| t.sum_all()).and_then(|t| t.to_scalar::<f32>()) {
                        if s.is_finite() { layer_expert_weight_sq[li][ei] += s; }
                    }
                }
            }
        }
    }
    (0..num_layers).map(|l| {
        let grad_norms: Vec<f32>   = layer_expert_grad_sq[l].iter().map(|&s| s.sqrt()).collect();
        let weight_norms: Vec<f32> = layer_expert_weight_sq[l].iter().map(|&s| s.sqrt()).collect();
        let mean_grad   = grad_norms.iter().sum::<f32>()   / num_experts as f32;
        let mean_weight = weight_norms.iter().sum::<f32>() / num_experts as f32;
        let wd_equiv    = mean_weight * wd as f32;  // expected weight pull per step ~ wd × ||w||
        let ratio       = if wd_equiv > 0.0 { mean_grad / wd_equiv } else { 0.0 };
        (mean_grad, wd_equiv, ratio)
    }).collect()
}

/// Scale up gradients for layers whose norm is below THRESHOLD.
/// Self-terminating: once a layer's norm rises past the threshold the
/// amplification stops automatically, so this doesn't need a manual off switch.
fn amplify_early_layers(
    varmap: &VarMap,
    grads: &mut candle_core::backprop::GradStore,
    layer_norms: &[f32],
    scale: f64,
) {
    // Layers whose per-layer grad norm falls below this are considered crystallized.
    // 0.005 catches L0–L5 at current training state (was 0.001, too narrow).
    const THRESHOLD: f32 = 0.005;

    let all_vars = varmap.data().lock().unwrap();
    for (name, var) in all_vars.iter() {
        let li = name.strip_prefix("blocks.")
            .and_then(|s| s.split('.').next())
            .and_then(|s| s.parse::<usize>().ok());
        if let Some(i) = li {
            if i < layer_norms.len() && layer_norms[i] < THRESHOLD {
                if let Some(g) = grads.get(var.as_tensor()).cloned() {
                    if let Ok(scaled) = &g * scale {
                        grads.insert(var.as_tensor(), scaled);
                    }
                }
            }
        }
    }
}

/// Direct LR boost for cold layers, applied after Adam's update step.
/// Normalises each gradient tensor by its own L2 norm before scaling, so every
/// cold-layer parameter receives a step of magnitude `boost_lr` regardless of
/// raw gradient scale — equivalent to per-tensor signed SGD with lr=boost_lr.
fn apply_layer_lr_boost(
    varmap: &VarMap,
    grads: &candle_core::backprop::GradStore,
    layer_norms: &[f32],
    boost_lr: f64,
) {
    const THRESHOLD: f32 = 0.005;
    let all_vars = varmap.data().lock().unwrap();
    for (name, var) in all_vars.iter() {
        let li = name.strip_prefix("blocks.")
            .and_then(|s| s.split('.').next())
            .and_then(|s| s.parse::<usize>().ok());
        if let Some(i) = li {
            if i < layer_norms.len() && layer_norms[i] < THRESHOLD {
                if let Some(g) = grads.get(var.as_tensor()) {
                    let _ = (|| -> candle_core::Result<()> {
                        let norm_sq = g.sqr()?.sum_all()?.to_scalar::<f32>()?;
                        let g_norm  = (norm_sq as f64 + 1e-12).sqrt();
                        let delta   = (g * (-boost_lr / g_norm))?;
                        var.set(&var.as_tensor().add(&delta)?)
                    })();
                }
            }
        }
    }
}

/// Emit a TELE line to the training log once per epoch.
/// The dashboard parses this to drive the live neural viz panels.
///
/// Format: TELE L=<layers> S=<sparsity per layer, comma> E=<expert activity, comma>
///
/// Sparsity: fraction of weights with |w| < 0.1 (i.e. ternary zero) per layer.
/// Expert activity: mean absolute weight in each expert's MLP, normalised 0–1.
fn emit_telemetry(varmap: &VarMap, config: &TransformerConfig, log_path: &str) {
    let num_layers = config.num_layers;
    let num_experts = config.num_experts;
    let all_vars = match varmap.data().lock() {
        Ok(v) => v,
        Err(_) => return,
    };

    let mut layer_zeros = vec![0u64; num_layers];
    let mut layer_total = vec![0u64; num_layers];
    let mut expert_sum  = vec![0.0f32; num_experts];
    let mut expert_cnt  = vec![0u64; num_experts];

    for (name, var) in all_vars.iter() {
        if !name.contains("weight") { continue; }

        let data: Vec<f32> = match var.as_tensor()
            .flatten_all()
            .and_then(|t| t.to_vec1::<f32>())
        {
            Ok(d) => d,
            Err(_) => continue,
        };

        // Per-layer sparsity: extract layer index from "blocks.N."
        let layer_idx = name.strip_prefix("blocks.")
            .and_then(|s| s.split('.').next())
            .and_then(|s| s.parse::<usize>().ok());

        if let Some(li) = layer_idx {
            if li < num_layers {
                let thr = config.layer_threshold(li);
                layer_zeros[li] += data.iter().filter(|&&w| w.abs() < thr).count() as u64;
                layer_total[li] += data.len() as u64;
            }
        }

        // Per-expert activity: extract expert index from "experts.N."
        if name.contains("experts.") {
            let ei = name.split("experts.")
                .nth(1)
                .and_then(|s| s.split('.').next())
                .and_then(|s| s.parse::<usize>().ok());
            if let Some(e) = ei {
                if e < num_experts {
                    expert_sum[e] += data.iter().map(|w| w.abs()).sum::<f32>();
                    expert_cnt[e] += data.len() as u64;
                }
            }
        }
    }

    let sparsity: Vec<String> = (0..num_layers).map(|i| {
        if layer_total[i] > 0 {
            format!("{:.3}", layer_zeros[i] as f32 / layer_total[i] as f32)
        } else { "0.000".to_string() }
    }).collect();

    // Normalise expert activity to 0–1 relative to the most active expert.
    let acts: Vec<f32> = (0..num_experts).map(|e| {
        if expert_cnt[e] > 0 { expert_sum[e] / expert_cnt[e] as f32 } else { 0.0 }
    }).collect();
    let max_act = acts.iter().cloned().fold(0.0f32, f32::max).max(1e-9);
    let expert_act: Vec<String> = acts.iter()
        .map(|&a| format!("{:.3}", a / max_act))
        .collect();

    let line = format!("TELE L={} S={} E={}",
        num_layers,
        sparsity.join(","),
        expert_act.join(","),
    );

    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = writeln!(f, "{}", line);
    }
}

/// Generate a unit vector in R^dim, deterministic from seed, via Box-Muller over LCG.
fn deterministic_unit_vec(dim: usize, seed: usize) -> Vec<f64> {
    let mut state = (seed as u64)
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    let mut v = Vec::with_capacity(dim);
    let mut i = 0usize;
    while i < dim {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let u1 = (state >> 11) as f64 / (1u64 << 53) as f64;
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let u2 = (state >> 11) as f64 / (1u64 << 53) as f64;
        let r = (-2.0 * u1.max(1e-300).ln()).sqrt();
        let theta = 2.0 * std::f64::consts::PI * u2;
        v.push(r * theta.cos());
        i += 1;
        if i < dim {
            v.push(r * theta.sin());
            i += 1;
        }
    }
    let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-12);
    v.into_iter().map(|x| x / norm).collect()
}

/// Seed F32 shadow expert weights with per-expert Householder rotations + deterministic scaling.
///
/// For each expert (layer, e): generate unit vector v from seed=layer*100+e, apply
/// Householder reflection H=I-2vv^T in the hidden_size space to both c_fc and c_proj
/// weight matrices. Additionally scale by exp(amplitude*(2e/(E-1)-1)) so each expert
/// has a distinct mean-abs value, guaranteeing DIVF32 > 1e-3 after seeding.
/// Aborts if post-seed DIVF32 < 1e-3 for any layer.
fn seed_experts_orthogonal(
    varmap: &VarMap,
    num_layers: usize,
    num_experts: usize,
    hidden_size: usize,
    device: &Device,
) -> Result<()> {
    const SCALE_AMPLITUDE: f64 = 0.8; // exp range [0.449, 2.225] across 12 experts

    // Snapshot expert vars while holding the lock minimally.
    let expert_vars: Vec<_> = {
        let data = varmap.data().lock().unwrap();
        data.iter()
            .filter(|(name, _)| name.contains(".moe.experts."))
            .map(|(name, var)| (name.clone(), var.clone()))
            .collect()
    };

    let mut tensors_modified = 0usize;

    for layer in 0..num_layers {
        for expert in 0..num_experts {
            let seed = layer * 100 + expert;
            let v = deterministic_unit_vec(hidden_size, seed); // unit vec in R^hidden_size

            // Per-expert deterministic scale: exp(A*(2e/(E-1)-1)), E experts
            let t = if num_experts > 1 {
                2.0 * expert as f64 / (num_experts - 1) as f64 - 1.0
            } else { 0.0 };
            let scale = (SCALE_AMPLITUDE * t).exp();

            let prefix = format!("blocks.{}.moe.experts.{}.", layer, expert);

            for (name, var) in &expert_vars {
                if !name.starts_with(&prefix) { continue; }

                let tensor = var.as_tensor();
                let shape = tensor.shape().dims().to_vec();
                let mut flat = tensor.flatten_all()?.to_vec1::<f32>()?;

                if name.ends_with("c_fc.weight") && shape.len() == 2 {
                    // Shape [hidden*4, hidden] — rotate in "in" (column) space: W_new = W @ H
                    // H = I - 2v*v^T → W @ H = W - 2*(W@v)*v^T   [O(out*hidden) op]
                    let (out_dim, in_dim) = (shape[0], shape[1]);
                    debug_assert_eq!(in_dim, hidden_size);
                    // compute W@v for each output row
                    let mut wv = vec![0.0f64; out_dim];
                    for i in 0..out_dim {
                        for j in 0..in_dim {
                            wv[i] += flat[i * in_dim + j] as f64 * v[j];
                        }
                    }
                    // apply + scale
                    for i in 0..out_dim {
                        for j in 0..in_dim {
                            flat[i * in_dim + j] = ((flat[i * in_dim + j] as f64
                                - 2.0 * wv[i] * v[j]) * scale) as f32;
                        }
                    }
                } else if name.ends_with("c_proj.weight") && shape.len() == 2 {
                    // Shape [hidden, hidden*4] — rotate in "out" (row) space: W_new = H @ W
                    // H @ W = W - 2*v*(v^T@W)   [O(hidden*in) op]
                    let (out_dim, in_dim) = (shape[0], shape[1]);
                    debug_assert_eq!(out_dim, hidden_size);
                    // compute v^T @ W for each input column
                    let mut vtw = vec![0.0f64; in_dim];
                    for j in 0..in_dim {
                        for i in 0..out_dim {
                            vtw[j] += v[i] * flat[i * in_dim + j] as f64;
                        }
                    }
                    // apply + scale
                    for i in 0..out_dim {
                        for j in 0..in_dim {
                            flat[i * in_dim + j] = ((flat[i * in_dim + j] as f64
                                - 2.0 * v[i] * vtw[j]) * scale) as f32;
                        }
                    }
                } else {
                    // 1D tensors (bias, seed_bias): just scale for mean-abs spread
                    for x in flat.iter_mut() { *x = (*x as f64 * scale) as f32; }
                }

                let new_tensor = Tensor::from_vec(flat, shape.as_slice(), device)?;
                var.set(&new_tensor)?;
                tensors_modified += 1;
            }
        }
    }

    println!("[seed] applied orthogonal F32 perturbation to {} expert tensors, target DIVF32 variance: 0.05-0.10",
        tensors_modified);

    // Post-seed verification: DIVF32 must exceed 1e-3 at every layer.
    let mut abort = false;
    for layer in 0..num_layers {
        let signals: Vec<f32> = (0..num_experts).filter_map(|expert| {
            let fc_key   = format!("blocks.{}.moe.experts.{}.c_fc.weight",   layer, expert);
            let proj_key = format!("blocks.{}.moe.experts.{}.c_proj.weight", layer, expert);
            // look up in the already-cloned vars list
            let fc_abs = expert_vars.iter().find(|(n, _)| *n == fc_key)
                .and_then(|(_, v)| v.as_tensor().abs().ok()?.mean_all().ok()?.to_scalar::<f32>().ok())?;
            let proj_abs = expert_vars.iter().find(|(n, _)| *n == proj_key)
                .and_then(|(_, v)| v.as_tensor().abs().ok()?.mean_all().ok()?.to_scalar::<f32>().ok())?;
            Some((fc_abs + proj_abs) / 2.0)
        }).collect();

        if signals.len() == num_experts {
            let mean = signals.iter().sum::<f32>() / signals.len() as f32;
            let var  = signals.iter().map(|&s| (s - mean).powi(2)).sum::<f32>() / signals.len() as f32;
            println!("[seed] L{} DIVF32 post-seed: {:.3e}", layer, var);
            if var < 1e-3 {
                eprintln!("[seed] ABORT: L{} DIVF32={:.3e} < 1e-3 threshold — seeding insufficient", layer, var);
                abort = true;
            }
        }
    }
    if abort {
        return Err(candle_core::Error::Msg(
            "[seed] ABORTING: post-seed DIVF32 verification failed. Check expert weight scaling.".into()
        ));
    }

    Ok(())
}

// Net2Net safe-copy layer surgery — whitepaper §11.2 (EvolutionManager)
fn perform_surgery(config_path: &str, checkpoint_path: &str, best_path: &str, device: &Device, root: &str) -> Result<()> {
    println!("[{}] --- INITIATING NEURAL SURGERY: Net2Net Safe Copy ---", timestamp());

    let config_str = fs::read_to_string(config_path).expect("Unable to read config.json");
    let mut config_json: Value = serde_json::from_str(&config_str).expect("Invalid JSON in config file.");
    let old_layers = config_json["num_layers"].as_u64().unwrap() as usize;
    let new_layers = old_layers + 1;
    config_json["num_layers"] = json!(new_layers);
    fs::write(config_path, serde_json::to_string_pretty(&config_json).unwrap())?;
    println!("[{}] Evolution: Architecture expanded to {} layers.", timestamp(), new_layers);

    // Surgery reads from the BEST checkpoint, not the latest — prevents expanding from bad weights.
    let source = if std::path::Path::new(best_path).exists() { best_path } else { checkpoint_path };
    let tensors = candle_core::safetensors::load(source, device)?;
    let mut new_tensors = HashMap::new();

    let source_layer = old_layers - 1;
    let target_layer = old_layers;

    for (name, tensor) in tensors.iter() {
        new_tensors.insert(name.clone(), tensor.clone());
        let prefix = format!("blocks.{}.", source_layer);
        if name.starts_with(&prefix) {
            let new_name = name.replace(&prefix, &format!("blocks.{}.", target_layer));
            new_tensors.insert(new_name, tensor.clone());
        }
    }

    // Break symmetry with Mandelbrot-parameterised perturbation.
    //
    // Each surgery maps the new layer to a unique "latitude" in Mandelbrot parameter
    // space (c_im derived from layer index via golden-ratio sequence). This makes the
    // perturbation pattern self-similar across layers but never identical — the same
    // fractal self-similarity property that motivated the module (whitepaper §11.3).
    //
    // Weights mapping to the Mandelbrot interior (stable basins) receive near-zero
    // perturbation — learned features are preserved. Weights near the boundary receive
    // the largest shake — uncertain, plastic weights are pushed to explore.
    // No external RNG: perturbation is fully deterministic from (value, index, depth).
    let mand = MandelbrotSurgery::new();
    let target_prefix = format!("blocks.{}.", target_layer);
    let perturb_keys: Vec<String> = new_tensors.keys()
        .filter(|k| k.starts_with(&target_prefix))
        .cloned()
        .collect();
    let tensor_count = perturb_keys.len();
    for key in perturb_keys {
        if let Some(t) = new_tensors.get(&key) {
            let shape = t.shape().clone();
            let flat  = t.flatten_all()?.to_vec1::<f32>()?;
            let perturbed_flat = mand.perturb(&flat, target_layer);
            let perturbed = Tensor::from_vec(perturbed_flat, &shape, device)?;
            new_tensors.insert(key, perturbed);
        }
    }
    println!("[{}] Symmetry break: Mandelbrot perturbation applied to {} tensors in layer {} (c_im={:.4}).",
        timestamp(), tensor_count, target_layer,
        moe_llm_core::mandelbrot::layer_c_im(target_layer));

    candle_core::safetensors::save(&new_tensors, checkpoint_path)?;
    // Archive the pre-surgery best checkpoint with a versioned name instead of deleting it.
    // Deleting was the original bug: if the new layer can't learn, the good pre-surgery weights
    // are lost forever. The archive preserves rollback capability.
    let archive_path = format!("{}/models/albert_v3.0.best.{}L.safetensors", root, old_layers);
    if std::path::Path::new(best_path).exists() {
        let _ = fs::rename(best_path, &archive_path);
        println!("[{}] Pre-surgery best archived to {}.", timestamp(), archive_path);
    }
    println!("[{}] Surgery Complete: Layer {} cloned from Layer {}.", timestamp(), target_layer, source_layer);
    Ok(())
}

fn perform_resurrection(checkpoint_path: &str, jobs: &[moe_llm_core::mycelium::ResurrectionJob], device: &Device) -> Result<()> {
    if jobs.is_empty() { return Ok(()); }
    let tensors = candle_core::safetensors::load(checkpoint_path, device)?;
    let mut new_tensors: HashMap<String, Tensor> = tensors.into_iter().collect();
    for job in jobs {
        let seed_prefix = format!("blocks.{}.moe.experts.{}.", job.layer, job.seed_expert);
        let dead_prefix  = format!("blocks.{}.moe.experts.{}.", job.layer, job.dead_expert);
        let seed_keys: Vec<String> = new_tensors.keys()
            .filter(|k| k.starts_with(&seed_prefix))
            .cloned()
            .collect();
        for seed_key in seed_keys {
            let dead_key = seed_key.replace(&seed_prefix, &dead_prefix);
            if let Some(t) = new_tensors.get(&seed_key) {
                let noise = Tensor::randn(0.0f32, job.noise_sigma, t.shape(), device)?;
                new_tensors.insert(dead_key, (t + noise)?);
            }
        }
        println!("[{}] MYCELIUM: Resurrected L{}E{} from L{}E{} (σ={:.3})",
            timestamp(), job.layer, job.dead_expert, job.layer, job.seed_expert, job.noise_sigma);
    }
    candle_core::safetensors::save(&new_tensors, checkpoint_path)?;
    Ok(())
}

fn train_cycle(
    tokens: &[u32],
    tokenizer: &BpeTokenizer,
    device: &Device,
    evolution_manager: &mut EvolutionManager,
    mycelium: &mut MyceliumModule,
    wald: &mut WaldModule,
    global_step: &mut usize,
    flags: &TrainFlags,
) -> Result<bool> {
    let r = &flags.root;
    let checkpoint_path = format!("{r}/models/albert_v3.0.safetensors");
    let best_path       = format!("{r}/models/albert_v3.0.best.safetensors");
    let config_path     = format!("{r}/models/albert_v3.0.config.json");
    let meta_path       = format!("{r}/models/albert_v3.0.meta");
    let best_meta_path  = format!("{r}/models/albert_v3.0.best_loss");
    let log_path        = format!("{r}/dashboard/training.log");
    // Borrow as &str for the many call sites that take &str
    let checkpoint_path = checkpoint_path.as_str();
    let best_path       = best_path.as_str();
    let config_path     = config_path.as_str();
    let meta_path       = meta_path.as_str();
    let best_meta_path  = best_meta_path.as_str();
    let log_path        = log_path.as_str();

    let config_str = fs::read_to_string(config_path).expect("Unable to read config.json");
    let config_json: Value = serde_json::from_str(&config_str).expect("Invalid JSON in config file.");

    let mut config = TransformerConfig::default();
    config.vocab_size  = tokenizer.vocab_size();
    config.hidden_size = config_json["hidden_size"].as_u64().unwrap() as usize;
    config.num_layers  = config_json["num_layers"].as_u64().unwrap() as usize;
    config.num_heads   = config_json["num_heads"].as_u64().unwrap() as usize;
    config.max_seq_len = config_json["max_seq_len"].as_u64().unwrap() as usize;
    config.num_experts = config_json["num_experts"].as_u64().unwrap() as usize;

    println!("[{}] Arch: {}L · {}H · {}E · {}CTX | Vocab: {}",
        timestamp(), config.num_layers, config.hidden_size,
        config.num_experts, config.max_seq_len, config.vocab_size);

    let varmap = VarMap::new();
    let vb     = VarBuilder::from_varmap(&varmap, DType::F32, device);
    let model  = Transformer::new(&config, vb)?;

    // Load latest checkpoint if present.
    if std::path::Path::new(checkpoint_path).exists() {
        let loaded = load_checkpoint(&varmap, checkpoint_path, device)?;
        println!("[{}] Loaded {} tensors from checkpoint.", timestamp(), loaded);
    }

    // Reset gate weights to kaiming-uniform to break the routing symmetry baked in
    // by 250+ epochs of near-uniform routing. Expert weights are preserved — they hold
    // 250 epochs of valuable signal. Only the gate needs fresh signal to differentiate.
    {
        let gate_vars: Vec<_> = {
            let all_vars = varmap.data().lock().unwrap();
            all_vars.iter()
                .filter(|(name, _)| name.contains(".moe.gate.weight"))
                .map(|(name, var)| (name.clone(), var.clone()))
                .collect()
        };
        for (name, var) in &gate_vars {
            let shape = var.shape().clone();
            let fan_in = shape.dims()[1] as f64; // gate is [num_experts × hidden], fan_in = hidden
            let std = (2.0_f64 / fan_in).sqrt() as f32;
            let fresh = Tensor::randn(0.0f32, std, shape.dims(), device)?;
            var.set(&fresh)?;
            println!("[{}] Gate reset: {} → kaiming-uniform std={:.4}", timestamp(), name, std);
        }
        if !gate_vars.is_empty() {
            println!("[{}] Gate weights reset to kaiming-uniform — routing symmetry broken.", timestamp());
        }
    }

    // Expert weight perturbation: independent Gaussian noise per expert tensor.
    // σ=0.02 was too small — ternary weights cluster near their quantized values so
    // 0.02 flips almost no weights between bins, leaving all experts weight-identical.
    // σ=0.15 flips ~20-30% of near-threshold weights per expert, giving each a distinct
    // enough representation that CE gradient can route between them meaningfully.
    // At current loss ≈ ln(vocab) the experts hold no useful signal anyway — the cost
    // of disruption is near-zero vs the benefit of breaking symmetry.
    {
        let expert_vars: Vec<_> = {
            let all_vars = varmap.data().lock().unwrap();
            all_vars.iter()
                .filter(|(name, _)| name.contains(".moe.experts."))
                .map(|(name, var)| (name.clone(), var.clone()))
                .collect()
        };
        let sigma = 0.15f32;
        for (_name, var) in &expert_vars {
            let noise = Tensor::randn(0.0f32, sigma, var.shape().dims(), device)?;
            let perturbed = (var.as_tensor() + noise)?;
            var.set(&perturbed)?;
        }
        if !expert_vars.is_empty() {
            println!("[{}] Expert symmetry break: σ={} noise applied to {} expert tensors.",
                timestamp(), sigma, expert_vars.len());
        }
    }

    // F32 shadow weight seeding — breaks Nash deadlock by placing each expert at a
    // distinct position in weight space (Householder rotation + scaling). Runs once
    // per binary invocation when --seed-experts is passed, before the epoch loop.
    if flags.seed_experts {
        seed_experts_orthogonal(&varmap, config.num_layers, config.num_experts, config.hidden_size, device)?;
    }

    // Gate diversity bias: fixed logit spread [−scale, +scale] across experts.
    // 0.0 (default) = disabled. Only active when --gate-diversity=F is passed.
    if flags.gate_diversity != 0.0 {
        set_gate_diversity_scale(flags.gate_diversity);
        println!("[{}] [gate-diversity] scale={:.3} — fixed asymmetric logit bias active.",
            timestamp(), flags.gate_diversity);
    }

    // Track best epoch-average loss across the entire training run.
    let mut best_epoch_loss: f32 = fs::read_to_string(best_meta_path)
        .ok()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .unwrap_or(f32::MAX);
    // Previous epoch average for delta computation in EPOCH_SUMMARY.
    let mut prev_avg_loss: f32 = best_epoch_loss;

    let mut total_epochs = if let Ok(c) = fs::read_to_string(meta_path) {
        c.trim().parse::<u32>().unwrap_or(0)
    } else { 0 };

    // Cosine LR: starts high, decays to near-zero over lr_cycle_steps global steps
    let base_lr        = 3e-4_f64;   // reset to known-good; BATCH_SIZE=13 (eff. 52) does not justify 5e-4
    let min_lr         = 2e-5_f64;
    let lr_cycle_steps = 500_usize;

    let mut opt = candle_nn::AdamW::new_lr(varmap.all_vars(), base_lr)?;
    let mut collapse_streak: u32 = 0;
    // WALD-driven early-layer amplification scale — updated each epoch.
    // Severity 0.0 → scale 4×; severity 1.0 → scale 48× (linear interpolation).
    let mut wald_amplify_scale: f64 = 8.0;

    let seq_len     = config.max_seq_len;
    let num_batches = 300_usize;

    // Per-layer grad norm EMA for burst detection. Initialized high (1.0) so early steps
    // don't false-trigger before EMA converges to the per-layer true baseline (~50 steps).
    let mut grad_norm_ema: Vec<f32> = vec![0.0_f32; config.num_layers]; // 0 = uninitialized; bootstrap to first observation
    // Safety circuit: how many TTL freeze events fired per layer this epoch.
    let mut epoch_burst_count: Vec<usize> = vec![0; config.num_layers];

    // Expert dominance tripwire: consecutive epoch count where any expert > 70% routing.
    let mut expert_dominance_streak: Vec<u8> = vec![0u8; config.num_experts];
    // LB ramp: epochs since LB was enabled in this binary run (used for warmup ramp).
    let mut lb_ramp_epoch: usize = 0;

    // Write arch metadata once per train_cycle so the dashboard can display it.
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = writeln!(f, "ARCH {}L {}H {}E {}CTX {}V",
            config.num_layers, config.hidden_size, config.num_experts,
            config.max_seq_len, config.vocab_size);
    }

    // Self-describing config banner — printed once at binary start so every log file
    // carries its own hyperparameters for SPRIND reviewers and future debugging.
    {
        let ema_window = (1.0 / GRAD_NORM_EMA_ALPHA) as usize;
        let initial_div_w = flags.div_weight_override
            .unwrap_or_else(|| div_loss_weight((total_epochs + 1) as usize));
        let lb_banner = if flags.lb_weight == 0.0 {
            "[lb] disabled — LB gradient will NOT flow this run".to_string()
        } else {
            format!("[lb] active weight={:.3e} ramp_epochs={}", flags.lb_weight, flags.lb_ramp_epochs)
        };
        let div_banner = if let Some(ow) = flags.div_weight_override {
            format!("[divloss] enabled weight={:.2e} (OVERRIDE — schedule bypassed)", ow)
        } else {
            format!("[divloss] enabled weight={:.2e} (decay {:.2e}->{:.2e} epochs {}-{})",
                initial_div_w, DIV_LOSS_WEIGHT_START, DIV_LOSS_WEIGHT_END,
                DIV_DECAY_START_EPOCH, DIV_DECAY_END_EPOCH - 1)
        };
        let banner = format!(
            "[ttlfreeze] enabled\n\
             [ttlfreeze]   ema_alpha={} (effective window ~{} steps)\n\
             [ttlfreeze]   burst_threshold={}x baseline\n\
             [ttlfreeze]   freeze_steps={}\n\
             [ttlfreeze]   max_bursts_per_epoch_per_layer={}\n\
             [ttlfreeze]   ema_init=bootstrap_to_first_observation\n\
             {}\n\
             {}",
            GRAD_NORM_EMA_ALPHA, ema_window,
            BURST_RATIO_THRESHOLD, TTL_FREEZE_GRAD_STEPS, MAX_BURSTS_PER_EPOCH,
            lb_banner, div_banner,
        );
        println!("{}", banner);
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = writeln!(f, "{}", banner);
        }
    }

    loop {
        let mut total_loss    = 0.0_f32;
        let mut counted_batches = 0u32; // only count non-skipped batches in avg
        total_epochs += 1;
        // Reset per-epoch TTL freeze safety counters at epoch boundary.
        epoch_burst_count.iter_mut().for_each(|c| *c = 0);
        let mut clipped_steps = 0u32;
        let mut skipped_steps = 0u32;
        // Mycelium epoch telemetry accumulators
        let mut last_tlight_states: Vec<String> = vec![String::new(); config.num_layers];
        let mut epoch_layer_norm_acc: Vec<f32>  = vec![0.0; config.num_layers];
        let mut epoch_layer_norm_count: usize   = 0;
        // Per-epoch routing accumulator for expert dominance tripwire.
        let mut epoch_route_acc: Vec<f32>  = vec![0.0; config.num_experts];
        let mut epoch_route_count: usize   = 0;
        // Effective LB weight this epoch, applying ramp when re-enabling from disabled.
        if flags.lb_weight > 0.0 { lb_ramp_epoch += 1; }
        let effective_lb = if flags.lb_weight == 0.0 {
            0.0_f64
        } else if lb_ramp_epoch <= flags.lb_ramp_epochs {
            flags.lb_weight * (0.1 + 0.9 * (lb_ramp_epoch - 1) as f64
                / flags.lb_ramp_epochs.max(1) as f64)
        } else {
            flags.lb_weight
        };
        // Cascade evidence: rolling 10-step L0-L3 history, forward checks, and resolved results.
        let mut l0l3_history: std::collections::VecDeque<[f32; 4]> = std::collections::VecDeque::with_capacity(11);
        // (fire_step, check_step=fire+30, layer_idx, pre_L0L3_avg)
        let mut pending_cascade: Vec<(usize, usize, usize, [f32; 4])> = Vec::new();
        // (layer_idx, fire_step, pre_L0L3_avg, post_L0L3)
        let mut cascade_results: Vec<(usize, usize, [f32; 4], [f32; 4])> = Vec::new();

        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .ok();

        let epoch_start = Instant::now();
        let mut accum_loss: Option<Tensor> = None;
        let mut accum_exploded = false;

        for batch_idx in 0..num_batches {
            let batch_start = Instant::now();

            let lr = cosine_lr(base_lr, min_lr, *global_step % lr_cycle_steps, lr_cycle_steps);
            opt.set_learning_rate(lr);

            let mut input_rows: Vec<Tensor> = Vec::with_capacity(BATCH_SIZE);
            let mut target_rows: Vec<Tensor> = Vec::with_capacity(BATCH_SIZE);
            for _ in 0..BATCH_SIZE {
                let start = rand::random::<usize>() % (tokens.len() - seq_len - 1);
                input_rows.push(Tensor::new(&tokens[start..start + seq_len], device)?
                    .to_dtype(DType::U32)?);
                target_rows.push(Tensor::new(&tokens[start + 1..start + seq_len + 1], device)?
                    .to_dtype(DType::U32)?);
            }
            let input_tensor = Tensor::stack(&input_rows, 0)?;   // [BATCH_SIZE, seq_len]
            let target_tensor = Tensor::stack(&target_rows, 0)?; // [BATCH_SIZE, seq_len]

            // Gate div computation to optimizer-step batches only — avoids running all
            // 12 experts on every batch (144 extra forward passes × GRAD_ACCUM_STEPS = 4×).
            let is_step_batch = (batch_idx + 1) % GRAD_ACCUM_STEPS == 0
                || batch_idx + 1 == num_batches;
            set_div_enabled(is_step_batch);

            clear_entropy_capture();
            clear_lb_capture();
            clear_tlight_capture();
            clear_div_capture();
            let logits      = model.forward(&input_tensor)?;
            let logits      = logits.reshape((BATCH_SIZE * seq_len, config.vocab_size))?;
            let target_flat = target_tensor.flatten_all()?;
            let ce_loss     = loss::cross_entropy(&logits, &target_flat)?;

            // ── Auxiliary losses ──────────────────────────────────────────────
            // L1 is gated on loss < 8.0 — applying to a collapsed model makes recovery harder.
            // LB: controlled by --lb-weight/--lb-disable flags (effective_lb=0 skips gradient).
            let real_loss_preview = ce_loss.to_scalar::<f32>().unwrap_or(f32::MAX);
            let aux_active     = real_loss_preview < 8.0;
            let l1_lambda      = if aux_active { 1e-5_f64  } else { 0.0_f64 };
            let entropy_lambda = 0.0_f64;

            // Drain both thread-locals regardless — keeps accumulators clean next batch.
            let entropy_term   = take_entropy_capture();
            let entropy_scalar = entropy_term.as_ref()
                .and_then(|t| t.to_scalar::<f32>().ok())
                .unwrap_or(0.0);
            let lb_term        = take_lb_capture();
            let lb_scalar      = lb_term.as_ref()
                .and_then(|t| t.to_scalar::<f32>().ok())
                .unwrap_or(0.0);

            let mut batch_loss = if aux_active {
                let l1_penalty = {
                    let vars = varmap.data().lock().unwrap();
                    let mut terms: Vec<Tensor> = Vec::new();
                    for (name, var) in vars.iter() {
                        if name.ends_with("weight") {
                            terms.push(var.abs()?.mean_all()?);
                        }
                    }
                    drop(vars);
                    if terms.is_empty() {
                        Tensor::zeros((), DType::F32, &device)?
                    } else {
                        Tensor::stack(&terms, 0)?.sum_all()?
                    }
                };
                let mut loss = (&ce_loss + (l1_penalty * l1_lambda)?)?;
                if let Some(et) = entropy_term {
                    let entr_now = (-entropy_scalar / config.num_layers as f32) as f64;
                    if entropy_lambda > 0.0 && entr_now < 2.2 {
                        loss = (&loss + (et * entropy_lambda)?)?;
                    }
                }
                loss
            } else {
                ce_loss.clone()
            };

            // LB loss: gated on effective_lb > 0. When LB is disabled (--lb-disable),
            // lb_term is drained but not added to batch_loss — no gradient flows.
            if effective_lb > 0.0 {
                if let Some(lb) = lb_term {
                    batch_loss = (&batch_loss + (lb * effective_lb)?)?;
                }
            }
            // When LB disabled, lb_term is already drained above via take_lb_capture().

            // Divergence loss: always active while weight > 0. Continuous output-space force.
            // Minimizes neg-variance of L2-normalized expert outputs → maximizes diversity.
            let cur_div_weight = flags.div_weight_override
                .unwrap_or_else(|| div_loss_weight(total_epochs as usize));
            let div_log_vals     = take_div_log_capture();      // per-layer ternary output variance
            let div_f32_log_vals = take_div_f32_log_capture(); // per-layer F32 shadow weight variance
            if cur_div_weight > 0.0 {
                if let Some(div_term) = take_div_capture() {
                    // div_term is sum of neg-variance across layers; normalize by num_layers.
                    let div_scaled = (div_term * (cur_div_weight / config.num_layers as f64))?;
                    batch_loss = (&batch_loss + div_scaled)?;
                }
            } else {
                take_div_capture(); // drain even when weight=0 to keep accumulators clean
            }

            let real_loss = ce_loss.to_scalar::<f32>()?;

            // Flag explosion — will flush and skip the whole accumulation window.
            if real_loss.is_nan() || real_loss.is_infinite() || real_loss > LOSS_EXPLOSION_THRESHOLD {
                accum_exploded = true;
                let crit = format!("[CRITICAL] loss={:.4} > {:.1} at step={} — explosion, skipping accumulation window",
                    real_loss, LOSS_EXPLOSION_THRESHOLD, *global_step);
                println!("[{}] {}", timestamp(), crit);
                if let Some(ref mut f) = log_file {
                    let _ = writeln!(f, "{}", crit);
                }
            }

            // Accumulate scaled loss (÷N so the effective gradient magnitude is unchanged).
            let scaled = (batch_loss * (1.0 / GRAD_ACCUM_STEPS as f64))?;
            accum_loss = Some(match accum_loss.take() {
                None    => scaled,
                Some(a) => (a + scaled)?,
            });

            total_loss      += real_loss;
            counted_batches += 1;
            wald.record_batch(real_loss);

            // Default: empty norms for non-step batches (used in GRAD log below).
            let mut layer_norms = vec![0.0_f32; config.num_layers];
            let mut norm = 0.0_f32;
            let mut div_grad_vals: Vec<f32> = Vec::new();

            if is_step_batch {
                if accum_exploded {
                    skipped_steps += 1;
                    println!("[{}] [SKIP] Accum window ending batch {} — explosion, preserving weights.",
                        timestamp(), batch_idx);
                    accum_loss     = None;
                    accum_exploded = false;
                    *global_step  += 1;
                    continue;
                }

                // ── Gradient step ─────────────────────────────────────────────
                // backward() + clip + step + cold-layer LR boost, once per GRAD_ACCUM_STEPS.
                if let Some(combined) = accum_loss.take() {
                    let mut grads = combined.backward()?;
                    // Capture expert grad variance before opt.step() consumes grads.
                    if is_step_batch && cur_div_weight > 0.0 {
                        div_grad_vals = expert_grad_variance(&varmap, &grads, config.num_layers, config.num_experts);
                        // WD ratio diagnostic: mean expert grad norm vs weight-decay pull.
                        // AdamW default wd=0.01. ratio < 1.0 = wd dominates → DIVF32 cannot grow.
                        let wd_ratios = expert_wd_ratio(&varmap, &grads, config.num_layers, config.num_experts, 0.01);
                        if let Some(ref mut f) = log_file {
                            let wd_str: Vec<String> = wd_ratios.iter()
                                .map(|(g, w, r)| format!("{:.2e}/{:.2e}/{:.2}", g, w, r))
                                .collect();
                            let _ = writeln!(f, "DIVWD step={} grad/wdequiv/ratio={}", *global_step, wd_str.join(","));
                        }
                    }
                    norm = global_grad_norm(&varmap, &grads);
                    layer_norms = per_layer_grad_norm(&varmap, &grads, config.num_layers);
                    for (i, &n) in layer_norms.iter().enumerate() {
                        if i < epoch_layer_norm_acc.len() { epoch_layer_norm_acc[i] += n; }
                    }
                    epoch_layer_norm_count += 1;

                    // Update rolling L0-L3 history and resolve any pending cascade checks.
                    if layer_norms.len() >= 4 {
                        let snap: [f32; 4] = [layer_norms[0], layer_norms[1], layer_norms[2], layer_norms[3]];
                        l0l3_history.push_back(snap);
                        if l0l3_history.len() > 10 { l0l3_history.pop_front(); }
                        let gs = *global_step;
                        pending_cascade.retain(|&(fs, cs, li, pre)| {
                            if gs >= cs {
                                cascade_results.push((li, fs, pre, snap));
                                false
                            } else {
                                true
                            }
                        });
                    }

                    // TTL burst detection — per-layer grad norm vs EMA baseline.
                    // When ratio > BURST_RATIO_THRESHOLD, freeze that layer's TTL logit modifiers
                    // for TTL_FREEZE_GRAD_STEPS optimizer steps so gate can learn without correction.
                    for (i, &n) in layer_norms.iter().enumerate() {
                        if n > 0.0 {
                            if grad_norm_ema[i] == 0.0 {
                                // Bootstrap: first real observation sets the baseline, no burst check.
                                grad_norm_ema[i] = n;
                            } else {
                                let baseline = grad_norm_ema[i];
                                let ratio = n / baseline.max(1e-12);
                                if ratio > BURST_RATIO_THRESHOLD && epoch_burst_count[i] < MAX_BURSTS_PER_EPOCH {
                                    epoch_burst_count[i] += 1;
                                    let freeze_update_steps = TTL_FREEZE_GRAD_STEPS * GRAD_ACCUM_STEPS;
                                    model.freeze_ttl_layer(i, freeze_update_steps);
                                    let freeze_end_step = *global_step + TTL_FREEZE_GRAD_STEPS;
                                    // Schedule cascade evidence capture at burst+30.
                                    let pre_l0l3: [f32; 4] = {
                                        let hn = l0l3_history.len() as f32;
                                        if hn == 0.0 { [0.0; 4] } else {
                                            let mut avg = [0.0f32; 4];
                                            for h in &l0l3_history { for j in 0..4 { avg[j] += h[j] / hn; } }
                                            avg
                                        }
                                    };
                                    pending_cascade.push((*global_step, *global_step + 30, i, pre_l0l3));
                                    println!("[{}] TTLFREEZE layer={} step={} grad={:.2e} base={:.2e} ratio={:.1} lr={:.2e} freeze_end={} fires_this_epoch={}",
                                        timestamp(), i, *global_step, n, baseline, ratio, lr, freeze_end_step, epoch_burst_count[i]);
                                    if let Some(ref mut f) = log_file {
                                        let _ = writeln!(f, "TTLFREEZE step={} layer={} grad={:.6} baseline={:.6} ratio={:.2} lr={:.2e} freeze_end={}",
                                            *global_step, i, n, baseline, ratio, lr, freeze_end_step);
                                    }
                                    if epoch_burst_count[i] >= MAX_BURSTS_PER_EPOCH {
                                        println!("[{}] TTLFREEZE WARN layer={} reached {} fires/epoch — suppressing further freezes this epoch",
                                            timestamp(), i, MAX_BURSTS_PER_EPOCH);
                                        if let Some(ref mut f) = log_file {
                                            let _ = writeln!(f, "TTLFREEZE WARN step={} layer={} max_bursts_reached={}",
                                                *global_step, i, MAX_BURSTS_PER_EPOCH);
                                        }
                                    }
                                } else if ratio > BURST_RATIO_THRESHOLD {
                                    // Cap already hit — log skipped burst for observability.
                                    println!("[{}] TTLFREEZE SKIP layer={} step={} grad={:.2e} base={:.2e} ratio={:.1} cap={}/epoch",
                                        timestamp(), i, *global_step, n, baseline, ratio, MAX_BURSTS_PER_EPOCH);
                                    if let Some(ref mut f) = log_file {
                                        let _ = writeln!(f, "TTLFREEZE SKIP step={} layer={} grad={:.6} baseline={:.6} ratio={:.2}",
                                            *global_step, i, n, baseline, ratio);
                                    }
                                }
                                // Update EMA after burst check so baseline tracks the normal level.
                                grad_norm_ema[i] = GRAD_NORM_EMA_ALPHA * n + (1.0 - GRAD_NORM_EMA_ALPHA) * baseline;
                            }
                        }
                    }

                    if norm > MAX_GRAD_NORM && norm.is_finite() {
                        clipped_steps += 1;
                        let scale = (MAX_GRAD_NORM / norm) as f64;
                        for var in varmap.all_vars() {
                            if let Some(g) = grads.get(&var) {
                                grads.insert(&var, (g * scale)?);
                            }
                        }
                    }
                    opt.step(&grads)?;
                    // Cold-layer LR boost after Adam — bypasses second-moment normalization.
                    // wald_amplify_scale is updated each epoch from WALD severity (up to 47×).
                    apply_layer_lr_boost(&varmap, &grads, &layer_norms, lr * wald_amplify_scale);
                }
                accum_exploded = false;
            }

            let batch_ms   = batch_start.elapsed().as_millis();
            let elapsed_s  = epoch_start.elapsed().as_secs();
            let remaining_s = if batch_idx > 0 {
                elapsed_s * (num_batches as u64 - batch_idx as u64) / batch_idx as u64
            } else { 0 };

            let log_line = format!("Epoch {} (Global {}), Batch {}: loss = {:.4}",
                config.num_layers, total_epochs, batch_idx, real_loss);

            println!("[{}] Epoch {:>2}L (Global {:>4}) | {:>3}/{} | Loss: {:.4} | LR: {:.2e} | {:>3}ms | ETA {:02}:{:02}",
                timestamp(),
                config.num_layers,
                total_epochs,
                batch_idx + 1,
                num_batches,
                real_loss,
                lr,
                batch_ms,
                remaining_s / 60,
                remaining_s % 60,
            );

            if let Some(ref mut f) = log_file {
                let _ = writeln!(f, "{}", log_line);

                // GRAD — per-layer gradient norm, every 10 batches to keep log readable.
                // Dashboard parses "GRAD step=N n=X.XXXX L=n0,n1,n2,..."
                if batch_idx % 10 == 0 || batch_idx == 0 {
                    let ln_str: Vec<String> = layer_norms.iter()
                        .map(|n| format!("{:.2e}", n)).collect();
                    let _ = writeln!(f, "GRAD step={} n={:.4} L={}", *global_step, norm, ln_str.join(","));
                }

                // ROUTE — expert routing weights, emitted every 10 batches to keep log lean.
                // Also at batch 5 for early gate-diversity baseline check.
                // Dashboard parses "ROUTE step=N E=w0,w1,...,w11"
                if batch_idx % 10 == 0 || batch_idx == 5 {
                    let routing = take_routing_capture(config.num_experts);
                    // Accumulate for epoch-level expert dominance tripwire.
                    for (e, &w) in routing.iter().enumerate() {
                        if e < epoch_route_acc.len() { epoch_route_acc[e] += w; }
                    }
                    epoch_route_count += 1;
                    let route_str: Vec<String> = routing.iter()
                        .map(|&w| format!("{:.3}", w)).collect();
                    let _ = writeln!(f, "ROUTE step={} E={}", *global_step, route_str.join(","));
                    // ENTR — routing entropy per MoE layer (nats). Max = ln(12) ≈ 2.485.
                    let entr_per_layer = if config.num_layers > 0 {
                        -entropy_scalar / config.num_layers as f32
                    } else { 0.0 };
                    let _ = writeln!(f, "ENTR step={} avg={:.4}", *global_step, entr_per_layer);
                    // LB — load-balancing loss value. Should decrease as routing diversifies.
                    let _ = writeln!(f, "LB step={} val={:.4}", *global_step, lb_scalar);
                    // TLIGHT — ternary traffic light state per layer.
                    // G=green (underloaded, boosted), O=orange (on-target, partial output),
                    // R=red (overloaded, suppressed). One entry per MoeBlock per step.
                    let tlight_layers = take_tlight_capture();
                    if !tlight_layers.is_empty() {
                        let layer_strs: Vec<String> = tlight_layers.iter().enumerate()
                            .map(|(i, (g, o, r, s))| format!("L{}:{}(G{}/O{}/R{})", i, s, g, o, r))
                            .collect();
                        let _ = writeln!(f, "TLIGHT step={} {}", *global_step, layer_strs.join(" "));
                        // Update mycelium's last-seen tlight per layer for epoch summary.
                        for (i, (_g, _o, _r, s)) in tlight_layers.iter().enumerate() {
                            if i < last_tlight_states.len() {
                                last_tlight_states[i] = s.clone();
                            }
                        }
                    }
                }
                clear_routing_capture();

                // DIV — per-layer normalized variance of expert outputs. Fires on every
                // optimizer step batch where div was active. 0=collapsed, ~1=diverse.
                // Outside the 10-batch block: batch_idx%10==0 and (batch_idx+1)%4==0
                // are mutually exclusive (gcd=2), so DIV would never fire inside it.
                if !div_log_vals.is_empty() {
                    let div_str: Vec<String> = div_log_vals.iter()
                        .map(|&v| format!("{:.4}", v)).collect();
                    let _ = writeln!(f, "DIV step={} w={:.2e} L={}", *global_step, cur_div_weight, div_str.join(","));
                }
                // DIVF32 — F32 shadow weight variance across experts per layer.
                // H3 diagnostic: if DIVF32 grows while DIV stays flat at 0.0036,
                // F32 weights are diverging but not crossing ternary thresholds.
                // If DIVF32 is also flat, DIV gradient is not flowing at all.
                if !div_f32_log_vals.is_empty() {
                    let f32_str: Vec<String> = div_f32_log_vals.iter()
                        .map(|&v| format!("{:.2e}", v)).collect();
                    let _ = writeln!(f, "DIVF32 step={} L={}", *global_step, f32_str.join(","));
                }
                // DIVGRAD — variance of expert weight gradient norms per layer.
                // Definitive H3 test: non-zero = DIV gradient differentially flowing;
                // all-zero/uniform = gradient not differentiating experts.
                if !div_grad_vals.is_empty() {
                    let grad_str: Vec<String> = div_grad_vals.iter()
                        .map(|&v| format!("{:.2e}", v)).collect();
                    let _ = writeln!(f, "DIVGRAD step={} L={}", *global_step, grad_str.join(","));
                }

                // TELE — sparsity snapshot every 30 batches (~60s) for live dashboard panels.
                // Epoch-end emit still fires below; this keeps LAYER/EXPERT panels from going stale.
                if batch_idx % 30 == 0 {
                    emit_telemetry(&varmap, &config, log_path);
                }

                let _ = f.flush();
            }

            *global_step += 1;
        }

        let avg_loss = if counted_batches > 0 {
            total_loss / counted_batches as f32
        } else {
            f32::MAX
        };

        let epoch_s = epoch_start.elapsed().as_secs();
        let summary = format!(
            "=== Epoch {}L done | Avg Loss: {:.4} | Clipped: {} | Skipped: {} | {:02}:{:02} elapsed ===",
            config.num_layers, avg_loss, clipped_steps, skipped_steps, epoch_s / 60, epoch_s % 60
        );
        println!("[{}] {}", timestamp(), summary);

        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = writeln!(f, "{}", summary);
        }

        evolution_manager.add_loss(avg_loss);

        // ── LB status log + expert dominance tripwire ─────────────────────────
        {
            let lb_line = if flags.lb_weight == 0.0 {
                format!("[lb] disabled epoch={}", total_epochs)
            } else {
                format!("[lb] active weight={:.3e} epoch={}", effective_lb, total_epochs)
            };
            println!("[{}] {}", timestamp(), lb_line);
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
                let _ = writeln!(f, "{}", lb_line);
            }

            if epoch_route_count > 0 {
                let avg_routes: Vec<f32> = epoch_route_acc.iter()
                    .map(|&s| s / epoch_route_count as f32)
                    .collect();
                for (e, &avg_r) in avg_routes.iter().enumerate() {
                    if avg_r > 0.70 {
                        expert_dominance_streak[e] = expert_dominance_streak[e].saturating_add(1);
                        if expert_dominance_streak[e] >= 3 {
                            let warn = format!(
                                "[lb] WARNING: Expert {} dominated routing {:.1}% for {} consecutive epochs — collapse risk",
                                e, avg_r * 100.0, expert_dominance_streak[e]
                            );
                            println!("[{}] {}", timestamp(), warn);
                            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
                                let _ = writeln!(f, "{}", warn);
                            }
                        }
                    } else {
                        expert_dominance_streak[e] = 0;
                    }
                }
            }
        }

        // ── Mycelium epoch recording ──────────────────────────────────────────
        let epoch_layer_norms: Vec<f32> = epoch_layer_norm_acc.iter()
            .map(|&s| if epoch_layer_norm_count > 0 { s / epoch_layer_norm_count as f32 } else { 0.0 })
            .collect();
        mycelium.record_epoch(&last_tlight_states, &epoch_layer_norms);
        let report = mycelium.generate_report();
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
            let pressure_str: Vec<String> = report.layer_pressure.iter()
                .map(|&p| format!("{:.5}", p)).collect();
            let _ = writeln!(f,
                "MYCELIUM epoch={} dead={} blooming={} hot=L{} cold=L{} pressure=[{}]",
                total_epochs, report.dead_expert_count, report.blooming_expert_count,
                report.hottest_layer, report.coldest_layer, pressure_str.join(","));
        }
        if !report.resurrections.is_empty() {
            println!("[{}] MYCELIUM: {} dead expert(s) detected — performing resurrection.",
                timestamp(), report.resurrections.len());
            let _ = perform_resurrection(checkpoint_path, &report.resurrections, device);
            // Reload resurrected weights into the live model.
            if let Ok(n) = load_checkpoint(&varmap, checkpoint_path, device) {
                println!("[{}] MYCELIUM: Reloaded {} tensors after resurrection.", timestamp(), n);
            }
        }

        // ── Wald: loss-space coverage analysis ───────────────────────────────
        let wald_report = wald.end_epoch();
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = writeln!(f, "{}", format_wald_line(total_epochs as usize, *global_step, &wald_report));
        }
        // WALD feedback → early-layer gradient amplification.
        // When the dead zone is structural (stale N+ epochs) amplification stops
        // helping — drop scale to 1.0 (neutral) to avoid over-boosting a plateau.
        // Re-arms automatically when loss shifts enough to reset stale_count.
        let severity = wald_report.low_gap_severity();
        if wald_report.stale {
            wald_amplify_scale = 1.0;
            println!(
                "[{}] WALD: structural plateau ({} stable epochs, sev={:.3} mass={:.3}) \
                 → amplify OFF",
                timestamp(), wald_report.stale_count, severity, wald_report.mass_center,
            );
        } else if severity > 0.15 {
            let t = ((severity - 0.15) / 0.85).min(1.0) as f64;
            wald_amplify_scale = 8.0 + t * 40.0;
            if let Some(ref zone) = wald_report.dead_low {
                println!(
                    "[{}] WALD: dead_low={:.2}–{:.2} severity={:.3} → early-layer scale={:.1}×  \
                     (fill={:.1}% mass={:.3})",
                    timestamp(), zone.lo, zone.hi, severity, wald_amplify_scale,
                    wald_report.fill_pct, wald_report.mass_center,
                );
            }
        } else {
            wald_amplify_scale = 4.0;
            println!("[{}] WALD: no significant dead zone — early-layer scale={:.1}×", timestamp(), wald_amplify_scale);
        }

        // ── Checkpoint (always save latest) ──────────────────────────────────
        save_checkpoint(&varmap, checkpoint_path)?;
        fs::write(meta_path, total_epochs.to_string())?;

        // ── Best Checkpoint (save only when avg_loss improves) — LLB §11.6 ───
        if avg_loss < best_epoch_loss {
            best_epoch_loss = avg_loss;
            save_checkpoint(&varmap, best_path)?;
            fs::write(best_meta_path, avg_loss.to_string())?;
            println!("[{}] ★ New best epoch loss: {:.4} — best checkpoint saved.", timestamp(), avg_loss);
        }

        // ── Hot-swap: file-triggered binary reload (safe — checkpoint already written) ──
        // Workflow: cargo build --release && touch models/.hot_reload
        // At the next epoch boundary the process flushes, exec()s the new binary,
        // which loads the fresh checkpoint and continues training from ep N+1.
        let hot_reload_trigger = format!("{}/models/.hot_reload", flags.root);
        if std::path::Path::new(&hot_reload_trigger).exists() {
            let _ = std::fs::remove_file(&hot_reload_trigger);
            println!("[{}] [HOT-SWAP] trigger detected — checkpoint fresh, launching new binary",
                timestamp());
            if let Some(ref mut f) = log_file { let _ = f.flush(); }
            drop(log_file.take()); // close log fd before exec
            let exe = std::env::current_exe().expect("hot-swap: current_exe");
            let args: Vec<String> = std::env::args().skip(1).collect();
            // exec() replaces the process image — only returns on failure
            eprintln!("[HOT-SWAP] exec failed: {:?}",
                std::process::Command::new(&exe).args(&args).exec());
            std::process::exit(1);
        }

        // ── Epoch summary (8-line pitch table row) ───────────────────────────
        {
            let ttlfreeze_total: usize = epoch_burst_count.iter().sum();
            let ttlfreeze_layers: Vec<String> = epoch_burst_count.iter().enumerate()
                .filter(|(_, c)| **c > 0)
                .map(|(i, c)| format!("L{}x{}", i, c))
                .collect();
            let freeze_str = if ttlfreeze_layers.is_empty() {
                "none".to_string()
            } else {
                ttlfreeze_layers.join(",")
            };
            let l03_pressure: Vec<String> = epoch_layer_norms.iter().take(4)
                .map(|&p| format!("{:.2e}", p)).collect();
            let summary_line = format!(
                "EPOCH_SUMMARY epoch={} loss_avg={:.4} (d{:+.4}) loss_best={:.4} \
                 wald_sev={:.3} wald_fill={:.1}% \
                 ttlfreeze={} ({}) \
                 myc_L0-L3=[{}] hot=L{} cold=L{}",
                total_epochs, avg_loss, avg_loss - prev_avg_loss, best_epoch_loss,
                wald_report.low_gap_severity(), wald_report.fill_pct,
                ttlfreeze_total, freeze_str,
                l03_pressure.join("/"),
                report.hottest_layer, report.coldest_layer,
            );
            println!("[{}] {}", timestamp(), summary_line);
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
                let _ = writeln!(f, "{}", summary_line);
            }
            prev_avg_loss = avg_loss;

            // Cascade evidence: L0-L3 avg norm at burst-10 vs burst+30, grouped by layer.
            if !cascade_results.is_empty() {
                let mut by_layer: std::collections::HashMap<usize, Vec<([f32; 4], [f32; 4])>> = std::collections::HashMap::new();
                for &(li, _fs, pre, post) in &cascade_results {
                    by_layer.entry(li).or_default().push((pre, post));
                }
                let mut layers_sorted: Vec<usize> = by_layer.keys().cloned().collect();
                layers_sorted.sort();
                for li in layers_sorted {
                    let events = &by_layer[&li];
                    let n = events.len() as f32;
                    let mut pre_avg = [0.0f32; 4];
                    let mut post_avg = [0.0f32; 4];
                    for (pre, post) in events {
                        for j in 0..4 { pre_avg[j] += pre[j] / n; post_avg[j] += post[j] / n; }
                    }
                    let pre_mean: f32 = pre_avg.iter().sum::<f32>() / 4.0;
                    let post_mean: f32 = post_avg.iter().sum::<f32>() / 4.0;
                    let pct = if pre_mean > 1e-12 { (post_mean / pre_mean - 1.0) * 100.0 } else { 0.0 };
                    let cascade_line = format!(
                        "TTLFREEZE CASCADE layer=L{} events={} L0-L3_pre={:.2e} post={:.2e} ({:+.0}%)",
                        li, events.len(), pre_mean, post_mean, pct
                    );
                    println!("[{}] {}", timestamp(), cascade_line);
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
                        let _ = writeln!(f, "{}", cascade_line);
                    }
                }
            }
        }

        // Emit telemetry for the dashboard neural viz panels.
        emit_telemetry(&varmap, &config, log_path);

        // Write training_telemetry.json for the public /benchmarks/training API endpoint.
        // The KPI workflow uploads this file to the Fly.io volume every sync cycle.
        let telem_json = serde_json::json!({
            "model": "albert-moe-13",
            "version": "v2.0.0",
            "timestamp": timestamp(),
            "architecture": {
                "layers": config.num_layers,
                "hidden": config.hidden_size,
                "experts": config.num_experts,
                "ctx": config.max_seq_len,
                "vocab": config.vocab_size
            },
            "training": {
                "global_epoch": total_epochs,
                "avg_loss": avg_loss,
                "best_loss": best_epoch_loss,
                "clipped_steps": clipped_steps,
                "skipped_steps": skipped_steps
            }
        });
        let _ = fs::write("dashboard/training_telemetry.json", telem_json.to_string());

        // ── Collapse Detection & Rollback ─────────────────────────────────────
        // A post-surgery layer can destabilise the model — avg loss locks at
        // ln(vocab) ≈ 8.987 (uniform distribution). Detect 3 consecutive such
        // epochs and roll back to the best checkpoint before triggering surgery.
        if avg_loss > COLLAPSE_THRESHOLD {
            collapse_streak += 1;
            println!("[{}] ⚠ Collapse streak {}/{}: avg loss {:.4} above threshold {:.1}",
                timestamp(), collapse_streak, COLLAPSE_STREAK_LIMIT, avg_loss, COLLAPSE_THRESHOLD);

            if collapse_streak >= COLLAPSE_STREAK_LIMIT {
                // If no usable best checkpoint exists, or its recorded loss is also
                // above threshold, rolling back won't help — force surgery now.
                let best_exists = std::path::Path::new(best_path).exists();
                let best_recorded: f32 = fs::read_to_string(best_meta_path)
                    .ok().and_then(|s| s.trim().parse().ok())
                    .unwrap_or(f32::MAX);
                if !best_exists || best_recorded >= COLLAPSE_THRESHOLD {
                    // Max-layers guard: forced surgery must respect the layer cap,
                    // same as the normal should_evolve path.
                    if config.num_layers >= evolution_manager.max_layers {
                        println!("[{}] COLLAPSE→SURGERY suppressed: already at max_layers={} — \
                            continuing at current depth.", timestamp(), evolution_manager.max_layers);
                        collapse_streak = 0;
                        evolution_manager.reset_history();
                        continue;
                    }
                    println!("[{}] ★ COLLAPSE→SURGERY: best checkpoint ({:.4}) also above threshold \
                        — rolling back won't recover. Forcing layer surgery.",
                        timestamp(), best_recorded);
                    collapse_streak = 0;
                    // Return true to trigger surgery in the outer loop.
                    return Ok(true);
                }

                let rollback_src = if std::path::Path::new(best_path).exists() {
                    best_path
                } else {
                    checkpoint_path
                };
                let msg = format!("COLLAPSE_ROLLBACK streak={} avg={:.4} src={}",
                    collapse_streak, avg_loss, rollback_src);
                println!("[{}] ★ {}", timestamp(), msg);
                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
                    let _ = writeln!(f, "{}", msg);
                }

                // Restore best weights.
                if let Ok(n) = load_checkpoint(&varmap, rollback_src, device) {
                    println!("[{}] Rolled back {} tensors from {}.", timestamp(), n, rollback_src);
                }
                // Fresh optimizer — AdamW momentum is stale/corrupted after collapse.
                opt = candle_nn::AdamW::new_lr(varmap.all_vars(), base_lr)?;
                wald_amplify_scale = 8.0; // fresh optimizer — reset to strong default

                collapse_streak = 0;
                evolution_manager.reset_history(); // don't immediately re-trigger surgery
                continue; // skip should_evolve this epoch — give the model a clean epoch
            }
        } else {
            collapse_streak = 0; // healthy epoch resets the streak
        }

        if evolution_manager.should_evolve(config.num_layers) {
            evolution_manager.reset_history();
            return Ok(true);
        }
    }
}

/// Stage-aware corpus loader. Loads all data/corpus/stage_N/ dirs where N ≤ num_layers.
/// Each surgery increments num_layers, automatically unlocking richer data.
///
/// Stage plan:
///   stage_3  — Bible + Alice          (grammar, vocab, basic syntax)
///   stage_6  — Gutenberg novels       (complex narrative, wider vocabulary)
///   stage_7  — Simple Wikipedia       (factual, diverse topics)
///   stage_9  — qa_instruction.txt     (User:/Albert: instruction format)
///   stage_11 — Linux docs, EU AI Act  (technical/specialized language)
fn load_corpus(tokenizer: &BpeTokenizer, num_layers: usize, root: &str) -> Vec<u32> {
    let corpus_root = format!("{root}/data/corpus");
    let corpus_root = corpus_root.as_str();
    // Tokenize file-by-file so we never hold the full corpus text in RAM.
    // Concatenating 635MB of text before encoding spiked peak RAM to 3-4GB (OOM).
    let mut all_tokens: Vec<u32> = Vec::new();
    let mut stages_loaded: Vec<usize> = Vec::new();

    // Collect all stage_N subdirectories where N ≤ num_layers
    if let Ok(entries) = fs::read_dir(corpus_root) {
        let mut stage_dirs: Vec<(usize, std::path::PathBuf)> = entries
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let name = e.file_name().into_string().ok()?;
                let n: usize = name.strip_prefix("stage_")?.parse().ok()?;
                Some((n, e.path()))
            })
            .filter(|(n, _)| *n <= num_layers)
            .collect();
        stage_dirs.sort_by_key(|(n, _)| *n);

        for (stage_n, dir) in &stage_dirs {
            if let Ok(files) = fs::read_dir(dir) {
                let mut paths: Vec<_> = files
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().map(|x| x == "txt").unwrap_or(false))
                    .map(|e| e.path())
                    .collect();
                paths.sort();
                for path in &paths {
                    match fs::read_to_string(path) {
                        Ok(text) => {
                            println!("[{}] Corpus stage_{}: {} ({} chars)",
                                timestamp(), stage_n, path.file_name().unwrap().to_string_lossy(), text.len());
                            all_tokens.extend(tokenizer.encode(&text));
                        }
                        Err(e) => eprintln!("Warning: could not read {:?}: {}", path, e),
                    }
                }
                stages_loaded.push(*stage_n);
            }
        }
    }

    if all_tokens.is_empty() {
        panic!("No corpus found in {}/stage_N/ dirs for num_layers={}", corpus_root, num_layers);
    }

    println!("[{}] Active corpus stages: {:?} (model depth: {}L)",
        timestamp(), stages_loaded, num_layers);

    // v3.0 additional corpora — multilingual, academic, fulltext, chaos.
    // Loaded unconditionally when present; these dirs don't follow stage_N naming.
    let v3_dirs = ["multilingual", "academic", "fulltext", "chaos"];
    for dir_name in &v3_dirs {
        let dir = std::path::Path::new(corpus_root).join(dir_name);
        if !dir.exists() { continue; }
        let read_dir = match fs::read_dir(&dir) {
            Ok(rd) => rd,
            Err(e) => { eprintln!("Warning: could not read corpus dir {:?}: {}", dir, e); continue; }
        };
        let mut paths: Vec<_> = read_dir
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|x| x == "txt").unwrap_or(false))
            .map(|e| e.path())
            .collect();
        paths.sort();
        let mut dir_bytes = 0usize;
        for path in &paths {
            match fs::read_to_string(path) {
                Ok(text) => {
                    dir_bytes += text.len();
                    all_tokens.extend(tokenizer.encode(&text));
                }
                Err(e) => eprintln!("Warning: could not read {:?}: {}", path, e),
            }
        }
        if dir_bytes > 0 {
            println!("[{}] Corpus {}: {} files · {:.1}MB",
                timestamp(), dir_name, paths.len(), dir_bytes as f64 / 1_048_576.0);
        }
    }

    all_tokens
}

/// Load tokenized corpus, using a binary cache to skip re-tokenization on restart.
/// Cache is stored as: 4 bytes (num_layers as u32 LE) + N×4 bytes (token ids).
/// Invalidated automatically when any corpus source file is newer than the cache,
/// or when num_layers changes (surgery / new stage unlocked).
fn load_corpus_cached(tokenizer: &BpeTokenizer, num_layers: usize, root: &str) -> Vec<u32> {
    let cache_path = format!("{root}/data/corpus_cache.bin");
    let cache_path = cache_path.as_str();

    // All dirs that contribute to the corpus — used for mtime freshness check.
    let watch_dirs = [
        format!("{root}/data/corpus"),
        format!("{root}/data/multilingual"),
        format!("{root}/data/academic"),
        format!("{root}/data/fulltext"),
        format!("{root}/data/chaos"),
    ];

    let cache_valid = (|| -> Option<bool> {
        let cache_mtime = fs::metadata(cache_path).ok()?.modified().ok()?;
        // Validate stored num_layers header
        let header_bytes = fs::read(cache_path).ok()?;
        if header_bytes.len() < 4 { return Some(false); }
        let stored_layers = u32::from_le_bytes(header_bytes[..4].try_into().ok()?) as usize;
        if stored_layers != num_layers { return Some(false); }
        // Check every corpus file is older than the cache
        for dir in &watch_dirs {
            if let Ok(rd) = fs::read_dir(dir) {
                for entry in rd.flatten() {
                    if let Ok(mtime) = entry.metadata().and_then(|m| m.modified()) {
                        if mtime > cache_mtime { return Some(false); }
                    }
                    // Recurse one level into stage_N subdirs
                    if entry.path().is_dir() {
                        if let Ok(sub) = fs::read_dir(entry.path()) {
                            for f in sub.flatten() {
                                if let Ok(mtime) = f.metadata().and_then(|m| m.modified()) {
                                    if mtime > cache_mtime { return Some(false); }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(true)
    })().unwrap_or(false);

    if cache_valid {
        if let Ok(bytes) = fs::read(cache_path) {
            if bytes.len() > 4 && bytes.len() % 4 == 0 {
                let tokens: Vec<u32> = bytes[4..].chunks_exact(4)
                    .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                    .collect();
                println!("[{}] Corpus cache hit — {} tokens loaded instantly (skipped tokenization).",
                    timestamp(), tokens.len());
                return tokens;
            }
        }
        eprintln!("Warning: cache file corrupt, re-tokenizing.");
    }

    let tokens = load_corpus(tokenizer, num_layers, root);

    // Write cache: 4-byte header (num_layers) + token data
    let mut bytes: Vec<u8> = (num_layers as u32).to_le_bytes().to_vec();
    bytes.extend(tokens.iter().flat_map(|&t| t.to_le_bytes()));
    match fs::write(cache_path, &bytes) {
        Ok(_) => println!("[{}] Corpus cache saved to {} ({:.0}MB) — next boot will be instant.",
            timestamp(), cache_path, bytes.len() as f64 / 1_048_576.0),
        Err(e) => eprintln!("Warning: could not write corpus cache: {e}"),
    }

    tokens
}

fn main() -> Result<()> {
    let flags = parse_args();
    let _ = ThreadPoolBuilder::new().num_threads(8).build_global();
    println!("--- ALBERT EVOLUTIONARY ORCHESTRATOR v3.0 (Multilingual · Mandelbrot Surgery · Chaos Protocol) ---");

    // Print flag summary so every log knows what run configuration was used.
    println!("[flags] lb_weight={} lb_ramp={} seed_experts={} div_override={}",
        flags.lb_weight,
        flags.lb_ramp_epochs,
        flags.seed_experts,
        flags.div_weight_override.map(|v| format!("{:.2e}", v)).unwrap_or_else(|| "schedule".into()),
    );

    let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
    println!("[{}] Device: {}", timestamp(), if matches!(device, Device::Cuda(_)) { "CUDA" } else { "CPU" });
    let r = &flags.root;
    let vocab_path      = format!("{r}/data/vocab_v3.json");
    let config_path     = format!("{r}/models/albert_v3.0.config.json");
    let checkpoint_path = format!("{r}/models/albert_v3.0.safetensors");
    let best_path       = format!("{r}/models/albert_v3.0.best.safetensors");
    let vocab_path      = vocab_path.as_str();
    let config_path     = config_path.as_str();
    let checkpoint_path = checkpoint_path.as_str();
    let best_path       = best_path.as_str();

    let tokenizer = BpeTokenizer::new(vocab_path);

    let mut evolution_manager = EvolutionManager::new();
    let mut global_step       = 0_usize;

    // Read initial layer count so MyceliumModule is sized correctly from the start.
    let initial_layers: usize = fs::read_to_string(config_path)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v["num_layers"].as_u64())
        .unwrap_or(3) as usize;
    evolution_manager.calibrate(initial_layers);
    let mut mycelium = MyceliumModule::new(initial_layers, 12);
    mycelium.set_lb_off_mode(flags.lb_weight == 0.0);
    let mut wald     = WaldModule::new();

    loop {
        // Read current layer depth to select the right corpus stages.
        let num_layers: usize = fs::read_to_string(config_path)
            .ok()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
            .and_then(|v| v["num_layers"].as_u64())
            .unwrap_or(3) as usize;

        let tokens = load_corpus_cached(&tokenizer, num_layers, &flags.root);
        println!("[{}] Total corpus: {} tokens ({}L model, stages ≤{})",
            timestamp(), tokens.len(), num_layers, num_layers);

        let needs_evolution = train_cycle(
            &tokens, &tokenizer, &device, &mut evolution_manager, &mut mycelium, &mut wald,
            &mut global_step, &flags
        )?;
        if needs_evolution {
            perform_surgery(config_path, checkpoint_path, best_path, &device, &flags.root)?;
            evolution_manager.promote_fib_target();
            mycelium.on_layer_added();
            wald.on_surgery();
            // Log the MAND event so the dashboard can mark surgery epochs visually.
            let post_layers: usize = fs::read_to_string(config_path)
                .ok()
                .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
                .and_then(|v| v["num_layers"].as_u64())
                .unwrap_or(0) as usize;
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("dashboard/training.log") {
                let mand_line = format_mandelbrot_line(
                    0,  // epoch not tracked at outer-loop scope; MAND line in perform_surgery has the detail
                    global_step as usize,
                    post_layers.saturating_sub(1),
                    0,
                    1e-3, 64,
                );
                let _ = writeln!(f, "{}", mand_line);
            }
            global_step = 0;
            // Corpus reloaded at top of next loop iteration with updated num_layers.
        }
    }
}
