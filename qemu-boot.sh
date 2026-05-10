#!/bin/bash
set -e

# Dependency check for QEMU
if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo "Error: qemu-system-x86_64 is not installed or not in PATH."
    echo "Please install QEMU (e.g., 'sudo apt install qemu-system-x86')."
    exit 1
fi

RECOVERY_MODE=0
if [ "$1" == "--recovery" ]; then
    echo "[INFO] Booting in RECOVERY MODE..."
    RECOVERY_MODE=1
fi

# Build the kernel
echo "Building SigmaOS kernel..."
if ! make all; then
    if [ $RECOVERY_MODE -eq 1 ]; then
        echo "Build failed, but continuing with fallback recovery image..."
    else
        echo "Kernel build failed. Use --recovery to boot fallback image."
        exit 1
    fi
fi

KERNEL_IMAGE="sigmaos.bin"
if [ $RECOVERY_MODE -eq 1 ] && [ -f "recovery.bin" ]; then
    KERNEL_IMAGE="recovery.bin"
fi

if [ ! -f "$KERNEL_IMAGE" ]; then
    echo "Error: Kernel image $KERNEL_IMAGE not found!"
    exit 1
fi

echo "Booting SigmaOS in QEMU..."
# Start QEMU with serial output directed to serial.log
qemu-system-x86_64 -m 1G -kernel "$KERNEL_IMAGE" -serial file:serial.log -nographic -no-reboot

echo "Checking boot logs..."
if grep -q "SOVEREIGN BOOT" serial.log; then
    echo "Boot successful: SOVEREIGN BOOT message found."
else
    echo "Boot failed: SOVEREIGN BOOT message not found."
    exit 1
fi
