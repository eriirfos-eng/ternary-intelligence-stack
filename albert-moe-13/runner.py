#!/usr/bin/env python3
"""
runner.py — runs inside Vertex AI container.
Downloads checkpoint + corpus from GCS, trains, syncs back on each epoch.
Equivalent to the train() function in train_modal.py.
"""

import os, sys, subprocess, threading, time, urllib.request
from google.cloud import storage as gcs

BUCKET      = os.environ.get("ALBERT_BUCKET", "albert-training-rfi")
GCS_PREFIX  = "albert"
LOCAL_ROOT  = "/albert"
NTFY_TOPIC  = "albert-rfi-irfos"
FIB_TARGETS = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987]

# Training args — mirror train_modal.py defaults
GATE_DIVERSITY = float(os.environ.get("GATE_DIVERSITY", "0.3"))
LB_WEIGHT      = float(os.environ.get("LB_WEIGHT",      "0.0"))
DIV_WEIGHT     = float(os.environ.get("DIV_WEIGHT",     "0.001"))
BATCH_SIZE     = int(os.environ.get("BATCH_SIZE",       "1"))


def ntfy(title: str, msg: str, priority: str = "3") -> None:
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


def gcs_download() -> None:
    client = gcs.Client()
    bucket = client.bucket(BUCKET)

    for prefix, local_dir in [
        (f"{GCS_PREFIX}/models/", f"{LOCAL_ROOT}/models"),
        (f"{GCS_PREFIX}/data/",   f"{LOCAL_ROOT}/data"),
    ]:
        os.makedirs(local_dir, exist_ok=True)
        for blob in bucket.list_blobs(prefix=prefix):
            rel  = blob.name[len(prefix):]
            if not rel:
                continue
            dest = os.path.join(local_dir, rel)
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            print(f"[gcs] ↓ {blob.name}", flush=True)
            blob.download_to_filename(dest)


def gcs_upload_models(upload_best: bool = False) -> None:
    client = gcs.Client()
    bucket = client.bucket(BUCKET)
    models_dir = f"{LOCAL_ROOT}/models"
    for fname in os.listdir(models_dir):
        if fname.endswith(".best.safetensors") and not upload_best:
            continue
        local = os.path.join(models_dir, fname)
        blob  = bucket.blob(f"{GCS_PREFIX}/models/{fname}")
        print(f"[gcs] ↑ {fname}", flush=True)
        blob.upload_from_filename(local)


def read_evo_status() -> str:
    evo = f"{LOCAL_ROOT}/models/albert_v3.0.evolution"
    try:
        lines   = open(evo).readlines()
        fib_idx = int(lines[0].strip())
        window  = FIB_TARGETS[fib_idx] if fib_idx < len(FIB_TARGETS) else "?"
        return f"fib_index={fib_idx} window={window}"
    except Exception as e:
        return f"evo_error={e}"


def main() -> None:
    os.makedirs(f"{LOCAL_ROOT}/models",    exist_ok=True)
    os.makedirs(f"{LOCAL_ROOT}/data",      exist_ok=True)
    os.makedirs(f"{LOCAL_ROOT}/dashboard", exist_ok=True)

    print("[runner] downloading checkpoint + corpus from GCS …", flush=True)
    gcs_download()
    print("[runner] download complete", flush=True)

    cmd = [
        "/app/train_bible",
        f"--root={LOCAL_ROOT}",
        f"--gate-diversity={GATE_DIVERSITY}",
        f"--lb-weight={LB_WEIGHT}",
        f"--div-weight={DIV_WEIGHT}",
        f"--batch-size={BATCH_SIZE}",
    ]

    evo_status = read_evo_status()
    ts = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    ntfy("albert. TRAINING STARTED",
         f"Vertex AI container running — {ts}\n{evo_status}\ncmd: {' '.join(cmd[-3:])}",
         priority="3")
    print(f"[runner] {' '.join(cmd)}", flush=True)

    log_path = f"{LOCAL_ROOT}/dashboard/training.log"

    proc   = subprocess.Popen(cmd, env={**os.environ, "HOME": "/tmp"})
    _lock  = threading.Lock()

    def tail() -> None:
        while not os.path.exists(log_path):
            if proc.poll() is not None:
                return
            time.sleep(0.2)
        with open(log_path) as f:
            while True:
                line = f.readline()
                if line:
                    sys.stdout.write(line)
                    sys.stdout.flush()
                    if line.startswith("EPOCH_SUMMARY"):
                        # parse since_best — upload best only when new ATL
                        upload_best = "since_best=0 " in line or "since_best=0\n" in line
                        threading.Thread(
                            target=_safe_upload,
                            args=(upload_best,),
                            daemon=True,
                        ).start()
                else:
                    if proc.poll() is not None:
                        break
                    time.sleep(0.05)

    def _safe_upload(upload_best: bool) -> None:
        with _lock:
            try:
                gcs_upload_models(upload_best=upload_best)
            except Exception as e:
                print(f"[gcs] upload error: {e}", flush=True)

    tailer = threading.Thread(target=tail, daemon=True)
    tailer.start()
    proc.wait()
    tailer.join(timeout=10)

    print("[runner] training ended — final GCS sync …", flush=True)
    try:
        gcs_upload_models(upload_best=True)
    except Exception as e:
        print(f"[gcs] final upload error: {e}", flush=True)

    print("[runner] done", flush=True)


if __name__ == "__main__":
    main()
