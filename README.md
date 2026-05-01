# MoE-13 Inference Runtime

MoE-13 is a deterministic, offline-first inference runtime supporting multiple model providers through a unified execution system.

## Capabilities
* **Run models from**:
  * Ollama (offline-first priority)
  * HuggingFace (local ingestion)
  * Custom provider plugins
* **Deterministic inference execution**
* **Plugin-based model extension system**
* **Offline execution guarantee**
* **Cross-provider unified API**

## Quick Start
```rust
use moe_sdk::*;

// Initialize the platform
let runtime = MoEPlatform::load(config)?;

// Run inference
let output = runtime.run("input_query")?;
```

## CLI Usage
```bash
moe run --provider ollama --model llama3
moe run --provider hf --model gemma
```

## Architecture
MoE-13 consists of:
* **Public SDK (`moe-sdk`)**: Simple, stable entrypoint for developers.
* **Execution Platform (`moe-platform` + `plugin-sdk`)**: High-performance runtime and extensibility layer.
* **Internal Runtime**: Optimized ternary execution substrate (hidden).

## Stability Guarantees
* Deterministic execution
* Offline-first by design
* Reproducible outputs
* Sandboxed plugins
* No runtime mutation of core engine
