#!/usr/bin/env python3
"""
train_modal.py — albert. GPU training on Modal

  python train_modal.py setup          # one-time: create volume, upload data + checkpoint
  modal run train_modal.py             # launch GPU training (streams logs live)
  modal run --detach train_modal.py    # same but survives terminal close
  python train_modal.py pull           # pull latest checkpoint back to local models/

Run all commands from the albert-moe-13/ directory.
"""

import os
import subprocess
import sys

# ---------------------------------------------------------------------------
# CLI commands (setup / pull) — handled before Modal imports so they work
# even when modal isn't installed yet in an edge case.
# ---------------------------------------------------------------------------

_HERE = os.path.dirname(os.path.abspath(__file__))
_VOL  = "albert-vol"

_UPLOADS = [
    # (local path relative to albert-moe-13/, remote path on volume)
    ("data/vocab_v3.json",               "/albert/data/vocab_v3.json"),
    ("data/corpus",                      "/albert/data/corpus"),
    ("data/corpus_cache.bin",            "/albert/data/corpus_cache.bin"),
    ("models/albert_v3.0.config.json",   "/albert/models/albert_v3.0.config.json"),
    ("models/albert_v3.0.meta",          "/albert/models/albert_v3.0.meta"),
    ("models/albert_v3.0.safetensors",   "/albert/models/albert_v3.0.safetensors"),
    ("models/albert_v3.0.best.safetensors", "/albert/models/albert_v3.0.best.safetensors"),
    ("models/albert_v3.0.best_loss",     "/albert/models/albert_v3.0.best_loss"),
]

_DOWNLOADS = [
    # (remote path on volume, local path)
    ("/albert/models/albert_v3.0.config.json",      "models/albert_v3.0.config.json"),
    ("/albert/models/albert_v3.0.safetensors",      "models/albert_v3.0.safetensors"),
    ("/albert/models/albert_v3.0.best.safetensors", "models/albert_v3.0.best.safetensors"),
    ("/albert/models/albert_v3.0.best_loss",        "models/albert_v3.0.best_loss"),
    ("/albert/models/albert_v3.0.meta",             "models/albert_v3.0.meta"),
    ("/albert/models/albert_v3.0.evolution",        "models/albert_v3.0.evolution"),
    ("/albert/dashboard/training.log",              "dashboard/training.log"),
    ("/albert/dashboard/epoch_history.log",         "dashboard/epoch_history.log"),
]


def _run(cmd: list[str]) -> int:
    print(f"  $ {' '.join(cmd)}")
    return subprocess.run(cmd, cwd=_HERE).returncode


def cmd_setup():
    print(f"[setup] creating volume {_VOL} (ok if already exists)")
    result = subprocess.run(["modal", "volume", "create", _VOL], cwd=_HERE)
    if result.returncode not in (0, 1):  # 1 = volume already exists (modal exit code)
        print(f"  WARNING: modal volume create exited {result.returncode}")

    total = len(_UPLOADS)
    for i, (local, remote) in enumerate(_UPLOADS, 1):
        local_abs = os.path.join(_HERE, local)
        if not os.path.exists(local_abs):
            print(f"  [{i}/{total}] SKIP {local} (not found)")
            continue
        size = _du(local_abs)
        print(f"\n[{i}/{total}] uploading {local}  ({size})")
        rc = _run(["modal", "volume", "put", _VOL, local, remote])
        if rc != 0:
            print(f"  ERROR: modal volume put exited {rc}")
            sys.exit(rc)

    print("\n[setup] done — run:  modal run train_modal.py")


def cmd_pull():
    print("[pull] downloading latest checkpoint from volume ...")
    for remote, local in _DOWNLOADS:
        local_abs = os.path.join(_HERE, local)
        print(f"\n  {remote}  ->  {local}")
        rc = _run(["modal", "volume", "get", "--force", _VOL, remote, local_abs])
        if rc != 0:
            print(f"  WARNING: could not pull {remote} (exit {rc})")
    print("\n[pull] done")


def _du(path: str) -> str:
    try:
        out = subprocess.check_output(["du", "-sh", path], stderr=subprocess.DEVNULL)
        return out.decode().split()[0]
    except Exception:
        return "?"


if __name__ == "__main__":
    if len(sys.argv) < 2 or sys.argv[1] not in ("setup", "pull"):
        print(__doc__)
        sys.exit(0)
    {"setup": cmd_setup, "pull": cmd_pull}[sys.argv[1]]()
    sys.exit(0)


# ---------------------------------------------------------------------------
# Modal app definition — only reached when invoked via `modal run`
# ---------------------------------------------------------------------------

import modal  # noqa: E402

# CUDA 12.1 + cuDNN 8 devel image with Rust pre-installed.
# add_local_dir bakes moe-llm-core/ into the image; Modal detects changes
# and rebuilds only that layer — fast since it's just file copying.
image = (
    modal.Image.from_registry(
        "nvidia/cuda:12.1.0-cudnn8-devel-ubuntu22.04",
        add_python="3.11",
    )
    .apt_install(
        "curl", "build-essential", "pkg-config",
        "libssl-dev", "libopenblas-dev", "ca-certificates",
    )
    .run_commands(
        "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs "
        "| sh -s -- -y --default-toolchain stable --profile minimal",
        "/root/.cargo/bin/rustc --version",
    )
    .add_local_dir(
        _HERE,
        remote_path="/src",
        ignore=["target/", ".git/", "__pycache__/", "data/", "models/", "benchmarks/", "dashboard/"],
    )
)

# Persistent volume — corpus, checkpoints, cargo build cache, logs.
vol = modal.Volume.from_name(_VOL, create_if_missing=True)

app = modal.App("albert-training")


@app.function(
    image=image,
    gpu="T4",            # swap to "A10G" for ~4x throughput, ~2x cost
    timeout=23 * 3600,   # 23-hour cap
    volumes={"/vol": vol},
    memory=16384,
    cpu=4.0,
)
def train(gate_diversity: float = 0.5, lb_weight: float = 0.03):
    rust_bin = "/root/.cargo/bin"
    cuda_bin = "/usr/local/cuda/bin"
    env = {
        **os.environ,
        "PATH": f"{rust_bin}:{cuda_bin}:{os.environ.get('PATH', '')}",
        # Cache downloaded crates on volume (skip re-downloads).
        # Build artifacts go to /tmp — always fresh so source changes are never
        # silently skipped due to stale timestamps on the persistent volume.
        "CARGO_HOME":       "/vol/cargo-home",
        "CARGO_TARGET_DIR": "/tmp/cargo-target",
        "CARGO_TERM_COLOR": "never",
    }

    # The real workspace Cargo.toml references /ternlang-root which doesn't
    # exist on Modal. Replace it with a minimal workspace containing only
    # moe-llm-core — all that's needed to build train_bible.
    with open("/src/Cargo.toml", "w") as f:
        f.write(
            '[workspace]\n'
            'members = ["moe-llm-core"]\n'
            'resolver = "2"\n'
            '\n'
            '[workspace.package]\n'
            'version    = "1.3.6"\n'
            'edition    = "2024"\n'
            'license    = "LGPL-3.0-or-later"\n'
            'repository = "https://github.com/eriirfos-eng/ternary-intelligence-stack"\n'
            'homepage   = "https://ternlang.com"\n'
        )

    print("[modal] cargo build --release --features cuda ...")
    sys.stdout.flush()

    result = subprocess.run(
        [f"{rust_bin}/cargo", "build", "--release",
         "--features", "cuda", "--bin", "train_bible"],
        cwd="/src",
        env=env,
        capture_output=True,
        text=True,
    )
    # Always print output so errors are visible in Modal logs
    if result.stdout:
        print(result.stdout[-4000:])
    if result.stderr:
        print(result.stderr[-4000:])
    sys.stdout.flush()

    if result.returncode != 0:
        raise RuntimeError("cargo build failed")

    binary = "/tmp/cargo-target/release/train_bible"
    print(f"[modal] build OK — {binary}")

    os.makedirs("/vol/albert/models",     exist_ok=True)
    os.makedirs("/vol/albert/data",       exist_ok=True)
    os.makedirs("/vol/albert/logs",       exist_ok=True)
    os.makedirs("/vol/albert/dashboard",  exist_ok=True)
    vol.commit()

    cmd = [
        binary,
        "--root=/vol/albert",
        f"--gate-diversity={gate_diversity}",
        f"--lb-weight={lb_weight}",
        "--div-weight=0.001",
    ]
    print(f"[modal] {' '.join(cmd)}")
    sys.stdout.flush()

    import threading, time

    log_path = "/vol/albert/dashboard/training.log"

    # Clear the volume log and write a RUN_START sentinel.
    # albert-train's stream loop detects this sentinel and truncates the local
    # dashboard log, keeping the frontend in sync across auto-restarts/preemptions.
    import time as _time
    with open(log_path, 'w') as _f:
        _f.write(f"RUN_START ts={int(_time.time())}\n")
    start_pos = 0

    def tail_log(proc):
        while not os.path.exists(log_path):
            if proc.poll() is not None:
                return
            time.sleep(0.2)
        with open(log_path) as f:
            f.seek(start_pos)
            while True:
                line = f.readline()
                if line:
                    sys.stdout.write(line)
                    sys.stdout.flush()
                else:
                    if proc.poll() is not None:
                        break
                    time.sleep(0.05)

    proc = subprocess.Popen(cmd, env=env)
    tailer = threading.Thread(target=tail_log, args=(proc,), daemon=True)
    tailer.start()
    proc.wait()
    tailer.join(timeout=3)

    vol.commit()

    if proc.returncode not in (0, 1):
        raise RuntimeError(f"train_bible crashed (exit {proc.returncode})")

    print("[modal] run complete — pull checkpoint:  python train_modal.py pull")


@app.local_entrypoint()
def main():
    # Always push the local config to the volume before launching — prevents
    # silent CTX/arch drift when config.json changes after initial setup.
    print("[main] syncing config.json to volume ...")
    rc = subprocess.run(
        ["modal", "volume", "put", "--force", _VOL,
         "models/albert_v3.0.config.json",
         "/albert/models/albert_v3.0.config.json"],
        cwd=_HERE,
    ).returncode
    if rc != 0:
        raise SystemExit(f"[main] config sync failed (exit {rc}) — aborting launch")

    # Push evolution state so fib_index survives restarts.
    # Non-fatal: file absent on first-ever setup; train_bible falls back to calibrate().
    evo_local = os.path.join(_HERE, "models/albert_v3.0.evolution")
    if os.path.exists(evo_local):
        print("[main] syncing evolution state to volume ...")
        evo_rc = subprocess.run(
            ["modal", "volume", "put", "--force", _VOL,
             "models/albert_v3.0.evolution",
             "/albert/models/albert_v3.0.evolution"],
            cwd=_HERE,
        ).returncode
        if evo_rc != 0:
            print(f"[main] evolution sync failed (exit {evo_rc}) — train_bible will recalibrate from scratch")
    train.remote(gate_diversity=0.3, lb_weight=0.0)
