#!/usr/bin/env python3
"""
spore_inject.py — Inject colony spore text into the albert. chaos corpus.

For each contributor in albert-spores, picks the best checkpoint (lowest loss
with a real safetensors file), runs albert_serve on it, generates text samples,
writes them to data/corpus/chaos/spores_<contributor>.txt, then rebuilds
corpus_cache.bin and pushes it to the Modal volume.

Usage:
  python3 scripts/spore_inject.py [options]

Options:
  --spores-dir DIR      Path to albert-spores/spores/ (default: ~/projects/albert-spores/spores)
  --samples N           Text samples to generate per contributor (default: 300)
  --max-tokens N        Max tokens per sample (default: 128)
  --temperature F       Sampling temperature (default: 0.85)
  --contributors NAMES  Comma-separated contributors to process (default: all)
  --dry-run             Generate text but skip retokenize and Modal push
  --no-modal            Retokenize locally but skip Modal volume push
  --port N              Port for albert_serve (default: 8765)
"""

import argparse
import json
import os
import random
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.request
import urllib.error
from pathlib import Path

# ── paths ──────────────────────────────────────────────────────────────────────
HERE        = Path(__file__).resolve().parent.parent   # albert-moe-13/
SPORES_REPO = Path.home() / "projects" / "albert-spores" / "spores"
CHAOS_DIR   = HERE / "data" / "corpus" / "chaos"
VOCAB_PATH  = HERE / "data" / "vocab_v3.json"
CACHE_BIN   = HERE / "data" / "corpus_cache.bin"
SERVE_BIN   = HERE / "target" / "release" / "albert_serve"
TOKENIZER   = HERE / "scripts" / "train_tokenizer_v3.py"
MODAL_VOL   = "albert-vol"

# ── safetensors check ──────────────────────────────────────────────────────────

def is_real_safetensors(path: Path) -> bool:
    """LFS pointer files are text starting with 'version https://'. Real ones are binary."""
    try:
        with open(path, "rb") as f:
            header = f.read(16)
        return not header.startswith(b"version https://")
    except OSError:
        return False

# ── spore discovery ────────────────────────────────────────────────────────────

def discover_spores(spores_dir: Path, contributors: list[str] | None) -> list[dict]:
    """
    Scan spores_dir for contributor subdirs. For each contributor, pick the
    checkpoint with the lowest loss that has a real (non-LFS) safetensors file.
    Returns a list of dicts: {contributor, json_path, st_path, manifest}.
    """
    selected = []

    for contrib_dir in sorted(spores_dir.iterdir()):
        if not contrib_dir.is_dir():
            continue
        contributor = contrib_dir.name
        if contributors and contributor not in contributors:
            continue

        # collect all json manifests across all date subdirs
        candidates = []
        for json_path in sorted(contrib_dir.rglob("*.json")):
            st_path = json_path.with_suffix(".safetensors")
            if not st_path.exists():
                continue
            if not is_real_safetensors(st_path):
                continue
            try:
                manifest = json.loads(json_path.read_text())
            except Exception:
                continue
            candidates.append((manifest.get("loss_at_production", 999.0), json_path, st_path, manifest))

        if not candidates:
            print(f"  [discover] {contributor}: no real safetensors found — skipping")
            continue

        candidates.sort(key=lambda x: x[0])  # lowest loss first
        loss, json_path, st_path, manifest = candidates[0]
        print(f"  [discover] {contributor}: best spore ep{manifest.get('epoch_produced','?')} loss={loss:.4f} ({json_path.parent.name})")
        selected.append({
            "contributor": contributor,
            "json_path":   json_path,
            "st_path":     st_path,
            "manifest":    manifest,
        })

    return selected

# ── build albert_serve ─────────────────────────────────────────────────────────

def build_albert_serve() -> bool:
    if SERVE_BIN.exists():
        print(f"  [build] albert_serve already built at {SERVE_BIN}")
        return True
    print("  [build] building albert_serve (this takes ~2 min on first run) ...")
    rc = subprocess.run(
        ["cargo", "build", "--release", "--bin", "albert_serve", "--features", "serve",
         "-p", "moe-llm-core"],
        cwd=HERE,
    ).returncode
    if rc != 0:
        print(f"  [build] ERROR: cargo exited {rc}")
        return False
    print("  [build] albert_serve ready")
    return True

# ── staging ────────────────────────────────────────────────────────────────────

def stage_spore(manifest: dict, st_path: Path, tmpdir: Path):
    """Set up a temp root dir that albert_serve can load."""
    models_dir = tmpdir / "models"
    data_dir   = tmpdir / "data"
    models_dir.mkdir()
    data_dir.mkdir()

    # config.json — derive from spore manifest + known albert v3.0 defaults
    config = {
        "num_layers":  manifest["architecture"]["num_layers"],
        "hidden_size": manifest["architecture"]["hidden_size"],
        "num_experts": manifest["architecture"]["num_experts"],
        "num_heads":   4,     # albert v3.0 constant
        "max_seq_len": 256,   # albert v3.0 constant
        "vocab_size":  manifest["architecture"]["vocab_size"],
    }
    (models_dir / "albert_v3.0.config.json").write_text(json.dumps(config, indent=2))

    # weights — symlink spore safetensors as the expected filename
    (models_dir / "albert_v3.0.safetensors").symlink_to(st_path.resolve())

    # vocab — symlink from main project
    (data_dir / "vocab_v3.json").symlink_to(VOCAB_PATH.resolve())

# ── serve lifecycle ────────────────────────────────────────────────────────────

def start_serve(tmpdir: Path, port: int):
    env = {**os.environ, "PORT": str(port)}
    proc = subprocess.Popen(
        [str(SERVE_BIN), str(tmpdir)],
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    return proc

def wait_ready(port: int, timeout: int = 90) -> bool:
    url = f"http://127.0.0.1:{port}/status"
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            with urllib.request.urlopen(url, timeout=2) as r:
                data = json.loads(r.read())
                if data.get("ready"):
                    return True
        except Exception:
            pass
        time.sleep(1)
    return False

# ── prompt seeding ─────────────────────────────────────────────────────────────

def load_seed_prompts(n: int) -> list[str]:
    """
    Sample n short prompts from the existing chaos corpus for diverse generation.
    Falls back to a hardcoded list if no corpus file is readable.
    """
    candidates = []
    for fname in ("chaos_en.txt",):
        fpath = CHAOS_DIR / fname
        if not fpath.exists():
            continue
        try:
            text = fpath.read_text(errors="ignore")
            # sample 128-char windows that start at a sentence boundary
            for _ in range(n * 10):
                if len(text) < 160:
                    break
                start = random.randint(0, len(text) - 160)
                window = text[start:start + 128].strip()
                if len(window) > 40:
                    candidates.append(window)
            break
        except OSError:
            continue

    if not candidates:
        candidates = [
            "The architecture of the system relies on",
            "In the beginning there was",
            "The function returns a value when",
            "According to the analysis,",
            "She walked into the room and",
            "The experiment showed that",
            "When the process completes,",
            "It is known that",
        ]

    random.shuffle(candidates)
    return candidates[:n]

# ── generation ─────────────────────────────────────────────────────────────────

def generate_samples(port: int, prompts: list[str], max_tokens: int, temperature: float) -> list[str]:
    url     = f"http://127.0.0.1:{port}/generate"
    samples = []
    for i, prompt in enumerate(prompts):
        payload = json.dumps({
            "prompt":      prompt,
            "max_tokens":  max_tokens,
            "temperature": temperature,
        }).encode()
        try:
            req = urllib.request.Request(url, data=payload,
                                         headers={"Content-Type": "application/json"})
            with urllib.request.urlopen(req, timeout=30) as r:
                resp = json.loads(r.read())
                text = resp.get("text", "").strip()
                if text:
                    samples.append(text)
        except Exception as e:
            print(f"    [gen] sample {i+1} failed: {e}")
        if (i + 1) % 50 == 0:
            tok_s = resp.get("tok_s", 0) if "resp" in dir() else 0
            print(f"    [gen] {i+1}/{len(prompts)} samples  tok/s={tok_s:.1f}")
    return samples

# ── chaos corpus write ─────────────────────────────────────────────────────────

def write_chaos_file(contributor: str, samples: list[str]) -> Path:
    CHAOS_DIR.mkdir(parents=True, exist_ok=True)
    out_path = CHAOS_DIR / f"spores_{contributor}.txt"
    with open(out_path, "w") as f:
        for s in samples:
            f.write(s.replace("\n", " ") + "\n")
    size_kb = out_path.stat().st_size // 1024
    print(f"  [chaos] wrote {len(samples)} samples → {out_path.name} ({size_kb} KB)")
    return out_path

# ── 90/10 invariant check ──────────────────────────────────────────────────────

def check_invariant():
    corpus_root = HERE / "data" / "corpus"
    total = 0
    chaos = 0
    for root, _, files in os.walk(corpus_root):
        for fname in files:
            size = (Path(root) / fname).stat().st_size
            total += size
            if "chaos" in root:
                chaos += size
    pct = chaos / total * 100 if total else 0
    print(f"  [invariant] chaos={chaos/1024/1024:.1f}MB  total={total/1024/1024:.1f}MB  "
          f"ratio={pct:.1f}%")
    if pct < 8.0:
        print(f"  [invariant] WARNING: chaos below 8% floor ({pct:.1f}%)")
    elif pct > 14.0:
        print(f"  [invariant] WARNING: chaos above 14% ceiling ({pct:.1f}%) — "
              f"consider trimming spore output or expanding main corpus")
    else:
        print(f"  [invariant] OK")

# ── corpus rebuild ─────────────────────────────────────────────────────────────

def rebuild_cache() -> bool:
    print("  [tokenize] rebuilding corpus_cache.bin ...")
    rc = subprocess.run([sys.executable, str(TOKENIZER)], cwd=HERE).returncode
    if rc != 0:
        print(f"  [tokenize] ERROR: train_tokenizer_v3.py exited {rc}")
        return False
    size_mb = CACHE_BIN.stat().st_size / 1024 / 1024 if CACHE_BIN.exists() else 0
    print(f"  [tokenize] corpus_cache.bin rebuilt ({size_mb:.1f} MB)")
    return True

# ── modal push ─────────────────────────────────────────────────────────────────

def modal_push() -> bool:
    if not shutil.which("modal"):
        print("  [modal] modal CLI not found — skipping push (run: pip install modal)")
        return False
    print("  [modal] pushing corpus_cache.bin to albert-vol ...")
    rc = subprocess.run(
        ["modal", "volume", "put", "--force", MODAL_VOL,
         "data/corpus_cache.bin", "/albert/data/corpus_cache.bin"],
        cwd=HERE,
    ).returncode
    if rc != 0:
        print(f"  [modal] ERROR: modal volume put exited {rc}")
        return False
    print("  [modal] pushed — Modal T4 will pick up new corpus on next training restart")
    return True

# ── main ───────────────────────────────────────────────────────────────────────

def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--spores-dir",    default=str(SPORES_REPO))
    ap.add_argument("--samples",       type=int,   default=300)
    ap.add_argument("--max-tokens",    type=int,   default=128)
    ap.add_argument("--temperature",   type=float, default=0.85)
    ap.add_argument("--contributors",  default=None, help="comma-separated list, default=all")
    ap.add_argument("--dry-run",       action="store_true", help="skip retokenize and Modal push")
    ap.add_argument("--no-modal",      action="store_true", help="skip Modal push only")
    ap.add_argument("--port",          type=int,   default=8765)
    args = ap.parse_args()

    spores_dir   = Path(args.spores_dir)
    contributors = [c.strip() for c in args.contributors.split(",")] if args.contributors else None

    print(f"\n=== spore_inject — {args.samples} samples × {args.max_tokens} tok @ T={args.temperature} ===\n")

    # 1 — discover
    print("[1/6] discovering spores ...")
    spores = discover_spores(spores_dir, contributors)
    if not spores:
        print("No qualifying spores found. Check --spores-dir and that safetensors are pulled (git lfs pull).")
        sys.exit(1)
    print(f"  {len(spores)} contributor(s) queued: {[s['contributor'] for s in spores]}\n")

    # 2 — build
    print("[2/6] checking albert_serve binary ...")
    if not build_albert_serve():
        sys.exit(1)
    print()

    # 3 — per-contributor inference
    print("[3/6] generating text from spore checkpoints ...")
    seed_prompts = load_seed_prompts(args.samples)
    any_written  = False

    for spore in spores:
        contributor = spore["contributor"]
        manifest    = spore["manifest"]
        arch        = manifest["architecture"]
        print(f"\n  --- {contributor}  ep{manifest.get('epoch_produced','?')}  "
              f"loss={manifest.get('loss_at_production','?')}  "
              f"{arch['num_layers']}L·{arch['hidden_size']}H·{arch['num_experts']}E ---")

        with tempfile.TemporaryDirectory(prefix="spore_stage_") as tmpstr:
            tmpdir = Path(tmpstr)
            stage_spore(manifest, spore["st_path"], tmpdir)

            proc = start_serve(tmpdir, args.port)
            try:
                print(f"  [serve] waiting for albert_serve on port {args.port} ...")
                if not wait_ready(args.port, timeout=120):
                    print(f"  [serve] ERROR: albert_serve did not become ready — stdout:")
                    out, _ = proc.communicate(timeout=5)
                    print(out.decode(errors="ignore")[-2000:])
                    continue

                print(f"  [serve] ready — generating {args.samples} samples ...")
                samples = generate_samples(args.port, seed_prompts, args.max_tokens, args.temperature)
                print(f"  [serve] generated {len(samples)}/{args.samples} samples")

            finally:
                proc.terminate()
                try:
                    proc.wait(timeout=5)
                except Exception:
                    proc.kill()

        if samples:
            write_chaos_file(contributor, samples)
            any_written = True

    if not any_written:
        print("\nNo samples written. Exiting without modifying corpus.")
        sys.exit(1)

    # 4 — invariant check
    print("\n[4/6] checking 90/10 invariant ...")
    check_invariant()

    if args.dry_run:
        print("\n[dry-run] skipping retokenize and Modal push.")
        print("Run without --dry-run to commit corpus changes.")
        return

    # 5 — rebuild
    print("\n[5/6] rebuilding corpus_cache.bin ...")
    if not rebuild_cache():
        sys.exit(1)

    # 6 — modal push
    if not args.no_modal:
        print("\n[6/6] pushing to Modal volume ...")
        modal_push()
    else:
        print("\n[6/6] --no-modal set — skipping Modal push")
        print(f"  corpus_cache.bin updated locally at {CACHE_BIN}")
        print(f"  To push manually: modal volume put --force albert-vol data/corpus_cache.bin /albert/data/corpus_cache.bin")

    print("\n=== spore_inject complete ===")
    print("albert. will train on colony text on the next Modal restart.")


if __name__ == "__main__":
    main()
