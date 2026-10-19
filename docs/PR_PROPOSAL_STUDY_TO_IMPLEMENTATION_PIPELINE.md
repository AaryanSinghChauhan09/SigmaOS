# Pull Request Proposal: Study-to-Implementation Pipeline Engine

## Executive Summary

This Pull Request proposal establishes SigmaOS's formal **Study → Extract Pattern → Implement Minimal Slice → Validate at Runtime** operating pipeline. While SigmaOS incorporates architectural design insight from 11 upstream reference operating systems (xv6, Linux Kernel, Redox OS, Arch Linux, Linux Mint, FreeBSD, OpenBSD, Debian, Fedora, NixOS, and OSDev), source code presence and unit test passing do not establish bootability, installation readiness, or hardware support.

This proposal introduces a 10-step repeatable cycle, a 12-week study curriculum, 12 mandatory implementation rules, a 10-point Definition of Done, a 12-week schedule, a first 30 days plan, and a structured `research/` directory containing upstream study records, Architecture Decision Records (ADRs), and comparative mechanism matrices.

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
| **Linux Mint** | Onboarding, update safety, recovery, troubleshooting | Installer & update/rollback UX | `research/upstream/mint/recovery.md` & ADR-004 |
| **FreeBSD** | Release engineering, jails, ZFS, networking | Release process, isolation, storage | `research/upstream/freebsd/` (release-engineering, jails) |
| **OpenBSD** | Simplicity, secure defaults, `pledge`/`unveil`, auditing | Least-privilege security model | `research/upstream/openbsd/` (secure-defaults, pledge-unveil) & ADR-005 |
| **Debian** | Stable release discipline, packaging, dependency management | Package repository & update channels | `research/upstream/debian/release-policy.md` |
| **Fedora** | CI, gating, atomic updates, developer experience | CI gates & update safety | `research/upstream/fedora/ci-gating.md` |
| **NixOS** | Declarative configuration and reproducibility | Declarative system configuration | `research/upstream/nixos/declarative-config.md` |
| **OSDev** | Bootloader, GDT/IDT, interrupts, VGA, basic drivers | First bootable kernel milestones | Boot trace & ADR-001 |

Each upstream study note answers:
1. What problem does the upstream design solve?
2. What is its core data structure and control flow?
3. What are its security and failure assumptions?
4. What can SigmaOS adopt directly?
5. What must be simplified or redesigned for Rust/SigmaOS?
6. What test proves the SigmaOS version works?
7. What are the licensing and attribution implications?

---

## 5-Phase Curriculum Schedule

### Phase 1: OS Fundamentals (Weeks 1-2)
- **Study Focus:** Boot process, firmware, CPU modes, paging, interrupts, scheduling, basic shell.
- **Primary References:** xv6, OSDev tutorials, Linux kernel documentation.
- **SigmaOS Implementation:** QEMU x86_64 UEFI boot, COM1 logging, FrameAllocator, PML4 paging, scheduler.
- **Exit Test:** QEMU x86_64 boots to shell, executes basic command, and shuts down cleanly.

### Phase 2: Kernel Services (Weeks 3-4)
- **Study Focus:** Fair scheduler, VFS path resolution, process creation, signals, pipes, syscall table.
- **Primary References:** Linux Kernel, FreeBSD, OpenBSD.
- **SigmaOS Implementation:** EEVDF scheduler, trait VFS, process fork/exec, pipes, signals.
- **Exit Test:** Process can create, open/read/write files, use pipes, receive signal, and exit.

### Phase 3: Userspace & Distribution (Weeks 5-6)
- **Study Focus:** `SigmaPkg` dependency resolver, dry-run package install, staged update & rollback.
- **Primary References:** Arch Linux, Debian, Fedora, Linux Mint, NixOS.
- **SigmaOS Implementation:** `SigmaPkg` metadata parser, SAT dependency solver, staged update, rollback, `system.toml`.
- **Exit Test:** Clean QEMU install boots, and deliberately failed update rolls back successfully.

### Phase 4: Security & Isolation (Weeks 7-8)
- **Study Focus:** `pledge`/`unveil`, Landlock capabilities, sandboxed script execution, signed update channels.
- **Primary References:** OpenBSD, FreeBSD, Linux Security.
- **SigmaOS Implementation:** `pledge` promise masks, `unveil` path trees, Landlock compatibility, signed update manifests.
- **Exit Test:** Protected operations fail without capabilities and scripts cannot escape sandbox.

### Phase 5: Desktop & Hardware (Weeks 9-12)
- **Study Focus:** VirtIO-blk, VirtIO-net, framebuffer, input, minimal compositor, hardware matrices, declarative TOML schema.
- **Primary References:** Linux DRM, VirtIO specs, Linux Mint.
- **SigmaOS Implementation:** VirtIO-blk, VirtIO-net, framebuffer graphics, Zenith Compositor, hardware matrix.
- **Exit Test:** QEMU boots to graphical session, accepts input, launches app without panic.

---

## 12-Week Suggested Schedule

| Week | Study Focus | Implementation Focus | Validation |
| :--- | :--- | :--- | :--- |
| **1** | xv6 boot, OSDev bootloader, interrupts | QEMU UEFI boot, serial logging | Boot trace |
| **2** | xv6 paging/processes, Linux memory basics | Frame allocator, paging, scheduler | Boot-to-shell |
| **3** | Linux syscalls/VFS, xv6 filesystem | Syscall table, VFS, RAM disk | File operations |
| **4** | Linux processes/signals, IPC | Processes, pipes, signals, FDs | Integration tests |
| **5** | Arch/Debian packaging | Package metadata parser, dependency resolver | Dry-run tests |
| **6** | Fedora atomic updates, Mint recovery | Update manifest, rollback, recovery | Failed-update test |
| **7** | OpenBSD secure defaults, Linux capabilities | Capability model, sandbox policy | Denial tests |
| **8** | FreeBSD jails, supply-chain security | Sandboxed package execution, signed updates | Security tests |
| **9** | Linux input/display, Redox architecture | Framebuffer, input, minimal compositor | Graphical boot |
| **10** | Driver models, virtio | VirtIO-blk, VirtIO-net, VGA | Device matrix |
| **11** | FreeBSD/OpenBSD release engineering | Release channels, known issues, rollback docs | Nightly artifact |
| **12** | NixOS declarative configuration | Configuration schema and validation | Config test suite |

---

## First 30 Days Plan

1. Create `research/upstream/`, `research/decisions/`, and `research/comparisons/`.
2. Study xv6 boot and OSDev bootloader patterns; write `ADR-001-boot-path.md`.
3. Implement QEMU x86_64 UEFI boot with serial logging.
4. Study xv6 memory and Linux paging; write `ADR-002-memory-model.md`.
5. Implement frame allocation, paging, interrupts, and timer.
6. Study xv6 processes and Linux scheduler concepts; implement a minimal scheduler.
7. Add boot-to-shell smoke test.
8. Study Linux VFS and xv6 filesystem; write `ADR-003-vfs-interface.md`.
9. Implement a RAM-disk VFS and basic file operations.
10. Publish a weekly “studied → implemented → validated” report.

---

## 12 Mandatory Implementation Rules

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
    mint/ (recovery.md)

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
