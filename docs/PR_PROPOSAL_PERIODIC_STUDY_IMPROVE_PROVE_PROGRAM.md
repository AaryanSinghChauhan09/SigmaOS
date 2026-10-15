# Pull Request Proposal: SigmaOS Periodic "Study → Improve Component → Prove Improvement" Program

## Pull Request Summary
- **Title**: `feat(governance): Establish Periodic Study-Improve-Prove Program and Runtime Evidence Gates`
- **Branch**: `feature/periodic-study-improve-prove-program`
- **Status**: `Proposed / Specification Ready`
- **Scope**: Distro maintenance, periodic improvement cadences, 10-step component improvement loop, 12-component ownership mapping, 5-level component maturity scale, research repository layout, P0–P2 backlog, and 14 industry-standard operating rules.

---

## Executive Summary

SigmaOS shall run a **periodic “study → improve component → prove improvement” program**: each cycle selects one SigmaOS component, studies how a mature Linux/BSD project solves the same problem, implements a narrowly scoped improvement, and promotes the component only after runtime evidence.

The repository already provides component-specific agent guides, capability/status inventories, tests, scripts, installer assets, and a comparative gap-analysis document—but its README is clear that passing unit tests and having source code do not establish bootability, installation readiness, or hardware support.

This PR proposal establishes the fixed rhythm, component improvement loop, ownership boundaries, weekly schedule, monthly milestones, component maturity scale, research directory contract, backlog priorities, and 14 industry-standard operating rules that govern all component development across SigmaOS.

---

## 1. Periodic Improvement Cadence

Use a fixed rhythm so study always produces a measurable component improvement.

| Cadence | Purpose | Output |
|---|---|---|
| **Daily** | Keep build, tests, and boot status healthy | Status report and failure triage |
| **Weekly** | Study one upstream mechanism and improve one component | Research note, ADR, small PR, test evidence |
| **Biweekly** | Re-test a component against its runtime contract | Regression report and updated capability status |
| **Monthly** | Complete one component milestone | Runtime-validated feature or recovery path |
| **Quarterly** | Reassess architecture, gaps, security, and roadmap | Updated roadmap, archived stale plans, prioritized backlog |

SigmaOS states that roadmap items remain proposals until their completion criteria pass; therefore, every periodic cycle must end in evidence, not merely documentation.

---

## 2. Component Improvement Loop

Apply this 10-step loop to every SigmaOS component:

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

SigmaOS provides specialized agent documentation for kernel, memory, filesystem, network, security, desktop, package, distro compatibility, audio, Bluetooth, drivers, crypto, IPC, and architecture portability. Use those files as ownership boundaries.

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

## 4. Weekly Improvement Cycle

### Day 1: Choose one component and one gap
Select a narrow target, such as:
- Improve VFS path-resolution error handling.
- Add scheduler starvation detection.
- Harden syscall argument validation.
- Add virtio-blk error recovery.
- Improve package dependency conflict reporting.
- Add update rollback verification.
- Improve Zenith input latency diagnostics.

Do not choose “improve the kernel” or “improve security.”

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

## 5. Monthly Component Milestones

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

---

## 6. Component Maturity Model

Use a consistent maturity scale for every component.

| Level | Meaning | Promotion requirement |
|---|---|---|
| `Proposed` | Design exists | ADR and gap analysis |
| `Prototype` | Isolated implementation exists | Unit tests |
| `Integrated` | Connected to runtime path | Integration test plus QEMU/runtime evidence |
| `Hardened` | Handles failure, abuse, and edge cases | Fuzzing, negative tests, security review |
| `Supported` | Reliable enough for a release channel | Reproducible boot/install/update evidence and rollback |

SigmaOS’s `CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml` are the machine-readable sources for this maturity state.

---

## 7. Repository & Research Structure

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

This structure connects `Agents/`, `.agents/tasks`, `docs/`, `tests/`, `scripts/`, `tools/`, `fuzz/`, `installer/`, and `zenith_desktop/` into one repeatable improvement workflow.

---

## 8. Improvement Backlog

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

## 9. Industry-Standard Rules

1. **One component, one improvement per cycle**: Do not mix unrelated kernel, desktop, packaging, and security changes in one PR.
2. **Study produces a decision**: Every study cycle must produce an ADR stating what SigmaOS adopts, simplifies, changes, or rejects.
3. **Runtime evidence controls status**: A component cannot be promoted because its unit tests pass. It must be exercised through QEMU boot, installer, update, package, or hardware runtime paths.
4. **CI gates every improvement**: Required CI includes clean build, `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, formatting, and relevant runtime tests.
5. **Reproducibility**: Record commit hash, toolchain, container image, commands, timestamps, exit codes, and artifact hashes.
6. **Least privilege**: Minimum required permissions for agents, CI jobs, packages, and services. No broad credentials or unrestricted package-script execution.
7. **Security review is separate**: Mandatory security review for capability enforcement, sandboxing, syscalls, memory safety, cryptography, package execution, networking, and update mechanisms.
8. **Fail safely**: Define rollback or recovery behavior before merging boot, installer, update, filesystem, or storage changes.
9. **Regression tests are mandatory**: Every bug fix and component improvement must include a test that would fail if the behavior regressed.
10. **License compliance first**: Dual MIT/GPL-2.0 license compliance. Verify license compatibility and attribution or use clean-room design.
11. **No borrowed-status claims**: Upstream capabilities do not imply SigmaOS parity. Reference projects are design references, not status claims.
12. **Human approval for high-risk work**: Human approval required for bootloader, installer, update, security-boundary, package-execution, release, and destructive-storage changes.
13. **Archive stale research**: Research older than two quarters must be reaffirmed, updated, or archived.
14. **Keep documentation executable**: Link every component page to its tests, runtime evidence, owner, known limitations, and next improvement.

---

## 10. Metrics & First 30 Days

### Metrics
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

### First 30 Days Plan
- **Week 1 (Establish Truth)**: Audit `CAPABILITY_MATRIX.toml`, `FEATURE_STATUS.toml`, README, docs, and component pages. Create component maturity model and evidence policy. Implement QEMU x86_64 boot-to-shell validator. Produce first boot matrix.
- **Week 2 (Improve Kernel Foundation)**: Study xv6 memory and Linux paging. Improve memory initialization, frame allocation, and paging diagnostics. Add allocation and boot tests.
- **Week 3 (Improve Scheduling & Syscalls)**: Study xv6 process switching and Linux scheduler concepts. Improve scheduler fairness, context switching, and syscall validation. Add process-switch and syscall negative tests.
- **Week 4 (Improve VFS & Boot Integration)**: Study Linux VFS and xv6 filesystem. Improve path resolution, file descriptors, and RAM-disk operations. Add VFS integration tests and QEMU RAM-disk boot validation. Publish first monthly component-improvement report.
