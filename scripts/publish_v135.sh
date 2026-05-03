#!/bin/bash
# Global Publish Script for TIS v1.3.5
set -e

# Configuration
VERSION="1.3.5"
DELAY=10 # Increased delay for crates.io indexing

echo "Starting global publication of TIS $VERSION..."

# Group 1: Ternlang Core Stack (Root Workspace / ternlang-root)
TERN_CRATES=(
    "ternlang-core"
    "ternlang-codegen"
    "ternlang-runtime"
    "ternlang-ml"
    "ternlang-api"
    "ternlang-moe"
    "ternlang-mcp"
    "ternlang-hdl"
    "ternpkg"
    "ternlang-compat"
    "ternlang-lsp"
    "ternlang-cli"
    "ternlang-ruvector"
    "ternlang-test"
    "ternlang-compress"
)

# Group 2: MoE-13 Neural Stack (albert-moe-13 workspace)
MOE_CRATES=(
    "moe-core"
    "moe-ddel"
    "moe-platform"
    "moe-plugin-sdk"
    "moe-runtime"
    "moe-sdk"
    "moe-uril"
    "moe-validation-suite"
)

# Group 3: Agent CLI Stack (agent_albert_cli workspace)
ALBERT_CRATES=(
    "albert-api"
    "albert-tools"
    "albert-commands"
    "albert-compat"
    "albert-runtime"
    "moe-llb"
    "moe-reference"
    "albert-cli"
)

# Group 4: New LLM Crates (Root workspace members)
NEW_CRATES=(
    "moe-llm-core"
    "moe-compute"
    "moe-test"
    "ternaudit-guard"
    "pytern"
)

publish_crate() {
    local workspace_path=$1
    local crate=$2
    echo "--- Attempting to publish $crate ---"
    if cargo publish --manifest-path "$workspace_path/Cargo.toml" -p "$crate" --allow-dirty --no-verify; then
        echo "SUCCESS: $crate"
        return 0
    else
        echo "RETRY LATER: $crate (might be dependency lag or already published)"
        return 1
    fi
}

# Iterative publication to handle dependencies
for i in {1..3}; do
    echo "=== PUBLICATION PASS $i ==="
    
    for c in "${TERN_CRATES[@]}"; do publish_crate "ternlang-root" "$c" || true; sleep $DELAY; done
    for c in "${MOE_CRATES[@]}"; do publish_crate "albert-moe-13" "$c" || true; sleep $DELAY; done
    for c in "${ALBERT_CRATES[@]}"; do publish_crate "agent_albert_cli/rust" "$c" || true; sleep $DELAY; done
    for c in "${NEW_CRATES[@]}"; do publish_crate "." "$c" || true; sleep $DELAY; done
done

echo "Global publication process finished."
