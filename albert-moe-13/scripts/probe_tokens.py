#!/usr/bin/env python3
"""
probe_tokens.py — snapshot the token embedding neighborhood for all canonical probe tokens.

Usage:
    python3 scripts/probe_tokens.py --label pre-s6 --epoch 2064 --arch 17 --loss 9.8117
    python3 scripts/probe_tokens.py --label post-s6 --epoch 2120 --arch 18 --loss 9.71
    python3 scripts/probe_tokens.py --label pre-s6 --epoch 2064 --arch 17 --loss 9.8117 --host localhost:8888

Writes to:
    token-probes/snapshots/<label>_ep<epoch>_<arch>L/
        manifest.json
        <token>.json   (one per probe token)

Requires the dashboard server to be running (albert-train starts it automatically).
"""

import argparse
import json
import os
import sys
import urllib.request
import urllib.parse
from datetime import datetime, timezone

HERE    = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
PROBES  = os.path.join(HERE, "token-probes", "tokens.json")
SNAPS   = os.path.join(HERE, "token-probes", "snapshots")

NEIGHBORS_K = 50


def query(host: str, word: str, k: int = NEIGHBORS_K) -> dict:
    url = f"http://{host}/api/mycelium/query?word={urllib.parse.quote(word)}&k={k}"
    try:
        with urllib.request.urlopen(url, timeout=10) as r:
            return json.loads(r.read())
    except Exception as e:
        return {"error": str(e), "word": word}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--label",  required=True,
                    help="snapshot label, e.g. pre-s6, post-s6, pre-s7")
    ap.add_argument("--epoch",  required=True, type=int,
                    help="global epoch at time of probe")
    ap.add_argument("--arch",   required=True, type=int,
                    help="number of layers, e.g. 17")
    ap.add_argument("--loss",   required=True, type=float,
                    help="epoch-average loss at time of probe")
    ap.add_argument("--host",   default="localhost:8888",
                    help="dashboard server host:port (default localhost:8888)")
    ap.add_argument("--k",      default=NEIGHBORS_K, type=int,
                    help=f"neighbors to fetch per token (default {NEIGHBORS_K})")
    ap.add_argument("--note",   default="",
                    help="optional human note for this snapshot")
    args = ap.parse_args()

    # Load canonical token list
    with open(PROBES) as f:
        probe_cfg = json.load(f)
    tokens = [t["token"] for t in probe_cfg["tokens"]]

    # Build snapshot directory name
    snap_name = f"{args.label}_ep{args.epoch}_{args.arch}L"
    snap_dir  = os.path.join(SNAPS, snap_name)
    os.makedirs(snap_dir, exist_ok=True)

    ts = datetime.now(timezone.utc).isoformat(timespec="seconds")

    print(f"[probe_tokens] snapshot: {snap_name}")
    print(f"[probe_tokens] server:   {args.host}")
    print(f"[probe_tokens] tokens:   {', '.join(tokens)}")
    print()

    results = {}
    for tok in tokens:
        print(f"  querying '{tok}' ...", end=" ", flush=True)
        data = query(args.host, tok, args.k)
        if "error" in data:
            print(f"ERROR — {data['error']}")
        else:
            n = len(data.get("neighbors", []))
            top3 = ", ".join(x["word"] for x in data["neighbors"][:3])
            print(f"{n} neighbors  top3: [{top3}]")
        results[tok] = data
        # Write individual token file
        out = os.path.join(snap_dir, f"{tok}.json")
        with open(out, "w") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)

    # Write manifest
    manifest = {
        "label":        args.label,
        "epoch":        args.epoch,
        "arch_layers":  args.arch,
        "loss":         args.loss,
        "timestamp":    ts,
        "note":         args.note,
        "host":         args.host,
        "neighbors_k":  args.k,
        "tokens":       tokens,
        "summary": {
            tok: {
                "resolved": results[tok].get("word", tok),
                "top5":     [x["word"] for x in results[tok].get("neighbors", [])[:5]],
                "top_sim":  results[tok]["neighbors"][0]["sim"] if results[tok].get("neighbors") else None,
            }
            for tok in tokens
        }
    }
    with open(os.path.join(snap_dir, "manifest.json"), "w") as f:
        json.dump(manifest, f, indent=2, ensure_ascii=False)

    print()
    print(f"[probe_tokens] written to token-probes/snapshots/{snap_name}/")
    print(f"[probe_tokens] manifest summary:")
    for tok, s in manifest["summary"].items():
        top = ", ".join(s["top5"])
        print(f"  {tok:10s} → [{top}]")


if __name__ == "__main__":
    main()
