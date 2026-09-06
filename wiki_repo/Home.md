# 🇸🇴 SigmaOS Sovereign Operating System

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/Version-v1.0.0--sovereign-blue.svg)](https://github.com/AaryanSinghChauhan09/SigmaOS/releases)

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
- **AI Agents Master Guide:** [[AI_AGENTS_GUIDE]] - Authoritative reference for autonomous coding agents and subagents.
- **AI Agents UX Management Guide:** [[AI_AGENTS_UX_MANAGEMENT_GUIDE]] - Interface, visual layout, and UX guidelines for autonomous AI agents.
- **AI Agents Time Management Guide:** [[AI_AGENTS_TIME_MANAGEMENT_GUIDE]] - Timekeeping primitives, clock sync, and temporal architecture for autonomous AI agents.
- **AI Agents Security Management Guide:** [[AI_AGENTS_SECURITY_MANAGEMENT_GUIDE]] - Capability sandboxing, PQC attestation, MAC, and digital forensics for autonomous AI agents.
- **AI Agents Procedure Call Management Guide:** [[AI_AGENTS_PROCEDURE_CALL_MANAGEMENT_GUIDE]] - Syscall dispatchers, FFI bindings, zero-copy IPC ring channels, and RPC for autonomous AI agents.
- **AI Agents Ballooning Management Guide:** [[AI_AGENTS_BALLOONING_MANAGEMENT_GUIDE]] - VirtIO memory ballooning, RAM inflation/deflation, and hypervisor overcommit management for AI agents.
- **AI Agents Boot Management Guide:** [[AI_AGENTS_BOOT_MANAGEMENT_GUIDE]] - UEFI/BIOS handoff, Multiboot2, Secure Boot verification, boot optimization, and init handoff for AI agents.
- **AI Agents Process Management Guide:** [[AI_AGENTS_PROCESS_MANAGEMENT_GUIDE]] - EEVDF/BORE scheduling algorithms, ELF loading, POSIX process control, and cgroups v2 job objects.
- **AI Agents Capability Tickets Guide:** [[AI_AGENTS_CAPABILITY_TICKETS_GUIDE]] - PQC Dilithium-5 capability tickets, POSIX bitmasks, Pledge promise tokens, and Capsicum descriptor rights.
- **AI Agents Circular Wait Management Guide:** [[AI_AGENTS_CIRCULAR_WAIT_MANAGEMENT_GUIDE]] - Coffman deadlock conditions, RAG cycle detection, lock hierarchy, and Banker's algorithm.

---

## 📊 Development Status & Performance Notes

| Component | Status | Notes |
|---|---|---|
| **Kernel Microkernel Core** | Beta ✅ | Working: scheduler, MMU, IPC stubs. TODO: real hardware drivers |
| **Memory Management** | Production ✅ | BuddyAllocator + SlabAllocator, W^X enforcement, NUMA support |
| **Security (pledge/unveil)** | Production ✅ | OpenBSD-compatible capability sandboxing with path traversal hardening |
| **Syscall Implementation** | Beta ✅ | 17+ syscalls integrated: file (open/read/write/close), process (fork/exec/wait/exit), network (socket/bind/connect/listen/send/recv), signal (rt_sigaction/kill). SyscallContext provides unified interface to all subsystems. |
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

## 📈 Recent Progress (September 2026)

### v0.6 Milestone: Consolidation Complete ✅

- [Home](wiki/Home.md)
- [Architecture](wiki/Architecture.md)
- [Linux Distros Architecture & Parity Guide](wiki/Linux-Distros-Architecture.md)
- [BSD Security Hardening Guide](wiki/BSD-Security-Hardening.md)
- [Declarative Package Management](wiki/Declarative-Package-Management.md)
- [Security Model](wiki/Security.md)
- [Driver Development](wiki/Driver-Development.md)
- [Installation Guide](wiki/Installation.md)
- [Roadmap](wiki/Roadmap.md)

---

## 📄 License

SigmaOS is licensed under the [MIT License](LICENSE).
