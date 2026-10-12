# SigmaOS Kernel Test Taxonomy

## Overview

SigmaOS classifies kernel testing into seven distinct tiers to ensure comprehensive coverage across unit logic, system integration, runtime security, stress resilience, and hardware simulation.

---

## Test Categories

| Test Type | Purpose | Scope / Target | Automation & Tooling | Required Gate |
|---|---|---|---|---|
| **Unit** | Pure logic, algorithm correctness, and state mutations | Isolated data structures (`BoundedRunQueue`, `InodeManager`, `PidAllocator`) | `rustc --test` | Gate G0 / PR |
| **Property** | Invariants under randomly generated inputs | Memory allocators, path normalization, packet parsing | `cargo-fuzz` / quickcheck patterns | Gate G0 / PR |
| **Integration** | Cross-subsystem interaction and state transitions | Scheduler + Timer, VFS + Block Storage, Syscall Dispatcher + Capabilities | Standalone test runners (`tests/kernel/integration/`) | Gate G1 / Biweekly |
| **Boot** | Firmware-to-shell boot state machine progression | ISO image loading, QEMU boot-to-shell execution | `scripts/ci/qemu_boot_smoke.sh` | Gate G3 / Monthly |
| **Fuzz** | Malformed input resilience and memory error discovery | Syscall arguments, ELF parser, VFS path traversal, USB descriptors | `fuzz/fuzz_targets/` (libFuzzer) | Gate G1 / Monthly |
| **Stress** | Multi-threaded race detection, deadlock checks, and OOM handling | Physical memory frame exhaustion, rapid fork/exec, lock contension | Targeted stress benchmarks | Gate G1 / Biweekly |
| **Fault Injection** | Failure recovery and graceful degradation testing | Corrupted block reads, memory allocation failures, hardware reset | `installer/tests/failure_injection.rs` | Gate G4 / Quarterly |

---

## Minimum Enforcement Requirements

1. **No Model-Only Completion:** A kernel module cannot be marked complete without passing integration or boot validation.
2. **Regression Standard:** Every resolved bug requires an associated unit or integration test added to the regression test suite.
