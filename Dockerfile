FROM lukemathwalker/cargo-chef:latest-rust-1.83.0 AS chef
WORKDIR /usr/src/
RUN apt update && apt install lld clang -y
FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json
FROM chef AS builder
COPY --from=planner /usr/src//recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .

# Build our project
RUN cargo build --release --bin pictora-cli
FROM debian:bullseye-slim AS runtime
WORKDIR /usr/app
RUN apt-get update -y \
&& apt-get install -y --no-install-recommends openssl ca-certificates \
# Clean up
&& apt-get autoremove -y \
&& apt-get clean -y \
&& rm -rf /var/lib/apt/lists/*
WORKDIR /usr/app
COPY --from=builder /usr/src/assets assets
COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/pictora-cli /usr/app/pictora
ENTRYPOINT ["/usr/app/pictora"]