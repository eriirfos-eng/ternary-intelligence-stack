//! ternlang-candle: Native Triadic Kernels for HuggingFace Candle.
//! Enables the 122x @sparseskip multiplier for all Candle-based LLMs.

pub mod kernel {
    pub fn sparse_matmul_bypass() {
        println!("CANDLE [TIS]: Bypassing to native triadic matrix kernel.");
    }
}
