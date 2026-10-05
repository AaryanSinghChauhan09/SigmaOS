# 📖 SigmaOS Operations & Continuous OS Improvement Guide

## 📋 Overview & Strategic Philosophy
This document establishes the official operations plan and continuous engineering rhythm for **SigmaOS**. Designed to systematically advance SigmaOS relative to mature open-source operating systems (Redox, seL4, Tock OS, Fuchsia, WASI, Linux, and BSDs), this guide outlines recurring engineering tasks, component backlogs, open-source feature absorption frameworks, 90-day roadmaps, and key performance indicators (KPIs).

---

## 🔄 Recurring Engineering Tasks & Cadences

### 1. **Per-PR / On Every Commit (Fast Checks)**
- **Automated Formatting & Lints**: Execute `rustfmt --check` and `clippy -- -D warnings`.
- **Targeted Module Testing**: Run standalone unit tests for modified files using:
  ```bash
  rustc --test --edition=2021 <file_path> -o build/module_test && ./build/module_test
  ```
- **`#![no_std]` Enforcer**: Execute `./scripts/no_std_check.sh` to ensure zero direct `std::` imports in core kernel crates.
- **Syscall Capability Audit**: Verify that syscall entrypoints enforce capability token verification (`CapabilityToken` / `verify_token`).
- **QEMU Smoke Test**: Trigger `scripts/qemu_smoke_test.py` on changes affecting boot, initialization, or kernel core.
- **PR Template Verification**: Ensure PRs adhere to `.github/PULL_REQUEST_TEMPLATE.md` guidelines.

### 2. **Daily Triage & Maintainer Cadence**
- **Issue Classification**: Triage bot labels incoming issues (`bug`, `enhancement`, `driver`, `security`, `oss-adoption`).
- **Regression Resolution**: Address high-severity CI or boot test regressions within 24 hours.

### 3. **Weekly Backlog & Verification Cadence**
- **Backlog Grooming**: Prioritize issues affecting boot, memory safety, driver stability, and security.
- **Full Integration Suite**: Execute master test runner `./run_sigma_tests.sh`.
- **Microbenchmark Tracking**: Run `tools/sigma_microbench_compat.rs` to detect syscall and IPC latency regressions.

### 4. **Monthly Security, Fuzzing & Compatibility Cadence**
- **Fuzzing Campaigns**: Run `cargo-fuzz` targets (`fuzz/fuzz_targets/ipc_parser.rs`, `fuzz_package_parser.rs`) for IPC, package parsing, and driver ioctl validation.
- **Unsafe Code Audit & MIRI**: Run MIRI and host-targeted checks on unsafe memory and allocator abstractions.
- **Driver Matrix Validation**: Validate drivers across QEMU device configurations and target architectures.
- **Documentation TODO Scan**: Run `./scripts/find_doc_todos.sh` to update the engineering backlog summary.

### 5. **Quarterly Architecture & Release Cadence**
- **Architecture Review**: Evaluate capability models, Paged vs. NonPaged pool usage under stress, and post-quantum crypto integration.
- **Benchmarking vs. Competitors**: Measure boot time, footprint, IPC throughput, and syscall latency against Redox, seL4, Tock, and Linux minimal images.
- **Signed Snapshot Release**: Generate reproducible build artifacts, SBOM metadata (Syft/Cosign), and update release channels.

---

## 🧩 5-Stage Component Development Backlog Strategy
For every core subsystem (Kernel, Memory, IPC, Drivers, VFS, Network, Userland, Security), maintainers apply a 5-stage lifecycle:
1. **Implement / Extend**: Write clean `#![no_std]` code with explicit type annotations.
2. **Unit & Integration Test**: Accompany all code with standalone tests (`#[cfg(test)]`).
3. **Fuzz & Benchmark**: Add `cargo-fuzz` targets and record latency metrics.
4. **Document & Template**: Update `docs/` and driver/subsystem templates.
5. **CI Automation**: Integrate automated checks into GitHub Actions matrix workflows.

---

## 🌐 Open-Source Competitor Feature Absorption Framework

SigmaOS systematically evaluates and absorbs architectural strengths from peer open-source projects:

| Project | Key Architectural Pattern | SigmaOS Adaptation Strategy |
| :--- | :--- | :--- |
| **Redox OS** | Userspace scheme architecture & microkernel IPC | Scheme-like VFS drivers isolated behind CapabilityToken gates |
| **seL4** | Formal capability proofs & strict isolation | Executable proptest models for CapabilityToken revocation & delegation |
| **Tock OS** | Capsule driver sandboxing | DriverObject / DeviceObject lifecycle templates with DMA boundary clamping |
| **Fuchsia / Zircon** | Handle-based async IPC & component manifests | Structured handle passing & capability-scoped process manifests |
| **WASI / Wasmtime** | Sandboxed capability-based app runtime | Minimal userland WASM runtime execution environment |
| **Linux / BSD** | Driver testing matrix, OpenBSD pledge/unveil, Landlock | Sandboxing primitives and multi-device driver matrix testing |

---

## 📊 Key Performance Indicators (KPIs)

- **CI Pass Rate**: > 98% first-pass CI success rate on PRs.
- **Fuzz Crash Resolution**: MTTR < 7 days for unique fuzzer findings.
- **Syscall Dispatch Latency**: Maintain < 120 ns dispatch overhead.
- **Zero-Dependency Integrity**: 100% `#![no_std]` enforcement across kernel and driver crates.
- **Reproducible Builds**: 100% hash matching on nightly release artifacts.

---

## 📅 90-Day Priority Implementation Roadmap

- **Month 1 (Days 0–30)**: Deploy automated PR templates, issue templates, `no_std` CI checks, and IPC fuzzing harnesses.
- **Month 2 (Days 31–60)**: Expand MIRI verification for NonPagedPool allocators and run automated driver lifecycle tests.
- **Month 3 (Days 61–90)**: Conduct quarterly architecture review, execute competitor benchmarking suite, and publish signed release snapshot with SBOM provenance.
