# SigmaOS Contributor & AI Agent Development Rules (`docs/AGENTS_CONTRIBUTOR_DEVELOPMENT_RULES.md`)

This document defines core engineering standards, development workflows, and AI agent operational directives for contributing to the SigmaOS project.

---

## 1. Golden Rules of Development

1. **Strict `#![no_std]` & Zero Third-Party Dependencies:**
   - Kernel, `klib`, driver, and core subsystem components must remain strictly `#![no_std]` compliant.
   - Do NOT introduce external crate dependencies under `[dependencies]` in `Cargo.toml`.
   - Utilize `alloc::` primitives (`alloc::vec::Vec`, `alloc::string::String`, `alloc::format`) or custom `klib` zero-dependency data structures.

2. **Object-Oriented & Modular Design Patterns:**
   - Organize new features using established software design patterns:
     - **Factory Pattern:** For instantiating package adapters, drivers, and process handles.
     - **Adapter Pattern:** For wrapping cross-distro compatibility layers (e.g., APT, RPM, Pacman, Portage, XBPS).
     - **Observer Pattern:** For dispatching system events, fedmsg messages, and UI telemetry updates.
     - **Singleton / Static Manager:** For system-wide governors, memory allocators, and hardware monitors.

3. **Memory Safety & Safety Invariants:**
   - Minimize `unsafe` code blocks. All `unsafe` memory shifts or raw pointer dereferences must include explicit `// SAFETY:` comments.
   - For intra-array shifts (e.g. element insertion/removal), use `core::ptr::copy` (memmove) to prevent undefined behavior when ranges overlap.

4. **Multi-Distro & Multi-Architecture Parity:**
   - Features inspired by Linux or BSD distributions (Debian, Fedora, Arch, FreeBSD, OpenBSD, NetBSD, Void, Alpine, etc.) must integrate into the universal distro bridge (`SovereignUniversalDistroBridge`).
   - Hardware abstractions must support target CPU HAL architectures (`x86_32`, `x86_64`, `AArch32`, `AArch64`, `RISC-V`, `LoongArch64`, `PowerPC64`).

---

## 2. Code Quality & Verification Checklist

Every pull request or commit must satisfy:
- **Build Cleanliness:** `cargo check` and `cargo check --lib` compile with 0 errors.
- **Code Formatting:** Code passes `cargo fmt` without unexpected formatting diffs.
- **Testing:** Native tests pass cleanly via `./run_sigma_tests.sh`. Standalone module test suites verify via `rustc --test`.
- **Pre-Commit Verification:** Run `pre_commit_instructions` and ensure all code review and learning recording steps are completed.

---

## 3. Directives for Autonomous AI Agents

- **Proactive Verification:** After making any code or documentation modification, verify file contents using read-only tools (`read_file`, `list_files`).
- **Comprehensive Coverage:** When implementing missing features or parity specifications from roadmap documents, accompany code changes with complete unit test cases.
- **Memory & Journal Recording:** Document critical codebase learnings in `.jules/` journal logs for future session context.
