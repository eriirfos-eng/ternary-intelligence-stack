#!/usr/bin/env bash
# promote_checkpoint.sh — Human authorization gate for albert. checkpoint promotion
#
# EU AI Act Art. 53(1)(e) requires technical measures for human oversight.
# This script is the MANDATORY gate between a trained checkpoint and any
# external deployment or distribution. It must be run and confirmed by the
# lead architect before any checkpoint is:
#   - Uploaded to a public release
#   - Made available via an API or demo
#   - Used as a base for further public fine-tuning
#
# Usage: ./scripts/promote_checkpoint.sh [checkpoint_path]
set -e

CHECKPOINT="${1:-models/albert_v3.0.best.safetensors}"
LOG="exports/promotion_log.jsonl"

if [ ! -f "$CHECKPOINT" ]; then
    echo "ERROR: Checkpoint not found: $CHECKPOINT"
    exit 1
fi

mkdir -p exports

echo ""
echo "========================================================"
echo "  albert. CHECKPOINT PROMOTION GATE"
echo "  EU AI Act Art. 53(1)(e) — Human Oversight"
echo "========================================================"
echo ""
echo "Checkpoint: $CHECKPOINT"
echo "Size:       $(du -sh "$CHECKPOINT" | cut -f1)"
echo "SHA256:     $(sha256sum "$CHECKPOINT" | cut -d' ' -f1)"
echo ""
echo "You are about to authorize this checkpoint for external use."
echo "Before proceeding, confirm the following:"
echo ""
echo "  [ ] Loss is at an acceptable level for the intended use case"
echo "  [ ] No known training instabilities in the last 50 epochs"
echo "  [ ] Benchmark suite has been run (albert-test /bench)"
echo "  [ ] MODEL_CARD.md reflects the current architecture and epoch"
echo "  [ ] DATA_PROVENANCE.md is up to date"
echo "  [ ] No PII or sensitive data in training corpus changes since last promotion"
echo "  [ ] SECURITY.md limitations are accurate for this checkpoint"
echo ""
read -p "Enter your name to authorize promotion (or Ctrl+C to abort): " AUTHORIZER
if [ -z "$AUTHORIZER" ]; then
    echo "Aborted — no authorizer name provided."
    exit 1
fi

read -p "Confirm intended deployment target (e.g. 'GitHub Release bench-v3.0.0', 'internal demo'): " TARGET
if [ -z "$TARGET" ]; then
    echo "Aborted — no deployment target specified."
    exit 1
fi

TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
SHA=$(sha256sum "$CHECKPOINT" | cut -d' ' -f1)
ENTRY="{\"timestamp\":\"$TS\",\"authorizer\":\"$AUTHORIZER\",\"checkpoint\":\"$CHECKPOINT\",\"sha256\":\"$SHA\",\"target\":\"$TARGET\"}"

echo "$ENTRY" >> "$LOG"

echo ""
echo "========================================================"
echo "  PROMOTION AUTHORIZED"
echo "  Logged to: $LOG"
echo "========================================================"
echo ""
echo "  Checkpoint: $CHECKPOINT"
echo "  Authorized: $AUTHORIZER"
echo "  Target:     $TARGET"
echo "  Timestamp:  $TS"
echo "  SHA256:     $SHA"
echo ""
echo "You may now proceed with the deployment."
