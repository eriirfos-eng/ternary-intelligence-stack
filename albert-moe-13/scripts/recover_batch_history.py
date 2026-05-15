#!/usr/bin/env python3
"""
Recover batch history from Modal volume's training.log.

Pulls /albert/dashboard/training.log from the volume into /tmp/,
parses all batch loss lines, and merges new points into batch_history.csv
without touching the live training.log in dashboard/.
"""

import os, re, subprocess, sys

HERE         = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BATCH_CSV    = os.path.join(HERE, "dashboard", "batch_history.csv")
TMP_LOG      = "/tmp/albert_vol_training.log"
VOL          = "albert-vol"
REMOTE       = "/albert/dashboard/training.log"
BATCHES_PER_EPOCH = 300

BATCH_RE = re.compile(
    r'Epoch\s+\d+\s+\(Global\s+(\d+)\),\s+Batch\s+(\d+):\s+loss\s*=\s*([\d.]+)'
)

def pull_log():
    print(f"[recover] pulling {REMOTE} from volume {VOL} → {TMP_LOG}")
    rc = subprocess.run(
        ["modal", "volume", "get", "--force", VOL, REMOTE, TMP_LOG],
        cwd=HERE
    ).returncode
    if rc != 0:
        print(f"[recover] ERROR: modal volume get exited {rc}", file=sys.stderr)
        sys.exit(rc)
    print(f"[recover] pulled {os.path.getsize(TMP_LOG):,} bytes")

def load_existing():
    seen = {}
    if not os.path.exists(BATCH_CSV):
        return seen
    with open(BATCH_CSV) as f:
        for line in f:
            comma = line.find(',')
            if comma < 0: continue
            try:
                x = float(line[:comma])
                y = float(line[comma+1:].strip())
                seen[x] = y
            except ValueError:
                pass
    return seen

def parse_log(path):
    points = {}
    with open(path, errors='replace') as f:
        for line in f:
            m = BATCH_RE.search(line)
            if not m: continue
            global_ep = int(m.group(1))
            batch     = int(m.group(2))
            loss      = float(m.group(3))
            x = global_ep + batch / BATCHES_PER_EPOCH
            points[x] = loss
    return points

def main():
    pull_log()

    existing = load_existing()
    print(f"[recover] existing batch_history.csv: {len(existing):,} points")

    vol_points = parse_log(TMP_LOG)
    print(f"[recover] volume training.log: {len(vol_points):,} batch points")

    new_points = {x: y for x, y in vol_points.items() if x not in existing}
    print(f"[recover] new points to merge: {len(new_points):,}")

    if not new_points:
        print("[recover] nothing to add — batch_history.csv already up to date")
        return

    # Merge: combine existing + new, sort by x, write back
    merged = {**existing, **new_points}
    merged_sorted = sorted(merged.items())

    with open(BATCH_CSV, 'w') as f:
        for x, y in merged_sorted:
            f.write(f"{x:.6f},{y}\n")

    print(f"[recover] wrote {len(merged_sorted):,} total points to batch_history.csv")
    print(f"[recover] done — hard refresh dashboard to see restored SMAs")

if __name__ == "__main__":
    main()
