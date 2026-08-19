use crate::{TritMatrix, sparse_matmul};
use ternlang_core::Trit;
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

    /// Get a scalar weight value at (layer_index, sub_layer, weight_index).
    /// Sub-layer 0 = w1 (input→hidden), 1 = w2 (hidden→output).
    /// Returns 0.0 for out-of-bounds or if the weight is Tend (zero-state).
    pub fn get_weight(&self, layer_idx: usize, sub_layer: usize, weight_idx: usize) -> f64 {
        if layer_idx >= self.layers.len() {
            return 0.0;
        }
        let layer = &self.layers[layer_idx];
        layer.get_weight(sub_layer, weight_idx)
    }
}

impl Layer {
    pub fn to_trit_matrix(&self) -> TritMatrix {
        unpack_layer(self)
    }

    /// Get a scalar weight value at (sub_layer, weight_index).
    /// Returns 0.0 for out-of-bounds.
    pub fn get_weight(&self, sub_layer: usize, weight_idx: usize) -> f64 {
        match &self.storage {
            Storage::Dense(packed) => {
                let idx = if sub_layer == 0 { weight_idx } else { packed.rows * packed.cols / 2 + weight_idx };
                if idx < packed.packed.len() {
                    let byte = packed.packed[idx];
                    // Unpack from 5-trit packed representation
                    let trit_val = if byte >= 128 { 1 } else if byte <= 1 { -1 } else { 0 };
                    trit_val as f64
                } else {
                    0.0
                }
            }
        }
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
    
    // Create a mock input: [1 x in_features] with all Affirm (+1) inputs.
    let data = vec![Trit::Affirm; w.rows];
    let mut input = TritMatrix::new(1, w.rows, data);
    for i in 0..w.rows {
        input.set(0, i, Trit::Affirm);
    }

    println!("Running sparse_matmul (Forward Pass)...");
    let output = sparse_matmul(&input, &w).0;

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
    use ternlang_core::Trit;

    // Synthetic coherence test — no external model file required.
    // Constructs a small 4×4 ternary layer in-memory and verifies that
    // unpack_layer + sparse_matmul produce a non-collapsed output signal.
    // This is the same logic exercised by the one-off sparseskip benchmark
    // that ran against a quantised third-party checkpoint (llama32-1b); that
    // experiment is documented in BENCHMARKS.md §F1 and TERNARY_FINDINGS.md.
    #[test]
    fn test_coherence_synthetic() {
        // Build a 4×4 packed layer: alternating Affirm(10)/Reject(01) pattern.
        // Two trits per two bits, four per byte → 16 trits need 4 bytes.
        // Encoding: Affirm=0b10, Reject=0b01, Tend=0b11
        // Byte layout (bits 7..0): trit3|trit2|trit1|trit0 (2 bits each)
        // Alternating: Affirm(10) Reject(01) Affirm(10) Reject(01) → 0b_01_10_01_10 = 0x6A
        let packed: Vec<u8> = vec![0x6A, 0x6A, 0x6A, 0x6A];
        let layer = Layer {
            name: "synthetic.test.weight".to_string(),
            scale: 1.0,
            sparsity: 0.5,
            storage: Storage::Dense(PackedDense { rows: 4, cols: 4, packed }),
        };

        let matrix = unpack_layer(&layer);
        assert_eq!(matrix.rows, 4);
        assert_eq!(matrix.cols, 4);

        // All-Affirm input → forward pass
        let mut input = TritMatrix::zeros(1, matrix.rows);
        for i in 0..matrix.rows {
            input.set(0, i, Trit::Affirm);
        }

        let (output, _skipped) = sparse_matmul(&input, &matrix);
        assert_eq!(output.rows, 1);
        assert_eq!(output.cols, 4);

        // Signal must not be fully collapsed (not all Tend)
        let non_zeros = output.data.iter().filter(|&&t| t != Trit::Tend).count();
        assert!(non_zeros > 0, "coherence check: output signal fully collapsed");
    }
}
