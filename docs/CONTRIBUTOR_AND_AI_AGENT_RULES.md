# 📜 SigmaOS Contributor & AI Agent Execution Rules

This document outlines the mandatory rules, development policies, and execution standards for both human contributors and autonomous AI engineering agents working on the SigmaOS codebase.

---

## Executive Summary & Core Directives

1. **Self-Contained `#![no_std]` Codebase**: SigmaOS is designed to be fully self-sufficient with zero third-party dependencies in `Cargo.toml`.
2. **Standardized Branching**: All work must occur on branches named `jules-*`.
3. **Multi-Distro Interoperability**: Every subsystem must support cross-distribution event routing and compatibility across 25+ Linux and BSD distribution modes.
4. **Mandatory Testing & Verification**: Every change must pass standalone unit tests and the master test runner `./run_sigma_tests.sh`.

---

## 1. Zero-Dependency & `#![no_std]` Architecture Directives

1. **Strict Zero Third-Party Crates:**
   - SigmaOS is built with a zero-dependency architecture.
   - Contributors and AI agents MUST NOT add external third-party crates to `Cargo.toml`.
   - All data structures, algorithms, drivers, and OS services must be implemented natively or using `alloc::` primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::format`).

2. **C-Language and Shell Script Elimination:**
   - Prefer native, memory-safe Rust implementations for kernel components, drivers, and utilities over legacy C code or external shell scripts.

---

## 2. Git Branch Naming & Subsystem Guidelines

1. **Mandatory Branch Prefix:**
   - All Git branches created by developers or AI agents MUST use the `jules-` prefix (e.g., `jules-kernel-psi-improvements`).

2. **Linux & BSD Parity & Interoperability:**
   - Subsystem features should incorporate paradigms from major Linux distributions (Arch, Debian, Fedora, Gentoo, CachyOS, Alpine, NixOS, Solus, Void) and BSD systems (FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Illumos/SmartOS).
   - Security features must bridge Linux Landlock v5 with FreeBSD Capsicum capability rights, OpenBSD pledge/unveil restrictions, and Fedora SELinux MLS/MCS contexts.

---

## 3. Testing, Verification, and Quality Assurance

1. **Autonomous Self-Verification:**
   - After making any code changes, contributors and AI agents must run relevant tests.
   - Run the full test suite via `./run_sigma_tests.sh`.
   - Standalone Rust files containing unit tests must be verified using `rustc --edition=2021 --test <file_path> -o build/test_bin && ./build/test_bin`.

2. **Pre-Commit Verification Checklist:**
   - Ensure zero warnings or syntax errors.
   - Execute `pre_commit_instructions` tool when working as an AI agent.
   - Confirm all standalone test binaries compile and pass 100%.

---

## 4. Code Review and Documentation Standards

1. **Documentation Updates:**
   - Any new subsystem or kernel feature must be documented and re-exported in `src/lib.rs` and the corresponding module `mod.rs`.
   - Architectural decisions and agent directives must be referenced in `AGENTS.md` and `docs/RULES.md`.

2. **Review Feedback Loop:**
   - Address all code review annotations prompt by re-verifying test execution and ensuring strict adherence to the project's whitepaper and contributor charter.
