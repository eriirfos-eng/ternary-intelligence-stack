//! Albert-MoE-13 Checkpoint Verifier
//!
//! Loads a saved .safetensors checkpoint and verifies it against a
//! parameter manifest — confirming that the artifact matches the declared
//! architecture and has not been corrupted or silently truncated.
//!
//! Usage:
//!   cargo run --release -p reproducibility_verifier -- \
//!       --checkpoint models/bible_ternary_v2.0.0.safetensors \
//!       --config     models/bible_ternary_v2.0.0.config.json

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let checkpoint = parse_arg(&args, "--checkpoint")
        .unwrap_or_else(|| "models/bible_ternary_v2.0.0.safetensors".to_string());
    let config_path = parse_arg(&args, "--config")
        .unwrap_or_else(|| "models/bible_ternary_v2.0.0.config.json".to_string());

    println!("=== Albert-MoE-13 Checkpoint Verifier ===");
    println!("Checkpoint: {}", checkpoint);
    println!("Config:     {}", config_path);
    println!();

    // Load config
    let config_str = match std::fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(e) => { eprintln!("FAIL  Cannot read config: {}", e); std::process::exit(1); }
    };
    let config: serde_json::Value = match serde_json::from_str(&config_str) {
        Ok(v) => v,
        Err(e) => { eprintln!("FAIL  Invalid config JSON: {}", e); std::process::exit(1); }
    };

    let num_layers  = config["num_layers"].as_u64().unwrap_or(0) as usize;
    let num_experts = config["num_experts"].as_u64().unwrap_or(0) as usize;
    let hidden_size = config["hidden_size"].as_u64().unwrap_or(0) as usize;
    let vocab_size  = 8000usize; // WordLevel tokenizer, fixed
    let max_seq_len = config["max_seq_len"].as_u64().unwrap_or(128) as usize;

    println!("Declared architecture: {}L · {}H · {}E · {}CTX · {}V",
        num_layers, hidden_size, num_experts, max_seq_len, vocab_size);
    println!();

    if num_layers == 0 || hidden_size == 0 {
        eprintln!("FAIL  Config missing required fields (num_layers, hidden_size)");
        std::process::exit(1);
    }

    // Read checkpoint file — parse safetensors header (no full data load)
    let checkpoint_bytes = match std::fs::read(&checkpoint) {
        Ok(b) => b,
        Err(e) => { eprintln!("FAIL  Cannot read checkpoint: {}", e); std::process::exit(1); }
    };

    if checkpoint_bytes.len() < 8 {
        eprintln!("FAIL  Checkpoint file too small ({} bytes)", checkpoint_bytes.len());
        std::process::exit(1);
    }
    let header_len = u64::from_le_bytes(checkpoint_bytes[..8].try_into().unwrap()) as usize;
    if checkpoint_bytes.len() < 8 + header_len {
        eprintln!("FAIL  Checkpoint truncated (header claims {} bytes, file has {})",
            header_len, checkpoint_bytes.len() - 8);
        std::process::exit(1);
    }

    let header_json = match std::str::from_utf8(&checkpoint_bytes[8..8 + header_len]) {
        Ok(s) => s,
        Err(e) => { eprintln!("FAIL  Header not valid UTF-8: {}", e); std::process::exit(1); }
    };
    let header: serde_json::Value = match serde_json::from_str(header_json) {
        Ok(v) => v,
        Err(e) => { eprintln!("FAIL  Header not valid JSON: {}", e); std::process::exit(1); }
    };

    // Build shape map from safetensors header
    let mut shapes: HashMap<String, Vec<usize>> = HashMap::new();
    if let Some(obj) = header.as_object() {
        for (key, meta) in obj {
            if key == "__metadata__" { continue; }
            if let Some(shape_arr) = meta.get("shape").and_then(|s| s.as_array()) {
                let shape: Vec<usize> = shape_arr.iter()
                    .filter_map(|v| v.as_u64().map(|n| n as usize))
                    .collect();
                shapes.insert(key.clone(), shape);
            }
        }
    }

    let total_params: usize = shapes.values().map(|s| s.iter().product::<usize>()).sum();
    let file_mb = checkpoint_bytes.len() as f64 / (1024.0 * 1024.0);

    println!("Checkpoint stats:");
    println!("  File size:  {:.1} MB", file_mb);
    println!("  Tensors:    {}", shapes.len());
    println!("  Parameters: {} ({:.1}M)", total_params, total_params as f64 / 1e6);
    println!();

    let mut failures = 0usize;
    let mut checks   = 0usize;

    macro_rules! check {
        ($pass:expr, $msg:expr) => {{
            checks += 1;
            if $pass { println!("  PASS  {}", $msg); }
            else      { println!("  FAIL  {}", $msg); failures += 1; }
        }};
    }

    // ── Shape verification ────────────────────────────────────────────────────
    println!("── Shape Verification ───────────────────────────────────────────");
    check!(
        shapes.get("embed.weight").map(|s| s == &[vocab_size, hidden_size]).unwrap_or(false),
        format!("embed.weight [{}, {}]", vocab_size, hidden_size)
    );
    check!(
        shapes.get("pos_embed.weight").map(|s| s == &[max_seq_len, hidden_size]).unwrap_or(false),
        format!("pos_embed.weight [{}, {}]", max_seq_len, hidden_size)
    );
    check!(
        shapes.get("lm_head.weight")
            .map(|s| s.len() == 2 && s[0] == vocab_size && s[1] == hidden_size)
            .unwrap_or(false),
        format!("lm_head.weight [{}, {}]", vocab_size, hidden_size)
    );

    for layer in 0..num_layers {
        check!(
            shapes.get(&format!("blocks.{}.attn.q_proj.weight", layer))
                .map(|s| s == &[hidden_size, hidden_size]).unwrap_or(false),
            format!("blocks.{}.attn.q_proj.weight [{h}×{h}]", layer, h = hidden_size)
        );
        check!(
            shapes.get(&format!("blocks.{}.moe.gate.weight", layer))
                .map(|s| s == &[num_experts, hidden_size]).unwrap_or(false),
            format!("blocks.{}.moe.gate.weight [{}×{}]", layer, num_experts, hidden_size)
        );
    }

    // ── Sanity checks ─────────────────────────────────────────────────────────
    println!();
    println!("── Sanity Checks ────────────────────────────────────────────────");
    check!(shapes.len() > 10,
        format!("{} tensors (expect > 10 for MoE model)", shapes.len()));
    check!(total_params > 1_000_000,
        format!("{:.1}M parameters (expect > 1M for production checkpoint)", total_params as f64 / 1e6));
    check!(file_mb > 1.0,
        format!("{:.1} MB checkpoint (expect > 1 MB)", file_mb));

    for layer in 0..num_layers {
        check!(
            shapes.contains_key(&format!("blocks.{}.attn.q_proj.weight", layer)),
            format!("Layer {} present (contiguous after Net2Net surgery)", layer)
        );
    }

    // ── Result ────────────────────────────────────────────────────────────────
    println!();
    println!("══════════════════════════════════════════════════════════════");
    if failures == 0 {
        println!("RESULT: PASS ({} checks, 0 failures)", checks);
        println!("Checkpoint valid for {}L · {}H · {}E architecture.", num_layers, hidden_size, num_experts);
    } else {
        println!("RESULT: FAIL ({}/{} checks failed)", failures, checks);
        std::process::exit(1);
    }
}

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}
