# ALBERT.md

This file provides guidance to the **Albert** agent when working in this repository.

## Detected stack

- **Primary language**: Rust (Cargo workspace, edition 2021)
- **Binary**: `albert` (crate: `albert-cli`, path: `rust/crates/albert-cli`)
- **Workspace root**: `rust/`

## Verification

```bash
# From rust/
cargo build --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Install and run
cargo install --path crates/albert-cli --force
albert
```

## Repository shape

```
rust/
  crates/
    albert-runtime/      — session management, MCP, OAuth, bash execution, file ops
    albert-api/          — multi-provider LLM client (Anthropic, OpenAI, Gemini, Ollama, XAI...)
    albert-commands/     — slash command library (/plan, /tdd, /loop, /compress ...)
    albert-tools/        — tool execution layer (read, write, edit, bash, glob, grep, MCP)
    albert-compat/       — upstream manifest extraction and path resolution
    albert-cli/  — main binary (package: albert-cli, binary: albert)
    rtk-integration/     — RTK token filter (vendored external project, publish = false)
src/                     — Python + TypeScript legacy source
tests/                   — validation surfaces
```

## Working agreement

- **Sovereignty First**: Albert acts via tools. No narration without action.
- **Model-agnostic**: Never hardcode a provider. Provider is resolved from env vars at runtime.
- **Token discipline**: RTK is integrated — keep context lean, use `/compress` proactively.
- **Zero-trust**: Keys live in `~/.config/albert/secrets.json`. Nothing leaves the machine except direct provider calls.
- **Session memory**: Sessions persist in `~/.config/albert/sessions/`. Always resume context.

## Crate dependency order (for publishing)

1. `albert-runtime` (no internal deps)
2. `albert-api`, `albert-commands` (depend on albert-runtime)
3. `albert-tools` (depends on albert-api, albert-runtime)
4. `albert-compat` (depends on albert-commands, albert-tools, albert-runtime)
5. `albert-cli` (depends on all above)

## Ecosystem integration

Albert lives in the Ternary Intelligence Stack monorepo alongside:
- `ternlang-core` / `ternlang-runtime` — the ternary compiler and VM
- `ternlang-api` — the institutional REST API (ternlang-api.fly.dev)
- TernStudio — the visual flow IDE (ternlang-api.fly.dev/studio)

Integration hooks (`albert-api` and `albert-runtime` feature `ternlang`) are stubbed and will be activated once the ternlang-* crates reach crates.io.
