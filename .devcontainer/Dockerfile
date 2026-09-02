# SigmaOS Developer Container - Inspired by Linux (Debian, Arch, Alpine) & BSD Systems Environments
FROM mcr.microsoft.com/devcontainers/cpp:1-ubuntu-22.04

LABEL maintainer="SigmaOS Sovereign Engineering Team"
LABEL description="Sovereign bare-metal OS development environment with multi-language toolchains (Rust, Nim, Zig, C/C++) and QEMU hardware emulation."

ENV DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:/usr/local/zig:/usr/local/nim/bin:${PATH}

# Install core build utilities, LLVM/Clang suite, QEMU system emulators, and Python testing framework
RUN apt-get update && apt-get -y install --no-install-recommends \
        build-essential \
        crossbuild-essential-arm64 \
        clang \
        lld \
        llvm \
        gdb \
        clang-format \
        cmake \
        ninja-build \
        git \
        curl \
        wget \
        ca-certificates \
        qemu-system-x86 \
        qemu-system-arm \
        qemu-utils \
        iproute2 \
        net-tools \
        python3 \
        python3-pip \
        python3-pytest \
        xorriso \
        mtools \
        syslinux \
        isolinux \
        pkg-config \
        libssl-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain (Debian/Arch/Alpine parity)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile default \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME \
    && rustup component add clippy rustfmt

# Install Zig toolchain
RUN ZIG_VERSION="0.11.0" \
    && wget -q https://ziglang.org/download/${ZIG_VERSION}/zig-linux-x86_64-${ZIG_VERSION}.tar.xz \
    && tar -xf zig-linux-x86_64-${ZIG_VERSION}.tar.xz -C /usr/local \
    && mv /usr/local/zig-linux-x86_64-${ZIG_VERSION} /usr/local/zig \
    && rm zig-linux-x86_64-${ZIG_VERSION}.tar.xz

# Install Nim toolchain
RUN NIM_VERSION="2.0.2" \
    && wget -q https://nim-lang.org/download/nim-${NIM_VERSION}-linux_x64.tar.xz \
    && tar -xf nim-${NIM_VERSION}-linux_x64.tar.xz -C /usr/local \
    && mv /usr/local/nim-${NIM_VERSION} /usr/local/nim \
    && rm nim-${NIM_VERSION}-linux_x64.tar.xz

# Configure environment for vscode non-root user
USER vscode

WORKDIR /workspace
