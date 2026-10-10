# AI Agent Roadmap: Arch Linux Distro Parity Plan (PR Format)

## Overview & Architecture Principles
This document defines the AI Agent Roadmap in Pull Request (PR) proposal format for achieving 100% native feature parity with **Arch Linux** and Arch-based distributions (e.g., CachyOS, EndeavourOS, Manjaro) in zero-dependency `#![no_std]` Rust within SigmaOS.

Inspired by standard Arch Linux tooling, SigmaOS implements native Rust parity engines in `src/distro/arch_missing_components.rs` and `src/sigpkg/` that replace external shell scripts and binaries with zero-allocation, lockless, memory-safe algorithms.

---

## Roadmap Phases & Pull Request Matrix

### Phase 1: Makepkg & Package Building Engine (`ArchMakepkgEngine`)
- **PR Title:** `feat(arch): Implement native PKGBUILD parser and makepkg build engine`
- **Inspiration:** Arch Linux `makepkg` bash suite.
- **SigmaOS Subsystem:** `src/distro/arch_missing_components.rs` -> `ArchMakepkgEngine`
- **Key Capabilities:**
  - Zero-allocation parsing of `PKGBUILD` metadata arrays (`pkgname`, `pkgver`, `pkgrel`, `arch`, `depends`, `makedepends`).
  - Landlock & Unveil sandboxed build tree execution.
  - Generating `.PKGINFO` and `.BUILDINFO` binary manifests with SLSA v1.0 attestations.

### Phase 2: Namcap Package Linter & Security Auditor (`ArchNamcapLinterEngine`)
- **PR Title:** `feat(arch): Implement Namcap static analysis & policy compliance auditor`
- **Inspiration:** Arch Linux `namcap` package linter.
- **SigmaOS Subsystem:** `src/distro/arch_missing_components.rs` -> `ArchNamcapLinterEngine`
- **Key Capabilities:**
  - Auditing `.pkg.tar.zst` packages for missing runtime library dependencies, redundant dynamic linkages, and insecure directory permissions (`0777`, SUID/SGID).
  - Validating license identifier strings against SPDX standards.

### Phase 3: ALPM Database Integrity & Lockless Transaction Engine (`ArchAlpmDbIntegrityEngine`)
- **PR Title:** `feat(arch): Implement ALPM local/sync database sync and lockless transaction manager`
- **Inspiration:** libalpm (`/var/lib/pacman/local/` and `/var/lib/pacman/sync/`).
- **SigmaOS Subsystem:** `src/distro/arch_missing_components.rs` -> `ArchAlpmDbIntegrityEngine`
- **Key Capabilities:**
  - Fast memory-mapped ALPM database parsing with zero-copy slice indexing.
  - Transactional rollback checkpoints for safe package installation and removal.

### Phase 4: AUR Web RPC v5 Client & PQC Keyring (`ArchAurWebRpcClient` & `ArchPacmanKeyringPqcEngine`)
- **PR Title:** `feat(arch): Implement AUR v5 RPC client with PQC Dilithium5 signature verification`
- **Inspiration:** AUR RPC v5 & `pacman-key` Web of Trust.
- **SigmaOS Subsystem:** `src/distro/arch_missing_components.rs` -> `ArchAurWebRpcClient`, `ArchPacmanKeyringPqcEngine`
- **Key Capabilities:**
  - Asynchronous HTTPS JSON API querying for AUR package search and info requests.
  - Hybrid Post-Quantum Cryptography (PQC Dilithium5 + OpenPGP Ed25519) package signature verification.

### Phase 5: Modular Initramfs & System Boostraper (`ArchMkinitcpioEngine` & `ArchPacstrapChrootEngine`)
- **PR Title:** `feat(arch): Implement mkinitcpio hook generator, pacstrap, and arch-chroot engine`
- **Inspiration:** Arch Linux `mkinitcpio`, `pacstrap`, and `arch-chroot`.
- **SigmaOS Subsystem:** `src/distro/arch_missing_components.rs` -> `ArchMkinitcpioEngine`, `ArchPacstrapChrootEngine`
- **Key Capabilities:**
  - Assembling minimal `#![no_std]` initramfs archives with systemd-boot compatibility.
  - Isolated chroot environment bootstrapping with virtual VFS mount point propagation.

---

## Verification & Continuous Integration
All Arch Linux parity engines are continuously verified using standalone Rust unit tests:
```bash
rustc --test --edition=2021 src/distro/arch_missing_components.rs -o build/arch_test && ./build/arch_test
```
