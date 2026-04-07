#!/bin/bash
# --- RFI-IRFOS Triadic Git Bridge ---
# Ensures absolute alignment between local and remote repositories.
# License: Tier 3 (Enterprise) - Internal Use Only.

REPO_DIR="/home/eri-irfos/Desktop/Ternary Intelligence Stack (TIS)"
TOKEN="${GITHUB_TOKEN}"
POLL_INTERVAL=60 # Seconds

cd "$REPO_DIR" || exit 1

sync_now() {
    echo "[$(date +%T)] INFO | Initiating Triadic Sync..."
    
    # 1. Fetch latest from remote
    git fetch origin main --quiet
    
    # 2. Check if we are behind
    LOCAL=$(git rev-parse @)
    REMOTE=$(git rev-parse @{u})
    BASE=$(git merge-base @ @{u})

    if [ $LOCAL = $REMOTE ]; then
        echo "[$(date +%T)] INFO | System state: Synchronized."
    elif [ $LOCAL = $BASE ]; then
        echo "[$(date +%T)] WARN | Local outdated. Fast-forwarding..."
        git pull --rebase origin main
    elif [ $REMOTE = $BASE ]; then
        echo "[$(date +%T)] INFO | Local has ahead-of-state commits. Pushing..."
        git push origin main
    else
        echo "[$(date +%T)] ERROR | Divergent state detected. Manual resolution required."
    fi
}

watch_sync() {
    echo "[$(date +%T)] START | Git Bridge Daemon Active. Polling every ${POLL_INTERVAL}s."
    while true; do
        sync_now
        sleep $POLL_INTERVAL
    done
}

case "$1" in
    "sync")
        sync_now
        ;;
    "watch")
        watch_sync
        ;;
    *)
        echo "Usage: $0 {sync|watch}"
        exit 1
        ;;
esac
