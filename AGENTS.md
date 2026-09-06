# AGENTS.md — Security & Architectural Instructions for AI Agents in SigmaOS

Welcome, Autonomous AI Agent (Jules, Sentinel, Bolt, Palette, or Subagent). This document establishes mandatory security standards, architectural rules, coding guidelines, and verification workflows for modifying or contributing code to the **SigmaOS** repository.

---

## 1. Core Directives & Guiding Security Principles

All AI agents working on SigmaOS must strictly adhere to the following principles:

1. **Bare-Metal Zero-Dependency OOP Purity**:
   - Primary kernel and system components MUST maintain `#![no_std]` compatibility wherever applicable.
   - Use `alloc::boxed::Box`, `alloc::vec::Vec`, `alloc::string::String`, and `crate::klib` for data structures.
   - External dependencies MUST NOT be introduced without explicit security review and justification.

2. **Least Privilege & Sandboxing (OpenBSD `pledge` & `unveil` Rule)**:
   - System tools, userland utilities, and shell processes MUST restrict file access via `unveil(2)` paths and restrict syscall capabilities via `pledge(2)` promises (e.g., `stdio rpath wpath cpath inet`).
   - Default policy is **strict deny-all** unless explicitly permitted by an authorized security profile.

3. **Zero Hardcoded Credentials / Secrets**:
   - **NEVER** insert hardcoded passwords, tokens, API keys, or private cryptographic keys into source files (`.rs`, `.py`, `.cpp`, `.yml`).
   - Mock variables in test suites MUST use `mock`, `test`, `example`, or `TODO` in variable names to prevent false positives in hardcoded secret scanners.

4. **Post-Quantum Cryptography (PQC) & Hardware Enclaves**:
   - Critical key exchange and signature verification MUST support Dilithium-5 and Kyber PQC primitives alongside TPM 2.0 PCR attestation gates.

5. **Amnesic Memory Scrubbing**:
   - All sensitive data structures (session tokens, keys, passwords) MUST be securely zeroized/scrubbed from memory upon deallocation or shutdown (taking inspiration from Tails OS and Qubes OS isolation).

---

## 2. Coding Standards & Architectural Guidelines

- **Rust Edition & Toolchain**:
  - Target edition is **Rust 2021**.
  - All CI workflows MUST specify `with: toolchain: stable` for `dtolnay/rust-toolchain`.
- **Deduplication Rule**:
  - Before declaring a new struct or impl, search existing files (`src/distro/`, `src/sigpkg/`, `src/shell/`, `src/compatibility/`) to avoid duplicate type namespace definitions (`E0428`, `E0592`, `E0119`).
- **Paging & Alignment Safety**:
  - Page table indexing, DMA ring allocations, and memory buddy alignment MUST be verified for 4KiB page boundary compliance.
- **Error Handling**:
  - Avoid raw panics (`unwrap()`, `expect()`) in kernel/subsystem code. Return explicit `Result<T, &'static str>` or error enums.

---

## 3. Verification & Testing Directives

- **Immediate Change Verification**:
  - After editing or creating any file, verify changes using `read_file` or `git status`.
- **Test Execution Suite**:
  - Run `./run_sigma_tests.sh` to execute the full Rust (224 tests), C++ (40 tests), and Python (14 tests) validation suite.
  - To test standalone Rust files: `rustc --test --edition 2021 src/<path>/<file>.rs -o build/<test_bin> && ./build/<test_bin>`.
  - All test artifacts MUST be compiled into `build/` (never `/tmp/`).

---

## 4. Pre-Commit Verification Gate

Before submitting any Pull Request or change:
1. Run `./run_sigma_tests.sh` and ensure **100% pass rate**.
2. Run `cargo check --lib` to verify zero library compilation errors.
3. Call `pre_commit_instructions` and follow all instructions.
