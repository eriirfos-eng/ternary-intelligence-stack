// albert_serve — HTTP inference server for albert.
//
// POST /generate  {"prompt": "...", "max_tokens": 64, "temperature": 0.9}
//              -> {"text": "...", "tok_s": 23.4, "epoch": 2946, "arch": "18L·256H·12E"}
// GET  /status -> {"epoch": 2946, "arch": "18L·256H·12E·Top3·256CTX·32kV", "ready": true}
//
// Run locally:
//   cargo run --release --bin albert_serve --features serve -p moe-llm-core -- .
//
// The single positional argument is the model root directory (default ".").
// Expects:
//   <root>/models/albert_v3.0.config.json  (or albert_v3.0.best.safetensors first)
//   <root>/data/vocab_v3.json

#![cfg(feature = "serve")]

use axum::{
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use candle_core::{DType, Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use moe_llm_core::model::{Transformer, TransformerConfig};
use moe_llm_core::tokenizer::BpeTokenizer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

// ─── shared state ────────────────────────────────────────────────────────────

struct ModelState {
    model:     Transformer,
    tokenizer: BpeTokenizer,
    config:    TransformerConfig,
    epoch:     u64,
    arch_str:  String,
}

type SharedState = Arc<Mutex<ModelState>>;

// ─── request / response types ─────────────────────────────────────────────────

#[derive(Deserialize)]
struct GenerateReq {
    prompt:      String,
    #[serde(default = "default_max_tokens")]
    max_tokens:  usize,
    #[serde(default = "default_temperature")]
    temperature: f64,
}
fn default_max_tokens()  -> usize { 128 }
fn default_temperature() -> f64   { 0.9 }

#[derive(Serialize)]
struct GenerateResp {
    text:   String,
    tok_s:  f64,
    epoch:  u64,
    arch:   String,
}

#[derive(Serialize)]
struct StatusResp {
    ready: bool,
    epoch: u64,
    arch:  String,
}

// ─── sampling ────────────────────────────────────────────────────────────────

fn sample_token(logits: &Tensor, temperature: f64) -> candle_core::Result<u32> {
    if temperature <= 0.0 {
        // greedy
        let idx = logits.argmax(0)?.to_scalar::<u32>()?;
        return Ok(idx);
    }
    let scaled = (logits / temperature)?;
    // softmax
    let max_val = scaled.max(0)?;
    let shifted = scaled.broadcast_sub(&max_val)?;
    let exp     = shifted.exp()?;
    let sum     = exp.sum_all()?;
    let probs   = exp.broadcast_div(&sum)?;
    let probs_v: Vec<f32> = probs.to_vec1()?;

    let r: f32 = rand::random();
    let mut cum = 0.0f32;
    for (i, &p) in probs_v.iter().enumerate() {
        cum += p;
        if r < cum {
            return Ok(i as u32);
        }
    }
    Ok((probs_v.len() - 1) as u32)
}

// ─── generate ────────────────────────────────────────────────────────────────

fn generate(state: &ModelState, prompt: &str, max_new: usize, temperature: f64)
    -> (String, f64)
{
    let device = Device::Cpu;
    let prompt_ids = state.tokenizer.encode(prompt);
    if prompt_ids.is_empty() {
        return (String::new(), 0.0);
    }

    // Prefill: process full prompt once, populate KV-cache.
    state.model.clear_kv_cache();
    let prompt_tensor = Tensor::new(prompt_ids.as_slice(), &device).unwrap()
        .to_dtype(DType::U32).unwrap()
        .unsqueeze(0).unwrap();
    let prefill_logits = state.model.forward_prefill(&prompt_tensor).unwrap();
    let dims  = prefill_logits.dims();
    let last  = prefill_logits.i((0, dims[1] - 1)).unwrap();
    let first = sample_token(&last, temperature).unwrap();

    let t0 = std::time::Instant::now();
    let mut new_tokens: Vec<u32> = vec![first];

    // Decode: single-token steps, O(1) per step via KV-cache.
    for _ in 1..max_new {
        let logits = state.model.forward_decode(*new_tokens.last().unwrap(), &device).unwrap();
        let last   = logits.i((0, 0)).unwrap(); // [1, 1, V] → [V]
        let next   = sample_token(&last, temperature).unwrap();
        new_tokens.push(next);
    }

    let elapsed = t0.elapsed().as_secs_f64();
    let tok_s   = (max_new - 1).max(1) as f64 / elapsed;

    let mut all_tokens = prompt_ids;
    all_tokens.extend_from_slice(&new_tokens);
    (state.tokenizer.decode(&all_tokens), tok_s)
}

// ─── handlers ────────────────────────────────────────────────────────────────

async fn status(State(state): State<SharedState>) -> Json<StatusResp> {
    let s = state.lock().await;
    Json(StatusResp { ready: true, epoch: s.epoch, arch: s.arch_str.clone() })
}

async fn generate_handler(
    State(state): State<SharedState>,
    Json(req): Json<GenerateReq>,
) -> impl IntoResponse {
    if req.prompt.trim().is_empty() {
        return (StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "prompt is empty"}))).into_response();
    }
    let max_tokens  = req.max_tokens.min(256);
    let temperature = req.temperature.clamp(0.0, 2.0);

    let s = state.lock().await;
    let (text, tok_s) = generate(&s, &req.prompt, max_tokens, temperature);
    Json(GenerateResp {
        text,
        tok_s,
        epoch: s.epoch,
        arch:  s.arch_str.clone(),
    }).into_response()
}

// ─── main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let root = args.first().cloned().unwrap_or_else(|| ".".to_string());

    let config_path = format!("{root}/models/albert_v3.0.config.json");
    let vocab_path  = format!("{root}/data/vocab_v3.json");
    let ckpt_best   = format!("{root}/models/albert_v3.0.best.safetensors");
    let ckpt_latest = format!("{root}/models/albert_v3.0.safetensors");
    // Serve the LATEST checkpoint — it always matches config.json's architecture.
    // best.safetensors is the lowest-LOSS snapshot, which can lag the architecture
    // across surgeries (a pre-cord single-stream "best" vs a dual-stream config) and
    // would fail to load. Latest is written with the current model, so it's safe.
    let ckpt        = if std::path::Path::new(&ckpt_latest).exists() { &ckpt_latest } else { &ckpt_best };

    println!("[albert_serve] loading config  {config_path}");
    let config_str  = fs::read_to_string(&config_path)?;
    let config_json: Value = serde_json::from_str(&config_str)?;

    let mut config  = TransformerConfig::default();
    config.num_layers  = config_json["num_layers"].as_u64().unwrap_or(18) as usize;
    config.hidden_size = config_json["hidden_size"].as_u64().unwrap_or(256) as usize;
    config.num_heads   = config_json["num_heads"].as_u64().unwrap_or(4) as usize;
    config.num_experts = config_json["num_experts"].as_u64().unwrap_or(12) as usize;
    config.max_seq_len = config_json["max_seq_len"].as_u64().unwrap_or(256) as usize;
    // Dual-stream (cord surgery): build both attention hemispheres + anastomosis
    // fusion gates. Without these the checkpoint's stream_a/stream_b/anastomosis
    // tensors have no home and weight-load fails. forward_prefill/forward_decode
    // already handle the fusion when num_streams >= 2.
    config.num_streams = config_json["num_streams"].as_u64().unwrap_or(1) as usize;
    config.fusion_layers = config_json["fusion_layers"].as_array()
        .map(|a| a.iter().filter_map(|v| v.as_u64().map(|n| n as usize)).collect())
        .unwrap_or_default();
    let epoch          = config_json["global_epoch"].as_u64().unwrap_or(0);

    println!("[albert_serve] loading vocab   {vocab_path}");
    let tokenizer = BpeTokenizer::new(&vocab_path);
    config.vocab_size = tokenizer.vocab_size();

    println!("[albert_serve] loading weights {ckpt}");
    let device  = Device::Cpu;
    let tensors = candle_core::safetensors::load(ckpt, &device)?;
    let vb      = VarBuilder::from_tensors(tensors, DType::F32, &device);
    let model   = Transformer::new(&config, vb)?;
    model.prepare_inference()?;

    let arch_str = format!(
        "{}L·{}H·{}E·Top3·{}CTX·{}kV",
        config.num_layers, config.hidden_size, config.num_experts,
        config.max_seq_len, config.vocab_size / 1000
    );
    println!("[albert_serve] ready  arch={arch_str}  epoch={epoch}");

    let shared = Arc::new(Mutex::new(ModelState { model, tokenizer, config, epoch, arch_str }));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/status",   get(status))
        .route("/generate", post(generate_handler))
        .with_state(shared)
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{port}");
    println!("[albert_serve] listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
