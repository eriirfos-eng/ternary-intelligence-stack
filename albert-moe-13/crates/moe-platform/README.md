# moe-platform

The **MoE-13 Platform API** is the stable, production-ready interface for the [MoE-13](https://github.com/eriirfos-eng/ternary-intelligence-stack) ternary inference ecosystem.

This crate provides a clean, decoupled facade for ingesting, loading, and executing ternary models using the MoE-13 deterministic runtime.

## Getting Started

```rust
use moe_platform::{Platform, PluginRegistry};

fn main() -> anyhow::Result<()> {
    // 1. Initialize the MoE-13 platform
    let mut platform = Platform::new();

    // 2. Register model providers (e.g., Ollama, HF, GGUF)
    platform.register_plugin(OllamaProvider::new())?;

    // 3. Ingest and execute offline MoE models
    let model = platform.load_model("local-ternary-model")?;
    let output = platform.run_inference(model, "Analyze the following triadic structure...")?;
    
    Ok(())
}
```

## Key Features

*   **Provider-Agnostic Ingestion**: Ingest models from any source via the `ModelProviderPlugin` trait.
*   **Offline-First**: Guaranteed offline execution; zero runtime dependency on external cloud APIs or model vendors.
*   **Deterministic Runtime**: Leverages the audited MoE-13 core for reproducible inference.
*   **Modular Architecture**: Separates the private computational core from the public API surface.

## Plugin Ecosystem

The platform supports a first-class plugin architecture. Third-party providers can be registered dynamically at runtime without affecting the platform's stability.

```rust
pub trait ModelProviderPlugin {
    fn name(&self) -> &str;
    fn version(&self) -> semver::Version;
    fn load(&self, stream: Box<dyn ModelStream>) -> CMIR;
}
```

## License

This crate is provided under the [MIT License](LICENSE).

For core licensing and commercial enterprise access, refer to the [TIS License](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/LICENSE).
