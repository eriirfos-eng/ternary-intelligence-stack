#!/usr/bin/env bash
# Albert-MoE-13 Reproducibility Verification Script
# Runs the repro_check binary and validates exit code.
#
# Usage:
#   cd albert-moe-13
#   bash scripts/verify_reproducibility.sh
#
# Expected output: "RESULT: PASS" with exit code 0.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== Albert-MoE-13 Reproducibility Verification ==="
echo "Project root: $PROJECT_ROOT"
echo ""

# Ensure we're in the project root
cd "$PROJECT_ROOT"

# Build repro_check in release mode
echo "Building repro_check..."
cargo build --release -p moe-llm-core --bin repro_check 2>&1 | grep -E "Compiling|Finished|error"
echo ""

# Run the check
echo "Running reproducibility checks..."
./target/release/repro_check
EXIT_CODE=$?

echo ""
if [ "$EXIT_CODE" -eq 0 ]; then
    echo "=== Verification PASSED — Albert-MoE-13 is reproducible ==="
else
    echo "=== Verification FAILED (exit code $EXIT_CODE) ==="
    exit "$EXIT_CODE"
fi
