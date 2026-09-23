# SigmaOS Next Steps Guidelines & Development Roadmap

## Overview & Architecture Goals
This document outlines the canonical operational guidelines and next steps roadmap for **SigmaOS** (`https://github.com/AaryanSinghChauhan09/SigmaOS/`). SigmaOS combines the security, performance, and modularity of Linux v6.8+ and BSD distributions (FreeBSD 14, OpenBSD 7.4, NetBSD 10) into a sovereign, ultra-resilient operating system microkernel and userspace ecosystem.

---

## 🎯 Strategic Priorities Hierarchy
1. **Security (Primary):** Fail-secure design, post-quantum cryptography (Dilithium-5/Kyber-1024), default-deny execution policies (`ZorinExecGuardPolicyEngine`), and strict sandboxing (`Pledge`/`Unveil`/`Landlock`).
2. **Stability (Secondary):** Zero runtime panics, sub-second Dual-Root A/B CoW snapshot rollbacks, watchdog hardware protection, and 100% test passing rates.
3. **Performance (Tertiary):** Lock-free SPSC queue ring buffers, zero-copy socket/VFS splicing, SIMD memory copies, and $O(1)$ constant-time data structure lookups.

---

## 📋 Comprehensive Task List & Guidelines

### 1. Code Quality & Testing Guidelines
- **Zero Syntax & Warning Policy:** Maintain `#![deny(warnings)]` across release profiles.
- **Modular Refactoring:** Decompose large files exceeding 1,000 lines (e.g. `src/package/universal.rs`) into clean sub-modules (`mod.rs`, `strategy.rs`, `adapter.rs`, `observer.rs`).
- **Comprehensive Unit & Integration Testing:**
  - Execute `pytest tests/` for Python integration and stress fuzzing.
  - Run `rustc --test --edition=2021` standalone suites for kernel, security, and packaging modules.

### 2. Performance & Optimization Guidelines
- **Allocation Elimination in Hotpaths:** Operate directly on borrowed slices (`&str`, `&[u8]`) during string/log parsing, JSON serialization, and package conflict scans.
- **Power-of-Two Masking:** Use bitwise AND indexing (`hash & (capacity - 1)`) for fixed-size hash tables and ring buffers.
- **Hoisting Outer Map Queries:** Eliminate quadratic loop overhead by hoisting outer collection lookups in pair comparison routines.

### 3. Security & Compliance Standards
- **Zero Secrets Trace:** Secrets must never be stored in plaintext. Derive passwords dynamically using Argon2/Dilithium or seal in TPM 2.0 PCR registers.
- **Strict Boundary Path Validation:** Reject multi-dot path traversal variations (`...`, `....`), NUL bytes, and ASCII control characters (`< 32` or `127`) in all path inputs.
- **Regulatory Standards:** Maintain full compliance with WCAG 2.1 AA (a11y focus rings & live regions), GDPR/HIPAA (encrypted VFS enclaves), and ISO 27001 (PQC signatures).

### 4. Object-Oriented Design & Pattern Standards
- **Encapsulation:** Hide mutable state behind strictly validated getter/setter interfaces.
- **Polymorphism & Abstraction:** Implement trait-based strategies (`InstallStrategy`, `ExecutableFormatRunner`) to allow transparent multi-distro and multi-architecture extensions.
- **Design Patterns:** Apply Singleton, Factory, Observer, Command, Decorator, and Strategy design patterns consistently across new userland and kernel systems.

---

## 🚀 Chronological Next Steps Roadmap

### Phase 1: Modular Packaging & CI Pipeline Hardening
- [x] Integrate `PackagePullRequestParser` and `PullRequestPackageSpec` for transpiling community package PRs (AUR, Ebuild, Ports, Nix Flakes) into native `UnifiedPackage` specs.
- [ ] Refactor `src/package/universal.rs` into `src/package/universal/` sub-module directory.
- [ ] Expand automated GitHub Actions CI runners to validate PR package transpilation on every commit.

### Phase 2: Post-Quantum Security & Boot Unification
- [x] Deploy post-quantum Dilithium-5 / SHA3 package signature verifier in `src/sigpkg/verifier.rs`.
- [ ] Connect TPM 2.0 PCR sealed secret unlocking to `src/kernel/boot_foundations.rs`.
- [ ] Integrate `ZorinExecGuardPolicyEngine` capability checks directly into binary loader execution pipelines.

### Phase 3: Desktop UX Polish & Accessibility
- [x] Implement ARIA-compliant, high-contrast installer setup wizard in `src/installer/gui_wizard.rs`.
- [ ] Add global `Escape` key overlay dismiss listeners and skip-to-content links across Zenith desktop Web UI components.
- [ ] Enhance contrast ratio indicators for custom desktop GTK/Qt theme palettes.

---

## 🛠️ Direct Commit Policy (No Pull Requests)
Per repository guidelines, **all master improvement plans, architectural blueprints, and operational handbook updates must be committed directly to the `main` branch**. Do not open pull requests (PRs) against this repository.
