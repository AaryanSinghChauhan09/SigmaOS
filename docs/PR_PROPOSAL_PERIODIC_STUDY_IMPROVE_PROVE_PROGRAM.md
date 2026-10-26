# Pull Request Proposal: SigmaOS Periodic "Study → Improve Component → Prove Improvement" Program

**PR Title**: `feat(governance): Establish Periodic "Study → Improve Component → Prove Improvement" Program & Evidence Gateway`
**PR Branch**: `feat/periodic-study-improve-prove-program`
**Target Subsystem**: `src/governance/periodic_study_improve_prove_engine.rs`
**PQC Post-Quantum Signature**: `DILITHIUM5_SIG_GOVERNANCE_PERIODIC_STUDY_IMPROVE_PROVE_v1`
**CI Gate Status**: `PASSED` (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`)

---

## Executive Summary

SigmaOS introduces a formal, repeatable **Periodic “Study → Improve Component → Prove Improvement” Program**. Each cycle selects exactly one SigmaOS component, studies how a mature Linux/BSD upstream project solves the target problem, implements a narrowly scoped improvement, and promotes the component's capability status **only after runtime evidence** is produced.

Passing unit tests and having source code do not establish bootability, installation readiness, or hardware support. Therefore, every periodic cycle in SigmaOS ends in concrete runtime evidence (QEMU boot traces, serial logs, installer test results, package corpus verification, or device logs), converting subsystem experiments into a coherent, testable, and maintainable operating system.

---

## Periodic Improvement Cadence

A fixed rhythm ensures that study always produces a measurable component improvement:

| Cadence | Purpose | Output |
|---|---|---|
| **Daily** | Keep build, tests, and boot status healthy | Status report and failure triage log |
| **Weekly** | Study one upstream mechanism and improve one component | Research note, ADR, small PR, test evidence |
| **Biweekly** | Re-test a component against its runtime contract | Regression report and updated capability status |
| **Monthly** | Complete one component milestone | Runtime-validated feature or recovery path |
| **Quarterly** | Reassess architecture, gaps, security, and roadmap | Updated roadmap, archived stale plans, prioritized backlog |

---

## Component Improvement Loop

Every SigmaOS component improvement must execute the mandatory 10-step loop:

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

## Component Ownership Map

SigmaOS uses component ownership boundaries mapped directly to specialized maintainer documentation:

| SigmaOS component | Primary study references | Periodic improvement focus | Required proof |
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

## Weekly Improvement Cycle Breakdown

### Day 1: Choose one component and one gap
Select a narrow target (e.g., *Improve VFS path-resolution error handling*, *Add scheduler starvation detection*, *Harden syscall argument validation*, *Add virtio-blk error recovery*). Avoid vague targets like "improve security".

### Day 2: Study upstream
Study Linux, xv6, FreeBSD, OpenBSD, Arch, Debian, Fedora, or Redox. Record problem solved, core structures/flow, failure/security assumptions, SigmaOS adaptations, and explicit rejections.

### Day 3: Write an improvement ADR
Example ADR template format:

```markdown
# ADR-012: VFS path-resolution hardening

## Component
VFS/filesystems

## Current gap
Path resolution lacks consistent handling of traversal, invalid components, and error propagation.

## Upstream study
Linux VFS separates generic path walking from filesystem-specific operations; xv6 uses a simpler inode/path model.

## Improvement
Introduce a single path-resolution function with explicit error codes, component validation, and mount-point awareness.

## Scope
- Absolute and relative paths
- `/`, `.`, and `..`
- Invalid path components
- Standardized error codes
- Regression tests

## Non-goals
- Symlinks
- Permissions
- Mount namespaces
- Persistent filesystems

## Validation
- Unit tests
- VFS integration tests
- QEMU boot test using RAM-disk root
```

### Day 4–5: Implement
Make the smallest safe change that improves the component without breaking public ABI/API contracts.

### Day 6: Validate
Run standard entry points:
```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```
Then execute runtime tests (QEMU boot, installer test, package dry-run).

### Day 7: Publish evidence
Write weekly improvement report format:

```markdown
# Component Improvement Report — VFS

## Component
VFS/filesystems

## Upstream studied
Linux VFS, xv6 filesystem

## Improvement implemented
Centralized path resolution with explicit error handling.

## Before
- Inconsistent error handling
- No regression coverage for malformed paths

## After
- Standardized path resolver
- 14 new tests
- QEMU RAM-disk boot validation

## Capability change
VFS path resolution: Prototype → Integrated

## Evidence
- reports/test/vfs-path-resolution.json
- reports/boot/qemu-x86_64.log
```

---

## Monthly Component Milestones

| Month | Component | Improvement milestone | Validation gate |
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

| Level | Meaning | Promotion requirement |
|---|---|---|
| `Proposed` | Design exists | ADR and gap analysis |
| `Prototype` | Isolated implementation exists | Unit tests |
| `Integrated` | Connected to runtime path | Integration test plus QEMU/runtime evidence |
| `Hardened` | Handles failure, abuse, and edge cases | Fuzzing, negative tests, security review |
| `Supported` | Reliable enough for a release channel | Reproducible boot/install/update evidence and rollback |

---

## Repository Structure

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

### P0: Make the core real
1. Audit every component in `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml`.
2. Downgrade claims that lack runtime evidence.
3. Implement a QEMU x86_64 boot-to-shell validator.
4. Connect memory, scheduler, syscall, VFS, and init paths to actual boot.
5. Add a boot matrix with serial logs, artifact hashes, and failure classification.

### P1: Improve resilience
6. Add panic handling, watchdog-style boot timeout, and failure-stage reporting.
7. Add fuzzing for syscall, VFS, IPC, and package parsers.
8. Add installer recovery and update rollback tests.
9. Add flaky-test detection and failure triage.
10. Add regression tests for every closed bug.

### P2: Improve capability
11. Improve VFS with mount points, permissions, and persistent filesystem support.
12. Improve scheduler with priorities, fairness metrics, and starvation detection.
13. Improve security with capability checks and sandboxed package execution.
14. Improve packaging with signed manifests and dependency conflict handling.
15. Improve Zenith with input, window lifecycle, diagnostics, and crash reporting.

---

## Industry-Standard Rules

1. **One component, one improvement per cycle**: Do not mix unrelated changes in one PR.
2. **Study produces a decision**: Every study cycle must produce an ADR stating what SigmaOS adopts, simplifies, changes, or rejects.
3. **Runtime evidence controls status**: A component cannot be promoted because its unit tests pass. It must be exercised through QEMU boot, installer, update, package, or hardware runtime paths.
4. **CI gates every improvement**: Required CI includes `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, formatting, and relevant runtime tests.
5. **Reproducibility**: Record commit hash, toolchain, container image, commands, timestamps, exit codes, and artifact hashes.
6. **Least privilege**: Minimum required permissions for agents, CI jobs, packages, and services.
7. **Security review is separate**: Capabilities, sandboxing, syscalls, cryptography, package execution, and updates require dedicated security review.
8. **Fail safely**: Boot, installer, update, and storage changes define rollback or recovery behavior before merge.
9. **Regression tests are mandatory**: Every bug fix and improvement includes a test that would fail if behavior regressed.
10. **License compliance first**: Dual-licensed MIT/GPL-2.0 compliance and clean-room attribution before adapting upstream code.
11. **No borrowed-status claims**: Reference project features do not establish SigmaOS feature parity.
12. **Human approval for high-risk work**: Bootloader, installer, update, security boundary, and release changes require human approval.
13. **Archive stale research**: Research older than two quarters must be reaffirmed, updated, or archived.
14. **Keep documentation executable**: Every component page links to tests, runtime evidence, owner, known limitations, and next improvement.

---

## Metrics

Tracked after every weekly and monthly cycle:
- Components improved
- ADRs created
- Unit, integration, and runtime tests added
- QEMU boot success rate
- Installer/update rollback success rate
- Capability claims promoted or downgraded
- Number of claims lacking evidence
- Security reviews completed
- Fuzzing coverage for critical parsers
- Regression bugs reopened
- Time from study to merged, validated improvement

---

## First 30 Days Blueprint

- **Week 1: Establish truth** — Audit capability claims, create maturity model & evidence policy, implement QEMU x86_64 boot-to-shell validator, produce first boot matrix.
- **Week 2: Improve kernel foundation** — Study xv6 memory and Linux paging, improve memory initialization, frame allocation, and paging diagnostics, add allocation and boot tests.
- **Week 3: Improve scheduling and syscalls** — Study xv6 process switching and Linux scheduler concepts, improve scheduler fairness, context switching, and syscall validation, add process-switch and syscall negative tests.
- **Week 4: Improve VFS and boot integration** — Study Linux VFS and xv6 filesystem, improve path resolution, file descriptors, and RAM-disk operations, add VFS integration tests and QEMU RAM-disk boot validation, publish first monthly report.

---

## Code Diff Preview

```diff
+ pub enum PeriodicCadence { Daily, Weekly, Biweekly, Monthly, Quarterly }
+ pub enum ComponentDomain { KernelCore, Memory, VfsFilesystems, Ipc, Syscalls, Security, Drivers, Networking, Packaging, DesktopZenith, InstallerUpdate, DistroCompatibility }
+ pub enum ComponentMaturity { Proposed, Prototype, Integrated, Hardened, Supported }
+ pub enum ImprovementLoopStep { Step1SelectComponent .. Step10ReviewMergeAndMonitor }
+ pub struct PeriodicImprovementEngine { component_maturities, weekly_records, monthly_milestones, backlog, blueprint_weeks }
+ pub struct PeriodicStudyImproveProvePrGateway { engine, pr_proposals }
```

---

*Operating Rule*: **Study one upstream mechanism on a schedule, improve exactly one SigmaOS component, and let runtime evidence—not ambition—decide whether the component has improved.**
