# Pull Request Proposal: Periodic Research-and-Implementation Program Roadmap for SigmaOS

**PR Title:** `feat(research): Establish Periodic Research-and-Implementation Program`
**PR Branch:** `feature/periodic-research-implementation-program`
**Target Branch:** `main`
**Status:** Proposal / Specification Ready
**Component Scope:** Architecture, Research, Kernel, Security, VFS, Drivers, Packaging, Desktop, Release Engineering, Agent Infrastructure

---

## 1. Executive Summary & Problem Statement

SigmaOS must run a **periodic research-and-implementation program**: each cycle studies one narrow OS mechanism from Linux/BSD projects, converts it into an Architecture Decision Record (ADR), implements the smallest connected slice, validates it in QEMU/runtime tests, and records what was adopted, changed, or rejected.

While the repository already possesses extensive study targets, subsystem agent guides, capability matrix files, unit tests, scripts, installer assets, and comparative gap analysis, it requires a fixed cadence so research becomes working, verified code rather than an accumulating set of planning documents. Code presence and passing unit tests alone do not establish kernel integration or hardware support—only runtime test evidence and reproducible QEMU verification gates control capability status.

---

## 2. Periodic Operating Cadence

SigmaOS uses a four-tier cadence. Every cycle ends with an artifact: a research note, decision record, implementation PR, test result, or updated capability status.

| Cadence | Purpose | Duration | Required Output |
|---|---|---|---|
| **Daily** | Keep the system healthy | 30–60 minutes | Build, test, and QEMU boot status report |
| **Weekly** | Study and implement one focused mechanism | 3–5 hours | Research note + Architecture Decision Record + small implementation PR |
| **Monthly** | Complete one vertical OS capability | 2–4 weeks | Runtime-validated end-to-end feature slice |
| **Quarterly** | Reassess architecture and roadmap | 1 week | Updated capability matrix, gap register, and consolidated roadmap |

---

## 3. Weekly Study Cycle

Run the same five-stage cycle every week:

### Stage 1: Select One Mechanism (Day 1)
Choose one narrowly bounded topic (e.g., Linux scheduler run queues, xv6 process switching, Linux VFS path resolution, OpenBSD `pledge`/`unveil` confinement, Arch package dependency resolution, Fedora atomic updates, FreeBSD release engineering, virtio-blk driver initialization). Do not select broad topics such as "Linux," "BSD," or "package management."

### Stage 2: Study Upstream (Day 2)
Read upstream source code, documentation, design discussions, and known limitations.
Record:
- Problem solved
- Core data structures
- Control flow
- Failure and security assumptions
- Performance tradeoffs
- What SigmaOS should adopt, simplify, change, or reject

### Stage 3: Write a Design Note / ADR (Day 3)
Create an Architecture Decision Record under `research/decisions/` stating status, upstream study context, decision drivers, SigmaOS decision, scope, non-goals, and validation criteria.

### Stage 4: Implement the Smallest Slice (Days 4–5)
Implement only the slice defined in the ADR. Keep the PR small, self-contained, and scoped to one subsystem boundary.

### Stage 5: Validate & Publish Learning (Days 6–7)
Execute validation checks:
```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```
Run the relevant QEMU boot or subsystem integration test, and publish a weekly report under `research/reports/weekly/`.

---

## 4. Monthly Implementation Cycle

Each month closes one user-visible or architecturally important capability.

| Month | Study Theme | Implementation Outcome | Validation Gate |
|---|---|---|---|
| **1** | Boot and kernel entry | QEMU x86_64 boot-to-shell | Boot trace, serial log |
| **2** | Memory and scheduling | Frame allocator, paging, basic scheduler | Process-switch test |
| **3** | Syscalls and VFS | Syscall table, RAM-disk filesystem, file operations | File I/O tests |
| **4** | Processes and IPC | Processes, pipes, signals, FDs | Interop tests |
| **5** | Installer and updates | Clean install, atomic update, rollback | Install/recovery test |
| **6** | Security | Capability checks, sandbox policy, fuzzing | Denial tests |
| **7** | Desktop foundation | Framebuffer, input, window launch | Graphical smoke test |
| **8** | Drivers and hardware | virtio-blk, virtio-net, VGA | Device matrix |
| **9** | Packages | Metadata parse, dependency graph, dry-run | Package tests |
| **10** | Release engineering | Nightly artifact, known issues, rollback docs | Release checklist |

---

## 5. Quarterly Review Cycle

Every quarter, perform a strategic reset.

### Review Inputs
- Capability matrix (`CAPABILITY_MATRIX.toml`)
- Feature status (`FEATURE_STATUS.toml`)
- Boot matrix and test logs
- Failing or flaky test register
- Research notes and ADRs
- Upstream project changes & security advisories
- Hardware-support evidence and user-facing gaps

### Review Outputs
- Updated roadmap
- Retired or archived stale plans (> 2 quarters old)
- New P0/P1/P2 gap list
- Architecture decisions to revisit
- Security review priorities and next quarter's monthly themes

---

## 6. Study Curriculum

- **Cycle 1: xv6 and OSDev**
  - *Study:* Bootloader, GDT/IDT, interrupts, VGA, keyboard, xv6 process model, context switching, syscalls, VFS.
  - *Implement:* QEMU UEFI/BIOS boot path, serial console, memory-map parsing, frame allocator, paging, timer, keyboard, scheduler, init, shell.
  - *Validation:* QEMU boots to shell; command executes; shutdown is clean.
- **Cycle 2: Linux Kernel**
  - *Study:* Scheduler classes and run queues, VFS and mount semantics, syscall ABI, driver model, namespaces, capability model.
  - *Implement:* Dynamic scheduler, VFS abstraction, process lifecycle, signals, pipes, IPC, initial security boundary model.
  - *Validation:* Processes, files, pipes, and signals pass integration tests and QEMU runtime path.
- **Cycle 3: Arch Linux and ArchWiki**
  - *Study:* Package format, dependency resolution, transparent configuration, task-oriented troubleshooting.
  - *Implement:* Package metadata schema, dependency resolver, dry-run installation, configuration validation.
  - *Validation:* Package manifest parsed, dependencies resolved, installation simulated safely.
- **Cycle 4: Linux Mint**
  - *Study:* Onboarding, update guidance, troubleshooting, recovery, rollback.
  - *Implement:* Installer workflow, update manifest, rollback engine, recovery boot entry.
  - *Validation:* Clean QEMU install succeeds; failed update restores prior system state.
- **Cycle 5: Fedora**
  - *Study:* CI gating, atomic updates, developer experience, release quality process.
  - *Implement:* CI merge gates, nightly artifact pipeline, staged update activation, release checklist.
  - *Validation:* Nightly artifact produced only after build, tests, QEMU boot, install, and rollback pass.
- **Cycle 6: FreeBSD and OpenBSD**
  - *Study:* FreeBSD release engineering & jails, OpenBSD secure defaults (`pledge`/`unveil`), code audit culture, network/storage design.
  - *Implement:* Capability-based sandboxing, package-script isolation, signed update manifests, hardware support matrix.
  - *Validation:* Protected operations fail without capabilities; package scripts restricted to sandbox permissions.
- **Cycle 7: Redox OS and Rust OS Design**
  - *Study:* Rust ownership in OS components, userspace/kernel separation, capability-oriented design, safe kernel wrappers.
  - *Implement:* Rust module boundaries, unsafe-code policy, kernel/userspace contracts, memory-safe wrappers.
  - *Validation:* Unsafe code isolated, documented, and covered by tests/formal review.

---

## 7. Repository Structure

Stable research and implementation layout:

```text
research/
  upstream/
    xv6/
    linux/
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
    scheduler.md
    filesystem.md
    security.md
    packaging.md
    hardware.md
  reports/
    weekly/
    monthly/
    quarterly/

.agents/
  runbooks/
    weekly-study.md
    monthly-capability.md
    quarterly-review.md
  tasks/
    open/
    in-progress/
    done/
    blocked/
```

---

## 8. Task Templates

### Weekly Study Task (`.agents/tasks/open/weekly-template.toml`)
```toml
id = "STUDY-2026-W42"
title = "Study Linux VFS path resolution"
cycle = "weekly"
theme = "filesystem"
upstream = ["linux", "xv6"]
deliverables = [
  "research/upstream/linux/vfs-path-resolution.md",
  "research/decisions/ADR-008-vfs-path-resolution.md",
  "implementation PR",
  "test report"
]

success = [
  "design note explains upstream model",
  "SigmaOS adaptation is defined",
  "unit tests pass",
  "runtime path is identified",
  "capability status updated only with evidence"
]
```

### Monthly Capability Task (`.agents/tasks/open/monthly-template.toml`)
```toml
id = "CAP-2026-M10"
title = "Deliver QEMU x86_64 boot-to-shell"
cycle = "monthly"
priority = "P0"
owner = "boot-runtime-agent"

required_evidence = [
  "reports/build/build-manifest.json",
  "reports/test/test-report.json",
  "reports/boot/qemu-x86_64.log",
  "reports/boot/boot-matrix.json"
]

definition_of_done = [
  "clean build succeeds",
  "unit and integration tests pass",
  "QEMU reaches shell",
  "smoke command succeeds",
  "no kernel panic",
  "rollback or recovery path documented"
]
```

---

## 9. Industry-Standard Rules

1. **Rule 1: One mechanism per cycle:** Study and implement one narrow mechanism per weekly cycle.
2. **Rule 2: Research must produce a decision:** Every study cycle ends with an ADR stating what SigmaOS adopts, simplifies, changes, or rejects.
3. **Rule 3: Implementation must be a vertical slice:** Prefer a complete small path over disconnected subsystem improvements.
4. **Rule 4: Runtime evidence controls status:** Unit tests prove logic; QEMU boot, installer, update, and hardware tests prove the OS.
5. **Rule 5: CI gates every change:** Required CI includes clean build, `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, formatting, and QEMU smoke tests.
6. **Rule 6: Reproducibility is mandatory:** Build/test artifacts must record commit hash, toolchain, container image, commands, timestamps, exit codes, and hashes.
7. **Rule 7: License compliance first:** Dual-license MIT/GPL-2.0 compliance. Verify license compatibility and attribution before adaptation; otherwise use clean-room design notes.
8. **Rule 8: Security review is separate:** Dedicated security review for capability enforcement, sandboxing, syscalls, cryptography, package execution, networking, and updates.
9. **Rule 9: Least privilege:** Agents, CI jobs, packages, and services receive only required permissions.
10. **Rule 10: Fail safely:** Boot, installer, update, storage, and recovery changes must define rollback behavior before merge.
11. **Rule 11: Regression tests are mandatory:** Every implemented mechanism needs a test preventing behavioral regression.
12. **Rule 12: Human approval for high-risk work:** Humans approve bootloader, installer, update, security-boundary, package-execution, release, and destructive storage changes.
13. **Rule 13: Archive stale research:** Research older than two quarters must be reaffirmed, updated, or archived.
14. **Rule 14: No borrowed-status claims:** Reference projects do not establish SigmaOS parity claims without runtime evidence.

---

## 10. Program Metrics & First 30 Days

### Metrics Tracked
- Number of mechanisms studied vs. ADRs created vs. mechanisms implemented vs. runtime-validated
- QEMU boot success rate and test pass rate
- Capability claims upgraded or downgraded based on evidence
- Security reviews completed and stale research documents archived
- Cycle time from study to merged, validated implementation

### First 30 Days Execution Schedule
- **Week 1:** Initialize `research/`, ADR templates, weekly report template, study task templates. Study xv6 and OSDev bootloader patterns. Implement QEMU x86_64 UEFI boot with serial logging and produce boot trace.
- **Week 2:** Study xv6 memory management and Linux paging. Implement frame allocation, paging, early memory init, and memory tests.
- **Week 3:** Study xv6 processes and Linux scheduling. Implement minimal scheduler, context switching, and two-process alternation test.
- **Week 4:** Study Linux syscalls and xv6 filesystem. Implement minimal syscall table, RAM-disk VFS, boot-to-shell, file-operation tests, and first monthly capability report.
