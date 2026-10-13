# PR Proposal: SigmaOS Periodic "Study -> Improve Component -> Prove Improvement" Program

## Executive Summary

SigmaOS introduces a formal, repeatable **periodic "study -> improve component -> prove improvement" program**. Each cycle selects exactly one SigmaOS component, studies how a mature Linux, BSD, or reference OS project solves the problem, implements a narrowly scoped, safe improvement, and promotes the component's maturity level only after verifying runtime test evidence.

Passing unit tests and having source code in the repository do not alone establish bootability, installation readiness, or hardware support. Therefore, every periodic improvement cycle ends in runtime evidence (e.g. QEMU boot traces, device logs, package corpus dry-runs, or install/rollback verification) rather than documentation claims alone.

---

## 1. Periodic Improvement Cadence

| Cadence | Purpose | Output Deliverable |
|---|---|---|
| **Daily** | Keep build, tests, and boot status healthy | Status report and failure triage log (`research/reports/daily/`) |
| **Weekly** | Study one upstream mechanism and improve one component | Upstream research note, ADR, small PR, and runtime test evidence (`research/reports/weekly/`) |
| **Biweekly** | Re-test a component against its runtime contract | Regression report and updated capability matrix status (`research/reports/biweekly/`) |
| **Monthly** | Complete one component milestone | Runtime-validated feature or recovery path (`research/reports/monthly/`) |
| **Quarterly** | Reassess architecture, gaps, security, and roadmap | Updated roadmap, archived stale research, prioritized backlog (`research/reports/quarterly/`) |

---

## 2. Component Improvement Loop

Every SigmaOS component follows a mandatory 10-step loop:

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

A component improves only when its behavior, tests, documentation, and runtime validation all move forward together.

---

## 3. Component Ownership Map

Specialized agent documentation and ownership boundaries align with SigmaOS subsystems:

| SigmaOS Component | Primary Study References | Periodic Improvement Focus | Required Proof Standard |
|---|---|---|---|
| **Kernel Core** | Linux, xv6, Redox | Boot, interrupts, syscalls, scheduling, panic handling | QEMU x86_64 boot trace |
| **Memory** | Linux, xv6, Redox | Frame allocator, paging, slab/buddy allocators, leak detection | Allocation tests and memory runtime traces |
| **VFS / Filesystems** | Linux, xv6, FreeBSD | Path resolution, mounts, file operations, crash consistency | Filesystem conformance tests & RAM-disk boot |
| **IPC** | Linux, FreeBSD | Pipes, signals, shared memory, message queues, eventfd | Cross-process interoperability tests |
| **Syscalls** | Linux, FreeBSD | ABI stability, argument validation, error codes, coverage | Syscall table & negative error path tests |
| **Security** | OpenBSD, Linux, FreeBSD | Capabilities, sandboxing, seccomp-like promises, `unveil` | Denial tests and threat model verification |
| **Drivers** | Linux, FreeBSD | Device probe, initialization, error handling, VirtIO | QEMU VirtIO device logs & probe traces |
| **Networking** | Linux, FreeBSD | Socket API, TCP/IP stack, packet filtering, WireGuard | Network integration tests & veth isolation |
| **Packaging** | Arch, Debian, Fedora | Metadata, dependency resolution, install, update, rollback | Package corpus test suite & dry-run |
| **Desktop / Zenith** | Linux Mint, Omarchy, Redox | Input dispatch, window lifecycle, launcher, diagnostics | Direct framebuffer graphical smoke test |
| **Installer / Update** | Debian, Fedora, Mint | Clean install, atomic update, rollback, recovery | Install and Timeshift-style rollback tests |
| **Distro Compatibility** | Arch, Debian, Fedora, FreeBSD | Metadata compatibility, ABI boundaries, sandboxing | Compatibility matrix verification |

---

## 4. Weekly Improvement Cycle

- **Day 1 (Choose Gap):** Select a single, narrow gap (e.g. VFS path resolution error handling, scheduler starvation detection, syscall argument validation).
- **Day 2 (Study Upstream):** Analyze upstream designs in Linux, xv6, FreeBSD, OpenBSD, Arch, Debian, Fedora, or Redox. Record assumptions, trade-offs, and what SigmaOS explicitly adopts, simplifies, or rejects.
- **Day 3 (Write ADR):** Draft an Architecture Decision Record in `research/decisions/` defining scope, non-goals, control flow, and validation criteria.
- **Day 4-5 (Implement):** Implement the minimal safe change in Rust without breaking public subsystem contracts.
- **Day 6 (Validate):** Execute standard repository entry points (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, `make format`) and the component-specific runtime test (QEMU boot, package dry-run, or installer test).
- **Day 7 (Publish Evidence):** Publish weekly report in `research/reports/weekly/`, update `CAPABILITY_MATRIX.toml` or `FEATURE_STATUS.toml`, and link execution logs.

---

## 5. Monthly Component Milestones

| Month | Component | Improvement Milestone | Validation Gate |
|---|---|---|---|
| **Month 1** | Kernel / Boot | Reproducible QEMU x86_64 boot-to-shell | Serial log and boot matrix |
| **Month 2** | Memory | Frame allocator, paging, and leak detection | Allocation and boot runtime traces |
| **Month 3** | Scheduler | Fair scheduling, preemption, and starvation protection | Process-switch trace |
| **Month 4** | VFS | Path resolution, file descriptors, RAM-disk operations | Filesystem conformance suite |
| **Month 5** | Processes / IPC | Pipes, signals, process lifecycle | Interop test suite |
| **Month 6** | Installer / Update | Clean install, atomic update, rollback | Install and recovery tests |
| **Month 7** | Security | Capability checks and sandbox policy | Denial and threat model tests |
| **Month 8** | Drivers | VirtIO-blk, VirtIO-net, VGA framebuffer | QEMU device logs |
| **Month 9** | Packaging | Metadata parsing and dependency conflict resolution | Package corpus test suite |
| **Month 10** | Desktop | Minimal graphical session and input dispatch | Graphical smoke test |
| **Month 11** | Networking | Socket API, TCP/IP stack path | Network integration test |
| **Month 12** | Release Engineering | Nightly artifact, known issues, rollback docs | Release checklist |

---

## 6. Component Maturity Model

| Maturity Level | Meaning | Promotion Standard |
|---|---|---|
| `Proposed` | Design exists | Written ADR and gap analysis |
| `Prototype` | Isolated implementation exists | Unit test pass |
| `Integrated` | Connected to runtime path | Integration test plus QEMU/runtime evidence |
| `Hardened` | Handles failure, abuse, and edge cases | Fuzzing, negative error path tests, security review |
| `Supported` | Reliable enough for release channel | Reproducible boot/install/update evidence & rollback safety |

---

## 7. Repository Structure

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

## 8. 14 Industry-Standard Rules

1. **Rule 1: One component, one improvement per cycle** — No mixing unrelated changes in a single PR.
2. **Rule 2: Study produces a decision** — Every study cycle produces an ADR.
3. **Rule 3: Runtime evidence controls status** — Only runtime execution moves capability maturity.
4. **Rule 4: CI gates every improvement** — Clean build, unit tests, script suite, formatting.
5. **Rule 5: Reproducibility** — Commit hash, toolchain, commands, exit codes, hashes recorded.
6. **Rule 6: Least privilege** — Minimum required permissions for agents, CI jobs, and scripts.
7. **Rule 7: Security review is separate** — Dedicated security review for boundary changes.
8. **Rule 8: Fail safely** — Pre-defined rollback and recovery behavior before merge.
9. **Rule 9: Regression tests are mandatory** — Every fix includes a regression test.
10. **Rule 10: License compliance first** — Dual MIT/GPL-2.0 compatibility and clean room design.
11. **Rule 11: No borrowed-status claims** — Reference capabilities do not imply SigmaOS parity without proof.
12. **Rule 12: Human approval for high-risk work** — Bootloader, installer, update, security boundary changes require human signoff.
13. **Rule 13: Archive stale research** — Research older than 2 quarters must be reaffirmed or archived.
14. **Rule 14: Keep documentation executable** — Documentation links to tests, evidence, owner, and limits.

---

## 9. Verification & Acceptance Criteria

- All governance structures and data models in `src/governance/periodic_study_improve_prove_engine.rs` pass unit and integration testing.
- `cargo check --lib`, `cargo test --lib`, and `./run_sigma_tests.sh` run completely green.
- `CAPABILITY_MATRIX.toml` and capability reports accurately reflect evidence-gated promotion rules.
