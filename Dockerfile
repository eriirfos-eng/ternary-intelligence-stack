# ─── Stage 1: build ──────────────────────────────────────────────────────────
FROM rust:1.86-slim AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# 1. Build-only workspace manifest — trimmed to the ternlang crates. The real
#    Cargo.toml also lists albert-moe-13/* (which carry ~859 MB of model weights we
#    can't ship here) + pytern/ternaudit-guard; none are ternlang-api deps. Using the
#    trimmed manifest keeps the real workspace + local albert builds untouched.
COPY ternlang-build.Cargo.toml Cargo.toml
COPY Cargo.lock ./

# 2. Copy the ternlang source (preserving structure)
COPY ternlang-root/      ternlang-root/
COPY ternlang-translator/ ternlang-translator/

# 3. Build the API
RUN cargo build --release -p ternlang-api

# ─── Stage 2: runtime ────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates python3 python3-pip && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /build/target/release/ternlang-api /usr/local/bin/ternlang-api

# Static assets the API serves at runtime (ServeDir/include_str! calls in
# main.rs resolve these as plain relative paths off the process CWD, which is
# "/" by default — they were never copied into this runtime stage at all, so
# every static asset under /studio, /assets, and /translator 404'd in
# production (confirmed via `fly ssh console`: /ternlang-studio didn't exist
# in the running container). Build stage nests everything under
# ternlang-root/ (its build context is the repo root, not ternlang-root/
# itself), so the paths get flattened back to what main.rs actually expects.
COPY --from=builder /build/ternlang-root/ternlang-web       /ternlang-web
COPY --from=builder /build/ternlang-root/ternlang-studio    /ternlang-studio
COPY --from=builder /build/ternlang-root/stdlib             /stdlib
COPY --from=builder /build/ternlang-translator              /ternlang-translator

# Copy the "Spine" generator
COPY scripts/generate_kpi.py /usr/local/bin/generate_kpi.py
COPY scripts/start-production.sh /usr/local/bin/start-production.sh
RUN chmod +x /usr/local/bin/start-production.sh

# Fly.io configuration
ENV PORT=8080
# Not set anywhere in this file's fly.toml either — main.rs's stdlib_read/
# stdlib_list handlers need this to find the copied /stdlib dir above.
ENV STDLIB_PATH=/stdlib
EXPOSE 8080

CMD ["/usr/local/bin/start-production.sh"]
