# Installation Guide

This guide covers building SigmaOS from source and running it in a virtual machine.

***

## Table of Contents

1.  [Prerequisites](#prerequisites)
2.  [Building from Source](#building-from-source)
3.  [Running in QEMU](#running-in-qemu)
4.  [Cross-Compilation](#cross-compilation)
5.  [Building a Bootable ISO](#building-a-bootable-iso)
6.  [Installing on Real Hardware](#installing-on-real-hardware)
7.  [Troubleshooting](#troubleshooting)

***

## Prerequisites

### Required Tools

| Tool | Minimum Version | Purpose |
|------|----------------|---------|
| Rust | nightly-2024-01-01 | Primary build toolchain |
| GCC / G++ | 12.0+ | C kernel components |
| LLVM / Clang | 16+ | LTO, CFI, BOLT optimisations |
| QEMU | 7.0+ | Virtual machine testing |
| CMake | 3.20+ | C/C++ build coordination |
| Python | 3.10+ | Build scripts |
| Git | 2.40+ | Version control |

### Install on Ubuntu/Debian

```bash
# System packages
sudo apt-get update
sudo apt-get install -y \
    build-essential gcc g++ clang llvm \
    cmake ninja-build \
    qemu-system-x86 qemu-system-arm \
    python3 python3-pip \
    git curl wget

# Rust nightly toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup default nightly

# Additional Rust targets
rustup target add x86_64-unknown-none
rustup target add aarch64-unknown-none
rustup target add riscv64gc-unknown-none-elf

# Required Rust components
rustup component add rust-src llvm-tools-preview
```

### Install on Arch Linux

```bash
sudo pacman -S base-devel clang llvm cmake ninja qemu-base python git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly && rustup default nightly
rustup target add x86_64-unknown-none aarch64-unknown-none
rustup component add rust-src llvm-tools-preview
```

### Install on Fedora

```bash
sudo dnf install -y gcc g++ clang llvm cmake ninja-build qemu python3 git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly && rustup default nightly
rustup target add x86_64-unknown-none
rustup component add rust-src llvm-tools-preview
```

***

## Building from Source

### 1. Clone the Repository

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

### 2. Check and Build

```bash
# Verify the build environment
cargo check 2>&1

# Build in debug mode
cargo build

# Build in release mode (optimised)
cargo build --release

# Build with specific features
cargo build --release --features "microkernel,desktop,ai"
```

### 3. Run Tests

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --test algorithm_and_components_inspection_tests
cargo test --test distro_inspection_and_security_tests
cargo test --test linux_bsd_inspection_tests
cargo test --test sovereign_inspection_suite

# Run the full inspection binary
./algorithm_and_components_inspection_tests

# Stress and fuzz tests
cargo test --test stress_and_fuzz_tests
```

***

## Running in QEMU

### Quick Start (x86\_64)

```bash
# Build and run the QEMU smoke test
python3 scripts/qemu_smoke_test.py

# Or manually with QEMU
qemu-system-x86_64 \
    -kernel build/sigma_kernel \
    -m 512M \
    -nographic \
    -serial mon:stdio
```

### QEMU with Networking

```bash
qemu-system-x86_64 \
    -kernel build/sigma_kernel \
    -m 1G \
    -netdev user,id=net0,hostfwd=tcp::2222-:22 \
    -device e1000,netdev=net0 \
    -nographic
```

### QEMU with Disk

```bash
# Create a disk image
qemu-img create -f qcow2 sigma_disk.qcow2 20G

qemu-system-x86_64 \
    -kernel build/sigma_kernel \
    -m 2G \
    -drive file=sigma_disk.qcow2,format=qcow2 \
    -nographic
```

***

## Cross-Compilation

### aarch64 (ARM64)

```bash
# Install cross-toolchain
sudo apt-get install gcc-aarch64-linux-gnu

# Build with CMake
cmake -B build-aarch64 \
    -DCMAKE_TOOLCHAIN_FILE=toolchain-aarch64.cmake \
    -DCMAKE_BUILD_TYPE=Release
cmake --build build-aarch64

# Or with Cargo
cargo build --target aarch64-unknown-none --release
```

### RISC-V 64

```bash
# Install cross-toolchain
sudo apt-get install gcc-riscv64-linux-gnu

cmake -B build-riscv64 \
    -DCMAKE_TOOLCHAIN_FILE=toolchain-riscv64.cmake
cmake --build build-riscv64

# Or with Cargo (experimental)
cargo build --target riscv64gc-unknown-none-elf --release
```

***

## Building a Bootable ISO

```bash
# Build the full bootable ISO
bash scripts/build-iso.sh

# The ISO will be at:
ls -la build/sigmaos.iso

# Test the ISO in QEMU
qemu-system-x86_64 \
    -cdrom build/sigmaos.iso \
    -m 2G \
    -boot d \
    -vga std
```

### ISO Contents

The ISO follows standard Linux ISO structure:

    iso_root/
    ├── boot/
    │   ├── sigma_kernel      # Kernel binary
    │   └── grub/             # GRUB2 bootloader
    ├── installer/
    │   └── install.sh        # Automated installer
    └── sigma/
        └── store/            # Base system packages

***

## Installing on Real Hardware

> ⚠️ **Warning**: SigmaOS is pre-release software. Installing on real hardware may result in data loss. Always back up your data first.

### Minimal Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | x86\_64, 1 GHz | x86\_64 4+ cores, 2+ GHz |
| RAM | 256 MB | 2 GB+ |
| Storage | 4 GB | 20 GB+ SSD |
| Architecture | x86\_64 | x86\_64 or aarch64 |

### Installation Steps

```bash
# 1. Write ISO to USB drive (replace /dev/sdX with your device)
sudo dd if=build/sigmaos.iso of=/dev/sdX bs=4M status=progress

# 2. Boot from USB

# 3. Run the installer
bash /installer/install.sh

# 4. Follow the on-screen prompts
```

### Installer Options

```bash
# Interactive installation
bash install.sh --interactive

# Automated installation (expert users)
bash install.sh \
    --disk /dev/sda \
    --hostname my-sigmaos \
    --username sigma \
    --timezone America/New_York \
    --locale en_US.UTF-8
```

***

## Troubleshooting

### Build Fails with "unknown feature"

```bash
# Ensure you're on nightly Rust
rustup show
rustup default nightly
```

### QEMU: "No bootable device"

```bash
# Check kernel binary exists
ls -la build/sigma_kernel

# Rebuild from scratch
cargo clean && cargo build --release
```

### Linker Errors for kernel binary

```bash
# Install LLVM linker
sudo apt-get install lld
# Set linker in .cargo/config.toml:
# [target.x86_64-unknown-none]
# linker = "rust-lld"
```

### Test Suite Failures

```bash
# Run with verbose output
cargo test -- --nocapture 2>&1 | head -100

# Check for compilation warnings
cargo check 2>&1 | grep -i warning | head -20
```

### Out of Memory During Build

```bash
# Reduce parallel codegen units
CARGO_BUILD_JOBS=2 cargo build --release
```

***

## Development Workflow

```bash
# Quick iteration loop
cargo check          # Fast compile check (no linking)
cargo test           # Run all tests
cargo clippy         # Lint with suggestions

# Before committing
cargo fmt            # Format code
cargo clippy -- -D warnings   # Fail on any warning
cargo test           # Full test suite
```

***

## Getting Help

*   **GitHub Issues**: https://github.com/AaryanSinghChauhan09/SigmaOS/issues
*   **GitHub Discussions**: https://github.com/AaryanSinghChauhan09/SigmaOS/discussions
*   **Wiki**: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki
*   **Contributing Guide**: [CONTRIBUTING.md](CONTRIBUTING)
