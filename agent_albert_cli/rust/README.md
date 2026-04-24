# Ternlang CLI (Ternlang)

The Ternlang CLI is an autonomous, local-first development agent engine designed for Ternary Intelligence Systems. 

## Overview
Ternlang CLI is the central "engine block" of your development environment. It provides a sovereign, zero-trust coding loop that runs locally on your machine, integrating deeply with your codebase while maintaining full control over your LLM API keys and project context.

## Key Features
- **Sovereign Autonomous Loop**: A local-first agent loop that performs research, generates Ternlang code, validates it, and saves verified skills.
- **Self-Authoring**: Autonomous creation of new skills through the `create_skill` tool, with built-in validation gates using the Ternlang compiler.
- **Optimized Context**: Integrated `RTK` (Rust Token Killer) to perform high-performance token-aware context filtering, ensuring only the most relevant, compressed semantic data is sent to the LLM.
- **Zero-Trust**: No data or billing info leaves your local machine. You hold the brain; you hold the keys.

## Architecture
- **Engine Block**: The `ternlang-cli` daemon handles the core agentic loop, tool execution, and local storage.
- **Context Management**: Powered by integrated RTK context filtering to minimize token consumption and maximize agent efficiency.
- **Skill Engine**: A registry of `SKILL.md` documents providing specialized capabilities for neural training, swarm coordination, and agentic workflows.

## Configuration
Configuration is managed locally via the `.ternlang` directory in your project root or `~/.ternlang` in your home directory.
- `TERNLANG_API_KEY`: Set your model provider key in your environment.
- `ALBERT.md`: Define custom agent instructions for your repository here.

## Development
- **Build**: `cargo build --workspace`
- **Install**: `cargo install --path crates/rusty-ternlang-cli --force`
- **Run**: `ternlang-cli`

## License
MIT
