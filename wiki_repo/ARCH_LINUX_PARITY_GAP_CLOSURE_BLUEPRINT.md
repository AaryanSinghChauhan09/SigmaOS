# SigmaOS Arch Linux Parity Gap Closure Blueprint

## Overview
This document establishes the comprehensive technological parity and supremacy blueprint between **SigmaOS** and **Arch Linux** (including AUR, Pacman, ALPM, makepkg, archinstall, arch-audit, and ALA).

## Core Architectural & Algorithmic Parity Pillars

### 1. Pacman & ALPM Database Management
- **Pacman Repository Syncing**: Fast mirror ranking, delta downloads, database lock management (`/var/lib/pacman/db.lck`), and transaction hooks (`/etc/pacman.d/hooks/`).
- **ALPM Library Integration**: Native Rust libalpm equivalent providing transactional package installation, removal, dependency resolution (`AlpmDatabase`), and orphan package detection (`pacman -Qtdq`).

### 2. Arch User Repository (AUR) & PKGBUILD Build Pipeline
- **AUR v5 Web RPC**: Querying package metadata, votes, popularity, and out-of-date status via `ArchAurWebRpcClient`.
- **PKGBUILD Lexer & Parser**: Complete parsing of `pkgname`, `pkgver`, `pkgrel`, `arch`, `depends`, `makedepends`, `source`, `sha256sums`, `build()`, and `package()` functions (`ArchPkgbuild`).
- **Makepkg Clean Chroot Sandbox**: Hermetic build environment (`ArchMakepkgEngine` / `ArchRecipeSandboxCompiler`) producing `.pkg.tar.zst` binary archives with `.PKGINFO` and `.BUILDINFO` attestation.

### 3. Arch Linux Archive (ALA) & Time-Travel Package Engine
- **Historical Snapshot Repository**: Time-travel package lookups (`https://archive.archlinux.org/repos/YYYY/MM/DD/$repo/os/$arch`) via `ArchLinuxArchiveEngine`.
- **Historical Mirrorlist Generation**: Dynamic creation of dated pacman mirrorlists for byte-exact historical system reproduction and regression testing.

### 4. Arch Security & Auditing (`arch-audit` Parity)
- **ASA Security Advisory Parser**: Ingestion of Arch Security Advisories (`ASA-YYYYMM-X`) and CVE mapping via `ArchAuditScannerEngine`.
- **Installed Package Vulnerability Scanner**: Automated comparison of installed package versions against known vulnerabilities to alert administrators of unpatched packages.

### 5. Arch Testing Repositories & Release Pipelines
- **Testing Repository Management**: Granular control over `[core-testing]`, `[extra-testing]`, and `[multilib-testing]` repositories using `ArchTestingRepositoryManager`.
- **Testing-to-Stable Staging**: Automated QA verification before promoting packages from testing to stable repositories.

### 6. Arch Pacman Keyring & PGP/PQC Web of Trust (`archlinux-keyring`)
- **Master Signing Keys**: Management of Arch Linux Master Keys (`3E80CA1B`, `6D1655C1`) via `ArchPacmanKeyringManager`.
- **Web of Trust Verification**: PGP & Post-Quantum Cryptography (Dilithium5) signature verification for all downloaded database files and package tarballs.

### 7. Automated Archinstall Profile Engine (`archinstall` Parity)
- **Declarative Profile Generation**: Automated JSON/Rust profile generation for disk layout, partitioning, LUKS2 volume encryption, and Btrfs subvolumes (`@root`, `@home`, `@snapshots`, `@log`).
- **Bootloader & Desktop Provisioning**: Automated configuration of `systemd-boot` and Zenith hyperdesktop.

---

## Verification Protocol
All Arch Linux parity engines are fully integrated into the native Rust codebase (`src/distro/arch_gap_closure.rs`, `src/distro/arch_missing_components.rs`, `src/sigpkg/arch_pacman_engine.rs`) and verified via `./run_sigma_tests.sh`.
