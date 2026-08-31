# 🏠 SigmaOS Wiki — Home

> **SigmaOS** is the world's most advanced sovereign, bare-metal operating system for the next generation of silicon sovereignty. Built 100% in Rust with a zero-dependency architecture targeting x86_64, AArch64, and RISC-V 64.

---

## 🚀 Quick Navigation

### Getting Started
| Page | Description |
|------|-------------|
| [Getting Started](Getting-Started) | First steps with SigmaOS |
| [Building SigmaOS](Building-SigmaOS) | How to build from source |
| [Quick Start Guide](Quick-Start-Guide) | 5-minute setup guide |
| [Installation Guide](Installation-Guide) | Full installation walkthrough |
| [FAQ](FAQ) | Frequently asked questions |

### Core Architecture
| Page | Description |
|------|-------------|
| [Architecture Overview](Architecture-Overview) | High-level architecture diagram |
| [Kernel Internals](Kernel-Internals) | Microkernel deep-dive |
| [Boot Process](Boot-Process-Architecture) | From UEFI to userspace |
| [Filesystem Spec](Filesystem-Spec-and-Virtual-FS) | SigmaFS and VFS layer |
| [AI Subsystem](AI-Subsystem) | S-AI integration details |

### Components
| Page | Description |
|------|-------------|
| [**Components Master Table**](Components-Master-Table) | 🆕 Full component table with status, source files |
| [Components & Inspirations](Components-and-Inspirations) | Component inspiration sources |

---

## 📊 Current Build Status

```
Kernel Core:           ████████████████████  95% ✅
Security Subsystem:    ████████████████████  90% ✅
Package Management:    ████████████████░░░░  80% ✅
Desktop (Zenith):      ████████████████░░░░  80% ✅
Networking:            ████████████░░░░░░░░  60% 🔄
Driver Ecosystem:      ████████████░░░░░░░░  60% 🔄
Virtualization:        ██████████░░░░░░░░░░  50% 🔄
AI Orchestration:      ████░░░░░░░░░░░░░░░░  20% 📋
Cloud / Enterprise:    ████░░░░░░░░░░░░░░░░  20% 📋
```

---

## 🐧 Linux & BSD Integration

SigmaOS implements ideas and compatibility layers from **13+ Linux distributions and BSDs**:

| Distro | Key Features Adopted | Status |
|--------|---------------------|--------|
| Arch Linux | AUR, rolling release, ALPM hooks | ✅ |
| Debian/Ubuntu | APT parity, AppArmor, netplan | ✅ |
| Fedora/RHEL | SELinux, podman, firewalld | ✅ |
| Gentoo | USE flags, hardened kernel, portage | ✅ |
| NixOS | Declarative config, atomic rollback | ✅ |
| Alpine Linux | musl-libc, apk speed, tiny footprint | ✅ |
| CachyOS | BORE scheduler, LTO, zstd | ✅ |
| Void Linux | runit, XBPS, no-systemd | ✅ |
| Artix Linux | Init choice, OpenRC/s6 compat | ✅ |
| OpenSUSE | snapper, transactional-update | ✅ |
| Parrot/Kali | Security tools, hardened kernel | 🔄 |
| QubesOS | VM compartmentalization, domains | 🔄 |
| FreeBSD/BSDs | pledge/unveil, jails, ZFS, pf | ✅ |

📄 Full details: [Linux Distro Ideas Implementation](Linux-Distro-Ideas-Implementation)

---

## 🔗 Quick Links

- **Repository**: [github.com/AaryanSinghChauhan09/SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS)
- **Issues**: [Report a bug or request a feature](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)
- **Pull Requests**: [Contribute code](https://github.com/AaryanSinghChauhan09/SigmaOS/pulls)
- **Discussions**: [Community forum](https://github.com/AaryanSinghChauhan09/SigmaOS/discussions)

---

*SigmaOS — Silicon Sovereignty. Built in Rust. Built to Last.*