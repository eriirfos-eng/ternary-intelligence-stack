// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// Commercial tier. See LICENSE-COMMERCIAL in the repository root.

//! ternlang-ruvector: Enterprise Bridge for Vector Database Acceleration
//!
//! This crate formalizes the RuVector Sparse GEMV acceleration pipeline,
//! dramatically speeding up RAG workflows by using ternary-quantized embeddings
//! and sparse inference kernels.

use ternlang_core::trit::Trit;
use ternlang_ml::{TritMatrix, quantize, bitnet_threshold};
use rayon::prelude::*;
use serde::{Serialize, Deserialize};

/// A high-performance ternary-quantized vector database.
#[derive(Serialize, Deserialize)]
pub struct RuVectorDB {
    /// The document embeddings matrix [num_docs x embedding_dim].
    pub embeddings: TritMatrix,
    /// Metadata associated with each document (e.g. original text or ID).
    pub metadata: Vec<String>,
}

impl RuVectorDB {
    /// Create a new database from f32 embeddings.
    pub fn from_f32(embeddings: &[Vec<f32>], metadata: Vec<String>) -> anyhow::Result<Self> {
        if embeddings.is_empty() {
            return Err(anyhow::anyhow!("Embeddings cannot be empty"));
        }
        let rows = embeddings.len();
        let cols = embeddings[0].len();
        
        // Flatten for quantization
        let flat_f32: Vec<f32> = embeddings.iter().flatten().cloned().collect();
        let threshold = bitnet_threshold(&flat_f32);
        
        let trit_matrix = TritMatrix::from_f32(rows, cols, &flat_f32, threshold);
        
        Ok(Self {
            embeddings: trit_matrix,
            metadata,
        })
    }

    /// Search for the top-k most similar documents using ternary Sparse GEMV.
    ///
    /// The query vector is quantized to ternary before search.
    pub fn search(&self, query_f32: &[f32], top_k: usize) -> Vec<SearchResult> {
        let threshold = bitnet_threshold(query_f32);
        let query_trits = quantize(query_f32, threshold);
        
        // Similarity scores using ternary dot product
        let scores = self.sparse_gemv_similarity(&query_trits);
        
        let mut results: Vec<SearchResult> = scores.into_iter()
            .enumerate()
            .map(|(i, score)| SearchResult {
                index: i,
                score,
                metadata: self.metadata.get(i).cloned().unwrap_or_default(),
            })
            .collect();
        
        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(top_k);
        
        results
    }

    /// Optimized Sparse GEMV similarity kernel.
    /// Computes the dot product between the query vector and every row in the database.
    fn sparse_gemv_similarity(&self, query: &[Trit]) -> Vec<f32> {
        let num_docs = self.embeddings.rows;
        let dim = self.embeddings.cols;
        
        // Flatten query to i8 for speed
        let q_flat: Vec<i8> = query.iter().map(|&t| match t {
            Trit::Affirm => 1,
            Trit::Reject => -1,
            Trit::Tend   => 0,
        }).collect();

        // Database embeddings in i8
        let db_flat = self.embeddings.to_i8_vec();

        // Parallel row similarity
        (0..num_docs).into_par_iter().map(|row_idx| {
            let row_data = &db_flat[row_idx * dim .. (row_idx + 1) * dim];
            let mut acc: i32 = 0;
            
            // Sparse loop: skip zeros in either the query or the database row.
            // In a production "RuVector" bridge, we'd use CSC/CSR formats here.
            for i in 0..dim {
                let qi = q_flat[i];
                if qi == 0 { continue; }
                
                let di = row_data[i];
                if di == 0 { continue; }
                
                acc += (qi * di) as i32;
            }
            
            acc as f32
        }).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub index: usize,
    pub score: f32,
    pub metadata: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruvector_search() {
        let embeddings = vec![
            vec![1.0, 0.0, -1.0, 0.5],
            vec![-1.0, 1.0, 0.0, 0.0],
            vec![0.1, 0.1, 0.1, 0.1], // Mostly zeros
        ];
        let metadata = vec!["Doc A".to_string(), "Doc B".to_string(), "Doc C".to_string()];
        
        let db = RuVectorDB::from_f32(&embeddings, metadata).unwrap();
        
        let query = vec![1.0, 0.0, -1.0, 0.0];
        let results = db.search(&query, 2);
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].metadata, "Doc A");
        assert!(results[0].score > results[1].score);
    }
}
