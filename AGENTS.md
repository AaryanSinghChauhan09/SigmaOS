# AGENTS.md - AI Agent Operational & File Management Guidelines for SigmaOS

Welcome, AI Agent! This document outlines operational procedures, architectural conventions, and strict file management guidelines when working on the **SigmaOS** codebase.

---

## 1. Core Directives & Architecture Principles

1. **Zero External Dependencies (`#![no_std]`)**:
   - SigmaOS operates on a strict sovereign zero-dependency philosophy.
   - Do **not** add external third-party crates under `[dependencies]` in `Cargo.toml`.
   - Use `alloc::` primitives (`alloc::string::String`, `alloc::vec::Vec`, `alloc::boxed::Box`, `alloc::format!`) for heap allocations in kernel/distro space.

2. **Multi-Architecture Support**:
   - Maintain multi-arch abstractions across supported architectures: `x86_32`, `x86_64`, `aarch64`, `riscv64`, `loongarch64`, `powerpc64`, and `s390x`.
   - Architectural register contexts and trap handlers live in `src/arch/portability.rs`.

3. **Subsystem Interoperability**:
   - Core subsystem bridges (VFS, Init, Package Management, Security, Kernel, Memory) route through `src/distro/linux_bsd_inspirations.rs` (`SovereignUniversalDistroBridge`).

---

## 2. File Management & Organization Guidelines

### 2.1 Code Base Layout
- **Kernel Core**: `src/kernel/`, `src/klib/`, `src/memory/`, `src/arch/`
- **Distro Innovations & Parity**: `src/distro/`
  - `src/distro/linux_bsd_inspirations.rs` - Cross-subsystem distro bridge, Landlock v5, eBPF XDP zero-copy, OpenBSD pledge/unveil, FreeBSD jails, and Illumos zones.
  - `src/distro/sovereign_nextgen_distro_leap.rs` - `sched_ext` BPF scheduling, CAS store, HAMMER2 CoW deduplication.
- **Package Management Subsystem**: `src/package/`, `src/sigpkg/`
  - `src/sigpkg/universal_adapter.rs` - Universal package format adapter router (.deb, .rpm, PKGBUILD, ebuild, apk, snap, flatpak, hpkg).
  - `src/sigpkg/universal_oop_system.rs` - Strategy, Adapter, Factory, Decorator, and Observer pattern implementations for package management.
- **Compatibility & Standards**: `src/compatibility/`
- **Drivers & Hardware**: `src/drivers/`
- **Documentation**: `docs/` and `wiki/`

### 2.2 Rules for Modifying Existing Files
1. **Targeted Editing**:
   - Always trace imports and conditional compilation gates (`#[cfg(test)]`, `#[cfg(feature = "standalone_test")]`) before editing source files.
   - Avoid modifying generated build artifacts under `build/` or `target/`.
2. **Atomic & Reversible File Operations**:
   - Use Copy-on-Write (CoW) transaction semantics when updating critical configuration or state files.
   - Perform verification with read-only tools immediately after any file creation, modification, or deletion.

### 2.3 Rules for Adding New Files
1. Place new module source files under the appropriate domain directory (`src/distro/`, `src/package/`, `src/drivers/`, etc.).
2. Re-export new modules in `mod.rs` and `src/lib.rs` as required.
3. Add standalone unit test blocks or test binaries in `tests/` or via inline `#[cfg(test)]` / `#[cfg(feature = "standalone_test")]` blocks.

---

## 3. Testing Protocols & Verification

All changes must be validated against the native SigmaOS test suite:

```bash
# Run full test suite (atomic unit tests, subsystem inspection, Python pytest suite)
./run_sigma_tests.sh

# Run standalone module tests
rustc --edition=2021 --test --cfg 'feature="standalone_test"' src/distro/linux_bsd_inspirations.rs -o build/test_inspirations && ./build/test_inspirations
```

---

## 4. Pull Request & Commit Guidelines
- Repository git branches must follow the naming convention starting with `jules-`.
- Maintain descriptive commit messages following standard git conventions.
