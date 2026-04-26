#!/bin/bash

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

# Start the Rust API
echo "[API] Starting ternlang-api on port $PORT..."
exec ternlang-api
