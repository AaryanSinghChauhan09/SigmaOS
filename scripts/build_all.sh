#!/bin/bash
# =============================================================================
# SIGMAOS: UNIFIED BUILD ORCHESTRATOR
# =============================================================================

ARCHS=("x86_64" "aarch64" "riscv64")

echo "[BUILD] Starting Unified SigmaOS Zenith Build..."

for ARCH in "${ARCHS[@]}"; do
    echo "[BUILD] Orchestrating $ARCH Shard Lattice..."
    make singularity ARCH=$ARCH
    if [ $? -eq 0 ]; then
        echo "[SUCCESS] $ARCH Shard Verified."
    else
        echo "[FAILURE] $ARCH Shard Failed."
    fi
done

echo "[BUILD] Generating Unified Manifest..."
ls -lh sigmaos-*.bin

echo "[STATUS] All Industrial Shards Processed."
