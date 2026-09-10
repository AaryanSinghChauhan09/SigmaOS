# 🏛️ SigmaOS Master Wiki

**SigmaOS** is a sovereign, secure, next-generation bare-metal operating system written in safe Rust (`#![no_std]`) — combining the performance, flexibility, and philosophy of **Arch Linux** with multi-distro Linux and BSD compatibility.

> ✅ **Build Status:** 0 compilation errors as of September 2026. All open PRs merged and verified.

---

## 🏛️ Master Arch-Style Wiki Portal

Explore the **[SigmaOS Arch Wiki Portal](SIGMAOS_ARCH_WIKI_PORTAL)** for comprehensive, Arch Linux Wiki-standard technical guides covering system administration, package management, security hardening, performance tuning, and Wayland desktop compositor setup.

---

## 🎯 The SigmaOS Philosophy (Arch Principles)

1. **Simplicity:** Zero-dependency, pure Rust microkernel shards and userland.
2. **Modernity:** Wayland wire protocol, eBPF XDP zero-copy networking, and CachyOS BORE scheduler.
3. **Pragmatism:** Universal package compatibility (`.deb`, `.rpm`, `PKGBUILD` / `.pkg.tar.zst`, `.apk`, `.xbps`, `.txz`).
4. **User-Centricity:** Transparent privilege delegation, sandboxing, and hardware Governor control.
5. **Versatility:** Rolling release agility with sub-millisecond atomic transactional state rollbacks (Snapper CoW, NixOS generations, FreeBSD ZFS boot environments).

---

## 🚀 Getting Started

| Page | Description |
|------|-------------|
| [SIGMAOS_ARCH_WIKI_PORTAL](SIGMAOS_ARCH_WIKI_PORTAL) | **Master Wiki Portal (Arch Linux Standard)** |
| [INSTALL](INSTALL) | Installation and build instructions |
| [BUILD](BUILD) | Build system, toolchain, and compilation |
| [CONTRIBUTING](CONTRIBUTING) | How to contribute to SigmaOS |
| [DEVELOPMENT_GUIDE](DEVELOPMENT_GUIDE) | Developer setup and workflow |

---

## 🏗️ Architecture & Core Kernel Shards

| Page | Description |
|------|-------------|
| [ARCHITECTURE](ARCHITECTURE) | System architecture overview |
| [kernel](kernel) | Kernel design and internals |
| [memory-management](memory-management) | Memory management subsystem |
| [process-management](process-management) | Process and task management |
| [filesystem](filesystem) | Virtual filesystem (VFS) |
| [networking](networking) | Networking stack |
| [drivers](drivers) | Driver model and hardware abstraction |
| [bootloader](bootloader) | Bootloader and early init |
| [shell](shell) | SigmaShell (sigma_sh) |

---

## 📦 Package Management & AUR Build Pipeline

| Page | Description |
|------|-------------|
| [ARCH_LINUX_PARITY_FEATURES](ARCH_LINUX_PARITY_FEATURES) | Arch Linux pacman, PKGBUILD, AUR, & Reflector parity |
| [PACKAGE_MANAGEMENT](PACKAGE_MANAGEMENT) | Complete sigpkg reference |
| [package-manager](package-manager) | Package manager architecture |
| [LINUX_BSD_DISTRO_COMPATIBILITY_GUIDE](LINUX_BSD_DISTRO_COMPATIBILITY_GUIDE) | Linux/BSD distro compatibility |
| [FEDORA_PARITY_FEATURES](FEDORA_PARITY_FEATURES) | Fedora-parity features |

---

## 🔐 Security & Hardening

| Page | Description |
|------|-------------|
| [SECURITY](SECURITY) | Security model and policies |
| [security](security) | Security subsystem internals |
| [api-reference](api-reference) | Public API reference |

---

## 📋 API & Namespace

| Page | Description |
|------|-------------|
| [API_DOCUMENTATION_v0.9](API_DOCUMENTATION_v0.9) | Full API documentation v0.9 |
| [NAMESPACE_IMPLEMENTATION](NAMESPACE_IMPLEMENTATION) | Namespace implementation details |
| [NAMESPACE_SYSCALLS_API_REFERENCE](NAMESPACE_SYSCALLS_API_REFERENCE) | Namespace syscall API |

---

## 🗺️ Roadmap & Status

| Page | Description |
|------|-------------|
| [ROADMAP](ROADMAP) | Development roadmap |
| [FUTURE-DEVELOPMENT-ROADMAP](FUTURE-DEVELOPMENT-ROADMAP) | Long-term vision |
| [CHANGELOG](CHANGELOG) | Version changelog |
| [RELEASE_NOTES_v0.9](RELEASE_NOTES_v0.9) | v0.9 release notes |
| [WHAT_IS_WORKING_AND_NOT_WORKING](WHAT_IS_WORKING_AND_NOT_WORKING) | Current feature status |
| [NEXT_STEPS_GUIDELINES](NEXT_STEPS_GUIDELINES) | Contributor guidelines for next steps |
| [TIER1_FEATURES](TIER1_FEATURES) | Tier-1 feature tracking |

---

## 🌐 Linux & BSD Distro Parity

| Page | Description |
|------|-------------|
| [SigmaOS-vs-Linux-Distros-Comparative-Dashboard](SigmaOS-vs-Linux-Distros-Comparative-Dashboard) | Feature parity dashboard vs major distros |
| [SigmaOS_Gap_Closing_Roadmap](SigmaOS_Gap_Closing_Roadmap) | Gap-closing roadmap vs Linux |
| [LINUX_BSD_INNOVATIONS_IMPLEMENTED](LINUX_BSD_INNOVATIONS_IMPLEMENTED) | Implemented Linux/BSD innovations |
| [Operations-and-Continuous-Improvement-Guide](Operations-and-Continuous-Improvement-Guide) | Ops and CI guide |
| [SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V22](SOVEREIGN_OS_ABSOLUTE_OMNIPRESENT_SELF_SUFFICIENCY_ULTRA_ENCYCLOPEDIA_V22) | Sovereign OS Encyclopedia V22 |

---

## 💡 Strategic Planning & Improvement Ideas

| Page | Description |
|------|-------------|
| [100-Improvement-Ideas](100-Improvement-Ideas) | 100 improvement ideas |
| [ImprovementPlan](ImprovementPlan) | Improvement plan |
| [DETAILED_IMPROVEMENT_PLAN](DETAILED_IMPROVEMENT_PLAN) | Detailed improvement plan |
| [SIGMAOS_500_REPOS_TRI_AGENT_ABSORPTION_AND_IMPLEMENTATION_PLAN](SIGMAOS_500_REPOS_TRI_AGENT_ABSORPTION_AND_IMPLEMENTATION_PLAN) | 500-repo absorption plan |

---

*Last updated: September 2026 — Documentation aligned with Arch Linux Wiki principles.*
