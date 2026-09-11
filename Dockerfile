# Build application
FROM --platform=linux/amd64 ubuntu:22.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        curl \
        build-essential \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --profile minimal

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /build

# Copy TUI application
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release


# Runtime(creating Vivado environment)
FROM --platform=linux/amd64 ubuntu:22.04

WORKDIR /workspace

COPY --from=builder /build/target/release/VEM /usr/local/bin/VEM

# Run TUI on startup
ENTRYPOINT ["/usr/local/bin/VEM"]
