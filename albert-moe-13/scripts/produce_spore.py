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
LOCAL_LOG   = os.path.expanduser("~/.albert/training.log")
CONFIG_FILE = os.path.join(MODELS_DIR, "albert_v3.0.config.json")

# Canonical checkpoint names written by train_bible
CANONICAL_CHECKPOINTS = [
    "albert_v3.0.safetensors",
    "albert_v3.0_best.safetensors",
]

EPOCH_SUMMARY_RE = re.compile(
    r"EPOCH_SUMMARY epoch=(\d+) loss_avg=([\d.]+)"
)

def find_checkpoint():
    """Return the canonical training checkpoint, or None with a clear error."""
    for name in CANONICAL_CHECKPOINTS:
        path = os.path.join(MODELS_DIR, name)
        if os.path.exists(path):
            return path
    # No canonical checkpoint — don't fall back to arbitrary .safetensors
    return None

def read_best_epoch():
    """Parse training log for the best epoch and loss.

    Checks ~/.albert/training.log first (local CPU training output),
    then falls back to dashboard/epoch_history.log (main-run history).
    """
    best_ep, best_loss = None, float("inf")
    for log_path in [LOCAL_LOG, EPOCH_LOG]:
        if not os.path.exists(log_path):
            continue
        with open(log_path) as f:
            for line in f:
                m = EPOCH_SUMMARY_RE.search(line)
                if m:
                    ep   = int(m.group(1))
                    loss = float(m.group(2))
                    if loss < best_loss:
                        best_loss = loss
                        best_ep   = ep
        if best_ep is not None:
            return best_ep, best_loss  # local log had data — stop here
    return best_ep, best_loss

def git_short_sha(path):
    try:
        return subprocess.check_output(
            ["git", "rev-parse", "--short", "HEAD"], cwd=path, stderr=subprocess.DEVNULL
        ).decode().strip()
    except Exception:
        return "unknown"

def gh_username():
    """Return the authenticated GitHub username, or fall back to the OS login."""
    try:
        name = subprocess.check_output(
            ["gh", "api", "user", "--jq", ".login"],
            stderr=subprocess.DEVNULL,
        ).decode().strip()
        if name:
            return name
    except Exception:
        pass
    try:
        return subprocess.check_output(["whoami"], stderr=subprocess.DEVNULL).decode().strip()
    except Exception:
        return "contributor"


def main():
    parser = argparse.ArgumentParser(description="Export albert. checkpoint as a spore")
    parser.add_argument("--name",        default=None,    help="Contributor name — defaults to your GitHub login")
    parser.add_argument("--spores-repo", default=None,    help="Path to albert-spores git repo (default: ~/projects/albert-spores)")
    parser.add_argument("--epoch",       type=int,        help="Override epoch number")
    parser.add_argument("--loss",        type=float,      help="Override loss value")
    parser.add_argument("--corpus-mix",  default="default", help="Describe corpus mix used (e.g. 'standard+hu')")
    parser.add_argument("--notes",       default="",      help="Free-form notes about this spore")
    parser.add_argument("--dry-run",     action="store_true", help="Print what would happen without writing")
    args = parser.parse_args()

    if not args.name:
        args.name = gh_username()
        print(f"[produce_spore] contributor: {args.name} (from gh auth)")

    spores_repo = args.spores_repo or os.path.expanduser("~/projects/albert-spores")

    # ── Find checkpoint ───────────────────────────────────────────────────────
    checkpoint = find_checkpoint()
    if not checkpoint:
        print(f"[produce_spore] ERROR: no training checkpoint found in {MODELS_DIR}", file=sys.stderr)
        print(f"[produce_spore] Run  albert-train  and let it complete at least one full epoch,", file=sys.stderr)
        print(f"[produce_spore] then run  albert-spore  again.", file=sys.stderr)
        sys.exit(1)
    size_mb = os.path.getsize(checkpoint) // 1024 // 1024
    print(f"[produce_spore] checkpoint: {checkpoint} ({size_mb} MB)")

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

    # Skip commit if nothing changed (same spore already committed).
    nothing_staged = subprocess.run(
        ["git", "diff", "--cached", "--quiet"], cwd=spores_repo
    ).returncode == 0
    if nothing_staged:
        print(f"[produce_spore] spore already committed — nothing new to push")
        print(f"[produce_spore] done — spore is live")
        return

    # Use -c flags so git identity is never required in global config.
    # GitHub's noreply address keeps contributor email private by default.
    git_email = f"{args.name}@users.noreply.github.com"
    subprocess.run(
        ["git",
         "-c", f"user.name={args.name}",
         "-c", f"user.email={git_email}",
         "commit", "-m", f"spore: {args.name} ep{epoch} loss={loss:.4f}"],
        cwd=spores_repo, check=True
    )
    # Pull any remote commits (e.g. maintainer fixes) before pushing.
    subprocess.run(["git", "pull", "--rebase"], cwd=spores_repo, check=True)
    subprocess.run(["git", "push"], cwd=spores_repo, check=True)
    print(f"[produce_spore] pushed to albert-spores")
    print(f"[produce_spore] done — spore is live")

if __name__ == "__main__":
    main()
