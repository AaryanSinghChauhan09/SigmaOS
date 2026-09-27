# 📖 SigmaOS Development Rules & Contribution Guidelines

Inspired by the engineering discipline of the **Linux Kernel**, **FreeBSD Committers' Handbook**, **OpenBSD Security Principles**, and **NixOS Declarative Reproducibility**, this guide outlines mandatory rules, coding standards, and PR procedures for all contributors and AI agents developing SigmaOS.

---

## 🏛️ Core Principles & Governance

### 1. **Zero External Crates Philosophy**
- SigmaOS maintains absolute self-sufficiency and security by adhering to a **strict `#![no_std]` zero-third-party-crate rule**.
- **No external dependencies** may be added to `Cargo.toml`.
- All data structures and abstractions must rely solely on Rust `core::` and `alloc::` primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::collections::BTreeMap`, `alloc::format`).

### 2. **Memory Safety & Unsafe Invariants**
- **Safe Rust First**: Always prioritize memory safety without garbage collection.
- **Unsafe Boundaries**: `unsafe` blocks are permitted **only** for direct MMIO register access, bare-metal hardware instructions, or foreign function interfaces (FFI).
- **Mandatory Safety Documentation**: Every `unsafe` block must be preceded by a explicit `// SAFETY:` comment explaining why the invariant holds.

### 3. **Defensive Error Handling & Anti-Panic Policy**
- **No Production Panics**: Do not use `unwrap()`, `expect()`, `panic!()`, or unreachable branches in production execution paths.
- **Graceful Error Recovery**: Explicitly handle errors using `Result<T, E>` or `Option<T>`.

---

## 🔒 Security & Sandboxing Standards

### 1. **Multi-Model Access Control Integration**
- New system calls, VFS handlers, and network IPC mechanisms must integrate with `UnifiedAccessControlSuite`:
  * **OpenBSD `pledge()` & `unveil()`**: System call domain reduction and path boundary restrictions.
  * **Linux Landlock v5**: Unprivileged filesystem hierarchy sandboxing.
  * **FreeBSD Capsicum**: Capability-mode file descriptor access rights (`cap_rights_limit`).
  * **SELinux / AppArmor**: Context-based mandatory access control (MAC).

### 2. **Input Validation & Sanitization**
- Perform single-pass $O(N)$ byte validation on all path strings, shell inputs, and network frame headers.
- Path traversal sequences (`..`, `...`, URL-encoded `%2e%2e`) must be actively rejected.

---

## 🏎️ Performance & Latency Guidelines

- **Zero-Allocation Hot Paths**: Critical kernel loops, packet routers, and scheduler dispatches must avoid dynamic allocations in hot paths.
- **NUMA-Aware Schedulers**: Schedulers must respect NUMA core affinity and ACPI SLIT distance matrices.
- **Tickless Timer Wheel**: High-resolution timers must utilize `SovereignHrtimerWheel` to avoid unnecessary timer interrupts.

---

## 📋 Pull Request (PR) Workflow & Checklist

All pull requests submitted to SigmaOS must satisfy the following checklist before merge:

### PR Requirement Checklist
- [ ] **Zero Dependency Compliance**: Confirmed `Cargo.toml` contains zero external crates.
- [ ] **Standalone Test Pass**: The modified module compiles and passes standalone testing:
  ```bash
  mkdir -p build && rustc --test --edition=2021 <path_to_file>.rs -o build/test_mod && ./build/test_bin
  ```
- [ ] **Master Test Suite Pass**: Executed `./run_sigma_tests.sh` with 100% pass rate across all 186+ test suites.
- [ ] **Format Verification**: Ran `cargo fmt` or confirmed clean Rust formatting.
- [ ] **Git Commit Hygiene**: Commit message follows standard conventions (subject <= 50 chars, blank line, detailed body).

---

*Document Version: 1.0.0*
*Classification: Engineering & Development Rules*
*Target Project: SigmaOS*
