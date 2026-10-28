# SigmaOS Kernel Test Taxonomy

## Overview
SigmaOS enforces a 7-tier test taxonomy to ensure that pure logic, invariants, integration boundaries, boot behavior, fuzz resilience, concurrency, and fault recovery are all independently verified.

---

## The 7 Test Tiers

| Test Tier | Purpose | Target Subsystems | Execution Harness |
|---|---|---|---|
| **1. Unit** | Pure logic and data structures in isolation | All kernel modules, allocators, data structures | `cargo test --lib`, `rustc --test` |
| **2. Property** | Invariants verified across generated randomized inputs | Allocators, VFS path parsing, ELF loader, range math | `cargo test` with property generators |
| **3. Integration** | Cross-subsystem interactions & contracts | Memory + Paging + Scheduler + VFS + Syscalls | `tests/kernel/integration/` |
| **4. Boot Smoke** | Firmware-to-shell validation in emulator | Full kernel image + Limine + QEMU | `scripts/ci/qemu_boot_smoke.sh` |
| **5. Fuzzing** | Malformed external input resilience | Syscall arguments, ELF parsers, VFS paths, packets | `cargo fuzz`, `fuzz/fuzz_targets/` |
| **6. Stress & Concurrency** | Resource exhaustion, race detection, lock contention | Physical allocator OOM, swap queues, run-queue lock | Multithreaded integration harnesses |
| **7. Fault Injection** | Failure recovery and crash isolation | Storage disconnects, allocation failure simulation | Fault injection test suites |

---

## Enforcement Rules

1. Every new kernel feature or fix requires a corresponding Unit test.
2. Parsers handling external or untrusted data MUST have a dedicated Fuzz target.
3. Subsystem interactions must pass Integration tests before PR merge.
4. Boot-affecting changes MUST pass the QEMU Boot Smoke test.
