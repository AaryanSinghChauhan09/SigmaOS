# Contributor and AI Agent Development Rules for SigmaOS

## Executive Summary

This document establishes the mandatory engineering standards, architectural rules, and verification protocols for **human contributors** and **AI coding agents** developing and maintaining **SigmaOS**.

---

## Core Engineering Rules & Mandates

### 1. Zero External Dependency Mandate (`klib`)
- All kernel, system, and userland code must be written in **pure safe Rust** (`#![no_std]`).
- External C libraries (`libc`, `malloc`, `free`), Python runtimes, Node.js V8, or unverified crates are strictly prohibited.
- Use native `klib` primitives in `src/klib/` for string parsing, hashing, cryptography, data structures, and memory allocation.

### 2. Kernel ABI (KABI) Binary Layout Stability
- Kernel exports and syscall structures (`src/kernel/exports.rs`) must maintain backward binary layout compatibility.
- Never reorder, delete, or alter struct field offsets in public KABI headers.
- Always run KABI compliance unit tests before submitting changes.

### 3. Pre-Commit Self-Testing & Verification
- Before submitting any code change, you **MUST** run the full test suite:
  ```bash
  ./run_sigma_tests.sh
  ```
- All 13 test stages (Python integration, Package Caching, Universal Adapter, Unimplemented Features/Tools, Open Source Gap Closure, Arch, Fedora, UI/UX Benchmarks, CLI simulation, and Parity Inspection) must pass with **0 failures**.

### 4. Zero-Drift Documentation Mirroring
- Whenever documentation is added or modified in `docs/` or `wiki/`, it **MUST** be mirrored to `wiki_repo/`.
- Landing page indices (`wiki/Home.md` and `wiki_repo/Home.md`) must be updated with valid relative links.

### 5. Memory Recording Protocol for AI Agents
- AI coding agents must call `initiate_memory_recording` upon completing code reviews, bug fixes, or architecture implementations to document key learnings and repository patterns.
