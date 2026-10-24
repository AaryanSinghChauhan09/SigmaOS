# SigmaOS Kernel Test Taxonomy

This document defines the 7 distinct test tiers required for verifying the SigmaOS kernel codebase.

## Test Tier Overview

| Test Tier | Purpose | Target Coverage | Required For |
|---|---|---|---|
| **Unit** | Pure logic, data structures, and state transitions | Individual functions and structs | All kernel modules |
| **Property** | Invariant validation under generated inputs | Boundary conditions, algorithms | Allocators, VFS path parsing, Syscall ABI |
| **Integration** | Multi-subsystem cross-boundary interaction | Subsystem contracts and IPC | Scheduler, Memory, VFS, Syscalls |
| **Boot** | Full firmware-to-shell end-to-end execution | 11-stage boot state machine | All boot-affecting changes |
| **Fuzz** | Robustness against malformed/untrusted inputs | Parsers, loaders, decoders | Syscalls, ELF, Filesystems, Drivers |
| **Stress** | Stability under resource exhaustion and concurrency | Race conditions, memory leaks, OOM | Memory, IPC, Sockets, Scheduler |
| **Fault Injection** | Failure handling and recoverability | Error recovery and rollbacks | Storage, Swap, Installer, Driver hotplug |

---

## Detailed Tier Specifications

### 1. Unit Tests
- Executed via `rustc --test`.
- Must execute in `#![no_std]` compatible environments or mock harnesses.
- Example: Testing BoundedRunQueue priority bitmap logic.

### 2. Property Tests
- Generate arbitrary input sequences to assert invariants (e.g., `alloc_frame` followed by `free_frame` never leaks or double-frees).
- Example: VFS path normalization invariants.

### 3. Integration Tests
- Test cross-module contracts (e.g., Syscall dispatcher invoking memory page mapping and process table update).
- Example: `tests/kernel/integration/subsystem_contracts_test.rs`.

### 4. Boot Smoke Tests
- Executed in QEMU x86_64 emulator.
- Must reach `shell_ready` marker within timeout limits.
- Example: `scripts/ci/qemu_boot_smoke.sh` and `tests/boot/boot_to_shell_test.py`.

### 5. Fuzzing
- Automated fuzz targets in `fuzz/fuzz_targets/`.
- Must cover untrusted user input entrypoints.
- Example: ELF binary loader fuzzing.

### 6. Stress Tests
- Simulate max process counts, swap saturation, and concurrent thread contention.
- Assert zero kernel deadlock or memory exhaustion panics.

### 7. Fault Injection Tests
- Simulate block driver read errors, corrupt disk headers, and allocator failures.
- Assert kernel gracefully handles error codes without unhandled panics.
