# PR Proposal: SigmaOS Periodic Research-and-Implementation Program

## Summary
This PR proposal establishes a formal, evidence-gated **Periodic Research-and-Implementation Program** for SigmaOS. Each cycle studies one narrow OS mechanism from reference projects (Linux, BSD, xv6, Arch, Mint, Fedora, Redox), converts it into an Architecture Decision Record (ADR), implements the smallest connected slice, validates it in QEMU/runtime tests, and records what was adopted, changed, or rejected.

## Motivation & Objectives
SigmaOS possesses an extensive research base, subsystem agent guides, capability/status matrices, test suites, and scripts. However, without a fixed periodic cadence, architectural research risks accumulating as planning documentation without converting into working, verified code. This program mechanically enforces the core rule in SigmaOS's README: **Proposals become completed work only after their code path and stated checks are verified in runtime testing.**

---

## 1. Periodic Operating Cadence

SigmaOS operates under a four-tier cadence where every cycle produces a concrete, auditable artifact.

| Cadence | Purpose | Duration | Required Output |
| :--- | :--- | :--- | :--- |
| **Daily** | Keep the system healthy | 30–60 minutes | Build, unit test, and QEMU boot status report |
| **Weekly** | Study & implement one focused mechanism | 3–5 hours | Research note + ADR + scoped implementation PR |
| **Monthly** | Complete one vertical OS capability | 2–4 weeks | Runtime-validated user-visible OS capability |
| **Quarterly** | Strategic architecture & roadmap reset | 1 week | Consolidated roadmap, gap register, and retired plans |

---

## 2. Weekly Study Cycle Workflow

Every weekly cycle follows a strict five-stage progression across seven days:

### Day 1: Select One Mechanism
Select a single, narrowly bounded OS mechanism (e.g., Linux run queues, xv6 context switching, OpenBSD `pledge`/`unveil`, Arch dependency resolution). Broad topics such as "Linux kernel" or "Package management" are explicitly rejected.

### Day 2: Study Upstream
Analyze upstream source code, design discussions, and operational trade-offs. Record:
- Problem solved
- Core data structures
- Control flow & entry points
- Failure mode and security assumptions
- Performance trade-offs
- Adoption decision: adopt, simplify, change, or reject.

### Day 3: Write Architecture Decision Record (ADR)
Create an ADR file under `research/decisions/ADR-XXX-<topic>.md`:

```markdown
# ADR-007: Minimal Scheduler Model

## Status
Proposed

## Upstream Study
xv6 uses a simple context-switch model; Linux uses runnable queues, scheduler classes, and preemption.

## SigmaOS Decision
Implement a fixed-priority round-robin scheduler for the first bootable milestone.

## Scope
- Process states
- Run queue
- Context switch
- Timer preemption
- Basic fairness test

## Non-goals
- NUMA scheduling
- Real-time classes
- CPU affinity
- Energy-aware scheduling

## Validation
- Unit tests for queue ordering
- Integration test for two alternating processes
- QEMU boot trace showing scheduler initialization
```

### Day 4–5: Implement the Smallest Slice
Implement only the minimal slice defined in the ADR. Keep PRs small and bounded by subsystem interfaces.

### Day 6: Validate Runtime Evidence
Execute the mandatory SigmaOS validation suite:
```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```
Followed by the subsystem runtime test (QEMU boot or integration test).

### Day 7: Publish Weekly OS Study Report
Generate a weekly report under `research/reports/weekly/`:

```markdown
# Weekly OS Study Report — Week 12

## Topic
Linux VFS path resolution

## Upstream Studied
Linux VFS, xv6 filesystem

## Design Adopted
Layered VFS operations with explicit mount points and standardized errors.

## Implemented
Path resolver with `/`, `.`, `..`, absolute/relative paths, and error codes.

## Validation
- 18 unit tests
- 3 integration tests
- QEMU boot test using RAM-disk root

## Rejected for now
Persistent filesystem, permissions, symlinks, and mount namespaces.

## Next Cycle
File descriptors and standard I/O.
```

---

## 3. Monthly Implementation Cycle Roadmap

Each month delivers a bootable, user-visible, or architecturally vital capability:

| Month | Study Theme | Implementation Outcome | Validation Gate |
| :--- | :--- | :--- | :--- |
| **Month 1** | Boot & Kernel Entry | QEMU x86_64 boot-to-shell | Boot trace, serial log |
| **Month 2** | Memory & Scheduling | Frame allocator, paging, basic scheduler | Process-switch test |
| **Month 3** | Syscalls & VFS | Syscall table, RAM-disk filesystem, file operations | File I/O tests |
| **Month 4** | Processes & IPC | Processes, pipes, signals, file descriptors | Interop tests |
| **Month 5** | Installer & Updates | Clean install, atomic update, rollback | Install/recovery test |
| **Month 6** | Security | Capability checks, sandbox policy, fuzzing | Denial tests |
| **Month 7** | Desktop Foundation | Framebuffer, input, window launch | Graphical smoke test |
| **Month 8** | Drivers & Hardware | virtio-blk, virtio-net, VGA | Device matrix |
| **Month 9** | Packages | Metadata parse, dependency graph, dry-run | Package tests |
| **Month 10** | Release Engineering | Nightly artifact, known issues, rollback docs | Release checklist |

---

## 4. Quarterly Review Process

At the start of each quarter, conduct a strategic architecture review:
- **Inputs**: Capability matrix (`CAPABILITY_MATRIX.toml`), feature status (`FEATURE_STATUS.toml`), test pass logs, hardware support matrix, security advisories.
- **Outputs**: Consolidated roadmap, updated P0/P1/P2 gap register, archive of stale plans older than 2 quarters, refreshed monthly themes.

---

## 5. Seven-Cycle Study Curriculum

1. **Cycle 1: xv6 and OSDev** (Bootloader, GDT/IDT, interrupts, serial console, frame allocator, paging, keyboard, minimal scheduler, shell).
2. **Cycle 2: Linux Kernel** (Scheduler run queues, VFS, syscall ABI, driver model, namespaces, capability bounds).
3. **Cycle 3: Arch Linux and ArchWiki** (Package metadata schema, dependency resolution engine, dry-run installation, declarative state).
4. **Cycle 4: Linux Mint** (Installer workflow, update manifests, Timeshift snapshot rollback, recovery boot entry).
5. **Cycle 5: Fedora** (CI gating, nightly artifact pipeline, staged update activation, release verification).
6. **Cycle 6: FreeBSD and OpenBSD** (Capability-based sandboxing, pledge/unveil confinement, signed update manifests, hardware matrix).
7. **Cycle 7: Redox OS and Rust OS Design** (Ownership in OS components, kernel/userspace separation, memory-safe wrappers around low-level hardware operations).

---

## 6. Repository Layout & Standards

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

## 7. Machine-Readable Task Schemas

### Weekly Study Task Schema (`.agents/tasks/open/STUDY-2026-W42.toml`)
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

### Monthly Capability Task Schema (`.agents/tasks/open/CAP-2026-M10.toml`)
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

## 8. Fourteen Industry-Standard Program Rules

1. **One mechanism per cycle**: Weekly studies must be tightly bounded.
2. **Research must produce a decision**: Every study cycle ends with an ADR.
3. **Vertical slice implementation**: Implement connected end-to-end paths over disconnected stubs.
4. **Runtime evidence controls status**: Unit tests prove logic; QEMU boot and hardware tests prove capability.
5. **CI gates every change**: Clean build, test suite, and QEMU smoke tests gate all merges.
6. **Reproducibility is mandatory**: Log toolchains, container digests, commit hashes, and build artifacts.
7. **License compliance first**: Verify MIT/GPL-2.0 compatibility and attribution before adopting code.
8. **Security review is separate**: Dedicated security audit for capability, syscall, and update mechanisms.
9. **Least privilege**: Agents and services receive minimal required permissions.
10. **Fail safely**: Define rollback behavior prior to merging boot, installer, or storage changes.
11. **Regression tests are mandatory**: Every new feature must include a failing negative/regression test.
12. **Human approval for high-risk work**: Security, bootloader, installer, and update PRs require human signoff.
13. **Archive stale research**: Move research older than two quarters without implementation to archive.
14. **No borrowed-status claims**: Upstream feature presence does not establish SigmaOS parity without runtime proof.

---

## 9. First 30 Days Kickoff Plan

- **Week 1**: Initialize `research/` structure, ADR templates, study xv6 bootloader, implement QEMU x86_64 UEFI boot with serial log output.
- **Week 2**: Study xv6 memory management and Linux paging, implement frame allocation & paging, run memory tests.
- **Week 3**: Study xv6 process model and Linux scheduling, implement minimal round-robin scheduler & context switching, validate process alternation.
- **Week 4**: Study Linux syscalls and xv6 VFS, implement RAM-disk VFS and syscall dispatch, run boot-to-shell integration tests, publish first Monthly Capability Report.
