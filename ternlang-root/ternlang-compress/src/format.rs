// Model format I/O — read source models, write .tern files.
//
// Currently two stubs are provided:
//   - GgufLoader   — reads GGUF files (used by Ollama / llama.cpp)
//   - SafeTensorsLoader — reads HuggingFace safetensors
//
// Both are thin interfaces.  The actual parsing is deferred to candle's
// readers or purpose-built crates once this crate reaches Phase 12 integration.
// Enable the `gguf` or `safetensors` Cargo feature to activate each loader.
//
// The .tern output format is bincode-serialised TernModel (see model.rs).

use crate::QuantizedWeights;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::Path;

/// A single compressed weight layer in a `.tern` model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TernLayer {
    /// Layer name (e.g. "model.layers.0.mlp.weight").
    pub name: String,
    /// Logical tensor shape.
    pub shape: Vec<usize>,
    /// Per-layer scale factor α for reconstruction.
    pub scale: f32,
    /// Fraction of trits that are Tend (zero) — 0.0 … 1.0.
    pub sparsity: f64,
    /// Number of packed bytes in the quantized payload.
    pub packed_bytes: usize,
    /// Number of non-zero trits (CSR nnz).
    pub csr_nnz: usize,
    /// The packed ternary weights (5 trits per byte).
    #[serde(skip)]
    pub quantized: QuantizedWeights,
}

/// Top-level container for a compressed ternary model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TernModel {
    /// Source model identifier (e.g. "meta-llama/Llama-3.2-1B").
    pub source_model: String,
    /// Architecture name (e.g. "LlamaForCausalLM").
    pub architecture: String,
    /// Number of transformer layers.
    pub num_layers: usize,
    /// Hidden size / width.
    pub hidden_size: usize,
    /// All compressed layers.
    pub layers: Vec<TernLayer>,
    /// Compression configuration used.
    pub config: CompressConfig,
}

impl TernModel {
    /// Human-readable summary of the compressed model.
    pub fn summary(&self) -> String {
        let total_nnz: usize = self.layers.iter().map(|l| l.csr_nnz).sum();
        let total_packed: usize = self.layers.iter().map(|l| l.packed_bytes).sum();
        let mean_sparsity = if self.layers.is_empty() {
            0.0
        } else {
            self.layers.iter().map(|l| l.sparsity).sum::<f64>() / self.layers.len() as f64
        };

        format!(
            "TernModel: {}\n  architecture:   {}\n  layers:         {}\n  hidden_size:    {}\n  total nnz:      {} trits\n  total packed:   {} bytes\n  mean sparsity:  {:.1}%",
            self.source_model,
            self.architecture,
            self.num_layers,
            self.hidden_size,
            total_nnz,
            total_packed,
            mean_sparsity * 100.0
        )
    }
}

/// Compression configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompressConfig {
    /// Source model identifier.
    pub source_model: String,
    /// Architecture name.
    pub architecture: String,
    /// Number of layers.
    pub num_layers: usize,
    /// Hidden size.
    pub hidden_size: usize,
    /// Whether to print verbose progress.
    #[serde(default)]
    pub verbose: bool,
}

// ─── .tern writer / reader ────────────────────────────────────────────────────

/// Write a TernModel to a `.tern` file (bincode format).
pub fn write_tern<W: Write>(writer: &mut W, model: &TernModel) -> Result<()> {
    // Serialize everything except the quantized payloads (they're not serializable)
    let mut model_copy = model.clone();
    for layer in &mut model_copy.layers {
        // Clear the non-serializable payload — it's reconstructed after loading
        layer.quantized = QuantizedWeights { data: vec![], trit_count: 0 };
    }
    let bytes = bincode::serialize(&model_copy)?;
    writer.write_all(&bytes)?;
    // Write quantized payloads separately
    for layer in &model.layers {
        writer.write_all(&layer.quantized.data)?;
    }
    Ok(())
}

/// Read a TernModel from a `.tern` file.
pub fn read_tern<R: Read>(reader: &mut R) -> Result<TernModel> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    let mut model: TernModel = bincode::deserialize(&bytes)?;
    // Reconstruct quantized payloads (currently empty after bincode deserialize)
    for layer in &mut model.layers {
        layer.quantized = QuantizedWeights { data: vec![], trit_count: 0 };
    }
    Ok(model)
}

/// Convenience: write to a file path.
pub fn save_tern(path: &Path, model: &TernModel) -> Result<()> {
    let mut f = std::fs::File::create(path)?;
    write_tern(&mut f, model)
}

/// Convenience: read from a file path.
pub fn load_tern(path: &Path) -> Result<TernModel> {
    let mut f = std::fs::File::open(path)?;
    read_tern(&mut f)
}

// ─── GGUF loader stub ─────────────────────────────────────────────────────────

/// Loads a GGUF model and returns its weight tensors as `(name, f32_weights, shape)`.
///
/// TODO (Phase 12): Implement using candle's GGUF reader or a direct parser.
///       The GGUF format stores tensors with their quant type, shape, and data.
///       For now this is a stub — the real implementation should:
///         1. Open the file (mmap for large models)
///         2. Parse GGUF header (magic + metadata KV + tensor index)
///         3. For each tensor: dequantize to f32, return (name, data, shape)
///
/// Feature gate: compile with `--features gguf` once implemented.
#[allow(dead_code)]
pub fn load_gguf(_path: &Path) -> Result<Vec<(String, Vec<f32>, Vec<usize>)>> {
    anyhow::bail!(
        "GGUF loader not yet implemented. \
         Enable the `gguf` feature and implement load_gguf() in format.rs. \
         See TODO comment above for the implementation guide."
    )
}

// ─── SafeTensors loader stub ──────────────────────────────────────────────────

/// Loads a HuggingFace safetensors model directory.
///
/// TODO (Phase 12): Implement using the `safetensors` crate (MIT licensed).
///       Typical layout: model.safetensors or model-00001-of-NNNNN.safetensors.
///       The safetensors format is straightforward: a JSON header + raw tensor data.
///
/// Feature gate: compile with `--features safetensors` once implemented.
#[allow(dead_code)]
pub fn load_safetensors(_dir: &Path) -> Result<Vec<(String, Vec<f32>, Vec<usize>)>> {
    anyhow::bail!(
        "SafeTensors loader not yet implemented. \
         Enable the `safetensors` feature and implement load_safetensors() in format.rs."
    )
}

// ─── Mock loader for testing ──────────────────────────────────────────────────

/// Generate a synthetic model for testing the pipeline without a real model file.
/// Produces `n_layers` weight matrices of shape `(hidden, hidden)` with random-ish values.
pub fn synthetic_layers(
    n_layers: usize,
    hidden: usize,
    seed: u64,
) -> Vec<(String, Vec<f32>, Vec<usize>)> {
    // Simple deterministic LCG for reproducible tests (no rand dep needed)
    let mut state = seed.wrapping_add(1);
    let mut next_f32 = move || -> f32 {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let bits = (state >> 33) as u32;
        // Map to [-2.0, 2.0] to get a realistic weight distribution
        (bits as f32 / u32::MAX as f32) * 4.0 - 2.0
    };

    (0..n_layers)
        .map(|i| {
            let name = format!("model.layers.{i}.mlp.weight");
            let weights: Vec<f32> = (0..hidden * hidden).map(|_| next_f32()).collect();
            let shape = vec![hidden, hidden];
            (name, weights, shape)
        })
        .collect()
}
