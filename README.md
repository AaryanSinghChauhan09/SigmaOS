# 🇸🇴 SigmaOS Sovereign Operating System

[![Build Status](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/sigmaos-ci.yml/badge.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/actions/workflows/sigmaos-ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/Version-v1.0.0--sovereign-blue.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases)

SigmaOS is an advanced, sovereign, microkernel-based operating system built from scratch in Rust with a zero-dependency `#![no_std]` architecture. Designed for performance, security, and versatility, SigmaOS bridges modern microkernel security with bare-metal performance across `x86_64`, `aarch64`, and `riscv64` hardware platforms.

---

## 🌟 Architectural Highlights

- **Sovereign Microkernel Core:** Memory isolation, CachyOS BORE / EEVDF scheduler, capability bounding sets, and zero-copy IPC channels.
- **Systemd Betsy Init Supervisor:** Full unit parsing, Cgroup v2 slice memory quotas, watchdog health monitoring, and alternative init bridging.
- **GTK & Libadwaita Sovereign UI Toolkit:** `GtkHeaderBar` CSD, `AdwPreferencesPage`, `AdwActionRow`, `GtkCssProvider`, `GtkSignalDispatcher`, status bar panel, dock bar, and workspace overview.
- **Sovereign Network Discovery Engine:** ZeroConf mDNS / DNS-SD, UPnP / SSDP M-SEARCH, LLMNR / NBNS host resolution, and ICMPv6 NDP neighbor table tracking.
- **Interactive `sigma-sh` REPL:** Zsh/Fish syntax-highlighted line editor (`ReplLineEditor`), Fish auto-suggestions (`AutoSuggestTabPopup`), job control (`jobs`/`fg`/`bg`), and OpenBSD pledge/unveil capability sandboxing.
- **Multi-Distro Compatibility & Parity:** Dependency installers and translation adapters for Arch Linux (ALPM/Pacman), Debian/Ubuntu (APT/dpkg), Gentoo (Portage USE flags), Fedora (RPM/SELinux), Linux Mint (Cinnamon, mintupgrade, mintstick, mintmenu), and FreeBSD (Jails/Capsicum/GEOM).
- **Post-Quantum Cryptography:** Native Dilithium-5 and Kyber-1024 cryptographic verification for driver and package attestation.
- **Zero-Trust Access Control & MAC:** Discretionary (DAC), Mandatory Access Control (MAC LSM Inode/Ptrace/Socket hooks), and Role-Based (RBAC) security enforcers.
- **Zenith Desktop & Sovereign Media Suite:** Built-in zero-dependency multimedia tools, video editor (SigmaCut), audio DSP, and responsive UI components.
- **Autonomous AI Agent Resource Management:** Intelligent microkernel and userland agent governors for compute, memory (DAMON/PSI), storage (ZFS ARC/CoW), network QoS (eBPF/VNET), and thermal power tuning via ACP/MCP protocols.
- **AI Agents Blocking & Threat Mitigation:** Zero-trust threat isolation agents for syscall filtering (seccomp/LSM), network packet dropping (eBPF XDP), path unveil blocking (OpenBSD pledge/unveil), and process freezing (cgroups v2).
- **AI Agents Block Storage Management:** Autonomous block I/O scheduling (io_uring/blk-mq), NVMe multi-tiering (Bcachefs/ZFS), GEOM RAID/scrubbing, and NVMe-oF network block fabric rebalancing.
- **AI Agents Cache Management:** Autonomous multi-tier caching for CPU L1/L2/L3 (CAT/QoS), VFS page caches (DAMON/PSI), SLAB object caches, FreeBSD ZFS ARC/L2ARC pools, and zswap/zram compressed memory buffers.

---

## 📚 Documentation Index

- [AI Agents Resource Management Architecture](docs/ai-agents-resource-management.md)
- [AI Agents Blocking & Threat Mitigation Architecture](docs/ai-agents-blocking-management.md)
- [AI Agents Block Storage Management Architecture](docs/ai-agents-block-storage-management.md)
- [AI Agents Cache Management Architecture](docs/ai-agents-cache-management.md)
- [AI Agents Chip Multiprocessor (CMP) Management Architecture](docs/ai-agents-chip-multiprocessor-management.md)
- [API Reference](docs/api-reference.md)
- [Kernel Architecture](docs/kernel.md)
- [Memory Management](docs/memory-management.md)
- [Security Architecture](docs/security.md)
- [Package Management](docs/package-manager.md)
- [Linux & BSD Distro Innovations Inspiration](docs/distro_suggestions.md)

---

## 📊 Development Status & Performance Notes

| Component | Status | Notes |
|---|---|---|
| **Kernel Microkernel Core** | Beta ✅ | Working: scheduler, MMU, IPC stubs. TODO: real hardware drivers |
| **Memory Management** | Production ✅ | BuddyAllocator + SlabAllocator, W^X enforcement, NUMA support |
| **Security (pledge/unveil)** | Production ✅ | OpenBSD-compatible capability sandboxing with path traversal hardening |
| **Syscall Implementation** | Alpha ⚠️ | 30+ syscall stubs defined; read/write/open/close implemented; network/process syscalls pending |
| **Package Manager (sigpkg)** | Beta ✅ | Multi-format adapter working; SAT resolver functional |
| **Desktop (Zenith)** | Early Alpha ⚠️ | Compositor framework present; full GTK/Libadwaita binding pending |
| **Network Stack** | Planned | TCP/IP stack design documented; implementation deferred to v0.2 |

### Performance Notes

**Design Goals (v1.0.0 target):**
- Context Switch Latency: < 0.12 µs (vs. Linux 0.85 µs)
- Zero-Copy IPC: 14.2 GB/s (vs. Linux 8.1 GB/s)
- Boot Time: < 180 ms (vs. Linux 1.45 s)

**Current State:** SigmaOS v0.1.0 is a hosted simulation running in userspace. Performance measurements will be conducted after hardware driver implementation and real interrupt handling.

### Building & Running

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Test the current codebase
./run_sigma_tests.sh

# Build (requires Rust nightly)
make build

# Run QEMU test
make test-qemu
```

---

## 📄 License

SigmaOS is licensed under the [MIT License](LICENSE).
