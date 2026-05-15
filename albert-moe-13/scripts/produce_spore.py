#!/usr/bin/env python3
"""
produce_spore.py — export current albert. checkpoint as a spore for federated ingestion.

Usage:
    python3 scripts/produce_spore.py --name zabih
    python3 scripts/produce_spore.py --name lisa --spores-repo ~/projects/albert-spores
    python3 scripts/produce_spore.py --name zabih --epoch 200 --loss 10.31

Produces:
    {spores-repo}/spores/{name}/{YYYY-MM-DD}/
        spore_ep{epoch}_{loss}.safetensors   ← full checkpoint
        spore_ep{epoch}_{loss}.json          ← metadata for EvolutionManager
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from datetime import datetime

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

MODELS_DIR  = os.path.join(HERE, "models")
DASH_DIR    = os.path.join(HERE, "dashboard")
EPOCH_LOG   = os.path.join(DASH_DIR, "epoch_history.log")
CONFIG_FILE = os.path.join(MODELS_DIR, "albert_v3.0.config.json")

EPOCH_SUMMARY_RE = re.compile(
    r"EPOCH_SUMMARY epoch=(\d+) loss_avg=([\d.]+)"
)

def find_checkpoint():
    """Return the path to the best available safetensors checkpoint."""
    candidates = [
        os.path.join(MODELS_DIR, "albert_v3.0.safetensors"),
        os.path.join(MODELS_DIR, "albert_v3.0_best.safetensors"),
    ]
    for c in candidates:
        if os.path.exists(c):
            return c
    # fallback: any .safetensors in models/
    for f in sorted(os.listdir(MODELS_DIR)):
        if f.endswith(".safetensors"):
            return os.path.join(MODELS_DIR, f)
    return None

def read_best_epoch():
    """Parse epoch_history.log for the best epoch and its loss."""
    best_ep, best_loss = None, float("inf")
    if not os.path.exists(EPOCH_LOG):
        return best_ep, best_loss
    with open(EPOCH_LOG) as f:
        for line in f:
            m = EPOCH_SUMMARY_RE.search(line)
            if m:
                ep   = int(m.group(1))
                loss = float(m.group(2))
                if loss < best_loss:
                    best_loss = loss
                    best_ep   = ep
    return best_ep, best_loss

def git_short_sha(path):
    try:
        return subprocess.check_output(
            ["git", "rev-parse", "--short", "HEAD"], cwd=path, stderr=subprocess.DEVNULL
        ).decode().strip()
    except Exception:
        return "unknown"

def main():
    parser = argparse.ArgumentParser(description="Export albert. checkpoint as a spore")
    parser.add_argument("--name",        required=True,   help="Contributor name (e.g. zabih)")
    parser.add_argument("--spores-repo", default=None,    help="Path to albert-spores git repo (default: ~/projects/albert-spores)")
    parser.add_argument("--epoch",       type=int,        help="Override epoch number")
    parser.add_argument("--loss",        type=float,      help="Override loss value")
    parser.add_argument("--corpus-mix",  default="default", help="Describe corpus mix used (e.g. 'standard+hu')")
    parser.add_argument("--notes",       default="",      help="Free-form notes about this spore")
    parser.add_argument("--dry-run",     action="store_true", help="Print what would happen without writing")
    args = parser.parse_args()

    spores_repo = args.spores_repo or os.path.expanduser("~/projects/albert-spores")

    # ── Find checkpoint ───────────────────────────────────────────────────────
    checkpoint = find_checkpoint()
    if not checkpoint:
        print(f"[produce_spore] ERROR: no .safetensors found in {MODELS_DIR}", file=sys.stderr)
        sys.exit(1)
    print(f"[produce_spore] checkpoint: {checkpoint} ({os.path.getsize(checkpoint)//1024//1024} MB)")

    # ── Read best epoch / loss from log ───────────────────────────────────────
    log_ep, log_loss = read_best_epoch()
    epoch = args.epoch if args.epoch is not None else (log_ep or 0)
    loss  = args.loss  if args.loss  is not None else (log_loss if log_loss < float("inf") else 0.0)

    print(f"[produce_spore] epoch={epoch}  loss={loss:.4f}")

    # ── Build output paths ────────────────────────────────────────────────────
    date_str  = datetime.now().strftime("%Y-%m-%d")
    spore_name = f"spore_ep{epoch}_{loss:.4f}"
    out_dir   = os.path.join(spores_repo, "spores", args.name, date_str)

    safetensors_out = os.path.join(out_dir, f"{spore_name}.safetensors")
    meta_out        = os.path.join(out_dir, f"{spore_name}.json")

    if args.dry_run:
        print(f"[produce_spore] DRY RUN — would write:")
        print(f"  {safetensors_out}")
        print(f"  {meta_out}")
        return

    # ── Write ─────────────────────────────────────────────────────────────────
    os.makedirs(out_dir, exist_ok=True)
    print(f"[produce_spore] copying checkpoint → {safetensors_out}")
    shutil.copy2(checkpoint, safetensors_out)

    # Read config for architecture metadata
    arch = {}
    if os.path.exists(CONFIG_FILE):
        with open(CONFIG_FILE) as f:
            arch = json.load(f)

    meta = {
        "contributor":      args.name,
        "date":             date_str,
        "epoch_produced":   epoch,
        "loss_at_production": round(loss, 6),
        "corpus_mix":       args.corpus_mix,
        "notes":            args.notes,
        "base_checkpoint":  git_short_sha(HERE),
        "architecture": {
            "num_layers":   arch.get("num_layers"),
            "hidden_size":  arch.get("hidden_size"),
            "num_experts":  arch.get("num_experts"),
            "vocab_size":   arch.get("vocab_size"),
        },
        "hardware":         "unknown",  # overridable
        "safetensors_size_bytes": os.path.getsize(safetensors_out),
    }

    with open(meta_out, "w") as f:
        json.dump(meta, f, indent=2)
    print(f"[produce_spore] metadata → {meta_out}")

    # ── Git commit + push ─────────────────────────────────────────────────────
    if not os.path.isdir(os.path.join(spores_repo, ".git")):
        print(f"[produce_spore] WARNING: {spores_repo} is not a git repo — skipping push")
        print(f"[produce_spore] Spore written locally. Push manually:")
        print(f"  cd {spores_repo} && git add . && git commit -m 'spore: {args.name} ep{epoch}' && git push")
        return

    subprocess.run(["git", "add", safetensors_out, meta_out], cwd=spores_repo, check=True)
    subprocess.run(
        ["git", "commit", "-m", f"spore: {args.name} ep{epoch} loss={loss:.4f}"],
        cwd=spores_repo, check=True
    )
    subprocess.run(["git", "push"], cwd=spores_repo, check=True)
    print(f"[produce_spore] pushed to albert-spores")
    print(f"[produce_spore] done — spore is live")

if __name__ == "__main__":
    main()
