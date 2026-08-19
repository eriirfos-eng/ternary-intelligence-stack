// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// ternlang-compress — LLM-to-ternary compression pipeline
//
// Pipeline:
//   Float model (GGUF / safetensors)
//     → per-layer ternary quantization  (PTQ)
//     → sparse zero-index construction  (CSR)
//     → .tern export                    (TernModel on-disk format)

//! LLM-to-ternary compression pipeline — quantize float models to {-1,0,+1}, build sparse zero-index, export .tern files for ternlang-ml inference.

pub mod format;
pub mod quantize;

pub use format::{CompressConfig, TernLayer, TernModel, load_tern, save_tern, synthetic_layers};
pub use quantize::{PerLayerQuant, quantize_layers, quantize_layers_parallel};

/// A single quantized weight tensor — 5 trits packed per byte.
#[derive(Debug, Clone, Default)]
pub struct QuantizedWeights {
    /// Packed ternary bytes (5 trits per byte via pack_5_trits).
    pub data: Vec<u8>,
    /// Total number of trit elements encoded in `data`.
    pub trit_count: usize,
}

/// Run the full compression pipeline on per-layer quantized weights.
///
/// 1. Build sparse CSR index for each layer (skipping zero-weight positions).
/// 2. Pack ternary values 5-per-byte using `pack_5_trits`.
/// 3. Assemble `TernModel` with metadata.
pub fn compress(layers: Vec<PerLayerQuant>, cfg: CompressConfig) -> anyhow::Result<TernModel> {
    let mut tern_layers: Vec<TernLayer> = Vec::with_capacity(layers.len());

    for quant in &layers {
        if cfg.verbose {
            println!(
                "  compressing layer '{}' (sparsity: {:.1}%)",
                quant.name,
                quant.sparsity * 100.0
            );
        }

        // Pack 5 trits per byte
        let mut packed = Vec::new();
        for chunk in quant.trits.chunks(5) {
            let mut trits5 = [ternlang_core::Trit::Tend; 5];
            for (j, &t) in chunk.iter().enumerate() {
                trits5[j] = t;
            }
            packed.push(ternlang_core::pack_5_trits(trits5));
        }

        // Count non-zero (CSR nnz)
        let nnz = quant
            .trits
            .iter()
            .filter(|&&t| t != ternlang_core::Trit::Tend)
            .count();

        tern_layers.push(TernLayer {
            name: quant.name.clone(),
            shape: quant.shape.clone(),
            scale: quant.scale,
            sparsity: quant.sparsity,
            packed_bytes: packed.len(),
            csr_nnz: nnz,
            quantized: QuantizedWeights {
                data: packed,
                trit_count: quant.trits.len(),
            },
        });
    }

    Ok(TernModel {
        source_model: cfg.source_model.clone(),
        architecture: cfg.architecture.clone(),
        num_layers: cfg.num_layers,
        hidden_size: cfg.hidden_size,
        layers: tern_layers,
        config: cfg,
    })
}
