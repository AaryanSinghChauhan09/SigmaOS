#!/bin/bash
# =============================================================================
# SIGMAOS: INDUSTRIAL QEMU BOOTSTRAP v2.15 (ZENITH)
# =============================================================================
# Supports Multi-Arch Deployment: x86_64, AArch64, RISCV64
# =============================================================================

ARCH=${1:-x86_64}
MEMORY=${2:-2G}
IMAGE="sigmaos-${ARCH}.bin"

echo "[BOOT] Launching SigmaOS Zenith Singularity (${ARCH})..."

if [ ! -f "$IMAGE" ]; then
    echo "[ERROR] Kernel image $IMAGE not found. Run 'make singularity ARCH=$ARCH' first."
    exit 1
fi

case $ARCH in
    x86_64)
        qemu-system-x86_64 \
            -kernel "$IMAGE" \
            -m "$MEMORY" \
            -serial stdio \
            -device virtio-rng-pci \
            -net nic,model=virtio -net user \
            -display none \
            -cpu host,invtsc \
            -enable-kvm 2>/dev/null || qemu-system-x86_64 -kernel "$IMAGE" -m "$MEMORY" -serial stdio -display none
        ;;
    aarch64)
        qemu-system-aarch64 \
            -M virt \
            -cpu cortex-a57 \
            -m "$MEMORY" \
            -kernel "$IMAGE" \
            -serial stdio \
            -display none
        ;;
    riscv64)
        qemu-system-riscv64 \
            -M virt \
            -m "$MEMORY" \
            -kernel "$IMAGE" \
            -serial stdio \
            -display none
        ;;
    *)
        echo "[ERROR] Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

echo "[STATUS] Singularity Shut Down."
