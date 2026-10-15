# Pull Request Proposal: SigmaOS Sovereign Periodic Research-and-Implementation Program

## Pull Request Summary
- **Title**: `feat(governance): Establish Sovereign Periodic Research-and-Implementation Program`
- **Branch**: `feature/periodic-research-program`
- **Status**: `Proposed / Specification Ready`
- **Scope**: Periodic research operating cadence, weekly study cycles, monthly capability roadmaps, quarterly strategic reviews, study curricula, research directory architecture, task schemas, 14 governance rules, and automated verification.

---

## 1. Executive Overview & Core Operating Principle
SigmaOS establishes a **Periodic Research-and-Implementation Program** to convert OS research into working, verified code rather than accumulating planning documents. Each cycle studies one narrow OS mechanism from reference projects (xv6, Linux, Arch, Mint, Fedora, FreeBSD, OpenBSD, Redox), converts it into an Architecture Decision Record (ADR), implements the smallest connected slice, validates it in QEMU/runtime tests, and updates capability status only when backed by runtime evidence.

---

## 2. Four-Tier Periodic Operating Cadence

Every operating cycle produces a mandatory, verifiable artifact:

| Cadence | Purpose | Duration | Required Output Artifact |
|:---|:---|:---|:---|
| **Daily** | Keep build, tests, and QEMU boot status healthy | 30–60 mins | Build/test/boot status report (`reports/steward/daily-status.md`) |
| **Weekly** | Study and implement one focused mechanism | 3–5 hours | Research note + ADR + small PR + test evidence |
| **Monthly** | Complete one vertical OS capability | 2–4 weeks | Runtime-validated feature (`reports/boot/qemu-x86_64.log`) |
| **Quarterly** | Reassess architecture, gaps, and roadmap | 1 week | Updated roadmap + retired stale plans + gap register |

---

## 3. Five-Stage Weekly Study Cycle

Every weekly cycle follows a strict 7-day, 5-stage loop:

1. **Day 1: Select One Mechanism**: Choose a narrowly bounded topic (e.g., Linux scheduler run queues, xv6 process switching, Linux VFS path resolution, OpenBSD `pledge`/`unveil`, Arch package dependency resolution). Broad topics like "Linux" or "package management" are explicitly forbidden.
2. **Day 2: Study Upstream**: Read upstream source code, documentation, design notes, and failure modes. Record problem solved, core data structures, control flow, security assumptions, and tradeoffs in `research/upstream/<project>/<topic>.md`.
3. **Day 3: Write a Design Note (ADR)**: Create an Architecture Decision Record under `research/decisions/ADR-XXX-<topic>.md` specifying status, upstream study, SigmaOS decision, scope, non-goals, and validation gates.
4. **Days 4–5: Implement the Smallest Slice**: Implement only the narrow slice defined in the ADR scoped to one subsystem boundary.
5. **Day 6–7: Validate & Publish**: Run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, `make check`, `make test`, `make format`, and QEMU runtime tests. Publish weekly report in `research/reports/weekly/`.

---

## 4. Ten-Month Capability Implementation Roadmap

| Month | Study Theme | Implementation Outcome | Validation Gate |
|:---|:---|:---|:---|
| **Month 1** | Boot and kernel entry | QEMU x86_64 boot-to-shell | Boot trace, serial log |
| **Month 2** | Memory and scheduling | Frame allocator, paging, basic scheduler | Process-switch test |
| **Month 3** | Syscalls and VFS | Syscall table, RAM-disk filesystem, file operations | File I/O tests |
| **Month 4** | Processes and IPC | Processes, pipes, signals, file descriptors | Interop tests |
| **Month 5** | Installer and updates | Clean install, atomic update, rollback | Install/recovery test |
| **Month 6** | Security | Capability checks, sandbox policy, fuzzing | Denial tests |
| **Month 7** | Desktop foundation | Framebuffer, input, window launch | Graphical smoke test |
| **Month 8** | Drivers and hardware | virtio-blk, virtio-net, VGA | Device matrix |
| **Month 9** | Packages | Metadata parse, dependency graph, dry-run | Package tests |
| **Month 10** | Release engineering | Nightly artifact, known issues, rollback docs | Release checklist |

---

## 5. Quarterly Review Cycle & Inputs/Outputs

### Review Inputs
1. Capability matrix (`CAPABILITY_MATRIX.toml`)
2. Feature status (`FEATURE_STATUS.toml`)
3. Boot matrix (`reports/boot/qemu-x86_64.json`)
4. Failing or flaky test logs
5. Research notes and ADRs
6. Upstream project developments
7. Security advisories
8. Hardware-support evidence
9. User-facing gap list

### Review Outputs
1. Updated roadmap
2. Retired or archived stale plans (>2 quarters old)
3. New P0/P1/P2 gap register
4. Revisited architecture decisions
5. Security review priorities
6. Next quarter's monthly themes

---

## 6. Seven-Cycle Study Curriculum

- **Cycle 1: xv6 & OSDev**: Bootloader, GDT/IDT, interrupts, serial, frame allocator, scheduler, shell.
- **Cycle 2: Linux Kernel**: Scheduler classes, VFS mount semantics, syscall ABI, driver model, namespaces.
- **Cycle 3: Arch Linux & ArchWiki**: Package metadata, dependency resolution, dry-run install, transparent configuration.
- **Cycle 4: Linux Mint**: Onboarding, update guidance, Timeshift rollback, recovery entries.
- **Cycle 5: Fedora**: CI gating, atomic updates, nightly artifact pipeline, staged updates.
- **Cycle 6: FreeBSD & OpenBSD**: Capability sandboxing, `pledge`/`unveil`, package isolation, release engineering.
- **Cycle 7: Redox OS & Rust OS Design**: Rust module boundaries, unsafe-code policy, safe wrappers around low-level operations.

---

## 7. Stable Repository Layout Contract

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
      weekly-study-template.toml
      monthly-capability-template.toml
    in-progress/
    done/
    blocked/
```

---

## 8. Fourteen Industry-Standard Operating Rules

1. **Rule 1: One mechanism per cycle** - Narrow topic selection required.
2. **Rule 2: Research must produce a decision** - Every cycle ends with an ADR.
3. **Rule 3: Implementation must be a vertical slice** - Small complete connected paths.
4. **Rule 4: Runtime evidence controls status** - QEMU/hardware evidence required for promotion.
5. **Rule 5: CI gates every change** - All checks and tests required.
6. **Rule 6: Reproducibility is mandatory** - Record commit hash, toolchain, commands, exit codes.
7. **Rule 7: License compliance first** - Dual MIT/GPL-2.0 verification or clean-room implementation.
8. **Rule 8: Security review is separate** - Dedicated review for security boundaries.
9. **Rule 9: Least privilege** - Sandboxed task execution.
10. **Rule 10: Fail safely** - Define rollback behavior before merge.
11. **Rule 11: Regression tests are mandatory** - Tests required for every slice.
12. **Rule 12: Human approval for high-risk work** - Bootloader, installer, update, security changes require human approval.
13. **Rule 13: Archive stale research** - Research older than 2 quarters must be archived or reaffirmed.
14. **Rule 14: No borrowed-status claims** - Reference project features do not establish SigmaOS parity without runtime proof.

---

## 9. Verification & Enforcement

The program is enforced by `src/governance/sovereign_periodic_research_program_engine.rs` and verified via `tests/test_sovereign_periodic_research_program.rs` in `./run_sigma_tests.sh`.
