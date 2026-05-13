#!/usr/bin/env bash
# Albert MoE-13 — TIS Benchmark Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/albert-moe-13/benchmarks/install.sh | sh
set -e

RELEASE="https://github.com/eriirfos-eng/ternary-intelligence-stack/releases/download/bench-v2.0.0"
REPO="https://github.com/eriirfos-eng/ternary-intelligence-stack.git"
DIR="albert-bench"
ORIG_DIR="$(pwd)"

OS=$(uname -s)
ARCH=$(uname -m)

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Albert MoE-13 — TIS Benchmark Suite v2.0.0        ║"
echo "║         Ternary Intelligence Stack / RFI-IRFOS             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Platform : ${OS}-${ARCH}"
echo "Dest     : ./${DIR}/"
echo ""

mkdir -p "${DIR}/models" "${DIR}/data"

# ── Detect Termux (Android) ──────────────────────────────────────────────────
IS_TERMUX=0
if [ -n "${TERMUX_VERSION}" ] || [ -d "/data/data/com.termux" ]; then
    IS_TERMUX=1
fi

# ── Source-build helper (macOS, Termux, Linux ARM64) ─────────────────────────
source_build() {
    local PKG_MANAGER="$1"   # "brew", "pkg", or "apt"

    # Ensure git
    if ! command -v git &>/dev/null; then
        case "$PKG_MANAGER" in
            pkg) echo "  Installing git via pkg..."; pkg install -y git ;;
            apt) echo "  Installing git via apt..."; apt-get install -y git ;;
            *)
                echo "Error: git is required."
                echo "Install Xcode Command Line Tools:  xcode-select --install"
                exit 1
                ;;
        esac
    fi

    # Ensure Rust/cargo
    if ! command -v cargo &>/dev/null; then
        case "$PKG_MANAGER" in
            pkg)
                echo "  Installing Rust via pkg..."
                pkg install -y rust
                ;;
            *)
                echo "  Rust not found — installing via rustup..."
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
                    | sh -s -- -y --no-modify-path
                export PATH="${HOME}/.cargo/bin:${PATH}"
                ;;
        esac
    else
        echo "  Rust found: $(cargo --version)"
    fi

    # Sparse clone — source only, skip large model blobs
    TMPCLONE=$(mktemp -d)
    trap 'rm -rf "$TMPCLONE"' EXIT

    echo "  Cloning source (sparse)..."
    git clone \
        --depth=1 \
        --filter=blob:none \
        --sparse \
        "$REPO" "$TMPCLONE" 2>&1 | grep -v "^$"

    cd "$TMPCLONE"
    git sparse-checkout set albert-moe-13 2>/dev/null
    git checkout 2>/dev/null

    echo "  Building moe-test..."
    cd albert-moe-13

    # Bypass the workspace Cargo.toml entirely — it references crates not
    # included in the sparse clone (moe-ddel, moe-platform, etc.).
    # --manifest-path points directly at moe-test; path deps (moe-llm-core)
    # resolve via the relative path in moe-test/Cargo.toml without needing
    # the workspace. --target-dir keeps the binary at the expected path.
    cargo build --release \
        --manifest-path moe-test/Cargo.toml \
        --target-dir ./target \
        2>&1 | grep -E "^(error|warning: unused|   Compiling|    Finished)"

    cp target/release/moe-test "${ORIG_DIR}/${DIR}/moe-test"
    chmod +x "${ORIG_DIR}/${DIR}/moe-test"
    cd "${ORIG_DIR}"
    echo "  Build complete."
    echo ""
}

# ── Binary: pre-built for Linux x86_64, source-build everywhere else ─────────
if [ "$IS_TERMUX" -eq 1 ]; then
    echo "[1/5] Termux (Android) detected — building from source (one-time, ~10-20 min)..."
    echo "      Make sure you have a working internet connection and ~500 MB free."
    echo ""
    source_build "pkg"

else
    case "${OS}-${ARCH}" in
        Linux-x86_64)
            echo "[1/5] Downloading binary (Linux x86_64)..."
            curl -fsSL --progress-bar "${RELEASE}/moe-test-linux-x86_64" -o "${DIR}/moe-test"
            chmod +x "${DIR}/moe-test"
            ;;

        Linux-aarch64)
            echo "[1/5] Linux ARM64 — building from source (one-time, ~10 min)..."
            echo ""
            source_build "apt"
            ;;

        Darwin-*)
            echo "[1/5] macOS detected — building from source (one-time, ~5-10 min)..."
            echo ""
            source_build "brew"
            ;;

        *)
            echo "Unsupported platform: ${OS}-${ARCH}"
            echo ""
            echo "Windows users: install WSL2, then re-run this script inside WSL."
            echo "  wsl --install   (in PowerShell as Admin, then restart)"
            echo ""
            echo "Or build from source:"
            echo "  git clone ${REPO}"
            echo "  cd ternary-intelligence-stack/albert-moe-13"
            echo "  cargo build --release -p moe-test"
            exit 1
            ;;
    esac
fi

# ── Model assets (same for all platforms) ────────────────────────────────────
echo "[2/5] Downloading model weights (142 MB)..."
curl -fsSL --progress-bar "${RELEASE}/bible_ternary_v2.0.0.safetensors" \
    -o "${DIR}/models/bible_ternary_v2.0.0.safetensors"

echo "[3/5] Downloading model config..."
curl -fsSL --progress-bar "${RELEASE}/bible_ternary_v2.0.0.config.json" \
    -o "${DIR}/models/bible_ternary_v2.0.0.config.json"
curl -fsSL --progress-bar "${RELEASE}/bible_ternary_v2.0.0.meta" \
    -o "${DIR}/models/bible_ternary_v2.0.0.meta"

echo "[4/5] Downloading vocabulary + eval sample (WikiText-2, held-out)..."
curl -fsSL --progress-bar "${RELEASE}/vocab.json" -o "${DIR}/data/vocab.json"
curl -fsSL --progress-bar "${RELEASE}/eval_sample.txt" -o "${DIR}/eval_sample.txt"

echo "[5/5] Running benchmark suite..."
echo ""

cd "${DIR}"
./moe-test --bench --csv albert_bench_results.csv

echo ""
echo "Results saved to ./${DIR}/albert_bench_results.csv"
echo "To run again:    cd ${DIR} && ./moe-test --bench"
echo "Interactive TUI: cd ${DIR} && ./moe-test"
