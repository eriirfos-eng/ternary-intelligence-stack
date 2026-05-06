use candle_core::{Device, DType, Result, Tensor};
use candle_nn::{Optimizer, VarBuilder, loss, VarMap};
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use moe_llm_core::evolution::EvolutionManager;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use rayon::ThreadPoolBuilder;
use serde_json::{Value, json};
use std::collections::HashMap;

fn timestamp() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

fn cosine_lr(base_lr: f64, min_lr: f64, step: usize, total_steps: usize) -> f64 {
    let t = step as f64 / total_steps.max(1) as f64;
    min_lr + 0.5 * (base_lr - min_lr) * (1.0 + (std::f64::consts::PI * t).cos())
}

fn perform_surgery(config_path: &str, checkpoint_path: &str, device: &Device) -> Result<()> {
    println!("[{}] --- INITIATING NEURAL SURGERY: Net2Net Safe Copy ---", timestamp());

    let config_str = fs::read_to_string(config_path).expect("Unable to read config.json");
    let mut config_json: Value = serde_json::from_str(&config_str).expect("Invalid JSON in config file.");
    let old_layers = config_json["num_layers"].as_u64().unwrap() as usize;
    let new_layers = old_layers + 1;
    config_json["num_layers"] = json!(new_layers);
    fs::write(config_path, serde_json::to_string_pretty(&config_json).unwrap())?;
    println!("[{}] Evolution: Architecture expanded to {} layers.", timestamp(), new_layers);

    let tensors = candle_core::safetensors::load(checkpoint_path, device)?;
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
    let config_path     = "models/bible_ternary_v2.0.0.config.json";
    let meta_path       = "models/bible_ternary_v2.0.0.meta";
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

    if std::path::Path::new(checkpoint_path).exists() {
        let checkpoint_data = candle_core::safetensors::load(checkpoint_path, device)?;
        let all_vars = varmap.data().lock().unwrap();
        for (name, var) in all_vars.iter() {
            if let Some(tensor) = checkpoint_data.get(name) {
                if tensor.shape() == var.shape() {
                    var.set(tensor)?;
                }
            }
        }
    }

    let mut total_epochs = if let Ok(c) = fs::read_to_string(meta_path) {
        c.trim().parse::<u32>().unwrap_or(0)
    } else { 0 };

    // Cosine LR: starts high, decays to near-zero over lr_cycle_steps global steps
    let base_lr       = 2e-4_f64;
    let min_lr        = 1e-5_f64;
    let lr_cycle_steps = 500_usize;

    let mut opt = candle_nn::AdamW::new_lr(varmap.all_vars(), base_lr)?;

    let seq_len    = config.max_seq_len;
    let num_batches = 300_usize;

    // Write arch metadata once per train_cycle so the dashboard can display it
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = writeln!(f, "ARCH {}L {}H {}E {}CTX {}V",
            config.num_layers, config.hidden_size, config.num_experts,
            config.max_seq_len, config.vocab_size);
    }

    loop {
        let mut total_loss = 0.0_f32;
        total_epochs += 1;

        // Open log file ONCE per epoch, close when epoch ends
        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .ok();

        let epoch_start = Instant::now();

        for batch_idx in 0..num_batches {
            let batch_start = Instant::now();

            // Update LR on cosine schedule
            let lr = cosine_lr(base_lr, min_lr, *global_step % lr_cycle_steps, lr_cycle_steps);
            opt.set_learning_rate(lr);

            // Single forward pass, single backward step — correct gradient descent
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

            // L1 sparsity reward: push weights toward 0 so ternary zeroing is strategic,
            // not random. Lambda small enough not to compete with CE loss.
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
            opt.backward_step(&batch_loss)?;

            let real_loss  = ce_loss.to_scalar::<f32>()?;
            total_loss    += real_loss;
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
                let _ = f.flush();
            }

            *global_step += 1;
        }

        let avg_loss  = total_loss / num_batches as f32;
        let epoch_s   = epoch_start.elapsed().as_secs();
        let summary = format!("=== Epoch {}L done | Avg Loss: {:.4} | {:02}:{:02} elapsed ===",
            config.num_layers, avg_loss, epoch_s / 60, epoch_s % 60);
        println!("[{}] {}", timestamp(), summary);

        // Write epoch summary to log so dashboard reference line can parse it
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = writeln!(f, "{}", summary);
        }

        evolution_manager.add_loss(avg_loss);

        // Checkpoint
        {
            let all_vars = varmap.data().lock().unwrap();
            let mut tensor_map = HashMap::new();
            for (name, var) in all_vars.iter() {
                tensor_map.insert(name.clone(), var.as_tensor().clone());
            }
            candle_core::safetensors::save(&tensor_map, checkpoint_path)?;
        }
        fs::write(meta_path, total_epochs.to_string())?;

        if evolution_manager.should_evolve(config.num_layers) {
            evolution_manager.reset_history();
            return Ok(true);
        }
    }
}

fn load_corpus(tokenizer: &BpeTokenizer) -> Vec<u32> {
    // Load every text file we have and concatenate into one token stream.
    // Order: Bible (primary), then any extras found in data/corpus/.
    let corpus_dir = "data/corpus";
    let mut all_text = String::new();

    // Deterministic order: sort filenames
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
    println!("--- ALBERT EVOLUTIONARY ORCHESTRATOR v2.3 ---");

    let device      = Device::Cpu;
    let vocab_path  = "data/vocab.json";
    let config_path = "models/bible_ternary_v2.0.0.config.json";
    let checkpoint_path = "models/bible_ternary_v2.0.0.safetensors";

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
            perform_surgery(config_path, checkpoint_path, &device)?;
            global_step = 0;  // restart cosine LR at base_lr after surgery
        }
    }
}
