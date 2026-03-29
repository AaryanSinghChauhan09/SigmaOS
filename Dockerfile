# =============================================================================
# Σ SIGMAOS ZENITH SUPREME: INDUSTRIAL CONTAINER SHARD (v94.0)
# =============================================================================
# Mission: Sovereign, Zero-Dependency Container Runtime.
# USP: Docker, Podman, and BuildKit parity in a single industrial shard.
# =============================================================================

# --- 1. BUILD STAGE (Sovereign Toolchain) ---
FROM fedora:39 AS builder

# Install industrial dev tools (C, C++, ASM, Rust)
RUN dnf groupinstall -y "Development Tools" "C Development Tools and Libraries" \
    && dnf install -y nasm rustc lld make \
    && dnf clean all

# Sync the Zenith repository
WORKDIR /sigmaos
COPY . .

# Build the Sovereign Zenith binary (Direct silicon execution)
RUN make zenith

# --- 2. RUNTIME STAGE (Zero-Interference Shard) ---
FROM scratch

# Σ SIGMAOS: The container itself IS the OS sharding logic.
# No base alpine/ubuntu - pure SigmaOS binary.
COPY --from=builder /sigmaos/build/sigmaos_zenith /sigmaos_zenith
COPY --from=builder /sigmaos/index.html /index.html
COPY --from=builder /sigmaos/index.js /index.js

# Entrypoint logic for the Zenith Master
ENTRYPOINT ["/sigmaos_zenith"]
CMD ["--mode=industrial"]

# EXPOSE industrial ports (Sovereign Network Mesh)
EXPOSE 80 443 2222 5555
