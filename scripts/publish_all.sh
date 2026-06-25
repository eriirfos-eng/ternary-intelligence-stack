#!/bin/bash
set -e

cd /home/eri-irfos/projects/ternary-intelligence-stack

CRATES=(
    "ternlang-core"
    "ternlang-ml"
    "ternlang-codegen"
    "ternlang-runtime"
    "ternlang-compress"
    "ternlang-api"
    "ternlang-mcp"
    "ternlang-moe"
    "ternlang-ruvector"
    "ternlang-engram"
    "ternlang-test"
    "ternlang-hdl"
    "ternpkg"
    "ternlang-compat"
    "ternlang-lsp"
    "ternlang-cli"
)

# Keep looping through the crates until all are published (or max 5 iterations)
for i in {1..5}; do
    echo "Iteration $i"
    ALL_SUCCESS=true
    for crate in "${CRATES[@]}"; do
        echo "Attempting to publish $crate..."
        # If it fails, maybe it's already published or dependencies aren't ready yet
        if ! cargo publish --manifest-path ternlang-root/Cargo.toml -p $crate; then
            echo "Failed to publish $crate in iteration $i"
            ALL_SUCCESS=false
        else
            echo "Successfully published $crate!"
        fi
        sleep 1 # Be nice to crates.io API
    done
    if $ALL_SUCCESS; then
        echo "All ternlang crates published successfully!"
        break
    fi
done

ALBERT_CRATES=(
    "albert-runtime"
    "albert-api"
    "albert-commands"
    "albert-tools"
    "albert-compat"
    "albert-cli"
)

for crate in "${ALBERT_CRATES[@]}"; do
    echo "Attempting to publish $crate..."
    cargo publish --manifest-path agent_albert_cli/rust/Cargo.toml -p $crate || echo "Failed to publish $crate"
    sleep 1
done

echo "Done"
