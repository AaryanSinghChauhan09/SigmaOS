#!/usr/bin/env bash
# QEMU Hardware Driver Test Harness
# Use this to test SigmaOS driver development (USB, NVMe, Audio, Net)

set -e

echo "Starting SigmaOS QEMU Driver Test Harness..."

# Base QEMU flags for x86_64
QEMU_FLAGS=(
    -m 2G
    -cpu host
    -enable-kvm
    -smp 4
    -serial stdio
    -d guest_errors
)

# 1. Network Driver Test (e1000 + VirtIO Net)
QEMU_FLAGS+=(
    -netdev user,id=net0,hostfwd=tcp::8080-:80
    -device e1000,netdev=net0
    -device virtio-net,netdev=net0
)

# 2. USB Driver Test (xHCI + Keyboard + Mouse + Mass Storage)
# Creates a virtual USB 3.0 controller and attaches devices
dd if=/dev/zero of=usb_disk.img bs=1M count=64 2>/dev/null
QEMU_FLAGS+=(
    -device qemu-xhci,id=xhci
    -device usb-kbd,bus=xhci.0
    -device usb-mouse,bus=xhci.0
    -drive if=none,id=usbdrv,format=raw,file=usb_disk.img
    -device usb-storage,bus=xhci.0,drive=usbdrv
)

# 3. NVMe Driver Test
# Creates a virtual NVMe controller and a 1GB namespace
dd if=/dev/zero of=nvme_disk.img bs=1M count=1024 2>/dev/null
QEMU_FLAGS+=(
    -drive file=nvme_disk.img,if=none,id=nvmegen1,format=raw
    -device nvme,serial=deadbeef,drive=nvmegen1
)

# 4. Audio Driver Test (Intel HDA / AC97)
QEMU_FLAGS+=(
    -audiodev pa,id=snd0
    -device intel-hda -device hda-output,audiodev=snd0
    -device AC97,audiodev=snd0
)

# Replace this with the actual path to your bootable ISO/kernel
KERNEL_IMAGE="target/x86_64-unknown-none/debug/sigmaos-kernel"

if [ -f "$KERNEL_IMAGE" ]; then
    echo "Running with kernel image: $KERNEL_IMAGE"
    qemu-system-x86_64 "${QEMU_FLAGS[@]}" -kernel "$KERNEL_IMAGE"
else
    echo "Kernel not built. Run 'cargo build -p sigmaos-kernel' first."
    echo "Command would be: qemu-system-x86_64 ${QEMU_FLAGS[*]}"
    exit 1
fi
