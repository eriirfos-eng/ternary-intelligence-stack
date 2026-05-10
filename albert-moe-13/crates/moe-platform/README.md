# MoE-Platform — Inference Runtime (Planned)

Future production inference API for Albert MoE-13. This crate is a placeholder for the deployment-facing interface that will wrap the trained ternary model for external consumption.

For current inference and benchmarking, use the `moe-test` crate.

---

## Current Inference Interface (`moe-test`)

Until `moe-platform` is built, all inference runs through `moe-test`:

```bash
# Interactive TUI (type prompts, see tok/s live)
./target/release/moe-test

# Full benchmark suite — speed + @sparseskip + perplexity + CSV export
./target/release/moe-test --bench --csv results.csv

# Perplexity evaluation on a text file
cargo run --release -p moe-llm-core --bin eval_perplexity data/corpus/stage_3/alice.txt

# One-line installer for external evaluators (Linux + macOS)
curl -fsSL https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/albert-moe-13/bench/install.sh | sh
```

**Measured performance (v2.0.0, 12L × 12E × 256H, CPU only):**
- 84.4 tok/s on Intel i7-4800MQ (2013 laptop)
- 75% expert skip rate per decode step (@sparseskip)
- No GPU required

---

## Planned Scope

The platform crate will decouple inference from the training code:

- **Model loading** from `.safetensors` + `config.json`
- **Batched inference** with top-k / temperature sampling
- **Ternary-native execution** — pre-ternarized weights at load time, integer-only matmuls
- **REST API** via Axum for serving Albert as a local endpoint
- **MCP server integration** — expose Albert as a tool callable from Claude / TernLang-MCP

---

## Integration Target

```rust
// Future API (not yet implemented)
use moe_platform::Albert;

let albert = Albert::load("models/albert_v3.0")?;
let response = albert.generate("In the beginning", 128)?;
```

---

## See Also

- [Main README](../../README.md) — current architecture and training setup
- [moe-test](../../moe-test/) — current inference + benchmark binary
- [Benchmark installer](../../bench/install.sh) — one-line external evaluator setup
- [TernLang-MCP](../../../ternlang-root/ternlang-mcp/) — MCP server (live)
