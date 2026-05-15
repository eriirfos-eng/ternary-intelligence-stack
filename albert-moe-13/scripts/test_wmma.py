#!/usr/bin/env python3
"""
test_wmma.py — compile wmma_test.cu on the Modal T4 container and report result.

Usage:
    modal run scripts/test_wmma.py

Success output: "WMMA OK — fragment instantiation succeeded"
Failure output: full nvcc stderr so we can read the exact error
"""

import os
import subprocess
import sys

_HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

import modal  # noqa: E402

image = (
    modal.Image.from_registry(
        "nvidia/cuda:12.1.0-cudnn8-devel-ubuntu22.04",
        add_python="3.11",
    )
    .apt_install("build-essential")
    .add_local_file(
        os.path.join(_HERE, "moe-llm-core/cuda/wmma_test.cu"),
        remote_path="/wmma_test.cu",
    )
)

app = modal.App("wmma-compile-test")


@app.function(image=image, gpu="T4", timeout=120)
def test_wmma_compile():
    import subprocess, sys

    nvcc = "/usr/local/cuda/bin/nvcc"
    cmd = [nvcc, "--gpu-architecture", "sm_75", "-c", "/wmma_test.cu", "-o", "/dev/null"]

    result = subprocess.run(cmd, capture_output=True, text=True)

    if result.returncode == 0:
        print("WMMA OK -- fragment instantiation succeeded")
        if result.stderr.strip():
            print("nvcc stderr (warnings only):")
            print(result.stderr)
    else:
        print("WMMA FAIL -- nvcc returned", result.returncode)
        print("--- stdout ---")
        print(result.stdout)
        print("--- stderr ---")
        print(result.stderr)
        sys.exit(1)


@app.local_entrypoint()
def main():
    test_wmma_compile.remote()
