#!/bin/bash
# ─── TernFlow Studio Hot-Patch Utility ──────────────────────────────────────
# Syncs frontend assets directly to a running Fly.io instance.
# bypasses full Docker build for JS/CSS/HTML changes.

APP_NAME="ternlang-api" # Matches fly.toml
REMOTE_PATH="/ternlang-studio"

if [[ -z $(command -v flyctl) ]]; then
  echo "❌ Error: flyctl not found. Please install Fly CLI."
  exit 1
fi

echo "🚀 Initiating Hot-Patch for $APP_NAME..."

# Ensure remote path exists
flyctl ssh console -a $APP_NAME -C "mkdir -p $REMOTE_PATH"

# Sync studio files
for file in ternlang-studio/*.js; do
  echo "  -> Patching $file..."
  flyctl sftp shell -a $APP_NAME -c "put $file $REMOTE_PATH/$(basename $file)"
done

for file in ternlang-studio/*.html; do
  echo "  -> Patching $file..."
  flyctl sftp shell -a $APP_NAME -c "put $file $REMOTE_PATH/$(basename $file)"
done

# Sync assets folder if it exists
if [ -d "ternlang-studio/assets" ]; then
  echo "  -> Syncing assets folder..."
  # SFTP 'put -r' for the whole directory
  flyctl sftp shell -a $APP_NAME -c "put -r ternlang-studio/assets $REMOTE_PATH/"
fi

echo "✅ Hot-Patch Complete. Refresh your browser."
