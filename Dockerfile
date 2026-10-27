# ============================================================
# TechScript Multi-Stage Production Dockerfile
# ============================================================

# Stage 1: Builder
FROM rust:slim-bookworm AS builder

WORKDIR /build

# Install system compilation dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY compiler/ ./compiler/
COPY runtime/ ./runtime/
COPY stdlib/ ./stdlib/
COPY cli/ ./cli/
COPY tools/ ./tools/
COPY assets/ ./assets/

# Compile optimized release binary
RUN cargo build --release -p techscript_cli --bin tsc

# Stage 2: Final Minimal Runtime
FROM debian:bookworm-slim

# Metadata
LABEL org.opencontainers.image.title="TechScript" \
      org.opencontainers.image.description="Unified General-Purpose Programming Language & WebForge Toolchain" \
      org.opencontainers.image.url="https://techscript.is-a.dev/" \
      org.opencontainers.image.source="https://github.com/Tcode-Motion/techscript" \
      org.opencontainers.image.licenses="Apache-2.0"

# Install SSL certificates for HTTPS networking
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create techscript user
RUN useradd -m -s /bin/bash techscript

# Copy binaries from builder
COPY --from=builder /build/target/release/tsc /usr/local/bin/tsc

# Create standard aliases
RUN ln -s /usr/local/bin/tsc /usr/local/bin/techscript && \
    ln -s /usr/local/bin/tsc /usr/local/bin/webforge && \
    ln -s /usr/local/bin/tsc /usr/local/bin/tech

# Set workspace
WORKDIR /workspace
RUN chown -R techscript:techscript /workspace

USER techscript

ENTRYPOINT ["/usr/local/bin/tsc"]
CMD ["--help"]
