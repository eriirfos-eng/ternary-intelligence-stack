// TernModel — in-memory representation of a fully quantized LLM.
//
// One TernModel holds all compressed layers.  Each TernLayer can store
// weights in either dense (TritMatrix) or sparse (SparseIndex) form,
// chosen automatically based on measured sparsity.

use serde::{Deserialize, Serialize};
use crate::sparse::SparseIndex;

/// Storage format chosen per layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerStorage {
    /// Dense 2-bit-packed ternary.  Better for sparsity < ~75%.
    Dense {
        rows: usize,
        cols: usize,
        /// Trit values packed 4-per-byte (2 bits each: 00=0, 01=+1, 10=-1).
        packed: Vec<u8>,
    },
    /// CSR sparse.  Better for sparsity ≥ ~75%.
    Sparse(SparseIndex),
}

/// One compressed weight matrix (one transformer projection, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TernLayer {
    /// Layer name (e.g. "model.layers.0.self_attn.q_proj.weight").
    pub name: String,
    /// Per-layer scale α = mean(|W_original|).
    pub scale: f32,
    /// Weight storage (dense or sparse).
    pub storage: LayerStorage,
    /// Fraction of zero weights.
    pub sparsity: f64,
    /// Original dtype (for metadata / roundtrip info).
    pub original_dtype: String,
}

impl TernLayer {
    /// Total number of parameters in this layer.
    pub fn num_params(&self) -> usize {
        match &self.storage {
            LayerStorage::Dense { rows, cols, .. } => rows * cols,
            LayerStorage::Sparse(idx) => idx.rows * idx.cols,
        }
    }

    /// Memory used by ternary weights in bytes.
    pub fn memory_bytes(&self) -> usize {
        match &self.storage {
            LayerStorage::Dense { rows, cols, .. } => (rows * cols + 3) / 4,
            LayerStorage::Sparse(idx) => idx.memory_bytes(),
        }
    }
}

/// A complete ternary-quantized LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TernModel {
    /// Source model identifier (e.g. "meta-llama/Llama-3.2-1B").
    pub source_model: String,
    /// ternlang-compress format version.
    pub format_version: u32,
    /// All compressed weight layers.
    pub layers: Vec<TernLayer>,
    /// Model architecture metadata (passed through from source).
    pub architecture: String,
    /// Vocabulary size.
    pub vocab_size: usize,
    /// Hidden dimension.
    pub hidden_size: usize,
    /// Number of transformer layers.
    pub num_layers: usize,
}

impl TernModel {
    /// Total number of parameters across all layers.
    pub fn total_params(&self) -> usize {
        self.layers.iter().map(|l| l.num_params()).sum()
    }

    /// Total ternary weight memory in bytes.
    pub fn total_memory_bytes(&self) -> usize {
        self.layers.iter().map(|l| l.memory_bytes()).sum()
    }

    /// Average sparsity across all layers.
    pub fn mean_sparsity(&self) -> f64 {
        if self.layers.is_empty() { return 0.0; }
        let sum: f64 = self.layers.iter().map(|l| l.sparsity).sum();
        sum / self.layers.len() as f64
    }

    /// Compression ratio vs original f16 storage.
    pub fn compression_ratio_vs_f16(&self) -> f64 {
        let f16_bytes = self.total_params() * 2;
        if f16_bytes == 0 { return 1.0; }
        f16_bytes as f64 / self.total_memory_bytes() as f64
    }

    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "TernModel: {}\n  Params:      {:>12}\n  Ternary mem: {:>12} MB\n  Mean sparsity: {:.1}%\n  vs f16:      {:.1}× smaller",
            self.source_model,
            self.total_params(),
            self.total_memory_bytes() / 1_048_576,
            self.mean_sparsity() * 100.0,
            self.compression_ratio_vs_f16(),
        )
    }
}
