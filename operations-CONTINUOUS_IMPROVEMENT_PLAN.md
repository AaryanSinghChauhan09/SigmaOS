# SigmaOS Continuous OS Improvement Operational Plan

This document establishes a concrete, repeatable operational framework for the continuous development, testing, security auditing, and open-source OS feature adoption of **SigmaOS**.

---

## 1. Core Architectural Invariants

All engineering contributions to SigmaOS must comply with the following mandatory architectural rules:

1. **`#![no_std]` Bare-Metal Core**: Kernel, driver, and core subsystem modules must enforce `#![no_std]` (with `alloc` when necessary) and avoid standard library dependencies except under `#[cfg(test)]`.
2. **Capability-Based Security**: Every system call entrypoint and privileged resource access must explicitly verify a valid `CapabilityToken`.
3. **WDM-Style Driver Architecture**: Hardware drivers must implement the Windows Driver Model (WDM) lifecycle pattern (`IoManager`, `DriverObject`, `DeviceObject`, `DeviceExtension`) with explicit attach, dispatch, and detach unit tests.
4. **Paged / NonPaged Memory Pools**: Memory allocations in kernel space must strictly distinguish between `Paged` (swappable) and `NonPaged` (pinned physical memory) pools.
5. **Explicit Type Annotations**: Public APIs, syscall signatures, and core collections must use explicit type declarations.
6. **Bounds-Clamped Operations**: All raw memory copies (`copy_nonoverlapping`) and buffer manipulations must perform explicit length and bounds checking.
7. **Standalone File Unit Testing**: Every Rust source file should be compilable and testable individually via `rustc --test --edition=2021 <filepath>`.

---

## 2. Recurring Cadences & Operational Rhythms

### Per-PR / On Every Commit (Fast Checks)
* **CI Validation**: Execute `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `./scripts/no_std_check.sh`, and `./scripts/changed_files_rustc_tests.sh`.
* **Automated Checklist**: Validate PR compliance using `.github/PULL_REQUEST_TEMPLATE.md` (capability tokens, explicit types, bounds checks, driver lifecycle).
* **QEMU Smoke Test**: Execute headless QEMU boot test (`python3 scripts/qemu_smoke_test.py`) when core boot or kernel files are modified.

### Daily / Triage
* **Issue Triage**: Automated bot labeling and routing of newly filed issues and PRs.
* **Regression Hotfixes**: Maintain an MTTR < 48 hours for breaking CI regressions.

### Weekly
* **Backlog Grooming**: Prioritize issues affecting boot stability, capability enforcement, memory safety, and hardware compatibility.
* **Documentation Scanning**: Execute `./scripts/find_doc_todos.sh` to extract planned ideas and `TODO` items from documentation into actionable tracking issues.
* **Performance Baseline**: Run microbenchmark suites for syscall/IPC latency and track time-series deltas.

### Monthly
* **Fuzzing & MIRI Analysis**: Run `cargo-fuzz` targets against IPC parsers, syscall deserializers, and driver ioctl endpoints; run MIRI against unsafe memory blocks in host-targeted test harnesses.
* **Competitor Innovation Scan**: Execute `./scripts/competitor_scan.py` to evaluate upstream releases across Redox, seL4, Tock, Fuchsia, and Linux, creating 1-page adoption proposals.
* **Test Harness Expansion**: Add QEMU hardware emulation models and driver compatibility matrix tests.

### Quarterly
* **Architecture & Security Sync**: Review capability model evolution, Paged/NonPaged memory pool stress handling, and post-quantum crypto integration.
* **Benchmarking Matrix**: Evaluate SigmaOS against Redox, seL4, Tock, and minimal Linux kernels across boot time, memory footprint, and IPC throughput.
* **Signed Release Snapshot**: Generate reproducible release builds accompanied by Software Bill of Materials (SBOM) and Cosign signatures.

### Annual
* **Supply Chain & Threat Model Audit**: Comprehensive threat-model update and third-party security audit of kernel entrypoints.

---

## 3. Component Development Backlog & OSS Adoption Pipeline

| Subsystem | Primary Focus | OSS Reference & Idea Adoption | Verification Gate |
| :--- | :--- | :--- | :--- |
| **Boot & Platform** | Measured boot, UEFI ESP, ACPI MADT / APIC discovery | Fuchsia / Linux verified boot | QEMU smoke test & Bare-Metal boot suite |
| **Kernel Core** | POSIX 450+ syscall dispatcher, demand paging, signal delivery | seL4 capability revocation & preemption | Tier 1 Kernel Execution suite |
| **Memory Management** | Frame allocator, 4-level x86_64 paging, Swap slot manager | Linux folio compaction & BSD zone allocators | Standalone physical, paging & virtual tests |
| **IPC & Syscalls** | Pipes, Unix Sockets, Shared Memory, Capability tokens | Fuchsia handle IPC & WASI capability scoping | Fuzzing & microbenchmark latency suite |
| **Drivers & HAL** | WDM Driver objects, ISA PnP, NVMe 2.0 ZNS, Wi-Fi 7 MLO | Tock driver capsules & Linux mac80211 | Driver lifecycle & phase 7 expansion test |
| **VFS & Storage** | Inode lifecycle, flock, Block Cache, Mount manager | Redox TFS & OpenBSD Unveil path resolution | Storage & VFS integration test suites |
| **Userland & Shell** | Interactive POSIX shell, builtins, WASM execution runtime | WASI sandboxing & GNU coreutils parity | Shell conformance test & coreutils gateway |

---

## 4. 90-Day Roadmap

* **Days 0–14 (Foundations)**: Implement fast PR checks, driver lifecycle templates, no_std verification scripts, and doc TODO scanner.
* **Days 15–45 (Quality & Automation)**: Establish nightly reproducible build pipeline with SBOM generation, microbenchmark harness, and cargo-fuzz targets.
* **Days 46–90 (Security & Ecosystem)**: Expand MIRI verification for unsafe blocks, implement proptest property tests for CapabilityTokens, and run quarterly architecture benchmarking.

---

## 5. Key Performance Indicators (KPIs)

* **CI Pass Rate**: > 98% of PRs passing fast checks on initial run.
* **Fuzzing Effectiveness**: Zero unhandled panic crashes in IPC and syscall deserializers.
* **Performance Latency**: Syscall dispatch < 150 ns; IPC context switch < 500 ns.
* **Unsafe Code Hygiene**: 100% of `unsafe` blocks audited, documented with safety invariants, and covered by unit tests.
* **Build Reproducibility**: 100% matching hashes for nightly release artifacts.
