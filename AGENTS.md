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
# 🤖 SigmaOS AI Agent Governance Specification (`AGENTS.md`)

**Version:** 1.2.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Process, Memory, & Loader Management

---

## EXECUTIVE SUMMARY & AGENT ARCHITECTURE

SigmaOS features an AI-native process, memory, and module loader architecture where autonomous agent processes govern kernel scheduling, memory pools, security auditing, and dynamic module loading.

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT GOVERNANCE     |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • Boot Speed Profiling     • Bootloader UI Styling     • Module Signature Audit
  • Module Load Optimization • Boot Splash Graphics      • Secure Boot Key Check
  • Sub-µs Memory Access     • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 4. Pull Request & Commit Guidelines
- Repository git branches must follow the naming convention starting with `jules-`.
- Maintain descriptive commit messages following standard git conventions.
## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Scope**: CPU scheduling, `cgroups v2`, boot time profiling (`src/tools/bootloader.rs`), initramfs decompression speed, zero-allocation hot paths.
- **Rules**:
  - Profile kernel module loading times (`src/kernel/module_loader.rs`) and eliminate boot delay bottlenecks.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Scope**: Desktop compositor layout, boot menu styling, console progress indicators, accessibility state trees.
- **Rules**:
  - Maintain WCAG 2.1 AA compliant boot and desktop interfaces.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Scope**: LSM auditing, OpenBSD `pledge`/`unveil`, Post-Quantum Dilithium-5 kernel module signature verification, Secure Boot validation.
- **Rules**:
  - Enforce Dilithium-5 digital signature checks prior to kernel module loading.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. PROCESS & LOADER MANAGEMENT POLICIES

### Module Loader Rules (`src/kernel/module_loader.rs`)
- **Signature Verification**: Every kernel module must be signed with Dilithium-5 keys before symbol relocation.
- **A/B Boot Rollback**: Failed module or stage-2 boot attempts trigger automatic fallback via `Firmitas` A/B slot mechanics.

---

## 3. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
