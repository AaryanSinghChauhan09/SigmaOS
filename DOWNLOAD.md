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

## Verification

Every release artefact ships with a `.sig` file signed with Dilithium-5:

```bash
sigma-pkg verify sigmaos-15.0.0.sigpkg   # verify a sigpkg
sha256sum -c SigmaOS-SHA256SUMS          # verify raw images
```

---

*See also: [INSTALL.md](INSTALL.md) · [Architecture.md](Architecture.md) · [BRANCH_GUIDE.md](BRANCH_GUIDE.md) · [Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)*
