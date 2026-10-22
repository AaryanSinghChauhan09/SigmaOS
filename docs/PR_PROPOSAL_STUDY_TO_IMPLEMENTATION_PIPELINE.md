# PR Proposal: SigmaOS Study-to-Implementation Pipeline & Reference Architecture

## Title
`feat(governance): enforce study -> extract pattern -> implement minimal slice -> validate pipeline`

## Description
Establishes the canonical 10-step Study-to-Implementation pipeline for borrowing upstream OS concepts from reference projects (xv6, Linux Kernel, Redox OS, Arch Linux, Linux Mint, FreeBSD, OpenBSD, Debian, Fedora, NixOS, OSDev) into SigmaOS.

## Core Directives & Pipeline Architecture

### The 10-Step Repeatable Cycle
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

### Projects to Study & Deliverables
- **xv6**: Boot, paging, processes, syscalls, filesystem -> Annotated boot-to-shell map (`research/upstream/xv6/`)
- **Linux Kernel**: VFS, scheduler, driver model, syscalls, namespaces -> Interface comparison matrix (`research/upstream/linux/`)
- **Redox OS**: Rust OS structure, capability-oriented design, userspace separation -> Rust architecture note (`research/upstream/redox/`)
- **Arch Linux**: Transparent configuration, package model, user control -> Package/config design note (`research/upstream/arch/`)
- **Linux Mint**: Onboarding, update safety, recovery, troubleshooting -> Recovery workflow (`research/upstream/mint/`)
- **FreeBSD**: Release engineering, jails, ZFS, networking -> Release checklist (`research/upstream/freebsd/`)
- **OpenBSD**: Simplicity, secure defaults, pledge/unveil, auditing -> Threat model (`research/upstream/openbsd/`)
- **Debian**: Stable release discipline, packaging, dependency management -> Release/packaging policy (`research/upstream/debian/`)
- **Fedora**: CI, gating, atomic updates, developer experience -> CI gates (`research/upstream/fedora/`)
- **NixOS**: Declarative configuration and reproducibility -> Configuration schema (`research/upstream/nixos/`)
- **OSDev**: Bootloader, GDT/IDT, interrupts, VGA, basic drivers -> QEMU boot trace (`research/upstream/osdev/`)

### Study Curriculum (5 Phases)
1. **Phase 1: OS Fundamentals (2 Weeks)** -> Boot-to-shell in QEMU x86_64.
2. **Phase 2: Kernel Services (3 Weeks)** -> EEVDF scheduler, VFS, process fork/exec, pipes, signals.
3. **Phase 3: Userspace and Distribution (3 Weeks)** -> SigmaPkg metadata, SAT solver, staged atomic updates & Timeshift rollback.
4. **Phase 4: Security and Isolation (3 Weeks)** -> Dual pledge/unveil capability model, sandboxed package execution.
5. **Phase 5: Desktop and Hardware (4 Weeks)** -> Graphical session, VirtIO drivers, WCAG AAA accessibility.

### Implementation Rules
- **Rule 1**: Study one concept at a time.
- **Rule 2**: Extract the invariant, not the code.
- **Rule 3**: License and attribution first (MIT/GPL-2.0 compatibility).
- **Rule 4**: Implement the smallest vertical slice.
- **Rule 5**: Runtime validation is mandatory (QEMU boot or real runtime path).
- **Rule 6**: CI gates merges (`./run_sigma_tests.sh` + formatting).
- **Rule 7**: Security changes get separate review.
- **Rule 8**: Reproducibility recorded for all experiments.
- **Rule 9**: Least privilege for agents, CI jobs, and package scripts.
- **Rule 10**: Document adoption boundaries (adopted, modified, rejected).
- **Rule 11**: Regression tests are mandatory.
- **Rule 12**: Human approval for high-risk work (bootloader, update, security, storage).

## Verification Evidence
- `src/governance/study_to_implementation_pipeline_pr.rs`
- `tests/test_study_to_implementation_pipeline.rs`
- `./run_sigma_tests.sh`
