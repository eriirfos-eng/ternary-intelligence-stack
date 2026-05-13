# Ternary Intelligence Stack (TIS)

[![crates.io](https://img.shields.io/crates/v/ternlang-core.svg)](https://crates.io/crates/ternlang-core)
[![version](https://img.shields.io/badge/version-v1.3.7-blue)](#architecture)
[![license](https://img.shields.io/badge/license-LGPL--3.0%20%2F%20BSL--1.1-blue)](LICENSE)
[![tests](https://img.shields.io/badge/tests-138%20CI%20%7C%205%20crates-yellow)](#architecture)
[![API](https://img.shields.io/badge/API-live-brightgreen)](https://ternlang-api.fly.dev/health)
[![EU AI Act](https://img.shields.io/badge/EU%20AI%20Act-Article%2013,14+15%20Compliant%20Design-003399?logo=european-union)](https://ternlang.com/compliance)
[![MCP](https://img.shields.io/badge/MCP-34_tools-orange)](#mcp-server--v040-34-tools)
[![smithery badge](https://smithery.ai/badge/rfi-irfos/ternlang)](https://smithery.ai/servers/rfi-irfos/ternlang)
[![examples](https://img.shields.io/badge/examples-3.9k%2B_.tern_programs-blueviolet)](#architecture)
[![stdlib](https://img.shields.io/badge/stdlib-28k+_open_%26_2.5k+_premium-blue)](ternlang-root/stdlib/PREMIUM.md)
[![DOI](https://img.shields.io/badge/DOI-10.17605%2FOSF.IO%2FTZ7DC-informational)](https://doi.org/10.17605/OSF.IO/TZ7DC)
[![moe-core](https://img.shields.io/crates/v/moe-core.svg?label=moe-core)](https://crates.io/crates/moe-core)
[![moe-platform](https://img.shields.io/crates/v/moe-platform.svg?label=moe-platform)](https://crates.io/crates/moe-platform)
[![moe-runtime](https://img.shields.io/crates/v/moe-runtime.svg?label=moe-runtime)](https://crates.io/crates/moe-runtime)


Built by [RFI-IRFOS](https://ternlang.com) · Graz, Austria · Whitepaper [https://osf.io/cyn28]

---
### Full Documentation

- **[README.md](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/README.md)** — Full explanation, technical details, and compiler specifications
- **[albert-moe-13: Ternary Scaling Research](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/albert-moe-13)** — Native ternary training framework, EvolutionManager, live dashboard
- **[Convergence Log](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/albert-moe-13/docs/convergence_log.md)** — Live training loss history across all albert. versions
- **[Agent Albert CLI](https://github.com/eriirfos-eng/ternary-intelligence-stack/tree/main/agent_albert_cli)** — Terminal-native, model-agnostic AI agent built in pure Rust
- **[Ternlang Studio (Preview)](https://ternlang-api.fly.dev/studio)** — Work-in-progress developer dashboard and SDK
- **[Session Log](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/docs/session_log.md)** — Production fixes and refinements addressed during deployment
- **[Roadmap](https://github.com/eriirfos-eng/ternary-intelligence-stack/blob/main/ternlang-root/docs/ROADMAP.md)** — Phases 1–20 and priority matrix

---

## 1. What is Ternlang? (The Infrastructure Layer)

Ternlang is a systems programming language, compiler, and high-performance inference runtime built on balanced ternary logic. 
The Stack provides a fundamental architectural shift for **Explainable AI (XAI)** and European technological sovereignty by moving beyond the binary limitations of current systems.


The core type is `trit`: three values — `−1` (reject), `0` (hold), `+1` (affirm). This allows for **Deterministic Uncertainty**, where the zero state is a first-class routing instruction: *"insufficient confidence — do not act yet."*

### Quick Start
```bash
# One line — installs Rust (if needed) + ternlang-cli, ready immediately
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source "$HOME/.cargo/env" && cargo install ternlang-cli
```
> **Note:** Do not use `sudo apt install cargo` — Ubuntu's packaged version is too old (1.75). The line above installs the current toolchain via rustup.

```bash
ternlang                       # launch interactive REPL immediately
ternlang my_program.tern       # run a .tern file directly
ternlang run my_program.tern   # explicit form
```



## 2. What is albert.? (The Intelligence Layer)

**albert.** is a ternary Mixture-of-Experts language model trained natively from scratch — not quantized from a float model. Every weight is in `{-γ, 0, +γ}` throughout training via Straight-Through Estimator (STE). The architecture expands itself autonomously via Net2Net surgery when it plateaus, guided by the Mandelbrot complexity monitor. The engineering repo label is `albert-moe-13`.

**Current state (2026-05-13):** 12L · 256H · 12E · Top-3 routing · 256CTX · 32,000 vocab (ByteLevel BPE, multilingual EN/DE/FR/ES/PT/IT/NL/PL) · ~58M params · training on Modal T4 GPU · v3.0 Global Epoch 473+ · loss floor ~10.27, surgery gate armed.

A live training dashboard streams telemetry in real time at `localhost:8888` during training runs — layer topology, expert routing, gradient norms, TTL state, and loss curve with Fibonacci retracement overlays.

### What makes it different

| Feature | albert. | Standard LLM |
|--------|---------|--------------|
| Weight precision | Ternary `{-γ, 0, +γ}` from scratch | Float32 / post-hoc INT4 |
| **@sparseskip** | 75% expert skip per decode step — 9 of 12 experts inactive | Dense MoE: all experts active |
| Architecture growth | Autonomous Net2Net surgery + Mandelbrot complexity monitor | Fixed at init |
| Gate router | F32 for routing resolution — expert MLPs remain ternary | — |
| Inference speed | **83–125 tok/s (v2.0.0, 8k vocab, CPU-only)** | Requires GPU at this quality |
| Two sparsity layers | Routing-level 75% skip + weight-level 10–26% ternary zeros | Single sparsity axis |
| Patent | A50296/2026 (@sparseskip primitive) | — |

### Try it (API — no install needed)
```bash
# Ternary decision: affirm / hold / reject
curl -s https://ternlang-api.fly.dev/api/trit_decide \
  -H "Content-Type: application/json" \
  -d '{"statement": "This architecture is worth funding"}' | jq .

# Sparse MoE reasoning over 13 expert domains
curl -s https://ternlang-api.fly.dev/api/moe/orchestrate \
  -H "Content-Type: application/json" \
  -H "X-Ternlang-Key: YOUR_KEY" \
  -d '{"query": "What is ternary logic?", "evidence": [0.9, 0.1]}' | jq .
```

*See [`BENCHMARKS.md`](ternlang-root/BENCHMARKS.md) for full sparsity speedup data, [`albert-moe-13/`](albert-moe-13/) for training code, and the [Convergence Log](albert-moe-13/docs/convergence_log.md) for live loss history.*

### Why this combination is one system, not nine features

Each component in albert. is the enabling condition for the next. Remove any one piece and the others lose their justification.

**Ternary weights as substrate.** Every weight in `{-γ, 0, +γ}` throughout training — not post-hoc quantization. The Straight-Through Estimator (STE) treats quantization boundaries as soft gates during backprop, allowing discrete weights to train stably. This is the foundation everything else builds on.

**@sparseskip as the inference consequence.** When a weight is exactly zero, its multiply-accumulate contributes nothing — skip it exactly, not approximately. Ternary weights create this property structurally. Two levels of sparsity stack: Top-3 routing skips 9 of 12 experts per decode step (routing-level), and within each active expert, zero-weight operations are skipped element-wise (weight-level). This is what makes a small ternary model genuinely fast rather than theoretically compact.

**Cheap growth as the architectural consequence.** @sparseskip makes inference cheap proportional to sparsity. Cheap inference makes architectural growth events safe — adding a layer doesn't require expensive retraining from scratch, and the grown model serves efficiently immediately. Without sparsity, growth events are costly and hard to justify at the research prototype stage.

**Autonomous triggering as the operational consequence.** Safe growth enables autonomous triggering: the model can initiate its own expansion without operator intervention because the cost of a false trigger is low and recoverable. The EvolutionManager watches for genuine plateau signals — not transient instability, not routing collapse, not Nash equilibria in the gating network. Distinguishing these requires instrumentation.

**WALD as the plateau instrument.** Named after Abraham Wald (the statistician who corrected WWII aircraft reinforcement by noting the planes hit in the engines never returned), WALD tracks loss-space coverage per epoch: which 0.25-nat-wide buckets of the loss histogram receive batch visits, and which remain structurally empty. A model genuinely plateaued at its architectural ceiling has a stable dead zone below its mass center — not a transient gap but a structural one. WALD quantifies this. Without WALD, the plateau trigger cannot distinguish "done at this depth" from "stuck on a routing problem."

**MYCELIUM as the routing instrument.** Per-layer routing pressure telemetry — which layers carry the most expert-selection activity, which are dormant. Post-surgery, MYCELIUM watches whether the new layer earns routing share or stays cold. If it blooms, the growth was useful. If it stays dormant, the architecture has capacity it cannot use — informative signal for the next threshold calibration. The model votes on whether its own new layer is necessary.

**Fibonacci tempo as the growth schedule.** The plateau patience window equals the current Fibonacci milestone: 13 epochs for the 12→13L transition, 21 for 13→21L, 34 for 21→34L. This is not a human-chosen patience number. The Fibonacci sequence governs leaf arrangement in sunflowers, spiral arms in galaxies, branching angles in trees — biological self-organization discovered this tempo as optimal packing under growth constraints. albert. inherits it. The same mathematics that describes how a pine cone grows also describes when albert. should.

**Mandelbrot perturbation as the surgery geometry.** Net2Net safe copy clones the deepest layer as a function-preserving identity: the 13L model computes exactly the same function as the 12L model at surgery time. Then Mandelbrot-set-parameterized perturbation breaks the symmetry. Each weight in the new layer receives a coordinate `c` in the complex Mandelbrot parameter space, assigned via a golden-ratio sequence — deterministic, reproducible, unique per surgery in the model's lifetime. Weights mapping to Mandelbrot interior points (stable basins where iteration remains bounded) receive near-zero perturbation: learned representations are preserved. Weights mapping to the boundary (the fractal edge between order and chaos) receive maximum perturbation: plasticity injected precisely where the geometry says the system can absorb change without destabilizing. Random Gaussian noise makes no such distinction. Mandelbrot geometry does.

**The literature intersection.** Net2Net (Chen et al., 2015) provides function-preserving growth in F32. BitNet provides ternary weights with no growth. TC-MoE uses ternary routing choices, not ternary weights, and does not grow. MorphNet, Firefly, GradMax, and MixtureGrowth provide growth strategies in F32 with gradient-based or template-mixing initialization. Mandelbrot fractals appear in the ML literature as training data for classification, never as initialization geometry. Fibonacci appears in architecture metaphor, not as a training schedule. No published work combines ternary-weight substrate, function-preserving growth, Mandelbrot-parameterized perturbation, golden-ratio surgery sequencing, Fibonacci-tempo triggers, autonomous plateau detection, MYCELIUM post-surgery routing telemetry, and @sparseskip native sparse inference. The combination is not incremental novelty — it is a coherent claim about what a self-organizing ternary system looks like when all the pieces are present simultaneously.

### Live-intervention training, not post-hoc analysis

Most ML training operates in one mode: launch, observe from a distance, analyze after the run. If something goes wrong, restart from checkpoint and hope. Intervention during a live run is rare — the cost of a bad intervention usually exceeds the cost of suboptimal continuation, and the tooling to make good interventions doesn't exist.

albert.'s training stack is built for a different mode: **controlled experiments on a live training process**.

The conditions that make this possible:

- **Custom dashboard with convergence indicators.** SMA-21/55/144/377 overlays, Bollinger Bands, MACD, Fibonacci retracement zones, per-epoch OHLC candlesticks, Heikin-Ashi trend smoothing, and always-on crosshair telemetry. Trend drift is visible quantitatively before it becomes catastrophic — not "something looks off" but "SMA-21 crossed SMA-144 three epochs ago and MACD is diverging."
- **Checkpoint-resumable single binary.** State preservation is cheap: pull checkpoint, patch, fire again. One CLI command (`albert-train`). No orchestration framework to fight with. The cost of "discard last 15 minutes and patch" is rational, not desperate.
- **WALD loss-space coverage.** Tracks which loss regions the model visits structurally. A routing collapse looks different from an architectural plateau in the WALD histogram — the instrument distinguishes them so the intervention decision is informed, not guessed.
- **MYCELIUM routing telemetry.** Per-layer expert routing pressure, reported each epoch. Layer-level anomalies surface immediately, not after a full run completes.
- **Fibonacci-window plateau detection.** The EvolutionManager holds a 13-epoch rolling window and computes the loss delta across it. "Plateau" is a quantitative signal (Δ < 0.02 nats over 13 epochs), not gut feel.
- **Sub-million-parameter model on a single T4.** Compute is cheap enough (~$0.003/epoch) that live observation and intervention are economically rational. This scale is deliberate: the training methodology is developed here before being applied at larger scale.
- **15-minute monitoring cadence.** Continuous observation throughout a run — not periodic checks after the fact.

The result is a training workflow that resembles a controlled experiment rather than a batch job. Hyperparameters, gate thresholds, and corpus composition can be adjusted between epochs with full state preservation, and the dashboard provides immediate quantitative feedback on whether the adjustment worked. This is rare in ML practice precisely because it requires all the above conditions to hold simultaneously.

### Known Limitations (honest)
- albert. at 12L is a **research prototype**, not a production LLM. It generates statistically coherent multilingual text. Instruction-following capability is targeted with instruction fine-tuning at a later stage.
- Training runs on a single T4 GPU (Modal serverless). Multi-GPU distributed training is on the roadmap (Phase 23).
- Held-out perplexity vs float32 baseline: `cargo run --release -p moe-llm-core --bin moe-test -- --bench`
- The CUDA custom kernel (`cuda_matmul.rs`) is at TRL 3 — the candle CUDA backend is active and used in training; the hand-rolled kernel is not yet integrated.

### Core Algorithm Files (direct links)

All core training and inference primitives are open-source under LGPL-3.0 and live inside the `albert-moe-13/moe-llm-core` Cargo workspace member:

| File | What it implements |
|------|--------------------|
| [`albert-moe-13/moe-llm-core/src/model/ste.rs`](albert-moe-13/moe-llm-core/src/model/ste.rs) | Straight-Through Estimator — keeps weights ternary during backprop |
| [`albert-moe-13/moe-llm-core/src/model/ternary_linear.rs`](albert-moe-13/moe-llm-core/src/model/ternary_linear.rs) | Ternary linear layer with `forward_sparse()` — element-level @sparseskip |
| [`albert-moe-13/moe-llm-core/src/model/moe.rs`](albert-moe-13/moe-llm-core/src/model/moe.rs) | MoE router + Top-K dispatch — routing-level @sparseskip (75% skip) |
| [`albert-moe-13/moe-llm-core/src/model/transformer.rs`](albert-moe-13/moe-llm-core/src/model/transformer.rs) | Full transformer stack, attention, decode loop |
| [`albert-moe-13/moe-llm-core/src/model/evolution.rs`](albert-moe-13/moe-llm-core/src/model/evolution.rs) | EvolutionManager — Fibonacci-tempo plateau trigger + Net2Net surgery dispatch |
| [`albert-moe-13/moe-llm-core/src/wald.rs`](albert-moe-13/moe-llm-core/src/wald.rs) | WALD — loss-space coverage tracking; detects structural dead zones below mass center |
| [`albert-moe-13/moe-llm-core/src/bin/train_bible.rs`](albert-moe-13/moe-llm-core/src/bin/train_bible.rs) | Full training loop: STE backward, cosine LR, EvolutionManager integration |

> These files form the complete ternary training stack. The `@sparseskip` primitive (Patent A50296/2026) spans `ternary_linear.rs` (weight-level) and `moe.rs` (routing-level).

### Model Artifact (downloadable)

The first publicly released native-ternary checkpoint is attached to the [v2.0.0 GitHub release](https://github.com/eriirfos-eng/ternary-intelligence-stack/releases/tag/v2.0.0):

| File | Size | Description |
|------|------|-------------|
| [`bible_ternary_v2.0.0.trit`](https://github.com/eriirfos-eng/ternary-intelligence-stack/releases/download/v2.0.0/bible_ternary_v2.0.0.trit) | 13 MB | Packed ternary weights — 3L · 256H · 12E · 8k vocab · `{-γ, 0, +γ}` throughout |
| [`bible_ternary_v2.0.0.config.json`](https://github.com/eriirfos-eng/ternary-intelligence-stack/releases/download/v2.0.0/bible_ternary_v2.0.0.config.json) | 105 B | Architecture config |

Format: custom `.trit` binary (see [`quantize_model.rs`](albert-moe-13/moe-llm-core/src/bin/quantize_model.rs) for the packer and [`packing.rs`](albert-moe-13/moe-llm-core/src/model/packing.rs) for the spec). Header = tensor count (u32 LE), then per-tensor: name length, name bytes, shape dims, type byte (0=raw f32, 1=packed ternary), data.

---

## 3. Agent Albert — The Sovereign AI Assistant

**Albert** is also the terminal-native interface for the TIS. He runs entirely in your terminal, connects to local models (Ollama) or commercial bridges, and never “phones home.”

### Quick Start
```bash
# One line — installs Rust (if needed) + albert-cli, ready immediately
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source "$HOME/.cargo/env" && cargo install albert-cli
```
> **Note:** Do not use `sudo apt install cargo` — Ubuntu's packaged version is too old (1.75). The line above installs the current toolchain via rustup.

```bash
albert-cli     # launch immediately
```
---

### Key Capabilities

| Capability | Details |
|---|---|
| **Autonomous agent loop** | `/loop <mission>` — runs up to 10 tool-use turns to complete a goal |
| **Local-First** | Native Ollama support for completely offline and free operation |
| **Model-agnostic** | Connect to Gemini, Claude, GPT-4o, or Grok as "capability bridges" |
| **Self-reflection memory** | Commits key facts to local storage to maintain long-term context |

---

## 4. Repository Architecture

This repository is split into three primary domains, each serving a distinct purpose in the TIS ecosystem:

| Path | Purpose |
|-----------|----------------|
| [`ternlang-root/`](ternlang-root/) | **The Orchestration Layer:** Compiler, BET VM, and the MoE-13 Orchestrator MCP server. This layer handles logical routing and ternary decision-making. |
| [`albert-moe-13/`](albert-moe-13/) | **Model Development Framework:** The native research framework for training scaling. Houses the crates responsible for ternary manifold adaptation, STE-based training, and model architecture. |
| [`agent_albert_cli/`](agent_albert_cli/) | **Sovereign Agent Layer:** The terminal-native, model-agnostic AI agent (Albert) built in pure Rust for autonomous coding and orchestration. |

### Note on Training Infrastructure
The full training pipeline — including the Straight-Through Estimator (STE) backward pass, EvolutionManager, and cosine LR schedule — is implemented in `albert-moe-13/moe-llm-core/src/bin/train_bible.rs`. The STE quantization primitive is at `albert-moe-13/moe-llm-core/src/model/ste.rs`. Massive-scale distributed training on GPU clusters is on the roadmap (Phase 23).

---

## Licensing

Three license tiers apply across this repository. Every crate declares its license explicitly in `Cargo.toml`.

### Open Core — LGPL-3.0-or-later (free, copyleft)

The entire research and language infrastructure layer is open source. This includes everything needed to study, reproduce, and build on the ternary training stack:

| Component | Crates / paths |
|-----------|----------------|
| **albert. training stack** | `albert-moe-13/moe-llm-core`, `moe-compute`, `moe-data`, `moe-test`, `reproducibility_verifier` |
| **Ternlang compiler + VM** | `ternlang-root/ternlang-core`, `ternlang-vm`, `ternlang-parser`, `ternlang-wasm` |
| **CLI + LSP** | `ternlang-root/ternlang-cli`, `ternlang-lsp`, `ternlang-test` |
| **Agent Albert CLI** | `agent_albert_cli/` |
| **Open stdlib** | `ternlang-root/stdlib/` — 28,500+ `.tern` modules |
| **Python bindings** | `pytern/` |

> All `albert-moe-13` crates inherit `license = "LGPL-3.0-or-later"` from the workspace root. The `@sparseskip` primitive (Patent A50296/2026) is implemented in these open-source files — the patent covers the method; the implementation is freely readable and modifiable under LGPL.

### Commercial Infrastructure — BSL-1.1 (source-available, converts to Apache-2.0 after 4 years)

Operational and domain-specific crates that require a commercial subscription for production use. Source is publicly readable but production deployment requires a licence:

`ternlang-api` · `ternlang-mcp` · `ternlang-sec` · `ternlang-crypto` · `ternlang-ml` · `ternlang-compress` · `ternlang-gate` · `ternlang-hdl` · `ternlang-hft` · `ternlang-bci` · `ternlang-bio` · `ternlang-qutrit` · `ternlang-ros2` · `ternlang-posix` · `ternlang-contract` · `ternlang-consensus` · `ternlang-astro` · `ternlang-tson`

### Proprietary (Enterprise only, source not public)

`ternlang-ruvector` — on-premise vector inference engine; available under the Enterprise tier only.

### Subscription tiers

| Tier | Price | What you get |
|------|-------|---------|
| Community (LGPL-3.0) | Free | Compiler, VM, CLI, LSP, 28,500+ open-core stdlib modules + 34 MCP tools |
| Pro Standard (BSL-1.1) | €99/month | REST API, server-side memory, Tier 2 'Masterwork' modules |
| Industrial (BSL-1.1) | €349/month | QNN, SEC, T-HAL, TernAudit + Tier 3 'Masterwork' modules |
| Enterprise (Proprietary) | From €2,500/month | On-premise, FPGA, custom SLA + Tier 4 'Masterwork' modules + **unlimited API calls/month** |

---

## Team

The Ternary Intelligence Stack is built by a core team of five co-founders from **RFI-IRFOS**, Graz:

*   **Simeon Kepp**: Head of Research & Systems Architect.
*   **Nikoletta Csonka**: Head of Strategic Outreach & EU Relations.
*   **Zabih Karimi**: Principal Network & ML Engineer.
*   **Lisa Scharler**: Head of Social Technology & Ecocentric Systems.
*   **Louis Ehrig**: Corporate Secretary and Press & Media Relations.

→ **[Read our BIO and Mission in LEADERSHIP.md](LEADERSHIP.md)**

---

<div align="center">
  <img src="ternlang-root/ternlang-web/assets/ternlang_logo_notext.png" alt="Ternlang Logo" width="100">
</div>
