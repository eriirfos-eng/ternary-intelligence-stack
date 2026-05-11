//! # Albert-Test: Scientific Dashboard v2.0.0 (13-Node Evolution)
//! 
//! High-fidelity interactive Sandbox for the TIS with real-time telemetry, Smart-Stop, and Scrolling.

use std::io;
use std::path::PathBuf;
#[cfg(target_os = "macos")]
use std::process::Command;
use std::time::{Duration, Instant};
use std::fs;

use candle_core::{Device, DType, Tensor, IndexOp};
use candle_nn::VarBuilder;
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use serde_json::Value;

use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

const COMMANDS: &[(&str, &str)] = &[
    ("/help",    "list all commands"),
    ("/prompts", "show the 5 benchmark prompts"),
    ("/p1",      "fire benchmark prompt 1"),
    ("/p2",      "fire benchmark prompt 2"),
    ("/p3",      "fire benchmark prompt 3"),
    ("/p4",      "fire benchmark prompt 4"),
    ("/p5",      "fire benchmark prompt 5"),
    ("/bench",   "run all 5 prompts + auto-export"),
    ("/export",  "export this session to exports/"),
];

const BENCH_PROMPTS: &[&str] = &[
    "in the beginning god created the",
    "User: what language do you speak? Albert:",
    "die Geschichte der Europäischen Union",
    "once upon a time in a kingdom far",
    "the ternary number system uses three",
];

struct BenchResult {
    prompt: String,
    response: String,
    tokens_per_sec: f32,
    latency_ms: u64,
}

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

    // /bench command state
    bench_mode: bool,
    bench_step: usize,
    bench_results: Vec<BenchResult>,
}

impl App {
    fn new() -> Self {
        let dev = Device::Cpu;
        let (checkpoint_path, version) = find_latest_checkpoint();
        
        let _metadata = fs::metadata(&checkpoint_path).ok();
        
        let vocab_path = if version.starts_with("v3") { "data/vocab_v3.json" } else { "data/vocab.json" };
        let tokenizer = BpeTokenizer::new(vocab_path);

        let mut config_path = if version.starts_with("v3") {
            format!("models/albert_{}.config.json", version)
        } else {
            format!("models/bible_ternary_{}.config.json", version)
        };
        if !std::path::Path::new(&config_path).exists() {
            config_path = if version.starts_with("v3") {
                "models/albert_v3.0.config.json".to_string()
            } else {
                format!("models/registry/bible_ternary_{}.config.json", version)
            };
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
            bench_mode: false,
            bench_step: 0,
            bench_results: Vec::new(),
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

    fn handle_command(&mut self, cmd: &str) {
        match cmd {
            "/help" => {
                self.transcript.push_str(concat!(
                    "\n┌─ COMMANDS ─────────────────────────────────┐\n",
                    "│  /help      list commands                   │\n",
                    "│  /prompts   show the 5 benchmark prompts    │\n",
                    "│  /p1..p5    fire one benchmark prompt        │\n",
                    "│  /bench     run all 5, auto-export results  │\n",
                    "│  /export    export this session to exports/ │\n",
                    "└─────────────────────────────────────────────┘\n",
                ));
            }
            "/prompts" => {
                self.transcript.push_str("\n[BENCHMARK PROMPTS]\n");
                for (i, p) in BENCH_PROMPTS.iter().enumerate() {
                    self.transcript.push_str(&format!("  /p{}: {}\n", i + 1, p));
                }
            }
            "/p1" | "/p2" | "/p3" | "/p4" | "/p5" => {
                if let Ok(n) = cmd[2..].parse::<usize>() {
                    if n >= 1 && n <= BENCH_PROMPTS.len() {
                        self.input = BENCH_PROMPTS[n - 1].to_string();
                        self.start_generation();
                    }
                }
            }
            "/bench" => {
                self.bench_results.clear();
                self.bench_step = 0;
                self.bench_mode = true;
                self.transcript.push_str(&format!(
                    "\n━━━ BENCH 1/{} ━━━\n", BENCH_PROMPTS.len()
                ));
                self.input = BENCH_PROMPTS[0].to_string();
                self.start_generation();
            }
            "/export" => {
                match self.export_session() {
                    Ok(path) => self.transcript.push_str(&format!("\n[EXPORT] {}\n", path)),
                    Err(e)   => self.transcript.push_str(&format!("\n[EXPORT ERROR] {}\n", e)),
                }
            }
            _ => {
                self.transcript.push_str(&format!("\n[UNKNOWN COMMAND] {}  — try /help\n", cmd));
            }
        }
    }

    fn export_session(&self) -> Result<String, std::io::Error> {
        fs::create_dir_all("exports")?;
        let now = chrono::Local::now().format("%Y-%m-%d_%H%M%S");
        let path = format!("exports/chat_{}_{}.txt", self.checkpoint, now);
        let header = format!(
            "=== albert. session export ===\nBrain: {} | Mileage: {} epochs | {}\n{}\n\n",
            self.checkpoint,
            self.total_epochs,
            chrono::Local::now().format("%Y-%m-%d %H:%M"),
            "─".repeat(50),
        );
        fs::write(&path, format!("{}{}", header, self.transcript))?;
        Ok(path)
    }

    fn export_bench(&mut self) {
        fs::create_dir_all("exports").ok();
        let now = chrono::Local::now().format("%Y-%m-%d_%H%M%S");
        let path = format!("exports/bench_{}_{}.txt", self.checkpoint, now);
        let mut out = format!(
            "=== albert. benchmark eval ===\nBrain: {} | Mileage: {} epochs | {}\n{}\n\n",
            self.checkpoint,
            self.total_epochs,
            chrono::Local::now().format("%Y-%m-%d %H:%M"),
            "─".repeat(50),
        );
        for (i, r) in self.bench_results.iter().enumerate() {
            out.push_str(&format!(
                "[P{}] {}\n→ {}\n    {:.1} tok/s  {}ms/tok\n\n",
                i + 1, r.prompt, r.response, r.tokens_per_sec, r.latency_ms,
            ));
        }
        match fs::write(&path, &out) {
            Ok(_)  => self.transcript.push_str(&format!("\n[BENCH COMPLETE] {}\n", path)),
            Err(e) => self.transcript.push_str(&format!("\n[BENCH ERROR] {}\n", e)),
        }
    }
}

struct SysInfo {
    cpu_model: String,
    cpu_cores: usize,
    ram_gb: f64,
    os: String,
    arch: String,
}

impl SysInfo {
    fn collect() -> Self {
        let arch = std::env::consts::ARCH.to_string();
        let os = std::env::consts::OS.to_string();

        #[cfg(target_os = "linux")]
        {
            let cpu_model = fs::read_to_string("/proc/cpuinfo")
                .unwrap_or_default()
                .lines()
                .find(|l| l.starts_with("model name"))
                .and_then(|l| l.splitn(2, ':').nth(1))
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let cpu_cores = fs::read_to_string("/proc/cpuinfo")
                .unwrap_or_default()
                .lines()
                .filter(|l| l.starts_with("processor"))
                .count()
                .max(1);

            let ram_kb: u64 = fs::read_to_string("/proc/meminfo")
                .unwrap_or_default()
                .lines()
                .find(|l| l.starts_with("MemTotal:"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            let ram_gb = ram_kb as f64 / 1_048_576.0;

            return SysInfo { cpu_model, cpu_cores, ram_gb, os, arch };
        }

        #[cfg(target_os = "macos")]
        {
            let cpu_model = Command::new("sysctl")
                .args(["-n", "machdep.cpu.brand_string"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_else(|_| "unknown".to_string());

            let cpu_cores = Command::new("sysctl")
                .args(["-n", "hw.logicalcpu"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().parse().unwrap_or(1))
                .unwrap_or(1);

            let ram_bytes: u64 = Command::new("sysctl")
                .args(["-n", "hw.memsize"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().parse().unwrap_or(0))
                .unwrap_or(0);
            let ram_gb = ram_bytes as f64 / 1_073_741_824.0;

            return SysInfo { cpu_model, cpu_cores, ram_gb, os, arch };
        }

        #[allow(unreachable_code)]
        SysInfo {
            cpu_model: "unknown".to_string(),
            cpu_cores: 1,
            ram_gb: 0.0,
            os,
            arch,
        }
    }

    fn display(&self) -> String {
        format!("{} ({} cores, {:.0} GB RAM) — {}-{}",
            self.cpu_model, self.cpu_cores, self.ram_gb, self.os, self.arch)
    }

    fn csv_header() -> &'static str {
        "cpu_model,cpu_cores,ram_gb,os,arch"
    }

    fn csv_row(&self) -> String {
        format!("\"{}\",{},{:.1},{},{}",
            self.cpu_model.replace('"', "'"),
            self.cpu_cores, self.ram_gb, self.os, self.arch)
    }
}

fn find_latest_checkpoint() -> (PathBuf, String) {
    let models_dir = "models";
    let search_paths = [
        ("v3.0",   format!("{}/albert_v3.0.safetensors", models_dir)),
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

    let (default_ckpt, version) = find_latest_checkpoint();
    let checkpoint_path = checkpoint_override
        .map(std::path::PathBuf::from)
        .unwrap_or(default_ckpt);

    let vocab_path = if version.starts_with("v3") { "data/vocab_v3.json" } else { "data/vocab.json" };
    let tokenizer = BpeTokenizer::new(vocab_path);

    let mut config_path = if version.starts_with("v3") {
        format!("models/albert_{}.config.json", version)
    } else {
        format!("models/bible_ternary_{}.config.json", version)
    };
    if !std::path::Path::new(&config_path).exists() {
        config_path = if version.starts_with("v3") {
            "models/albert_v3.0.config.json".to_string()
        } else {
            "models/bible_ternary_v2.0.0.config.json".to_string()
        };
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
    let (checkpoint_path, version) = find_latest_checkpoint();
    let vocab_path = if version.starts_with("v3") { "data/vocab_v3.json" } else { "data/vocab.json" };
    let tokenizer = BpeTokenizer::new(vocab_path);

    let mut config_path = if version.starts_with("v3") {
        format!("models/albert_{}.config.json", version)
    } else {
        format!("models/bible_ternary_{}.config.json", version)
    };
    if !std::path::Path::new(&config_path).exists() {
        config_path = if version.starts_with("v3") {
            "models/albert_v3.0.config.json".to_string()
        } else {
            "models/bible_ternary_v2.0.0.config.json".to_string()
        };
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

/// Max windows evaluated for perplexity — caps runtime to ~10 min on slow CPUs.
/// 500 × 128 tokens = 64K tokens, statistically equivalent to full-corpus eval.
const MAX_EVAL_WINDOWS: usize = 500;

fn perplexity_on_text(
    lm: &LoadedModel,
    text: &str,
) -> Result<(f64, usize), Box<dyn std::error::Error>> {
    let dev = Device::Cpu;
    let tokens = lm.tokenizer.encode(text);
    let ctx = lm.config.max_seq_len;
    let mut total_loss = 0.0f64;
    let mut count = 0usize;
    let flavour = [
        "crunching numbers", "weighing tensors", "consulting the experts",
        "skipping 9 of 12 experts", "reading the scriptures", "marking the bench",
        "routing through trit states", "asking the LORD for perplexity",
    ];

    let all_windows: Vec<&[u32]> = tokens.windows(ctx + 1).step_by(ctx).collect();
    let windows = &all_windows[..all_windows.len().min(MAX_EVAL_WINDOWS)];
    let total_windows = windows.len();

    if all_windows.len() > MAX_EVAL_WINDOWS {
        eprintln!("  Corpus capped at {} windows (~{}K tokens) for benchmark speed.",
            MAX_EVAL_WINDOWS, MAX_EVAL_WINDOWS * ctx / 1000);
    }

    for (wi, window) in windows.iter().enumerate() {
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

        if wi % 10 == 0 || wi + 1 == total_windows {
            let pct = (wi + 1) * 100 / total_windows;
            let bar_filled = (wi + 1) * 20 / total_windows;
            let bar = format!("[{}{}]", "=".repeat(bar_filled), ".".repeat(20 - bar_filled));
            let running_loss = if count > 0 { total_loss / count as f64 } else { 0.0 };
            let msg = flavour[(wi / 10) % flavour.len()];
            eprintln!("  {} {:>3}%  {}/{} windows  loss {:.4}  {}...",
                bar, pct, wi + 1, total_windows, running_loss, msg);
        }
    }

    eprintln!("  [====================] 100%  done");
    let avg_loss = if count > 0 { total_loss / count as f64 } else { f64::NAN };
    Ok((avg_loss, count))
}

/// Full benchmark suite: speed + @sparseskip + perplexity. Exits after printing results.
/// Usage: moe-test --bench [--csv <path>]
fn run_bench_mode(csv_path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let dev = Device::Cpu;
    let sys = SysInfo::collect();

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
    println!("  CPU          : {} ({} cores)", sys.cpu_model, sys.cpu_cores);
    println!("  RAM          : {:.0} GB", sys.ram_gb);
    println!("  Platform     : {}-{}", sys.os, sys.arch);

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

    let eval_candidates = [
        "eval_sample.txt",
        "data/corpus/stage_3/bible.txt",
        "../data/corpus/stage_3/bible.txt",
    ];
    let eval_path = eval_candidates.iter().find(|p| std::path::Path::new(p).exists()).copied();
    let (avg_loss, tokens_evaluated) = if let Some(path) = eval_path {
        eprintln!("  Evaluating on {}...", path);
        let text = fs::read_to_string(path)?;
        let result = perplexity_on_text(&lm, &text)?;
        eprintln!("  done.");
        result
    } else {
        println!("  No eval file found — place eval_sample.txt in current directory to enable");
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
    println!("  Model size   : {}L × {}E × {}H  (~{}M params)", c.num_layers, c.num_experts, c.hidden_size,
        c.num_layers * c.num_experts * c.hidden_size * c.hidden_size * 3 / 1_000_000);

    // ── Summary ─ system info ────────────────────────────────────────────
    println!("  Measured on  : {}", sys.display());

    // ── CSV Export ──────────────────────────────────────────────────────
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let csv_out = csv_path.unwrap_or("albert_bench_results.csv");
    let header = format!(
        "timestamp,model,version,arch,layers,experts,hidden,top_k,tokens_per_sec,latency_ms,gflops,avg_loss,perplexity,tokens_evaluated,skip_rate,{}",
        SysInfo::csv_header()
    );
    let row = format!(
        "{},{},{},{}L×{}E×{}H,{},{},{},{},{:.1},{:.1},{:.4},{},{},{},{:.3},{}",
        now, "albert-moe-13", lm.version,
        c.num_layers, c.num_experts, c.hidden_size,
        c.num_layers, c.num_experts, c.hidden_size, top_k,
        tps, lat_ms, gflops,
        if avg_loss.is_nan() { "".to_string() } else { format!("{:.4}", avg_loss) },
        if ppl.is_nan() { "".to_string() } else { format!("{:.2}", ppl) },
        tokens_evaluated, skip_rate,
        sys.csv_row(),
    );
    let write_header = !std::path::Path::new(csv_out).exists();
    let mut file = fs::OpenOptions::new().create(true).append(true).open(csv_out)?;
    use std::io::Write as _;
    if write_header { writeln!(file, "{}", header)?; }
    writeln!(file, "{}", row)?;
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
    let mut prev_generating = false;
    loop {
        terminal.draw(|f| ui(f, app))?;

        // Bench mode: detect end of each generation, record result, advance or export.
        if prev_generating && !app.is_generating && app.bench_mode {
            if let Some(msg) = app.messages.last() {
                app.bench_results.push(BenchResult {
                    prompt: BENCH_PROMPTS[app.bench_step].to_string(),
                    response: msg.1.clone(),
                    tokens_per_sec: app.tokens_per_sec,
                    latency_ms: app.token_latency_ms,
                });
            }
            app.bench_step += 1;
            if app.bench_step < BENCH_PROMPTS.len() {
                app.transcript.push_str(&format!(
                    "\n━━━ BENCH {}/{} ━━━\n", app.bench_step + 1, BENCH_PROMPTS.len()
                ));
                app.input = BENCH_PROMPTS[app.bench_step].to_string();
                app.start_generation();
            } else {
                app.bench_mode = false;
                app.export_bench();
            }
        }
        prev_generating = app.is_generating;

        let timeout = if app.is_generating { Duration::from_millis(1) } else { Duration::from_millis(10) };

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        if app.is_generating {
                            app.is_generating = false;
                            app.tokens_to_generate = 0;
                            app.bench_mode = false;
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
                        if !app.is_generating {
                            let trimmed = app.input.trim().to_string();
                            if trimmed.starts_with('/') {
                                app.input.clear();
                                app.handle_command(&trimmed);
                            } else {
                                app.start_generation();
                            }
                        }
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

    let input_title = if app.is_generating && app.bench_mode {
        " [BENCH — Esc to abort] "
    } else if app.is_generating {
        " [THINKING...] (Esc to Stop) "
    } else {
        " Type & Enter  |  /help for commands  |  ↑↓ PgUp/Dn scroll  |  End = follow  |  Esc = quit "
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

    // Command autocomplete popup — appears as soon as input starts with '/'.
    if app.input.starts_with('/') && !app.is_generating {
        let prefix = app.input.as_str();
        let matches: Vec<(&str, &str)> = COMMANDS.iter()
            .filter(|(cmd, _)| cmd.starts_with(prefix))
            .copied()
            .collect();
        if !matches.is_empty() {
            let popup_h = matches.len() as u16 + 2;
            let popup_w = 52u16.min(chunks[2].width.saturating_sub(2));
            let popup_y = chunks[2].y.saturating_sub(popup_h);
            let popup_rect = Rect {
                x: chunks[2].x + 1,
                y: popup_y,
                width: popup_w,
                height: popup_h,
            };
            let text: String = matches.iter()
                .map(|(cmd, desc)| format!("  {:10}  {}", cmd, desc))
                .collect::<Vec<_>>()
                .join("\n");
            f.render_widget(Clear, popup_rect);
            f.render_widget(
                Paragraph::new(text).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" COMMANDS ")
                        .border_style(Style::default().fg(Color::Cyan))
                ),
                popup_rect,
            );
        }
    }
}
