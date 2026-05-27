### Stage 1 — compile train_bible with CUDA
FROM nvidia/cuda:12.1.0-devel-ubuntu22.04 AS builder

RUN apt-get update -qq && apt-get install -y -qq \
    curl build-essential pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:/usr/local/cuda/bin:${PATH}"
ENV CUDA_HOME=/usr/local/cuda
ENV CARGO_TARGET_DIR=/tmp/cargo-target
# T4 compute capability — lets candle-kernels skip nvidia-smi on CPU build machines
ENV CUDA_COMPUTE_CAP=75

WORKDIR /src
COPY moe-llm-core/ ./moe-llm-core/
COPY Cargo.lock ./
# Minimal workspace — same trick as train_modal.py
RUN printf '[workspace]\nmembers = ["moe-llm-core"]\nresolver = "2"\n\n[workspace.package]\nversion = "1.3.6"\nedition = "2024"\nlicense = "LGPL-3.0-or-later"\nrepository = "https://github.com/eriirfos-eng/ternary-intelligence-stack"\nhomepage = "https://ternlang.com"\n' > Cargo.toml

RUN cargo build --release --features cuda --bin train_bible


### Stage 2 — lean runtime image
FROM nvidia/cuda:12.1.0-cudnn8-runtime-ubuntu22.04

RUN apt-get update -qq && apt-get install -y -qq python3 python3-pip \
    && pip3 install --no-cache-dir google-cloud-storage==2.16.0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /tmp/cargo-target/release/train_bible /app/train_bible
COPY runner.py /app/runner.py

RUN chmod +x /app/train_bible
WORKDIR /app
ENTRYPOINT ["python3", "/app/runner.py"]
