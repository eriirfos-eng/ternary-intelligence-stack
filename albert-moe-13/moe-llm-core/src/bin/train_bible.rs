use candle_core::{Device, DType, Result, Tensor};
use candle_nn::{Optimizer, VarBuilder, loss, VarMap};
use moe_llm_core::model::{Transformer, TransformerConfig, clear_routing_capture, take_routing_capture, clear_entropy_capture, take_entropy_capture, clear_lb_capture, take_lb_capture, clear_tlight_capture, take_tlight_capture};
use moe_llm_core::tokenizer::BpeTokenizer;
use moe_llm_core::evolution::EvolutionManager;
use moe_llm_core::mycelium::MyceliumModule;
use moe_llm_core::wald::{WaldModule, format_wald_line};
use moe_llm_core::mandelbrot::{MandelbrotSurgery, format_mandelbrot_line};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use rayon::ThreadPoolBuilder;
use serde_json::{Value, json};
use std::collections::HashMap;

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
// Direct LR boost applied after Adam's step for cold layers (norm < THRESHOLD).
// Adam normalizes away gradient amplification via its second-moment denominator,
// so we bypass it entirely: a sign-normalized SGD step with lr = current_lr * COLD_LR_BOOST.
// Self-terminating: once a layer's norm exceeds THRESHOLD the condition is false.
const COLD_LR_BOOST: f64 = 6.0;

// TTL burst detection — freeze logit modifiers when per-layer grad norm spikes.
// GRAD_NORM_EMA_ALPHA: ~1/α ≈ 50-step window for the per-layer baseline EMA.
// BURST_RATIO_THRESHOLD: fire when current_norm / baseline > this value.
// TTL_FREEZE_GRAD_STEPS: freeze duration in optimizer steps; multiply by GRAD_ACCUM_STEPS
//   for the update() call count passed to TrafficLight::freeze().
// MAX_BURSTS_PER_EPOCH: safety circuit — disable further freezes after this many per epoch per layer.
const GRAD_NORM_EMA_ALPHA: f32 = 0.02;
const BURST_RATIO_THRESHOLD: f32 = 5.0;
const TTL_FREEZE_GRAD_STEPS: usize = 50;
const MAX_BURSTS_PER_EPOCH: usize = 3;

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

// Net2Net safe-copy layer surgery — whitepaper §11.2 (EvolutionManager)
fn perform_surgery(config_path: &str, checkpoint_path: &str, best_path: &str, device: &Device) -> Result<()> {
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
    let archive_path = format!("models/albert_v3.0.best.{}L.safetensors", old_layers);
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
) -> Result<bool> {
    let checkpoint_path = "models/albert_v3.0.safetensors";
    let best_path       = "models/albert_v3.0.best.safetensors";
    let config_path     = "models/albert_v3.0.config.json";
    let meta_path       = "models/albert_v3.0.meta";
    let best_meta_path  = "models/albert_v3.0.best_loss";
    let log_path        = "dashboard/training.log";

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
    let base_lr        = 3e-4_f64;   // 2e-4 → 3e-4: stronger signal to escape zero basin
    let min_lr         = 1e-5_f64;
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
    let mut grad_norm_ema: Vec<f32> = vec![1.0_f32; config.num_layers];
    // Safety circuit: how many TTL freeze events fired per layer this epoch.
    let mut epoch_burst_count: Vec<usize> = vec![0; config.num_layers];

    // Write arch metadata once per train_cycle so the dashboard can display it.
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = writeln!(f, "ARCH {}L {}H {}E {}CTX {}V",
            config.num_layers, config.hidden_size, config.num_experts,
            config.max_seq_len, config.vocab_size);
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

            let start = rand::random::<usize>() % (tokens.len() - seq_len - 1);
            let input_tensor = Tensor::new(&tokens[start..start + seq_len], device)?
                .reshape((1, seq_len))?
                .to_dtype(DType::U32)?;
            let target_tensor = Tensor::new(&tokens[start + 1..start + seq_len + 1], device)?
                .reshape((1, seq_len))?
                .to_dtype(DType::U32)?;

            clear_entropy_capture();
            clear_lb_capture();
            clear_tlight_capture();
            let logits      = model.forward(&input_tensor)?;
            let logits      = logits.reshape((seq_len, config.vocab_size))?;
            let target_flat = target_tensor.flatten_all()?;
            let ce_loss     = loss::cross_entropy(&logits, &target_flat)?;

            // ── Auxiliary losses ──────────────────────────────────────────────
            // L1 is gated on loss < 8.0 — applying to a collapsed model makes recovery harder.
            // LB loss is NOT gated: it works through gate probabilities independent of CE signal,
            // and this is the core fix for expert homogeneity — must always be active.
            let real_loss_preview = ce_loss.to_scalar::<f32>().unwrap_or(f32::MAX);
            let aux_active     = real_loss_preview < 8.0;
            let l1_lambda      = if aux_active { 1e-5_f64  } else { 0.0_f64 };
            let entropy_lambda = 0.0_f64;
            // Switch Transformer load-balancing coefficient. Bumped 0.01→0.02 at Global ~361, 0.02→0.03 at Global ~385 to arrest ENTR re-uniformization.
            let lb_lambda      = 0.03_f64;

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

            // LB loss: always active. Gradient through P_i pushes over-used experts down.
            // This is independent of CE — works even when experts produce identical outputs.
            if let Some(lb) = lb_term {
                batch_loss = (&batch_loss + (lb * lb_lambda)?)?;
            }

            let real_loss = ce_loss.to_scalar::<f32>()?;

            // Flag explosion — will flush and skip the whole accumulation window.
            if real_loss.is_nan() || real_loss.is_infinite() || real_loss > LOSS_EXPLOSION_THRESHOLD {
                accum_exploded = true;
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

            let is_step_batch = (batch_idx + 1) % GRAD_ACCUM_STEPS == 0
                || batch_idx + 1 == num_batches;

            // Default: empty norms for non-step batches (used in GRAD log below).
            let mut layer_norms = vec![0.0_f32; config.num_layers];
            let mut norm = 0.0_f32;

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
                    norm = global_grad_norm(&varmap, &grads);
                    layer_norms = per_layer_grad_norm(&varmap, &grads, config.num_layers);
                    for (i, &n) in layer_norms.iter().enumerate() {
                        if i < epoch_layer_norm_acc.len() { epoch_layer_norm_acc[i] += n; }
                    }
                    epoch_layer_norm_count += 1;

                    // TTL burst detection — per-layer grad norm vs EMA baseline.
                    // When ratio > BURST_RATIO_THRESHOLD, freeze that layer's TTL logit modifiers
                    // for TTL_FREEZE_GRAD_STEPS optimizer steps so gate can learn without correction.
                    for (i, &n) in layer_norms.iter().enumerate() {
                        let baseline = grad_norm_ema[i];
                        if n > 0.0 {
                            let ratio = n / baseline.max(1e-12);
                            if ratio > BURST_RATIO_THRESHOLD && epoch_burst_count[i] < MAX_BURSTS_PER_EPOCH {
                                epoch_burst_count[i] += 1;
                                let freeze_update_steps = TTL_FREEZE_GRAD_STEPS * GRAD_ACCUM_STEPS;
                                model.freeze_ttl_layer(i, freeze_update_steps);
                                let freeze_end_step = *global_step + TTL_FREEZE_GRAD_STEPS;
                                println!("[{}] TTLFREEZE layer={} step={} grad={:.2e} base={:.2e} ratio={:.1} freeze_end={} fires_this_epoch={}",
                                    timestamp(), i, *global_step, n, baseline, ratio, freeze_end_step, epoch_burst_count[i]);
                                if let Some(ref mut f) = log_file {
                                    let _ = writeln!(f, "TTLFREEZE step={} layer={} grad={:.6} baseline={:.6} ratio={:.2} freeze_end={}",
                                        *global_step, i, n, baseline, ratio, freeze_end_step);
                                }
                                if epoch_burst_count[i] >= MAX_BURSTS_PER_EPOCH {
                                    println!("[{}] TTLFREEZE WARN layer={} reached {} fires/epoch — suppressing further freezes this epoch",
                                        timestamp(), i, MAX_BURSTS_PER_EPOCH);
                                    if let Some(ref mut f) = log_file {
                                        let _ = writeln!(f, "TTLFREEZE WARN step={} layer={} max_bursts_reached={}",
                                            *global_step, i, MAX_BURSTS_PER_EPOCH);
                                    }
                                }
                            }
                            // Update EMA after burst check so baseline tracks the normal level.
                            grad_norm_ema[i] = GRAD_NORM_EMA_ALPHA * n + (1.0 - GRAD_NORM_EMA_ALPHA) * baseline;
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
                    apply_layer_lr_boost(&varmap, &grads, &layer_norms, lr * COLD_LR_BOOST);
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

                // GRAD — per-layer gradient norm (computed above, before opt.step).
                // Dashboard parses "GRAD step=N n=X.XXXX L=n0,n1,n2,..."
                // 6dp so sub-millinorm block values are legible (embed dominates global norm).
                let ln_str: Vec<String> = layer_norms.iter()
                    .map(|n| format!("{:.6}", n)).collect();
                let _ = writeln!(f, "GRAD step={} n={:.4} L={}", *global_step, norm, ln_str.join(","));

                // ROUTE — expert routing weights, emitted every 10 batches to keep log lean.
                // Dashboard parses "ROUTE step=N E=w0,w1,...,w11"
                if batch_idx % 10 == 0 {
                    let routing = take_routing_capture(config.num_experts);
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
        // Severity maps linearly: 0.15→8×, 1.0→48×. Below 0.15 the dead zone is
        // small enough that early-layer freeze is not the limiting factor.
        let severity = wald_report.low_gap_severity();
        if severity > 0.15 {
            let t = ((severity - 0.15) / 0.85).min(1.0) as f64;
            wald_amplify_scale = 8.0 + t * 40.0;
        } else {
            wald_amplify_scale = 4.0;
        }
        if let Some(ref zone) = wald_report.dead_low {
            println!(
                "[{}] WALD: dead_low={:.2}–{:.2} severity={:.3} → early-layer scale={:.1}×  \
                 (fill={:.1}% mass={:.3})",
                timestamp(), zone.lo, zone.hi, severity, wald_amplify_scale,
                wald_report.fill_pct, wald_report.mass_center,
            );
        } else {
            println!("[{}] WALD: no dead zone below mass — early-layer scale={:.1}×", timestamp(), wald_amplify_scale);
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
fn load_corpus(tokenizer: &BpeTokenizer, num_layers: usize) -> Vec<u32> {
    let corpus_root = "data/corpus";
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
fn load_corpus_cached(tokenizer: &BpeTokenizer, num_layers: usize) -> Vec<u32> {
    const CACHE_PATH: &str = "data/corpus_cache.bin";

    // All dirs that contribute to the corpus — used for mtime freshness check.
    let watch_dirs = ["data/corpus", "data/multilingual", "data/academic", "data/fulltext", "data/chaos"];

    let cache_valid = (|| -> Option<bool> {
        let cache_mtime = fs::metadata(CACHE_PATH).ok()?.modified().ok()?;
        // Validate stored num_layers header
        let header_bytes = fs::read(CACHE_PATH).ok()?;
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
        if let Ok(bytes) = fs::read(CACHE_PATH) {
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

    let tokens = load_corpus(tokenizer, num_layers);

    // Write cache: 4-byte header (num_layers) + token data
    let mut bytes: Vec<u8> = (num_layers as u32).to_le_bytes().to_vec();
    bytes.extend(tokens.iter().flat_map(|&t| t.to_le_bytes()));
    match fs::write(CACHE_PATH, &bytes) {
        Ok(_) => println!("[{}] Corpus cache saved to {} ({:.0}MB) — next boot will be instant.",
            timestamp(), CACHE_PATH, bytes.len() as f64 / 1_048_576.0),
        Err(e) => eprintln!("Warning: could not write corpus cache: {e}"),
    }

    tokens
}

fn main() -> Result<()> {
    let _ = ThreadPoolBuilder::new().num_threads(8).build_global();
    println!("--- ALBERT EVOLUTIONARY ORCHESTRATOR v3.0 (Multilingual · Mandelbrot Surgery · Chaos Protocol) ---");

    let device          = Device::Cpu;
    let vocab_path      = "data/vocab_v3.json";
    let config_path     = "models/albert_v3.0.config.json";
    let checkpoint_path = "models/albert_v3.0.safetensors";
    let best_path       = "models/albert_v3.0.best.safetensors";

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
    let mut wald     = WaldModule::new();

    loop {
        // Read current layer depth to select the right corpus stages.
        let num_layers: usize = fs::read_to_string(config_path)
            .ok()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
            .and_then(|v| v["num_layers"].as_u64())
            .unwrap_or(3) as usize;

        let tokens = load_corpus_cached(&tokenizer, num_layers);
        println!("[{}] Total corpus: {} tokens ({}L model, stages ≤{})",
            timestamp(), tokens.len(), num_layers, num_layers);

        let needs_evolution = train_cycle(
            &tokens, &tokenizer, &device, &mut evolution_manager, &mut mycelium, &mut wald, &mut global_step
        )?;
        if needs_evolution {
            perform_surgery(config_path, checkpoint_path, best_path, &device)?;
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
