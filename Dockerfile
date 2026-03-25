# Σ SIGMA OS: SOVEREIGN CONTAINER ZENITH (DOCKER)
# ===============================================
# USP: Isolated Shard Execution Environment (Zero Forensics).
# Capability: Compiles and Runs the Native Sovereign Kernel.

FROM ubuntu:24.04

# 1. Install Bare-Metal Tooling (GCC/G++, Make, Glibc)
RUN apt-get update && apt-get install -y \
    build-essential \
    g++ \
    make \
    git \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 2. Setup Shard Workspace
WORKDIR /app/sigma_os
COPY . .

# 3. Silicon Compilation Sequence (SOLID)
# We use the existing Makefile for bit-perfect builds.
RUN make clean && make all

# 4. Environment Variables (Kernel Context)
ENV SIGMA_MODE=ZENITH
ENV SIGMA_LOG=DEBUG

# 5. Boot Sequence
# Launch the kernel directly into the container's PID-1.
CMD ["./sigma_kernel.exe"]
