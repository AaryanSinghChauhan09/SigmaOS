# BuildGuide

1

Welcome to SigmaOS. This guide provides practical steps for building, running, and deploying the Sovereign Silicon Entity on physical hardware or a virtual machine.

1

To build SigmaOS, you will need a modern Linux environment (Ubuntu 22.04+ recommended) with the following tools installed:

1

sudo apt update
sudo apt install build-essential g++ make qemu-system-x86 nasm mtools

1

1

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

1

1

1

The fastest way to test SigmaOS is via QEMU:

1

qemu-system-x86_64 -cdrom build/sigmaos-x86_64.iso -m 2G -serial stdio

1

1

To run SigmaOS on actual hardware:

1. Insert a USB flash drive.

1

   ```bash

   # Replace /dev/sdX with your actual USB device

   sudo dd if=build/sigmaos-x86_64.iso of=/dev/sdX bs=4M status=progress
   ```

1. Boot your machine from the USB drive. Ensure Legacy BIOS or UEFI compatibility mode is enabled in your firmware settings.

1

Upon boot, you will be greeted by the Zenith UI dashboard. You can switch to the `SigmaShell` fallback terminal by pressing `Ctrl + Alt + F1`.
