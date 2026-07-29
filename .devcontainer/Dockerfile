FROM mcr.microsoft.com/devcontainers/cpp:1-ubuntu-22.04

# Install additional SigmaOS dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        build-essential \
        gdb \
        clang-format \
        cmake \
        git \
        curl \
        qemu-system-x86 \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Add a non-root user (done by the base image as 'vscode')
USER vscode
