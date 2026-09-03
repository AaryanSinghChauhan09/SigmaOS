# 📜 SigmaOS Master Operations & Continuous OS Improvement Guide

> **Standard Operating Procedures, Recurring Engineering Cadences, Component Development Backlog, Competitor Feature Absorption Frameworks, and 90-Day Roadmap for SigmaOS.**

***

## 🎯 Executive Overview

SigmaOS is an open-source, post-quantum resilient, zero-dependency operating system written in Rust (`no_std`), utilizing microkernel security and bare-metal performance.

To ensure long-term architectural excellence, safety, and continuous parity with competitor OS environments (Redox, seL4, Tock, Fuchsia, Linux, and BSDs), this guide formalizes SigmaOS's repeatable engineering cadences, quality gates, and automated operations pipelines.

***

## 📅 Recurring Engineering Cadences

### 1. Per-PR / On-Commit (Fast Check Quality Gates)

*   **Automated CI Validation**: Run `.github/workflows/pr_fast_checks.yml`.
    *   Enforce `rustfmt` formatting and strict `clippy -- -D warnings`.
    *   Audit source files for strict `no_std` compliance using `./scripts/no_std_check.sh`.
    *   Execute standalone unit tests on modified files using `./scripts/changed_files_rustc_tests.sh`.
    *   Run QEMU smoke test (`python3 scripts/qemu_smoke_test.py`) if kernel or boot initialization files are modified.
*   **Architectural Rules Checklist** (Enforced via `.github/PULL_REQUEST_TEMPLATE.md`):
    *   Capability Token Verification: All syscall entrypoints must verify capability tokens (`verify_token`).
    *   Driver Object Lifecycles: Driver PRs must adhere to WDM standards (`DriverObject`, `DeviceObject`, `DeviceExtension`) and include attach/detach unit tests.
    *   Memory Pool Bounds: Explicit separation and bounds checks between Paged and NonPaged memory pools.
    *   Explicit Type Annotations & Safe Wrappers: All public APIs must have explicit type declarations and documented `// SAFETY:` invariants.

### 2. Daily / Bug Triage Cadence

*   **Automated Triage**: Bot scans new issues and applies initial labels (`bug`, `driver`, `security`, `competitor-scan`).
*   **CI Failure MTTR**: Monitor and fix CI regressions within a target MTTR < 24 hours.

### 3. Weekly Cadence

*   **Backlog Grooming & TODO Resolution**: Run `./scripts/find_doc_todos.sh` to extract codebase and documentation TODO/FIXME items into sprint tasks.
*   **Performance Regression Tracking**: Run microbenchmarks (`tools/sigma_microbench_compat.rs`) measuring IPC latency, capability check overhead, and memory allocation rates.

### 4. Monthly Cadence

*   **Fuzzing & MIRI Analysis**: Run long-running fuzzers (`./scripts/fuzz.sh`, `./scripts/fuzz_pqc.sh`) against IPC deserializers and driver IOCTL handlers. Execute MIRI on host-tested unsafe modules.
*   **Competitor Absorption Scan**: Execute `./scripts/competitor_scan.py` to scan Redox, seL4, Tock, Fuchsia, and Linux releases for breakthrough features.

### 5. Quarterly Cadence

*   **Architecture & Security Review**: Evaluate capability model invariants, memory pool health under stress, and post-quantum crypto integration.
*   **Reproducible Build Verification**: Build deterministic ISO artifacts and produce signed Software Bill of Materials (SBOM) with provenance records.
*   **Benchmark Dashboard Publishing**: Update comparative dashboards versus Redox, seL4, Linux, and FreeBSD.

### 6. Annual Cadence

*   **Threat Model & Supply Chain Audit**: Comprehensive code audit, reproducible build verification, and long-term roadmap refresh.

***

## 🔬 5-Stage Component Development Backlog

Every OS component in SigmaOS follows a 5-stage lifecycle:

1.  **Specification & Prototype**: Design RFC, capability scope, and standalone host prototype.
2.  **Implementation & Safety**: Write `no_std` kernel/driver implementation with explicit type annotations and bounds checking.
3.  **Unit & Standalone Testing**: Create per-file unit tests runnable via `rustc --test`.
4.  **Integration & Fuzzing**: Add QEMU integration tests and cargo-fuzz targets.
5.  **CI Automation & Documentation**: Register in CI workflows, write wiki guide, and publish benchmarks.

***

## 🌐 Competitor Feature Absorption Framework

| Source Project | Feature Inspiration | SigmaOS Adaptation Strategy |
| :--- | :--- | :--- |
| **Redox OS** | Microkernel URL schemes & userspace drivers | Integrated into `SigmaFsPlusPlus` and capability-gated driver channels. |
| **seL4** | Capability revocation & formal specs | Executable property-based tests (`proptest`) for `CapabilityToken` lifecycles. |
| **Tock OS** | Capsule driver isolation | Safe Rust encapsulation for memory-mapped I/O and hardware registers. |
| **Fuchsia** | Component manifests & Zircon handles | Userspace capability manifests and process sandboxing via `pledge`/`unveil`. |
| **Linux / BSD** | Driver models, POSIX/Capsicum parity | WDM-style `DriverObject` abstractions, eBPF policy verifiers, and Capsicum rights. |
| **WASI / Wasmtime**| Sandboxed Wasm app runtime | Embedded WASM execution engine in `src/kernel/subsystems/sovereign_modules.rs`. |

***

## 🗺️ 90-Day Prioritized Roadmap

*   **Days 0–14 (Foundations & Automation)**:
    *   Deploy PR Fast Checks CI workflow (`.github/workflows/pr_fast_checks.yml`).
    *   Deploy Pull Request Template (`.github/PULL_REQUEST_TEMPLATE.md`) and Issue Templates (`.github/ISSUE_TEMPLATE/`).
    *   Institutionalize `./scripts/no_std_check.sh`, `./scripts/changed_files_rustc_tests.sh`, and `./scripts/find_doc_todos.sh`.

*   **Days 15–45 (Quality, Benchmarking & Fuzzing)**:
    *   Schedule nightly cargo-fuzz runs for IPC and driver IOCTL handlers.
    *   Implement automated microbenchmark trend exports (`tools/sigma_microbench_compat.rs`) and regression thresholds.
    *   Expand driver lifecycle unit tests across all network and block storage drivers.

*   **Days 46–90 (Security, Formal Verification & Ecosystem)**:
    *   Execute MIRI checks on all unsafe memory allocator blocks (`src/kernel/memory.rs`, `perf_mm.rs`).
    *   Implement property-based capability revocation tests (`src/security/capability.rs`).
    *   Publish signed quarterly release snapshot with automated SBOM generation.

***

## 📈 Key Performance Indicators (KPIs)

*   **CI Success Rate**: > 95% of PRs passing fast checks on first run.
*   **`no_std` Compliance**: 100% std-free in core kernel and driver modules.
*   **Fuzzing Coverage**: Zero unhandled panics or memory corruptions in nightly 12-hour fuzz runs.
*   **IPC Latency**: < 1.2 microseconds per capability-verified IPC dispatch.
*   **Build Reproducibility**: 100% hash match between independent build environments.
