#!/bin/bash
# ternlang.com deployment script
# Run on your VPS after pointing GoDaddy DNS → server IP
#
# Requirements: Caddy, Rust toolchain
# Usage: TERNLANG_API_KEY=your-key bash deploy.sh

set -e

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WEB_DIR="/var/www/ternlang"
API_KEY="${TERNLANG_API_KEY:-dev-key}"

echo "[deploy] Building ternlang-api..."
cd "$REPO_ROOT"
cargo build --release -p ternlang-api

echo "[deploy] Deploying landing page to $WEB_DIR..."
sudo mkdir -p "$WEB_DIR"
sudo cp "$REPO_ROOT/ternlang-web/index.html" "$WEB_DIR/index.html"

echo "[deploy] Installing Caddyfile..."
sudo cp "$REPO_ROOT/ternlang-web/Caddyfile" /etc/caddy/Caddyfile
sudo systemctl reload caddy

echo "[deploy] Starting ternlang-api on port 3731..."
# Stop existing instance if running
pkill -f ternlang-api 2>/dev/null || true
sleep 1
TERNLANG_API_KEY="$API_KEY" PORT=3731 \
  nohup "$REPO_ROOT/target/release/ternlang-api" \
  >> /var/log/ternlang-api.log 2>&1 &

echo "[deploy] Done!"
echo "[deploy] API: https://ternlang.com/api/trit_decide"
echo "[deploy] Web: https://ternlang.com"
