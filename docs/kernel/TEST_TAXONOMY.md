# Kernel Testing Taxonomy

This document outlines the required test taxonomy and quality assurance tiers for SigmaOS kernel code.

---

## Testing Tiers

| Test Type | Purpose | Scope | Execution Requirement |
|---|---|---|---|
| **Unit Test** | Pure logic, data structures, and state transitions | Isolated functions and modules | Ran via `cargo test --lib` and standalone test binaries on every PR |
| **Property Test** | Validates structural invariants under randomized generated inputs | Allocators, VFS path parsing, Syscall ABI decoding | Ran via property/fuzz suites during biweekly audits |
| **Integration Test** | Verifies cross-subsystem interactions and multi-component behavior | Scheduler + Memory, VFS + Lock Manager, Syscall + Process Table | Ran via integration test harness (`tests/kernel/integration/`) |
| **Boot Test** | Validates firmware-to-shell boot execution in QEMU | Entire kernel image, init path, and shell prompt | Executed monthly or on boot-affecting PRs via `scripts/ci/qemu_boot_smoke.sh` |
| **Fuzz Test** | Detects crashes, memory errors, and panics on untrusted input | ELF parser, Syscall arguments, Packet headers, VFS paths | Executed in CI and monthly fuzz triage via `fuzz/` targets |
| **Stress Test** | Verifies behavior under resource exhaustion, high concurrency, and OOM | Physical memory, Swap operations, Thread spawn loops | Executed during biweekly audits and release candidates |
| **Fault Injection** | Tests recovery and panic paths under injected hardware/I/O failures | Storage block reads, Memory allocations, Driver probes | Executed via failure injection test runners (`installer/tests/failure_injection.rs`) |
