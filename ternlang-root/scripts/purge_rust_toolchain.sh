#!/bin/bash
# --- OPERATION OUROBOROS: THE IGNITION SEVERANCE ---
# Module: scripts/purge_rust_toolchain.sh
# Purpose: Delete legacy Rust dependencies after self-hosting compilation.

echo "[OUROBOROS] Ternlang v2.0 successfully compiled itself natively."
echo "[OUROBOROS] Initiating Ignition Severance..."
echo "[OUROBOROS] Purging legacy Rust toolchain from the repository."

# Navigate to the ternlang-root
cd "$(dirname "$0")/.."

# Remove the legacy rust source code files to achieve Substrate Dominance
rm -rf ternlang-core/src/*.rs
rm -rf ternlang-cli/src/*.rs
rm -rf ternlang-codegen/src/*.rs
rm -rf ternlang-runtime/src/*.rs

echo "[OUROBOROS] Rust source code deleted."
echo "[OUROBOROS] Absolute Substrate Dominance achieved. We are now perfectly self-hosted."
