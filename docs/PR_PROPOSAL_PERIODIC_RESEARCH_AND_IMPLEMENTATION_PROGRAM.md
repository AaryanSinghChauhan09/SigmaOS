# PR Proposal: Periodic Research-and-Implementation Program

## Abstract

SigmaOS establishes a **periodic research-and-implementation program**: each cycle studies one narrow OS mechanism from Linux/BSD and reference projects, converts it into an Architecture Decision Record (ADR), implements the smallest connected vertical slice, validates it in QEMU/runtime tests, and records what was adopted, changed, or rejected. The repository already has the study targets and infrastructure—`AGENTS.md`, subsystem agent guides, capability/status files, tests, scripts, installer assets, and comparative gap analysis—and this proposal provides the fixed cadence so research becomes working, verified code rather than an accumulating set of plans.

## Periodic Operating Cadence

The periodic operating cadence uses a four-tier cadence. Every cycle ends with an artifact: a research note, decision record, implementation PR, test result, or updated capability status.

| Cadence | Purpose | Duration | Required output |
|---|---|---|---|
| Daily | Keep the system healthy | 30–60 minutes | Build/test/boot status |
| Weekly | Study and implement one focused mechanism | 3–5 hours | Research note + small PR |
| Monthly | Complete one vertical OS capability | 2–4 weeks | Runtime-validated feature |
| Quarterly | Reassess architecture and roadmap | 1 week | Updated roadmap and gap register |

Proposals become completed work only after their code path and stated checks are verified in runtime environments; this periodic program mechanically enforces that rule.

## Weekly Study Cycle

Run the same five-stage cycle every week.

### Stage 1 (Day 1): Select One Mechanism

Choose one narrowly bounded topic, such as:
- Linux scheduler run queues
- xv6 process switching
- Linux VFS path resolution
- OpenBSD `pledge`/`unveil` confinement
- Arch package dependency resolution
- Fedora atomic updates
- FreeBSD release engineering
- virtio-blk driver initialization

Broad topics such as "Linux", "BSD", or "package management" are strictly prohibited.

### Stage 2 (Day 2): Study Upstream

Read the upstream source, documentation, design discussions, and known limitations.

Record:
- Problem solved
- Core data structures
- Control flow
- Failure and security assumptions
- Performance tradeoffs
- What SigmaOS should adopt, simplify, change, or reject

### Stage 3 (Day 3): Write a Design Note

Create an Architecture Decision Record under `research/decisions/`.

```markdown
# ADR-007: Minimal scheduler model

## Status
Proposed

## Upstream study
xv6 uses a simple context-switch model; Linux uses runnable queues,
scheduler classes, and preemption.

## SigmaOS decision
Implement a fixed-priority round-robin scheduler for the first
bootable milestone.

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

### Stage 4 (Days 4–5): Implement the Smallest Slice

Implement only the slice defined in the ADR. Keep the PR small and scoped to one subsystem boundary.

### Stage 5 (Day 6): Validate

Run the standard validation entry points:

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

Then run the relevant runtime test—usually a QEMU boot or subsystem integration test.

### Stage 6 (Day 7): Publish the Learning

Create a weekly report under `research/reports/weekly/`:

```markdown
# Weekly OS Study Report — Week 12

## Topic
Linux VFS path resolution

## Upstream studied
Linux VFS, xv6 filesystem

## Design adopted
Layered VFS operations with explicit mount points and standardized errors.

## Implemented
Path resolver with `/`, `.`, `..`, absolute/relative paths, and error codes.

## Validation
- 18 unit tests
- 3 integration tests
- QEMU boot test using RAM-disk root

## Rejected for now
Persistent filesystem, permissions, symlinks, and mount namespaces.

## Next cycle
File descriptors and standard I/O.
```

## Monthly Implementation Cycle

Each month closes one user-visible or architecturally important capability.

| Month | Study theme | Implementation outcome | Validation gate |
|---|---|---|---|
| 1 | Boot and kernel entry | QEMU x86_64 boot-to-shell | Boot trace, serial log |
| 2 | Memory and scheduling | Frame allocator, paging, basic scheduler | Process-switch test |
| 3 | Syscalls and VFS | Syscall table, RAM-disk filesystem, file operations | File I/O tests |
| 4 | Processes and IPC | Processes, pipes, signals, FDs | Interop tests |
| 5 | Installer and updates | Clean install, atomic update, rollback | Install/recovery test |
| 6 | Security | Capability checks, sandbox policy, fuzzing | Denial tests |
| 7 | Desktop foundation | Framebuffer, input, window launch | Graphical smoke test |
| 8 | Drivers and hardware | virtio-blk, virtio-net, VGA | Device matrix |
| 9 | Packages | Metadata parse, dependency graph, dry-run | Package tests |
| 10 | Release engineering | Nightly artifact, known issues, rollback docs | Release checklist |

## Quarterly Review Cycle

Every quarter, perform a strategic reset.

### Review Inputs
- Capability matrix (`CAPABILITY_MATRIX.toml`)
- Feature status (`FEATURE_STATUS.toml`)
- Boot matrix (`reports/boot/boot-matrix.json`)
- Failing or flaky tests
- Research notes and ADRs (`research/decisions/`)
- Upstream project changes
- Security advisories
- Hardware-support evidence
- User-facing gaps

### Review Outputs
- Updated roadmap (`docs/roadmap/`)
- Retired or archived stale plans
- New P0/P1/P2 gap list
- Architecture decisions to revisit
- Security review priorities
- Next quarter's monthly themes

## Study Curriculum

### Cycle 1: xv6 and OSDev
- **Study:** Bootloader, GDT/IDT, interrupts, VGA, keyboard, xv6 process model, context switching, syscalls, and filesystem.
- **Implement:** QEMU UEFI/BIOS boot path, serial console, memory-map parsing, frame allocator, paging, timer, keyboard input, scheduler, init, and shell.
- **Validation:** QEMU boots to shell; command executes; shutdown is clean.

### Cycle 2: Linux Kernel
- **Study:** Scheduler classes and run queues, VFS and mount semantics, syscall ABI and error conventions, driver model and device enumeration, namespaces, cgroups, and capability model.
- **Implement:** More capable scheduler, VFS abstraction, process lifecycle and signals, pipes and IPC, initial security boundary model.
- **Validation:** Processes, files, pipes, and signals work across integration tests and QEMU runtime path.

### Cycle 3: Arch Linux and ArchWiki
- **Study:** Package format and dependency resolution, transparent configuration, task-oriented troubleshooting, user-controlled system state.
- **Implement:** Package metadata schema, dependency resolver, dry-run installation, configuration documentation and validation.
- **Validation:** Package manifest parsed, dependencies resolved, installation simulated safely.

### Cycle 4: Linux Mint
- **Study:** Onboarding, update guidance, troubleshooting, recovery and rollback.
- **Implement:** Installer workflow, update manifest and rollback, recovery boot entry, troubleshooting docs generated from real failures.
- **Validation:** Clean QEMU install succeeds and failed update restores prior system state.

### Cycle 5: Fedora
- **Study:** CI gating, atomic updates, developer experience, release quality process.
- **Implement:** CI merge gates, nightly artifact pipeline, staged update activation, release checklist.
- **Validation:** Nightly artifact produced only after build, tests, QEMU boot, install, and rollback tests pass.

### Cycle 6: FreeBSD and OpenBSD
- **Study:** FreeBSD release engineering and jails, OpenBSD secure defaults, `pledge`, `unveil`, code audit culture, networking and storage design, hardware support documentation.
- **Implement:** Capability-based sandboxing, package-script isolation, signed update manifests, hardware support matrix, security review workflow.
- **Validation:** Protected operations fail without required capabilities; package scripts cannot exceed declared sandbox permissions.

### Cycle 7: Redox OS and Rust OS Design
- **Study:** Rust ownership in OS components, userspace/kernel separation, capability-oriented design, safe abstractions around unsafe kernel operations.
- **Implement:** Rust module boundaries, unsafe-code policy, kernel/userspace interface contracts, memory-safe wrappers around low-level operations.
- **Validation:** Unsafe code isolated, documented, and covered by tests or formal review.

## Repository Structure

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

## Task Templates

### Weekly Study Task (`.agents/tasks/open/STUDY-2026-W42.toml`)

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

### Monthly Capability Task (`.agents/tasks/open/CAP-2026-M10.toml`)

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

## Industry-Standard Rules

1. **Rule 1: One mechanism per cycle** — Each weekly cycle studies and implements one narrow mechanism. Broad studies produce broad, unverified plans.
2. **Rule 2: Research must produce a decision** — Every study cycle ends with an ADR stating what SigmaOS will adopt, simplify, change, or reject.
3. **Rule 3: Implementation must be a vertical slice** — Prefer a complete small path—such as boot → memory → scheduler → shell—over disconnected subsystem improvements.
4. **Rule 4: Runtime evidence controls status** — Unit tests prove logic; QEMU boot, installer, update, and hardware tests prove the operating system.
5. **Rule 5: CI gates every change** — Required CI includes clean build, `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, formatting, and relevant QEMU smoke tests.
6. **Rule 6: Reproducibility is mandatory** — Record commit hash, toolchain, container image, commands, timestamps, exit codes, and hashes.
7. **Rule 7: License compliance first** — Verify license compatibility and attribution for dual MIT/GPL-2.0 or use clean-room design notes and original implementation.
8. **Rule 8: Security review is separate** — Capability enforcement, sandboxing, syscalls, memory safety, cryptography, package execution, networking, and updates require a dedicated security review.
9. **Rule 9: Least privilege** — Agents, CI jobs, packages, and services receive only the permissions required. No broad credentials or unrestricted script execution.
10. **Rule 10: Fail safely** — Boot, installer, update, storage, and recovery changes must define rollback behavior before merge.
11. **Rule 11: Regression tests are mandatory** — Every implemented mechanism needs a test that would fail if behavior regressed.
12. **Rule 12: Human approval for high-risk work** — Humans approve bootloader, installer, update, security-boundary, package-execution, release, and destructive-storage changes.
13. **Rule 13: Archive stale research** — Research older than two quarters must be reaffirmed, updated, or moved to an archive.
14. **Rule 14: No borrowed-status claims** — A feature existing in Linux, FreeBSD, OpenBSD, Arch, or Redox does not mean it exists in SigmaOS without runtime evidence.

## Metrics

Track these after every cycle:
- Number of mechanisms studied
- Number of ADRs created
- Number of mechanisms implemented
- Number of mechanisms runtime-validated
- QEMU boot success rate
- Test pass rate and flaky-test count
- Number of capability claims upgraded or downgraded
- Number of security reviews completed
- Number of stale research documents archived
- Time from study to merged, validated implementation

## First 30 Days

- **Week 1:** Create `research/`, ADR template, weekly report template, study task template. Study xv6 boot and OSDev bootloader patterns. Implement QEMU x86_64 UEFI boot with serial logging. Produce first boot trace.
- **Week 2:** Study xv6 memory management and Linux paging. Implement frame allocation, paging, and early memory initialization. Add memory tests and boot validation.
- **Week 3:** Study xv6 processes and Linux scheduling. Implement minimal scheduler and context switching. Add two-process alternation test.
- **Week 4:** Study Linux syscalls and xv6 filesystem. Implement minimal syscall table and RAM-disk VFS. Add boot-to-shell and file-operation tests. Publish first monthly capability report.

## Pull Request Staging Manifest

```toml
[pull_request]
pr_id = 4221
title = "governance: enforce periodic research and implementation program"
branch = "feat/governance-periodic-research-program"
status = "Ready for Integration"
target_files = [
  "PR_PROPOSAL_PERIODIC_RESEARCH_AND_IMPLEMENTATION_PROGRAM.md",
  "docs/PR_PROPOSAL_PERIODIC_RESEARCH_AND_IMPLEMENTATION_PROGRAM.md",
  "docs/roadmap/PR_SPEC_PERIODIC_RESEARCH_AND_IMPLEMENTATION_PROGRAM.md",
  ".agents/runbooks/weekly-study.md",
  ".agents/runbooks/monthly-capability.md",
  "src/governance/sovereign_periodic_research_program_engine.rs",
  "tests/test_sovereign_periodic_research_program.rs"
]
```
