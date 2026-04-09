// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.
// Commercial tier. See LICENSE-COMMERCIAL in the repository root.
// Unauthorized use, copying, or distribution is prohibited.
// Reference Patent Pending A50296/2026.

use ternlang_core::trit::Trit;

/// Opcode: TSPARSE_MATMUL (0x1F)
/// 
/// Matrix Multiplication optimized for balanced ternary.
/// Uses the @sparseskip algorithm to bypass all State 0 (Tend) weights.
pub struct TSparseMatmul;

impl TSparseMatmul {
    /// Executes the sparse matmul logic.
    /// Bypasses 0-weights using the @sparseskip mechanism.
    pub fn execute(a_rows: usize, a_cols: usize, a_data: &[Trit], b_cols: usize, b_data: &[Trit]) -> Vec<Trit> {
        let mut result = vec![Trit::Tend; a_rows * b_cols];

        for i in 0..a_rows {
            for k in 0..a_cols {
                let a_val = a_data[i * a_cols + k];
                
                // @sparseskip: If the weight is State 0 (Tend), skip the entire dot-product contribution.
                if a_val == Trit::Tend {
                    continue;
                }

                for j in 0..b_cols {
                    let b_val = b_data[k * b_cols + j];
                    if b_val == Trit::Tend {
                        continue;
                    }

                    let (prod, _) = a_val * b_val;
                    let (sum, _) = result[i * b_cols + j] + prod;
                    result[i * b_cols + j] = sum;
                }
            }
        }
        result
    }
}
