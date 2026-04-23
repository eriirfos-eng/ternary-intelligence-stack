# Albert CLI (Ternlang Engine)

The **Albert CLI** is the terminal-based evolution of the Albert agentic framework, now powered by the high-performance Rust-based **Ternlang Engine**. Following **Operation Strip-Mine**, the Albert node has been hardened, expanded, and fully decoupled from proprietary dependencies.

## Key Enhancements (v1.3.0+)

### 🛡️ Sovereign Security Architecture
- **Deny-First AST Interception**: All shell commands are parsed by a native Rust interception pipeline. We proactively block command smuggling (substitution, backticks), dangerous pipes (e.g., `curl | bash`), and unauthorized system redirections before they hit the shell.
- **Revoked Flag Access**: The LLM no longer has dynamic control over security flags like `dangerously_disable_sandbox`. Security is enforced at the policy level, neutralizing prompt injection attacks targeting the sandbox.
- **Ternary Intelligence Stack (+1/0/-1)**: Every action is validated. 
    - `+1 (Proceed)`: Success/Safe.
    - ` 0 (Halt)`: Ambiguity or missing context detected; the agent pauses and prompts the user rather than hallucinating.
    - `-1 (Retry)`: Failure; the agent self-corrects based on feedback.

### 🌐 LLM Agnostic Core
- **BYOK (Bring Your Own Key)**: Use the new `/auth` command to connect to any provider: **OpenAI, Anthropic, Google (Gemini), Hugging Face, AWS, Azure**, or **Ternlang**.
- **Dynamic Resolution**: The engine automatically maps model names to their respective providers and configurations, ensuring seamless switching across the global LLM landscape.

### 🧠 Advanced Reasoning & Memory Arsenal
- **SequentialThinking**: Structured, multi-step reasoning that allows the agent to record and revise its internal thought chains sequentially.
- **Memory (Knowledge Graph)**: Persistent, local-first knowledge graph that tracks entities, relations, and observations across sessions. Albert "remembers" project-specific context and logic.
- **RepoMap**: Automated codebase navigation that generates structured maps of your project, helping the agent understand complex repositories instantly.
- **MCP Client**: Native Support for the Model Context Protocol, allowing integration with any third-party MCP server.

## Overview
This repository serves as the sovereign, local-first engine for the Ternary Intelligence Stack. It integrates a hardened agentic loop, a standardized tool harness, and sliding-window context management into a single, high-performance Rust binary.

## Key Components
- **Engine Block (`rust/`)**: The Rust-based core providing:
  - **Hardened Autonomous Loop**: Research -> Strategy -> Execution with Ternary validation.
  - **RTK (Rust Token Killer)**: Integrated token-optimized CLI proxy for 60-90% savings.
  - **Multi-Provider API Client**: Agnostic LLM communication layer.
- **Standardized Toolset**: Core tools (Bash, File Ops, Glob, Grep) + Arsenal tools (Memory, SequentialThinking, RepoMap).

## Quickstart
1. **Build and Install**:
   ```bash
   cd rust
   cargo install --path crates/rusty-ternlang-cli --force
   ```
2. **Authenticate with Your Provider**:
   ```bash
   ternlang-cli /auth
   ```
3. **Initialize Repository**:
   ```bash
   ternlang-cli /init
   ```
4. **Enter the REPL**:
   ```bash
   ternlang-cli
   ```

## Repository Layout
```text
.
├── rust/           # High-performance, Hardened Rust Engine (Ternlang)
│   ├── crates/
│   │   ├── api/        # LLM Agnostic Client
│   │   ├── runtime/    # Core Loop, Security, & Memory
│   │   ├── tools/      # Arsenal (Memory, RepoMap, etc.)
│   │   └── ...
├── src/            # Legacy Python Agent source (Reference only)
└── README.md       # This file
```

---
*A sovereign project of RFI-IRFOS. Standardizing high-performance, secure agentic engineering.*
