# Welcome to SigmaOS

**SigmaOS** is a sovereign, zero-dependency operating system written in Rust, designed to defeat Linux and BSD distros through superior architecture, security, and performance.

## 🚀 Core Features

- **Zero-Dependency Kernel (`#![no_std]`)**: No external third-party crates, pure Rust `alloc::` primitives for sovereign operations.
- **Multi-Architecture Support**: x86_32, x86_64, aarch64, riscv64, loongarch64, powerpc64, s390x.
- **Universal Package Management**: Native Sigma-pkg with cross-distro adapters (.deb, .rpm, PKGBUILD, ebuild, apk, snap, flatpak, hpkg).
- **OpenBSD pledge/unveil Security**: Capability-based sandboxing and path unveil for process isolation.
- **FreeBSD Jails & ZFS BootEnv**: Container-level isolation and boot environment management.
- **Illumos Zones & DTrace**: Solaris-inspired containerization and dynamic tracing framework.
- **NixOS Content-Addressed Store**: Hermetic package storage with atomic garbage collection.
- **Linux io_uring Parity**: Asynchronous I/O engine for high-performance networking and storage.
- **Zero-Trust Access Control & MAC:** Discretionary (DAC), Mandatory Access Control (MAC LSM Inode/Ptrace/Socket hooks), and Role-Based (RBAC) security enforcers.
- **Zenith Desktop & Sovereign Media Suite:** Built-in zero-dependency multimedia tools, video editor (SigmaCut), audio DSP, and responsive UI components.
- **AI Agents Master Guide:** [[AI_AGENTS_GUIDE]] - Authoritative reference for autonomous coding agents and subagents.
- **AI Agents UX Management Guide:** [[AI_AGENTS_UX_MANAGEMENT_GUIDE]] - Interface, visual layout, and UX guidelines for autonomous AI agents.
- **AI Agents Time Management Guide:** [[AI_AGENTS_TIME_MANAGEMENT_GUIDE]] - Timekeeping primitives, clock sync, and temporal architecture for autonomous AI agents.
- **AI Agents Security Management Guide:** [[AI_AGENTS_SECURITY_MANAGEMENT_GUIDE]] - Capability sandboxing, PQC attestation, MAC, and digital forensics for autonomous AI agents.
- **AI Agents Procedure Call Management Guide:** [[AI_AGENTS_PROCEDURE_CALL_MANAGEMENT_GUIDE]] - Syscall dispatchers, FFI bindings, zero-copy IPC ring channels, and RPC for autonomous AI agents.
- **AI Agents Ballooning Management Guide:** [[AI_AGENTS_BALLOONING_MANAGEMENT_GUIDE]] - VirtIO memory ballooning, RAM inflation/deflation, and hypervisor overcommit management for AI agents.
- **AI Agents Boot Management Guide:** [[AI_AGENTS_BOOT_MANAGEMENT_GUIDE]] - UEFI/BIOS handoff, Multiboot2, Secure Boot verification, boot optimization, and init handoff for AI agents.

---

## 📚 Documentation

Get started with SigmaOS through our comprehensive wiki:

- **[Quick Start](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Quick-Start)** - Build and run SigmaOS
- **[Architecture](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture)** - Core design and subsystems
- **[Tier 1 Features](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Tier-1-Features)** - Feature matrix and status
- **[Syscall Reference](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Syscall-Reference)** - Complete syscall documentation
- **[Contributing](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Contributing)** - Development guidelines
- **[Roadmap](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Roadmap)** - Phases 6-10 plans
- **[Release Notes](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Release-Notes)** - Version history
- **[FAQ](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/FAQ)** - Common questions
- **[API Documentation](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/API-Documentation)** - Public APIs
- **[Full Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)** - Complete documentation index

---

## 📊 Development Status & Performance Notes

- **Zero-Allocation Hot Paths**: Sub-microsecond syscalls and interrupt handling via `#![no_std]` core.
- **Cross-Subsystem Bridge**: `SovereignUniversalDistroBridge` in `src/distro/linux_bsd_inspirations.rs` integrates VFS, Init, Package Management, Security, Kernel, and Memory subsystems.
- **Multi-Distro Parity**: Comprehensive adapters for Arch Linux (pacman/AUR), Fedora (dnf/rpm-ostree), Debian (apt), Gentoo (portage), Alpine (apk), Void (xbps), FreeBSD (pkg), OpenBSD (pkg_add), and NixOS (nix).

---

## 🛡️ Security Architecture

- **Post-Quantum Cryptography**: Dilithium-5 module signatures and Kyber-712 key exchange.
- **LSM (Linux Security Modules)**: Inode, Ptrace, and Socket hooks for mandatory access control.
- **OpenBSD pledge/unveil**: Capability-based process sandboxing and filesystem path unveiling.
- **FreeBSD Capsicum**: Capability mode for fine-grained privilege separation.
- **eBPF XDP Zero-Copy**: Express BPF with XDP zero-copy socket redirection for high-performance networking.

---

## 🎯 Design Principles

SigmaOS follows strict software engineering principles:

- **OOPS**: Objects, Classes, Instances, Encapsulation, Abstraction, Inheritance, Polymorphism
- **SOLID**: Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, Dependency Inversion
- **DRY**: Don't Repeat Yourself
- **KISS**: Keep It Simple, Stupid
- **YAGNI**: You Aren't Gonna Need It
- **Separation of Concerns**
- **Composition Over Inheritance**
- **Design by Contract**

---

## 🔗 Quick Links

- **[Main Repository](https://github.com/AaryanSinghChauhan09/SigmaOS)** - Source code and issues
- **[Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki)** - Full documentation
- **[AGENTS.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/AGENTS.md)** - AI Agent operational guidelines
- **[LICENSE](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/LICENSE)** - MIT License

---

*Built with sovereign zero-dependency philosophy to defeat Linux and BSD distros through superior architecture.*