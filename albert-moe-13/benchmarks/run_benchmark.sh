#!/usr/bin/env bash
# Albert MoE-13 — end-to-end inference speed benchmark
# Produces the tok/s figures reported in the model card (84.4 tok/s peak on HP ZBook 15).
#
# Requirements: Rust toolchain (cargo), no GPU required.
# Runtime: ~2 min on a modern laptop CPU.
#
# Usage:
#   cd ternary-intelligence-stack/albert-moe-13
#   bash benchmarks/run_benchmark.sh
#   # or with CSV output:
#   bash benchmarks/run_benchmark.sh --csv benchmarks/results.csv
set -e

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CSV_ARG=""

if [ "$1" = "--csv" ] && [ -n "$2" ]; then
    CSV_ARG="--csv $2"
fi

echo "Building moe-test (release)..."
cargo build --release -p moe-test --manifest-path "${REPO_ROOT}/Cargo.toml" 2>&1

echo ""
echo "Running Albert MoE-13 inference benchmark..."
echo "15 prompts across EN/DE multilingual domains — reports tok/s per prompt"
echo ""
"${REPO_ROOT}/target/release/moe-test" --bench ${CSV_ARG}
