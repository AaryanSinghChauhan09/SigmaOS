# AI Agent Arch Linux Pacman & AUR Parity Maintenance & Development Guide

This document specifies guidelines and procedures for AI agents maintaining and extending Arch Linux pacman, ALPM (Arch Linux Package Management), and AUR integration components in SigmaOS.

---

## 1. Arch Linux Component Architecture

Arch Linux parity in SigmaOS is implemented in the following zero-dependency `#![no_std]` Rust files:

| Component | File Path | Scope & Responsibilities |
|---|---|---|
| **Pacman & ALPM Engine** | `src/sigpkg/arch_pacman_engine.rs` | `pacman.conf` option parser (`ArchPacmanConfig`), ALPM transaction hooks (`ArchAlpmHookEngine`), mirrorlist latency ranker (`ArchMirrorlistRanker`) |
| **Arch Namcap & ALPM Integrity** | `src/distro/arch.rs` | Namcap PKGBUILD linter (`ArchNamcapLinterEngine`), ALPM local database structure & checksum verifier (`ArchAlpmDbIntegrityEngine`), multi-kernel manager |
| **ALPM Parallel Sync Engine** | `src/compatibility/distro_parity_ultimate.rs` | Parallel download connection pool (`AlpmParallelSyncEngine`), mirror ranking by latency and error rate |

---

## 2. Technical Directives & Principles

1. **Pacman Configuration Parity**:
   - `ArchPacmanConfig` must parse `ParallelDownloads`, `ILoveCandy`, `Color`, `IgnorePkg`, `NoExtract`, and repo sections (`[core]`, `[extra]`, `[multilib]`).
2. **ALPM Hook Lifecycle**:
   - ALPM transaction hooks support `PreTransaction` and `PostTransaction` triggers matching on file glob patterns (`Target = usr/lib/initcpio/*`).
3. **Standalone Testing**:
   ```bash
   rustc --test --edition 2021 --cfg 'feature="standalone_test"' src/sigpkg/arch_pacman_engine.rs -o build/arch_pacman_test && ./build/arch_pacman_test
   rustc --test --edition 2021 src/distro/arch.rs -o build/arch_test && ./build/arch_test
   ```

---

## 3. Maintenance Guidelines

### A. Namcap Linter (`ArchNamcapLinterEngine`)
- Audit PKGBUILD variables (`pkgname`, `pkgver`, `pkgrel`, `arch`, `depends`, `makedepends`).
- Warn against forbidden FHS paths (`/usr/local`, `/var/run`) and insecure file permissions.

### B. ALPM Database Integrity (`ArchAlpmDbIntegrityEngine`)
- Verify local database entries in `/var/lib/pacman/local/`.
- Check file existence and SHA-256 hashes against package `desc` and `files` manifests.

---

## 4. Verification Checklist

- [ ] Run `rustc --test --edition 2021 --cfg 'feature="standalone_test"' src/sigpkg/arch_pacman_engine.rs -o build/arch_pacman_test && ./build/arch_pacman_test`
- [ ] Run `rustc --test --edition 2021 src/distro/arch.rs -o build/arch_test && ./build/arch_test`
- [ ] Run `./run_sigma_tests.sh`
- [ ] Confirm `#![no_std]` zero-dependency compliance.
