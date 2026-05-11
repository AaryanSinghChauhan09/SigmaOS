<<<<<<< HEAD

# 🚀 SigmaOS Installation Guide

Welcome to the SigmaOS Installation Guide. This document provides a step-by-step procedure to deploy the SigmaOS Sovereign Lattice onto physical hardware or a virtualized environment.

---
=======
﻿1


Welcome to the SigmaOS Installation Guide. This document provides a step-by-step procedure to deploy the SigmaOS Sovereign Lattice onto physical hardware or a virtualized environment.


1

>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f


## 💻 Supported Hardware

<<<<<<< HEAD
| Platform | Support Tier | Notes |
| :--- | :--- | :--- |
| **Raspberry Pi 5 (BCM2712)** | Tier 1 (Optimized) | Full silicon-level hardware acceleration. |
| **Raspberry Pi 4 (BCM2711)** | Tier 1 (Supported) | Primary development target. |
| **x86_64 Systems (Intel/AMD)** | Tier 2 (Stable) | Generic drivers for UEFI systems. |
| **RISC-V Generic** | Experimental | Basic kernel boot only. |

---


## ⚡ Quick Deployment (Physical Hardware)

1. **Download the Official Image**
   Download the latest `sigmaos_aarch64.img` from the GitHub Release page.
=======

1


1. **Download the Official Image**
   Download the latest `sigmaos_aarch64.img` from the release page.

1. **Flash to SD Card / NVMe**
   Use `dd` or a tool like BalenaEtcher to flash the image.
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

2. **Flash to SD Card / NVMe**
   Use `dd` or a tool like BalenaEtcher to flash the image.
   ```bash
   sudo dd if=sigmaos_aarch64.img of=/dev/sdX bs=4M status=progress
   ```

3. **Booting**
   Insert the drive and power on. The Sovereign Boot Manager will initialize the lattice shards and launch the Zenith UI automatically.

<<<<<<< HEAD
---
=======

1

>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f


## 🔬 Virtualization (QEMU)

If you want to test SigmaOS without physical hardware, use the QEMU target.

1. **Build the Kernel**
<<<<<<< HEAD
   Follow the instructions in [Build.md](Build.md) to compile `sigma_os.elf`.
=======
   Follow the instructions in the `Build.md` to compile `sigma_os.elf`.

1. **Run QEMU**
>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

2. **Run QEMU**
   ```bash
   qemu-system-aarch64 \
       -machine raspi4b \
       -cpu cortex-a72 \
       -m 2G \
       -kernel sigma_os.elf \
       -serial stdio
   ```

<<<<<<< HEAD
---


## 🌓 Dual-Boot Configuration
=======

1

>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f

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

<<<<<<< HEAD
---


## 🎯 Post-Installation

Upon first boot, the **Onboarding Wizard** will launch to:
- Detect your hardware and apply **Smart Defaults**.
- Prompt for your **Profession Profile** (Lawyer, Doctor, Engineer, etc.).
- Configure **Verified Boot** and PQC security levels.

---
*Welcome to the future of sovereign computing.*
=======

1


Upon first boot, the OS will automatically detect your profession profile and initialize the Zenith UI Dashboard. Use the Context Manager hooks to override defaults if needed.

>>>>>>> c682b9ae193869d405d851dfbeb13314cb964f9f
