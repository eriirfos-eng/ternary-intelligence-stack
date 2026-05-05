use candle_core::{Device, DType, Result, Tensor};
use candle_nn::{Optimizer, VarBuilder, loss, VarMap};
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use std::fs::{self, OpenOptions};
use std::io::Write;
use rayon::ThreadPoolBuilder;
use serde_json::{Value, json};
use std::collections::{VecDeque, HashMap};

struct EvolutionManager {
    loss_history: VecDeque<f32>,
    history_len: usize,
    plateau_threshold: f32,
    mastery_threshold: f32,
    max_layers: usize,
}

impl EvolutionManager {
    fn new() -> Self {
        Self {
            loss_history: VecDeque::with_capacity(5),
            history_len: 5,
            plateau_threshold: 0.05,
            mastery_threshold: 4.5, 
            max_layers: 12,
        }
    }

    fn add_loss(&mut self, loss: f32) {
        if self.loss_history.len() >= self.history_len {
            self.loss_history.pop_front();
        }
        self.loss_history.push_back(loss);
    }

    fn should_evolve(&self, current_layers: usize) -> bool {
        if current_layers >= self.max_layers { return false; }
        if self.loss_history.len() < self.history_len { return false; }

        let latest = *self.loss_history.back().unwrap();
        
        if latest < self.mastery_threshold {
            println!("--- MASTERY EVOLUTION TRIGGERED (Loss {:.4} < {:.4}) ---", latest, self.mastery_threshold);
            return true;
        }

        let first = *self.loss_history.front().unwrap();
        let diff = (first - latest).abs();
        if diff < self.plateau_threshold {
            println!("--- PLATEAU EVOLUTION TRIGGERED (Improvement {:.4} < {:.4}) ---", diff, self.plateau_threshold);
            return true;
        }

        false
    }
    
    fn reset_history(&mut self) {
        self.loss_history.clear();
    }
}

fn perform_surgery(config_path: &str, checkpoint_path: &str, device: &Device) -> Result<()> {
    println!("--- INITIATING NEURAL SURGERY: Net2Net Safe Copy ---");
    
    // 1. Update Config
    let config_str = fs::read_to_string(config_path).expect("Unable to read config.json");
    let mut config_json: Value = serde_json::from_str(&config_str).expect("Invalid JSON in config file.");
    let old_layers = config_json["num_layers"].as_u64().unwrap() as usize;
    let new_layers = old_layers + 1;
    config_json["num_layers"] = json!(new_layers);
    fs::write(config_path, serde_json::to_string_pretty(&config_json).unwrap())?;
    println!("Evolution: Architecture expanded to {} layers.", new_layers);

    // 2. Weight Surgery
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
    println!("Surgery Complete: Layer {} is now a safe clone of Layer {}.", target_layer, source_layer);
    Ok(())
}

fn train_cycle(
    tokens: &[u32], 
    tokenizer: &BpeTokenizer, 
    device: &Device, 
    evolution_manager: &mut EvolutionManager
) -> Result<bool> {
    let checkpoint_path = "models/bible_ternary_v2.0.0.safetensors";
    let config_path = "models/bible_ternary_v2.0.0.config.json";
    let meta_path = "models/bible_ternary_v2.0.0.meta";
    let log_path = "dashboard/training.log";

    let config_str = fs::read_to_string(config_path).expect("Unable to read config.json");
    let config_json: Value = serde_json::from_str(&config_str).expect("Invalid JSON in config file.");

    let mut config = TransformerConfig::default();
    config.vocab_size = tokenizer.vocab_size();
    config.hidden_size = config_json["hidden_size"].as_u64().unwrap() as usize;
    config.num_layers = config_json["num_layers"].as_u64().unwrap() as usize;
    config.num_heads = config_json["num_heads"].as_u64().unwrap() as usize;
    config.max_seq_len = config_json["max_seq_len"].as_u64().unwrap() as usize;
    config.num_experts = config_json["num_experts"].as_u64().unwrap() as usize;

    println!("Training Engine Loaded: {} layers, {} hidden", config.num_layers, config.hidden_size);

    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, device);
    let model = Transformer::new(&config, vb)?;

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

    let mut opt = candle_nn::AdamW::new_lr(varmap.all_vars(), 2e-4)?; 

    let batch_size = 1; 
    let accumulation_steps = 16;
    let seq_len = config.max_seq_len;
    
    loop {
        let mut total_loss = 0.0;
        let num_batches = 300; 
        total_epochs += 1;

        for batch_idx in 0..num_batches {
            let mut step_loss = 0.0;
            for _ in 0..accumulation_steps {
                let start = rand::random::<usize>() % (tokens.len() - seq_len - 1);
                let input_tokens = &tokens[start..start + seq_len];
                let target_tokens = &tokens[start + 1..start + seq_len + 1];
                
                let input_tensor = Tensor::new(input_tokens, device)?.reshape((batch_size, seq_len))?.to_dtype(DType::U32)?;
                let target_tensor = Tensor::new(target_tokens, device)?.reshape((batch_size, seq_len))?.to_dtype(DType::U32)?;

                let logits = model.forward(&input_tensor)?;
                let logits = logits.reshape((batch_size * seq_len, config.vocab_size))?;
                let target_tensor = target_tensor.flatten_all()?;
                
                let loss = loss::cross_entropy(&logits, &target_tensor)?;
                let loss = (loss / accumulation_steps as f64)?;
                opt.backward_step(&loss)?;
                step_loss += loss.to_scalar::<f32>()?;
            }

            total_loss += step_loss;
            
            // HIGH-FREQUENCY LOGGING: Every single batch for instant feedback
            let current_loss = step_loss * accumulation_steps as f32;
            let log_line = format!("Epoch {} (Global {}), Batch {}: loss = {:.4}", config.num_layers, total_epochs, batch_idx, current_loss);
            println!("{}", log_line);
            
            // CRITICAL: Open, Write, and Flush immediately to disk
            if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(log_path) {
                let _ = writeln!(f, "{}", log_line);
                let _ = f.flush(); // Ensure the OS commits the bytes now
            }
        }
        
        let avg_loss = total_loss / num_batches as f32;
        println!("Cycle Stage Complete. Avg loss: {:.4}", avg_loss);
        evolution_manager.add_loss(avg_loss);

        // Save progress
        {
            let all_vars = varmap.data().lock().unwrap();
            let mut tensor_map = HashMap::new();
            for (name, var) in all_vars.iter() {
                tensor_map.insert(name.clone(), var.as_tensor().clone());
            }
            candle_core::safetensors::save(&tensor_map, checkpoint_path)?;
        }
        fs::write(meta_path, total_epochs.to_string())?;

        // Evolution Check
        if evolution_manager.should_evolve(config.num_layers) {
            evolution_manager.reset_history();
            return Ok(true); 
        }
    }
}

fn main() -> Result<()> {
    let _ = ThreadPoolBuilder::new().num_threads(8).build_global();
    println!("--- ALBERT EVOLUTIONARY ORCHESTRATOR v2.2 ---");

    let device = Device::Cpu;
    let vocab_path = "data/vocab.json";
    let corpus_path = "data/corpus/bible.txt";
    let config_path = "models/bible_ternary_v2.0.0.config.json";
    let checkpoint_path = "models/bible_ternary_v2.0.0.safetensors";

    let tokenizer = BpeTokenizer::new(vocab_path);
    let text = fs::read_to_string(corpus_path).expect("Unable to read corpus.txt");
    let tokens = tokenizer.encode(&text);

    let mut evolution_manager = EvolutionManager::new();

    loop {
        let needs_evolution = train_cycle(&tokens, &tokenizer, &device, &mut evolution_manager)?;
        if needs_evolution {
            perform_surgery(config_path, checkpoint_path, &device)?;
        }
    }
}