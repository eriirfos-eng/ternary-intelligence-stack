// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// ternlang-compress — LLM-to-ternary compression pipeline
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
//
// Pipeline:
//   Float model (GGUF / safetensors)
//     → per-layer ternary quantization  (PTQ)
//     → sparse zero-index construction  (CSR)
//     → .tern export                    (TernModel on-disk format)
//
// The resulting model is loaded by ternlang-ml's sparse_matmul kernel,
// which skips all zero-weight positions at inference time.

pub mod quantize;
pub mod sparse;
pub mod model;
pub mod pipeline;
pub mod format;

pub use model::{TernModel, TernLayer};
pub use pipeline::{compress, CompressConfig};
pub use quantize::PerLayerQuant;
pub use sparse::SparseIndex;

/// Current on-disk format version.  Bump when the .tern format changes.
pub const FORMAT_VERSION: u32 = 1;
