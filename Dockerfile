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

# --- Rust Application Build ---
# Add the MUSL target and build the application statically
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=clang
ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 4: Runtime - Create the final minimal image
FROM debian:bookworm-slim AS runtime
WORKDIR /usr/src

# Install wget for health checks
RUN apt-get update && apt-get install -y --no-install-recommends wget && rm -rf /var/lib/apt/lists/*

# Copy required assets and configuration from the builder stage
COPY --from=builder /usr/src/assets assets
COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/pictora-cli /usr/src/pictora

# Expose the Loco app port
EXPOSE 3000

# Set the entrypoint for the container
ENTRYPOINT ["/usr/src/pictora"]
CMD ["start", "-e", "production"]

