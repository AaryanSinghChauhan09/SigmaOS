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
=======
- 🔄 TCP/UDP stack - Partial
- ✅ Ext4 + FAT32 filesystems
- ✅ NVMe + USB xHCI drivers
- ✅ Zenith Desktop prototype
- ✅ sigma-pkg CLI
>>>>>>> origin/improve-package-manager-and-containers-15562379424742924660
- ⬜ Bootable ISO (Phase G)


---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### High-Impact Areas

- Round-robin scheduler implementation
- Buddy allocator completion
- sigma-sh REPL
- USB HID keyboard driver

---

## 📄 License

### Repository Documentation

- [Future Development & Distro-Parity Roadmap](FUTURE-DEVELOPMENT-ROADMAP.md) — Strategic roadmap detailing gaps & improvements vs mainstream Linux distros
- [Legacy Compatibility & Subsystem Parity Blueprint](LEGACY_COMPATIBILITY_BLUEPRINT.md) — Architectural design and implementation of legacy adapters, bridges, and workload optimizers
- [Documentation Audit](docs/doc_audit_backlog.md) — Implementation status
- [Roadmap](Roadmap.md) — Development plan
- [INSTALL.md](INSTALL.md) — Build instructions
- [CONTRIBUTING.md](CONTRIBUTING.md) — Contribution guidelines
- [SECURITY_POLICY.md](SECURITY_POLICY.md) — Security policy
- [SUPPORT.md](SUPPORT.md) — Support and troubleshooting
- [FAQ](FAQ.md) — Common questions (coming soon)


### GitHub Wiki (Canonical Documentation)

Detailed conceptual documentation is managed exclusively in the GitHub Wiki:

- **Master Roadmap**: [Maturity & Distro-Parity Roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Maturity_Parity_Roadmap)
- **Kernel Evolution**: [Kernel Evolution Architecture](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Kernel_Evolution_Architecture)
- **Driver Ecosystem**: [Driver Ecosystem](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Driver_Ecosystem)
- **Strategic Planning**: [Gap Filling Strategic Plan](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/GAP_FILLING_STRATEGIC_PLAN)
- **Advanced Core Architecture**: [Advanced Absorption Matrix](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Advanced_Absorption)
- **Filesystem Design**: [SigmaFS Innovations](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/SigmaFS_Innovations)
- **Interactive UI Compositor**: [SigmaMedia Frameworks](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/SigmaMedia_Frameworks)
- **Local AI Daemon**: [Sigma AI Agents](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Sigma_AI_Agents)
- **Linux Distro Absorption**: [Strategic Distro Absorption Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/LINUX_DISTRO_ABSORPTION_SPEC)
- **S-Boot Firmware**: [Sovereign BIOS & UEFI Firmware Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/BIOS_FIRMWARE_SPEC)
- **Zenith Compositor**: [Wayland Zenith UI Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/WAYLAND_ZENITH_SPEC)
- **Portable Apps**: [Portable Application Format Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/PORTABLE_APP_FORMAT_PLAN)
- **Custom Personalization**: [Custom Personalization & Theme Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/CUSTOM_PERSONALIZATION_SPEC)
- **Kernel Performance**: [Kernel Performance Optimization Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/KERNEL_PERFORMANCE_PLAN)
- **Zig Driver Integration**: [Zig Language Driver Integration Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/ZIG_INTEGRATION_PLAN)
- **Nim Driver Integration**: [Nim Language Driver Integration Specification](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/NIM_INTEGRATION_PLAN)


---

## 📄 License

Dual-licensed under MIT and GPL-2.0. See the `LICENSE` file for details.
