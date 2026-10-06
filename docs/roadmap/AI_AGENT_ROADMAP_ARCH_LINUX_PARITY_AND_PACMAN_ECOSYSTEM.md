# AI Agent Roadmap: Arch Linux Parity & Pacman ALPM Ecosystem
# SigmaOS Future Development Specification

This document details the AI Agent Future Development Roadmap for **Arch Linux Parity, Pacman ALPM Library Integration, PKGBUILD Synthesis, and AUR Ecosystem Compatibility** in SigmaOS, taking inspiration from Arch Linux and BSD distributions.

---

## 1. Executive Summary & Design Inspiration

SigmaOS achieves native parity with the Arch Linux ecosystem (`pacman`, `makepkg`, `namcap`, `vercmp`, AUR v5 RPC, `reflector`, `archinstall`, `archiso`) via zero-dependency Safe Rust abstractions:

| Subsystem Component | Arch Linux Inspiration Source | SigmaOS Native `#![no_std]` / Safe Rust Implementation | AI Agent Autonomous Role |
| :--- | :--- | :--- | :--- |
| **Package Builder** | `makepkg` | `ArchMakepkgEngine` (`src/distro/arch_missing_components.rs`) | Parses `PKGBUILD` manifests, resolves `depends`/`makedepends`, builds `.pkg.tar.zst` tarballs, and computes Zstd/SHA256 checksums. |
| **Linter & Policy Auditor** | `namcap` | `ArchNamcapLinterEngine` | Inspects `.PKGBUILD` and built package archives for FHS path violations, unused dependencies, missing shared libraries, and security issues. |
| **Database & Orphan Cleanup** | `ALPM` + `pacman-db` | `ArchAlpmDatabaseEngine` | Tracks installed package records (`/var/lib/pacman/local`), identifies orphan dependencies (`pacman -Qtd`), and checks database locks. |
| **AUR Search & RPC Client** | Arch User Repository (AUR v5) | `ArchAurV5ClientEngine` (`src/package/arch_aur.rs`) | Queries AUR v5 Web RPC JSON APIs, fetches PKGBUILD repositories, and sandbox-compiles AUR helper builds. |
| **Version Comparison Engine** | `vercmp` | `ArchVercmpEngine` | Implements ALPM version comparison algorithm handling epochs (`1:2.0`), pkgver, pkgrel, and alphanumeric alpha/beta pre-release tags. |
| **Mirror Ranking Engine** | `reflector` | `ArchReflectorMirrorlistEngine` (`src/distro/arch_parity.rs`) | Filters and ranks global Arch mirrors based on download speed (kbps), sync latency (ms), country filters, and SSL status. |

---

## 2. Strategic Milestone Roadmap & AI Agent Workflows

### Milestone 1: ALPM Database & Transaction Hook Execution (Months 1–3)
- **AI Agent Workflow 1.1: ALPM Transaction Hook DAG Resolution**
  - Parse `.hook` files in `/usr/share/libalpm/hooks/`, construct pre/post transaction hook execution DAGs, and trigger `mkinitcpio` or `systemctl daemon-reload` hooks.
- **AI Agent Workflow 1.2: Pacman Cache Pruning (`paccache`)**
  - Scan `/var/cache/pacman/pkg/`, identify superseded `.pkg.tar.zst` versions, keep N configured releases, and purge outdated archives.

### Milestone 2: PKGBUILD Synthesis & Namcap Static Analysis (Months 4–6)
- **AI Agent Workflow 2.1: Automated PKGBUILD Generation**
  - Synthesize compliant `PKGBUILD` templates for upstream Git repositories, calculating source tarball hashes and setting standard `CFLAGS`/`RUSTFLAGS`.
- **AI Agent Workflow 2.2: Automated Namcap Security Auditing**
  - Run static analysis over build artifacts, flagging executable stacks, world-writable files, missing `pkgdesc`, or redundant library linkage.

### Milestone 3: AUR Build Sandbox & Live ISO Image Generation (Months 7–12)
- **AI Agent Workflow 3.1: Isolated AUR Sandbox Compilation**
  - Build untrusted AUR packages inside `SigmaSandbox` / `OpenBSD pledge` containers with strict file unveil policies and unprivileged UID isolation.
- **AI Agent Workflow 3.2: Archiso Live CD / Installer Generation**
  - Assemble SquashFS rootfs structures, construct Zstd compressed kernel initramfs images (`mkinitcpio`), and build bootable ISO images (`archiso`).

---

## 3. Verification & Compliance Standards

1. **Compilation:** Confirm clean compilation with `cargo check --lib`.
2. **Native Test Suite:** Execute `./run_sigma_tests.sh` and ensure 100% test pass rate across all 174 active subsystems.
3. **Zero External Downloads:** All ALPM database handlers, version comparison routines, and PKGBUILD parsers must operate natively in Safe Rust.

---
*Generated for SigmaOS Arch Linux Parity & Pacman ALPM Ecosystem Specification*
