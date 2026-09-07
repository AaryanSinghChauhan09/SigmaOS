# SigmaOS Architecture Guide for AI Agents (`docs/AGENTS.md`)

This document provides specialized architectural reference documentation for AI agents working within the `docs/` and `src/` hierarchy of SigmaOS.

---

## 1. System Architecture Overview

SigmaOS is designed as a **sovereign, zero-dependency, `#![no_std]` compliant operating system** in Rust. The architecture is divided into modular, decoupled layers:

### A. Architectural Pillars
1. **Multi-Architecture Portability Layer (`src/arch/`)**:
   - Hardware Abstraction Layer (HAL) supporting `x86_32`, `x86_64`, `aarch64`, `riscv64`, `loongarch64`, `powerpc64`, and `s390x`.
   - Context switching and trap frame handling via `SovereignContextSwitchEngine`.
   - CPU ISA feature auto-detection (x86-64-v1..v4, AVX-512, AMX, ARM64 Neoverse, RISC-V Vector) via `cpu_features.rs`.

2. **Kernel Core Subsystems (`src/kernel/` & `src/klib/`)**:
   - Hybrid Process Scheduling: EEVDF lag compensation, CachyOS BORE score calculations, FreeBSD ULE interactivity ranking, and Apache NuttX POSIX RT preemption-threshold gating.
   - Demand Paging & Memory Management: Lazy zone allocation, page fault handling, slab object caching, and buddy allocation.
   - Zero-Copy IPC: High-throughput lock-free ring buffers and Unix domain socket emulation.

3. **Distro Leapfrog & Parity Engines (`src/distro/`)**:
   - `SovereignSchedExtEngine`: Linux 6.12+ extensible BPF scheduler.
   - `SovereignLandlockV5Guard`: Linux Landlock v5 + FreeBSD Capsicum + OpenBSD Pledge/Unveil security.
   - `SovereignHermeticCasStoreEngine`: Nix/Guix Content-Addressed Storage store.
   - `SovereignMicroarchJitEngine`: Microarchitecture SIMD auto-tuning & JIT path routing.
   - `SovereignHammer2DeduplicationEngine`: DragonFly BSD HAMMER2 multi-master CoW block deduplication.

4. **Universal Package Management (`src/package/` & `src/sigpkg/`)**:
   - Multi-format package translation (DEB, RPM, Pacman, APK, Flatpak, Snap, AppImage, XBPS, Ebuild, Ports, PKG).
   - AUR integration, PKGBUILD recipe auditing, and generation-based package snapshot rollbacks.

5. **Clean-Room Compatibility Layers (`src/compatibility/`)**:
   - Fedora/RHEL core tooling (DNF, SELinux, Bodhi, Ignition, status.fpo, systemd-offline-update).
   - BSD subsystem parity (FreeBSD Jails, OpenBSD PF firewall).
   - LSB & FHS compliance tools, PAM, Cgroup v2 governor.

---

## 2. Coding Standards & Conventions for AI Agents

When implementing features or bug fixes in SigmaOS:

1. **Zero External Dependencies**: Maintain `[dependencies]` in `Cargo.toml` empty. Do not add third-party crates.
2. **Strict `#![no_std]` Compatibility**: Use `alloc::` primitives (`alloc::format`, `alloc::string::String`, `alloc::vec::Vec`, `alloc::collections::BTreeMap`) instead of `std` imports for `src/` modules.
3. **Trait Derivations**: Always derive `Debug`, `Clone`, and `PartialEq` where appropriate on data structures.
4. **Error Handling**: Use explicit `Result<T, &'static str>` or domain-specific enums instead of panicking.

---

## 3. Verification & Execution Commands

AI agents must verify their work using the following commands:

```bash
# 1. Compile & run standalone module unit tests
rustc --edition=2021 --test src/distro/sovereign_nextgen_distro_leap.rs -o build/test_nextgen_leap && ./build/test_nextgen_leap
rustc --edition=2021 --test src/arch/portability.rs -o build/test_arch_portability && ./build/test_arch_portability

# 2. Run global test suite
./run_sigma_tests.sh

# 3. Perform compilation check
cargo check
```

---
*End of docs/AGENTS.md*
