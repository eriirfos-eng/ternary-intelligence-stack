// GPU inference benchmark — same 15 prompts as moe-test --bench, on CUDA.
//
// On CPU: dispatches to the vpsignb i8quant kernel (forward_i8).
// On CUDA: dispatches to candle cuBLAS with cached f32 ternary weights.
//
// Run locally (CPU):
//   cargo run --release --bin gpu_bench -p moe-llm-core
//
// Run on Modal T4 (via train_modal.py bench_gpu):
//   python3 train_modal.py bench_gpu

use candle_core::{Device, DType, IndexOp, Tensor};
use candle_nn::VarBuilder;
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use serde_json::Value;
use std::fs;
use std::time::Instant;

const PROMPTS: &[&str] = &[
    "in the beginning god created the",
    "die Geschichte der Europäischen Union begann",
    "the ternary number system uses three distinct",
    "once upon a time in a kingdom far",
    "was ist das ternäre Zahlensystem und wie funktioniert",
    "the EU AI Act entered into force on",
    "die künstliche Intelligenz verändert die Gesellschaft",
    "Isaac Newton discovered the law of universal gravitation",
    "the transformer architecture introduced attention mechanisms",
    "der Quantencomputer nutzt Quantenmechanik um",
    "mixture of experts models improve efficiency by routing",
    "the mitochondria is the powerhouse of the",
    "in der Bibel steht im ersten Buch Mose geschrieben",
    "silicon has revolutionized computing because it allows",
    "the meaning of life according to",
];

const MAX_NEW_TOKENS: usize = 128;

fn generate(
    model:     &Transformer,
    tokenizer: &BpeTokenizer,
    prompt:    &str,
    max_new:   usize,
    max_seq:   usize,
    device:    &Device,
) -> (String, f64) {
    let prompt_ids = tokenizer.encode(prompt);
    let t0 = Instant::now();

    // Keep the growing token sequence on GPU — zero per-token CPU↔GPU syncs.
    let mut tokens_gpu = Tensor::new(prompt_ids.as_slice(), device).unwrap()
        .to_dtype(DType::U32).unwrap();

    for _ in 0..max_new {
        let len   = tokens_gpu.elem_count();
        let start = len.saturating_sub(max_seq);
        let ctx   = if start > 0 {
            tokens_gpu.narrow(0, start, len - start).unwrap()
        } else {
            tokens_gpu.clone()
        };
        let input  = ctx.unsqueeze(0).unwrap();
        let logits = model.forward(&input).unwrap();
        let dims   = logits.dims();
        let last   = logits.i((0, dims[1] - 1)).unwrap();
        // argmax stays on GPU — no sync
        let next   = last.argmax(0).unwrap()
                         .to_dtype(DType::U32).unwrap()
                         .unsqueeze(0).unwrap();
        // cat: grows the GPU buffer by 1 token, never leaves the device
        tokens_gpu = Tensor::cat(&[&tokens_gpu, &next], 0).unwrap();
    }

    // Single sync at the end to pull the full sequence back for decoding.
    let final_ids: Vec<u32> = tokens_gpu
        .to_device(&Device::Cpu).unwrap()
        .to_vec1::<u32>().unwrap();
    let elapsed = t0.elapsed().as_secs_f64();
    let tok_s   = max_new as f64 / elapsed;
    (tokenizer.decode(&final_ids), tok_s)
}

fn main() -> candle_core::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = args.first().cloned()
        .unwrap_or_else(|| ".".to_string());

    let config_path = format!("{root}/models/albert_v3.0.config.json");
    let vocab_path  = format!("{root}/data/vocab_v3.json");
    let ckpt_best   = format!("{root}/models/albert_v3.0.best.safetensors");
    let ckpt_latest = format!("{root}/models/albert_v3.0.safetensors");
    let ckpt = if std::path::Path::new(&ckpt_best).exists() { &ckpt_best } else { &ckpt_latest };

    let device = Device::cuda_if_available(0).unwrap_or(Device::Cpu);
    let device_name = match &device {
        Device::Cuda(_) => "CUDA (T4)",
        Device::Cpu     => "CPU (vpsignb i8quant kernel)",
        _               => "unknown",
    };

    let config_str  = fs::read_to_string(&config_path).expect("config.json not found");
    let config_json: Value = serde_json::from_str(&config_str).expect("invalid JSON");
    let mut config  = TransformerConfig::default();
    config.num_layers  = config_json["num_layers"].as_u64().unwrap_or(18) as usize;
    config.hidden_size = config_json["hidden_size"].as_u64().unwrap_or(256) as usize;
    config.num_heads   = config_json["num_heads"].as_u64().unwrap_or(4) as usize;
    config.num_experts = config_json["num_experts"].as_u64().unwrap_or(12) as usize;
    config.max_seq_len = config_json["max_seq_len"].as_u64().unwrap_or(256) as usize;
    let tokenizer   = BpeTokenizer::new(&vocab_path);
    config.vocab_size = tokenizer.vocab_size();

    let tensors = candle_core::safetensors::load(ckpt, &device)?;
    let vb = VarBuilder::from_tensors(tensors, DType::F32, &device);
    let model = Transformer::new(&config, vb)?;
    model.prepare_inference()?;

    let epoch = config_json["global_epoch"].as_u64().unwrap_or(0);
    println!("=== albert. GPU Inference Benchmark ===");
    println!("Device     : {device_name}");
    println!("Checkpoint : {ckpt}");
    println!("Arch       : {}L · {}H · {}E · {}CTX · {}V",
        config.num_layers, config.hidden_size, config.num_experts,
        config.max_seq_len, config.vocab_size);
    println!("Epoch      : {epoch}");
    println!("Tokens/run : {MAX_NEW_TOKENS}");
    println!("{}", "─".repeat(56));

    let mut total_tok_s = 0.0f64;

    for (i, prompt) in PROMPTS.iter().enumerate() {
        let (output, tok_s) = generate(
            &model, &tokenizer, prompt, MAX_NEW_TOKENS,
            config.max_seq_len, &device,
        );
        let preview: String = output.chars().take(60).collect();
        println!("[P{}] {}", i + 1, prompt);
        println!("→ {}…", preview.trim());
        println!("    {:.1} tok/s  {:.0}ms/tok", tok_s, 1000.0 / tok_s);
        println!();
        total_tok_s += tok_s;
    }

    let avg = total_tok_s / PROMPTS.len() as f64;
    println!("{}", "═".repeat(56));
    println!("Average : {:.1} tok/s  ({device_name})", avg);
    println!("{}", "═".repeat(56));

    Ok(())
}
