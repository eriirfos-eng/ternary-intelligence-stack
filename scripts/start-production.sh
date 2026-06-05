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

# Local Watchdog: Kills the API if it hangs for 3 consecutive checks (30s)
# This triggers Fly.io's auto-restart policy.
(
  echo "[Watchdog] Starting stability monitor..."
  FAIL_COUNT=0
  while true; do
    sleep 10
    # Try to ping the health endpoint locally
    if curl -sf http://localhost:8080/health > /dev/null; then
      FAIL_COUNT=0
    else
      FAIL_COUNT=$((FAIL_COUNT + 1))
      echo "[Watchdog] Health check failed ($FAIL_COUNT/3)"
      if [ $FAIL_COUNT -ge 3 ]; then
        echo "[Watchdog] API hung. Killing process for auto-restart."
        kill -9 $(pgrep ternlang-api)
      fi
    fi
  done
) &
WATCHDOG_PID=$!

# Kill background processes when this script exits
trap "kill $KPI_PID $WATCHDOG_PID 2>/dev/null; wait $KPI_PID $WATCHDOG_PID 2>/dev/null" EXIT

# Start the Rust API (exec replaces this shell process; trap still fires on exit)
echo "[API] Starting ternlang-api on port $PORT..."
exec ternlang-api
