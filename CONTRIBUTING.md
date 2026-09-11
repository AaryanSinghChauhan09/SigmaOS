# 🤝 Contributing to SigmaOS: Guidelines for Human Contributors & AI Agents

Thank you for contributing to **SigmaOS**! This document provides comprehensive contribution rules, development policies, and task guidelines for both human developers and autonomous AI engineering agents.

---

## Core Rules for Contributors & AI Agents

### 1. **Zero-Dependency `#![no_std]` Architecture**
- **Strict Self-Sufficiency**: All core kernel and userspace components in `src/` must maintain strict `#![no_std]` zero external crate dependency architecture.
- **No Unverified External Crates**: Do NOT add third-party crates to `Cargo.toml`. Utilize native Rust primitives or `alloc::` types (`alloc::vec::Vec`, `alloc::string::String`, `alloc::format!`, `alloc::collections::BTreeMap`).
- **Memory Safety First**: All code must prioritize safe Rust. Any `unsafe` blocks required for hardware MMIO or driver interaction must clearly document safety invariants.

### 2. **Mandatory Git Branch Naming Convention**
- **Branch Naming Rule**: All git branches created for features, fixes, refactoring, or docs MUST start with the `jules-` prefix (e.g., `jules-scheduler-ule-scoring`, `jules-fix-package-adapter`, `jules-fedora-compatibility`).

### 3. **Linux & BSD Distribution Engineering Guidelines**
- **Cross-Distro Interoperability**: Components taking inspiration from Linux and BSD distributions (Arch ALPM, Debian sbuild, Fedora DNF, Gentoo Portage, CachyOS BORE, FreeBSD Ports/Capsicum, OpenBSD Pledge/Unveil, NixOS Flakes/CAS) must maintain clean interfaces and support cross-subsystem event routing.
- **Security Sandboxing**: Application modules must declare sandboxing bounds using OpenBSD `pledge()`/`unveil()`, Linux Landlock v5, FreeBSD Capsicum capabilities, or Fedora SELinux MLS/MCS rules.

### 4. **Testing, Autonomous Verification & Pre-Commit Protocols**
- **Standalone Module Testing**: Modified files must be verified using standalone unit test compilation:
  ```bash
  rustc --edition=2021 --test <file_path> -o build/test_bin && ./build/test_bin
  ```
- **Master Test Runner Execution**: Before submitting any pull request or finalizing AI agent turns, execute `./run_sigma_tests.sh` to verify all test stages pass cleanly.
- **AI Agent Pre-Commit Protocol**: AI agents must execute `pre_commit_instructions`, verify test binaries, obtain code review confirmation (`request_code_review`), record learnings (`initiate_memory_recording`), and submit via `submit`.

### 5. **Code Review & Quality Assurance**
- Every pull request requires double maintainer review.
- PR commit messages must follow standard conventions: short subject line (50 chars max), blank line, and descriptive body outlining changes and testing results.
