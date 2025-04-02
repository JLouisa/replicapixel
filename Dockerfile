# Stage 1: Chef Base - Installs dependencies needed for building
FROM lukemathwalker/cargo-chef:latest-rust-1.83.0 AS chef
WORKDIR /usr/src/
# Install linker, C compiler, and MUSL tools for static linking
RUN apt-get update && apt-get install -y --no-install-recommends lld clang musl-tools && rm -rf /var/lib/apt/lists/*

# Stage 2: Planner - Create dependency recipe
FROM chef AS planner
WORKDIR /usr/src/
COPY . .
# Compute a lock-like file for our project dependencies
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder - Build dependencies and the application
FROM chef AS builder
WORKDIR /usr/src/

# Copy dependency recipe
COPY --from=planner /usr/src/recipe.json recipe.json
# Build project dependencies based on the recipe
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the application source code AFTER dependencies are built
COPY . . 

# --- Frontend Build ---
# Install Node.js (needed for some Bun operations or as fallback)
RUN curl -fsSL https://deb.nodesource.com/setup_current.x | bash - && \
    apt-get update && apt-get install -y --no-install-recommends nodejs && \
    rm -rf /var/lib/apt/lists/*

# Install Bun and build the frontend
RUN curl -fsSL https://bun.sh/install | bash && \
    export BUN_INSTALL="/root/.bun" && \
    export PATH="$BUN_INSTALL/bin:$PATH" && \
    cd frontend && bun install && bun run build

# --- Rust Application Build ---
# Source code is already copied.
# Add the MUSL target and build the application statically
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=clang
ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 4: Runtime - Create the final minimal image
FROM debian:bookworm-slim AS runtime
WORKDIR /usr/app

# Copy required assets and configuration from the builder stage
# These paths depend on the `COPY . .` putting them in /usr/src/ in the builder
COPY --from=builder /usr/src/assets assets
COPY --from=builder /usr/src/config config
# Make sure the frontend build output (e.g., static files) is part of 'assets'
# or copy it separately if needed. For Loco, assets usually contain static files.

# Copy the statically linked application binary from the builder stage
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/pictora-cli /usr/app/pictora

# Set the entrypoint for the container
ENTRYPOINT ["/usr/app/pictora"]
CMD ["start", "-e", "production"]
