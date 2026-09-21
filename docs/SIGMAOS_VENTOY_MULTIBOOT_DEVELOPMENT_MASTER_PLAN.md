# SigmaOS Ventoy Multi-Boot USB Tool Development Master Plan

## Executive Summary

The **SigmaOS Ventoy Multi-Boot USB Subsystem** (`ItsFossVentoyMultiBootUsbEngine` in `src/compatibility/itsfoss_inspiration_suite.rs`) is engineered as a cross-platform live USB creator bridging Linux multiboot paradigms (Ventoy dual-partition MBR/GPT disk layouts, GRUB2 loopback ISO scanning, exFAT/NTFS storage partitions, Memdisk/WIMBoot image mapping, `ventoy.json` persistence plugins) with BSD live bootable media tools (FreeBSD memstick ISO loopback, OpenBSD `bioctl` bootable USB keys, NetBSD live ISO images).

This document defines the master development plan for the SigmaOS Ventoy multiboot engine across architectural pillars, subsystem specifications, a 4-phase chronological development roadmap, and verification benchmark metrics.

---

## 1. Architectural Philosophy & Cross-Distro Inspirations

```
                          ┌──────────────────────────────────────────────────────────┐
                          │     SigmaOS Ventoy Multi-Boot Live USB Subsystem        │
                          └────────────────────────────┬─────────────────────────────┘
                                                       │
      ┌────────────────────────────────────────────────┼────────────────────────────────────────────────┐
      ▼                                                ▼                                                ▼
┌───────────────────────────┐            ┌───────────────────────────┐            ┌───────────────────────────┐
│     Linux Multiboot       │            │  Multi-Format Image Load  │            │   BSD Live Media Load     │
│ • Ventoy Dual-Partition   │            │ • Linux ISO (loopback)    │            │ • FreeBSD Memstick ISO    │
│ • Partition 1: exFAT Data │            │ • Windows WIMBoot / VHD   │            │ • OpenBSD bioctl Keys     │
│ • Partition 2: VTOY_EFI   │            │ • Raw IMG / Memdisk RAM   │            │ • NetBSD Live Boot        │
└───────────────────────────┘            └───────────────────────────┘            └───────────────────────────┘
```

---

## 2. Six Core Ventoy Multiboot Development Pillars

### Pillar 1: Dual-Partition Disk Layout Generator
* **Ventoy Partition Architecture**:
  - Partition 1 (`exFAT`/`NTFS`/`ext4`): Large user-accessible data storage partition holding ISO, IMG, VHD, and WIM images without file extraction.
  - Partition 2 (`VTOY_EFI`): Small 32MB FAT16 partition containing GRUB2 EFI bootloaders, Secure Boot certificates, and Ventoy core modules.

### Pillar 2: Dynamic GRUB2 Loopback Menu Generator
* **Automated Boot Menu Synthesizer**:
  - Scan Partition 1 recursively for bootable image files.
  - Dynamically generate `grub.cfg` entries:
    ```grub
    menuentry 'Ubuntu 24.04 LTS (ubuntu-24.04.iso)' {
        loopback loop /iso/ubuntu-24.04.iso
        linux (loop)/casper/vmlinuz boot=casper iso-scan/filename=/iso/ubuntu-24.04.iso quiet splash
        initrd (loop)/casper/initrd
    }
    ```

### Pillar 3: Multi-Format Image Support (ISO, IMG, VHD, WIMBoot, Memdisk)
* **Image Boot Loaders**:
  - **Linux ISO**: GRUB2 `loopback` + `iso-scan` parameter injection.
  - **Windows WIM/VHD**: WIMBoot memory-mapping loader for booting Windows ISOs and VHDX virtual disks directly.
  - **Floppy / IMG**: Memdisk RAM-disk loader mapping floppy images into physical memory.

### Pillar 4: Persistent Storage Overlay Manager (`persistence.dat`)
* **Overlay File Management**:
  - Create and link `persistence.dat` overlay files to specific Linux ISOs (Ubuntu `casper-rw`, Arch `cow_space`), persisting changes across live USB reboots.

### Pillar 5: Unattended Auto-Install Injection Plugin (`ventoy.json`)
* **Auto-Installer Integration**:
  - Parse `ventoy.json` configuration file to automatically inject Kickstart (`ks.cfg`), Preseed (`preseed.cfg`), or AutoUnattend (`autounattend.xml`) scripts during ISO boot.

### Pillar 6: Cryptographic Verification & Dilithium-5 Signature Attestation
* **PQC & Hash Verification**:
  - Compute SHA256 checksums and Dilithium-5 post-quantum signatures for scanned ISO images, warning users before executing unverified image files.

---

## 3. Four-Phase Chronological Roadmap

```
  Phase 1: Dual-Partition Layout & Dynamic GRUB2 Menu Engine (Months 1–3)
  ├── Dual-Partition Layout Engine (Partition 1 exFAT + Partition 2 VTOY_EFI)
  ├── Automated ISO Image File Scanner & Directory Traversal
  └── Dynamic GRUB2 `grub.cfg` Menu Synthesizer

  Phase 2: Multi-Format Image Support & WIMBoot / Memdisk Loaders (Months 3–6)
  ├── Windows WIMBoot & VHD Direct Image Boot Loader
  ├── Memdisk Floppy & IMG Memory Mapping Engine
  └── FreeBSD Memstick & OpenBSD USB Media Loopback Loaders

  Phase 3: Persistent Storage Overlays & Unattended Auto-Installer (Months 6–9)
  ├── Persistent Storage `persistence.dat` Overlay Manager
  ├── `ventoy.json` Plugin Parser & Kickstart / Preseed Script Injector
  └── SHA256 & Dilithium-5 Cryptographic Image Signature Verifier

  Phase 4: Zenith GUI Multiboot Creator & Release Engineering (Months 9–12)
  ├── Zenith Desktop One-Click Graphical Ventoy USB Creator Tool
  ├── Multi-OS Boot Compatibility Matrix Test Suite (100+ Distros)
  └── Sub-30 Second USB Provisioning Benchmark
```

---

## 4. Verification and Benchmark Metrics

| Subsystem Target | Benchmark Framework | Target Performance Metric |
| :--- | :--- | :--- |
| **USB Layout Creation** | Ventoy Formatting Test | < 30 seconds to format and provision dual-partition USB |
| **Boot Menu Synthesis** | 50 ISO Image Scan Test | Sub-2 second scan and `grub.cfg` generation latency |
| **Loopback Boot Success** | Multi-OS Boot Matrix | 100% boot success across Linux, BSD, and Windows ISOs |
| **Persistent Overlay IO** | Persistence Write Benchmark | > 60 MB/s sustained overlay file I/O throughput |
