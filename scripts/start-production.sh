#!/bin/bash
set -e

# Ensure the data directory exists
mkdir -p /data/kpi

# Start the Python "Spine" in the background
# It updates the KPI JSON files every hour
(
  while true; do
    echo "[Spine] Syncing KPI metrics..."
    # Tell the script where to save the data in production
    KPI_OUTPUT_DIR="/data/kpi" python3 /usr/local/bin/generate_kpi.py
    sleep 3600
  done
) &
KPI_PID=$!

# Kill the background Spine loop when this script exits (crash or SIGTERM from Fly.io)
trap "kill $KPI_PID 2>/dev/null; wait $KPI_PID 2>/dev/null" EXIT

# Start the Rust API (exec replaces this shell process; trap still fires on exit)
echo "[API] Starting ternlang-api on port $PORT..."
exec ternlang-api
