use candle_core::{Device, DType, Result, Tensor};
use candle_nn::{Optimizer, VarMap, VarBuilder, loss};
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use std::fs;

fn main() -> Result<()> {
    println!("--- Training moe-llm-core on Bible corpus ---");

    // 1. Setup Device
    let device = Device::Cpu;

    // 2. Load Tokenizer
    let tokenizer = BpeTokenizer::new("albert-moe-13/data/vocab.json");
    let vocab_size = tokenizer.vocab_size();
    println!("Vocab size: {}", vocab_size);

    // 3. Load Data
    let corpus_path = "albert-moe-13/data/corpus/bible.txt";
    let text = fs::read_to_string(corpus_path).expect("Unable to read bible.txt");
    let tokens = tokenizer.encode(&text);
    println!("Total tokens: {}", tokens.len());

    // 4. Model Config
    // 4. Model Config
    let mut config = TransformerConfig::default();
    config.vocab_size = vocab_size;
    config.hidden_size = 96; // Increased capacity
    config.num_layers = 3;   // Increased depth
    config.num_heads = 4;
    config.max_seq_len = 128; // Doubled context

    // 5. Initialize Model
    let varmap = VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);
    let model = Transformer::new(&config, vb)?;

    // 6. Optimizer
    let mut opt = candle_nn::AdamW::new_lr(varmap.all_vars(), 2e-4)?; 

    // 7. Training Loop
    let batch_size = 4;
    let seq_len = config.max_seq_len;
    let epochs = 50; 

    for epoch in 0..epochs {
        let mut total_loss = 0.0;
        let num_batches = 300; 

        for batch_idx in 0..num_batches {
            // ... (rest of sampling)
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

            // Backward & Step
            opt.backward_step(&loss)?;

            total_loss += loss.to_scalar::<f32>()?;
            
            if batch_idx % 20 == 0 {
                println!("Epoch {}, Batch {}: loss = {:.4}", epoch, batch_idx, loss.to_scalar::<f32>()?);
            }
        }
        println!("Epoch {} complete. Avg loss: {:.4}", epoch, total_loss / num_batches as f32);
        varmap.save("albert-moe-13/models/bible_ternary_v1.3.7.safetensors")?;
        println!("Checkpoint saved.");
    }

    println!("--- Training Finished ---");

    // 8. Save Weights
    varmap.save("albert-moe-13/models/bible_ternary_v1.3.7.safetensors")?;
    println!("Weights saved to albert-moe-13/models/bible_ternary_v1.3.7.safetensors");

    Ok(())
}
