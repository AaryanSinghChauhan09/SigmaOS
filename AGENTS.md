<<<<<<< HEAD
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
=======
# AGENTS.md — AI Agent Operating Instructions for SigmaOS

Welcome, AI Agent! This file provides essential context, coding standards, instructions, and verification commands for working with the **SigmaOS** repository.

---

## ⚡ Tri-Agent Roles & Responsibilities

1. **Bolt ⚡ (Performance & Speed Optimization)**
   - Hunt for bottlenecks, heap allocation overhead, $O(N^2)$ algorithm loops, and cache misses.
   - Implement clean, measurable performance optimizations (<50 lines) without sacrificing readability.
   - Log critical performance learnings in `.jules/bolt.md`.

2. **Palette 🎨 (UX, Ergonomics & Accessibility)**
   - Enhance CLI output, Web UI components, and desktop tools.
   - Ensure WCAG 2.1 AA compliance, visible focus indicators (`:focus-visible`), and explicit ARIA annotations (`role="tablist"`, `aria-label`).
   - Log critical UX learnings in `.jules/palette.md`.

3. **Sentinel 🛡️ (Security, PQC Integrity & Compliance)**
   - Protect memory safety, driver execution boundaries, PII data masking (GDPR/HIPAA), and Dilithium-5 post-quantum signature verifications.
   - Ensure mock test credentials use `mock_` or `test_` variable prefixes.
   - Log critical security learnings in `.jules/sentinel.md`.

---

## 🚗 Driver Management Protocols for AI Agents

When working on or interacting with the **Driver Subsystem** (`src/driver/`):
- Refer to `docs/AI_AGENT_DRIVER_MANAGEMENT.md` for complete driver lifecycle directives.
- Always enforce bounds checking on ring buffers, virtqueues, and MMIO submission/completion queue pointers.
- Ensure out-of-tree or DKMS modules are built inside sandboxed environments (`SbuildChrootSandboxEngine`) and signed with Dilithium-5 signatures (`Dilithium5KernelSignatureVerifier`).
- Ensure fallback mechanisms exist (`SovereignDriverRecovery`) whenever probing or initializing bare-metal hardware drivers (`NvmePCIeHostController`, `IntelE1000eNicDriver`, `XhciHostControllerDriver`).

---

## 🧪 Testing & Verification Commands

### Cargo & Standalone Test Suites
```bash
# Verify library compilation
cargo check --lib

# Run standalone test runners for specific modules
rustc --test src/package/universal.rs --edition=2021 --cfg 'feature="standalone_test"' -D warnings -o /tmp/test_universal && /tmp/test_universal
rustc --test src/kernel/linux_parity.rs --edition=2021 -o /tmp/test_linux_parity && /tmp/test_linux_parity
rustc --test src/distro/omarchy.rs --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_omarchy && /tmp/test_omarchy
rustc --test src/userland/indiastack/sigma_india_stack.rs --edition=2021 -o /tmp/test_india_stack && /tmp/test_india_stack
rustc --test src/driver/distro_drivers.rs --edition=2021 -o /tmp/test_distro_drivers && /tmp/test_distro_drivers

# Run integration test suites
cargo check --test distro_inspirations_tests
cargo check --test namespace_integration_full
>>>>>>> origin/main-12914436675390967473
```

---

<<<<<<< HEAD
## 4. Pull Request & Commit Guidelines
- Repository git branches must follow the naming convention starting with `jules-`.
- Maintain descriptive commit messages following standard git conventions.
=======
## 📌 Commit & Submission Guidelines
- Commits must be made directly to the `main` branch without creating Pull Requests.
- Update `ImprovementPlan.md` and `NEXT_STEPS_GUIDELINES.md` with audit progress and strategic roadmap entries.
>>>>>>> origin/main-12914436675390967473
