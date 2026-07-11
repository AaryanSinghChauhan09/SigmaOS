#!/bin/bash
# =========================================================================
# SIGMAOS: SOVEREIGN BUILD TOOLCHAIN SETUP
# =========================================================================
# Installs necessary cross-compiler and emulation tools for SigmaOS.

set -e

echo "[SIGMA-SETUP] Initializing Toolchain Installation..."

if [ -x "$(command -v apt-get)" ]; then
    echo "[SIGMA-SETUP] Debian/Ubuntu system detected."
    sudo apt-get update
    sudo apt-get install -y build-essential \
                            nasm \
                            xorriso \
                            grub-pc-bin \
                            grub-efi-amd64-bin \
                            qemu-system-x86 \
                            gcc-x86-64-linux-gnu \
                            g++-x86-64-linux-gnu
elif [ -x "$(command -v pacman)" ]; then
    echo "[SIGMA-SETUP] Arch Linux system detected."
    sudo pacman -Syu --noconfirm base-devel nasm xorriso grub qemu-desktop
elif [ -x "$(command -v brew)" ]; then
    echo "[SIGMA-SETUP] macOS (Homebrew) detected."
    brew install nasm xorriso qemu x86_64-elf-gcc x86_64-elf-binutils
else
    echo "[SIGMA-SETUP] Unsupported OS for automatic setup. Please install NASM, xorriso, GRUB, QEMU, and a cross-compiler manually."
    exit 1
fi

echo "[SIGMA-SETUP] Toolchain successfully provisioned. You can now run 'make all'."
