# ternpkg-registry

The central nervous system of the Ternary ecosystem. If NPM controls JavaScript and Crates.io controls Rust, `ternpkg-registry` controls the post-binary future.

## The Strategy: The Audit Gatekeeper
We do not just host code; we enforce the paradigm. Every package uploaded to the global registry is automatically scanned by `tern-audit`.

If a developer tries to publish a `.ternpkg` that uses binary boolean coercion or fails to exhaustively handle `deliberative hold` (The "Hold" State), **the registry rejects the package**. 

By controlling the distribution pipeline, we guarantee that the entire third-party ecosystem remains structurally dependent on the BET VM and RFI-IRFOS standards. 

## Features
- **Zero-Binary Tolerance:** Automatic compliance scoring via `tern-audit`.
- **Hardware-Targeted Builds:** Packages distribute pre-compiled BET VM bytecode alongside raw `.tern` source.
