// SPDX-License-Identifier: MIT
//! # Model File Loader
//!
//! Loads a `.tern.bin` ModelCoherence file produced by `transmute_llama.py`
//! and maps its ternarized layers into a 13-expert ExpertBank for EPIS inference.
//!
//! ## Layer → Expert mapping
//! Round-robin: layer i → expert (i % 13).
//! When multiple layers land on the same expert they are fused by majority vote:
//! agreement → keep the value, disagreement → 0 (hold).
//!
//! ## Dimension handling
//! Large transformer layers (e.g. 2048×2048) are truncated to EXPERT_DIMS.
//! Layers smaller than EXPERT_DIMS are zero-padded.

use anyhow::{Context, Result};
use std::path::Path;
use ternlang_ml::coherence::{ModelCoherence, unpack_layer};
use moe_core::core::mock_layer::TernaryLayer;
use moe_core::core::routing::ExpertBank13;

pub const EXPERT_INPUT_DIM: usize = 64;
pub const EXPERT_OUTPUT_DIM: usize = 64;

/// Metadata returned alongside a loaded ExpertBank.
pub struct ModelFileInfo {
    /// Source model name embedded in the .tern.bin file.
    pub source_model: String,
    /// Total number of layers read from the file.
    pub num_layers: usize,
    /// Overall weight sparsity across all layers (fraction of zeros).
    pub sparsity: f32,
    /// Mean absolute scale (alpha) across all experts.
    pub mean_alpha: f32,
}

/// Load a `.tern.bin` file and construct a real ExpertBank13 from its layers.
pub fn load_expert_bank(path: &str) -> Result<(ExpertBank13, ModelFileInfo)> {
    let model_path = Path::new(path);
    let coherence = ModelCoherence::load_bin(model_path)
        .with_context(|| format!("Failed to load tern.bin from '{}'", path))?;

    let source_model = coherence.source_model.clone();
    let num_layers = coherence.layers.len();

    if num_layers == 0 {
        anyhow::bail!("Model '{}' has no layers — file may be corrupted", path);
    }

    let needed = EXPERT_INPUT_DIM * EXPERT_OUTPUT_DIM;
    let mut shards: Vec<Option<Vec<i8>>> = (0..13).map(|_| None).collect();
    let mut alphas = vec![1.0f32; 13];
    let mut total_weights = 0usize;
    let mut total_zeros = 0usize;

    for (i, layer) in coherence.layers.iter().enumerate() {
        let eid = i % 13;
        let raw = unpack_layer(layer).to_i8_vec();

        total_weights += raw.len();
        total_zeros += raw.iter().filter(|&&w| w == 0).count();

        match shards[eid].as_mut() {
            None => {
                // First layer for this expert — truncate or zero-pad to target dims
                let mut shard: Vec<i8> = raw.into_iter().take(needed).collect();
                shard.resize(needed, 0);
                shards[eid] = Some(shard);
                alphas[eid] = layer.scale;
            }
            Some(existing) => {
                // Fuse subsequent layers via majority vote to preserve signal
                for (j, new_w) in raw.into_iter().take(needed).enumerate() {
                    existing[j] = majority_trit(existing[j], new_w);
                }
                // Update alpha toward the mean
                alphas[eid] = (alphas[eid] + layer.scale) * 0.5;
            }
        }
    }

    let experts: Vec<TernaryLayer> = (0..13)
        .map(|eid| TernaryLayer {
            weights: shards[eid].take().unwrap_or_else(|| vec![0i8; needed]),
            alpha: alphas[eid],
            bias: vec![0.0f32; EXPERT_OUTPUT_DIM],
            input_dim: EXPERT_INPUT_DIM,
            output_dim: EXPERT_OUTPUT_DIM,
        })
        .collect();

    let sparsity = if total_weights > 0 {
        total_zeros as f32 / total_weights as f32
    } else {
        0.0
    };

    let mean_alpha = alphas.iter().sum::<f32>() / 13.0;

    let info = ModelFileInfo {
        source_model,
        num_layers,
        sparsity,
        mean_alpha,
    };

    Ok((ExpertBank13 { experts }, info))
}

/// Ternary majority vote: same sign → keep, conflict → 0 (hold/uncertain).
#[inline]
fn majority_trit(a: i8, b: i8) -> i8 {
    if a == b { a } else { 0 }
}
