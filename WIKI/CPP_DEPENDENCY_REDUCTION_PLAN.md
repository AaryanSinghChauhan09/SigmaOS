# SigmaOS C++ Programming Language Dependency Reduction Plan

## 1. Executive Overview
SigmaOS aims to maximize memory safety, zero-dependency reliability, and compilation predictability by systematically reducing and migrating legacy C/C++ components to pure, idiomatic Rust. This plan details the strategy for replacing legacy C++ orchestrator shims, hardware driver wrappers, and userland CLI tools with zero-dependency native Rust implementations (`klib`, `src/tools/sigma_cli.rs`, `src/kernel/`, `src/drivers/`).

## 2. Current State & C++ Dependency Assessment
A repository analysis reveals legacy C/C++ files located in:
- `./orchestrator/main.cpp`
- `./sigmaos/core/src/*.cpp`
- `./kernel/core/*.cpp`
- `./kernel/drivers/*.cpp`
- `./userland/*.cpp`
- `./suites/*.cpp`

While these legacy files provided early C++ prototype wrappers during initial development phases, the native Rust kernel (`src/kernel/`), multi-architecture HAL (`src/hal/`), hardware drivers (`src/drivers/`), and master CLI (`src/tools/sigma_cli.rs`) in `src/` now provide complete, memory-safe, zero-dependency Rust implementations.

## 3. Migration Strategy & Phase Milestones

### Phase 1: Native Rust Master CLI & Fast-Path WASM Hostcalls
- **Master CLI Consolidation**: The unified master CLI (`src/tools/sigma_cli.rs`) written in pure Rust replaces legacy C++ orchestrators (`userland/tools/zenith_build.cpp`, `orchestrator/main.cpp`).
- **WASM Hostcall Interface**: System administration operations execute via native Rust WASM hostcall fast-paths, eliminating external C++ runtime dependencies.

### Phase 2: Native Driver & HAL Rust Substitution
- **NVMe & AHCI Drivers**: `src/drivers/modern_nvme.rs` and `src/drivers/modern_audio_intel_hda.rs` replace legacy C++ driver stubs.
- **Multi-Arch HAL**: `src/hal/multi_arch.rs` provides native Rust IRQ and MMIO dispatching across x86_64, AArch64, and RISC-V, deprecating `suites/S04_HAL/*.cpp`.

### Phase 3: Zero-Dependency `klib` & Memory Safety
- **Core Library Autonomy**: Custom collections (`BTreeMap`, `HashMap`, `Vec`), string parsers (`SigmaString`), and time primitives in `src/klib/` provide standard library parity without external C/C++ dynamic library linkage.
- **Guaranteed Memory Safety**: Replaces C++ manual pointers and raw memory management with Rust compile-time ownership, lifetime checking, and atomic safe wrappers.

---
*Maintained by the SigmaOS Architecture & Zero-Dependency Migration Steering Committee.*
