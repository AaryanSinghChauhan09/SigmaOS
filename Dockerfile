# SigmaOS Industrial Build Environment (Zenith v15.0)
FROM ubuntu:24.04

# Install Industrial Toolchain
RUN apt-get update && apt-get install -y \
    clang-18 \
    lld-18 \
    llvm-18 \
    nasm \
    make \
    cmake \
    nodejs \
    npm \
    qemu-system-x86 \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set Clang 18 as default compiler
RUN ln -s /usr/bin/clang-18 /usr/bin/clang++ && \
    ln -s /usr/bin/clang-18 /usr/bin/clang

# Workspace Configuration
WORKDIR /sigmaos
COPY . .

# Generate Professional Shards
RUN node populate_profiles.cjs

# Build System Baseline
CMD ["make", "all"]
