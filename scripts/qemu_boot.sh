#!/bin/bash
# =========================================================================
# Σ SIGMAOS ZENITH: QEMU BOOT LAUNCHER (v1.0)
# =========================================================================
# Usage: ./scripts/qemu_boot.sh
# Requires: qemu-system-x86_64, sigma_zenith.bin (from `make all`)
# =========================================================================

KERNEL_BIN="sigma_zenith.bin"
MEMORY="512M"
CPUS=4
SERIAL_LOG="serial.log"

# Check kernel binary exists
if [ ! -f "$KERNEL_BIN" ]; then
    echo "ERROR: $KERNEL_BIN not found. Run 'make all' first."
    exit 1
fi

echo "Σ [QEMU]: Booting SigmaOS Zenith Supreme..."
echo "  Kernel:  $KERNEL_BIN"
echo "  Memory:  $MEMORY"
echo "  CPUs:    $CPUS"
echo "  Serial:  $SERIAL_LOG"
echo ""

qemu-system-x86_64 \
    -kernel "$KERNEL_BIN" \
    -m "$MEMORY" \
    -smp "$CPUS" \
    -serial file:"$SERIAL_LOG" \
    -monitor stdio \
    -display sdl \
    -no-reboot \
    -no-shutdown \
    -d int,cpu_reset \
    -D qemu_debug.log

echo "Σ [QEMU]: SigmaOS session ended. Serial log: $SERIAL_LOG"
