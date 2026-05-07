use candle_core::{Device, DType, Result, Tensor};
use candle_nn::{Optimizer, VarBuilder, loss, VarMap};
use moe_llm_core::model::{Transformer, TransformerConfig, clear_routing_capture, take_routing_capture};
use moe_llm_core::tokenizer::BpeTokenizer;
use moe_llm_core::evolution::EvolutionManager;
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
// ln(vocab≈8006) ≈ 8.988 — anything above 9.5 is catastrophic.
const LOSS_EXPLOSION_THRESHOLD: f32 = 9.5;

// If the epoch-average loss sits at or above this for COLLAPSE_STREAK_LIMIT
// consecutive epochs the model has collapsed to uniform distribution.
// ln(8000) ≈ 8.987 — we trigger at 8.5 to catch early collapse before it
// wastes too many epochs. 2 epochs is enough signal at 6L+ — a genuine
// plateau never self-recovers at this depth; a transient would not hit 8.5.
const COLLAPSE_THRESHOLD:    f32 = 8.5;
const COLLAPSE_STREAK_LIMIT: u32 = 2;

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

    candle_core::safetensors::save(&new_tensors, checkpoint_path)?;
    // Best checkpoint is now outdated (wrong architecture) — remove it so we don't accidentally load it.
    let _ = fs::remove_file(best_path);
    println!("[{}] Surgery Complete: Layer {} cloned from Layer {}.", timestamp(), target_layer, source_layer);
    Ok(())
}

fn train_cycle(
    tokens: &[u32],
    tokenizer: &BpeTokenizer,
    device: &Device,
    evolution_manager: &mut EvolutionManager,
    global_step: &mut usize,
) -> Result<bool> {
    let checkpoint_path = "models/bible_ternary_v2.0.0.safetensors";
    let best_path       = "models/bible_ternary_v2.0.0.best.safetensors";
    let config_path     = "models/bible_ternary_v2.0.0.config.json";
    let meta_path       = "models/bible_ternary_v2.0.0.meta";
    let best_meta_path  = "models/bible_ternary_v2.0.0.best_loss";
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

    // Track best epoch-average loss across the entire training run.
    let mut best_epoch_loss: f32 = fs::read_to_string(best_meta_path)
        .ok()
        .and_then(|s| s.trim().parse::<f32>().ok())
        .unwrap_or(f32::MAX);

    let mut total_epochs = if let Ok(c) = fs::read_to_string(meta_path) {
        c.trim().parse::<u32>().unwrap_or(0)
    } else { 0 };

    // Cosine LR: starts high, decays to near-zero over lr_cycle_steps global steps
    let base_lr        = 2e-4_f64;
    let min_lr         = 1e-5_f64;
    let lr_cycle_steps = 500_usize;

    let mut opt = candle_nn::AdamW::new_lr(varmap.all_vars(), base_lr)?;
    let mut collapse_streak: u32 = 0;

    let seq_len     = config.max_seq_len;
    let num_batches = 300_usize;

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
        let mut clipped_steps = 0u32;
        let mut skipped_steps = 0u32;

        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .ok();

        let epoch_start = Instant::now();

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

            let logits      = model.forward(&input_tensor)?;
            let logits      = logits.reshape((seq_len, config.vocab_size))?;
            let target_flat = target_tensor.flatten_all()?;
            let ce_loss     = loss::cross_entropy(&logits, &target_flat)?;

            // L1 sparsity reward: push weights toward 0 so ternary zeroing is strategic.
            let l1_lambda = 1e-5_f64;
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
            let batch_loss = (&ce_loss + (l1_penalty * l1_lambda)?)?;

            // ── Gradient Clipping ─────────────────────────────────────────────
            // Split backward() and step() so we can inspect the gradient norm
            // before applying the update. If norm > MAX_GRAD_NORM, scale the
            // loss down proportionally (equivalent to clipping all gradients).
            let grads = batch_loss.backward()?;
            let norm  = global_grad_norm(&varmap, &grads);
            // Capture per-layer norms BEFORE opt.step() — optimizer updates tensor IDs in place,
            // which invalidates the GradStore lookup for any Var used after the step.
            let layer_norms = per_layer_grad_norm(&varmap, &grads, config.num_layers);

            let real_loss = ce_loss.to_scalar::<f32>()?;

            // Skip batches where the model has already exploded.
            if real_loss.is_nan() || real_loss.is_infinite() || real_loss > LOSS_EXPLOSION_THRESHOLD {
                skipped_steps += 1;
                println!("[{}] [SKIP] Batch {} — loss {:.4} (explosion), skip & preserve weights.",
                    timestamp(), batch_idx, real_loss);
                *global_step += 1;
                continue;
            }

            if norm > MAX_GRAD_NORM && norm.is_finite() {
                // Gradient norm is too large — recompute with scaled loss so the
                // effective gradient is exactly MAX_GRAD_NORM.
                clipped_steps += 1;
                let scale = (MAX_GRAD_NORM / norm) as f64;
                let scaled_loss = (&batch_loss * scale)?;
                opt.backward_step(&scaled_loss)?;
            } else {
                opt.step(&grads)?;
            }

            total_loss       += real_loss;
            counted_batches  += 1;

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
                // If the best checkpoint is itself above threshold, rolling back
                // won't help — the model needs more capacity. Force surgery now.
                let best_recorded: f32 = fs::read_to_string(best_meta_path)
                    .ok().and_then(|s| s.trim().parse().ok())
                    .unwrap_or(f32::MAX);
                if best_recorded >= COLLAPSE_THRESHOLD {
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

fn load_corpus(tokenizer: &BpeTokenizer) -> Vec<u32> {
    let corpus_dir = "data/corpus";
    let mut all_text = String::new();

    if let Ok(entries) = fs::read_dir(corpus_dir) {
        let mut paths: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|x| x == "txt").unwrap_or(false))
            .map(|e| e.path())
            .collect();
        paths.sort();
        for path in &paths {
            match fs::read_to_string(path) {
                Ok(text) => {
                    println!("[{}] Loaded corpus: {} ({} chars)", timestamp(), path.display(), text.len());
                    all_text.push_str(&text);
                    all_text.push('\n');
                }
                Err(e) => eprintln!("Warning: could not read {:?}: {}", path, e),
            }
        }
    }

    if all_text.is_empty() {
        panic!("No corpus files found in {}", corpus_dir);
    }

    tokenizer.encode(&all_text)
}

fn main() -> Result<()> {
    let _ = ThreadPoolBuilder::new().num_threads(8).build_global();
    println!("--- ALBERT EVOLUTIONARY ORCHESTRATOR v2.4 (Gradient Clipping + Best Checkpoint) ---");

    let device          = Device::Cpu;
    let vocab_path      = "data/vocab.json";
    let config_path     = "models/bible_ternary_v2.0.0.config.json";
    let checkpoint_path = "models/bible_ternary_v2.0.0.safetensors";
    let best_path       = "models/bible_ternary_v2.0.0.best.safetensors";

    let tokenizer = BpeTokenizer::new(vocab_path);
    let tokens    = load_corpus(&tokenizer);

    println!("[{}] Total corpus: {} tokens across all sources", timestamp(), tokens.len());

    let mut evolution_manager = EvolutionManager::new();
    let mut global_step       = 0_usize;

    loop {
        let needs_evolution = train_cycle(
            &tokens, &tokenizer, &device, &mut evolution_manager, &mut global_step
        )?;
        if needs_evolution {
            perform_surgery(config_path, checkpoint_path, best_path, &device)?;
            global_step = 0;
        }
    }
}
