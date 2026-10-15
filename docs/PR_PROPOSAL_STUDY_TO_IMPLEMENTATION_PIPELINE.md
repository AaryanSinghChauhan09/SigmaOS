# Pull Request Proposal: Study-to-Implementation Pipeline Engine

## Executive Summary

This Pull Request proposal establishes SigmaOS's formal **Study → Extract Pattern → Implement Minimal Slice → Validate at Runtime** operating pipeline. While SigmaOS incorporates architectural design insight from 11 upstream reference operating systems (xv6, Linux Kernel, Redox OS, Arch Linux, Linux Mint, FreeBSD, OpenBSD, Debian, Fedora, NixOS, and OSDev), source code presence and unit test passing do not establish bootability, installation readiness, or hardware support.

This proposal introduces a 10-step repeatable cycle, a 12-week study curriculum, 12 mandatory implementation rules, a 10-point Definition of Done, and a structured `research/` directory containing upstream study records, Architecture Decision Records (ADRs), and comparative mechanism matrices.

---

## 10-Step Repeatable Cycle

For every mechanism borrowed from an upstream reference OS, SigmaOS executes one repeatable cycle:

```text
1. SELECT a narrow OS concept
2. STUDY upstream implementation and documentation
3. WRITE a design note for SigmaOS (in research/upstream/)
4. DEFINE the smallest testable slice
5. IMPLEMENT behind a clear module boundary
6. ADD unit, integration, and runtime tests
7. VALIDATE in QEMU or on named hardware
8. DOCUMENT what was adopted, changed, and rejected
9. ADD regression protection
10. UPDATE capability status with evidence
```

---

## Reference OS Projects Study Focus & Deliverables

| Project | Study Focus | SigmaOS Application | Study Deliverable |
| :--- | :--- | :--- | :--- |
| **xv6** | Minimal kernel clarity: boot, paging, processes, syscalls, filesystem | Understandable reference kernel path | `research/upstream/xv6/` (boot, memory, filesystem notes) |
| **Linux Kernel** | VFS, scheduler, driver model, syscalls, namespaces | Scalable kernel abstractions | `research/upstream/linux/` (scheduler, vfs, drivers notes) |
| **Redox OS** | Rust OS structure, capability-oriented design, userspace separation | Rust-native architecture & unsafe-code policy | `research/upstream/` notes & ADRs |
| **Arch Linux** | Transparent configuration, package model, user control | `SigmaPkg` & system configuration | `research/upstream/arch/packaging.md` |
| **Linux Mint** | Onboarding, update safety, recovery, troubleshooting | Installer & update/rollback UX | `research/upstream/` & ADR-004 |
| **FreeBSD** | Release engineering, jails, ZFS, networking | Release process, isolation, storage | `research/upstream/freebsd/` (release-engineering, jails) |
| **OpenBSD** | Simplicity, secure defaults, `pledge`/`unveil`, auditing | Least-privilege security model | `research/upstream/openbsd/` & ADR-005 |
| **Debian** | Stable release discipline, packaging, dependency management | Package repository & update channels | `research/upstream/debian/release-policy.md` |
| **Fedora** | CI, gating, atomic updates, developer experience | CI gates & update safety | `research/upstream/fedora/ci-gating.md` |
| **NixOS** | Declarative configuration and reproducibility | Declarative system configuration | `research/upstream/nixos/declarative-config.md` |
| **OSDev** | Bootloader, GDT/IDT, interrupts, VGA, basic drivers | First bootable kernel milestones | Boot trace & ADR-001 |

---

## 5-Phase Curriculum Schedule

### Phase 1: OS Fundamentals (Weeks 1-2)
- **Study Focus:** Boot process, firmware, CPU modes, paging, interrupts, scheduling, basic shell.
- **Deliverable:** Boot-to-shell trace in QEMU x86_64 (`ADR-001-boot-path.md`).

### Phase 2: Kernel Services (Weeks 3-4)
- **Study Focus:** Fair scheduler, VFS path resolution, processes, signals, pipes, syscall table.
- **Deliverable:** Process creation, IPC, and VFS integration tests (`ADR-002`, `ADR-003`).

### Phase 3: Userspace & Distribution (Weeks 5-6)
- **Study Focus:** `SigmaPkg` dependency resolver, dry-run package install, staged update & rollback.
- **Deliverable:** Failed update rollback test (`ADR-004`).

### Phase 4: Security & Isolation (Weeks 7-8)
- **Study Focus:** `pledge`/`unveil`, Landlock capabilities, sandboxed script execution, signed update channels.
- **Deliverable:** Security denial test suite (`ADR-005`).

### Phase 5: Desktop & Hardware (Weeks 9-12)
- **Study Focus:** VirtIO-blk, VirtIO-net, framebuffer, input, minimal compositor, hardware matrices, declarative TOML schema.
- **Deliverable:** Graphical boot, input handling, and declarative configuration suite.

---

## 12 Implementation Rules

1. **Study one concept at a time:** Focus on single mechanisms rather than monolithic OSes.
2. **Extract the invariant, not the code:** Record underlying behavioral rules rather than translating code line-by-line.
3. **License and attribution first:** Ensure MIT / GPL-2.0 compatibility before adapting design concepts.
4. **Implement the smallest vertical slice:** Connected path from boot → memory → scheduler → syscall → VFS → shell.
5. **Runtime validation is mandatory:** Unit tests are necessary but insufficient; exercise through real QEMU runtime boot paths.
6. **CI gates merges:** Enforce `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, formatting, and QEMU smoke tests.
7. **Security changes get separate review:** Dedicated review for sandboxing, syscalls, capabilities, and crypto.
8. **Reproducibility:** Record toolchain, timestamps, exit codes, git commit, and hashes for experiments.
9. **Least privilege:** Restrict permissions across agents, package scripts, and CI runners.
10. **Document adoption boundaries:** Explicitly state what was adopted, simplified, changed, or rejected.
11. **Regression tests are mandatory:** Every implemented mechanism includes a failing test safeguard.
12. **Human approval for high-risk work:** Destructive storage, update engines, and security boundaries require human sign-off.

---

## Repository Artifact Structure

```text
research/
  upstream/
    xv6/ (boot.md, memory.md, filesystem.md)
    linux/ (scheduler.md, vfs.md, drivers.md)
    freebsd/ (release-engineering.md, jails.md)
    openbsd/ (secure-defaults.md, pledge-unveil.md)
    arch/ (packaging.md)
    debian/ (release-policy.md)
    fedora/ (ci-gating.md)
    nixos/ (declarative-config.md)

  decisions/
    ADR-001-boot-path.md
    ADR-002-scheduler-model.md
    ADR-003-vfs-interface.md
    ADR-004-package-format.md
    ADR-005-security-capabilities.md

  comparisons/
    scheduler.md
    filesystem.md
    security.md
    packaging.md
    hardware-support.md
```

---

## Definition of Done (10-Point Checklist)

1. Upstream study note recorded in `research/upstream/` answering all 7 questions.
2. Architecture Decision Record (ADR) accepted in `research/decisions/`.
3. Clear module boundary and owner in Rust codebase.
4. Unit and integration tests pass cleanly.
5. Exercised through QEMU x86_64 boot or real runtime execution path.
6. Regression test exists and guards against behavioral regressions.
7. Security implications reviewed.
8. Documentation state limitations and unsupported cases.
9. Capability status updated in `CAPABILITY_MATRIX.toml`.
10. Experiment is reproducible from a clean checkout.

---

## Verification & Status

Implementation verified via `tests/test_study_to_implementation_pipeline.rs` and `./run_sigma_tests.sh`.

Status: **Ready for Merge**.
