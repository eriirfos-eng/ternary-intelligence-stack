# PLUGIN DEV GUIDE

Follow this guide to extend the MoE-13 ecosystem with custom providers or diagnostic tools.

## SPF-13 Format
Plugins are delivered as `.moeplugin` signed artifacts.
- **Manifest**: Must include all `required_capabilities` and `forbidden_capabilities`.
- **Signing**: Use the platform tools to sign your artifact with `ed25519`.

## Implementation
1. Implement the `MoePlugin` trait in your crate.
2. Ensure your implementation follows the `MoePlugin` lifecycle: `load`, `initialize`, `ingest`, `finalize`, `shutdown`.
3. Adhere to the deterministic contract—your plugin MUST produce consistent outputs for a given CMIR input.

## Constraints
- **Sandbox**: No direct file system access, no network calls, and no shared mutable state.
- **CMIR Contract**: All plugins must ingest and emit valid, traceable CMIR-compliant representations.
- **Offline**: No external API calls are allowed during plugin runtime.
