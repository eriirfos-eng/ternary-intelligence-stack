# QUICKSTART.md

Get started with MoE-13, a deterministic, offline-first inference runtime.

## Installation
Add the SDK to your project:
```bash
cargo add moe-sdk
```

## Basic Usage
```rust
use moe_sdk::*;

// 1. Initialize the platform with your provider configuration
let config = serde_json::json!({"provider": "ollama", "model": "llama3"});
let runtime = MoEPlatform::load(config)?;

// 2. Run deterministic inference
let output = runtime.run("Explain the MoE-13 ecosystem in one sentence.")?;
println!("Inference: {}", output);
```

## CLI
```bash
moe run --provider ollama --model llama3
```
