use candle_core::{Device, DType, Result, Tensor};
use candle_nn::{Optimizer, VarMap, VarBuilder, loss};
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use std::fs::{self, OpenOptions};
use std::io::Write;
use rayon::ThreadPoolBuilder;

fn main() -> Result<()> {
    // Initialize Rayon thread pool to use all 8 cores
    let _ = ThreadPoolBuilder::new().num_threads(8).build_global();
    println!("--- Training moe-llm-core on Bible corpus ---");

    // 1. Setup Device
    let device = Device::Cpu;

    // 2. Load Tokenizer
    let tokenizer = BpeTokenizer::new("/home/eri-irfos/projects/ternary-intelligence-stack/albert-moe-13/data/vocab.json");
    let vocab_size = tokenizer.vocab_size();
    println!("Vocab size: {}", vocab_size);

    // 3. Load Data
    let corpus_path = "/home/eri-irfos/projects/ternary-intelligence-stack/albert-moe-13/data/corpus/bible.txt";
    let text = fs::read_to_string(corpus_path).expect("Unable to read bible.txt");
    let tokens = tokenizer.encode(&text);
    println!("Total tokens: {}", tokens.len());

    // 4. Model Config
    let mut config = TransformerConfig::default();
    config.vocab_size = vocab_size;
    config.hidden_size = 64; 
    config.num_layers = 2;   
    config.num_heads = 4;
    config.max_seq_len = 64; 
    config.num_experts = 0; 

    // 5. Initialize Model
    let mut varmap = VarMap::new();
    let checkpoint_path = "/home/eri-irfos/projects/ternary-intelligence-stack/albert-moe-13/models/bible_ternary_v1.3.6.safetensors";
    let meta_path = "/home/eri-irfos/projects/ternary-intelligence-stack/albert-moe-13/models/bible_ternary_v1.3.6.meta";
    
    // Create the model structure first so the VarMap has the correct keys
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = Transformer::new(&config, vb)?;

    if std::path::Path::new(checkpoint_path).exists() {
        println!("Resuming from checkpoint: {}", checkpoint_path);
        varmap.load(checkpoint_path).expect("Failed to load weights");
    } else {
        println!("No checkpoint found at {}. Starting from scratch.", checkpoint_path);
    }

    // 5.5 Metadata Odometer
    let mut total_epochs = if let Ok(c) = fs::read_to_string(meta_path) {
        c.trim().parse::<u32>().unwrap_or(0)
    } else { 0 };
    println!("Model Odometer: {} total epochs trained", total_epochs);

    // 6. Optimizer
    let mut opt = candle_nn::AdamW::new_lr(varmap.all_vars(), 2e-4)?; 

    // 6.5 Logging Setup
    let desktop_log_path = "/home/eri-irfos/Desktop/training_log/training.log";
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(desktop_log_path)
        .ok();

    // 7. Training Loop
    let batch_size = 1; 
    let accumulation_steps = 8; // Increased for extra memory headroom
    let seq_len = config.max_seq_len;
    let session_epochs = 50; 

    for epoch in 0..session_epochs {
        let mut total_loss = 0.0;
        let num_batches = 300; 
        
        total_epochs += 1; // Increment global odometer

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

                let input_tensor = Tensor::new(batch_inputs, &device)?.reshape((batch_size, seq_len))?;
                let target_tensor = Tensor::new(batch_targets, &device)?.reshape((batch_size, seq_len))?;

                // Forward
                let logits = model.forward(&input_tensor)?;
                
                // Loss
                let logits = logits.reshape((batch_size * seq_len, vocab_size))?;
                let target_tensor = target_tensor.flatten_all()?;
                let loss = loss::cross_entropy(&logits, &target_tensor)?;

                // Scale loss for accumulation
                let loss = (loss / accumulation_steps as f64)?;
                
                // Backward & Step
                opt.backward_step(&loss)?;

                step_loss += loss.to_scalar::<f32>()?;
            }

            total_loss += step_loss;
            
            if batch_idx % 20 == 0 {
                let log_line = format!("Epoch {} (Global {}), Batch {}: loss = {:.4}", epoch, total_epochs, batch_idx, step_loss * accumulation_steps as f32);
                println!("{}", log_line);
                if let Some(ref mut f) = log_file {
                    let _ = writeln!(f, "{}", log_line);
                }
            }
        }
        let end_epoch_line = format!("Epoch {} complete. Avg loss: {:.4}", epoch, total_loss / num_batches as f32);
        println!("{}", end_epoch_line);
        if let Some(ref mut f) = log_file {
            let _ = writeln!(f, "{}", end_epoch_line);
        }
        
        // Atomic Save: Weights
        let tmp_path = format!("{}.tmp", checkpoint_path);
        varmap.save(&tmp_path)?;
        std::fs::rename(&tmp_path, checkpoint_path)?;
        
        // Atomic Save: Odometer
        fs::write(meta_path, total_epochs.to_string())?;
        
        println!("Checkpoint saved. Odometer: {}", total_epochs);
    }

    println!("--- Training Finished ---");

    // Final Weight Save
    let tmp_path = format!("{}.tmp", checkpoint_path);
    varmap.save(&tmp_path)?;
    std::fs::rename(&tmp_path, checkpoint_path)?;
    println!("Final weights stabilized at {}", checkpoint_path);

    Ok(())
}
