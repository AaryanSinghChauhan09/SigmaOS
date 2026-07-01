#!/bin/bash
# SigmaOS QEMU Orchestrator
# A cleaner wrapper for running the Sovereign Lattice in emulation.

QEMU_BIN="qemu-system-x86_64"
ISO_IMAGE="build/sigmaos.iso"
SERIAL_LOG="serial.log"

echo "🦞 Launching SigmaOS Sovereign Lattice..."

# Check if ISO exists
if [ ! -f "$ISO_IMAGE" ]; then
    echo "❌ Error: ISO image not found at $ISO_IMAGE. Please run 'make all' first."
    exit 1
fi

# Launch QEMU
# -m: 2GB RAM
# -serial: Output to file
# -display: VGA support
$QEMU_BIN \
    -m 2G \
    -cdrom "$ISO_IMAGE" \
    -serial stdio \
    -vga std \
    -display gtk,zoom-to-fit=on \
    2>&1 | tee "$SERIAL_LOG"
