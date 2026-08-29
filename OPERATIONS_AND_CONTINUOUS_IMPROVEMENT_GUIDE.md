# SigmaOS Operations & Continuous OS Improvement Guide

This document defines the repeatable, continuous engineering program for **SigmaOS**. It establishes cadences, testing guidelines, open-source competitor feature absorption frameworks, component backlogs, automation pipelines, and KPIs.

---

## 1. Operating Cadences & Rhythms

To maintain high development velocity while preserving zero-dependency, microkernel safety, engineering activities follow a structured hierarchy of cadences:

### A. Per-PR / On-Commit (Fast Checks)
- **Formatting & Lints**: Enforce `rustfmt` and `cargo clippy -- -D warnings`.
- **`no_std` Compliance**: Run `./scripts/no_std_check.sh` to prevent accidental `std` imports in bare-metal targets.
- **Standalone Unit Tests**: Execute `./scripts/changed_files_rustc_tests.sh` to compile and run fast `rustc --test` checks on modified `.rs` source files.
- **QEMU Smoke Test**: Trigger headless QEMU boot check when changes touch `boot/`, `kernel/`, or architecture code.
- **Architectural Checklist**: PR template requires capability token verification, bounds clamping, explicit type annotations, and WDM driver lifecycle tests.

### B. Daily (Automation & Triage)
- **Automated Issue Triage**: Assign labels (`security`, `bug`, `driver`, `kernel`, `enhancement`) and auto-assign triage leads.
- **CI Failure MTTR**: Aim to resolve build and test regressions within 24–48 hours.

### C. Weekly (Backlog & Benchmarking)
- **Backlog Grooming**: Prioritize bootability, memory safety, driver compatibility, and security bugs.
- **Subsystem Test Suite**: Execute full subsystem integration tests (`./run_sigma_tests.sh`).
- **Microbenchmarks**: Run `tools/sigma_microbench_compat.rs` to track syscall latency, IPC throughput, and CapabilityToken verification overhead.

### D. Monthly (Security, Drivers & Competitor Scan)
- **Fuzzing & Sanitizers**: Run `scripts/fuzz.sh` and `cargo-fuzz` against IPC message parsers, filesystem inodes, and driver ioctl interfaces.
- **MIRI & Verification**: Execute MIRI on unsafe-critical host-test harnesses.
- **Driver Matrix**: Test device driver lifecycles on QEMU device topologies and hardware testbeds.
- **Competitor Scan**: Audit updates from Redox, seL4, Tock, Fuchsia, and Linux to evaluate and prototype transferable innovations.

### E. Quarterly (Architecture & Releases)
- **Architecture Review**: Evaluate capability delegation/revocation models, Paged/NonPaged memory pool pressure, and PQC security posture.
- **Cross-OS Benchmarking**: Benchmark boot times, RAM footprint, and IPC throughput against Redox, seL4, Tock, and minimal Linux images.
- **Snapshot Release**: Build reproducible images, generate Syft SBOM, and sign artifacts with Dilithium-5 / Cosign.

### F. Annually (Audits & Strategy)
- **Supply-Chain Audit**: Perform cryptographic verification of all toolchains and build dependencies.
- **Threat Model Refresh**: Conduct full microkernel threat modeling and review security boundaries.

---

## 2. Component-Focused Development Backlog

Every OS component follows a strict 5-stage progression:
`Implement/Extend` → `Test & Verify` → `Bench & Fuzz` → `Document` → `Automate in CI`

1. **Boot & Firmware**: Secure boot verification, measured boot hashes, QEMU boot matrix.
2. **Kernel Core & Scheduler**: BORE scheduler tuning, EDF real-time scheduling, priority boosting.
3. **Memory & Allocators**: Paged/NonPaged pool boundary enforcement, zero-on-free scrubbing (`sigma_secure_alloc_compat.rs`), NUMA awareness.
4. **IPC & Capability System**: 64-bit `CapabilityToken` verification gates, zero-copy IPC pipes, token revocation.
5. **Drivers & HAL**: WDM `IoManager` / `DriverObject` / `DeviceObject` lifecycles, DMA bounds checking, USB/NVMe/PCI adapters.
6. **Filesystems & VFS**: SovereignLatticeFS, crash-consistency validation, inode metadata fuzzing.
7. **Networking**: PQC TLS 1.3 0-RTT mesh network (`src/net/mesh.rs`), packet parser fuzzing, zero-copy sockets.
8. **Userland & Runtimes**: WASM/WASI sandboxed userland apps, POSIX shims, Zenith desktop compositor.

---

## 3. Systematic Competitor Idea Absorption Framework

SigmaOS continuously evaluates mature open-source OS projects:

| Target Project | Area to Adapt / Borrow | Actionable Implementation Goal |
| :--- | :--- | :--- |
| **Redox OS** | Scheme-based VFS & Userspace Drivers | Implement userland process launchers with capability-restricted scheme URLs. |
| **seL4** | Formal Capability Invariants & Revocation | Property-based testing (`proptest`) for token delegation & revocation trees. |
| **Tock OS** | Capsule Driver Isolation | Isolated capsule driver interfaces preventing panicked drivers from downing kernel. |
| **Fuchsia** | Async IPC & Handle Lifecycles | Structured IPC handle transfers with automatic capability lifetime bounds. |
| **Linux / BSD** | Driver Matrix & VFS Compliance | Adopt LTP-inspired VFS test cases and BSD `pledge` / `unveil` sandboxing. |
| **WASI** | Sandboxed Execution Engine | Run userland applications within capability-gated WebAssembly runtimes. |

---

## 4. Operational Tools & Automation Inventory

- **PR Fast Checks**: `.github/workflows/pr_fast_checks.yml`
- **Per-File Test Runner**: `scripts/changed_files_rustc_tests.sh`
- **`no_std` Audit**: `scripts/no_std_check.sh`
- **Doc TODO Scanner**: `scripts/find_doc_todos.sh`
- **Microbenchmarks**: `tools/sigma_microbench_compat.rs`
- **Full Test Runner**: `./run_sigma_tests.sh`

---

## 5. Key Performance Indicators (KPIs)

- **CI Health**: ≥95% passing PR rate; average fast CI duration <3 minutes.
- **Test Coverage**: Standalone test suites for 100% of driver and kernel core modules.
- **Latency Targets**:
  - CapabilityToken verification: <15 ns.
  - Syscall dispatch latency: <25 ns.
  - Zero-copy IPC transfer: <50 ns for 256B messages.
- **Security**: 0 unhandled fuzzer crashes in release builds; 100% unsafe block justification coverage.
- **Reproducibility**: 100% bit-for-bit deterministic nightly build verification.
