# 🛡️ SigmaOS — Sovereign, AI-Native Operating System

[![Build Status](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/sigma-build.yml/badge.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Language: Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Platform: x86_64 | ARM64 | RISC-V](https://img.shields.io/badge/platform-x86__64%20%7C%20ARM64%20%7C%20RISC--V-blue.svg)](#)
[![Security: Post-Quantum](https://img.shields.io/badge/security-Post--Quantum-critical.svg)](#)

> **"Sovereignty is the ultimate efficiency."**
> The world's first industrial-grade microkernel designed for total digital autonomy, post-quantum resilience, and Indian industrial compliance.

---

## 🎯 Overview

SigmaOS is a sovereign, zero-dependency, AI-native operating system built entirely in Rust. It discards legacy POSIX assumptions to build a hyper-secure, capability-based microkernel designed for an AI-first, object-oriented ecosystem.

SigmaOS draws inspiration from the best Linux distributions — Arch, NixOS, Debian, Fedora, Alpine, Gentoo, Void, openSUSE, QubesOS, and Parrot OS — while implementing every feature natively in Rust with `no_std` philosophy at its core.

### Core Pillars

| Pillar | Description |
|--------|-------------|
| 🔐 **Post-Quantum Cryptography** | Native Kyber-1024 KEM + Dilithium-5 signatures (NIST FIPS 203/204) |
| 🏛️ **Capability-Based Security** | 64-bit hardware-enforced permission model replacing legacy ACLs |
| ⚡ **Shard Architecture** | 600+ hot-swappable kernel modules with zero-latency IPC |
| 🤖 **AI-Native Design** | Local LLM inference as a first-class OS primitive |
| 🇮🇳 **India-First** | Native GST, Income Tax, UPI, and 22-language support |
| 🔄 **Atomic Updates** | NixOS-inspired generation management with instant rollback |
| 📦 **Universal Packages** | sigpkg handles .deb, .rpm, .pkg.tar.zst, XBPS, and native sigma formats |

---

## 📊 System Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         User Applications                                   │
│          (Zenith Desktop · Sigma Shell · sigpkg · AI Agents)                │
├──────────────────────────────┬──────────────────────────────────────────────┤
│     Userland Services        │          Userland Tools                      │
│  (init · daemons · D-Bus)    │  (coreutils · devtools · installer)          │
├──────────────────────────────┴──────────────────────────────────────────────┤
│                    Syscall Capability Gate (S-SEC)                          │
├────────────┬─────────────┬──────────────┬───────────────┬───────────────────┤
│  S-MM      │  S-SCHED    │   S-FS       │   S-NET       │   S-AI            │
│  Memory    │  Scheduler  │  Filesystem  │  Network      │  LLM Orchestrator │
│  (Buddy)   │ (MLFQ+CFS)  │ (VFS+SigFS) │ (TCP/IP+TLS)  │  (Local Inference)│
├────────────┴─────────────┴──────────────┴───────────────┴───────────────────┤
│                     Sovereign IPC Bus (Zero-Copy)                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                     Hardware Abstraction Layer (HAL)                        │
├─────────────┬───────────────┬──────────────────────────────────────────────┤
│   x86_64    │    ARM64      │              RISC-V                           │
└─────────────┴───────────────┴──────────────────────────────────────────────┘
```

### Kernel Shards (Modules)

| Shard | Responsibility | Inspired By |
|-------|---------------|-------------|
| **S-MM** | Memory management, buddy allocator, virtual memory | Multicore Linux MM |
| **S-SCHED** | MLFQ + CFS + EDF predictive scheduler | Linux CFS, BORE, FreeBSD ULE |
| **S-FS** | VFS + SigmaFS distributed filesystem | ext4, Btrfs, ZFS |
| **S-NET** | TCP/IP stack, TLS 1.3, WireGuard VPN | Linux netfilter |
| **S-SEC** | Capability system, MAC, PQC sandbox | QubesOS, SELinux, Capsicum |
| **S-AI** | Local LLM inference, AI task routing | First-class OS primitive |
| **S-IPC** | Zero-copy message passing, shared memory | L4 microkernel |

---

## 🚀 Quick Start

### Prerequisites

```bash
# Ubuntu/Debian
sudo apt install -y build-essential nasm cmake qemu-system-x86 golang-go xorriso rustup

# Install Rust nightly with bare-metal target
rustup install nightly
rustup target add x86_64-unknown-none --toolchain nightly
```

### Build & Run

```bash
# Clone
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build the system image
make clean && make all -j$(nproc)

# Run in QEMU (2 GB RAM, serial output)
qemu-system-x86_64 -cdrom build/sigmaos.iso -m 2G -serial stdio
```

### Build Profiles

SigmaOS supports declarative compilation profiles:

```bash
make PROFILE=standalone all    # Full desktop ISO (default)
make PROFILE=rtos all          # Hard real-time ELF binary
make PROFILE=cloud all         # Headless cloud image
make PROFILE=browser all       # WASM bundle for browser testing
make PROFILE=embedded all      # Minimal embedded target
```

### Package Management (sigpkg)

```bash
# Install a package
sigma install firefox

# Update all packages (atomic, with snapshot)
sigma upgrade

# Roll back to previous generation
sigma rollback

# Search community packages (AUR-inspired)
sigma search sigma-community <name>

# Install from source with build flags (Gentoo-inspired)
sigma install --use=+crypto,-gui vim
```

---

## 🔒 Security

SigmaOS features a capability-native access control system. Programs run with explicit privilege tokens rather than generic user IDs:

```rust
// Capability delegation example
let token = CapabilityToken::new()
    .allow_network("tcp", 80)
    .allow_read("/var/www")
    .expires_in(Duration::hours(1));

process.spawn_with_capability(token);
```

Key security features:
- **Post-Quantum Cryptography**: Kyber-1024 + Dilithium-5 (NIST FIPS 203/204)
- **Capability-Based Access**: No ambient authority, explicit delegation
- **Qubes-Inspired Isolation**: Compartmentalized security domains
- **Verified Boot**: TPM 2.0 + measured boot chain
- **Atomic Updates**: Btrfs snapshots before every upgrade
- **WireGuard VPN**: Built-in, always-on option

See [docs/SECURITY.md](docs/SECURITY.md) for the full threat model and security policy.

---

## 📦 Linux Distro Inspirations

SigmaOS absorbs the best ideas from the Linux ecosystem:

| Distro | What We Adopted |
|--------|----------------|
| **Arch Linux** | Rolling releases, AUR-like community repo, pacman-inspired sigpkg |
| **NixOS** | Atomic generations, reproducible builds, declarative `sigma.toml` |
| **Debian** | Stability tiers, package pinning, LTS support model |
| **Fedora** | SELinux-inspired MAC, DNF-like dependency resolution |
| **Alpine** | musl libc, minimal footprint, busybox-style multi-call binary |
| **Gentoo** | Source-based builds, USE flags → SigmaOS BuildFlags |
| **Void Linux** | runit-inspired init, XBPS-compatible package model |
| **openSUSE** | YaST-inspired config UI, Btrfs snapshot management |
| **QubesOS** | Compartmentalized domains, disposable VMs |
| **Parrot OS** | Security tools, forensics capabilities |

See [docs/LINUX_DISTRO_INSPIRATIONS.md](docs/LINUX_DISTRO_INSPIRATIONS.md) for full details.

---

## 🗺️ Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| **Phase 1** | 🔄 Active | Core kernel stability, driver framework, IPC |
| **Phase 2** | 📅 Planned | Zenith desktop environment, Wayland compositor |
| **Phase 3** | 📅 Planned | Application ecosystem, Flatpak/container support |
| **Phase 4** | 🔭 Future | AI-native features, on-device LLM integration |
| **Phase 5** | 🔭 Future | Sovereign cloud infrastructure |

See [docs/ROADMAP.md](docs/ROADMAP.md) for the detailed roadmap.

---

## 📁 Project Structure

```
SigmaOS/
├── src/                    # Rust source (600+ modules)
│   ├── kernel/             # Core kernel shards
│   ├── security/           # Capability system, PQC
│   ├── sigpkg/             # Package manager
│   ├── shell/              # Sigma shell & REPL
│   ├── desktop/            # Zenith desktop
│   ├── distro/             # Distro-inspired improvements
│   ├── ai/                 # AI/LLM orchestration
│   └── ...
├── kernel/                 # C++ kernel components (HAL, drivers)
├── userland/               # Userland binaries (shell, init, pkg)
├── docs/                   # Documentation
├── drivers/                # Hardware drivers
├── tests/                  # Test suites
├── scripts/                # Build & CI scripts
└── iso_root/               # ISO filesystem root
```

---

## 🤝 Contributing

We welcome contributions! Please read [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) before submitting a PR.

- **Code**: Rust (`no_std` preferred), C++ for legacy HAL components
- **Tests**: Required for all new kernel functionality
- **Security**: See [docs/SECURITY.md](docs/SECURITY.md) for vulnerability disclosure

---

## 📜 License

SigmaOS is licensed under the [MIT License](LICENSE).

Copyright © 2024–2026 Aaryan Singh Chauhan and the SigmaOS contributors.

---

## 📞 Contact & Community

- **GitHub Issues**: [Bug reports & feature requests](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)
- **Wiki**: [SigmaOS Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)
- **Security**: See [docs/SECURITY.md](docs/SECURITY.md) for responsible disclosure
Dual-licensed under MIT and GPL-2.0. See the `LICENSE` file for details.
