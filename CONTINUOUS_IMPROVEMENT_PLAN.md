# SigmaOS Continuous Open Source Improvement & Operating Plan

## Executive Summary

This document establishes a concrete, repeatable operations and engineering plan for **SigmaOS** to continuously evolve, harden, and innovate relative to peer open-source operating systems (Redox, seL4, Tock, Fuchsia, Linux, FreeBSD, OpenBSD).

The plan enforces SigmaOS's architectural axioms:
- `#![no_std]` compliance across core kernel, driver, and subsystem layers.
- Capability-based security (CapabilityToken verification on all privilege-sensitive syscalls and IPC interfaces).
- Explicit type annotations across public APIs and data structures.
- Bounded memory allocations with distinct Paged and NonPaged memory pools.
- Standalone unit testability for individual Rust source files via `rustc --test --edition=2021`.

---

## 1. Recurring Cadences & Operating Rhythms

| Cadence | Focus Area | Concrete Tasks & Gates |
|---|---|---|
| **Per-PR / On-Commit** | Fast CI & Architectural Invariants | 1. `cargo fmt --check`<br>2. `cargo clippy -- -D warnings`<br>3. Standalone rustc test for modified `.rs` files (`./scripts/changed_files_rustc_tests.sh`)<br>4. `no_std` compliance scan (`./scripts/no_std_check.sh`)<br>5. Capability check heuristic on syscall entrypoints<br>6. Headless QEMU boot smoke test if boot/kernel files changed |
| **Daily** | Issue & CI Triage | 1. Automated issue triage & labeling (security, bug, driver, hardware, docs)<br>2. Mean Time To Repair (MTTR) < 48 hours for CI failures |
| **Weekly** | Backlog Grooming & Microbenchmarks | 1. Prioritize bootability, memory safety, capability enforcement, and driver stability<br>2. Run unit and integration suites (`tests/`)<br>3. Collect syscall and IPC latency microbenchmarks<br>4. Toolchain and dependency audit |
| **Bi-Weekly (Sprint End)** | Merge Consolidation & RC Build | 1. Consolidate merged PRs, update CHANGELOG<br>2. Produce nightly release candidate artifact<br>3. Community and maintainer sync |
| **Monthly** | Security, Fuzzing & Hardware Tests | 1. Run `cargo-fuzz` (libFuzzer) on IPC (`fuzz/fuzz_targets/ipc_parser.rs`), syscall parsers, and driver IOCTLs<br>2. Run MIRI on unsafe modules under test harness<br>3. Hardware/QEMU driver compatibility matrix<br>4. Reconcile documentation TODOs (`./scripts/find_doc_todos.sh`)<br>5. Publish monthly report (CI health, fuzz crashes, benchmark trends) |
| **Quarterly** | Architecture Review & Snapshot Release | 1. Capability model and attack surface review<br>2. Benchmarking against Redox, seL4, Tock, and Linux minimal kernel<br>3. Publish signed snapshot with SBOM (Syft) and cosign signatures |
| **Annual** | Threat Model & Governance Refresh | 1. Full threat model audit and supply-chain provenance check<br>2. Long-term roadmap refresh and bug bounty evaluation |

---

## 2. Open-Source OS Paradigms & Adaptation Playbook

| OS Project | Borrowed Concepts & Best Practices | SigmaOS Concrete Adaptation |
|---|---|---|
| **Redox OS** | Microkernel userspace process isolation, scheme-based resource namespaces, TFS crash-consistency | Redefined process launcher with sandboxed scheme handles and userspace capability delegation |
| **seL4** | Formally-verified capability derivation trees, bounded CNode capability slots, zero implicit kernel memory | CapabilityToken formal invariants and property-based verification (`proptest`) for token lifecycles and revocation |
| **Tock OS** | Capsule-based memory-safe drivers, asynchronous driver grants, CBOR serialization | Driver capsule isolation model, WDM DriverObject/DeviceObject lifecycle tests, and safe grant buffers |
| **Fuchsia / Zircon** | Handle-based channel IPC, component manifests, structured async messaging | Channel IPC state machine with capability-verified message endpoints and structured error routing |
| **Linux Minimal Kernel** | eBPF bytecode execution, folio multi-page compaction, driver matrix testing | Driver test matrix harness, scheduler microbenchmarks, and fast boot optimization |
| **FreeBSD** | Capsicum sandboxing, Casper capability service daemons, NVMe-TLS storage integrity | Process-level capability mode restriction and Casper-style helper services |
| **OpenBSD** | Pinsyscall kernel-entry address validation, dynamic unveil filesystem restriction | Static and dynamic syscall entry verification and path unveil security primitives |

---

## 3. Component-by-Component Development Backlog

Each subsystem follows a 5-stage engineering lifecycle:
1. **Implement / Extend**
2. **Unit / Integration Tests**
3. **Fuzz / Microbenchmark**
4. **Document & Specify**
5. **CI Automation & Gating**

### A. Boot & Firmware
- Measured boot support and bootloader handoff verification.
- Headless QEMU smoke tests in CI (`scripts/qemu_smoke_test.py`).
- Deterministic memory map parsing.

### B. Kernel Core & Scheduler
- Scheduler latency and fairness microbenchmarks (`tools/sigma_microbench_compat.rs`).
- Paged and NonPaged pool boundary enforcement with guard pages.
- Capability token revocation and delegation tracking.

### C. Driver Framework & Hardware Abstraction
- WDM-style `IoManager`, `DriverObject`, and `DeviceObject` lifecycle requirements.
- Mandatory driver lifecycle tests (create, attach, handle IOCTL, detach, destroy).
- Monthly driver test lab execution across QEMU device models.

### D. IPC & Syscalls
- CapabilityToken validation required on privilege-sensitive entrypoints.
- Continuous libFuzzer fuzzing (`fuzz/fuzz_targets/ipc_parser.rs`).
- High-throughput zero-copy ring buffer channels.

### E. Filesystems & VFS
- Crash-consistency tests for RAMFS and block-based storage.
- Fuzzing of filesystem metadata decoders (`fuzz/fuzz_targets/vfs_path_parser.rs`).

---

## 4. Automation & Tooling Assets

SigmaOS provides ready-to-run automation scripts and workflows:
- **`scripts/changed_files_rustc_tests.sh`**: Scans modified `.rs` files and runs standalone `rustc --test --edition=2021`.
- **`scripts/no_std_check.sh`**: Audits codebase for unauthorized `std::` imports outside of test configurations.
- **`scripts/find_doc_todos.sh`**: Scans documentation and source files for `TODO`, `FIXME`, `PLANNED`, and `RFC` tags.
- **`.github/workflows/pr_fast_checks.yml`**: GitHub Actions workflow executing formatting, clippy, `no_std` check, and standalone file testing.
- **`fuzz/fuzz_targets/ipc_parser.rs`**: libFuzzer harness verifying IPC parsing and capability tokens.

---

## 5. 90-Day Prioritized Implementation Roadmap

- **Days 0–14 (Foundations & Fast CI)**:
  - Enforce PR templates with driver lifecycle and capability-check checklists.
  - Active GitHub Actions workflow `pr_fast_checks.yml`.
  - Continuous `no_std` enforcement in CI.
- **Days 15–45 (Quality, Verification & Baselines)**:
  - Nightly reproducible build checks, SBOM generation, and cosign signing.
  - Baseline syscall and IPC latency recording (`tools/sigma_microbench_compat.rs`).
  - Active nightly cargo-fuzz runs on IPC and VFS parsers.
- **Days 46–90 (Security, Ecosystem & Architecture Review)**:
  - MIRI runs on unsafe allocator and memory structures.
  - CapabilityToken property testing via `proptest`.
  - First quarterly comparative benchmark report vs Redox, seL4, and Linux.
  - Signed release snapshot.

---

## 6. Key Performance Indicators (KPIs)

1. **CI Fast-Check Pass Rate**: Target $\ge 95\%$ on PR submissions.
2. **Fuzzer Crash Triage Time**: Critical crashes triaged within 24 hours, fixed within 7 days.
3. **Syscall / IPC Latency Delta**: No regression exceeding $2\%$ across consecutive releases.
4. **Unsafe Block Ratio**: Decreasing unsafe block count per KLOC, 100% justified with safety comments and unit tests.
5. **Nightly Reproducibility**: 100% bit-for-bit reproducible artifact builds.
