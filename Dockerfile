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

# Copy TUI application and build binary
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

# Runtime(creating Vivado environment)
FROM --platform=linux/amd64 ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TERM=xterm-256color

RUN apt-get update && \
	apt-get install -y --no-install-recommends \
        python3-pip python3-dev build-essential git gcc-multilib g++ \
        ocl-icd-opencl-dev libjpeg62-dev libc6-dev-i386 graphviz make \
        unzip libtinfo5 xvfb libncursesw5 locales libswt-gtk-4-jni \
		ncurses-base ncurses-term \
	&& rm -rf /var/lib/apt/lists/*

# Install vivado
ARG VIVADO_INSTALLER_PATH=./vivado-installer
COPY ${VIVADO_INSTALLER_PATH} /tmp/vivado-installer
RUN cd /tmp/vivado-installer && \
    ./xsetup --agree 3rdPartyEULA,XilinxEULA --batch Install --config /tmp/vivado-installer/install_config.txt && \
    rm -rf /tmp/vivado-installer

WORKDIR /workspace

# Moves tui from builder into minimal container
COPY --from=builder /build/target/release/VEM /usr/local/bin/VEM

# Run TUI on startup
CMD ["/usr/local/bin/VEM"]
