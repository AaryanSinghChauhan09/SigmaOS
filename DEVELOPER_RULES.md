# 📋 SigmaOS Developer Rules & Task Guidelines

**Version:** 2.0.0
**Last Updated:** September 2026  
**Scope:** All developers, maintainers, contributors, and AI agents

---

## Table of Contents

1. [Core Principles](#core-principles)
2. [Distribution Engineering Task Guidelines](#distribution-engineering-task-guidelines)
3. [Development Workflow](#development-workflow)
4. [Code Standards](#code-standards)
5. [Security & Safety](#security--safety)
6. [Testing & Verification](#testing--verification)
7. [Documentation](#documentation)
8. [Special Interest Groups (SIGs)](#special-interest-groups-sigs)
9. [AI Agent Guidelines](#ai-agent-guidelines)
10. [Review & Approval Process](#review--approval-process)

---

## Core Principles

### 1. **Sovereignty & Zero Dependencies**
- **No External Unverified Dependencies**: Core kernel and userspace modules must use Rust's `#![no_std]` with explicit capability bounds.
- **Self-Sufficiency**: Avoid external crates under `[dependencies]` in `Cargo.toml`.
- **Minimal Surface**: Each component should have the smallest possible attack surface.

### 2. **Memory Safety & Security-First**
- **Prefer Safe Rust**: Memory safety is non-negotiable.
- **Unsafe Blocks**: Only use `unsafe` for hardware interaction, driver development, or low-level OS primitives.
- **Document Invariants**: Every `unsafe` block must document all safety invariants.
- **Post-Quantum Cryptography**: All cryptographic operations must use Kyber-1024 (KEM) or Dilithium-5 (signatures).

---

## Distribution Engineering Task Guidelines

Inspired by Linux & BSD distribution development standards (Arch Linux, Debian, Fedora, Alpine, Gentoo, NixOS, FreeBSD, OpenBSD):

### 1. **Arch Linux PKGBUILD & ALPM Purity Standards**
- **Cleanroom Package Recipes**: Maintain PKGBUILD and PKGINFO array variable purity (`depends`, `makedepends`, `provides`, `conflicts`, `sha256sums`).
- **ALPM Topological Resolution**: Guarantee acyclic dependency graph traversal with topological sorting and explicit cycle detection in package managers.
- **AUR Audit Gating**: All third-party package build recipes must pass automated PKGBUILD safety audits before compilation.

### 2. **Debian sbuild & Pristine-Tar Reproducible Build Guidelines**
- **Determinism**: Enforce `SOURCE_DATE_EPOCH` environment variables, zero-timestamp tar header normalization, and canonical directory sorting.
- **Cleanroom Chroots**: Package compilation must execute inside isolated ephemeral build containers (`sbuild` / `poudriere` cleanroom jails).
- **Bit-for-Bit Verification**: Build outputs must be validated using `ReproducibleBuildRecord` diff hashes against published SBOM records.

### 3. **FreeBSD Ports & Poudriere QA Directives**
- **Pre-flight QA Testing**: Port builds must verify stage directory execution (`stage-qa`), test for leftover temporary files (`check-orphans`), and validate shared library dependencies (`lib-depends`).
- **Capsicum Capability Sandboxing**: Desktop applications and utilities must delegate file descriptor capability rights using FreeBSD Capsicum interfaces (`cap_rights_init`).

### 4. **OpenBSD Pledge/Unveil & Syspatch Security Directives**
- **Strict Sandbox Declarations**: Every userspace binary must call OpenBSD `pledge()` to restrict syscall capabilities and `unveil()` to lock down filesystem visibility immediately upon entry.
- **Fastpath Errata Patching**: Maintain atomic kernel live-patching and userland errata update compatibility (`syspatch` parity).

### 5. **NixOS Declarative State & Hermetic CAS Store Directives**
- **Merkle Closure Store**: Software builds must be addressed by input hashes inside a content-addressed storage (CAS) store.
- **Atomic State Hot-Swapping**: System configuration updates and package state transitions must support sub-millisecond atomic generation rollbacks.

---

## Development Workflow

### 1. **Branch Strategy**

**Mandatory Branch Naming Convention:**
- All developer and AI agent branches MUST start with the `jules-` prefix followed by descriptive task text (e.g. `jules-feat-kernel-scheduler`, `jules-fix-hotkey-binding`).

### 2. **Commit Guidelines**

- Short subject line (50 chars max), blank line, detailed body.
- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `security`, `perf`.

---

## Testing & Verification

### 1. **Standalone Test Runner**
- Every Rust file modified in `src/` must be verifiable via standalone unit test compilation:
  `rustc --edition=2021 --test <filepath> -o /tmp/test_bin && /tmp/test_bin`

### 2. **Master Test Suite Execution**
- Before submitting changes, execute the native master test runner `./run_sigma_tests.sh` and ensure all 13 test runner stages pass cleanly.

---

## AI Agent Guidelines & Directives

### 1. **Autonomous Operation & Verification**
- AI agents operating on SigmaOS must autonomously diagnose build and test failures before changing package configurations or dependencies.
- Every state-modifying action (file edits, creations, or deletions) MUST be verified immediately using read-only inspection tools or standalone test compilation.

### 2. **Planning & Review Protocols**
- AI agents must formulate clear numbered plans and call `request_plan_review` prior to setting plans or making broad architectural modifications.
- AI agents must execute pre-commit checklists, including `pre_commit_instructions`, before submitting pull requests.

### 3. **Memory & Knowledge Recording**
- AI agents must document critical, codebase-specific performance, security, or architectural insights using `initiate_memory_recording`.
- AI agents must consult the internal knowledgebase (`knowledgebase_lookup`) when facing ambiguous system behavior or setup issues.

---

## Rules for Contributors

1. **Branch Naming**: All branches created by contributors or agents MUST start with `jules-` (e.g. `jules-feature-xxx`).
2. **Zero Dependency Integrity**: Never add unverified external crates to `Cargo.toml`.
3. **Safe Rust First**: Do not introduce `unsafe` blocks without documented safety invariants and explicit review.
4. **Verification**: Always execute `./run_sigma_tests.sh` and confirm all tests pass prior to submitting PRs.

---

**Last Updated:** September 2026
**Maintained By:** SigmaOS Core Architecture Team
