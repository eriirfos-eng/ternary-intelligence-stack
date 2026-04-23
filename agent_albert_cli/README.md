# Albert CLI (Ternlang Engine)

The **Albert CLI** is the terminal-based evolution of the Albert agentic framework, now powered by the high-performance Rust-based **Ternlang Engine**.

## Overview
This repository serves as the sovereign, local-first engine for the Ternary Intelligence Stack. It integrates the core agentic loop, tool harness, and context management into a single, high-performance binary.

## Key Components
- **Engine Block (`rust/`)**: The Rust-based core providing:
  - **Autonomous Loop**: Research, generate, validate, and save workflow.
  - **Context Filtering**: Integrated **RTK (Rust Token Killer)** for token-efficient LLM interactions.
  - **Skill Registry**: A system for creating and validating custom Ternlang skills.
- **Legacy Components (`src/`, `tests/`)**: Original Python-based implementations for historical reference and parity.

## Quickstart
1. **Navigate to the Engine Block**:
   ```bash
   cd rust
   ```
2. **Build and Install**:
   ```bash
   cargo build --workspace
   cargo install --path crates/rusty-ternlang-cli --force
   ```
3. **Initialize Repository**:
   ```bash
   ternlang-cli init
   ```
4. **Run**:
   ```bash
   export TERNLANG_API_KEY="your-api-key"
   ternlang-cli
   ```

## Repository Layout
```text
.
├── rust/           # NEW: High-performance Rust Engine (Ternlang)
├── src/            # Legacy Python Agent source
├── tests/          # Legacy verification suite
└── README.md       # This file
```

## Documentation
- `ALBERT.md`: Define custom repository-specific instructions.
- `rust/README.md`: Detailed documentation for the Rust-based Ternlang CLI engine.

---
*A sovereign project of RFI-IRFOS.*
