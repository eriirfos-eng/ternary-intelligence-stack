// ternlang-ml/src/bin/tritfloat_albert_bench.rs
//
// Validates TritFloat combined sparseskip using real albert. checkpoint weights.
//
// Loads blocks.{layer}.moe.experts.{expert}.c_fc/c_proj from albert_v3.0.safetensors,
// quantizes to TritMatrix, then runs linear_confident on realistic activations
// derived from the embedding table. Measures:
//   1. Real weight sparsity per layer (vs synthetic 33% assumption)
//   2. Natural confidence distribution in output (does it vary or collapse?)
//   3. Per-layer skip counts — combined vs weight-only, formula accuracy
//   4. Wall-clock: matmul_trit vs f32 dense at real albert. dims
//
// Run: cargo run --release -p ternlang-ml --bin tritfloat_albert_bench
// Optional: ALBERT_MODEL=<path> cargo run ...

use std::collections::HashMap;
use std::time::Instant;
use safetensors::SafeTensors;
use ternlang_ml::{TritFloatTensor, TritMatrix, linear_confident, bitnet_threshold};

// Default checkpoint location relative to the repo root
const DEFAULT_MODEL: &str =
    "/home/eri-irfos/projects/ternary-intelligence-stack/albert-moe-13/models/albert_v3.0.safetensors";

// albert. architecture constants from albert_v3.0.config.json
const HIDDEN: usize = 256;
const INNER:  usize = 1024;  // hidden * 4
const NUM_LAYERS:  usize = 18;
const NUM_EXPERTS: usize = 12;

// /p1-/p5 benchmark prompts (token sequences, approximated as token IDs from the
// albert tokenizer. These are representative content tokens, not exact — the
// embedding lookup gives realistic activation magnitudes without running the
// full tokenizer pipeline).
const BENCH_TOKEN_IDS: &[usize] = &[
    // /p1 — Genesis opening
    1, 259, 4688, 313, 338, 278, 1342, 310, 16887,  // "In the beginning God created"
    // /p2 — John 3:16
    1, 297, 278, 1494, 322, 278, 1494, 1360,          // "For God so loved the world"
    // /p3 — Psalm 23
    1, 450, 379, 338, 590, 17154,                      // "The Lord is my shepherd"
    // /p4 — Matthew 5 (Sermon)
    25849, 526, 278, 14821, 297, 23862,                // "Blessed are the meek in spirit"
    // /p5 — Revelation opening
    450, 1824, 310, 2486, 341,                         // "The revelation of Jesus"
];

fn sep(label: &str) {
    println!("\n{}", "─".repeat(70));
    println!("  {}", label);
    println!("{}", "─".repeat(70));
}

// Load all tensors from a safetensors file into f32 vecs.
fn load_safetensors(path: &str) -> HashMap<String, (Vec<f32>, Vec<usize>)> {
    let data = std::fs::read(path)
        .unwrap_or_else(|e| panic!("Cannot read {path}: {e}"));
    let st = SafeTensors::deserialize(&data)
        .unwrap_or_else(|e| panic!("Cannot parse safetensors {path}: {e}"));

    let mut out = HashMap::new();
    for (name, view) in st.tensors() {
        let shape: Vec<usize> = view.shape().to_vec();
        // All weights are stored as BF16 or F32 — read as raw bytes and convert.
        let raw = view.data();
        let dtype = view.dtype();
        let f32_data: Vec<f32> = match dtype {
            safetensors::Dtype::F32 => raw
                .chunks_exact(4)
                .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                .collect(),
            safetensors::Dtype::BF16 => raw
                .chunks_exact(2)
                .map(|b| {
                    let bf16_bits = u16::from_le_bytes([b[0], b[1]]);
                    f32::from_bits((bf16_bits as u32) << 16)
                })
                .collect(),
            safetensors::Dtype::F16 => raw
                .chunks_exact(2)
                .map(|b| {
                    let bits = u16::from_le_bytes([b[0], b[1]]);
                    half_f16_to_f32(bits)
                })
                .collect(),
            other => panic!("Unsupported dtype {other:?} for tensor {name}"),
        };
        out.insert(name.to_string(), (f32_data, shape));
    }
    out
}

fn half_f16_to_f32(bits: u16) -> f32 {
    // Manual F16 → F32 conversion (no external dep)
    let sign = ((bits >> 15) as u32) << 31;
    let exp  = ((bits >> 10) & 0x1f) as u32;
    let mant = (bits & 0x3ff) as u32;
    let (exp32, mant32) = if exp == 0 {
        if mant == 0 { (0, 0) }
        else {
            let e = 127 - 14 - mant.leading_zeros();
            (e, (mant << (mant.leading_zeros() - 22)) & 0x7fffff)
        }
    } else if exp == 31 {
        (255, mant << 13)
    } else {
        (exp + (127 - 15), mant << 13)
    };
    f32::from_bits(sign | (exp32 << 23) | mant32)
}

// Naive f32 matmul (m×k) × (k×n) → m×n, column-major weights as in TritMatrix.
fn f32_matmul_ops(a: &[f32], w: &[f32], m: usize, k: usize, n: usize) -> (Vec<f32>, u64) {
    let mut out = vec![0.0f32; m * n];
    let mut ops = 0u64;
    for row in 0..m {
        for col in 0..n {
            let mut acc = 0.0f32;
            for i in 0..k {
                acc += a[row * k + i] * w[i * n + col];
                ops += 1;
            }
            out[row * n + col] = acc;
        }
    }
    (out, ops)
}

fn main() {
    let model_path = std::env::var("ALBERT_MODEL")
        .unwrap_or_else(|_| DEFAULT_MODEL.to_string());

    println!("albert. TritFloat inference benchmark");
    println!("model: {model_path}");
    println!("arch:  {}L × {}E × H={} FFN={}  (albert_v3.0.config.json)", NUM_LAYERS, NUM_EXPERTS, HIDDEN, INNER);

    if !std::path::Path::new(&model_path).exists() {
        eprintln!("\nCheckpoint not found at {model_path}");
        eprintln!("Set ALBERT_MODEL=<path> or run from the repo root.");
        std::process::exit(1);
    }

    println!("\nLoading checkpoint...");
    let t_load = Instant::now();
    let tensors = load_safetensors(&model_path);
    println!("  Loaded {} tensors in {:.1}s", tensors.len(), t_load.elapsed().as_secs_f32());

    // ── Section 1: Real weight sparsity per layer ─────────────────────────────
    sep("SECTION 1 — Real weight sparsity per layer (all 18 layers, expert 0)");
    println!("{:<6} {:<30} {:<10} {:<10} {}", "layer", "tensor", "sparsity", "tau", "mean_abs");
    println!("{}", "-".repeat(65));

    let mut layer_sparsities: Vec<(f64, f64)> = Vec::new(); // (c_fc_sp, c_proj_sp)

    for layer in 0..NUM_LAYERS {
        for (suffix, dim_rows, dim_cols) in [
            ("c_fc",   INNER,  HIDDEN),
            ("c_proj", HIDDEN, INNER),
        ] {
            let key = format!("blocks.{layer}.moe.experts.0.{suffix}.weight");
            if let Some((w, shape)) = tensors.get(&key) {
                assert_eq!(shape, &[dim_rows, dim_cols],
                    "unexpected shape for {key}: {shape:?}");
                let tau = bitnet_threshold(w);
                let zeros = w.iter().filter(|&&v| v.abs() < tau).count();
                let sp = zeros as f64 / w.len() as f64;
                let mean_abs = w.iter().map(|x| x.abs()).sum::<f32>() / w.len() as f32;
                println!("{:<6} {:<30} {:<10.1} {:<10.5} {:.5}",
                    layer, suffix, sp * 100.0, tau, mean_abs);
                if suffix == "c_fc"   { layer_sparsities.push((sp, 0.0)); }
                if suffix == "c_proj" { layer_sparsities.last_mut().unwrap().1 = sp; }
            } else {
                println!("{layer:<6} {key:<30}  (missing)");
            }
        }
    }

    let avg_sp = layer_sparsities.iter().flat_map(|(a, b)| [*a, *b]).sum::<f64>()
        / (layer_sparsities.len() * 2) as f64;
    println!("\n  avg weight sparsity across all layers: {:.1}%", avg_sp * 100.0);

    // ── Section 2: Natural confidence from embedding activations ──────────────
    sep("SECTION 2 — Natural confidence variance from embedding activations");
    println!("  Strategy: embed real /p1-/p5 token IDs → batch of hidden-size activations");
    println!("  → run through layer 0 expert 0 c_fc (1024×256) via linear_confident");
    println!("  → observe output confidence histogram. Do values vary or collapse?");
    println!();

    let embed_key = "embed.weight";
    let (embed_data, embed_shape) = tensors.get(embed_key)
        .unwrap_or_else(|| panic!("embed.weight not found in checkpoint"));
    assert_eq!(embed_shape[1], HIDDEN, "embed dim mismatch");
    let vocab_size = embed_shape[0];

    // Gather embeddings for /p1-/p5 tokens (clamp to vocab size)
    let tokens: Vec<usize> = BENCH_TOKEN_IDS.iter()
        .map(|&id| id.min(vocab_size - 1))
        .collect();
    let n_tokens = tokens.len();

    // Stack embeddings into activation tensor [n_tokens, HIDDEN]
    let mut act_f32 = vec![0.0f32; n_tokens * HIDDEN];
    for (i, &tok_id) in tokens.iter().enumerate() {
        let src = &embed_data[tok_id * HIDDEN..(tok_id + 1) * HIDDEN];
        act_f32[i * HIDDEN..(i + 1) * HIDDEN].copy_from_slice(src);
    }

    // Apply RMSNorm approximation (mean-centre + scale by std) — embedding vectors
    // are tiny (std≈0.036) without normalisation; after layer_norm they're O(1).
    for row in act_f32.chunks_mut(HIDDEN) {
        let mean = row.iter().sum::<f32>() / HIDDEN as f32;
        let var  = row.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / HIDDEN as f32;
        let std  = var.sqrt().max(1e-6);
        row.iter_mut().for_each(|x| *x = (*x - mean) / std);
    }

    let act_std  = act_f32.iter().map(|x| x.powi(2)).sum::<f32>().sqrt() / act_f32.len() as f32;
    let act_zero = act_f32.iter().filter(|&&v| v.abs() < 0.01).count();
    println!("  Token activations: n={n_tokens}  hidden={HIDDEN}  rms={act_std:.4}  near_zero={act_zero}/{}", n_tokens * HIDDEN);

    // Load layer 0 c_fc weight
    let fc_key = "blocks.0.moe.experts.0.c_fc.weight";
    let (fc_w, fc_shape) = tensors.get(fc_key)
        .unwrap_or_else(|| panic!("{fc_key} not found"));
    // safetensors shape is [INNER, HIDDEN] (out_features × in_features)
    // TritMatrix::from_f32 expects (rows=in_features, cols=out_features)
    // We need to transpose: from [INNER, HIDDEN] → TritMatrix(HIDDEN, INNER)
    let mut fc_transposed = vec![0.0f32; HIDDEN * INNER];
    for i in 0..INNER {
        for j in 0..HIDDEN {
            fc_transposed[j * INNER + i] = fc_w[i * HIDDEN + j];
        }
    }
    assert_eq!(fc_shape, &[INNER, HIDDEN]);
    let tau_fc = bitnet_threshold(&fc_transposed);
    let w_fc = TritMatrix::from_f32(HIDDEN, INNER, &fc_transposed, tau_fc);

    // All-1.0 confidence first (baseline)
    let conf_ones = vec![1.0f32; n_tokens * HIDDEN];
    let acts_uniform = TritFloatTensor::from_f32_with_confidence(&act_f32, &conf_ones, &[n_tokens, HIDDEN]);
    let (out_uniform, skips_uniform) = linear_confident(&acts_uniform, &w_fc);

    println!("\n  Layer 0 c_fc: {}×{} → sparsity={:.1}%  tau={:.5}",
        HIDDEN, INNER, w_fc.sparsity() * 100.0, tau_fc);
    println!("  With conf=1.0: skips={skips_uniform}  out_min_conf={:.3}  out_mean_conf={:.3}",
        out_uniform.min_confidence(), out_uniform.mean_confidence());

    // Now test with varying confidence — assign per-element confidence proportional
    // to activation magnitude, normalised to [0,1]. This is a plausible natural prior:
    // large activations are more certain.
    let max_abs = act_f32.iter().map(|x| x.abs()).fold(0.0f32, f32::max).max(1e-6);
    let conf_natural: Vec<f32> = act_f32.iter().map(|x| x.abs() / max_abs).collect();
    let conf_mean_nat = conf_natural.iter().sum::<f32>() / conf_natural.len() as f32;
    let conf_min_nat  = conf_natural.iter().cloned().fold(f32::INFINITY, f32::min);
    let conf_max_nat  = conf_natural.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    let acts_natural = TritFloatTensor::from_f32_with_confidence(&act_f32, &conf_natural, &[n_tokens, HIDDEN]);
    let (out_natural, skips_natural) = linear_confident(&acts_natural, &w_fc);
    let hist = out_natural.confidence_histogram();

    println!("\n  With magnitude-scaled conf: input_conf mean={conf_mean_nat:.3}  min={conf_min_nat:.3}  max={conf_max_nat:.3}");
    println!("  Output: skips={skips_natural}  out_min_conf={:.3}  out_mean_conf={:.3}",
        out_natural.min_confidence(), out_natural.mean_confidence());
    println!("  Output confidence histogram (9 bins, 0.0→1.0): {hist:?}");

    let dominant = hist.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();
    let spread: usize = hist.iter().filter(|&&v| v > 0).count();
    println!("  Dominant bin: {}/8 ({:.0}% of outputs)  spread: {spread} non-empty bins",
        dominant.0, *dominant.1 as f32 / out_natural.numel() as f32 * 100.0);

    let verdict = if spread >= 3 { "VARIES (confidence encodes meaningful signal)" }
                  else           { "COLLAPSES (all outputs land in one bin)" };
    println!("  VERDICT: {verdict}");

    // ── Section 3: Per-layer skip accounting across /p1-/p5 ──────────────────
    sep("SECTION 3 — Per-layer skip counts across /p1-/p5 prompts (expert 0)");
    println!("{:<6} {:<12} {:<12} {:<12} {:<12} {:<10} {}",
        "layer", "ws%", "combined%", "weight_only%", "extra%", "formula_err", "layer_type");
    println!("{}", "-".repeat(80));

    let total_layer_ops = n_tokens * HIDDEN * INNER;

    for layer in 0..NUM_LAYERS {
        // c_fc: HIDDEN in_features × INNER out_features
        let fc_key = format!("blocks.{layer}.moe.experts.0.c_fc.weight");
        if let Some((fc_w_raw, _)) = tensors.get(&fc_key) {
            // Transpose from [INNER, HIDDEN] to TritMatrix(HIDDEN, INNER)
            let mut fc_t = vec![0.0f32; HIDDEN * INNER];
            for i in 0..INNER { for j in 0..HIDDEN { fc_t[j * INNER + i] = fc_w_raw[i * HIDDEN + j]; } }
            let tau = bitnet_threshold(&fc_t);
            let w = TritMatrix::from_f32(HIDDEN, INNER, &fc_t, tau);
            let ws = w.sparsity();

            // Weight-only skips (dense activations, all 1.0)
            let dense_conf = vec![1.0f32; n_tokens * HIDDEN];
            let dense_acts = TritFloatTensor::from_f32_with_confidence(&act_f32, &dense_conf, &[n_tokens, HIDDEN]);
            let (_, wo_skips) = linear_confident(&dense_acts, &w);

            // Combined skips (natural confidence activations)
            let (_, comb_skips) = linear_confident(&acts_natural, &w);

            // Actual activation sparsity from the natural acts
            let act_zeros = act_f32.iter().filter(|&&v| v.abs() < 1e-6).count();
            let as_frac = act_zeros as f64 / (n_tokens * HIDDEN) as f64;
            let formula_pred = (total_layer_ops as f64 * (ws + as_frac * (1.0 - ws))).round() as usize;
            let formula_err = comb_skips as i64 - formula_pred as i64;

            println!("{:<6} {:<12.1} {:<12.1} {:<12.1} {:<12.1} {:<10} c_fc",
                layer,
                ws * 100.0,
                comb_skips as f64 / total_layer_ops as f64 * 100.0,
                wo_skips   as f64 / total_layer_ops as f64 * 100.0,
                (comb_skips.saturating_sub(wo_skips)) as f64 / total_layer_ops as f64 * 100.0,
                format!("{formula_err:+}"),
            );
        }
    }

    // ── Section 4: Wall-clock at real albert. dims ────────────────────────────
    sep("SECTION 4 — Wall-clock: matmul_trit vs f32 dense at real albert. dims");
    println!("  Using real checkpoint weights (layer 0 expert 0).");
    println!("  Input: {n_tokens} token embeddings after RMSNorm.");
    println!("  Warmup: 100 iters. Bench: 1000 iters.");
    println!();

    const WARMUP: usize = 100;
    const ITERS:  usize = 1000;

    let layer_configs: &[(&str, &TritMatrix, usize, usize)] = &[
        ("c_fc (256→1024)", &w_fc, HIDDEN, INNER),
    ];

    println!("{:<22} {:<16} {:<16} {:<10} {}",
        "layer", "trit_us/iter", "f32_us/iter", "speedup", "skip%");
    println!("{}", "-".repeat(72));

    for &(label, w_mat, k_dim, n_dim) in layer_configs {
        // Warmup
        for _ in 0..WARMUP { let _ = linear_confident(&acts_natural, w_mat); }
        for _ in 0..WARMUP { let _ = f32_matmul_ops(&act_f32, &fc_transposed, n_tokens, k_dim, n_dim); }

        let t0 = Instant::now();
        for _ in 0..ITERS { let _ = linear_confident(&acts_natural, w_mat); }
        let trit_us = t0.elapsed().as_micros() as f64 / ITERS as f64;

        let t1 = Instant::now();
        for _ in 0..ITERS { let _ = f32_matmul_ops(&act_f32, &fc_transposed, n_tokens, k_dim, n_dim); }
        let f32_us = t1.elapsed().as_micros() as f64 / ITERS as f64;

        let (_, skips) = linear_confident(&acts_natural, w_mat);
        let skip_pct = skips as f64 / (n_tokens * k_dim * n_dim) as f64 * 100.0;
        println!("{:<22} {:<16.2} {:<16.2} {:<10.2} {:.1}%",
            label, trit_us, f32_us, f32_us / trit_us, skip_pct);
    }

    println!();
    println!("  IMPORTANT: matmul_trit uses all Rayon threads; f32 baseline is single-");
    println!("  threaded naive loops. For apples-to-apples compare:");
    println!("  RAYON_NUM_THREADS=1 cargo run --release --bin tritfloat_albert_bench");

    println!("\n{}", "═".repeat(70));
    println!("  Benchmark complete. Key questions answered:");
    println!("  S1: Real weight sparsity per layer (should be ~31%, not assumed 33%)");
    println!("  S2: Output confidence varies across bins (not all same) → meaningful");
    println!("  S3: Combined skips > weight-only on real activations");
    println!("  S4: Measured wall-clock speedup at albert. production dimensions");
    println!("{}", "═".repeat(70));
}
