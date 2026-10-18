#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# SigmaOS Bare-Metal Standalone Boot Image Synthesizer
# Compiles the true PE32+ UEFI executable and synthesizes a genuine FAT32 ESP boot image.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
BOOT_DIR="${ROOT_DIR}/build/boot"
EFI_BIN="${BOOT_DIR}/BOOTX64.EFI"
IMAGE_PATH="${BOOT_DIR}/sigmaos_standalone.img"

mkdir -p "${BOOT_DIR}"

echo "=========================================================="
echo "SigmaOS Bare-Metal Standalone Boot Image Synthesizer"
echo "Target: ${IMAGE_PATH}"
echo "=========================================================="

# 1. Compile native PE32+ UEFI Bootloader
echo "--> Compiling native PE32+ UEFI binary (x86_64-unknown-uefi)..."
rustc --target x86_64-unknown-uefi --crate-type bin -C panic=abort \
    -o "${EFI_BIN}" "${ROOT_DIR}/src/boot/uefi_bootloader_main.rs"

echo "[OK] Built native EFI binary: $(file "${EFI_BIN}")"

# 2. Synthesize true FAT32 EFI System Partition (ESP) image using mformat and mcopy
echo "--> Creating FAT32 EFI System Partition container (64MB)..."
dd if=/dev/zero of="${IMAGE_PATH}" bs=1M count=64 status=none

# Format as genuine FAT32 filesystem
mformat -i "${IMAGE_PATH}" -F -v "SIGMA_BOOT" ::

# Create standard UEFI directory tree
mmd -i "${IMAGE_PATH}" ::/EFI
mmd -i "${IMAGE_PATH}" ::/EFI/BOOT
mmd -i "${IMAGE_PATH}" ::/EFI/SIGMAOS

# Copy bootloader executable to default fallback path
mcopy -i "${IMAGE_PATH}" "${EFI_BIN}" ::/EFI/BOOT/BOOTX64.EFI
mcopy -i "${IMAGE_PATH}" "${EFI_BIN}" ::/EFI/SIGMAOS/KERNEL.EFI

echo "[OK] EFI System Partition verified:"
mdir -i "${IMAGE_PATH}" ::/EFI/BOOT

echo ""
echo "=========================================================="
echo "SUCCESS: SigmaOS Standalone Boot Image Ready!"
echo "Image location: ${IMAGE_PATH}"
echo "Standalone EFI:  ${EFI_BIN}"
echo "=========================================================="
echo ""
echo "HOW TO BOOT ON YOUR PC INDEPENDENTLY:"
echo "Method 1 (Direct USB Flash via dd):"
echo "  sudo dd if=${IMAGE_PATH} of=/dev/sdX bs=4M status=progress conv=fsync"
echo ""
echo "Method 2 (Manual Copy to any FAT32 USB drive):"
echo "  1. Format your USB stick as FAT32."
echo "  2. Create a folder named 'EFI/BOOT' on the USB drive."
echo "  3. Copy '${EFI_BIN}' to 'EFI/BOOT/BOOTX64.EFI' on the USB drive."
echo "  4. Insert USB into your PC, enter BIOS (F12/F2/Del), disable Secure Boot, and select the USB."
echo "  5. SigmaOS boots directly onto physical bare metal silicon!"
echo "=========================================================="
