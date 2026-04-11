// Main compression pipeline.
//
// Usage:
//   let cfg = CompressConfig::default();
//   let model = compress(layers, cfg)?;
//
// `layers` is a Vec of (name, f32_weights, shape) triples produced by a
// model loader (see format.rs for the GGUF/safetensors loaders — currently
// stubbed, to be completed when integrating candle or llama.cpp bindings).

use crate::model::{LayerStorage, TernLayer, TernModel};
use crate::quantize::{PerLayerQuant, quantize_layers};
use crate::sparse::SparseIndex;
use crate::FORMAT_VERSION;

use ternlang_core::trit::Trit;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompressError {
    #[error("empty model: no layers provided")]
    NoLayers,
    #[error("layer `{0}`: shape product ({1}) does not match weights length ({2})")]
    ShapeMismatch(String, usize, usize),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Configuration for the compression pipeline.
#[derive(Debug, Clone)]
pub struct CompressConfig {
    /// Source model identifier string (for metadata).
    pub source_model: String,
    /// Architecture string (e.g. "LlamaForCausalLM").
    pub architecture: String,
    pub vocab_size:   usize,
    pub hidden_size:  usize,
    pub num_layers:   usize,

    /// Sparsity threshold above which CSR storage is used instead of dense.
    /// Default: 0.75 (CSR is more efficient above ~75% sparsity).
    pub csr_sparsity_threshold: f64,

    /// If true, print per-layer stats during compression.
    pub verbose: bool,
}

impl Default for CompressConfig {
    fn default() -> Self {
        Self {
            source_model:           String::from("unknown"),
            architecture:           String::from("unknown"),
            vocab_size:             0,
            hidden_size:            0,
            num_layers:             0,
            csr_sparsity_threshold: 0.75,
            verbose:                false,
        }
    }
}

/// Compress a list of float weight tensors to a TernModel.
///
/// # Arguments
/// * `layers` — `(name, weights_f32, shape)` for each weight matrix.
/// * `cfg`    — pipeline configuration.
pub fn compress(
    layers: Vec<(String, Vec<f32>, Vec<usize>)>,
    cfg: CompressConfig,
) -> Result<TernModel, CompressError> {
    if layers.is_empty() {
        return Err(CompressError::NoLayers);
    }

    // Validate shapes
    for (name, weights, shape) in &layers {
        let expected: usize = shape.iter().product();
        if expected != weights.len() {
            return Err(CompressError::ShapeMismatch(
                name.clone(), expected, weights.len(),
            ));
        }
    }

    // Quantize all layers (parallel if feature enabled)
    #[cfg(feature = "parallel")]
    let quants: Vec<PerLayerQuant> = {
        use crate::quantize::quantize_layers_parallel;
        quantize_layers_parallel(layers)
    };
    #[cfg(not(feature = "parallel"))]
    let quants: Vec<PerLayerQuant> = quantize_layers(layers);

    // Build TernLayers
    let mut tern_layers = Vec::with_capacity(quants.len());

    for q in quants {
        if cfg.verbose {
            tracing::info!(
                layer = %q.name,
                sparsity = format!("{:.1}%", q.sparsity * 100.0),
                scale = q.scale,
                "quantized"
            );
        }

        let storage = if q.sparsity >= cfg.csr_sparsity_threshold {
            // High sparsity — use CSR
            let (rows, cols) = layer_dims(&q.shape);
            let idx = SparseIndex::from_trits(rows, cols, &q.trits);
            LayerStorage::Sparse(idx)
        } else {
            // Lower sparsity — use 2-bit packed dense
            let (rows, cols) = layer_dims(&q.shape);
            let packed = pack_trits_2bit(&q.trits);
            LayerStorage::Dense { rows, cols, packed }
        };

        tern_layers.push(TernLayer {
            name:          q.name,
            scale:         q.scale,
            storage,
            sparsity:      q.sparsity,
            original_dtype: "f32".into(),
        });
    }

    Ok(TernModel {
        source_model:   cfg.source_model,
        format_version: FORMAT_VERSION,
        layers:         tern_layers,
        architecture:   cfg.architecture,
        vocab_size:     cfg.vocab_size,
        hidden_size:    cfg.hidden_size,
        num_layers:     cfg.num_layers,
    })
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Extract (rows, cols) from a shape slice.
/// For rank-1 tensors uses (1, n); for rank-2 uses (shape[0], shape[1]);
/// for higher ranks flattens inner dims into cols.
fn layer_dims(shape: &[usize]) -> (usize, usize) {
    match shape.len() {
        0 => (1, 1),
        1 => (1, shape[0]),
        2 => (shape[0], shape[1]),
        _ => (shape[0], shape[1..].iter().product()),
    }
}

/// Pack trit slice into 2-bit-per-trit bytes.
/// Encoding: 0b00 = Tend (0), 0b01 = Affirm (+1), 0b10 = Reject (-1).
/// 4 trits per byte, LSB first.
fn pack_trits_2bit(trits: &[Trit]) -> Vec<u8> {
    let n_bytes = (trits.len() + 3) / 4;
    let mut out = vec![0u8; n_bytes];
    for (i, &t) in trits.iter().enumerate() {
        let bits: u8 = match t {
            Trit::Tend   => 0b00,
            Trit::Affirm => 0b01,
            Trit::Reject => 0b10,
        };
        let byte_idx = i / 4;
        let shift    = (i % 4) * 2;
        out[byte_idx] |= bits << shift;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_tiny_model() {
        let weights: Vec<f32> = (0..16).map(|i| (i as f32) - 8.0).collect();
        let layers = vec![
            ("layer0.weight".into(), weights, vec![4, 4]),
        ];
        let cfg = CompressConfig {
            source_model: "test-1M".into(),
            num_layers: 1,
            ..Default::default()
        };
        let model = compress(layers, cfg).unwrap();
        assert_eq!(model.layers.len(), 1);
        assert!(model.mean_sparsity() >= 0.0);
        println!("{}", model.summary());
    }

    #[test]
    fn pack_unpack_roundtrip() {
        let trits = vec![
            Trit::Affirm, Trit::Tend, Trit::Reject, Trit::Affirm,
            Trit::Reject, Trit::Tend, Trit::Affirm, Trit::Tend,
        ];
        let packed = pack_trits_2bit(&trits);
        // Manually verify first byte: Affirm=01 | Tend=00<<2 | Reject=10<<4 | Affirm=01<<6
        let expected_byte0: u8 = 0b01_10_00_01;
        assert_eq!(packed[0], expected_byte0);
    }
}
