# Pull Request Proposal: Study-to-Implementation Pipeline

## PR Details
- **PR Title**: `feat(governance): Adopt Study → Extract Pattern → Implement Minimal Slice → Validate at Runtime Pipeline`
- **Target Branch**: `main`
- **Branch Name**: `feature/study-to-implementation-pipeline`
- **Status**: Ready for Review / Merge
- **Category**: Governance, Architecture & Release Pipeline Engineering

---

## Executive Summary
SigmaOS uses a strict **study → extract pattern → implement minimal slice → validate at runtime** pipeline. The repository references Linux, Arch, Mint, Redox, xv6, FreeBSD/OpenBSD, Debian, Fedora, NixOS, and OSDev ideas, and includes agent guides for kernel, memory, filesystem, networking, security, desktop, packaging, drivers, and distro compatibility—with explicit clarity in `README.md` that source presence and unit tests do not establish bootability, installation readiness, or hardware support. Reference projects serve as sources of **design insight**, not feature checklists.

This PR proposal codifies the 10-step repeatable cycle, 11-project study focus matrix, 5-phase study curriculum, research directory artifacts layout, Architecture Decision Record (ADR) standards, 12 mandatory implementation rules, 12-week execution schedule, 10-point Definition of Done, and 30-day initial execution roadmap.

---

## Study-to-Implementation Pipeline

Use one repeatable 10-step cycle for every concept borrowed from an upstream OS project:

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

A reference project is a source of **design insight**, not a feature checklist. SigmaOS’s own documentation warns that reference-project feature names do not establish SigmaOS parity.

---

## Projects to Study

Study these in a fixed order, with one deliverable per project:

| Project | Study Focus | SigmaOS Application | Study Deliverable |
|---|---|---|---|
| xv6 | Minimal kernel clarity: boot, paging, processes, syscalls, filesystem | Build an understandable reference kernel path | Annotated boot-to-shell map |
| Linux Kernel | VFS, scheduler, driver model, syscalls, namespaces | Scalable kernel abstractions | Interface comparison matrix |
| Redox OS | Rust OS structure, capability-oriented design, userspace separation | Rust-native architecture and unsafe-code policy | Rust architecture note |
| Arch Linux | Transparent configuration, package model, user control | SigmaPkg and system configuration | Package/config design note |
| Linux Mint | Onboarding, update safety, recovery, troubleshooting | Installer and update/rollback UX | Recovery workflow |
| FreeBSD | Release engineering, jails, ZFS, networking | Release process, isolation, storage | Release checklist |
| OpenBSD | Simplicity, secure defaults, `pledge`/`unveil`, auditing | Least-privilege security model | Threat model |
| Debian | Stable release discipline, packaging, dependency management | Package repository and update channels | Release/packaging policy |
| Fedora | CI, gating, atomic updates, developer experience | CI gates and update safety | CI/release gate design |
| NixOS | Declarative configuration and reproducibility | Declarative system configuration | Configuration schema |
| OSDev tutorials | Bootloader, GDT/IDT, interrupts, VGA, basic drivers | First bootable kernel milestones | QEMU boot trace |

---

## Study Curriculum

### Phase 1: OS Fundamentals — 2 weeks

**Study**
- Boot process: firmware, bootloader, kernel entry, initrd, init.
- CPU modes, privilege rings, interrupts, exceptions, and system calls.
- Physical and virtual memory, paging, address spaces.
- Processes, threads, scheduling, context switching.
- Filesystems, VFS, file descriptors, and mount namespaces.

**Primary References**
- xv6 for a clean minimal implementation.
- OSDev tutorials for bootloader and interrupt basics.
- Linux kernel documentation for production-scale concepts.

**SigmaOS Implementation**
- Implement or validate a QEMU x86_64 boot path.
- Implement early serial logging, memory-map parsing, frame allocation, paging, interrupts, timer, scheduler, syscall entry, init, and shell.
- Produce a boot trace showing each stage.

**Exit Test**
QEMU x86_64 boots to a shell, executes a basic command, and shuts down cleanly.

---

### Phase 2: Kernel Services — 3 weeks

**Study**
- Linux scheduler classes and run queues.
- Linux VFS and mount semantics.
- Linux syscall ABI and error conventions.
- FreeBSD jail and OpenBSD `pledge`/`unveil` isolation concepts.
- IPC: pipes, signals, shared memory, sockets, event queues.

**SigmaOS Implementation**
- Scheduler with fairness and starvation safeguards.
- VFS abstraction with path resolution and file operations.
- Process creation, exit, wait, signals, and file descriptors.
- Pipes, basic IPC, and a stable syscall table.
- Initial security boundary model.

**Exit Test**
A process can be created, open/read/write files, use pipes, receive a signal, and exit; all operations are covered by integration tests.

---

### Phase 3: Userspace and Distribution — 3 weeks

**Study**
- Arch package format and Pacman dependency resolution.
- Debian package metadata, repositories, and stable-release policy.
- Fedora atomic update concepts.
- Mint update/recovery workflows.
- NixOS declarative configuration.

**SigmaOS Implementation**
- Minimal userland: shell, core utilities, service manager.
- RAM-disk or simple filesystem before persistent storage.
- Package metadata parser and dependency resolver.
- Dry-run package installation.
- Update manifest, staged activation, rollback, and recovery entry.

**Exit Test**
A clean QEMU install completes, boots after installation, and a deliberately failed update rolls back to the previous known-good state.

---

### Phase 4: Security and Isolation — 3 weeks

**Study**
- OpenBSD secure defaults and audit culture.
- FreeBSD jails and capability-like confinement.
- Linux namespaces, cgroups, seccomp, Landlock, and capabilities.
- Package-script attack surfaces.
- Supply-chain security and signed update channels.

**SigmaOS Implementation**
- Capability checks for filesystem, process, IPC, device, network, and package execution.
- Sandboxed package-script execution.
- Signed update manifests.
- Fuzzing for syscall, filesystem, IPC, and package parsers.
- Security review process separate from ordinary code review.

**Exit Test**
Every protected operation fails without the required capability, and malicious package scripts cannot escape their declared sandbox.

---

### Phase 5: Desktop and Hardware — 4 weeks

**Study**
- Linux input, DRM/KMS, and display-server concepts.
- USB and PCI device enumeration.
- Network driver and socket-stack layering.
- FreeBSD/OpenBSD hardware support matrices.
- Linux Mint hardware onboarding and troubleshooting patterns.

**SigmaOS Implementation**
- Minimal graphical session: framebuffer, input, window, application launch.
- Virtio-blk, virtio-net, VGA/framebuffer, and keyboard support in QEMU.
- Device support matrix with model, driver, test date, and logs.
- Real-hardware enablement only after QEMU parity.

**Exit Test**
QEMU boots to a graphical session, accepts input, launches an application, and records no kernel panic.

---

## Repository Study Artifacts

Create and maintain a structured `research/` directory hierarchy:

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
    redox/
      scheme-ipc.md
    mint/
      timeshift-recovery.md
    osdev/
      bootloader-uefi.md

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
    weekly/
```

Each upstream study note must answer:

1. What problem does the upstream design solve?
2. What is its core data structure and control flow?
3. What are its security and failure assumptions?
4. What can SigmaOS adopt directly?
5. What must be simplified or redesigned for Rust/SigmaOS?
6. What test proves the SigmaOS version works?
7. What are the licensing and attribution implications?

---

## Architecture Decision Records (ADRs)

For every significant borrowed idea, write an Architecture Decision Record:

### ADR Template
```markdown
# ADR-003: VFS interface

## Status
Proposed / Accepted

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

## Mandatory Implementation Rules

1. **Rule 1: Study one concept at a time** — Do not study "Linux" or "BSD" broadly. Study one mechanism at a time.
2. **Rule 2: Extract the invariant, not the code** — Record the underlying rule (e.g., error consistency on failure) rather than copying code.
3. **Rule 3: License and attribution first** — Verify dual-license (MIT/GPL-2.0) compatibility before adapting any code.
4. **Rule 4: Implement the smallest vertical slice** — Build the minimum connected path: boot → memory → scheduler → syscall → VFS → shell.
5. **Rule 5: Runtime validation is mandatory** — Code must be exercised in actual boot or runtime execution; unit tests alone do not establish parity.
6. **Rule 6: CI gates merges** — Required checks include `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, formatting, and QEMU smoke tests.
7. **Rule 7: Security changes get separate review** — Capabilities, sandboxing, syscalls, crypto, and update code require dedicated security review.
8. **Rule 8: Reproducibility** — Record commit hash, toolchain, container image, commands, timestamps, exit codes, and hashes.
9. **Rule 9: Least privilege** — Agents, CI jobs, and services receive minimal required privileges.
10. **Rule 10: Document adoption boundaries** — State what SigmaOS adopted, simplified, changed, rejected, and why.
11. **Rule 11: Regression tests are mandatory** — Every mechanism needs a test that fails if the behavior regresses.
12. **Rule 12: Human approval for high-risk work** — Required for bootloader, installer, update, security boundary, package execution, release, and storage changes.

---

## Suggested 12-Week Schedule

| Week | Study Focus | Implementation Focus | Validation |
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

## Definition of Done

A studied concept is considered implemented in SigmaOS only when:

1. A research note explains the upstream design and SigmaOS adaptation.
2. An ADR records the design decision.
3. The implementation has a clear module boundary and owner.
4. Unit and integration tests pass.
5. The feature is exercised through QEMU boot or another real runtime path.
6. A regression test exists.
7. Security implications are reviewed.
8. Documentation states limitations and unsupported cases.
9. `CAPABILITY_MATRIX.toml` or the relevant component page is updated with evidence.
10. The change is reproducible from a clean checkout.

---

## First 30 Days Roadmap

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

---

## Verification & Test Results
```bash
# Execute standalone Study-to-Implementation Pipeline test suite
mkdir -p build
rustc --test tests/test_study_to_implementation_pipeline.rs --edition=2021 -o build/test_study_pipeline
./build/test_study_pipeline
```

Output:
```text
running 4 tests
test study_to_implementation_pipeline_pr::tests::test_pr_gateway_summary_generation ... ok
test study_to_implementation_pipeline_pr::tests::test_pipeline_engine_initialization ... ok
test test_gateway_pr_summary_output ... ok
test test_full_pipeline_verification ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
