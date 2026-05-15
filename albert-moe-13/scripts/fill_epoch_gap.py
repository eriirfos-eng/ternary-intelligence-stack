#!/usr/bin/env python3
"""
Fill batch_history.csv gap from epoch_history.log.

For each EPOCH_SUMMARY not already covered by batch_history.csv,
inserts one representative point at x=epoch (loss_avg). This keeps
SMAs continuous across epochs where batch-level data was lost.
"""

import os, re

HERE      = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BATCH_CSV = os.path.join(HERE, "dashboard", "batch_history.csv")
EPOCH_LOG = os.path.join(HERE, "dashboard", "epoch_history.log")

EPOCH_RE = re.compile(r'^EPOCH_SUMMARY epoch=(\d+) loss_avg=([\d.]+)')

def load_batch_xs():
    xs = set()
    if not os.path.exists(BATCH_CSV):
        return xs
    with open(BATCH_CSV) as f:
        for line in f:
            comma = line.find(',')
            if comma > 0:
                try: xs.add(float(line[:comma]))
                except ValueError: pass
    return xs

def parse_epoch_log():
    epochs = {}
    with open(EPOCH_LOG) as f:
        for line in f:
            m = EPOCH_RE.match(line.strip())
            if m:
                ep  = int(m.group(1))
                avg = float(m.group(2))
                epochs[ep] = avg   # last occurrence wins (dedup)
    return epochs

def main():
    existing_xs = load_batch_xs()
    print(f"[fill] existing batch points: {len(existing_xs):,}")

    epoch_avgs = parse_epoch_log()
    print(f"[fill] epoch_history.log covers ep {min(epoch_avgs)}-{max(epoch_avgs)}")

    # Find epochs with no batch coverage
    new_points = []
    for ep, avg in sorted(epoch_avgs.items()):
        x = float(ep)
        if x not in existing_xs:
            new_points.append((x, avg))

    print(f"[fill] gap epochs to fill: {len(new_points)}")
    if not new_points:
        print("[fill] nothing to add")
        return

    for x, y in new_points:
        print(f"  ep{int(x):4d}  loss_avg={y}")

    # Load all existing, merge, sort, write
    existing = {}
    with open(BATCH_CSV) as f:
        for line in f:
            comma = line.find(',')
            if comma < 0: continue
            try:
                existing[float(line[:comma])] = line[comma+1:].strip()
            except ValueError: pass

    for x, y in new_points:
        existing[x] = str(y)

    merged = sorted(existing.items())
    with open(BATCH_CSV, 'w') as f:
        for x, y in merged:
            f.write(f"{x:.6f},{y}\n")

    print(f"[fill] wrote {len(merged):,} total points — hard refresh dashboard")

if __name__ == "__main__":
    main()
