//! # Albert-Test: Scientific Dashboard v2.0.0 (13-Node Evolution)
//! 
//! High-fidelity interactive Sandbox for the TIS with real-time telemetry, Smart-Stop, and Scrolling.

use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::fs;

use candle_core::{Device, DType, Tensor, IndexOp};
use candle_nn::VarBuilder;
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use serde_json::Value;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct App {
    model: Transformer,
    tokenizer: BpeTokenizer,
    dev: Device,
    input: String,
    transcript: String,
    messages: Vec<(String, String)>,
    model_id: String,
    checkpoint: String,
    total_epochs: u32,
    num_experts: usize,

    // Scientific Metrics
    token_latency_ms: u64,
    tokens_per_sec: f32,
    active_experts: String,
    est_gflops: f32,

    // UI State
    is_generating: bool,
    current_tokens: Vec<u32>,
    prompt_token_len: usize,
    tokens_to_generate: usize,
    scroll_pos: u16,
    auto_scroll: bool,

    // KV-cache decode state
    need_prefill: bool,   // true on first step after start_generation()
    kv_seq_pos: usize,    // absolute position of next token to decode
}

impl App {
    fn new() -> Self {
        let dev = Device::Cpu;
        let (checkpoint_path, version) = find_latest_checkpoint();
        
        let _metadata = fs::metadata(&checkpoint_path).ok();
        
        let vocab_path = "data/vocab.json";
        let tokenizer = BpeTokenizer::new(vocab_path);
        
        let mut config_path = format!("models/bible_ternary_{}.config.json", version);
        if !std::path::Path::new(&config_path).exists() {
            config_path = format!("models/registry/bible_ternary_{}.config.json", version);
        }
        let config_str = fs::read_to_string(&config_path).expect("Unable to read config.json. The HuggingFace standard requires a config file next to the model.");
        let config_json: Value = serde_json::from_str(&config_str).expect("Invalid JSON in config file.");

        let mut config = TransformerConfig::default();
        config.vocab_size = tokenizer.vocab_size();
        config.hidden_size = config_json["hidden_size"].as_u64().unwrap() as usize;
        config.num_layers = config_json["num_layers"].as_u64().unwrap() as usize;
        config.num_heads = config_json["num_heads"].as_u64().unwrap() as usize;
        config.max_seq_len = config_json["max_seq_len"].as_u64().unwrap() as usize;
        config.num_experts = config_json["num_experts"].as_u64().unwrap() as usize;

        let varmap = candle_nn::VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
        let model = Transformer::new(&config, vb).expect("Architecture initialization failed");

        let checkpoint_data = candle_core::safetensors::load(&checkpoint_path, &dev)
            .expect("Failed to load .safetensors weights");
        let all_vars = varmap.data().lock().unwrap();

        let mut loaded_count = 0;
        let mut missing_count = 0;

        for (name, var) in all_vars.iter() {
            if let Some(tensor) = checkpoint_data.get(name) {
                let _ = var.set(tensor);
                loaded_count += 1;
            } else {
                missing_count += 1;
            }
        }
        drop(all_vars);

        // Pre-ternarize all weights once — avoids re-quantizing on every decode step.
        model.prepare_inference().expect("inference weight cache failed");
        
        let meta_path = format!("models/bible_ternary_{}.meta", version);
        let total_epochs = fs::read_to_string(&meta_path).unwrap_or("0".to_string()).trim().parse::<u32>().unwrap_or(0);

        let num_experts = config.num_experts;
        Self {
            model,
            tokenizer,
            dev,
            input: String::new(),
            transcript: format!("System: Model {} loaded. Epoch Mileage: {}\n", version, total_epochs),
            messages: Vec::new(),
            model_id: "MoE-13-Ternary".to_string(),
            checkpoint: version,
            total_epochs,
            num_experts,
            token_latency_ms: 0,
            tokens_per_sec: 0.0,
            active_experts: format!("3/{} (Top-3, idle)", num_experts),
            est_gflops: 0.0,
            is_generating: false,
            current_tokens: Vec::new(),
            prompt_token_len: 0,
            tokens_to_generate: 0,
            scroll_pos: 0,
            auto_scroll: true,
            need_prefill: false,
            kv_seq_pos: 0,
        }
    }

    fn start_generation(&mut self) {
        if self.input.trim().is_empty() { return; }

        let user_msg = self.input.drain(..).collect::<String>();
        self.transcript.push_str(&format!("\nUser: {}\nAlbert: ", user_msg));

        self.current_tokens = self.tokenizer.encode(&user_msg);
        self.prompt_token_len = self.current_tokens.len();
        self.messages.push(("User".to_string(), user_msg));
        self.messages.push(("Albert".to_string(), String::new()));

        // Reset KV-cache for new conversation turn.
        self.model.clear_kv_cache();
        self.need_prefill = true;
        self.kv_seq_pos = 0;

        self.is_generating = true;
        self.tokens_to_generate = 64;
    }

    fn step_generation(&mut self) {
        if !self.is_generating || self.tokens_to_generate == 0 {
            self.is_generating = false;
            return;
        }

        let start = Instant::now();

        let next_token = if self.need_prefill {
            // ── Prefill: process full prompt through all layers, populate KV-cache ──
            let context = &self.current_tokens[..];
            let n_ctx = context.len();
            let est_active = (n_ctx * 3).min(self.num_experts);
            self.active_experts = format!("{}/{} (prefill)", est_active, self.num_experts);

            let input = Tensor::new(context, &self.dev).unwrap()
                .unsqueeze(0).unwrap().to_dtype(candle_core::DType::U32).unwrap();
            let logits = self.model.forward_prefill(&input).unwrap();
            let dims = logits.dims();
            let last_logits = logits.i((0, dims[1] - 1)).unwrap();

            self.kv_seq_pos = context.len(); // next token goes at this position
            self.need_prefill = false;

            let pr = candle_nn::ops::softmax(&last_logits, 0).unwrap().to_vec1::<f32>().unwrap();
            pr.iter().enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i as u32).unwrap()
        } else {
            // ── Decode: single token, O(1) per layer via KV-cache ──
            // 3 experts active, 9 skipped — SparseSkip is at full effect here.
            self.active_experts = format!("3/{} (3↑ 9↓ sparse)", self.num_experts);

            let last_token = *self.current_tokens.last().unwrap();
            let logits = self.model.forward_decode(last_token, self.kv_seq_pos, &self.dev).unwrap();
            self.kv_seq_pos += 1;

            let last_logits = logits.i((0, 0)).unwrap();
            let pr = candle_nn::ops::softmax(&last_logits, 0).unwrap().to_vec1::<f32>().unwrap();
            pr.iter().enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i as u32).unwrap()
        };

        self.current_tokens.push(next_token);
        self.tokens_to_generate -= 1;
        
        let elapsed = start.elapsed();
        self.token_latency_ms = elapsed.as_millis() as u64;
        if self.token_latency_ms > 0 {
            self.tokens_per_sec = 1000.0 / self.token_latency_ms as f32;
            self.est_gflops = (self.tokens_per_sec * 2_000_000.0) / 1_000_000_000.0;
        }

        // Decode only the generated tokens (everything after the prompt)
        let generated = &self.current_tokens[self.prompt_token_len..];
        let albert_text = self.tokenizer.decode(generated);

        if let Some(msg) = self.messages.last_mut() {
            let delta = if albert_text.len() > msg.1.len() {
                albert_text[msg.1.len()..].to_string()
            } else { String::new() };

            self.transcript.push_str(&delta);
            msg.1 = albert_text;
            
            // Stall detection
            let current_words: Vec<&str> = msg.1.split_whitespace().collect();
            if current_words.len() > 10 {
                let last_n = 3;
                if current_words.len() > last_n * 2 {
                    let tail = &current_words[current_words.len()-last_n..];
                    let prev = &current_words[current_words.len()-(last_n*2)..current_words.len()-last_n];
                    if tail == prev {
                        self.is_generating = false;
                        self.tokens_to_generate = 0;
                        self.transcript.push_str(" [STALL-VETO]");
                    }
                }
            }
        }

        // scroll_pos is computed in ui() when auto_scroll is true

        if self.tokens_to_generate == 0 {
            self.is_generating = false;
        }

        // Keep transcript under 100KB — trim from the front, preserving whole lines.
        const MAX_TRANSCRIPT_BYTES: usize = 100 * 1024;
        if self.transcript.len() > MAX_TRANSCRIPT_BYTES {
            let overflow = self.transcript.len() - MAX_TRANSCRIPT_BYTES;
            if let Some(nl) = self.transcript[overflow..].find('\n') {
                self.transcript = format!("[...trimmed...]\n{}", &self.transcript[overflow + nl + 1..]);
            }
        }
    }
}

fn find_latest_checkpoint() -> (PathBuf, String) {
    let models_dir = "models";
    let search_paths = [
        ("v2.0.0", format!("{}/bible_ternary_v2.0.0.safetensors", models_dir)),
        ("v1.3.7", format!("{}/registry/bible_ternary_v1.3.7.safetensors", models_dir)),
        ("v1.3.7", format!("{}/bible_ternary_v1.3.7.safetensors", models_dir)),
        ("v1.3.6", format!("{}/bible_ternary_v1.3.6.safetensors", models_dir)),
        ("v1.3.5", format!("{}/bible_ternary_v1.3.5.safetensors", models_dir)),
    ];

    for (version, path_str) in search_paths {
        let path = PathBuf::from(path_str);
        if path.exists() {
            return (path, version.to_string());
        }
    }
    panic!("No checkpoints found in models/ or models/registry/. Start training first!");
}

/// Eval mode: compute avg cross-entropy on a text file, print AVG_LOSS for eval_perplexity.py.
/// Usage: moe-test --eval <text_file> [--checkpoint <path>]
fn run_eval_mode(text_path: &str, checkpoint_override: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let dev = Device::Cpu;
    let vocab_path = "data/vocab.json";
    let tokenizer = BpeTokenizer::new(vocab_path);

    let (default_ckpt, version) = find_latest_checkpoint();
    let checkpoint_path = checkpoint_override
        .map(std::path::PathBuf::from)
        .unwrap_or(default_ckpt);

    let mut config_path = format!("models/bible_ternary_{}.config.json", version);
    if !std::path::Path::new(&config_path).exists() {
        config_path = "models/bible_ternary_v2.0.0.config.json".to_string();
    }
    let config_str = fs::read_to_string(&config_path)?;
    let config_json: Value = serde_json::from_str(&config_str)?;

    let mut config = moe_llm_core::model::TransformerConfig::default();
    config.vocab_size    = tokenizer.vocab_size();
    config.hidden_size   = config_json["hidden_size"].as_u64().unwrap_or(256) as usize;
    config.num_layers    = config_json["num_layers"].as_u64().unwrap_or(7)   as usize;
    config.num_heads     = config_json["num_heads"].as_u64().unwrap_or(8)    as usize;
    config.max_seq_len   = config_json["max_seq_len"].as_u64().unwrap_or(128) as usize;
    config.num_experts   = config_json["num_experts"].as_u64().unwrap_or(12) as usize;

    let varmap = candle_nn::VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
    let model = Transformer::new(&config, vb)?;

    let ckpt_data = candle_core::safetensors::load(&checkpoint_path, &dev)?;
    let all_vars = varmap.data().lock().unwrap();
    for (name, var) in all_vars.iter() {
        if let Some(t) = ckpt_data.get(name) { let _ = var.set(t); }
    }
    drop(all_vars);
    model.prepare_inference()?;

    let text = fs::read_to_string(text_path)?;
    let tokens = tokenizer.encode(&text);
    let ctx = config.max_seq_len;

    eprintln!("Eval: {} tokens, context={}, checkpoint={}", tokens.len(), ctx, checkpoint_path.display());

    let mut total_loss = 0.0f64;
    let mut count = 0usize;

    // Slide a non-overlapping window of size ctx over the token sequence.
    // Non-overlapping (step=ctx) is standard for LM perplexity and fastest on CPU.
    let windows: Vec<&[u32]> = tokens.windows(ctx + 1)
        .step_by(ctx)
        .collect();

    for window in &windows {
        let input_ids = &window[..ctx];
        let target_ids = &window[1..=ctx];

        let input_t = Tensor::from_slice(input_ids, (1, ctx), &dev)?
            .to_dtype(DType::U32)?;
        let logits = model.forward(&input_t)?; // [1, ctx, vocab]

        // Cross-entropy over all positions in window
        let vocab = config.vocab_size;
        let logits_2d = logits.squeeze(0)?; // [ctx, vocab]

        for (pos, &target) in target_ids.iter().enumerate() {
            let logit_row = logits_2d.narrow(0, pos, 1)?.squeeze(0)?; // [vocab]
            let log_probs = candle_nn::ops::log_softmax(&logit_row, 0)?;
            let lp = log_probs.to_vec1::<f32>()?;
            if (target as usize) < vocab {
                total_loss += -(lp[target as usize] as f64);
                count += 1;
            }
        }
    }

    let avg_loss = if count > 0 { total_loss / count as f64 } else { f64::NAN };
    println!("AVG_LOSS: {:.6}", avg_loss);
    println!("TOKENS_EVALUATED: {}", count);
    println!("PERPLEXITY: {:.2}", avg_loss.exp());
    Ok(())
}

struct LoadedModel {
    model: Transformer,
    tokenizer: BpeTokenizer,
    config: moe_llm_core::model::TransformerConfig,
    version: String,
}

fn load_model() -> Result<LoadedModel, Box<dyn std::error::Error>> {
    let dev = Device::Cpu;
    let tokenizer = BpeTokenizer::new("data/vocab.json");
    let (checkpoint_path, version) = find_latest_checkpoint();

    let mut config_path = format!("models/bible_ternary_{}.config.json", version);
    if !std::path::Path::new(&config_path).exists() {
        config_path = "models/bible_ternary_v2.0.0.config.json".to_string();
    }
    let config_str = fs::read_to_string(&config_path)?;
    let config_json: Value = serde_json::from_str(&config_str)?;

    let mut config = moe_llm_core::model::TransformerConfig::default();
    config.vocab_size  = tokenizer.vocab_size();
    config.hidden_size = config_json["hidden_size"].as_u64().unwrap_or(256) as usize;
    config.num_layers  = config_json["num_layers"].as_u64().unwrap_or(5)   as usize;
    config.num_heads   = config_json["num_heads"].as_u64().unwrap_or(8)    as usize;
    config.max_seq_len = config_json["max_seq_len"].as_u64().unwrap_or(128) as usize;
    config.num_experts = config_json["num_experts"].as_u64().unwrap_or(12) as usize;

    let varmap = candle_nn::VarMap::new();
    let vb = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
    let model = Transformer::new(&config, vb)?;

    let ckpt_data = candle_core::safetensors::load(&checkpoint_path, &dev)?;
    let all_vars = varmap.data().lock().unwrap();
    for (name, var) in all_vars.iter() {
        if let Some(t) = ckpt_data.get(name) { let _ = var.set(t); }
    }
    drop(all_vars);
    model.prepare_inference()?;

    Ok(LoadedModel { model, tokenizer, config, version })
}

fn perplexity_on_text(
    lm: &LoadedModel,
    text: &str,
) -> Result<(f64, usize), Box<dyn std::error::Error>> {
    let dev = Device::Cpu;
    let tokens = lm.tokenizer.encode(text);
    let ctx = lm.config.max_seq_len;
    let mut total_loss = 0.0f64;
    let mut count = 0usize;

    let windows: Vec<&[u32]> = tokens.windows(ctx + 1).step_by(ctx).collect();
    for window in &windows {
        let input_ids = &window[..ctx];
        let target_ids = &window[1..=ctx];
        let input_t = Tensor::from_slice(input_ids, (1, ctx), &dev)?.to_dtype(DType::U32)?;
        let logits = lm.model.forward(&input_t)?;
        let logits_2d = logits.squeeze(0)?;
        for (pos, &target) in target_ids.iter().enumerate() {
            let logit_row = logits_2d.narrow(0, pos, 1)?.squeeze(0)?;
            let log_probs = candle_nn::ops::log_softmax(&logit_row, 0)?;
            let lp = log_probs.to_vec1::<f32>()?;
            if (target as usize) < lm.config.vocab_size {
                total_loss += -(lp[target as usize] as f64);
                count += 1;
            }
        }
    }
    let avg_loss = if count > 0 { total_loss / count as f64 } else { f64::NAN };
    Ok((avg_loss, count))
}

/// Full benchmark suite: speed + @sparseskip + perplexity. Exits after printing results.
/// Usage: moe-test --bench [--csv <path>]
fn run_bench_mode(csv_path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let dev = Device::Cpu;
    let platform = format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH);

    eprintln!("Loading Albert MoE-13...");
    let lm = load_model()?;
    let c = &lm.config;
    let top_k = 3usize;

    let sep = "━".repeat(60);
    println!();
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║        Albert MoE-13 — TIS Benchmark Suite {}          ║", lm.version);
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("  Architecture : {}L × {}E × {}H  BitNet 1.58b", c.num_layers, c.num_experts, c.hidden_size);
    println!("  Routing      : @sparseskip Top-{}/{}", top_k, c.num_experts);
    println!("  Checkpoint   : bible_ternary_{}", lm.version);
    println!("  Platform     : {}", platform);

    // ── [1/3] Inference Speed ───────────────────────────────────────────
    println!("\n{}", sep);
    println!("[1/3] INFERENCE SPEED");
    println!("{}", sep);

    let bench_prompt = "In the beginning God created the heaven and the earth";
    let prompt_tokens = lm.tokenizer.encode(bench_prompt);
    let n_gen = 64usize;

    lm.model.clear_kv_cache();
    let input_t = Tensor::new(prompt_tokens.as_slice(), &dev)?
        .unsqueeze(0)?.to_dtype(DType::U32)?;
    let logits = lm.model.forward_prefill(&input_t)?;
    let dims = logits.dims();
    let last_logits = logits.i((0, dims[1] - 1))?;
    let pr = candle_nn::ops::softmax(&last_logits, 0)?.to_vec1::<f32>()?;
    let first_token = pr.iter().enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(i, _)| i as u32).unwrap();

    let mut current_tokens = prompt_tokens.clone();
    current_tokens.push(first_token);
    let mut kv_pos = prompt_tokens.len();

    let t_start = Instant::now();
    for _ in 0..n_gen {
        let last = *current_tokens.last().unwrap();
        let out = lm.model.forward_decode(last, kv_pos, &dev)?;
        kv_pos += 1;
        let row = out.i((0, 0))?;
        let p = candle_nn::ops::softmax(&row, 0)?.to_vec1::<f32>()?;
        let next = p.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i as u32).unwrap();
        current_tokens.push(next);
    }
    let elapsed = t_start.elapsed();
    let tps = n_gen as f64 / elapsed.as_secs_f64();
    let lat_ms = elapsed.as_millis() as f64 / n_gen as f64;
    let gflops = (tps * 2_000_000.0) / 1_000_000_000.0;
    let output_preview = {
        let decoded = lm.tokenizer.decode(&current_tokens[prompt_tokens.len()..]);
        if decoded.len() > 80 { format!("{}…", &decoded[..80]) } else { decoded }
    };

    println!("  Prompt    : \"{}\"", bench_prompt);
    println!("  Generated : {} tokens", n_gen);
    println!("  Speed     : {:.1} tok/s", tps);
    println!("  Latency   : {:.1} ms/tok", lat_ms);
    println!("  GFLOPS    : {:.4}", gflops);
    println!("  Output    : \"{}\"", output_preview);

    // ── [2/3] @sparseskip Routing Analysis ─────────────────────────────
    println!("\n{}", sep);
    println!("[2/3] @SPARSESKIP ROUTING ANALYSIS");
    println!("{}", sep);

    let skipped = c.num_experts - top_k;
    let skip_rate = skipped as f64 / c.num_experts as f64;
    let compute_pct = (top_k as f64 / c.num_experts as f64) * 100.0;

    println!("  Active experts / decode step : {} / {}", top_k, c.num_experts);
    println!("  Skipped experts / step       : {} / {}  ({:.0}%)", skipped, c.num_experts, skip_rate * 100.0);
    println!("  Compute vs. dense MoE        : {:.0}% ({} of {} experts executed)", compute_pct, top_k, c.num_experts);
    println!("  Patent reference             : A50296/2026 (pending)");

    // ── [3/3] Perplexity ───────────────────────────────────────────────
    println!("\n{}", sep);
    println!("[3/3] PERPLEXITY  (eval_sample.txt)");
    println!("{}", sep);

    let eval_path = "eval_sample.txt";
    let (avg_loss, tokens_evaluated) = if std::path::Path::new(eval_path).exists() {
        eprint!("  Evaluating...");
        let text = fs::read_to_string(eval_path)?;
        let result = perplexity_on_text(&lm, &text)?;
        eprintln!(" done.");
        result
    } else {
        println!("  eval_sample.txt not found — skipping perplexity");
        (f64::NAN, 0)
    };
    let ppl = avg_loss.exp();

    if !avg_loss.is_nan() {
        println!("  Tokens evaluated  : {}", tokens_evaluated);
        println!("  Avg cross-entropy : {:.4}", avg_loss);
        println!("  Perplexity        : {:.2}", ppl);
    }

    // ── Summary ─────────────────────────────────────────────────────────
    println!("\n{}", sep);
    println!("SUMMARY");
    println!("{}", sep);
    println!("  Tokens/sec   : {:.1}", tps);
    println!("  Latency      : {:.1} ms/tok", lat_ms);
    println!("  Perplexity   : {}", if avg_loss.is_nan() { "n/a".to_string() } else { format!("{:.2}", ppl) });
    println!("  Skip rate    : {:.0}%  ({} of {} experts skipped per step)", skip_rate * 100.0, skipped, c.num_experts);
    println!("  Model size   : {}L × {}E × {}H  ({} params)", c.num_layers, c.num_experts, c.hidden_size,
        c.num_layers * c.num_experts * c.hidden_size * c.hidden_size * 3 / 1_000_000);

    // ── CSV Export ──────────────────────────────────────────────────────
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let csv_out = csv_path.unwrap_or("albert_bench_results.csv");
    let header = "timestamp,model,version,arch,layers,experts,hidden,top_k,tokens_per_sec,latency_ms,gflops,avg_loss,perplexity,tokens_evaluated,skip_rate,platform";
    let row = format!(
        "{},{},{},{}L×{}E×{}H,{},{},{},{},{:.1},{:.1},{:.4},{},{},{},{:.3},{}",
        now, "albert-moe-13", lm.version,
        c.num_layers, c.num_experts, c.hidden_size,
        c.num_layers, c.num_experts, c.hidden_size, top_k,
        tps, lat_ms, gflops,
        if avg_loss.is_nan() { "".to_string() } else { format!("{:.4}", avg_loss) },
        if ppl.is_nan() { "".to_string() } else { format!("{:.2}", ppl) },
        tokens_evaluated, skip_rate, platform,
    );
    let csv_content = if std::path::Path::new(csv_out).exists() {
        format!("{}\n", row)
    } else {
        format!("{}\n{}\n", header, row)
    };
    fs::write(csv_out, csv_content)?;
    println!("\n  CSV exported  : {}", csv_out);
    println!();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // --bench [--csv <path>]
    if args.iter().any(|a| a == "--bench") {
        let csv = args.iter().position(|a| a == "--csv")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.as_str());
        return run_bench_mode(csv);
    }

    // --eval <text_file> [--checkpoint <path>]
    if let Some(eval_idx) = args.iter().position(|a| a == "--eval") {
        let text_path = args.get(eval_idx + 1)
            .expect("--eval requires a file path argument");
        let checkpoint = args.iter().position(|a| a == "--checkpoint")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.as_str());
        return run_eval_mode(text_path, checkpoint);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = if app.is_generating { Duration::from_millis(1) } else { Duration::from_millis(10) };
        
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        if app.is_generating {
                            app.is_generating = false;
                            app.tokens_to_generate = 0;
                        } else {
                            return Ok(());
                        }
                    }
                    KeyCode::Char(c) => {
                        if !app.is_generating { app.input.push(c); }
                    }
                    KeyCode::Backspace => {
                        if !app.is_generating { app.input.pop(); }
                    }
                    KeyCode::Enter => {
                        if !app.is_generating { app.start_generation(); }
                    }
                    KeyCode::Up => {
                        app.scroll_pos = app.scroll_pos.saturating_sub(1);
                        app.auto_scroll = false;
                    }
                    KeyCode::Down => {
                        app.scroll_pos = app.scroll_pos.saturating_add(1);
                        app.auto_scroll = false;
                    }
                    KeyCode::PageUp => {
                        app.scroll_pos = app.scroll_pos.saturating_sub(10);
                        app.auto_scroll = false;
                    }
                    KeyCode::PageDown => {
                        app.scroll_pos = app.scroll_pos.saturating_add(10);
                        app.auto_scroll = false;
                    }
                    KeyCode::End => {
                        app.auto_scroll = true;
                        app.scroll_pos = app.transcript.lines().count() as u16;
                    }
                    _ => {}
                }
            }
        }

        if app.is_generating {
            app.step_generation();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(10),   // Sandbox
                Constraint::Length(3), // Input
                Constraint::Length(3), // Metrics Row 1
                Constraint::Length(3), // Metrics Row 2
            ]
            .as_ref(),
        )
        .split(area);

    let header = Paragraph::new(format!(" {} | Scientific Dashboard", app.model_id))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)))
        .style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(header, chunks[0]);

    // Compute scroll offset: when auto-following, anchor to the last line of content.
    // chunks[1] is the sandbox rect; inner height = total - 2 borders.
    let sandbox_inner_h = chunks[1].height.saturating_sub(2);
    let raw_line_count = app.transcript.lines().count() as u16;
    let scroll_offset = if app.auto_scroll {
        raw_line_count.saturating_sub(sandbox_inner_h)
    } else {
        app.scroll_pos
    };

    let scroll_text = if app.auto_scroll {
        " [AUTO-FOLLOW] ".to_string()
    } else {
        format!(" [SCROLL: {}] ", app.scroll_pos)
    };

    let sandbox = Paragraph::new(app.transcript.as_str())
        .block(Block::default().borders(Borders::ALL).title(format!(" Sandbox @ Simeon{} ", scroll_text)))
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true })
        .scroll((scroll_offset, 0));
    f.render_widget(sandbox, chunks[1]);

    let input_title = if app.is_generating {
        " [THINKING...] (Esc to Stop) "
    } else {
        " Type & Enter to send  |  ↑↓ PgUp/Dn to scroll  |  End = follow bottom  |  Esc = quit "
    };
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title(input_title));
    f.render_widget(input, chunks[2]);
    
    if !app.is_generating {
        f.set_cursor_position((
            chunks[2].x + app.input.len() as u16 + 1,
            chunks[2].y + 1,
        ));
    }

    let m1_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
        .split(chunks[3]);

    f.render_widget(Paragraph::new(format!(" Brain: {}", app.checkpoint)).block(Block::default().borders(Borders::ALL).title(" Identity ")), m1_chunks[0]);
    f.render_widget(Paragraph::new(format!(" Speed: {:.2} tok/s", app.tokens_per_sec)).block(Block::default().borders(Borders::ALL).title(" Performance ")), m1_chunks[1]);
    f.render_widget(Paragraph::new(format!(" Experts: {}", app.active_experts)).block(Block::default().borders(Borders::ALL).title(" MoE Load ")), m1_chunks[2]);

    let m2_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)])
        .split(chunks[4]);

    f.render_widget(Paragraph::new(format!(" Mileage: {} Epochs", app.total_epochs)).block(Block::default().borders(Borders::ALL).title(" Total Experience ")), m2_chunks[0]);
    f.render_widget(Paragraph::new(format!(" Load: {:.4} GFLOPS", app.est_gflops)).block(Block::default().borders(Borders::ALL).title(" Intensity ")), m2_chunks[1]);
    f.render_widget(Paragraph::new(format!(" Latency: {}ms/tok", app.token_latency_ms)).block(Block::default().borders(Borders::ALL).title(" Depth ")), m2_chunks[2]);
}
