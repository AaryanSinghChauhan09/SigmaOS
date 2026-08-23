# 🚀 SigmaOS Operating System

[![Build Status](https://github.com/SigmaOS-Org/SigmaOS/actions/workflows/sigmaos-ci.yml/badge.svg)](https://github.com/SigmaOS-Org/SigmaOS/actions/workflows/sigmaos-ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)

SigmaOS is an advanced, sovereign, microkernel-based operating system built from scratch in Rust with a zero-dependency architecture. Designed for performance, security, and versatility, SigmaOS bridges modern microkernel security with bare-metal performance across `x86_64`, `aarch64`, and `riscv64` hardware platforms.

---

## ✨ Features & Subsystems

- **Sovereign Microkernel Core:** Memory isolation, capability bounding sets, and lightweight IPC channels (ALPC/Pipes).
- **Multi-Distro Compatibility & Parity:** Adapters and translation layers for Arch Linux (ALPM/Pacman), Debian/Ubuntu (APT/dpkg), Gentoo (Portage USE flags), Fedora (RPM/SELinux), CachyOS (BORE scheduler), and FreeBSD (Jails/Capsicum/GEOM).
- **Post-Quantum Cryptography:** Native Dilithium-5 and Kyber-1024 cryptographic verification for driver and package attestation.
- **Zero-Trust Access Control:** Integrated Discretionary (DAC), Mandatory (MAC / Bell-LaPadula), and Role-Based (RBAC) access controls.
- **Zenith Desktop & Sovereign Media Suite:** Built-in zero-dependency multimedia tools, video editor (SigmaCut), audio DSP, and responsive UI components.

---

## 🛠️ Building & Running Tests

To build and run the full native test harness:

```bash
./run_sigma_tests.sh
```

---

## 📚 Canonical Documentation (GitHub Wiki)

```text
Phase F (Competitor Crusher)   ████████████████████  100% ✅
Phase G (Kernel Boot)          ████████████░░░░░░░░   60% ← ACTIVE
Phase H (India Stack)          ░░░░░░░░░░░░░░░░░░░░    0% (blocked on G)
```

### Current Status

**Kernel Core:**
- ✅ Microkernel scheduler & IPC
- ✅ Physical & virtual memory manager
- ✅ Multi-core SMP support
- 🔄 x86_64 / AArch64 / RISC-V HAL
- 🔄 PCI / PCIe bus driver

**Security Subsystem:**
- ✅ Discretionary & Mandatory Access Control
- ✅ Post-quantum cryptographic attestation (Dilithium-5 / Kyber-1024)
- ✅ Capabilities & pledge sandboxing

**Userland & Applications:**
- ✅ Zenith Desktop frontend
- ✅ Sovereign Video & Audio Editor
- ✅ Disk usage analyzer
- ✅ System monitor
- ✅ Process manager
- 🔄 Virtual machine manager (QEMU/KVM)
- 🔄 Container manager (Docker/Podman)

**Package Management:**
- ✅ sigma-pkg CLI
- 🔄 Universal package manager
- 🔄 Rollback package snapshots

**Networking:**
- 🔄 Cloud sync engine
- 🔄 Built-in torrent client
- 🔄 Network traffic analyzer

**AI & Automation:**
- 🔄 AI orchestrator for system optimization

**Customization:**
- 🔄 Unified control center
- ✅ Declarative theming engine

**Boot & Deployment:**
- 🔄 TCP/UDP stack - Partial
- ✅ Ext4 + FAT32 filesystems
- ✅ NVMe + USB xHCI drivers
- ✅ Zenith Desktop prototype
- ✅ sigma-pkg CLI
- ⬜ Bootable ISO (Phase G)


---

## 📋 SigmaOS Components

See the **[Components Table](COMPONENTS-TABLE)** for a full list of all 102+ OS components with module paths, descriptions, and statuses.

---

## 🐧 Linux Distro Inspirations

SigmaOS implements ideas from 10+ Linux distributions. See **[Linux Distro Ideas & Inspirations](LINUX-DISTRO-IDEAS-AND-INSPIRATIONS)** for a complete breakdown.

---

## 📝 Branch & PR History

- **[Pull Requests and Merge History](Pull-Requests-and-Merge-History)** — Full history of all merged PRs
- **[Branch Consolidation Phase 6 (2026-08-23)](Branch-Consolidation-August-2026-Phase6)** — Latest consolidation report

---

## 🔗 Key Wiki Pages

| Category | Pages |
|----------|-------|
| **Architecture** | [Kernel Architecture](Kernel-Architecture), [Cross-Platform HAL](Cross-Platform-Kernel-HAL), [Zero-Dependency Subsystems](Zero-Dependency-Subsystems) |
| **Security** | [Security Architecture](Security-Architecture), [Post-Quantum Crypto](Post-Quantum-Cryptography-and-TLS-1.3), [Sovereign Cryptography](Sovereign-Cryptography-Guide) |
| **Package Mgmt** | [Package Manager](Package-Manager), [Arch Linux Parity](Arch-Linux-and-AUR-Parity), [Portage/Gentoo](Portage-and-Gentoo-Clean-Room) |
| **Distro Parity** | [AntiX Linux](antiX-Linux-Compatibility-Parity), [BSD Inspirations](BSD-Inspirations-and-Parity), [CachyOS BORE](CachyOS-BORE-Scheduler-Architecture) |
| **Performance** | [Performance Guide](PERFORMANCE_OPTIMIZATION_GUIDE), [Resource Management](Resource-Management-and-Cgroups), [Hardware GPU](Hardware-Acceleration-and-GPU) |
| **Development** | [Driver Guide](Driver-Development-Guide), [KMS/USB Architecture](KMS-and-USB-HCD-Driver-Architecture) |
| **Repository** | [Components Table](COMPONENTS-TABLE), [PR History](Pull-Requests-and-Merge-History), [Linux Ideas](LINUX-DISTRO-IDEAS-AND-INSPIRATIONS) |

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### High-Impact Areas

- Round-robin scheduler implementation
- Buddy allocator completion
- sigma-sh REPL
- USB HID keyboard driver
- Bootable ISO image (Phase G)
- Container manager (Docker/Podman parity)

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Last updated: 2026-08-23 | Branch consolidation phase 6 complete — only `main` branch remains*
