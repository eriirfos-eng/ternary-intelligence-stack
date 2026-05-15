#!/usr/bin/env python3
"""
merge_batch_history.py — merge ALL albert_full_*.csv downloads into batch_history.csv

Scans every CSV in ~/Desktop/Downloads/, extracts ## BATCH DATA rows, deduplicates
by x-coordinate (epoch + batch/300), and writes a clean sorted batch_history.csv.

Usage:
    python scripts/merge_batch_history.py [--dry-run]
"""

import csv
import glob
import os
import sys
from pathlib import Path

# Reject rows from other training runs — this run never went below ~10.2.
LOSS_MIN    = 9.5
LOSS_MAX    = 13.0

DOWNLOADS   = Path.home() / "Desktop" / "Downloads"
REPO_ROOT   = Path(__file__).parent.parent
HIST_PATH   = REPO_ROOT / "dashboard" / "batch_history.csv"
CSV_PATTERN = str(DOWNLOADS / "albert_full_*.csv")

def parse_csv(path: Path) -> dict[float, float]:
    """Return {x: y} from the ## BATCH DATA section of an albert_full CSV."""
    rows: dict[float, float] = {}
    in_batch = False
    header_skipped = False
    try:
        with open(path, encoding="utf-8", errors="replace") as f:
            for line in f:
                line = line.rstrip("\n")
                if line.strip() == "## BATCH DATA":
                    in_batch = True
                    header_skipped = False
                    continue
                if in_batch and line.startswith("##"):
                    in_batch = False
                    continue
                if not in_batch:
                    continue
                if not header_skipped:
                    header_skipped = True   # skip column header row
                    continue
                parts = line.split(",")
                if len(parts) < 5:
                    continue
                try:
                    step         = int(parts[0])
                    global_epoch = int(parts[1])
                    batch        = int(parts[2])
                    loss         = float(parts[4])
                    x = global_epoch + batch / 300.0
                    if not (LOSS_MIN <= loss <= LOSS_MAX):
                        continue  # skip rows from other training runs
                    rows[x] = loss
                except (ValueError, IndexError):
                    continue
    except Exception as e:
        print(f"  WARN: could not parse {path.name}: {e}")
    return rows

def main():
    dry_run = "--dry-run" in sys.argv

    files = sorted(glob.glob(CSV_PATTERN))
    if not files:
        print(f"No files found at {CSV_PATTERN}")
        sys.exit(1)
    print(f"Found {len(files)} albert_full CSV files")

    # Load existing batch_history.csv
    existing: dict[float, float] = {}
    if HIST_PATH.exists():
        with open(HIST_PATH) as f:
            reader = csv.reader(f)
            for row in reader:
                if len(row) == 2:
                    try:
                        existing[float(row[0])] = float(row[1])
                    except ValueError:
                        pass
        print(f"Existing batch_history.csv: {len(existing):,} rows")

    # Parse all CSVs
    all_points: dict[float, float] = dict(existing)
    for fpath in files:
        p = Path(fpath)
        new_rows = parse_csv(p)
        before = len(all_points)
        all_points.update(new_rows)
        added = len(all_points) - before
        if added > 0:
            ep_min = min((int(x) for x in new_rows), default=0)
            ep_max = max((int(x) for x in new_rows), default=0)
            print(f"  {p.name}: +{added:,} new points  (ep {ep_min}–{ep_max})")
        else:
            print(f"  {p.name}: no new points")

    total = len(all_points)
    print(f"\nTotal unique points: {total:,}  (was {len(existing):,}, +{total - len(existing):,})")

    if dry_run:
        print("Dry run — not writing.")
        return

    # Write sorted
    sorted_pts = sorted(all_points.items())
    with open(HIST_PATH, "w", newline="") as f:
        writer = csv.writer(f)
        for x, y in sorted_pts:
            writer.writerow([f"{x:.6f}", f"{y:.4f}"])

    print(f"Wrote {total:,} rows to {HIST_PATH}")

    # Report remaining gaps (epochs with 0 points)
    all_epochs = set(int(x) for x in all_points)
    if all_epochs:
        ep_min, ep_max = min(all_epochs), max(all_epochs)
        gaps = [e for e in range(ep_min, ep_max + 1) if e not in all_epochs]
        if gaps:
            # compress into ranges
            ranges, start = [], gaps[0]
            for i in range(1, len(gaps)):
                if gaps[i] != gaps[i-1] + 1:
                    ranges.append((start, gaps[i-1]))
                    start = gaps[i]
            ranges.append((start, gaps[-1]))
            print(f"\nRemaining gaps ({len(gaps)} epochs):")
            for a, b in ranges:
                print(f"  ep {a}–{b}  ({b-a+1} epochs)")
        else:
            print("\nNo gaps — full coverage from ep {ep_min} to ep {ep_max}")

if __name__ == "__main__":
    main()
