#!/usr/bin/env python3
"""
train_vertexai.py — submit albert training to Vertex AI Custom Training.
Replaces train_modal.py for GPU runs.

  python train_vertexai.py          # submit job + stream logs
  python train_vertexai.py pull     # download latest checkpoint from GCS
  python train_vertexai.py setup    # upload local checkpoint + corpus to GCS
"""

import json, os, subprocess, sys, time, urllib.request

PROJECT    = "myapplication-71b8874c"
REGION     = "europe-west4"
BUCKET     = "albert-training-rfi"
GCS_PREFIX = "albert"
IMAGE      = f"{REGION}-docker.pkg.dev/{PROJECT}/albert-training/train-bible:latest"
MACHINE    = "n1-standard-4"
GPU_TYPE   = "NVIDIA_TESLA_T4"
GPU_COUNT  = 1
HERE       = os.path.dirname(os.path.abspath(__file__))
GCLOUD     = os.path.expanduser("~/google-cloud-sdk/bin/gcloud")
GSUTIL     = os.path.expanduser("~/google-cloud-sdk/bin/gsutil")
NTFY_TOPIC = "albert-rfi-irfos"

_MODELS = [
    "albert_v3.0.config.json",
    "albert_v3.0.meta",
    "albert_v3.0.safetensors",
    "albert_v3.0.best.safetensors",
    "albert_v3.0.best_loss",
    "albert_v3.0.evolution",
]


def _ntfy(title: str, msg: str, priority: str = "3") -> None:
    try:
        req = urllib.request.Request(
            f"https://ntfy.sh/{NTFY_TOPIC}",
            data=msg.encode(),
            headers={"Title": title, "Priority": priority},
            method="POST",
        )
        urllib.request.urlopen(req, timeout=5)
    except Exception:
        pass


def cmd_setup() -> None:
    """Upload local checkpoint + corpus to GCS."""
    print("[setup] uploading models …")
    for f in _MODELS:
        local = os.path.join(HERE, "models", f)
        if os.path.exists(local):
            subprocess.run([GSUTIL, "cp", local,
                            f"gs://{BUCKET}/{GCS_PREFIX}/models/{f}"], check=True)
    print("[setup] uploading corpus …")
    subprocess.run([GSUTIL, "-m", "cp", "-r",
                    os.path.join(HERE, "data", "vocab_v3.json"),
                    os.path.join(HERE, "data", "corpus"),
                    os.path.join(HERE, "data", "corpus_cache.bin"),
                    f"gs://{BUCKET}/{GCS_PREFIX}/data/"], check=True)
    print("[setup] done")


def cmd_pull() -> None:
    """Download latest checkpoint from GCS to local models/."""
    os.makedirs(os.path.join(HERE, "models"), exist_ok=True)
    for f in _MODELS:
        dest = os.path.join(HERE, "models", f)
        r = subprocess.run(
            [GSUTIL, "cp", f"gs://{BUCKET}/{GCS_PREFIX}/models/{f}", dest],
            capture_output=True,
        )
        if r.returncode == 0:
            print(f"[pull] {f}")
        else:
            print(f"[pull] {f} — not found on GCS (skipped)")
    print("[pull] done — run albert-train pull to also sync evolution state")


def cmd_train() -> None:
    """Submit Vertex AI job and stream logs."""
    # Push current evolution state so fib_index survives restarts
    evo_local = os.path.join(HERE, "models", "albert_v3.0.evolution")
    if os.path.exists(evo_local):
        print("[train] syncing evolution state to GCS …")
        subprocess.run([GSUTIL, "cp", evo_local,
                        f"gs://{BUCKET}/{GCS_PREFIX}/models/albert_v3.0.evolution"],
                       check=True)

    job_name = f"albert-training-{int(time.time())}"
    print(f"[train] submitting job: {job_name}")

    submit = subprocess.run([
        GCLOUD, "ai", "custom-jobs", "create",
        f"--project={PROJECT}",
        f"--region={REGION}",
        f"--display-name={job_name}",
        (f"--worker-pool-spec="
         f"machine-type={MACHINE},"
         f"accelerator-type={GPU_TYPE},"
         f"accelerator-count={GPU_COUNT},"
         f"container-image-uri={IMAGE}"),
        "--format=json",
    ], capture_output=True, text=True)

    if submit.returncode != 0:
        print(f"[train] job submission failed:\n{submit.stderr}")
        _ntfy("albert. VERTEX SUBMIT FAILED", submit.stderr[:300], priority="5")
        sys.exit(1)

    job      = json.loads(submit.stdout)
    job_name = job["name"]           # projects/.../locations/.../customJobs/ID
    job_id   = job_name.split("/")[-1]
    print(f"[train] job ID: {job_id}")
    print(f"[train] streaming logs (may take ~90s for container start) …")

    # Stream logs via gcloud — albert-run's stream() parses this stdout normally
    log_proc = subprocess.Popen([
        GCLOUD, "ai", "custom-jobs", "stream-logs", job_id,
        f"--region={REGION}",
        f"--project={PROJECT}",
    ], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1)

    try:
        for line in log_proc.stdout:
            # Strip the "[worker-pool0-0]: " prefix that Vertex AI prepends
            if "]: " in line:
                line = line[line.index("]: ") + 3:]
            sys.stdout.write(line)
            sys.stdout.flush()
    except KeyboardInterrupt:
        print("\n[train] interrupted — job continues in cloud")
        print(f"[train] to re-attach: gcloud ai custom-jobs stream-logs {job_id} --region={REGION} --project={PROJECT}")
    finally:
        log_proc.terminate()


if __name__ == "__main__":
    sub = sys.argv[1] if len(sys.argv) > 1 else "train"
    {"train": cmd_train, "pull": cmd_pull, "setup": cmd_setup}.get(sub, cmd_train)()
