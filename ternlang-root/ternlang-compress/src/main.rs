// ternpress — CLI for the ternlang-compress pipeline.
//
// Usage:
//   ternpress --input model.gguf --output model.tern [--verbose]
//   ternpress --synthetic --layers 32 --hidden 4096 --output synthetic.tern
//   ternpress --info model.tern

use std::path::PathBuf;
use anyhow::Result;
use ternlang_compress::{
    compress, CompressConfig,
    format::{load_tern, save_tern, synthetic_layers},
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Minimal arg parsing (replace with clap once the GGUF loader is wired up)
    if args.len() < 2 {
        eprintln!("ternpress — ternlang LLM compression tool");
        eprintln!();
        eprintln!("Commands:");
        eprintln!("  --info <model.tern>          Print stats for a compressed model");
        eprintln!("  --synthetic [--layers N] [--hidden N] --output <out.tern>");
        eprintln!("                               Run pipeline on a synthetic model (no GPU needed)");
        eprintln!();
        eprintln!("Coming in Phase 12:");
        eprintln!("  --input <model.gguf>  --output <out.tern>   Compress a real Ollama model");
        eprintln!("  --input <dir/>        --output <out.tern>   Compress a safetensors directory");
        return Ok(());
    }

    match args[1].as_str() {
        "--info" => {
            let path = PathBuf::from(args.get(2).expect("--info requires a path"));
            let model = load_tern(&path)?;
            println!("{}", model.summary());
        }

        "--synthetic" => {
            let n_layers: usize = flag_val(&args, "--layers").unwrap_or(4);
            let hidden:   usize = flag_val(&args, "--hidden").unwrap_or(256);
            let out_path         = flag_str(&args, "--output").unwrap_or("synthetic.tern".into());
            let verbose          = args.contains(&"--verbose".to_string());

            println!("Generating synthetic model: {n_layers} layers × {hidden}×{hidden}…");
            let layers = synthetic_layers(n_layers, hidden, 42);

            let cfg = CompressConfig {
                source_model: format!("synthetic-{n_layers}L-{hidden}h"),
                architecture: "Synthetic".into(),
                num_layers:   n_layers,
                hidden_size:  hidden,
                verbose,
                ..Default::default()
            };

            let model = compress(layers, cfg)?;
            println!("{}", model.summary());

            let out = PathBuf::from(&out_path);
            save_tern(&out, &model)?;
            println!("Saved → {out_path}");
        }

        other => {
            eprintln!("Unknown command: {other}");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn flag_val<T: std::str::FromStr>(args: &[String], flag: &str) -> Option<T> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .and_then(|w| w[1].parse().ok())
}

fn flag_str(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].clone())
}
