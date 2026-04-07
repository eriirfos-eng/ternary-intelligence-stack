#!/bin/bash
set -e

# TIS IPFS Bridge
# "If the centralized web burns, the Triad remains."
# Mirrors TIS standards to the permanent web.

SPEC_DIR="ternlang-root/spec"
STANDARDS_DIR="$SPEC_DIR/standards"

BLUE='\033[0;34m'
GREEN='\033[0;32m'
NC='\033[0m'

echo -e "${BLUE}=== TERNARY INTELLIGENCE STACK (TIS) IPFS BRIDGE ===${NC}"

if ! command -v ipfs &> /dev/null; then
    echo -e "${BLUE}[INFO]${NC} ipfs command not found. Please install Kubo (ipfs) to deploy."
    echo "       See https://docs.ipfs.tech/install/command-line/"
    exit 0
fi

# 1. Prepare mirror directory
MIRROR_DIR=".ipfs_mirror"
rm -rf "$MIRROR_DIR"
mkdir -p "$MIRROR_DIR"

echo -e "Mirroring standards from ${GREEN}$STANDARDS_DIR${NC}..."
cp -r "$STANDARDS_DIR"/* "$MIRROR_DIR/"
cp "$SPEC_DIR"/grammar.ebnf "$MIRROR_DIR/"
cp "$SPEC_DIR"/*.md "$MIRROR_DIR/"
cp README.md "$MIRROR_DIR/"
cp CITATION.cff "$MIRROR_DIR/"

# 2. Add to IPFS
echo -e "${BLUE}Pinning TIS Standards to local IPFS node...${NC}"
HASH=$(ipfs add -rQ "$MIRROR_DIR")

echo -e "----------------------------------------------------"
echo -e "${GREEN}SUCCESS: Standards mirrored to IPFS!${NC}"
echo -e "Local CID: ${BLUE}$HASH${NC}"
echo -e "Public Gateway: ${BLUE}https://ipfs.io/ipfs/$HASH${NC}"
echo "----------------------------------------------------"
echo "Distribute this CID to all RFI-IRFOS nodes to ensure reachability."
