# SigmaOS Continuous OS Improvement & Operations Plan

> **Authoritative Operational Policy Document**
> **Target Subsystem:** SigmaOS Kernel, Universal Drivers, Security, and Governance
> **Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS

---

## 1. Executive Summary & Core Directives

SigmaOS is a Rust-centric, `#![no_std]` high-performance sovereign operating system. To ensure continuous progress relative to leading open-source OS projects (such as Redox, seL4, Tock, Fuchsia, Linux, and FreeBSD), this operations plan establishes a structured, repeatable engineering rhythm across all architectural layers.

### Architectural Invariants Enforced across All Changes
- **`#![no_std]` Purity:** Kernel and driver core modules strictly prohibit standard library usage (`std`), relying solely on `core` and `alloc`.
- **Capability-Based Security Tokens:** All syscall entrypoints enforce explicit capability token (`CapabilityToken`) validation before dispatch.
- **WDM-Style Driver Model:** Device drivers adhere to `IoManager`, `DriverObject`, `DeviceObject`, and `DeviceExtension` architectural patterns with full lifecycle handling (`create`, `attach`, `detach`, `destroy`).
- **Memory Pool Isolation:** Allocations strictly differentiate between `Paged` and `NonPaged` pool domains with bounds checking and guard pages.
- **Explicit Invariant & Type Annotations:** Public API declarations must carry explicit type annotations and documented safety invariants.
- **Standalone Module Testability:** Every Rust source module must be verifiable via standalone `rustc --test --edition=2021` compilation and execution without relying on unstable workspace features.

---

## 2. Recurring Engineering Cadences

```
      ┌─────────────────────────────────────────────────────────┐
      │  Per-PR / Commit: Fast Checks (rustfmt, clippy, test)   │
      └───────────────────────────┬─────────────────────────────┘
                                  │
                                  ▼
      ┌─────────────────────────────────────────────────────────┐
      │  Daily: Issue Triage, Automated Labeling, CI Fixes      │
      └───────────────────────────┬─────────────────────────────┘
                                  │
                                  ▼
      ┌─────────────────────────────────────────────────────────┐
      │  Weekly: Backlog Grooming, Integration & Microbench     │
      └───────────────────────────┬─────────────────────────────┘
                                  │
                                  ▼
      ┌─────────────────────────────────────────────────────────┐
      │  Monthly: Security Fuzzing, MIRI, OSS Competitor Scan   │
      └───────────────────────────┬─────────────────────────────┘
                                  │
                                  ▼
      ┌─────────────────────────────────────────────────────────┐
      │  Quarterly: Architecture Sync, SBOM, Signed Snapshot    │
      └─────────────────────────────────────────────────────────┘
```

### Daily / Per-PR (Fast Validation)
- **Automated CI Validation:**
  - Run `rustfmt` formatting and `clippy -- -D warnings` checks.
  - Run standalone `rustc --test` on all modified source files (`scripts/changed_files_rustc_tests.sh`).
  - Scan for unapproved `std` imports in kernel crates (`scripts/no_std_check.sh`).
  - Perform headless QEMU boot smoke test when bootloader or core initialization files are touched.
- **PR Gate Checklist (PR Template):**
  - Verify capability token validation on any new or modified syscall entrypoints.
  - Verify WDM driver lifecycle unit tests (`DriverObject` / `DeviceObject`).
  - Verify memory bounds clamping on `copy_nonoverlapping` operations.

### Weekly
- **Triage & Backlog Grooming:**
  - Categorize new issues (`security`, `bug`, `enhancement`, `hardware`, `driver`, `docs`).
  - Prioritize issues blocking kernel boot, memory safety, or capability validation.
- **Integration & Benchmark Suite:**
  - Run full subsystem integration test suites via `./run_sigma_tests.sh`.
  - Record syscall and IPC microbenchmark metrics into historical time-series logs.
- **Toolchain Audit:**
  - Verify pinned `rustc` toolchain and dependencies to preserve deterministic builds.

### Monthly
- **Security & Dynamic Analysis:**
  - Execute 24-48 hour fuzzing runs (`cargo-fuzz` / `libFuzzer`) targeting IPC message deserializers, syscall entrypoints, and driver ioctl handlers.
  - Run `MIRI` on host-testable unsafe-critical memory management routines.
- **Competitor Scan & Open-Source Idea Mining:**
  - Execute `scripts/competitor_scan.py` to analyze upstream developments across Redox, seL4, Tock, Fuchsia, and Linux/BSD.
  - Open structured RFC issues (`.github/ISSUE_TEMPLATE/oss_idea_adoption.yml`) for high-value feature adoption.

### Quarterly
- **Architecture Review & Strategic Roadmap:**
  - Evaluate capability revocation mechanics, Paged/NonPaged pool pressure under stress, and hardware device matrix.
  - Perform competitive benchmarking against Redox, seL4, Tock, and minimal Linux kernels.
- **Release Engineering & Supply Chain:**
  - Generate Software Bill of Materials (SBOM) using Syft.
  - Build deterministic release images and sign artifacts using Cosign.

### Annual
- Complete comprehensive threat model refresh and supply chain security audit.
- Review governance, contributor policies, and bug bounty initiatives.

---

## 3. Component-Focused Development Backlog

| Subsystem | Development Focus | Verification & Test Strategy |
| :--- | :--- | :--- |
| **Boot & Firmware** | Secure Boot verification, measured boot hashes, multiboot2/UEFI handoff. | QEMU boot smoke matrix (`scripts/qemu_smoke_test.sh`). |
| **Kernel Core & Sched** | EEVDF scheduler fairness, real-time priority slices, BORE priority boosting. | Red-Black vruntime tracking unit tests (`tests/scheduler/core.rs`). |
| **Memory Management** | Paged/NonPaged pool isolation, guard pages, page compaction, ZRAM reclaim. | Standalone frame allocator and page table tests (`tests/memory/physical.rs`). |
| **Capability Security** | Token generation, delegation, revocation, and lifetime tracking. | Property-based testing (`proptest`) for token delegation invariants. |
| **Drivers & HAL** | xHCI 3.2, NVMe 2.0, Wi-Fi 7 MLO, VBE 3.0, ISA PnP legacy support. | WDM lifecycle unit tests (`create`, `attach`, `detach`, `destroy`). |
| **VFS & Storage** | Btrfs CoW fail-closed backend, VirtIO-Block, block cache LRU. | File descriptor table and inode lifecycle tests (`tests/fs/fd_table.rs`). |
| **Networking** | WireGuard PQC mesh, nftables packet filtering, TCP windowing. | Packet parser fuzzing harness and smoltcp integration tests. |
| **Userland & WASM** | Wasmtime/WASI sandboxed app launcher, minimal POSIX compatibility layer. | Capability-restricted syscall boundary integration tests. |

---

## 4. Key Performance Indicators (KPIs)

- **CI Pass Rate & Build Speed:** >98% PR pass rate; PR fast-check duration < 3 minutes.
- **Security Health:** Zero unhandled fuzzer crash reports; median time-to-remediate for P0 vulnerabilities < 48 hours.
- **Syscall / IPC Latency:** System call entry-to-exit overhead < 150 ns; IPC message throughput > 2.5 GB/s.
- **Build Reproducibility:** 100% hash parity across nightly deterministic release builds.
- **Unsafe Code Hygiene:** Every `unsafe` block must be accompanied by a documented `# Safety` invariant block and covered by unit tests.

---

## 5. 90-Day Actionable Implementation Roadmap

- **Phase 1 (Days 0–14):** Formalize PR templates, fast CI checks (`.github/workflows/pr_fast_checks.yml`), `scripts/no_std_check.sh`, and `scripts/changed_files_rustc_tests.sh`.
- **Phase 2 (Days 15–45):** Deploy nightly reproducible build and SBOM generation pipeline; establish `cargo-fuzz` harnesses for IPC and syscall parsers.
- **Phase 3 (Days 46–90):** Integrate `MIRI` checks for unsafe memory routines; perform property-based testing on capability revocation tokens; execute quarterly open-source OS benchmark audit.
