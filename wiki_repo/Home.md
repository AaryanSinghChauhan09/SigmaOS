# 🇸🇴 SigmaOS Sovereign Operating System

> **A sovereign, post-quantum resilient, zero-dependency operating system written in Rust, combining microkernel security with bare-metal performance.**

***

## 🌟 Architectural Highlights

SigmaOS is designed from the ground up to transcend POSIX legacy limitations and monolithic kernel bloat, drawing strategic inspiration from leading Linux and BSD distributions:

*   **Sovereign Microkernel Core**: Zero-allocation, capability-gated microkernel with isolated userspace shards (`BuddyAllocator`, `CapabilityGate`).
*   **NixOS / Guix Parity**: Purely declarative system state configurations, content-addressed package store (CAS), and instant atomic rollbacks.
*   **Arch Linux & Gentoo Parity**: SAT-based zero-allocation dependency solver (`SatSolver`), PKGBUILD recipe sandbox compiler, and Portage USE-flag compilation.
*   **Clear Linux Parity**: Stateless `/usr` configuration overlay architecture (`ClearLinuxStatelessOverlayEngine`).
*   **OpenBSD Security Hardening**: Hardware-enforced process restriction (`pledge`), file path masking (`unveil`), W^X memory execution policies, and Retguard return-address canaries.
*   **FreeBSD Isolation**: Jails virtualization with nested hierarchies, RACCT/RCTL resource controls, and Capsicum descriptor capability delegation.
*   **DragonFly BSD & openSUSE Parity**: HAMMER2 PFS multi-version B-tree filesystem, variant symlinks (`varsyms`), and Snapper CoW pre/post transaction recovery.
*   **Zenith Desktop Compositor**: Direct-to-hardware framebuffer rendering without Wayland/X11 bloat, featuring HiDPI fractional scaling, Variable Refresh Rate (VRR), Sway-style tiling matrices, and Gamescope-inspired direct scanout blitting.

***

## 🚀 Quick Start

### Prerequisites

*   Rust nightly toolchain
*   QEMU (`qemu-system-x86_64`)
*   GCC / G++ toolchain

### Build & Run

```bash
# Clone the repository
git clone https://github.com/SigmaOS/SigmaOS.git
cd SigmaOS

# Run atomic test suite and inspection tests
./run_sigma_tests.sh

# Build bootable ISO image
bash scripts/build-iso.sh

# Run QEMU smoke test
python3 scripts/qemu_smoke_test.py
```

***

## 📚 Documentation & Wiki

Explore detailed specifications and guides in the `wiki/` directory and online GitHub Wiki:

*   [Home](wiki/Home)
*   [Architecture](wiki/Architecture)
*   [Linux Distros Architecture & Parity Guide](wiki/Linux-Distros-Architecture)
*   [BSD Security Hardening Guide](wiki/BSD-Security-Hardening)
*   [Declarative Package Management](wiki/Declarative-Package-Management)
*   [Security Model](wiki/Security)
*   [Driver Development](wiki/Driver-Development)
*   [Installation Guide](wiki/Installation)
*   [Roadmap](wiki/Roadmap)

***

## 📄 License

SigmaOS is licensed under the [MIT License](licensing.rs).
