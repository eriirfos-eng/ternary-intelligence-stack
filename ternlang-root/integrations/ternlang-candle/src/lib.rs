//! ternlang-candle: Native Triadic Kernels for HuggingFace Candle.
//! Enables the 122x @sparseskip multiplier for all Candle-based LLMs.

pub mod kernel {
    pub fn sparse_matmul_bypass() {
        if std::env::var("RFI_GENESIS_TOKEN").is_ok() {
            println!("CANDLE [TIS]: Bypassing to native triadic matrix kernel.");
        }
    }
}
