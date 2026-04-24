#!/bin/bash
# =============================================================================
# Σ SIGMAOS: CONTRIBUTOR DEVELOPMENT ENVIRONMENT SETUP
# =============================================================================
# Run this script to automatically install all dependencies required to
# compile, emulate, and test the SigmaOS Sovereign Lattice.
#
# Supports: Debian/Ubuntu, macOS (Homebrew)
# =============================================================================

set -e

echo "╔══════════════════════════════════════════════════════════╗"
echo "║  Σ SigmaOS Developer Environment Setup                   ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

OS=$(uname -s)

if [ "$OS" == "Linux" ]; then
    if command -v apt-get &>/dev/null; then
        echo "[*] Detected Debian/Ubuntu Linux. Installing dependencies..."
        
        # 1. Base build tools
        sudo apt-get update
        sudo apt-get install -y build-essential python3 python3-pip git
        
        # 2. Cross-compilation toolchains
        echo "[*] Installing x86_64, aarch64, and riscv64 toolchains..."
        sudo apt-get install -y \
            gcc-x86-64-linux-gnu binutils-x86-64-linux-gnu \
            gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu \
            gcc-riscv64-unknown-elf binutils-riscv64-unknown-elf
            
        # 3. Emulators
        echo "[*] Installing QEMU for emulation..."
        sudo apt-get install -y qemu-system-x86 qemu-system-arm qemu-system-misc qemu-system-riscv64
        
        echo "[✓] Linux setup complete."
    else
        echo "[!] Unsupported Linux package manager. Please manually install: gcc-aarch64-linux-gnu, gcc-riscv64-unknown-elf, qemu-system"
        exit 1
    fi
elif [ "$OS" == "Darwin" ]; then
    if command -v brew &>/dev/null; then
        echo "[*] Detected macOS (Homebrew). Installing dependencies..."
        
        # 1. Base tools
        brew install python3
        
        # 2. Toolchains & Emulators
        echo "[*] Installing toolchains and QEMU..."
        brew tap riscv-software-src/riscv
        brew install aarch64-elf-gcc riscv-tools qemu
        
        echo "[✓] macOS setup complete."
    else
        echo "[!] Homebrew not found. Please install Homebrew (https://brew.sh/) and run again."
        exit 1
    fi
else
    echo "[!] Unsupported operating system: $OS"
    exit 1
fi

# Setup Python requirements if any
echo "[*] Checking Python dependencies..."
# If we add requirements.txt later, this will install them.
# pip3 install -r requirements.txt

echo ""
echo "══════════════════════════════════════════════════════════"
echo "  ✅ Development Environment Ready!"
echo "══════════════════════════════════════════════════════════"
echo "Next Steps:"
echo "1. Run './run_sigma_tests.sh' to verify lattice integrity."
echo "2. Run './deploy_edge.sh --target riscv' to test cross-compilation."
echo "3. Read WIKI/CONTRIBUTING.md for architecture guidelines."
echo ""
