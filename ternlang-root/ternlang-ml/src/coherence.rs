use crate::{TritMatrix, sparse_matmul};
use ternlang_core::trit::Trit;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct PackedDense {
    pub rows: usize,
    pub cols: usize,
    pub packed: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Storage {
    Dense(PackedDense),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    pub name: String,
    pub scale: f32,
    pub sparsity: f32,
    pub storage: Storage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelCoherence {
    pub source_model: String,
    pub layers: Vec<Layer>,
}

impl ModelCoherence {
    /// Save model to a fast binary format.
    pub fn save_bin(&self, path: &Path) -> anyhow::Result<()> {
        let file = File::create(path)?;
        bincode::serialize_into(file, self)?;
        Ok(())
    }

    /// Load model from the fast binary format.
    pub fn load_bin(path: &Path) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let model: Self = bincode::deserialize_from(reader)?;
        Ok(model)
    }
}

impl Layer {
    pub fn to_trit_matrix(&self) -> TritMatrix {
        unpack_layer(self)
    }
}

pub fn unpack_layer(layer: &Layer) -> TritMatrix {
    match &layer.storage {
        Storage::Dense(dense) => {
            let mut trits = Vec::with_capacity(dense.rows * dense.cols);
            for (_byte_idx, &byte) in dense.packed.iter().enumerate() {
                for bit_idx in 0..4 {
                    if trits.len() >= dense.rows * dense.cols {
                        break;
                    }
                    let bits = (byte >> (bit_idx * 2)) & 0b11;
                    let trit = match bits {
                        0b01 => Trit::Reject,
                        0b11 => Trit::Tend,
                        0b10 => Trit::Affirm,
                        0b00 => Trit::Tend, // Should not happen in fixed script
                        _ => unreachable!(),
                    };
                    trits.push(trit);
                }
            }
            TritMatrix::from_trits(dense.rows, dense.cols, trits)
        }
    }
}

pub fn run_coherence_test(json_path: &Path, target_layer: &str) -> anyhow::Result<()> {
    println!("--- RFI-IRFOS TIS: Coherence Test [Phase 12A] ---");
    println!("Loading model from: {:?}", json_path);
    
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    let model: ModelCoherence = serde_json::from_reader(reader)?;
    
    println!("Model: {}", model.source_model);
    println!("Total layers: {}", model.layers.len());
    
    let layer = model.layers.iter().find(|l| l.name == target_layer)
        .ok_or_else(|| anyhow::anyhow!("Layer {} not found", target_layer))?;
        
    println!("Testing layer: {} (Sparsity: {:.2}%)", layer.name, layer.sparsity * 100.0);
    
    let w = unpack_layer(layer);
    
    // Create a mock input: [1 x in_features]
    // For a forward pass check, we use all Affirm (+1) inputs.
    let mut input = TritMatrix::new(1, w.rows);
    for i in 0..w.rows {
        input.set(0, i, Trit::Affirm);
    }
    
    println!("Running sparse_matmul (Forward Pass POC)...");
    let (output, skipped) = sparse_matmul(&input, &w);
    
    println!("Done.");
    println!("Output shape: {}x{}", output.rows, output.cols);
    println!("Skipped ops:  {} (Sparsity Advantage: {:.2}x)", 
        skipped, (skipped as f64 + output.rows as f64 * output.cols as f64 * w.rows as f64) / (output.rows as f64 * output.cols as f64 * w.rows as f64 - skipped as f64).max(1.0));
    
    // Check signal: how many non-zero trits in output?
    let non_zeros = output.data.iter().filter(|&&t| t != Trit::Tend).count();
    let signal_ratio = non_zeros as f32 / output.data.len() as f32;
    
    println!("Signal Ratio: {:.2}% ({} / {})", signal_ratio * 100.0, non_zeros, output.data.len());
    
    if signal_ratio > 0.05 {
        println!("[SUCCESS] Signal coherence detected. Model is not a void.");
    } else {
        println!("[WARNING] Low signal detected. Model might be overly sparse or collapsed.");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llama_coherence() {
        // CWD during cargo test is crate root: ternlang-root/ternlang-ml/
        // llama32-1b.tern.json is in TIS/ root (../../)
        let json_path = Path::new("../../llama32-1b.tern.json");
        if json_path.exists() {
            run_coherence_test(json_path, "blk.0.ffn_gate.weight").unwrap();
        } else {
            println!("Skipping coherence test: llama32-1b.tern.json not found at {:?}", json_path);
            // Fallback: try one level up just in case
            let fallback = Path::new("../llama32-1b.tern.json");
            if fallback.exists() {
                run_coherence_test(fallback, "blk.0.ffn_gate.weight").unwrap();
            } else {
                panic!("Could not find llama32-1b.tern.json at {:?} or {:?}", json_path, fallback);
            }
        }
    }
}
