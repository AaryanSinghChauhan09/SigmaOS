# SigmaOS Kernel Test Taxonomy

## Overview

SigmaOS classifies tests into 7 distinct tiers:

| Test Type | Purpose | Coverage & Location | Required For |
|---|---|---|---|
| **Unit** | Pure logic and data structures | `src/` inline modules, `tests/` | All kernel modules |
| **Property** | Invariants under generated inputs | `tests/` property tests | Allocators, VFS, syscall parsing |
| **Integration** | Cross-subsystem behavior | `tests/kernel/integration/` | Scheduler, memory, VFS, syscalls |
| **Boot Smoke** | Firmware-to-shell behavior | `scripts/ci/qemu_boot_smoke.sh` | All boot-affecting PRs |
| **Fuzz** | Malformed input resilience | `fuzz/fuzz_targets/` | Parsers, loaders, drivers, syscalls |
| **Stress / Race** | Resource exhaustion and races | `tests/stress_and_fuzz_tests.rs` | Memory, IPC, networking, scheduler |
| **Fault Injection** | Failure and recovery behavior | `installer/tests/failure_injection.rs` | Storage, swap, installer, drivers |
