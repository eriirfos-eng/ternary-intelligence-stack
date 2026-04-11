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
// This keeps it self-contained and dependency-light for now.  A proper
// GGUF-Q-ternary type can be registered once llama.cpp upstream ternary quant
// lands, at which point we swap bincode → GGUF writer here.

use std::io::{Read, Write};
use std::path::Path;
use anyhow::Result;
use crate::model::TernModel;

// ─── .tern writer / reader ────────────────────────────────────────────────────

/// Write a TernModel to a `.tern` file (bincode format).
pub fn write_tern<W: Write>(writer: &mut W, model: &TernModel) -> Result<()> {
    let bytes = bincode::serialize(model)?;
    writer.write_all(&bytes)?;
    Ok(())
}

/// Read a TernModel from a `.tern` file.
pub fn read_tern<R: Read>(reader: &mut R) -> Result<TernModel> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    let model = bincode::deserialize(&bytes)?;
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
    // Dequantization note: GGUF supports Q4_0, Q4_1, Q8_0, F16, F32, etc.
    // We dequant to f32 first, then re-quantize to ternary.
    // Direct F16→ternary (without f32 intermediate) would reduce peak memory —
    // worth implementing once the pipeline is validated on Llama-3.2-1B.
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
    hidden:   usize,
    seed:     u64,
) -> Vec<(String, Vec<f32>, Vec<usize>)> {
    // Simple deterministic LCG for reproducible tests (no rand dep needed)
    let mut state = seed.wrapping_add(1);
    let mut next_f32 = move || -> f32 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let bits = (state >> 33) as u32;
        // Map to [-2.0, 2.0] to get a realistic weight distribution
        (bits as f32 / u32::MAX as f32) * 4.0 - 2.0
    };

    (0..n_layers).map(|i| {
        let name    = format!("model.layers.{i}.mlp.weight");
        let weights: Vec<f32> = (0..hidden * hidden).map(|_| next_f32()).collect();
        let shape   = vec![hidden, hidden];
        (name, weights, shape)
    }).collect()
}
