#!/bin/bash
# =============================================================================
# Σ SIGMAOS: EDGE & IOT DEPLOYMENT ORCHESTRATOR
# =============================================================================
# Cross-compiles the Sovereign Lattice for lightweight Edge and IoT targets.
# Supports:
#   - rpi3   : ARM64 Broadcom BCM2837 (Raspberry Pi 3)
#   - riscv  : SiFive RISC-V 64-bit boards & QEMU virt
#
# Usage:
#   ./deploy_edge.sh --target rpi3
#   ./deploy_edge.sh --target riscv
# =============================================================================

set -e

TARGET=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --target) TARGET="$2"; shift 2 ;;
        *)        echo "Unknown option: $1"; exit 1 ;;
    esac
done

if [ -z "$TARGET" ]; then
    echo "Usage: ./deploy_edge.sh --target [rpi3|riscv]"
    exit 1
fi

echo "╔══════════════════════════════════════════════════════════╗"
echo "║  Σ SigmaOS Edge Deployment Orchestrator                  ║"
echo "║  Target: $(printf '%-39s' "$TARGET") ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# Generate feature flags tailored for Edge targets
# Edge targets need minimal memory overhead and specific driver configs
cat <<EOF > sigma_features.json
{
    "arch": "$(if [ "$TARGET" == "rpi3" ]; then echo "aarch64"; else echo "riscv64"; fi)",
    "drivers": {
        "serial": "$(if [ "$TARGET" == "rpi3" ]; then echo "bcm2837"; else echo "sifive"; fi)",
        "display": "none",
        "storage": "none",
        "network": "none"
    },
    "features": {
        "slab_pools": true,
        "ai": true,
        "gui": false,
        "network": false
    },
    "memory": {
        "pool_pages": 4,
        "max_alloc_mb": 16
    },
    "build": {
        "version": 12,
        "channel": "edge"
    }
}
EOF

echo "Σ [1/3] Configured edge capabilities (GUI disabled, AI enabled, footprint reduced)"

# Run the cross-compilation builder
ARCH=$(if [ "$TARGET" == "rpi3" ]; then echo "aarch64"; else echo "riscv64"; fi)
echo "Σ [2/3] Cross-compiling for $ARCH..."

if command -v python3 &>/dev/null; then
    python3 scripts/sovereign_builder.py "$ARCH"
else
    echo "❌ Error: Python 3 required for Sovereign Builder."
    exit 1
fi

# Package for the specific board
echo "Σ [3/3] Packaging payload for $TARGET..."
if [ "$TARGET" == "rpi3" ]; then
    echo "  >> Generating kernel8.img for Raspberry Pi SD Card..."
    # Normally we would use aarch64-linux-gnu-objcopy here
    if command -v aarch64-linux-gnu-objcopy &>/dev/null; then
        aarch64-linux-gnu-objcopy -O binary build/sigmaos_aarch64.bin build/kernel8.img
        echo "  ✅ Done. Copy build/kernel8.img to the boot partition of your Pi's SD card."
    else
        echo "  ⚠️  Missing aarch64-linux-gnu-objcopy. Cannot generate raw kernel8.img."
        echo "  ℹ️  Binary available at build/sigmaos_aarch64.bin"
    fi
elif [ "$TARGET" == "riscv" ]; then
    echo "  >> Kernel ready for OpenSBI injection..."
    echo "  ✅ Done. Boot with QEMU: qemu-system-riscv64 -machine virt -bios default -kernel build/sigmaos_riscv64.bin -nographic"
fi

echo ""
echo "Deployment synthesis complete."
