# SigmaOS Download Guide

> **v15.0.0 Zenith — Stable Release**
> All branches unified on `main` · PQC-signed · [Interactive Download Page →](download.html)

---

## Overview

SigmaOS ships from a **single unified codebase** via CMake profile flags. Every format below
is compiled from the same `main` branch — there are no divergent forks.

```
main (unified)
 ├─ PROFILE=standalone    → 🖥️  Full desktop ISO
 ├─ PROFILE=rtos          → ⚙️  Hard real-time ELF
 ├─ PROFILE=cloud         → ☁️  Immutable cloud image
 ├─ PROFILE=microkernel   → 🧩  <512KB kernel
 ├─ PROFILE=mobile        → 📱  ARM64 APK/IPA
 ├─ PROFILE=dualboot      → 💻  Dual-boot installer
 ├─ PROFILE=distributed   → 🌐  Multi-node mesh
 ├─ PROFILE=browser       → 🌍  WASM / Chromium shell
 └─ PROFILE=app           → 📦  Cross-platform app bundle
```

All artefacts are signed with **Dilithium-5** and keys exchanged via **Kyber-1024 KEM**.

---

## 📦 Application Formats

| Format | Command / Link | Status |
|--------|----------------|--------|
| Native (Normal) | `make PROFILE=app all` | ✅ Stable |
| Electron | `npm start` or download installer | ✅ Stable |
| Java JAR / WAR | `java -jar sigmaos.jar` | 🔄 Preview |
| .NET / Mono | `dotnet run` / Mono EXE | 🔄 Preview |
| Python (PyInstaller / Wheel) | `pip install sigmaos` | ✅ Stable |
| AppImage / Snap / Flatpak | `./SigmaOS.AppImage` | ✅ Stable |
| WASM (WebAssembly) | `npm install sigmaos-wasm` | ✅ Stable |
| Mobile (APK / IPA) | Sideload or TestFlight | 🔄 Preview |
| ELF Binary | `make KERNEL_TYPE=monolithic all` | ✅ Stable |
| sigpkg | `sigma-pkg install sigmaos` | ✅ Stable |

---

## 🖥️ Standalone Formats

| Format | Build Flag | Status |
|--------|-----------|--------|
| Native Executable (ISO) | `PROFILE=standalone` | ✅ Stable |
| AppImage (Linux) | `PROFILE=standalone` | ✅ Stable |
| Portable EXE (Windows) | `PROFILE=standalone` | 🔄 Preview |
| Electron | `PROFILE=standalone` | ✅ Stable |
| Java JAR | `PROFILE=standalone` | 🔄 Preview |
| Python (PyInstaller) | `PROFILE=standalone` | ✅ Stable |
| WASM Bundle | `PROFILE=browser` | ✅ Stable |

---

## ⚙️ RTOS Formats

| Format | Build Flag | IRQ Latency | Status |
|--------|-----------|-------------|--------|
| Monolithic | `PROFILE=rtos RTOS_TYPE=monolithic` | <10 µs | ✅ Stable |
| Microkernel | `PROFILE=rtos RTOS_TYPE=microkernel` | <10 µs | ✅ Stable |
| Layered / Modular | `PROFILE=rtos RTOS_TYPE=layered` | <10 µs | ✅ Stable |
| Exokernel / Minimalist | `PROFILE=rtos RTOS_TYPE=exokernel` | <5 µs | 🔄 Preview |
| RTOS with POSIX Layer | `PROFILE=rtos RTOS_TYPE=posix` | <15 µs | ✅ Stable |
| Bare-Metal | `PROFILE=rtos RTOS_TYPE=baremetal` | <1 µs | ✅ Stable |

---

## 📱 Mobile Formats

| Format | Platform | Status |
|--------|----------|--------|
| Native APK | Android 12+ | 🔄 Preview |
| Native IPA | iOS 16+ (TestFlight) | 🔄 Preview |
| Hybrid (HTML/CSS/JS) | Android + iOS (Capacitor) | ✅ Stable |
| Cross-Platform (RN/Flutter) | Android + iOS + Desktop | ✅ Stable |
| PWA | Any modern browser | ✅ Stable |
| Mobile Game Engine | Unity / Godot | ⬜ v16.0 |

---

## 🧩 Microkernel Variants

| Variant | Kernel Size | Build Flag | Status |
|---------|------------|-----------|--------|
| Pure | <512 KB | `MK_TYPE=pure` | ✅ Stable |
| Hybrid | <1 MB | `MK_TYPE=hybrid` | ✅ Stable |
| Modular | <512 KB base | `MK_TYPE=modular` | ✅ Stable |
| Exokernel / Minimalist | <32 KB | `MK_TYPE=exokernel` | 🔄 Preview |
| With POSIX Layer | <768 KB | `MK_TYPE=posix` | ✅ Stable |

---

## 💻 Dual Boot Formats

| Format | Risk Level | Status |
|--------|-----------|--------|
| Traditional Partition | Low | 🔄 Preview |
| Separate Disk | Zero | 🔄 Preview |
| Nested Boot / Chainloading | Very Low | 🔄 Preview |
| Virtualized (QCOW2/VMDK) | Zero | ✅ Stable |
| Live USB | Zero | ✅ Stable |

---

## 🌐 Distributed Formats

| Topology | Consensus | Build Flag | Status |
|----------|-----------|-----------|--------|
| Client–Server | Central | `DIST_TYPE=client-server` | ✅ Stable |
| Peer-to-Peer | ZeroNet + CRDT | `DIST_TYPE=p2p` | ✅ Stable |
| Clustered System | RAFT (SovereignConsensus) | `DIST_TYPE=cluster` | ✅ Stable |
| Grid Computing | Work-stealing | `DIST_TYPE=grid` | 🔄 Preview |
| Cloud / SOA | sigma-bus | `DIST_TYPE=soa` | ✅ Stable |
| Distributed Ledger | PQC Txns | `DIST_TYPE=ledger` | 🔄 Preview |
| Actor Model | sigma-bus mailbox | `DIST_TYPE=actor` | ✅ Stable |

---

## ☁️ Cloud Formats

| Deployment | Image Format | Build Flag | Status |
|------------|-------------|-----------|--------|
| Public Cloud (AWS/GCE/Azure) | QCOW2 / VHD | `CLOUD_TYPE=public` | ✅ Stable |
| Private Cloud (OpenStack/Proxmox) | QCOW2 / VMDK | `CLOUD_TYPE=private` | ✅ Stable |
| Hybrid Cloud | WireGuard bundle | `CLOUD_TYPE=hybrid` | ✅ Stable |
| Multi-Cloud | Operator tar.gz | `CLOUD_TYPE=multi` | 🔄 Preview |
| Community Cloud | tar.gz | `CLOUD_TYPE=community` | ✅ Stable |
| IaaS | RAW / QCOW2 | `CLOUD_TYPE=iaas` | ✅ Stable |
| PaaS | OCI container | `docker pull sigmaos/paas:15.0` | ✅ Stable |
| SaaS | Module tar.gz | `CLOUD_TYPE=saas` | 🔄 Preview |
| FaaS / Serverless | ZIP (Lambda runtime) | `CLOUD_TYPE=faas` | 🔄 Preview |

---

## 🌍 Browser Formats

| Variant | Description | Status |
|---------|-------------|--------|
| Native Desktop | Custom Chromium + `navigator.sigmaos.*` | ✅ Stable |
| Mobile Browser | Touch-adapted, Android APK / iOS WKWebView | 🔄 Preview |
| Embedded / WebViews | Electron, Tauri, WebView2 SDK | ✅ Stable |
| Headless | No-UI Chromium for CI/automation | ✅ Stable |
| Minimalist / Lightweight | <50 MB, kiosk / IoT | ✅ Stable |
| Specialised | SecureBrowse / DevBrowser / RTBrowser | 🔄 Preview |

---

## 🖥️ Kernel Formats

| Kernel Model | Size | Scheduler | Build Flag | Status |
|-------------|------|-----------|-----------|--------|
| Monolithic | ~2 MB | MLFQ+EDF+CFS | `KERNEL_TYPE=monolithic` | ✅ Stable |
| Microkernel | <512 KB | Round-robin | `KERNEL_TYPE=microkernel` | ✅ Stable |
| Hybrid | ~1 MB | MLFQ | `KERNEL_TYPE=hybrid` | ✅ Stable |
| Exokernel | <64 KB | App-managed | `KERNEL_TYPE=exokernel` | 🔄 Preview |
| Nanokernel | <8 KB | IRQ-only | `KERNEL_TYPE=nanokernel` | 🔄 Preview |
| Modular | ~512 KB base | MLFQ | `KERNEL_TYPE=modular` | ✅ Stable |
| Monolithic + Modular | ~2 MB | MLFQ+EDF | `KERNEL_TYPE=mono-modular` | ✅ Stable |

---

## Quick Build Reference

```bash

# Clone (all branches unified)

git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Full desktop (standalone)

make PROFILE=standalone all -j$(nproc)

# Hard real-time RTOS

make PROFILE=rtos RTOS_TYPE=monolithic all -j$(nproc)

# Minimal microkernel

make PROFILE=microkernel MK_TYPE=pure all -j$(nproc)

# Cloud headless

make PROFILE=cloud CLOUD_TYPE=iaas all -j$(nproc)

# Mobile ARM64

make PROFILE=mobile ARCH=arm64 all -j$(nproc)

# WASM/Browser

make PROFILE=browser all -j$(nproc)

# Check stub implementations

make check-stubs

# Run in QEMU

qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -serial stdio
```

---

## Installation Instructions

### Windows Dual-Boot Installation

**Prerequisites**:

- Windows 10/11 with at least 20GB free space

- 8GB USB drive

- Stable internet connection

**Steps**:

1. Download SigmaOS ISO: `wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.0.0/sigmaos-15.0.0.iso`

2. Verify checksum: `sha256sum -c SigmaOS-SHA256SUMS`

3. Create bootable USB using Rufus or balenaEtcher

4. Boot from USB (press F12/F2 during startup)

5. Select "Dual Boot Installation" from menu

6. Follow on-screen instructions to partition disk

7. Install SigmaOS alongside Windows

8. Reboot and select SigmaOS from boot menu

**Risk Level**: Low (separate partitions)

### Linux Replacement Installation

**Prerequisites**:

- Existing Linux installation with backup

- 8GB USB drive

- Stable internet connection

**Steps**:

1. Backup important data

2. Download SigmaOS ISO: `wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.0.0/sigmaos-15.0.0.iso`

3. Verify checksum: `sha256sum -c SigmaOS-SHA256SUMS`

4. Create bootable USB: `sudo dd if=sigmaos-15.0.0.iso of=/dev/sdX bs=4M status=progress`

5. Boot from USB

6. Select "Full Disk Installation" from menu

7. Follow on-screen instructions to replace existing Linux

8. Reboot into SigmaOS

**Risk Level**: Medium (replaces existing Linux)

### Virtual Machine Installation

**Prerequisites**:

- VirtualBox, VMware, or QEMU

- At least 4GB RAM allocated

- 20GB disk space

**Steps**:

1. Download SigmaOS ISO or QCOW2 image

2. Verify checksum: `sha256sum -c SigmaOS-SHA256SUMS`

3. Create new VM with 4GB+ RAM, 20GB+ disk

4. Attach SigmaOS ISO to VM

5. Boot VM and select "Installation"

6. Follow on-screen instructions

7. Install guest tools for better performance

**QEMU Quick Start**:
```bash
qemu-system-x86_64 -cdrom sigmaos-15.0.0.iso -m 4G -smp 2 -enable-kvm
```

**Risk Level**: Zero (isolated environment)

## Verification

Every release artefact ships with a `.sig` file signed with Dilithium-5:

```bash

# Verify sigpkg

sigma-pkg verify sigmaos-15.0.0.sigpkg

# Verify raw images

sha256sum -c SigmaOS-SHA256SUMS

# Verify signature

sigma-pkg verify-sig sigmaos-15.0.0.sig sigmaos-15.0.0.iso
```

**Checksum Verification**:

- Download SHA256SUMS file from releases

- Compare with downloaded file

- Use provided verification script: `./verify-checksum.sh sigmaos-15.0.0.iso`

## Troubleshooting

### Installation Issues

**Boot fails after installation**:

- Check BIOS boot order (ensure SigmaOS is first)

- Try UEFI mode if using Legacy BIOS

- Disable Secure Boot temporarily

- Check disk partitioning with GParted

**USB not booting**:

- Re-create bootable USB with different tool

- Try USB 2.0 port instead of USB 3.0

- Verify ISO integrity with checksum

- Try different USB drive

**Graphics issues**:

- Boot with `nomodeset` parameter

- Try VESA driver: `sigmaos video=vesa`

- Update graphics drivers post-installation

- Check hardware compatibility matrix

### Post-Installation Issues

**Network not working**:

- Check driver compatibility matrix

- Install additional drivers: `sigma-pkg install network-drivers`

- Try USB tethering for initial setup

- Check network configuration: `sigma-net config`

**Audio not working**:

- Install audio drivers: `sigma-pkg install audio-drivers`

- Check audio settings: `sigma-audio config`

- Try different audio backend

- Check hardware compatibility matrix

**Performance issues**:

- Check system resources: `sigma-top`

- Disable unnecessary services: `sigma-service disable <service>`

- Update system: `sigma-pkg update`

- Check for driver updates

### Getting Help

- **Documentation**: [Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)

- **Issues**: [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

- **Community**: [Discord/Forum](#)

- **Email**: support@sigmaos.org

---

*See also: [INSTALL.md](INSTALL.md) · [Architecture.md](Architecture.md) · [BRANCH_GUIDE.md](BRANCH_GUIDE.md) · [Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)*
