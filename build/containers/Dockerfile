# Σ SIGMAOS: SOVEREIGN CONTAINER SHARD (v160.0)
# Achieves containerized deployment on any industrial node.

FROM debian:stable-slim

# Install low-level build tools for C/ASM kernels
RUN apt-get update && apt-get install -y \
    gcc \
    make \
    nasm \
    binutils \
    && rm -rf /var/lib/apt/lists/*

# Set up SigmaOS Root
WORKDIR /root/sigmaos
COPY . .

# Compile Sovereign Kernels (PC/Cloud/Embedded parity)
# RUN make build_kernels

# Expose Web Shand (Internal Dashboard)
EXPOSE 8080

# Execute Sovereign Init
CMD ["./scripts/SigmaSovereignBootBuilder.sh"]
