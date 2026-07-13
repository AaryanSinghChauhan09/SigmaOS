# SigmaOS Installation Guide

> **Build Status**: ![CI](https://github.com/AaryanSinghChauhan09/SigmaOS/workflows/SigmaOS%20Build%20and%20Test%20Pipeline/badge.svg)

This guide covers building and installing SigmaOS from source for various deployment targets.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Build Profiles](#build-profiles)
- [Platform-Specific Instructions](#platform-specific-instructions)
- [Troubleshooting](#troubleshooting)
- [Advanced Configuration](#advanced-configuration)

---

## Prerequisites

### Common Requirements

All builds require:

- **Git**: For cloning the repository
- **Make**: For build orchestration
- **CMake**: >= 3.15 for cross-platform builds
- **Python 3**: >= 3.8 for build scripts

### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    nasm \
    cmake \
    qemu-system-x86 \
    golang-go \
    xorriso \
    grub-pc-bin \
    grub-efi-amd64-bin \
    git \
    python3 \
    python3-pip
```

### macOS

```bash
brew install \
    nasm \
    cmake \
    qemu \
    go \
    xorriso \
    coreutils
```

### Windows (WSL2)

```bash
# Install WSL2 with Ubuntu
wsl --install

# Inside WSL2 Ubuntu:
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    nasm \
    cmake \
    qemu-system-x86 \
    golang-go \
    xorriso \
    grub-pc-bin \
    grub-efi-amd64-bin
```

---

## Quick Start

### Clone Repository

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
```

### Build for QEMU (x86_64)

```bash
make clean
make all ARCH=x86_64
```

### Run in QEMU

```bash
qemu-system-x86_64 \
    -cdrom build/sigmaos.iso \
    -m 2G \
    -serial stdio \
    -no-reboot
```

### Build ISO for Bare Metal

```bash
make PROFILE=standalone all
```

The ISO will be available at `build/sigmaos.iso`.

---

## Build Profiles

SigmaOS supports multiple build profiles for different deployment targets:

### Standalone (Full Desktop)

```bash
make PROFILE=standalone all
```

**Includes**: Full desktop environment, all drivers, Zenith compositor, AI tools

**Target**: Desktop/laptop hardware

**Output**: `build/sigmaos-standalone.iso` (~2GB)

### Microkernel

```bash
make PROFILE=microkernel all
```

**Includes**: Minimal kernel, core shards only

**Target**: Embedded systems, containers

**Output**: `build/sigmaos-microkernel.bin` (<512KB)

### RTOS (Real-Time)

```bash
make PROFILE=rtos all
```

**Includes**: Hard real-time scheduler, deterministic timing

**Target**: Industrial control, medical devices

**Output**: `build/sigmaos-rtos.elf`

### Cloud

```bash
make PROFILE=cloud all
```

**Includes**: Headless image, cloud-init support

**Target**: AWS, GCP, Azure deployments

**Output**: `build/sigmaos-cloud.img.qcow2`

### Mobile

```bash
make PROFILE=mobile all
```

**Includes**: Touch-optimized UI, mobile drivers

**Target**: ARM64 Android/iOS devices

**Output**: `build/sigmaos-mobile.apk` or `.ipa`

### Browser

```bash
make PROFILE=browser all
```

**Includes**: WASM-compiled kernel, web UI

**Target**: Web browsers via WebAssembly

**Output**: `build/sigmaos-browser.wasm`

---

## Platform-Specific Instructions

### x86_64 (Intel/AMD)

```bash
make clean
make all ARCH=x86_64
```

**Supported**: QEMU, bare metal, virtualization platforms

### ARM64 (aarch64)

```bash
make clean
make all ARCH=aarch64 CROSS_COMPILE=aarch64-linux-gnu-
```

**Supported**: Raspberry Pi 4/5, ARM servers, Apple Silicon

**Requirements**: aarch64-linux-gnu toolchain

### RISC-V

```bash
make clean
make all ARCH=riscv64 CROSS_COMPILE=riscv64-linux-gnu-
```

**Supported**: RISC-V development boards, SiFive hardware

**Requirements**: riscv64-linux-gnu toolchain

---

## Development Build

### Debug Build

```bash
make DEBUG=1 all
```

Enables debug symbols, disables optimizations.

### Release Build

```bash
make RELEASE=1 all
```

Enables optimizations, strips debug symbols.

### Verbose Build

```bash
make V=1 all
```

Shows all compiler commands.

### Parallel Build

```bash
make -j$(nproc) all
```

Uses all available CPU cores.

---

## Testing

### Run Smoke Tests

```bash
./scripts/smoke-test.sh
```

### Run Unit Tests

```bash
make test
```

### Run Integration Tests

```bash
make test-integration
```

### Run QEMU Boot Test

```bash
make test-qemu
```

---

## Troubleshooting

### Build Fails with "command not found"

Install missing prerequisites from the [Prerequisites](#prerequisites) section.

### QEMU Boot Fails

Ensure QEMU is installed and the ISO was built successfully:

```bash
ls -lh build/sigmaos.iso
qemu-system-x86_64 --version
```

### Cross-Compilation Errors

Verify the cross-compiler toolchain is in your PATH:

```bash
aarch64-linux-gnu-gcc --version
riscv64-linux-gnu-gcc --version
```

### Out of Memory During Build

Reduce parallel jobs:

```bash
make -j2 all
```

### Permission Denied on Scripts

Make scripts executable:

```bash
chmod +x scripts/*.sh
```

---

## Advanced Configuration

### Custom Kernel Configuration

Edit `Config.sigma` to customize kernel parameters:

```toml
[kernel]
memory = "2048M"
cores = 4
debug = true

[shards]
enable = ["s-mm", "s-sched", "s-net", "s-fs"]
```

### Build with Custom Toolchain

```bash
make CC=/path/to/gcc CXX=/path/to/g++ all
```

### Build Specific Components

```bash
make kernel
make drivers
make userspace
```

### Clean Build Artifacts

```bash
make clean          # Remove build artifacts
make distclean      # Remove all generated files
make mrproper       # Remove everything including config
```

---

## Installation to Disk

### Create Bootable USB (Linux)

```bash
# Insert USB drive (replace /dev/sdX with your device)
sudo dd if=build/sigmaos.iso of=/dev/sdX bs=4M status=progress
sudo sync
```

### Create Bootable USB (macOS)

```bash
# Insert USB drive (replace disk2 with your device)
diskutil list
sudo diskutil unmountDisk /dev/disk2
sudo dd if=build/sigmaos.iso of=/dev/disk2 bs=4m
sudo sync
```

### Create Bootable USB (Windows)

Use [Rufus](https://rufus.ie/) or [Etcher](https://www.balena.io/etcher/):

1. Download Rufus from https://rufus.ie/
2. Select `build/sigmaos.iso`
3. Select your USB drive
4. Click "Start"

---

## Verification

### Verify ISO Integrity

```bash
sha256sum build/sigmaos.iso
```

Compare with the checksum in the release notes.

### Verify Build

```bash
./scripts/smoke-test.sh
```

All tests should pass.

---

## Next Steps

- Read [ARCHITECTURE.md](ARCHITECTURE.md) for system design
- Read [CONTRIBUTING.md](CONTRIBUTING.md) to contribute
- Join the community at [COMMUNITY.md](COMMUNITY.md)
- Report issues at [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

---

## Getting Help

- **Documentation**: [Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
- **Issues**: [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- **Support**: [SUPPORT.md](SUPPORT.md)

---

*Last Updated: 2026-07-13*
