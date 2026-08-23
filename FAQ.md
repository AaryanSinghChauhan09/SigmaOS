# ❓ Frequently Asked Questions — SigmaOS

---

## General Questions

### What is SigmaOS?

SigmaOS is a sovereign, bare-metal operating system built from scratch in Rust. It features a microkernel architecture, zero-dependency userland, post-quantum cryptography, and compatibility layers for major Linux distributions.

### Why Rust?

Rust provides memory safety guarantees without a garbage collector, making it ideal for OS development where memory management bugs are catastrophic. SigmaOS leverages Rust's `#![no_std]` support to build a truly zero-dependency kernel.

### What architectures are supported?

| Architecture | Status |
|-------------|--------|
| x86_64 | ✅ Primary |
| AArch64 (ARM64) | 🔄 In Progress |
| RISC-V 64 | 🔄 In Progress |

### Is SigmaOS production-ready?

SigmaOS is currently in active development (Phase G of the roadmap). The kernel core, security subsystem, and package manager are implemented. Full boot on real hardware is the Phase G goal.

---

## Technical Questions

### Does SigmaOS use systemd?

No. SigmaOS uses `SovereignInitSupervisor`, a purpose-built init system inspired by systemd, s6, runit, and OpenRC. It supports parallel service startup, socket activation, and dependency graph resolution.

### What filesystems does SigmaOS support?

- **SigmaFS**: Native CoW filesystem (inspired by Btrfs/ZFS)
- **ZFS-compatible**: Snapshot pools via `SigmaZFS`
- **LUKS2**: Transparent encryption via `SigmaEncrypt`
- **POSIX VFS**: ext4, xfs, fat32 (read-only compat)

### How does package management work?

SigmaOS uses `sigma-pkg`, a universal package manager supporting:
- Native SigmaPkg format (content-addressed)
- Arch Linux ALPM/pacman compatible
- Debian/Ubuntu APT/deb compatible  
- RPM (Fedora/RHEL) compatible
- Atomic rollback via `PackageSnapshotRollback`

### What security model does SigmaOS use?

SigmaOS implements a layered security model:
1. **MAC**: Mandatory Access Control (SELinux-compatible) via `SigmaMAC`
2. **DAC**: Discretionary Access Control with POSIX ACLs
3. **RBAC**: Role-Based Access Control via `SigmaRBAC`
4. **pledge/unveil**: OpenBSD-style syscall restriction
5. **Capabilities**: Linux-compatible capability bounding sets
6. **Post-quantum crypto**: Kyber-1024 + Dilithium-5

---

## Development Questions

### How can I contribute?

See the [Contributing](Contributing) guide. Quick steps:
1. Fork the repository
2. Create a feature branch
3. Submit a Pull Request targeting `main`
4. All PRs are reviewed and merged into `main` only

### Where do I report bugs?

Open an [Issue](https://github.com/AaryanSinghChauhan09/SigmaOS/issues) on GitHub with:
- Rust version (`rustc --version`)
- Host OS and architecture
- Steps to reproduce
- Error output

### What's on the roadmap?

| Phase | Status | Goal |
|-------|--------|------|
| F (Competitor Crusher) | ✅ Complete | Match feature parity with top Linux distros |
| G (Kernel Boot) | 🔄 Active | Boot on real x86_64 hardware |
| H (India Stack) | 📋 Planned | UPI, Aadhaar, DigiLocker integration |
| I (AI + Enterprise) | 📋 Planned | AI orchestration, K8s compat |

---

*Last updated: 2026-08-23 | [Back to Home](Home)*
