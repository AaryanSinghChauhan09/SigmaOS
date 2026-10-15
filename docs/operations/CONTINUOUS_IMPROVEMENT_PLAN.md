# SigmaOS Continuous OS Improvement Operational Plan

This operational plan provides a repeatable engineering rhythm for continuously advancing SigmaOS relative to other open-source operating systems (such as Redox, seL4, Tock, Fuchsia, Linux, and FreeBSD).

It incorporates SigmaOS core architectural invariants:
- `#![no_std]` microkernel context enforcement.
- Capability-based security token verification on syscall entrypoints.
- WDM-style driver architecture (`IoManager`, `DriverObject`, `DeviceObject`, `DeviceExtension`).
- Paged vs NonPaged memory pool isolation and guard boundaries.
- Explicit type annotations on public API declarations and key collections.
- Bounds-checked/clamped memory operations (`copy_nonoverlapping` validation).
- Standalone file unit testing via `rustc --test`.

---

## 1. Recurring Engineering Cadences

### Per PR / On Each Commit
- **Fast Automated Checks**:
  - Code formatting: `cargo fmt --check`.
  - Lints & Warnings: `cargo clippy --all-targets --all-features -- -D warnings`.
  - `no_std` compliance verification via `./scripts/no_std_check.sh`.
  - Standalone rustc unit tests for modified files via `./scripts/changed_files_rustc_tests.sh`.
- **Architectural PR Checklist**:
  - Capability token checks verified on all syscall entrypoints (`verify_token` or equivalent).
  - Explicit type annotations present on public API parameters and struct fields.
  - Bounds clamping verified for any raw pointer operations (`copy_nonoverlapping`).
  - Driver PRs follow WDM patterns (`DriverObject`/`DeviceObject`/`DeviceExtension`) and include driver lifecycle tests (`create`, `attach`, `detach`, `destroy`).
- **Smoke Tests**:
  - Trigger headless QEMU boot smoke test if bootloader, kernel init, or memory management files are modified.

### Weekly
- **Triage & Backlog Grooming**:
  - Classify and prioritize incoming issues (`security`, `bug`, `enhancement`, `hardware`, `driver`, `docs`).
  - Prioritize issues impacting bootability, memory safety, or capability token verification.
- **Test Suite Execution**:
  - Execute complete test suites via `./run_sigma_tests.sh`.
- **Microbenchmarking**:
  - Run microbenchmarks for syscall latency, context switch overhead, and IPC throughput.
- **Dependency & Toolchain Audit**:
  - Verify pinned toolchain compatibility (`rust-toolchain.toml`) and ensure zero unwanted `std` dependencies.

### Bi-Weekly (Sprint Cadence)
- **Release Candidate & Merge Window**:
  - Consolidate merged PRs, update `CHANGELOG.md`, and produce release candidate build artifacts.
- **Community & Contributor Sync**:
  - Share release notes, highlight open high-priority tasks, and review design proposals.

### Monthly
- **Fuzzing & Security Sanity**:
  - Run `cargo-fuzz` / `libFuzzer` harnesses against IPC serialization, syscall parsers, and driver IOCTL handlers.
  - Run MIRI and host-targeted sanitizers for unsafe-critical memory modules.
- **Competitor Innovation Scan**:
  - Run `scripts/competitor_scan.py` to evaluate recent upstream changes in Redox, seL4, Tock, Fuchsia, and Linux/BSD.
- **Test Harness Expansion**:
  - Add new QEMU test topologies, multi-arch targets, and device emulation profiles.

### Quarterly
- **Architecture & Design Review**:
  - Evaluate CapabilityToken lifecycle, memory pool behavior under stress (Paged vs NonPaged bounds), and scheduler fairness.
- **Comparative Benchmarking**:
  - Benchmark boot time, memory footprint, scheduler latency, and IPC throughput against Redox, seL4, and minimal Linux images.
- **Signed Snapshot Release**:
  - Generate reproducible build artifacts, produce SBOMs via Syft, and sign releases with Cosign.

### Annual
- **Threat Model & Supply Chain Audit**:
  - Perform full threat model refresh (`docs/security/THREAT_MODEL.md`) and supply chain security verification.
- **Roadmap & Governance Refresh**:
  - Update long-term roadmap (`FUTURE-DEVELOPMENT-ROADMAP.md`) and review contributor governance.

---

## 2. Component-Focused Development Backlog

### Boot & Firmware
- Implement measured boot with TPM 2.0 / PQC attestation.
- Expand multi-architecture boot validation across x86_64, AArch64, and RISC-V 64 in QEMU.

### Kernel Core & Scheduler
- Refine CFS / EEVDF scheduler with priority level bitmap selection and starvation boosting.
- Add formal property-based testing (`proptest`) for CapabilityToken delegation and revocation invariants.

### Memory Management & Pools
- Enforce strict Paged Pool vs NonPaged Pool memory boundaries with guard page fault detection.
- Monitor physical frame allocation, double-free prevention, and TLB shootdown performance.

### Device Drivers & HAL
- Maintain WDM-style driver pattern consistency across USB, Storage, Network, and Display drivers.
- Require driver lifecycle unit tests (`init`, `attach`, `detach`, `reset`, `shutdown`) for all driver PRs.

### Filesystems & VFS
- Maintain process-local file descriptor tables (`ProcessFdTable`), inode lifecycle (`InodeManager`), and VFS path normalization.
- Run crash-consistency fuzzers on VFS path parsing and block cache writeback routines.

### Network Stack & IPC
- Expand zero-copy socket IPC and packet ring buffers (`SovereignRingBuffer`).
- Add packet-level fuzzing harnesses for DHCP, DNS, TCP state machine, and IPv4/IPv6 packet parsers.

### Userland & Runtimes
- Develop WASM/WASI runtime sandboxes for capability-scoped user applications.
- Enhance custom init system, POSIX shell, and core utilities (`useradd`, `passwd`, `sudo`, `visudo`).

---

## 3. Open-Source OS Adoption Patterns

| Project | Concept to Borrow / Adapt | SigmaOS Adoption Action |
| :--- | :--- | :--- |
| **seL4** | Capability model proofs & mathematical invariants | Property-based tests for `CapabilityToken` delegation, expiration, and revocation. |
| **Redox** | Safe Rust microkernel userspace process isolation | Process-local capability-scoped file descriptor tables and scheme handlers. |
| **Tock OS** | Capsule driver isolation & memory safety | WDM-style isolated driver extensions with explicit lifecycle contracts. |
| **Fuchsia** | Component manager & handle-based IPC | Capability-manifest component launcher for WASM userland applications. |
| **Linux / BSD** | Drivers, VFS patterns, EEVDF scheduler, Capsicum | EEVDF scheduler implementation, Capsicum-inspired rights, and BSD file locking. |
| **WASI** | Capability-based system interface for WebAssembly | Integrated WASM runtime sandbox enforcing capability token policies on userland syscalls. |

---

## 4. CI Automation & Developer Tooling

- **PR Fast Checks (`.github/workflows/pr_fast_checks.yml`)**:
  - Executes `cargo fmt`, `cargo clippy -- -D warnings`, `scripts/no_std_check.sh`, and `scripts/changed_files_rustc_tests.sh`.
- **Nightly Fuzzing & Security (`.github/workflows/kernel_bsd_security_fuzzing.yml`)**:
  - Runs cargo-fuzz targets for VFS path parsing, shell lexer/parser, and IPC serialization.
- **Reproducible Build & SBOM (`.github/workflows/reproducible-sbom-cosign.yml`)**:
  - Builds reproducible ISO/kernel images, generates SPDX SBOMs, and verifies build hash determinism.
- **Local Developer Scripts**:
  - `scripts/no_std_check.sh`: Scans kernel crates for std imports.
  - `scripts/changed_files_rustc_tests.sh`: Compiles and runs standalone rustc tests for modified source files.
  - `scripts/find_doc_todos.sh`: Audits documentation for outstanding TODO items.
  - `scripts/competitor_scan.py`: Tracks competitive open-source OS advancements.

---

## 5. 90-Day Prioritized Roadmap

```
Phase 1: Days 0–14 (Foundations & Fast CI)
├── Enforce PR fast checks (clippy deny-warnings, formatting, standalone rustc tests)
├── Maintain no_std compliance scanner and doc TODO auditor
└── Apply automated PR templates for capability verification and driver lifecycles

Phase 2: Days 15–45 (Automation & Quality Gates)
├── Schedule nightly fuzzing runs on IPC, VFS, and driver IOCTL entrypoints
├── Expand microbenchmarking harness for syscall latency and IPC throughput
└── Integrate reproducible build hash verification and SBOM generation in CI

Phase 3: Days 46–90 (Capability Security & Ecosystem)
├── Add property-based testing for CapabilityToken revocation and delegation
├── Convert all driver modules to enforce WDM lifecycle contracts
└── Conduct quarterly architecture review and publish comparative OS benchmark report
```

---

## 6. Key Performance Indicators (KPIs)

- **CI Health**: 100% passing PR checks; mean time to resolve (MTTR) broken builds < 24 hours.
- **Memory & Kernel Safety**: Zero `std` leaks in `#![no_std]` crates; 100% explicit type annotations on public APIs.
- **Security & Fuzzing**: Zero unhandled panic crashes in IPC and syscall fuzzer runs.
- **Performance**: Syscall latency < 200 ns; IPC throughput > 10 Gbps (in-memory buffer stream).
- **Reproducibility**: 100% hash match on nightly reproducible builds.
