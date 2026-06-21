# 🔐 SigmaOS — Sovereign Operating System

<div align="center">

**Zero-Dependency · Zero-Compromise · Sovereign Silicon**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/AaryanSinghChauhan09/SigmaOS/actions)
[![Architecture](https://img.shields.io/badge/arch-x86__64%20%7C%20ARM64-orange)](docs/)
[![Phase](https://img.shields.io/badge/phase-F%20(Active)-purple)](CURRENT_PROBLEMS_MANIFEST.md)

</div>

---

## What is SigmaOS?

SigmaOS is a **from-scratch operating system** built on three principles:

| Principle | Meaning |
|-----------|---------|
| **Silicon Sovereignty** | No dependency on external runtimes, libc, or POSIX ABIs |
| **Zero-Trust by Default** | Every process, driver, and subsystem runs in an isolated capability ring |
| **Reproducible Determinism** | NixOS-inspired declarative configs; identical build = identical system |

---

## Quick Start

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build a bootable ISO (standalone desktop profile)
make PROFILE=standalone iso

# Boot in QEMU for testing
./scripts/qemu-boot.sh standalone

# Build for other targets
make PROFILE=browser-wasm     # WebAssembly / browser
make PROFILE=cloud-native     # Container / cloud
make PROFILE=vm-image         # Virtual machine image
make PROFILE=container-docker # Docker container
make PROFILE=iot-arm64        # ARM64 embedded
make PROFILE=serverless       # Serverless function runtime
```

---

## Architecture Overview

```
SigmaOS
├── kernel/                # Sovereign microkernel
│   ├── core/              # Process, scheduler, namespace, syscall gate
│   ├── memory/            # SovereignVMM — 4-level page tables, CoW, demand paging
│   ├── security/          # Zero-Trust engine, PAM/ACL, PQC integration
│   └── drivers/           # Driver registry & hot-reload
├── drivers/               # Bare-metal hardware drivers
│   ├── storage/           # NVMe 1.4, AHCI/SATA
│   ├── usb/               # xHCI USB 3.x host controller
│   ├── graphics/          # KMS/DRM for AMD/Intel
│   └── net/               # E1000, RTL8139 NICs
├── net/                   # TCP/IP stack — RFC 793 compliant
├── fs/                    # Ext4 (JBD2 journal), VFS abstraction
├── crypto/                # Kyber-1024 KEM, Dilithium-5 signatures
├── zenith_desktop/        # Zenith compositor & desktop environment
│   ├── compositor/        # Native C++ Wayland-inspired compositor
│   ├── settings/          # Declarative control center (NixOS-style)
│   └── neural/            # AVX-512 accelerated neural UI
├── userland/              # Shell, package manager, tools
│   ├── shell/             # sigma-sh — full pipeline, tab completion, scripting
│   └── pkg/               # sigma-pkg — reproducible .spkg package manager
├── runtime/               # WASM/WASI runtime, Linux ELF compat layer
├── hal/                   # Hardware Abstraction Layer (PCIe MSI-X, ACPI)
└── wiki_repo/             # GitHub Wiki source
```

---

## Core Subsystems

### 🛡️ Security — Post-Quantum, Zero-Trust
- **Kyber-1024** key encapsulation + **Dilithium-5** signatures (NIST PQC Level 5)
- **Zero-Trust PAM/ACL** — every VFS access verified against capability rings
- **Immutable Audit Trail** — CRC32C-checksummed, append-only kernel log
- **Adaptive Zero-Trust Engine** — runtime threat scoring with automatic isolation

### 💾 Storage — Full Ext4 with JBD2
- Complete Ext4 journal (descriptor/commit/revoke blocks, CRC32C, crash replay)
- NVMe 1.4 driver with admin/IO queue pairs and scatter-gather DMA
- VFS block-layer abstraction — swap NVMe ↔ SATA without driver changes

### 🌐 Networking — RFC-compliant TCP/IP
- TCP state machine: LISTEN → SYN_RECV → ESTABLISHED → TIME_WAIT
- Congestion control: slow-start + additive-increase/multiplicative-decrease
- ARP resolution, IPv4 routing, E1000/RTL8139 NIC drivers

### 🖥️ Zenith Desktop
- Native C++ compositor (no X11, no Wayland dependency)
- AVX-512 accelerated Neural UI rendering
- Declarative settings exported as JSON (NixOS-style replication)
- Tiling window manager with configurable gap sizes

### 📦 Package Management — sigma-pkg
- Reproducible `.spkg` format with content-addressed storage
- Community recipe pipeline for third-party packages
- Zero-dependency package resolution (no Python/Ruby tooling)

---

## Hardware Support

| Component | Status | Driver |
|-----------|--------|--------|
| NVMe Storage | ✅ Full | `drivers/storage/sigma_nvme.cpp` |
| USB 3.x (xHCI) | ✅ Full | `drivers/usb/sigma_xhci.cpp` |
| AMD/Intel KMS/GPU | ✅ Stub | `drivers/graphics/sigma_kms.cpp` |
| E1000 NIC | ✅ Full | `net/nic/SovereignE1000.cpp` |
| ACPI Power Mgmt | ✅ Full | `kernel/power/sigma_power_manager.cpp` |
| Wi-Fi / Bluetooth | ⚠️ Planned | Protocol stack in progress |
| Audio (HDA) | ⚠️ Planned | Codec enumeration planned |

---

## Profiles & Spins

| Profile | Target | Description |
|---------|--------|-------------|
| `standalone` | Desktop x86_64 | Default balanced profile |
| `forensic` | Security | CAINE-style read-only imaging |
| `enterprise` | Server | Hardened audit + strict ACL |
| `iot-arm64` | Embedded | Raspberry Pi / ARM64 minimal |
| `education` | Lab | Permissive sandbox for learning |
| `gaming` | Desktop | High-perf GPU + audio tuning |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide.

```bash
# Fork, clone, create a feature branch
git checkout -b feat/your-feature

# Run the linter / build check
make check

# Submit a PR against main
```

**Good first issues:** Look for the `good-first-issue` label in [Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues).

---

## Roadmap

See [ROADMAP.md](ROADMAP.md) for the full milestone timeline.

| Milestone | Target | Status |
|-----------|--------|--------|
| Phase E: Core subsystems | Q2 2026 | ✅ Done |
| Phase F: Type hardening | Q2 2026 | ✅ Done |
| Phase G: Wi-Fi/BT stacks | Q3 2026 | 🔄 Active |
| Phase H: Recovery GUI | Q3 2026 | 📋 Planned |
| Phase I: First ISO release | Q4 2026 | 📋 Planned |

---

## License

MIT © 2026 Aaryan Singh Chauhan
