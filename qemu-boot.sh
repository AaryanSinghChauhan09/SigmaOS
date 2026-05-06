#!/bin/bash
set -e

# Build the kernel
echo "Building SigmaOS kernel..."
make -C kernel all

echo "Booting SigmaOS in QEMU..."
# Start QEMU with serial output directed to serial.log
qemu-system-x86_64 -m 1G -kernel kernel/sigmaos.bin -serial file:serial.log -nographic -no-reboot

echo "Checking boot logs..."
if grep -q "SOVEREIGN BOOT" serial.log; then
    echo "Boot successful: SOVEREIGN BOOT message found."
else
    echo "Boot failed: SOVEREIGN BOOT message not found."
    exit 1
fi
