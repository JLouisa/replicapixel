# Stage 1: Chef Base - Installs dependencies needed for building
FROM lukemathwalker/cargo-chef:latest-rust-1.83.0@sha256:4d3858029892a324ce199006b7272cff35ab35ebd4ac7cc78575976976708b64 AS chef

WORKDIR /usr/src/

# Install linker, C compiler, MUSL tools, prerequisites for Bun, and Bun itself
RUN apt-get update && apt-get install -y --no-install-recommends \
    lld clang musl-tools ca-certificates \
    curl unzip xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Bun
RUN curl -fsSL https://bun.sh/install | bash

# If not, the path might be /home/<user>/.bun/bin
ENV PATH="/root/.bun/bin:${PATH}"

# Stage 2: Planner - Create dependency recipe
FROM chef AS planner
WORKDIR /usr/src/

# Copy root Cargo files
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/Cargo.toml

# Compute a lock-like file for our project dependencies
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Builder - Build dependencies and the application
FROM chef AS builder
WORKDIR /usr/src/

# Copy dependency recipe
COPY --from=planner /usr/src/recipe.json recipe.json
# Build project dependencies based on the recipe
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the full project source AFTER dependencies are built
COPY . .

WORKDIR /usr/src/frontend
# RUN bun install --production
RUN bun install
# Build assets using the script from package.json
RUN bun run build
# Change back to the root WORKDIR for Rust build if necessary
WORKDIR /usr/src/

# Add the MUSL target and build the application statically
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=clang
ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl --locked

# Stage 4: Runtime - Create the final minimal image
FROM debian:bookworm-slim@sha256:b1211f6d19afd012477bd34fdcabb6b663d680e0f4b0537da6e6b0fd057a3ec3 AS runtime
WORKDIR /usr/src

# Install only what's needed for runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN adduser --disabled-password --gecos "" appuser

# Copy required assets and configuration
COPY --from=builder /usr/src/assets ./assets
COPY --from=builder /usr/src/config ./config
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/replicapixel-cli /usr/src/replicapixel

# Set ownership for all copied files and executable permission for the binary
RUN chown -R appuser:appuser /usr/src/assets /usr/src/config /usr/src/replicapixel \
    && chmod +x /usr/src/replicapixel

# Expose the Loco app port
EXPOSE 3000

# Switch to non-root user
USER appuser


# Set the entrypoint for the container
ENTRYPOINT ["/usr/src/replicapixel"]
CMD ["start", "--server-and-worker", "-e", "production"]
