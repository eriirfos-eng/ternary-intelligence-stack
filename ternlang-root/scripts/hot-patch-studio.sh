#!/bin/bash
# ─── TernFlow Studio Hot-Patch Utility ──────────────────────────────────────
# Syncs frontend assets directly to a running Fly.io instance.
# bypasses full Docker build for JS/CSS/HTML changes.

APP_NAME="ternlang-studio" # Update if your Fly app name differs
REMOTE_PATH="/ternlang-studio"

if [[ -z $(command -v flyctl) ]]; then
  echo "❌ Error: flyctl not found. Please install Fly CLI."
  exit 1
fi

echo "🚀 Initiating Hot-Patch for $APP_NAME..."

# Sync studio assets
flyctl ssh console -C "mkdir -p $REMOTE_PATH"
for file in ternlang-studio/*.js ternlang-studio/*.css ternlang-studio/*.html; do
  echo "  -> Patching $file..."
  flyctl sftp shell -c "put $file $REMOTE_PATH/$(basename $file)"
done

# Sync assets folder if it exists
if [ -d "ternlang-studio/assets" ]; then
  echo "  -> Syncing assets folder..."
  flyctl sftp shell -c "put -r ternlang-studio/assets $REMOTE_PATH/"
fi

echo "✅ Hot-Patch Complete. Refresh your browser."
