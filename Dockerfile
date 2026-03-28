# Σ SIGMAOS: SOVEREIGN ZENITH CONTAINER (v10.0)
# ==========================================
# Mission: Cross-Platform Shard Deployment.
# Base: Minimal Sovereign Scratch (Alpine-based)
# ==========================================

FROM alpine:latest AS builder

RUN apk add --no-cache g++ make nasm rust cargo

WORKDIR /sigmaos
COPY . .

# Build Sovereign Kernel Shards
RUN make kernel_zenith || true
RUN make userland_zenith || true

# Production Image
FROM alpine:latest
WORKDIR /root/sigmaos

# Copy only native binaries (No Python, No interpreted bloat)
COPY --from=builder /sigmaos/SigmaOS_Kernel.exe .
COPY --from=builder /sigmaos/SovereignShell.exe .
COPY --from=builder /sigmaos/index.html .
COPY --from=builder /sigmaos/userland ./userland

EXPOSE 80
EXPOSE 2222

# Launch the Metal-Nexus Web Bridge
CMD ["./SigmaOS_Kernel.exe", "--web-bridge", "80"]
