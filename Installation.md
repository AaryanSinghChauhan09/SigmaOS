# SigmaOS Installation Guide

Welcome to the SigmaOS Installation Guide. This document provides a step-by-step procedure to deploy the SigmaOS Sovereign Lattice onto physical hardware or a virtualized environment.

## Supported Hardware
1. **Raspberry Pi 5 (BCM2712)** - *Tier 1 Supported (Hardware-Optimized)*
2. **Raspberry Pi 4 (BCM2711)** - *Tier 1 Supported*
3. **x86_64 Systems (Intel/AMD)** - *Tier 2 Supported*
4. **RISC-V Generic** - *Experimental*

## Method 1: Bare-Metal Flashing (Recommended)

1. **Download the Official Image**
   Download the latest `sigmaos_aarch64.img` from the release page.

2. **Flash to SD Card / NVMe**
   Use `dd` or a tool like BalenaEtcher to flash the image.
   ```bash
   sudo dd if=sigmaos_aarch64.img of=/dev/sdX bs=4M status=progress
   ```

3. **Booting**
   Insert the drive into your device and power it on. The Sovereign Boot Manager will initialize the context manager and load the lattice.

## Method 2: QEMU Virtualization

If you want to test SigmaOS without physical hardware, you can use QEMU.

1. **Build the Kernel**
   Follow the instructions in the `Build.md` to compile `sigma_os.elf`.

2. **Run QEMU**
   ```bash
   qemu-system-aarch64 \
       -machine raspi4b \
       -cpu cortex-a72 \
       -m 2G \
       -kernel sigma_os.elf \
       -serial stdio
   ```

## Method 3: Dual-Boot via Systemd-Boot (x86_64 Only)

The `SovereignPartitionManager` supports scanning existing GPT partitions and bridging with systemd-boot.

1. **Copy the Kernel**
   Copy `sigma_os_x64.elf` to your EFI partition.
   ```bash
   sudo cp sigma_os_x64.elf /boot/efi/EFI/sigmaos/
   ```

2. **Add Boot Entry**
   Create `/boot/efi/loader/entries/sigmaos.conf`:
   ```ini
   title   SigmaOS Sovereign Lattice
   linux   /EFI/sigmaos/sigma_os_x64.elf
   options root=PARTUUID=XXXX-XXXX rw
   ```

## Post-Installation Setup
Upon first boot, the OS will automatically detect your profession profile and initialize the Zenith UI Dashboard. Use the Context Manager hooks to override defaults if needed.
