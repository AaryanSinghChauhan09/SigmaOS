# 🌐 SigmaOS — Sovereign Operating System

> **A sovereign, post-quantum resilient, zero-dependency operating system written in Rust, combining microkernel security with bare-metal performance.**

---

## 🌟 What is SigmaOS?

SigmaOS is a from-scratch operating system designed for the next era of computing. It transcends POSIX legacy limitations and monolithic kernel bloat by drawing the best innovations from every major Linux and BSD distribution into a unified, sovereign platform.

**Key properties:**
- 🦀 **100% Rust** kernel and userspace (zero unsafe without justification)
- 🔒 **Post-quantum secure** — CRYSTALS-Kyber key encapsulation by default
- 📦 **Zero external dependencies** — everything built from scratch
- ⚡ **Bare-metal performance** — custom allocators, no runtime overhead
- 🔄 **Atomic upgrades** — NixOS-inspired generations, instant rollback
- 🛡️ **Capability-based security** — pledge/unveil/Capsicum enforced kernel-wide

---

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Build
cargo build --release

# Run tests
cargo test

# Run in QEMU
python3 scripts/qemu_smoke_test.py

# Build bootable ISO
bash scripts/build-iso.sh
```

---

## 📚 Wiki Navigation

### Getting Started
| Page | Description |
|------|-------------|
| [Installation](Installation) | Build from source, QEMU, real hardware |
| [FAQ](FAQ) | Frequently asked questions |
| [Contributing](Contributing) | How to contribute to SigmaOS |

### Architecture & Internals
| Page | Description |
|------|-------------|
| [Architecture](Architecture) | System architecture overview |
| [Kernel Development](Kernel-Development) | Kernel internals and APIs |
| [Package Manager](Package-Manager) | sigpkg documentation |
| [Security](Security) | Security model and features |
| [Build System](Build-System) | Build system documentation |

### Compatibility & Features
| Page | Description |
|------|-------------|
| [Linux & BSD Compatibility](Linux-BSD-Compatibility) | Distro parity documentation |
| [Distro Inspirations](Distro-Inspirations) | Features from Linux distros |
| [Feature Matrix](Feature-Matrix) | Complete feature comparison |

### Development
| Page | Description |
|------|-------------|
| [Roadmap](Roadmap) | Development roadmap |
| [Changelog](Changelog) | Version history |
| [Testing](Testing) | Test suite documentation |
| [API Reference](API-Reference) | Key API documentation |

---

## 🏗️ Architecture Highlights

```
┌─────────────────────────────────────────────────┐
│               USERSPACE SHARDS                   │
│  SigmaShell  ZenithDE  SigmaWeb  Applications   │
├─────────────────────────────────────────────────┤
│            SYSCALL INTERFACE (Sigma + POSIX)     │
├─────────────────────────────────────────────────┤
│                  KERNEL CORE                     │
│  Scheduler · VFS · Network · IPC · Drivers      │
│  BuddyAllocator · SlabAllocator · Paging        │
├─────────────────────────────────────────────────┤
│              KERNEL LIBRARY (klib)               │
│  HashMap · Vec · String · BTreeMap · Async      │
├─────────────────────────────────────────────────┤
│             SECURITY SUBSYSTEM                   │
│  pledge · unveil · Capsicum · SELinux · PQC     │
├─────────────────────────────────────────────────┤
│           HARDWARE ABSTRACTION (HAL)             │
│      x86_64 · aarch64 · riscv64                 │
└─────────────────────────────────────────────────┘
```

---

## 🐧 Distro Parity Matrix

| Distribution | Key Feature Imported | Status |
|-------------|---------------------|--------|
| Arch Linux | Rolling release, AUR, PKGBUILD, pacman | ✅ |
| NixOS | Declarative config, atomic upgrades, generations | ✅ |
| Gentoo | USE flags, Portage resolver, ebuilds | ✅ |
| Fedora | Cockpit, PipeWire, FreeIPA, Anitya | ✅ |
| CachyOS | BORE scheduler, LLVM PGO/BOLT, x86-64-v3 | ✅ |
| Alpine | musl parity, apk format, minimal footprint | ✅ |
| Debian | apt-compat, dpkg, stable release model | ✅ |
| FreeBSD | Capsicum, Jails, PF, ZFS | ✅ |
| OpenBSD | pledge, unveil, KARL, W^X | ✅ |
| DragonFly BSD | HAMMER2 B-tree filesystem | ✅ |
| Garuda Linux | Zen performance, ZRAM | ✅ |
| openSUSE | Snapper snapshots, zypper compat | ✅ |

---

## 🔒 Security Features

| Layer | Feature | Standard |
|-------|---------|----------|
| Process | pledge() + unveil() | OpenBSD |
| Process | Capsicum capability mode | FreeBSD |
| Kernel | pledge/Capsicum + Jails | Combined |
| MAC | SELinux type enforcement | NSA/Red Hat |
| Memory | W^X, KASLR, KARL, Retguard | OpenBSD/Linux |
| Crypto | CRYSTALS-Kyber (post-quantum) | NIST PQC |
| Network | WireGuard, TLS 1.3 only | RFC 8446 |
| Build | Reproducible, hermetic, signed | Debian/Nix |

---

## 📦 Package Manager Quick Reference

```bash
# Install
sigpkg install nginx

# Install from any format
sigpkg install package.pkg.tar.zst   # Arch
sigpkg install package.deb          # Debian
sigpkg install package.rpm          # Fedora
sigpkg install package.apk          # Alpine

# Upgrade all
sigpkg upgrade

# Rollback
sigpkg rollback

# Install from AUR
sigpkg aur install brave-bin
```

---

## 🧪 Test Status

Run the full test suite:

```bash
cargo test --all 2>&1 | tail -20
```

Key test suites:
- `algorithm_and_components_inspection_tests` — core algorithms
- `distro_inspection_and_security_tests` — distro parity + security
- `linux_bsd_inspection_tests` — Linux/BSD compatibility
- `sovereign_inspection_suite` — full system verification
- `stress_and_fuzz_tests` — robustness testing

---

## 🤝 Contributing

See [Contributing](Contributing) for the full guide.

Quick summary:
1. Fork the repository
2. Create a feature branch: `feat/my-feature`
3. Follow the [Rust style guide](https://doc.rust-lang.org/stable/style-guide/)
4. Add tests for new features
5. Open a Pull Request targeting `main`

---

## 📄 License

SigmaOS is dual-licensed under **MIT OR GPL-2.0**.

---

*Last updated: September 2026*
