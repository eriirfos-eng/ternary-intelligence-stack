// ternlang-ml/src/bin/inference.rs
// Verifies TritTransformer inference on the 1.2B ternary model.

use ternlang_ml::coherence::ModelCoherence;
use ternlang_ml::{TritTransformer, TritTransformerConfig};
use std::time::Instant;

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- TIS Ternary Inference Runner ---");

    let model_path = Path::new("/home/eri-irfos/Desktop/Ternary Intelligence Stack (TIS)/llama32-1b.tern.bin");
    
    println!("Loading Binary Model: {:?}...", model_path);
    let start = Instant::now();
    let coherence = ModelCoherence::load_bin(model_path)?;
    println!("Loaded in {:?}.", start.elapsed());

    let config = TritTransformerConfig {
        dim: 2048,
        n_layers: 16,
        n_heads: 32,
        n_kv_heads: 8,
        vocab_size: 128256,
        multiple_of: 256,
        ffn_dim_multiplier: None,
        norm_eps: 1e-5,
        max_seq_len: 2048,
    };

    let model = TritTransformer::from_coherence(coherence, config);
    println!("Model initialized.");

    // Sample: Start token (<s> is usually 128000 or 1 in Llama-3)
    let start_token = 128000;
    println!("Performing forward pass for token {} at pos 0...", start_token);
    
    let start = Instant::now();
    let logits = model.forward(start_token, 0);
    println!("Forward pass completed in {:?}.", start.elapsed());

    // Find top-5 tokens
    let mut indexed_logits: Vec<(usize, f32)> = logits.into_iter().enumerate().collect();
    indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("\nTop 5 predicted tokens:");
    for i in 0..5 {
        println!("  {}. Token ID: {:>6} | Logit: {:.4}", i + 1, indexed_logits[i].0, indexed_logits[i].1);
    }

    Ok(())
}
