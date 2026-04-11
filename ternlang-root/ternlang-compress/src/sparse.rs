// Sparse zero-index (CSR — Compressed Sparse Row) for ternary weight matrices.
//
// A ternary matrix has three states: {-1, 0, +1}.  The majority of weights
// in a quantized LLM are 0 (typically 60-99% depending on sparsity).
//
// Storing the FULL matrix wastes memory on zeros.  Instead we store:
//   - `values`:  only the non-zero trit values (+1 or -1), packed as i8
//   - `col_idx`: column index for each non-zero value (u32)
//   - `row_ptr`: pointer into `values` for the start of each row (u64)
//
// At inference the sparse_matmul kernel iterates `row_ptr[row]..row_ptr[row+1]`
// and accumulates only the non-zero contributions — zeros are never touched.
//
// Memory breakdown for a 4096×4096 weight matrix at 60% sparsity:
//   Dense ternary (2-bit packed): 4096×4096×2b = 4 MB
//   CSR (i8 values + u32 col_idx): 6.7M non-zeros × 5 bytes + 16KB row_ptr ≈ 32 MB
//   — dense is better here. CSR wins at >80% sparsity:
//   80% sparse: 3.3M nnz × 5 bytes ≈ 16 MB < dense.
//
// Practical recommendation: use CSR for sparsity > 75%, packed ternary otherwise.
// The pipeline chooses automatically based on measured sparsity.

use ternlang_core::trit::Trit;
use serde::{Deserialize, Serialize};

/// Compressed Sparse Row index for one weight matrix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseIndex {
    pub rows: usize,
    pub cols: usize,
    /// Non-zero trit values, stored as i8 (+1 or -1).
    pub values: Vec<i8>,
    /// Column of each non-zero value.  Same length as `values`.
    pub col_idx: Vec<u32>,
    /// `row_ptr[r]` = index into `values` where row `r` starts.
    /// Length = rows + 1.  `row_ptr[rows]` = values.len().
    pub row_ptr: Vec<u64>,
    /// Number of non-zero elements.
    pub nnz: usize,
    /// Sparsity = 1 - nnz / (rows * cols).
    pub sparsity: f64,
}

impl SparseIndex {
    /// Build a CSR index from a flat row-major slice of trits.
    pub fn from_trits(rows: usize, cols: usize, data: &[Trit]) -> Self {
        assert_eq!(data.len(), rows * cols);

        let mut values  = Vec::new();
        let mut col_idx = Vec::new();
        let mut row_ptr = vec![0u64; rows + 1];

        for r in 0..rows {
            row_ptr[r] = values.len() as u64;
            for c in 0..cols {
                let t = data[r * cols + c];
                if t != Trit::Tend {
                    values.push(match t {
                        Trit::Affirm =>  1i8,
                        Trit::Reject => -1i8,
                        Trit::Tend   =>  0i8,
                    });
                    col_idx.push(c as u32);
                }
            }
        }
        row_ptr[rows] = values.len() as u64;

        let nnz      = values.len();
        let total    = rows * cols;
        let sparsity = 1.0 - nnz as f64 / total as f64;

        Self { rows, cols, values, col_idx, row_ptr, nnz, sparsity }
    }

    /// Reconstruct the dense trit slice (for testing / correctness checks).
    pub fn to_dense(&self) -> Vec<Trit> {
        let mut out = vec![Trit::Tend; self.rows * self.cols];
        for r in 0..self.rows {
            let start = self.row_ptr[r] as usize;
            let end   = self.row_ptr[r + 1] as usize;
            for idx in start..end {
                let c = self.col_idx[idx] as usize;
                out[r * self.cols + c] = if self.values[idx] > 0 {
                    Trit::Affirm
                } else {
                    Trit::Reject
                };
            }
        }
        out
    }

    /// Memory footprint in bytes.
    pub fn memory_bytes(&self) -> usize {
        self.values.len()      // i8 = 1 byte each
        + self.col_idx.len() * 4  // u32 = 4 bytes each
        + self.row_ptr.len() * 8  // u64 = 8 bytes each
    }

    /// Whether CSR is more memory-efficient than a 2-bit packed dense matrix.
    pub fn is_efficient(&self) -> bool {
        let dense_packed = (self.rows * self.cols + 3) / 4; // 2 bits per trit
        self.memory_bytes() < dense_packed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_sparse() {
        let trits = vec![
            Trit::Affirm, Trit::Tend,   Trit::Reject,
            Trit::Tend,   Trit::Tend,   Trit::Affirm,
            Trit::Reject, Trit::Affirm, Trit::Tend,
        ];
        let idx = SparseIndex::from_trits(3, 3, &trits);
        assert_eq!(idx.nnz, 5);
        let dense = idx.to_dense();
        assert_eq!(dense, trits);
    }
}
