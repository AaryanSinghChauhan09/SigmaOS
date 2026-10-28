# PR Proposal: Study-to-Implementation Pipeline Strategy

**PR Title:** `feat(governance): Establish Study-to-Implementation Pipeline Strategy`
**PR Branch:** `feature/study-to-implementation-pipeline`
**Status:** Approved / Specification Ready
**Patch Digest:** `0xa3f891bc471d8032`

---

## Executive Summary

SigmaOS operates under a strict **study → extract pattern → implement minimal slice → validate at runtime** pipeline. While SigmaOS references ideas from Linux, Arch, Mint, Redox, xv6, FreeBSD, OpenBSD, Debian, Fedora, NixOS, and OSDev, its documentation explicitly maintains that source presence or passing unit tests alone do not establish bootability, installation readiness, or hardware support.

This specification formalizes the repeatable 10-step study-to-implementation workflow, study curriculum, upstream study requirements, Architecture Decision Record (ADR) templates, 12 mandatory implementation rules, 12-week schedule, definition of done, and first 30 days execution plan.

---

## Study-to-Implementation Pipeline

SigmaOS uses one repeatable cycle for every concept borrowed from an upstream OS project:

```text
1. SELECT a narrow OS concept
2. STUDY upstream implementation and documentation
3. WRITE a design note for SigmaOS
4. DEFINE the smallest testable slice
5. IMPLEMENT behind a clear module boundary
6. ADD unit, integration, and runtime tests
7. VALIDATE in QEMU or on named hardware
8. DOCUMENT what was adopted, changed, and rejected
9. ADD regression protection
10. UPDATE capability status with evidence
```

A reference project is a source of **design insight**, not a feature checklist. Reference-project feature names do not establish SigmaOS feature parity.

---

## Upstream Projects to Study

Study upstream projects in a fixed order, producing one study deliverable per project:

| Project | Study focus | SigmaOS application | Study deliverable |
|---|---|---|---|
| **xv6** | Minimal kernel clarity: boot, paging, processes, syscalls, filesystem | Build an understandable reference kernel path | Annotated boot-to-shell map |
| **Linux Kernel** | VFS, scheduler, driver model, syscalls, namespaces | Scalable kernel abstractions | Interface comparison matrix |
| **Redox OS** | Rust OS structure, capability-oriented design, userspace separation | Rust-native architecture and unsafe-code policy | Rust architecture note |
| **Arch Linux** | Transparent configuration, package model, user control | SigmaPkg and system configuration | Package/config design note |
| **Linux Mint** | Onboarding, update safety, recovery, troubleshooting | Installer and update/rollback UX | Recovery workflow |
| **FreeBSD** | Release engineering, jails, ZFS, networking | Release process, isolation, storage | Release checklist |
| **OpenBSD** | Simplicity, secure defaults, `pledge`/`unveil`, auditing | Least-privilege security model | Threat model |
| **Debian** | Stable release discipline, packaging, dependency management | Package repository and update channels | Release/packaging policy |
| **Fedora** | CI, gating, atomic updates, developer experience | CI gates and update safety | CI/release gate design |
| **NixOS** | Declarative configuration and reproducibility | Declarative system configuration | Configuration schema |
| **OSDev tutorials** | Bootloader, GDT/IDT, interrupts, VGA, basic drivers | First bootable kernel milestones | QEMU boot trace |

---

## Study Curriculum

### Phase 1: OS Fundamentals (2 weeks)
- **Study:** Boot process (firmware, bootloader, kernel entry, initrd, init), CPU modes, privilege rings, interrupts, exceptions, syscalls, physical/virtual memory, paging, address spaces, processes, threads, scheduling, context switching, filesystems, VFS, file descriptors, mount namespaces.
- **Primary References:** xv6, OSDev tutorials, Linux kernel documentation.
- **SigmaOS Implementation:** Implement and validate QEMU x86_64 boot path, early serial logging, memory-map parsing, frame allocation, paging, interrupts, timer, scheduler, syscall entry, init, and shell. Produce boot trace showing each stage.
- **Exit Test:** QEMU x86_64 boots to a shell, executes a basic command, and shuts down cleanly.

### Phase 2: Kernel Services (3 weeks)
- **Study:** Linux scheduler classes and run queues, Linux VFS and mount semantics, Linux syscall ABI and error conventions, FreeBSD jail and OpenBSD `pledge`/`unveil` isolation concepts, IPC (pipes, signals, shared memory, sockets, event queues).
- **SigmaOS Implementation:** Scheduler with fairness and starvation safeguards, VFS abstraction with path resolution and file operations, process creation, exit, wait, signals, file descriptors, pipes, basic IPC, stable syscall table, initial security boundary model.
- **Exit Test:** A process can be created, open/read/write files, use pipes, receive a signal, and exit; all operations are covered by integration tests.

### Phase 3: Userspace and Distribution (3 weeks)
- **Study:** Arch package format and Pacman dependency resolution, Debian package metadata (DEB822 stanzas), repositories, stable-release policy, Fedora atomic update concepts, Mint update/recovery workflows, NixOS declarative configuration.
- **SigmaOS Implementation:** Minimal userland (shell, core utilities, service manager), RAM-disk/in-memory filesystem before persistent storage, package metadata parser, dependency resolver, dry-run package installation, update manifest, staged activation, rollback, recovery entry.
- **Exit Test:** A clean QEMU install completes, boots after installation, and a deliberately failed update rolls back to the previous known-good state.

### Phase 4: Security and Isolation (3 weeks)
- **Study:** OpenBSD secure defaults and audit culture, FreeBSD jails and capability-like confinement, Linux namespaces, cgroups, seccomp, Landlock, capabilities, package-script attack surfaces, supply-chain security, signed update channels.
- **SigmaOS Implementation:** Capability checks for filesystem, process, IPC, device, network, package execution; sandboxed package-script execution; signed update manifests; fuzzing for syscall, filesystem, IPC, package parsers; separate security review process.
- **Exit Test:** Every protected operation fails without the required capability, and malicious package scripts cannot escape their declared sandbox.

### Phase 5: Desktop and Hardware (4 weeks)
- **Study:** Linux input, DRM/KMS, display-server concepts, USB and PCI device enumeration, network driver and socket-stack layering, FreeBSD/OpenBSD hardware support matrices, Linux Mint hardware onboarding/troubleshooting patterns.
- **SigmaOS Implementation:** Minimal graphical session (framebuffer, input, window, app launch), VirtIO-blk, VirtIO-net, VGA/framebuffer, keyboard support in QEMU, device support matrix with model/driver/test date/logs. Real-hardware enablement only after QEMU parity.
- **Exit Test:** QEMU boots to a graphical session, accepts input, launches an application, and records no kernel panic.

---

## Repository Study Artifacts

Study results are stored under `research/`:

```text
research/
  upstream/
    xv6/
      boot.md
      memory.md
      filesystem.md
    linux/
      scheduler.md
      vfs.md
      drivers.md
    freebsd/
      release-engineering.md
      jails.md
    openbsd/
      secure-defaults.md
      pledge-unveil.md
    arch/
      packaging.md
    debian/
      release-policy.md
    fedora/
      ci-gating.md
    nixos/
      declarative-config.md

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

  reports/
    daily/
    weekly/
    monthly/
    quarterly/
```

Each upstream study note answers 7 canonical questions:
1. What problem does the upstream design solve?
2. What is its core data structure and control flow?
3. What are its security and failure assumptions?
4. What can SigmaOS adopt directly?
5. What must be simplified or redesigned for Rust/SigmaOS?
6. What test proves the SigmaOS version works?
7. What are the licensing and attribution implications?

---

## Design Decision Records (ADRs)

For every significant borrowed idea, write an Architecture Decision Record:

```markdown
# ADR-003: VFS interface

## Status
Accepted

## Context
SigmaOS needs a filesystem abstraction supporting multiple backends.

## Upstream study
Linux VFS separates generic operations from filesystem-specific implementations.
xv6 provides a simpler inode/path-based model.

## Decision
Adopt a trait-based VFS interface with explicit path resolution,
mount points, file descriptors, and standardized error codes.

## Consequences
- Filesystem backends can be added independently.
- Path resolution must be tested for traversal and permission edge cases.
- The first backend will be a RAM disk; persistent filesystems come later.

## Validation
- Unit tests for path resolution and file operations.
- Integration test across VFS and process layer.
- QEMU boot test using the RAM disk as root.
```

---

## 12 Implementation Rules

1. **Study one concept at a time:** Focus on a single mechanism (scheduler, VFS, package update, capability sandbox) rather than broad OS distributions.
2. **Extract the invariant, not the code:** Record the underlying rule (e.g., consistency under failure) rather than copying C or foreign code.
3. **License and attribution first:** Verify dual MIT/GPL-2.0 compatibility and respect upstream attribution requirements before adaptation. Prefer clean-room design notes and original Rust implementations.
4. **Implement the smallest vertical slice:** Build the minimum connected path: `boot -> memory -> scheduler -> syscall -> VFS -> shell`.
5. **Runtime validation is mandatory:** A component is complete only when exercised through real boot/runtime execution in QEMU. Unit-tested models do not establish runtime parity alone.
6. **CI gates merges:** All pull requests must pass `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, formatting, and QEMU smoke tests.
7. **Security changes get separate review:** Capability enforcement, sandboxing, syscalls, memory safety, cryptography, package execution, networking, and updates require a dedicated security review.
8. **Reproducibility:** Every experiment records commit hash, toolchain, container image, commands, timestamps, exit codes, and artifact hashes.
9. **Least privilege:** Restrict permissions for processes, scripts, CI jobs, and agents.
10. **Document adoption boundaries:** Explicitly state what was adopted, modified, simplified, or rejected and why.
11. **Regression tests are mandatory:** Every fix and feature includes an automated test that fails if behavior regresses.
12. **Human approval for high-risk work:** Mandatory human sign-off for bootloader, installer, update, security-boundary, package-execution, release, and destructive storage changes.

---

## Suggested 12-Week Schedule

| Week | Study focus | Implementation focus | Validation |
|---|---|---|---|
| 1 | xv6 boot, OSDev bootloader, interrupts | QEMU UEFI boot, serial logging | Boot trace |
| 2 | xv6 paging/processes, Linux memory basics | Frame allocator, paging, scheduler | Boot-to-shell |
| 3 | Linux syscalls/VFS, xv6 filesystem | Syscall table, VFS, RAM disk | File operations |
| 4 | Linux processes/signals, IPC | Processes, pipes, signals, FDs | Integration tests |
| 5 | Arch/Debian packaging | Package metadata parser, dependency resolver | Dry-run tests |
| 6 | Fedora atomic updates, Mint recovery | Update manifest, rollback, recovery | Failed-update test |
| 7 | OpenBSD secure defaults, Linux capabilities | Capability model, sandbox policy | Denial tests |
| 8 | FreeBSD jails, supply-chain security | Sandboxed package execution, signed updates | Security tests |
| 9 | Linux input/display, Redox architecture | Framebuffer, input, minimal compositor | Graphical boot |
| 10 | Driver models, virtio | virtio-blk, virtio-net, VGA | Device matrix |
| 11 | FreeBSD/OpenBSD release engineering | Release channels, known issues, rollback docs | Nightly artifact |
| 12 | NixOS declarative configuration | Configuration schema and validation | Config test suite |

---

## Definition of Done (10-Point Checklist)

1. Research note explains upstream design and SigmaOS adaptation.
2. ADR records the design decision.
3. Implementation has a clear module boundary and owner.
4. Unit and integration tests pass.
5. Feature is exercised through QEMU boot or real runtime path.
6. Automated regression test exists.
7. Security implications are reviewed.
8. Documentation states limitations and unsupported cases.
9. `CAPABILITY_MATRIX.toml` or component page is updated with evidence.
10. Change is reproducible from a clean checkout.

---

## First 30 Days Action Plan

1. Create `research/upstream/`, `research/decisions/`, `research/comparisons/`, and `research/reports/`.
2. Study xv6 boot and OSDev bootloader patterns; write `ADR-001-boot-path.md`.
3. Implement QEMU x86_64 UEFI boot with serial logging.
4. Study xv6 memory and Linux paging; write `ADR-002-memory-model.md`.
5. Implement frame allocation, paging, interrupts, and timer.
6. Study xv6 processes and Linux scheduler concepts; implement a minimal scheduler.
7. Add boot-to-shell smoke test.
8. Study Linux VFS and xv6 filesystem; write `ADR-003-vfs-interface.md`.
9. Implement a RAM-disk VFS and basic file operations.
10. Publish a weekly "studied → implemented → validated" report.
