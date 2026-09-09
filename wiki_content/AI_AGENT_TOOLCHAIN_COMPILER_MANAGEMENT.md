# AI Agent Toolchain & Compiler Management Guidelines

## 1. Overview & Architecture
This document specifies operational protocols, development rules, and maintenance standards for AI agents managing the native compiler toolchain, C++-to-Rust conversion engine, pure Rust assembler/linker, and self-hosted build pipelines in SigmaOS (`src/distro/transformation_engine.rs`, `src/tools/`).

---

## 2. AI Agent Toolchain Protocols

### 2.1 Pure Rust Native Toolchain
- **Zero C/C++ Compiler Dependencies**: AI agents must ensure that all native toolchain tools operate entirely in memory-safe, `#![no_std]` capable Rust without calling external `gcc` or `clang` binaries.
- **Assembler & Linker Integration**: Native assembly generation (x86_64, AArch64, RISC-V) and ELF binary linking (`ld` equivalent) must be handled by pure Rust symbol resolution and section layout engines.

### 2.2 Self-Hosted Build Verification
- **Self-Hosting Milestone**: AI agents maintaining compiler passes must verify that SigmaOS can compile its own core kernel, userland tools, and package manager without relying on host operating system toolchains.
- **Hermetic Build Sandboxing**: All toolchain execution must be wrapped in isolated chroot or container sandboxes (`src/container/distro_sandbox.rs`).

---

## 3. Related Files
- `src/distro/transformation_engine.rs`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`
- `docs/DEVELOPER_RULES.md`
