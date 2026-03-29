# =============================================================================
# Σ SIGMAOS: SOVEREIGN CONTAINER (v21.0 - ZERO-DEPENDENCY FINALITY)
# =============================================================================
# Mission: Absolute Sovereignty (No base OS, no libraries).
# Strategy: MULTI-STAGE BUILD -> STATIC BINARY -> FROM SCRATCH
# =============================================================================

# -- STAGE 1: Forge Engine (Build) --
FROM gcc:latest AS forge
WORKDIR /forge
COPY . .
# Perform static build with direct syscall integration (Sigma v21.0)
RUN g++ -static -nostdlib -O3 SovereignLauncherZenith.cpp SovereignLibC.asm -o sigma_os_master

# -- STAGE 2: Sovereign Finality (Runtime) --
FROM scratch
LABEL maintainer="SigmaOS-Project"
LABEL architecture="x86_64"
LABEL dependency="ZERO"

# Pure machine code shard - No OS required
COPY --from=forge /forge/sigma_os_master /sigma_os_master
COPY --from=forge /forge/index.html /index.html
COPY --from=forge /forge/os_guide.md /os_guide.md

ENTRYPOINT ["/sigma_os_master"]
