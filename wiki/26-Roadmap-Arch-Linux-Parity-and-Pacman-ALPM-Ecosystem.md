# AI Agent Roadmap: Arch Linux Parity & Pacman ALPM Ecosystem

This document details the strategic AI agent roadmap for **Arch Linux Parity, Pacman ALPM Library Integration, PKGBUILD Synthesis, and AUR Ecosystem Compatibility** in SigmaOS.

---

## 1. Arch Linux Parity Architecture in SigmaOS

| Component | Inspiration | SigmaOS Implementation | AI Agent Autonomous Function |
| :--- | :--- | :--- | :--- |
| **Package Builder** | `makepkg` | `ArchMakepkgEngine` (`src/distro/arch_missing_components.rs`) | Parses `PKGBUILD` manifests, resolves `depends`/`makedepends`, builds `.pkg.tar.zst` tarballs, and computes checksums. |
| **Package Linter** | `namcap` | `ArchNamcapLinterEngine` | Audits PKGBUILDs and built packages for FHS path violations, unused dependencies, or insecure file permissions. |
| **Database & Orphans** | `ALPM` / `pacman-db` | `ArchAlpmDatabaseEngine` | Manages local package records (`/var/lib/pacman/local`), identifies orphan dependencies (`pacman -Qtd`), and cleans database locks. |
| **AUR Helper & RPC Client** | Arch User Repository | `ArchAurV5ClientEngine` (`src/package/arch_aur.rs`) | Queries AUR v5 Web RPC JSON APIs, fetches PKGBUILD repositories, and sandbox-compiles AUR helper builds. |
| **Version Comparison** | `vercmp` | `ArchVercmpEngine` | Implements ALPM version comparison algorithm handling epochs (`1:2.0`), pkgver, pkgrel, and alphanumeric alpha/beta pre-release tags. |
| **Mirror Ranker** | `reflector` | `ArchReflectorMirrorlistEngine` (`src/distro/arch_parity.rs`) | Filters and ranks global Arch mirrors based on download speed (kbps), sync latency (ms), country filters, and SSL status. |

---

## 2. Milestone Roadmap Tracks

### Track A: ALPM Database & Transaction Hook Graph Execution (Months 1–3)
1. **ALPM Transaction Hook Graph**: Parse `.hook` files in `/usr/share/libalpm/hooks/`, construct pre/post transaction hook execution DAGs, and trigger `mkinitcpio` or `systemctl daemon-reload` hooks.
2. **Pacman Cache Pruning (`paccache`)**: Scan `/var/cache/pacman/pkg/`, identify superseded `.pkg.tar.zst` versions, keep N configured releases, and purge outdated archives.

### Track B: PKGBUILD Synthesis & Namcap Static Analysis (Months 4–6)
1. **Automated PKGBUILD Generation**: Synthesize compliant `PKGBUILD` templates for upstream Git repositories, calculating source tarball hashes and setting standard `CFLAGS`/`RUSTFLAGS`.
2. **Automated Namcap Security Auditing**: Run static analysis over build artifacts, flagging executable stacks, world-writable files, missing `pkgdesc`, or redundant library linkage.

### Track C: AUR Build Sandbox & Live ISO Image Generation (Months 7–12)
1. **Isolated AUR Sandbox Compilation**: Build untrusted AUR packages inside `SigmaSandbox` / `OpenBSD pledge` containers with strict file unveil policies and unprivileged UID isolation.
2. **Archiso Live CD / Installer Generation**: Assemble SquashFS rootfs structures, construct Zstd compressed kernel initramfs images (`mkinitcpio`), and build bootable ISO images (`archiso`).

---

## 3. Verification & Compliance Standards

- **Unit Test Coverage:** All Arch Linux parity modules must include unit tests in `src/distro/arch*.rs` and `src/package/arch*.rs`.
- **System Verification:** Execute `./run_sigma_tests.sh` to confirm 100% pass rate across all 174 active subsystems.
- **Zero External Dependencies:** Native `#![no_std]` or Safe-Rust implementations without external crate dependencies.
