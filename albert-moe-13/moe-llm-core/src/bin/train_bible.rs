use candle_core::{Device, DType, Result, Tensor};
use candle_nn::{Optimizer, VarMap, VarBuilder, loss};
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use std::fs::{self, OpenOptions};
use std::io::Write;
use rayon::ThreadPoolBuilder;
use serde_json::Value;

fn main() -> Result<()> {
    let _ = ThreadPoolBuilder::new().num_threads(8).build_global();
    println!("--- Training moe-llm-core on Bible corpus ---");

    let device = Device::Cpu;

    let vocab_path = "data/vocab.json";
    let corpus_path = "data/corpus/bible.txt";
    let checkpoint_path = "models/bible_ternary_v2.0.0.safetensors";
    let meta_path = "models/bible_ternary_v2.0.0.meta";
    let config_path = "models/bible_ternary_v2.0.0.config.json";
    let log_path = "dashboard/training.log";

    if !std::path::Path::new(vocab_path).exists() {
        panic!("Vocab file not found at {}. Are you running from the project root?", vocab_path);
    }
    let tokenizer = BpeTokenizer::new(vocab_path);
    let vocab_size = tokenizer.vocab_size();
    println!("Vocab size: {}", vocab_size);

    let text = fs::read_to_string(corpus_path).expect("Unable to read corpus.txt");
    let tokens = tokenizer.encode(&text);
    println!("Total tokens: {}", tokens.len());

    let config_str = fs::read_to_string(config_path).expect("Unable to read config.json. The HuggingFace standard requires a config file next to the model.");
    let config_json: Value = serde_json::from_str(&config_str).expect("Invalid JSON in config file.");

    let mut config = TransformerConfig::default();
    config.vocab_size = vocab_size;
    config.hidden_size = config_json["hidden_size"].as_u64().unwrap() as usize;
    config.num_layers = config_json["num_layers"].as_u64().unwrap() as usize;
    config.num_heads = config_json["num_heads"].as_u64().unwrap() as usize;
    config.max_seq_len = config_json["max_seq_len"].as_u64().unwrap() as usize;
    config.num_experts = config_json["num_experts"].as_u64().unwrap() as usize;

    println!("Loaded Architecture: {} layers, {} hidden, {} experts", config.num_layers, config.hidden_size, config.num_experts);

    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = Transformer::new(&config, vb)?;

    if std::path::Path::new(checkpoint_path).exists() {
        println!("Resuming from checkpoint: {}", checkpoint_path);
        let checkpoint_data = candle_core::safetensors::load(checkpoint_path, &device)?;
        let all_vars = varmap.data().lock().unwrap();
        
        let mut loaded_count = 0;
        let mut missing_count = 0;

        for (name, var) in all_vars.iter() {
            if let Some(tensor) = checkpoint_data.get(name) {
                if tensor.shape() != var.shape() {
                    panic!("SHAPE MISMATCH FATAL ERROR! Tensor {} expects shape {:?}, but checkpoint has shape {:?}. Config and weights are out of sync!", name, var.shape(), tensor.shape());
                }
                var.set(tensor)?;
                loaded_count += 1;
            } else {
                missing_count += 1;
            }
        }
        
        if missing_count > 5 {
            panic!("TOO MANY MISSING TENSORS. Expected architecture does not match checkpoint!");
        }
        
        println!("Full load complete: {} tensors restored.", loaded_count);
    } else {
        println!("No checkpoint found at {}. Starting from scratch.", checkpoint_path);
    }

    let mut total_epochs = if let Ok(c) = fs::read_to_string(meta_path) {
        c.trim().parse::<u32>().unwrap_or(0)
    } else { 0 };
    println!("Model Odometer: {} total epochs trained", total_epochs);

    let mut opt = candle_nn::AdamW::new_lr(varmap.all_vars(), 2e-4)?; 

    fs::create_dir_all("dashboard").unwrap_or(());

    let batch_size = 1; 
    let accumulation_steps = 16;
    let seq_len = config.max_seq_len;
    let session_epochs = 200; 

    for epoch in 0..session_epochs {
        let mut total_loss = 0.0;
        let num_batches = 300; 
        
        total_epochs += 1;

        for batch_idx in 0..num_batches {
            let mut step_loss = 0.0;

            for _ in 0..accumulation_steps {
                let mut batch_inputs = Vec::new();
                let mut batch_targets = Vec::new();

                for _ in 0..batch_size {
                    let start = rand::random::<usize>() % (tokens.len() - seq_len - 1);
                    let input_tokens = &tokens[start..start + seq_len];
                    let target_tokens = &tokens[start + 1..start + seq_len + 1];
                    batch_inputs.extend_from_slice(input_tokens);
                    batch_targets.extend_from_slice(target_tokens);
                }

                let input_tensor = Tensor::new(&batch_inputs[..], &device)?.reshape((batch_size, seq_len))?.to_dtype(DType::U32)?;
                let target_tensor = Tensor::new(&batch_targets[..], &device)?.reshape((batch_size, seq_len))?.to_dtype(DType::U32)?;

                let logits = model.forward(&input_tensor)?;
                let logits = logits.reshape((batch_size * seq_len, config.vocab_size))?;
                let target_tensor = target_tensor.flatten_all()?;
                
                let loss = loss::cross_entropy(&logits, &target_tensor)?;
                let loss = (loss / accumulation_steps as f64)?;
                
                opt.backward_step(&loss)?;

                step_loss += loss.to_scalar::<f32>()?;
            }

            total_loss += step_loss;
            
            // INCREASED LOGGING FREQUENCY: Every 10 batches
            if batch_idx % 10 == 0 {
                let log_line = format!("Epoch {} (Global {}), Batch {}: loss = {:.4}", epoch, total_epochs, batch_idx, step_loss * accumulation_steps as f32);
                println!("{}", log_line);
                
                // ROBUST LOGGING: Open, write, and close every time.
                // This survives file system changes (git pull, manual deletes, etc.)
                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
                    let _ = writeln!(f, "{}", log_line);
                    let _ = f.flush();
                }
            }
        }
        let end_epoch_line = format!("Epoch {} complete. Avg loss: {:.4}", epoch, total_loss / num_batches as f32);
        println!("{}", end_epoch_line);
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
            let _ = writeln!(f, "{}", end_epoch_line);
            let _ = f.flush();
        }
        
        let tmp_path = format!("{}.tmp", checkpoint_path);
        varmap.save(&tmp_path)?;
        std::fs::rename(&tmp_path, checkpoint_path)?;
        
        fs::write(meta_path, total_epochs.to_string())?;
        
        println!("Checkpoint saved. Odometer: {}", total_epochs);
    }

    println!("--- Training Finished ---");

    let tmp_path = format!("{}.tmp", checkpoint_path);
    varmap.save(&tmp_path)?;
    std::fs::rename(&tmp_path, checkpoint_path)?;
    println!("Final weights stabilized at {}", checkpoint_path);

    Ok(())
}
