# SigmaOS Sovereign Native Toolchain Build Environment
# Enables cross-platform compilation of the C11/Assembly Lattice

FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    build-essential \
    gcc-x86-64-linux-gnu \
    nasm \
    make \
    python3 \
    git \
    clang-tidy \
    cppcheck \
    qemu-system-x86 \
    valgrind \
    linux-tools-common \
    linux-tools-generic \
    rustc \
    cargo \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /sigmaos

# Command to build the OS
CMD ["make", "all"]
