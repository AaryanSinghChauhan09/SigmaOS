# Pull Request Proposal: Periodic "Study → Improve Component → Prove Improvement" Program Roadmap for SigmaOS

**PR Title:** `feat(governance): Establish Periodic "Study → Improve Component → Prove Improvement" Program`
**PR Branch:** `feature/periodic-study-improve-prove-program`
**Target Branch:** `main`
**Status:** Proposal / Specification Ready
**Component Scope:** Architecture, Governance, Kernel, Memory, VFS, IPC, Syscalls, Security, Drivers, Networking, Packaging, Desktop/Zenith, Installer/Update, Distro Compatibility, Release Engineering

---

## 1. Executive Summary & Problem Statement

SigmaOS should run a **periodic “study → improve component → prove improvement” program**: each cycle selects one SigmaOS component, studies how a mature Linux/BSD project solves the same problem, implements a narrowly scoped improvement, and promotes the component only after runtime evidence.

The repository already has component-specific agent guides, capability/status inventories, tests, scripts, installer assets, and a comparative gap-analysis document—but its README is clear that passing unit tests and having source code do not establish bootability, installation readiness, or hardware support. Code presence and passing unit tests alone do not prove kernel integration—only runtime test evidence and reproducible QEMU verification gates control capability status.

---

## 2. Periodic Improvement Cadence

Use a fixed rhythm so study always produces a measurable component improvement.

| Cadence | Purpose | Duration | Required Output |
|---|---|---|---|
| **Daily** | Keep build, tests, and boot status healthy | 30–60 minutes | Status report and failure triage |
| **Weekly** | Study one upstream mechanism and improve one component | 3–5 hours | Research note, ADR, small PR, test evidence |
| **Biweekly** | Re-test a component against its runtime contract | 1 day | Regression report and updated capability status |
| **Monthly** | Complete one component milestone | 2–4 weeks | Runtime-validated feature or recovery path |
| **Quarterly** | Reassess architecture, gaps, security, and roadmap | 1 week | Updated roadmap, archived stale plans, prioritized backlog |

SigmaOS states that roadmap items remain proposals until their completion criteria pass; therefore, every periodic cycle must end in evidence, not merely documentation.

---

## 3. Component Improvement Loop

Apply this loop to every SigmaOS component:

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

## 4. Component Ownership Map

SigmaOS already provides specialized agent documentation for kernel, memory, filesystem, network, security, desktop, package, distro compatibility, audio, Bluetooth, drivers, crypto, IPC, and architecture portability. Use those files as ownership boundaries.

| SigmaOS component | Primary study references | Periodic improvement focus | Required proof |
|---|---|---|---|
| Kernel core | Linux, xv6, Redox | Boot, interrupts, syscalls, scheduling, panic handling | QEMU boot trace |
| Memory | Linux, xv6, Redox | Frame allocator, paging, slab/buddy allocators, leak detection | Allocation tests and runtime traces |
| VFS/filesystems | Linux, xv6, FreeBSD | Path resolution, mounts, file operations, crash consistency | Filesystem conformance tests |
| IPC | Linux, FreeBSD | Pipes, signals, shared memory, message queues, eventfd | Cross-process interoperability tests |
| Syscalls | Linux, FreeBSD | ABI stability, validation, error codes, syscall coverage | Syscall table and negative tests |
| Security | OpenBSD, Linux, FreeBSD | Capabilities, sandboxing, seccomp-like restrictions, `unveil` concepts | Denial tests and threat model |
| Drivers | Linux, FreeBSD | Device probe, initialization, error handling, virtio devices | QEMU device logs |
| Networking | Linux, FreeBSD | Socket API, TCP/IP, packet filtering, WireGuard concepts | Network integration tests |
| Packaging | Arch, Debian, Fedora | Metadata, dependencies, install, update, rollback | Package test corpus |
| Desktop/Zenith | Linux Mint, Omarchy, Redox | Input, windows, launcher, diagnostics, recovery | Graphical smoke test |
| Installer/update | Debian, Fedora, Mint | Clean install, atomic update, rollback, recovery | Install and rollback tests |
| Distro compatibility | Arch, Debian, Fedora, FreeBSD | Metadata compatibility, ABI boundaries, sandboxing | Compatibility matrix |

---

## 5. Weekly Improvement Cycle

### Day 1: Choose one component and one gap

Select a narrow target, such as:
- Improve VFS path-resolution error handling.
- Add scheduler starvation detection.
- Harden syscall argument validation.
- Add virtio-blk error recovery.
- Improve package dependency conflict reporting.
- Add update rollback verification.
- Improve Zenith input latency diagnostics.

*Do not choose “improve the kernel” or “improve security.”*

### Day 2: Study upstream

Study the relevant implementation in Linux, xv6, FreeBSD, OpenBSD, Arch, Debian, Fedora, or Redox.

Record:
- The problem the upstream design solves.
- Core data structures and control flow.
- Failure, security, and performance assumptions.
- What SigmaOS should adopt directly.
- What must be simplified or redesigned for Rust/SigmaOS.
- What SigmaOS explicitly rejects and why.

### Day 3: Write an improvement ADR

Example:

```markdown
# ADR-012: VFS path-resolution hardening

## Component
VFS/filesystems

## Current gap
Path resolution lacks consistent handling of traversal,
invalid components, and error propagation.

## Upstream study
Linux VFS separates generic path walking from filesystem-specific
operations; xv6 uses a simpler inode/path model.

## Improvement
Introduce a single path-resolution function with explicit
error codes, component validation, and mount-point awareness.

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

Make the smallest change that improves the component without breaking its public contract. One PR should address one component improvement.

### Day 6: Validate

Run the repository’s documented checks:

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

Then run the component’s runtime test—typically QEMU boot, installer test, package dry-run, or device-specific test. SigmaOS identifies these commands as its current build and test entry points.

### Day 7: Publish evidence

Write a weekly improvement report:

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

## 6. Monthly Component Milestones

Each month should improve one component to a new, verifiable milestone.

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

This sequence follows SigmaOS’s stated near-term priorities: validate boot-to-shell, make installer/update paths recoverable, document hardware with evidence, and connect subsystem models to runtime paths.

---

## 7. Component Maturity Model

Use a consistent maturity scale for every component.

| Level | Meaning | Promotion requirement |
|---|---|---|
| `Proposed` | Design exists | ADR and gap analysis |
| `Prototype` | Isolated implementation exists | Unit tests |
| `Integrated` | Connected to runtime path | Integration test plus QEMU/runtime evidence |
| `Hardened` | Handles failure, abuse, and edge cases | Fuzzing, negative tests, security review |
| `Supported` | Reliable enough for a release channel | Reproducible boot/install/update evidence and rollback |

SigmaOS’s `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml` should be the machine-readable sources for this maturity state.

---

## 8. Repository Structure

Add a durable research-and-improvement area:

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

The repository already includes `Agents/`, `.agents/tasks`, `docs/`, `tests/`, `scripts/`, `tools/`, `fuzz/`, `installer/`, and `zenith_desktop/`; this structure connects those areas into one repeatable improvement workflow.

---

## 9. Improvement Backlog

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

## 10. Industry-Standard Rules

### Rule 1: One component, one improvement per cycle
Do not mix unrelated kernel, desktop, packaging, and security changes in one PR.

### Rule 2: Study produces a decision
Every study cycle must produce an ADR stating what SigmaOS adopts, simplifies, changes, or rejects.

### Rule 3: Runtime evidence controls status
A component cannot be promoted because its unit tests pass. It must be exercised through QEMU boot, installer, update, package, or hardware runtime paths. SigmaOS explicitly distinguishes unit-tested models from kernel integration and hardware support.

### Rule 4: CI gates every improvement
Required CI must include clean build, `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, formatting, and relevant runtime tests.

### Rule 5: Reproducibility
Every improvement must record commit hash, toolchain, container image, commands, timestamps, exit codes, and artifact hashes.

### Rule 6: Least privilege
Agents, CI jobs, packages, and services receive only minimum required permissions. No broad credentials, unrestricted package-script execution, or force-push access.

### Rule 7: Security review is separate
Capability enforcement, sandboxing, syscalls, memory safety, cryptography, package execution, networking, and update mechanisms require dedicated security review.

### Rule 8: Fail safely
Boot, installer, update, filesystem, and storage changes must define rollback or recovery behavior before merge.

### Rule 9: Regression tests are mandatory
Every bug fix and component improvement must include a test that would fail if the behavior regressed.

### Rule 10: License compliance first
SigmaOS is dual-licensed MIT/GPL-2.0. Before adapting upstream code, verify license compatibility and attribution; otherwise use clean-room design notes and original implementation.

### Rule 11: No borrowed-status claims
A capability in Linux, FreeBSD, OpenBSD, Arch, Fedora, or Redox does not imply the same capability in SigmaOS. The README states that reference projects are design references and their feature names alone do not establish SigmaOS parity.

### Rule 12: Human approval for high-risk work
Humans approve bootloader, installer, update, security-boundary, package-execution, release, and destructive-storage changes.

### Rule 13: Archive stale research
Research older than two quarters must be reaffirmed, updated, or archived. It cannot override current component status.

### Rule 14: Keep documentation executable
Every component page must link to its tests, runtime evidence, owner, known limitations, and next improvement.

---

## 11. Quantitative Metrics

Track these after every weekly and monthly cycle:
- Components improved.
- ADRs created.
- Unit, integration, and runtime tests added.
- QEMU boot success rate.
- Installer/update rollback success rate.
- Capability claims promoted or downgraded.
- Number of claims lacking evidence.
- Security reviews completed.
- Fuzzing coverage for critical parsers.
- Regression bugs reopened.
- Time from study to merged, validated improvement.

---

## 12. First 30 Days Schedule

### Week 1: Establish truth
- Audit `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, README, docs, and component pages.
- Create the component maturity model and evidence policy.
- Implement the QEMU x86_64 boot-to-shell validator.
- Produce the first boot matrix.

### Week 2: Improve kernel foundation
- Study xv6 memory and Linux paging.
- Improve memory initialization, frame allocation, and paging diagnostics.
- Add allocation and boot tests.

### Week 3: Improve scheduling and syscalls
- Study xv6 process switching and Linux scheduler concepts.
- Improve scheduler fairness, context switching, and syscall validation.
- Add process-switch and syscall negative tests.

### Week 4: Improve VFS and boot integration
- Study Linux VFS and xv6 filesystem.
- Improve path resolution, file descriptors, and RAM-disk operations.
- Add VFS integration tests and QEMU RAM-disk boot validation.
- Publish the first monthly component-improvement report.

---

The operating rule is: **study one upstream mechanism on a schedule, improve exactly one SigmaOS component, and let runtime evidence—not ambition—decide whether the component has improved.** That will gradually convert SigmaOS’s broad set of subsystem experiments into a coherent, testable, and maintainable operating system.
