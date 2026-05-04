#!/bin/bash
# TIS Triad Synthesis Validation
# Verifies the triad field formula against raw inputs via the compiler backend.

echo "--- Executing Triad Field Formula Validation ---"

# Step 1: Ensure ternlang is built
if [ ! -f "./target/release/ternlang" ]; then
    echo "ternlang binary not found. Building..."
    cargo build --release -p ternlang-cli
fi

# Step 2: Run the synthesis test
./target/release/ternlang run ./projects/ternary-intelligence-stack/ternlang-root/tests/triad_synthesis.tern

if [ $? -eq 0 ]; then
    echo "Validation Complete: [VERIFIED]"
else
    echo "Validation Failed: Synthesis formula mismatch."
    exit 1
fi
