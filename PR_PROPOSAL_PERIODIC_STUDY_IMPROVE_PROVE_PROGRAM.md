# PR Proposal: Periodic "Study → Improve Component → Prove Improvement" Program

## Overview

SigmaOS implements a **periodic “study → improve component → prove improvement” program**. Each cycle selects one SigmaOS component, studies how a mature Linux/BSD project solves the same problem, implements a narrowly scoped improvement, and promotes the component maturity level **only after runtime evidence** (QEMU boot, installer dry-run, serial logs, or hardware execution) is provided.

This proposal formalizes the periodic improvement governance, establishing the operating cadences, component improvement loop, component ownership boundaries, weekly schedule, monthly component milestones, component maturity model, research directory tree, improvement backlog, 14 industry-standard operating rules, and core evaluation metrics.

---

## Periodic Improvement Cadence

A fixed, predictable rhythm ensures that upstream research directly translates into measurable, testable component improvements.

| Cadence | Purpose | Output Artifact |
|---|---|---|
| **Daily** | Keep build, unit/integration tests, and QEMU boot status healthy | Status report and failure triage log |
| **Weekly** | Study one upstream mechanism and improve one component | Upstream research note, ADR, small PR, test evidence |
| **Biweekly** | Re-test a component against its runtime contract | Regression report and updated capability status |
| **Monthly** | Complete one component milestone | Runtime-validated feature or recovery path |
| **Quarterly** | Reassess architecture, gaps, security, and roadmap | Updated roadmap, archived stale plans, prioritized backlog |

*Operating Principle*: Roadmap items remain proposals until their completion criteria pass; every periodic cycle must end in verifiable runtime evidence, not merely code or documentation.

---

## Component Improvement Loop

Every SigmaOS component improvement must progress through the mandatory 10-step loop:

```text
1. SELECT COMPONENT
2. MEASURE CURRENT STATE
3. SELECT UPSTREAM REFERENCE
4. STUDY THE MECHANISM
5. WRITE AN IMPROVEMENT PROPOSAL
6. IMPLEMENT THE SMALLEST SAFE CHANGE
7. RUN UNIT, INTEGRATION, AND RUNTIME TESTS
8. UPDATE DOCUMENTATION AND CAPABILITY STATUS
9. ADD REGRESSION PROTECTION
10. REVIEW, MERGE, AND MONITOR
```

A component is promoted only when its code behavior, automated test suite, documentation, and runtime validation move forward together in lockstep.

---

## Component Ownership Map

SigmaOS leverages specialized documentation boundaries (`.agents/runbooks/`, `.agents/registry.toml`, and subsystem modules) to define explicit ownership and proof requirements across 12 core OS domains:

| SigmaOS Component | Primary Study References | Periodic Improvement Focus | Required Proof |
|---|---|---|---|
| **Kernel core** | Linux, xv6, Redox | Boot, interrupts, syscalls, scheduling, panic handling | QEMU boot trace |
| **Memory** | Linux, xv6, Redox | Frame allocator, paging, slab/buddy allocators, leak detection | Allocation tests and runtime traces |
| **VFS/filesystems** | Linux, xv6, FreeBSD | Path resolution, mounts, file operations, crash consistency | Filesystem conformance tests |
| **IPC** | Linux, FreeBSD | Pipes, signals, shared memory, message queues, eventfd | Cross-process interoperability tests |
| **Syscalls** | Linux, FreeBSD | ABI stability, validation, error codes, syscall coverage | Syscall table and negative tests |
| **Security** | OpenBSD, Linux, FreeBSD | Capabilities, sandboxing, seccomp-like restrictions, `unveil` concepts | Denial tests and threat model |
| **Drivers** | Linux, FreeBSD | Device probe, initialization, error handling, virtio devices | QEMU device logs |
| **Networking** | Linux, FreeBSD | Socket API, TCP/IP, packet filtering, WireGuard concepts | Network integration tests |
| **Packaging** | Arch, Debian, Fedora | Metadata, dependencies, install, update, rollback | Package test corpus |
| **Desktop/Zenith** | Linux Mint, Omarchy, Redox | Input, windows, launcher, diagnostics, recovery | Graphical smoke test |
| **Installer/update** | Debian, Fedora, Mint | Clean install, atomic update, rollback, recovery | Install and rollback tests |
| **Distro compatibility** | Arch, Debian, Fedora, FreeBSD | Metadata compatibility, ABI boundaries, sandboxing | Compatibility matrix |

---

## Weekly Improvement Cycle Schedule

### Day 1: Choose One Component and One Gap
Select a single, narrow target. Examples:
- Standardize VFS path-resolution error codes (`ENOENT`, `ENOTDIR`).
- Add EEVDF scheduler starvation detection counters.
- Harden syscall user-pointer alignment and bounds checks.
- Implement virtio-blk ring buffer error recovery.
- Improve package dependency cycle reporting.
- Validate A/B partition rollback logic in the installer.
- Measure Zenith input event processing latency.

*Anti-pattern*: Do not choose broad, unmeasurable goals such as "improve security" or "refactor the kernel."

### Day 2: Study Upstream Reference
Examine upstream reference implementations in Linux, xv6, FreeBSD, OpenBSD, Arch, Debian, Fedora, or Redox. Record:
1. Problem solved by upstream design.
2. Core data structures and control flow.
3. Security, concurrency, and failure assumptions.
4. Mechanisms SigmaOS should adopt directly.
5. Simplifications or redesigns required for safe Rust.
6. Explicitly rejected upstream choices and justification.

### Day 3: Write Architectural Decision Record (ADR)
Author an ADR in `research/decisions/` capturing component scope, current gap, upstream findings, proposed improvement, non-goals, and validation gates.

### Day 4–5: Implement
Make the smallest safe change that delivers the target capability without violating existing subsystem contracts. Maintain strict single-component PR scope.

### Day 6: Validate
Run all mandatory validation scripts and commands:
```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```
Followed by the target component's runtime gate (QEMU boot, installer dry-run, package corpus test, or device execution).

### Day 7: Publish Evidence & Status
Generate a weekly improvement report in `research/reports/weekly/`, linking QEMU serial logs, JSON test matrices, ADR documents, and updating maturity state in `CAPABILITY_MATRIX.toml`.

---

## Monthly Component Milestones

Each month advances one core component to a concrete, evidence-backed milestone:

| Month | Component | Improvement Milestone | Validation Gate |
|---|---|---|---|
| **1** | Kernel/boot | Reproducible QEMU x86_64 boot-to-shell | Serial log and boot matrix |
| **2** | Memory | Frame allocator, paging, and leak detection | Allocation and boot tests |
| **3** | Scheduler | Fair scheduling and preemption | Process-switch trace |
| **4** | VFS | Path resolution, file descriptors, RAM-disk operations | Filesystem tests |
| **5** | Processes/IPC | Pipes, signals, process lifecycle | Interop tests |
| **6** | Installer/update | Clean install, atomic update, rollback | Install/recovery tests |
| **7** | Security | Capability checks and sandbox policy | Denial tests |
| **8** | Drivers | virtio-blk, virtio-net, VGA | Device logs |
| **9** | Packaging | Metadata parsing and dependency resolution | Package corpus tests |
| **10** | Desktop | Minimal graphical session and input | Graphical smoke test |
| **11** | Networking | Basic socket and network stack path | Network integration test |
| **12** | Release engineering | Nightly artifact, known issues, rollback docs | Release checklist |

---

## Component Maturity Model

A standardized 5-level maturity scale governs component lifecycle status:

| Maturity Level | Definition | Promotion Criteria |
|---|---|---|
| `Proposed` | Design document exists | ADR and gap analysis merged in `research/` |
| `Prototype` | Isolated implementation exists | Unit tests pass in `cargo test --lib` |
| `Integrated` | Connected to boot/runtime path | Integration test passes with QEMU/runtime log evidence |
| `Hardened` | Resilient against abuse & edge cases | Fuzzing, negative tests, and security review complete |
| `Supported` | Release-ready reliability | Reproducible boot/install/update evidence & verified rollback |

`CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml` serve as the single machine-readable sources of truth for maturity ratings.

---

## Repository Structure Contract

To support the periodic study-improve-prove workflow, the repository enforces the following directory conventions:

```text
research/
  upstream/
    linux/
    xv6/
    arch/
    mint/
    fedora/
    freebsd/
    openbsd/
    redox/
  decisions/
    ADR-001-boot-path.md
    ADR-002-memory-model.md
    ADR-003-vfs-interface.md
    ADR-004-scheduler.md
    ADR-005-package-format.md
    ADR-006-security-capabilities.md
  comparisons/
    boot.md
    memory.md
    scheduler.md
    filesystem.md
    security.md
    packaging.md
  reports/
    daily/
    weekly/
    monthly/
    quarterly/

.agents/
  runbooks/
    daily-health.md
    weekly-component-improvement.md
    monthly-milestone.md
    quarterly-review.md
  tasks/
    open/
    in-progress/
    done/
    blocked/
```

---

## Improvement Backlog

### P0: Make the Core Real
1. Audit every component in `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml`.
2. Downgrade claims lacking runtime log evidence.
3. Implement a QEMU x86_64 boot-to-shell automated validator.
4. Connect memory, scheduler, syscall, VFS, and init paths to physical/QEMU boot.
5. Generate a daily boot matrix with serial logs, artifact hashes, and failure classification.

### P1: Improve Resilience
6. Add panic handling, watchdog timer boot timeouts, and failure-stage reporting.
7. Integrate fuzzing engines for syscall table, VFS path parsing, IPC buffers, and package manifests.
8. Implement installer recovery and update rollback integration tests.
9. Add automated flaky-test detection and failure triage.
10. Enforce regression test inclusion for every resolved bug.

### P2: Expand Capabilities
11. Extend VFS with mount namespaces, permission verification, and persistent filesystem support.
12. Upgrade scheduler with real-time priority queues, fairness metrics, and starvation prevention.
13. Harden security with process capability bounds and sandboxed package execution.
14. Upgrade packaging engine with Dilithium PQC signed manifests and conflict resolution.
15. Enhance Zenith Desktop with input event routing, window lifecycle diagnostics, and crash reporting.

---

## 14 Industry-Standard Rules

1. **Rule 1: One Component, One Improvement Per Cycle** — Never mix unrelated kernel, desktop, packaging, and security modifications in a single PR.
2. **Rule 2: Study Produces a Decision** — Every study phase must yield an ADR detailing what SigmaOS adopts, simplifies, adapts, or rejects.
3. **Rule 3: Runtime Evidence Controls Status** — Passing unit tests alone cannot promote a component to `Integrated`, `Hardened`, or `Supported`. Promotion requires QEMU boot logs, installer traces, or hardware execution proof.
4. **Rule 4: CI Gates Every Improvement** — All PRs must pass `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, `make format`, and relevant runtime gates.
5. **Rule 5: Reproducibility** — Execution records must capture commit hashes, Rust toolchain version, environment variables, timestamps, exit codes, and artifact digests.
6. **Rule 6: Least Privilege** — CI runners and agent tools operate under minimum necessary permissions. No force-push permissions or unvetted script execution.
7. **Rule 7: Dedicated Security Review** — Capabilities, sandboxing, syscalls, memory management, cryptographic operations, package scripts, and update paths require independent security verification.
8. **Rule 8: Fail-Safe Behavior** — Bootloader, storage, installer, and update changes must define recovery or rollback mechanisms prior to merge.
9. **Rule 9: Mandatory Regression Tests** — Every bug fix and component enhancement must include a regression test that fails without the change.
10. **Rule 10: License Compliance First** — Respect dual MIT/GPL-2.0 licensing. Verify upstream license compatibility or utilize clean-room specification notes.
11. **Rule 11: No Borrowed-Status Claims** — Upstream capabilities in Linux or FreeBSD do not imply feature parity in SigmaOS without explicit runtime evidence.
12. **Rule 12: Human Approval for High-Risk Changes** — Bootloader, kernel memory layout, security boundaries, package script execution, and destructive storage operations require human maintainer signoff.
13. **Rule 13: Archive Stale Research** — Research exceeding two quarters without implementation must be re-validated or archived.
14. **Rule 14: Executable Documentation** — Documentation pages must directly link to automated test suites, runtime evidence logs, domain owners, known issues, and scheduled improvements.

---

## Core Tracking Metrics

- Total components improved per cycle.
- ADRs created and approved in `research/decisions/`.
- Automated unit, integration, and runtime tests added.
- QEMU boot success rate across target architectures.
- Installer clean install and atomic update rollback success rate.
- Capability claims promoted vs. downgraded based on evidence.
- Fuzzing coverage for critical parsers (VFS, syscalls, IPC, packaging).
- Security reviews completed and vulnerability findings resolved.
- Time delta from upstream study to merged, evidence-validated PR.

---

## First 30 Days Execution Schedule

- **Week 1: Establish Baseline Truth** — Audit `CAPABILITY_MATRIX.toml`, create maturity evidence policy, implement automated QEMU boot validator, and publish initial boot matrix.
- **Week 2: Kernel & Memory Foundation** — Study xv6 and Linux paging models, improve frame allocation diagnostics and page table checks, and add allocation regression tests.
- **Week 3: Scheduler & Syscall Hardening** — Study xv6 process table and Linux scheduler, improve preemption and syscall pointer validation, and add negative syscall tests.
- **Week 4: VFS & Boot Path Integration** — Study Linux VFS namei and xv6 file operations, introduce centralized path resolver with error code standardization, run QEMU RAM-disk boot validation, and publish Monthly Milestone #1 report.
