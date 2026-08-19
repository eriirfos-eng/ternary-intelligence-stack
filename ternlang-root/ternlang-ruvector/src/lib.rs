// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.

//! ternlang-ruvector: Enterprise Bridge for Vector Database Acceleration
//!
//! This crate formalizes the RuVector Sparse GEMV acceleration pipeline,
//! dramatically speeding up RAG workflows by using ternary-quantized embeddings
//! and sparse inference kernels.

//! Ternary-optimized Sparse GEMV acceleration for Vector Databases (RuVector Bridge).

use std::collections::HashMap;
use ternlang_core::Trit;

/// A ternary-quantized sparse vector for RuVector acceleration.
#[derive(Debug, Clone, PartialEq)]
pub struct TritVector {
    /// Trit values (Affirm/Reject/Tend).
    pub trits: Vec<ternlang_core::Trit>,
    /// Dimensionality of the vector.
    pub dim: usize,
}

/// A sparse vector store with ternary-quantized embeddings.
/// Supports add, search (sparse GEMV), and bulk import.
#[derive(Debug, Clone)]
pub struct RuVectorDB {
    /// Internal storage: id → (embedding, payload)
    vectors: HashMap<String, (TritVector, HashMap<String, String>)>,
    /// Embedding dimensionality.
    dim: usize,
}

impl RuVectorDB {
    /// Create a new empty store with the given dimensionality.
    pub fn new(dim: usize) -> Self {
        Self {
            vectors: HashMap::new(),
            dim,
        }
    }

    /// Build from f32 embeddings by quantizing each to ternary.
    /// Each embedding is mapped to Trit via sign: >+threshold → Affirm, <+threshold → Reject, else Tend.
    pub fn from_f32(
        embeddings: &[Vec<f32>],
        metadata: Vec<String>,
    ) -> Result<Self, String> {
        if embeddings.is_empty() {
            return Err("no embeddings provided".into());
        }
        let dim = embeddings[0].len();
        let mut db = Self::new(dim);
        for (emb, id) in embeddings.iter().zip(metadata) {
            let trits: Vec<ternlang_core::Trit> = emb
                .iter()
                .map(|&w| {
                    if w > 0.01 {
                        ternlang_core::Trit::Affirm
                    } else if w < -0.01 {
                        ternlang_core::Trit::Reject
                    } else {
                        ternlang_core::Trit::Tend
                    }
                })
                .collect();
            db.vectors.insert(id, (TritVector { trits, dim }, HashMap::new()));
        }
        Ok(db)
    }

    /// Insert a vector with a string ID and optional payload metadata.
    pub fn insert(&mut self, id: impl Into<String>, trits: Vec<ternlang_core::Trit>) -> bool {
        let trits: Vec<_> = trits;
        if trits.len() != self.dim {
            return false;
        }
        self.vectors.insert(id.into(), (TritVector { trits, dim: self.dim }, HashMap::new()));
        true
    }

    /// Number of stored vectors.
    pub fn len(&self) -> usize {
        self.vectors.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.vectors.is_empty()
    }

    /// Search for the top-k most similar vectors using sparse ternary GEMV.
    /// Similarity = dot product of trit vectors (Affirm=+1, Reject=-1, Tend=0).
    pub fn search(&self, query: &[ternlang_core::Trit], k: usize) -> Vec<(String, i64)> {
        if query.len() != self.dim {
            return vec![];
        }
        let mut results: Vec<(String, i64)> = self
            .vectors
            .iter()
            .map(|(id, (vec, _))| {
                let score: i64 = vec
                    .trits
                    .iter()
                    .zip(query.iter())
                    .map(|(a, b)| (*a as i8 as i64) * (*b as i8 as i64))
                    .sum();
                (id.clone(), score)
            })
            .collect();
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.truncate(k);
        results
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trit_vector_and_search() {
        let mut db = RuVectorDB::new(4);
        let _ = db.insert("a", vec![Trit::Affirm, Trit::Affirm, Trit::Tend, Trit::Reject]);
        let _ = db.insert("b", vec![Trit::Affirm, Trit::Tend, Trit::Tend, Trit::Tend]);

        let results = db.search(&[Trit::Affirm, Trit::Affirm, Trit::Tend, Trit::Tend], 2);
        assert_eq!(results[0].0, "a");
    }
}
