# AGENT Guidelines & Roadmap: Missing Linux & BSD Components Gap Closure in SigmaOS

This document provides operational AI agent guidelines and architectural roadmap specifications for completing missing components in SigmaOS when compared against modern Linux distributions (Arch, Debian, Fedora, Alpine, NixOS, Gentoo, Void, CachyOS, Omarchy) and BSD operating systems (FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Illumos/Solaris).

---

## 🧭 Agent Operational Guidelines

When implementing missing components or extending subsystem parity in SigmaOS:

1. **Zero External Dependencies**:
   - All kernel and core userland code must be written in pure, native, safe Rust (`#![no_std]` compatible for kernel modules).
   - Do not pull in third-party C/C++ libraries or external crates in kernel/core modules.

2. **Cross-Distro Interoperability**:
   - Ensure every new subsystem registers with `SovereignUniversalDistroBridge` in `src/distro/linux_bsd_inspirations.rs`.
   - Maintain compatibility across all 174 supported distro subsystem modes.

3. **Standalone Unit Test Protocol**:
   - Every module must contain an isolated `#[cfg(test)]` test suite.
   - Verify that standalone compilation via `rustc --edition=2021 --test` produces zero warnings and 100% test pass rates.

---

## 🎯 Architectural Roadmap & Subsystem Gap Closure Matrix

### Phase 1: Init & Service Management
- **Target Systems**: Systemd, OpenRC, Runit, Shepherd, Dinit, SMF, RCD.
- **Key Missing Capabilities**:
  - Socket activation and dynamic `systemd-homed` post-quantum storage image mounting.
  - Service dependency graph resolution with exponential backoff respawning.
  - Fine-grained service cgroups v2 resource capping.

### Phase 2: Package Management & Build Pipelines
- **Target Systems**: Apt (`.deb`), Pacman (`.pkg.tar.zst`), Dnf (`.rpm`), Alpine (`.apk`), Void (`.xbps`), Gentoo (`.ebuild`), Nix Flakes, FreeBSD/OpenBSD `.pkg`.
- **Key Missing Capabilities**:
  - Multi-format package transpilation gateway converting foreign package specs to native `.sigpkg`.
  - Content-Addressed Storage (CAS) with reachability mark-and-sweep garbage collection.
  - SAT dependency solver using Davis-Putnam-Logemann-Loveland (DPLL) with cycle detection.
  - Post-Quantum Cryptography (PQC) signature validation (Dilithium / Falcon / ML-KEM).

### Phase 3: Kernel Mechanics & System Calls
- **Target Systems**: Linux 6.x+, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Illumos.
- **Key Missing Capabilities**:
  - eBPF x86_64 JIT compilation and zero-copy AF_XDP socket maps (`XSK`).
  - OpenBSD KARL (Kernel Address Randomized Link) and W^X memory page allocation.
  - NetBSD Rump Kernel userland driver host bridge for isolated driver execution.
  - DragonFly BSD lockless per-CPU netpoll ring & variant symlinks (`varsyms`) resolution.

### Phase 4: Security & Access Control
- **Target Systems**: Landlock LSM, FreeBSD Capsicum, OpenBSD Pledge/Unveil, Linux Capabilities, SELinux/AppArmor.
- **Key Missing Capabilities**:
  - Landlock v5 network port rule enforcement (`TcpBind`, `TcpConnect`).
  - Strict unveil restriction locking and immutable pledge transition gates.
  - IOMMU DMA fault containment and Capsicum capability rights enforcement.

### Phase 5: Storage & Filesystems
- **Target Systems**: Bcachefs, OpenZFS, Btrfs, HAMMER2, Ext4, XFS.
- **Key Missing Capabilities**:
  - Bcachefs multi-tier storage engine with dynamic SSD promotion and cold HDD demotion.
  - OpenZFS Fletcher-4 checksum verification, Copy-On-Write transaction groups, and zero-copy dataset clones.
  - DragonFly HAMMER2 MVCC B-Tree snapshotting and cluster quorum consensus.

### Phase 6: Graphics, Display & Desktop Environments
- **Target Systems**: Wayland 1.28+, DRM/KMS, GTK4/Qt6, Cinnamon/XApp, Hyprland, Zenith Desktop.
- **Key Missing Capabilities**:
  - Wayland sub-millisecond direct KMS scanout pipeline with 3D LUT HDR color transformations.
  - Cinnamon/XApp desktop integration (Desklets, Warpinator LAN transfer, Timeshift snapshots, Hypnotix IPTV).
  - Omarchy Developer Tools Suite (Theme Switcher, Stow Manager, Hyprland Binds, Fastfetch, Herdr AI Orchestrator).

---

## 🛠️ Verification & Compliance Commands

```bash
# Verify standalone compilation for Linux & BSD distro inspirations
rustc --edition=2021 --test --cfg 'feature="standalone_test"' src/distro/linux_bsd_inspirations.rs -o build/test_inspirations && ./build/test_inspirations

# Execute complete SigmaOS test suite
bash run_sigma_tests.sh
```
