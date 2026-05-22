// Perplexity evaluator for Albert-MoE-13.
//
// Loads the current 12L checkpoint, tokenizes a corpus file, and computes
// per-token cross-entropy loss in non-overlapping 128-token windows.
// PPL = exp(mean CE loss) — reported per-token, not per-character.
//
// Usage:
//   cargo run --release -p moe-llm-core --bin eval_perplexity [corpus_file]
//
// Defaults to data/corpus/stage_3/alice.txt (fast, ~150KB, <1 min on CPU).
// Pass a path argument to evaluate any other corpus file.

use candle_core::{Device, DType, Result, Tensor};
use candle_nn::VarBuilder;
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use std::fs;
use std::time::Instant;
use serde_json::Value;

const CHECKPOINT: &str = "models/albert_v3.0.safetensors";
const CONFIG:     &str = "models/albert_v3.0.config.json";
const VOCAB:      &str = "data/vocab_v3.json";
const DEFAULT_CORPUS: &str = "data/corpus/stage_3/alice.txt";

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let corpus_path = args.iter()
        .find(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| DEFAULT_CORPUS.to_string());
    let max_windows: Option<usize> = args.iter()
        .find(|a| a.starts_with("--max-windows="))
        .and_then(|a| a["--max-windows=".len()..].parse().ok());

    println!("=== Albert-MoE-13 Perplexity Eval ===");
    println!("Checkpoint : {}", CHECKPOINT);
    println!("Corpus     : {}", corpus_path);

    // ── Load config ──────────────────────────────────────────────────────────
    let config_str = fs::read_to_string(CONFIG).expect("config.json not found");
    let config_json: Value = serde_json::from_str(&config_str).expect("invalid config JSON");
    let mut config = TransformerConfig::default();
    config.num_layers  = config_json["num_layers"].as_u64().unwrap_or(12) as usize;
    config.hidden_size = config_json["hidden_size"].as_u64().unwrap_or(256) as usize;
    config.num_heads   = config_json["num_heads"].as_u64().unwrap_or(4) as usize;
    config.num_experts = config_json["num_experts"].as_u64().unwrap_or(12) as usize;
    config.max_seq_len = config_json["max_seq_len"].as_u64().unwrap_or(128) as usize;

    // ── Load tokenizer ───────────────────────────────────────────────────────
    let tokenizer = BpeTokenizer::new(VOCAB);
    config.vocab_size = tokenizer.vocab_size();

    println!("Architecture: {}L · {}H · {}E · {}CTX · {}V",
        config.num_layers, config.hidden_size, config.num_experts,
        config.max_seq_len, config.vocab_size);

    // ── Load model from checkpoint ───────────────────────────────────────────
    let device = Device::Cpu;
    let tensors = candle_core::safetensors::load(CHECKPOINT, &device)?;
    let n_tensors = tensors.len();
    let vb = VarBuilder::from_tensors(tensors, DType::F32, &device);
    let model = Transformer::new(&config, vb)?;
    model.prepare_inference()?;
    println!("Checkpoint loaded ({} tensors, weights ternarized).", n_tensors);

    // ── Tokenize corpus ──────────────────────────────────────────────────────
    let bytes = fs::read(&corpus_path)
        .unwrap_or_else(|e| panic!("Cannot read corpus {}: {}", corpus_path, e));
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let tokens = tokenizer.encode(&text);
    println!("Corpus     : {} chars → {} tokens", text.len(), tokens.len());

    let seq_len = config.max_seq_len; // 128
    let n_windows_full = tokens.len().saturating_sub(1) / seq_len;
    let n_windows = max_windows.map(|m| m.min(n_windows_full)).unwrap_or(n_windows_full);
    if n_windows == 0 {
        println!("Corpus too short for a single evaluation window.");
        return Ok(());
    }
    if max_windows.is_some() {
        println!("Windows    : {} × {} tokens (sample: {}/{})", n_windows, seq_len, n_windows, n_windows_full);
    } else {
        println!("Windows    : {} × {} tokens", n_windows, seq_len);
    }
    println!("Evaluating …");

    // ── Evaluate cross-entropy window by window ──────────────────────────────
    let mut total_ce: f64 = 0.0;
    let mut total_tokens: usize = 0;
    let t0 = Instant::now();

    for w in 0..n_windows {
        let start = w * seq_len;
        let input_ids: Vec<u32> = tokens[start .. start + seq_len].to_vec();
        let target_ids: Vec<u32> = tokens[start + 1 .. start + seq_len + 1].to_vec();

        let input = Tensor::from_vec(
            input_ids.iter().map(|&x| x as i64).collect::<Vec<_>>(),
            (1, seq_len),
            &device,
        )?;

        let logits = model.forward(&input)?; // (1, seq_len, vocab)
        let (_, s, v) = logits.dims3()?;

        // Reshape to (seq_len, vocab) and compute CE against targets.
        let logits_2d = logits.reshape((s, v))?;
        let targets = Tensor::from_vec(
            target_ids.iter().map(|&x| x as i64).collect::<Vec<_>>(),
            (s,),
            &device,
        )?;

        // Log-softmax + NLL = cross-entropy.
        let log_probs = candle_nn::ops::log_softmax(&logits_2d, 1)?;
        // Gather log prob of each target token.
        let targets_u = targets.to_dtype(DType::U32)?;
        let nll = log_probs.gather(&targets_u.unsqueeze(1)?, 1)?.squeeze(1)?;
        // nll is negative log-probs; mean CE = -mean(nll)
        let window_ce = -nll.mean(0)?.to_scalar::<f32>()? as f64;

        total_ce += window_ce * seq_len as f64;
        total_tokens += seq_len;

        if w % 10 == 0 || w == n_windows - 1 {
            let elapsed = t0.elapsed().as_secs_f64();
            let pct = (w + 1) as f64 / n_windows as f64 * 100.0;
            let eta = if w > 0 {
                elapsed / (w + 1) as f64 * (n_windows - w - 1) as f64
            } else { 0.0 };
            print!("\r  [{:5.1}%] window {}/{} | running PPL {:.1} | ETA {:.0}s   ",
                pct, w + 1, n_windows,
                (total_ce / total_tokens as f64).exp(),
                eta);
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
    }
    println!();

    // ── Report ───────────────────────────────────────────────────────────────
    let mean_ce = total_ce / total_tokens as f64;
    let ppl     = mean_ce.exp();
    let elapsed = t0.elapsed().as_secs_f64();

    println!();
    println!("=== Results ===");
    println!("  Mean CE loss : {:.4}", mean_ce);
    println!("  Perplexity   : {:.1}", ppl);
    println!("  Tokens eval  : {}", total_tokens);
    println!("  Time         : {:.1}s", elapsed);
    println!("  Corpus       : {}", corpus_path);
    println!("  Checkpoint   : {}", CHECKPOINT);
    println!();
    println!("PPL = {:.1}  (exp({:.4}))", ppl, mean_ce);

    Ok(())
}
