"""
serve_modal.py — Deploy albert. CPU inference as a Modal web endpoint.

Deploy:
    modal deploy serve_modal.py

The deployed URL looks like:
    https://rfi-irfos--albert-serve-serve.modal.run

Set ALBERT_SERVE_URL in ternlang-api Fly.io secrets to that URL:
    fly secrets set ALBERT_SERVE_URL=https://rfi-irfos--albert-serve-serve.modal.run \\
        --app ternlang-api
"""

import os
import subprocess
import sys

import modal

_VOL  = "albert-vol"
_HERE = os.path.dirname(os.path.abspath(__file__))

# CPU-only image — no CUDA needed for inference.
# Shares cargo-home cache with the training volume to avoid re-downloading crates.
image = (
    modal.Image.from_registry("ubuntu:22.04", add_python="3.11")
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
        ignore=["target/", ".git/", "__pycache__", "data/", "models/",
                "benchmarks/", "dashboard/", "token-probes/"],
    )
)

vol = modal.Volume.from_name(_VOL, create_if_missing=False)
app = modal.App("albert-serve")


@app.function(
    image=image,
    cpu=4.0,
    memory=3072,          # checkpoint ~537MB + working memory + inference buffers
    volumes={"/vol": vol},
    min_containers=1,     # one always-on container — model stays loaded
    timeout=3600,
)
@modal.web_server(8000, startup_timeout=180)
def serve():
    env = {
        **os.environ,
        "PATH": f"/root/.cargo/bin:{os.environ.get('PATH', '')}",
        "CARGO_HOME":       "/vol/cargo-home",   # shared with training
        "CARGO_TARGET_DIR": "/tmp/cargo-target",
        "CARGO_TERM_COLOR": "never",
    }

    # Minimal workspace Cargo.toml — only moe-llm-core needed
    with open("/src/Cargo.toml", "w") as f:
        f.write(
            '[workspace]\n'
            'members = ["moe-llm-core"]\n'
            'resolver = "2"\n'
            '\n'
            '[workspace.package]\n'
            'version    = "1.3.7"\n'
            'edition    = "2024"\n'
            'license    = "LGPL-3.0-or-later"\n'
            'repository = "https://github.com/eriirfos-eng/ternary-intelligence-stack"\n'
            'homepage   = "https://ternlang.com"\n'
        )

    cached_bin = "/vol/albert/bin/albert_serve_v3"  # v3: dual-stream (cord) — reads num_streams/fusion_layers, serves latest ckpt
    os.makedirs("/vol/albert/bin", exist_ok=True)

    if os.path.exists(cached_bin):
        print("[serve] using cached binary from volume")
        import shutil
        shutil.copy2(cached_bin, "/tmp/albert_serve")
        os.chmod("/tmp/albert_serve", 0o755)
        bin_path = "/tmp/albert_serve"
    else:
        print("[serve] cargo build --release --features serve ...")
        result = subprocess.run(
            ["/root/.cargo/bin/cargo", "build", "--release",
             "--bin", "albert_serve", "--features", "serve",
             "-p", "moe-llm-core"],
            cwd="/src",
            env=env,
        )
        if result.returncode != 0:
            print("[serve] build failed — exiting", file=sys.stderr)
            sys.exit(1)
        # cache binary to volume for future cold starts
        import shutil
        shutil.copy2("/tmp/cargo-target/release/albert_serve", cached_bin)
        os.chmod(cached_bin, 0o755)
        print(f"[serve] cached binary to {cached_bin}")
        bin_path = "/tmp/cargo-target/release/albert_serve"

    print("[serve] starting albert_serve on :8000 with root=/vol/albert")
    subprocess.Popen(
        [bin_path, "/vol/albert"],
        env={**env, "PORT": "8000"},
    )
