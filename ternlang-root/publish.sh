#!/bin/bash
# ternlang crates.io publication script
# Run AFTER: cargo login   (paste token from crates.io → Account Settings → API Tokens)
#
# Publish order matters — each crate must be on crates.io before dependents.
# All inter-crate deps already carry version = "0.1.0".
#
# Usage: bash publish.sh
# Dry run only: bash publish.sh --dry-run

set -e
DRY=${1:-""}
FLAG=""
[ "$DRY" = "--dry-run" ] && FLAG="--dry-run" && echo "[publish] DRY RUN — nothing will actually upload"

publish() {
    echo ""
    echo "══════════════════════════════════════"
    echo "  Publishing: $1"
    echo "══════════════════════════════════════"
    cargo publish $FLAG -p "$1"
    # crates.io needs ~15s to index before dependents can resolve it
    [ -z "$FLAG" ] && sleep 20
}

# ── Tier 1: Open Core (LGPL-3.0) ──────────────────────────────────────────────
publish ternlang-core     # no internal deps
publish ternlang-hdl      # no internal deps (depends on ternlang-core)
publish ternlang-ml       # depends on ternlang-core
publish ternlang-compat   # depends on ternlang-core
publish ternlang-lsp      # depends on ternlang-core
publish ternlang-runtime  # depends on ternlang-core
publish ternlang-cli      # depends on all of the above

# ── Tier 2: BSL crates ────────────────────────────────────────────────────────
# Note: BSL-licensed crates may be published to crates.io for visibility.
# Users are subject to the BSL 1.1 license terms.
# Uncomment when ready:
# publish ternlang-mcp
# publish ternlang-api

echo ""
echo "✓ All open-core crates published."
echo "  → https://crates.io/search?q=ternlang"
