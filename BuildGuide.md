# Getting Started with SigmaOS

Welcome to SigmaOS. This guide provides practical steps for building, running, and deploying the Sovereign Silicon Entity on physical hardware or a virtual machine.

## Prerequisites

To build SigmaOS, you will need a modern Linux environment (Ubuntu 22.04+ recommended) with the following tools installed:

```bash

sudo apt update
sudo apt install build-essential g++ make qemu-system-x86 nasm mtools

```

## Building from Source

SigmaOS uses the `s-cli` orchestrator for all build operations.

1. **Compile the Orchestrator:**
   If you haven't already, compile the CLI:

   ```bash
   g++ -std=c++20 orchestrator/main.cpp -o s-cli
   ```

1. **Select a Build Profile:**
   Configure the system for your target environment:

   ```bash
   ./s-cli profile dev
   ```

1. **Compile the Lattice:**
   Build the kernel and all enabled shards for the x86_64 architecture:

   ```bash
   ./s-cli build x86_64
   ```

###    This will generate a bootable `.iso` image in the `build/` directory.

## Running SigmaOS

### In an Emulator (QEMU)

The fastest way to test SigmaOS is via QEMU:

```bash

qemu-system-x86_64 -cdrom build/sigmaos-x86_64.iso -m 2G -serial stdio

```

### On Bare Metal

To run SigmaOS on actual hardware:

1. Insert a USB flash drive.

# Warning: This will erase all data on the USB drive.

   ```bash
   # Replace /dev/sdX with your actual USB device
   sudo dd if=build/sigmaos-x86_64.iso of=/dev/sdX bs=4M status=progress
   ```

1. Boot your machine from the USB drive. Ensure Legacy BIOS or UEFI compatibility mode is enabled in your firmware settings.

## Navigating the Zenith UI

Upon boot, you will be greeted by the Zenith UI dashboard. You can switch to the `SigmaShell` fallback terminal by pressing `Ctrl + Alt + F1`.

