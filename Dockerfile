# -------- STAGE 1: Builder --------
FROM lukemathwalker/cargo-chef:latest-rust-1.83.0 AS builder

WORKDIR /usr/src

# Install linker, C compiler, and MUSL tools for static linking
RUN apt-get update && apt-get install -y --no-install-recommends \
    lld clang musl-tools ca-certificates curl unzip xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

# Cache build dependencies
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source and build
COPY . .
WORKDIR /usr/src/frontend
RUN bun install && bun run build
WORKDIR /usr/src
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl --locked

# Download static ffmpeg binary only
WORKDIR /tmp
RUN curl -L https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz \
  | tar -xJf - --wildcards --strip-components=1 '*/ffmpeg'

# Set permissions manually (if needed at build time):
RUN chmod +x /usr/src/target/x86_64-unknown-linux-musl/release/replicapixel-cli /tmp/ffmpeg

# -------- STAGE 2: Runtime --------
FROM debian:bookworm-slimdebian@sha256:6ac2c08566499cc2415926653cf2ed7c3aedac445675a013cc09469c9e118fdd AS runtime

# Set working directory
WORKDIR /usr/src

# Install only what's needed for runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    wget curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN adduser --disabled-password --gecos "" appuser

# 🔧 Create storage directory and give ownership to `appuser`
RUN mkdir -p /usr/src/storage && chown -R appuser:appuser /usr/src/storage

# Copy runtime files from builder
COPY --from=builder /usr/src/assets ./assets
COPY --from=builder /usr/src/config ./config
COPY --from=builder /tmp/ffmpeg /usr/local/bin/ffmpeg
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/replicapixel-cli /replicapixel

# Expose the Loco app port
EXPOSE 3000

# 🔐 Switch to non-root user (after storage is owned)
USER appuser

# Entrypoint
ENTRYPOINT ["/replicapixel"]
CMD ["start", "--server-and-worker", "-e", "production"]
