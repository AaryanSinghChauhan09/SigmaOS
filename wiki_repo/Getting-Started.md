# SigmaOS Getting Started Guide

Welcome to the SigmaOS development environment! This guide will walk you through setting up your environment, building the kernel, and running it in an emulator or on hardware.

## 1. Environment Setup

SigmaOS relies on a cross-compiled x86_64-elf toolchain. We provide a setup script to automate this process.

**Linux (Debian/Ubuntu/Arch)** & **macOS (Homebrew)**:
```bash
chmod +x scripts/setup.sh
./scripts/setup.sh
```

This will install:

- `gcc-x86-64-linux-gnu` / `x86_64-elf-gcc`

- `nasm` (Assembler)

- `qemu-system-x86` (Emulator)

- `xorriso` and `grub-pc-bin` (ISO generation)

## 2. Building the OS

SigmaOS uses a standard Makefile. From the root directory, run:

```bash
make all
```

This compiles the kernel, links it, and generates a bootable ISO image (`build/sigmaos.iso`) using GRUB.

## 3. Running in QEMU

To boot the newly compiled OS in QEMU, run:

```bash
make qemu
```

This will launch QEMU with 2GB of RAM and attach the serial output to your terminal. You should see the `[BOOT] SSB: Initializing Boot Nexus` messages in your terminal.

## 4. Hardware Deployment

To boot SigmaOS on real hardware, you can flash the ISO to a USB drive using `dd` (Linux/macOS) or Rufus (Windows).

```bash

# Example on Linux (Replace /dev/sdX with your USB drive)

sudo dd if=build/sigmaos.iso of=/dev/sdX bs=4M status=progress
```

**Note:** Ensure your hardware is supported and secure boot is disabled, as SigmaOS is currently self-signed.
