#!/usr/bin/env bash
# Albert MoE-13 — TIS Benchmark Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/eriirfos-eng/ternary-intelligence-stack/main/albert-moe-13/bench/install.sh | sh
set -e

RELEASE="https://github.com/eriirfos-eng/ternary-intelligence-stack/releases/download/bench-v2.0.0"
DIR="albert-bench"

OS=$(uname -s)
ARCH=$(uname -m)

case "${OS}-${ARCH}" in
    Linux-x86_64)   BINARY="moe-test-linux-x86_64" ;;
    Linux-aarch64)  BINARY="moe-test-linux-aarch64" ;;
    Darwin-arm64)   BINARY="moe-test-macos-arm64" ;;
    Darwin-x86_64)  BINARY="moe-test-macos-x86_64" ;;
    *)
        echo "Unsupported platform: ${OS}-${ARCH}"
        echo "Please build from source: cargo build --release -p moe-test"
        echo "Repository: https://github.com/eriirfos-eng/ternary-intelligence-stack"
        exit 1
        ;;
esac

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Albert MoE-13 — TIS Benchmark Suite v2.0.0        ║"
echo "║         Ternary Intelligence Stack / RFI-IRFOS             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Platform : ${OS}-${ARCH}"
echo "Dest     : ./${DIR}/"
echo ""

mkdir -p "${DIR}/models" "${DIR}/data"

echo "[1/5] Downloading binary..."
curl -fsSL --progress-bar "${RELEASE}/${BINARY}" -o "${DIR}/moe-test"
chmod +x "${DIR}/moe-test"

echo "[2/5] Downloading model weights (142 MB)..."
curl -fsSL --progress-bar "${RELEASE}/bible_ternary_v2.0.0.safetensors" \
    -o "${DIR}/models/bible_ternary_v2.0.0.safetensors"

echo "[3/5] Downloading model config..."
curl -fsSL --progress-bar "${RELEASE}/bible_ternary_v2.0.0.config.json" \
    -o "${DIR}/models/bible_ternary_v2.0.0.config.json"
curl -fsSL --progress-bar "${RELEASE}/bible_ternary_v2.0.0.meta" \
    -o "${DIR}/models/bible_ternary_v2.0.0.meta"

echo "[4/5] Downloading vocabulary + eval sample..."
curl -fsSL --progress-bar "${RELEASE}/vocab.json" -o "${DIR}/data/vocab.json"
curl -fsSL --progress-bar "${RELEASE}/eval_sample.txt" -o "${DIR}/eval_sample.txt"

echo "[5/5] Running benchmark suite..."
echo ""

cd "${DIR}"
./moe-test --bench --csv albert_bench_results.csv

echo ""
echo "Results saved to ./${DIR}/albert_bench_results.csv"
echo "To run again: cd ${DIR} && ./moe-test --bench"
echo "Interactive TUI: cd ${DIR} && ./moe-test"
