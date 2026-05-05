# MoE-Platform — Inference Runtime (Planned)

Future production inference API for Albert MoE-13. Not yet implemented — this crate is a placeholder for the deployment-facing interface that will wrap the trained ternary model.

---

## Planned Scope

The platform crate will decouple inference from the training code and provide:

- **Model loading** from `.safetensors` checkpoint + `config.json`
- **Batched inference** with top-k / temperature sampling
- **Ternary-native execution** — apply STE quantization at load time and run integer-only matmuls
- **REST API** via Axum for serving Albert as a local endpoint
- **MCP server integration** — expose Albert as a tool callable from Claude/TernLang-MCP

---

## Current State

Training and inference share the same `moe-llm-core` crate. The `Transformer::generate()` method in `transformer.rs` handles greedy/sampled generation for local testing. This is sufficient for research purposes.

The `moe-platform` crate will be built once the model reaches stable loss convergence and the architecture is frozen for a production release.

---

## Integration Target

```rust
// Future API (not yet implemented)
use moe_platform::Albert;

let albert = Albert::load("models/bible_ternary_v2.0.0")?;
let response = albert.generate("In the beginning", 128)?;
```

---

## See Also

- [Main README](../../README.md) — current training setup
- [Architecture](../../docs/architecture.md) — model internals
- [TernLang-MCP](../../../ternlang-root/ternlang-mcp/) — MCP server (live)
