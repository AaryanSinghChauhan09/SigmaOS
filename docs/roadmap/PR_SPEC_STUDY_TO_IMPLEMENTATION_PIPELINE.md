# Roadmap Specification: Study-to-Implementation Pipeline

## Specification Metadata
- **Spec Name**: `PR_SPEC_STUDY_TO_IMPLEMENTATION_PIPELINE`
- **Module Path**: `src/governance/study_to_implementation_pipeline_pr.rs`
- **Status**: Implemented / Verified
- **Target Release**: SigmaOS 1.0 Milestone

---

## Overview
This specification governs the 10-step repeatable cycle, 11-project study focus matrix, 5-phase study curriculum, research directory structure, ADR standards, 12 mandatory rules, 12-week schedule, definition of done, and first 30 days roadmap for borrowing and adapting concepts from upstream operating systems into SigmaOS.

---

## 1. Repeatable 10-Step Cycle
1. **SELECT**: Choose a narrow OS concept.
2. **STUDY**: Analyze upstream implementation and documentation.
3. **WRITE**: Author a design note in `research/upstream/`.
4. **DEFINE**: Specify the minimum testable slice.
5. **IMPLEMENT**: Code behind clean `no_std` + `alloc` Rust module boundaries.
6. **ADD TESTS**: Implement unit, integration, and runtime tests.
7. **VALIDATE**: Exercise via QEMU or named hardware runtime path.
8. **DOCUMENT**: Record adopted, modified, and rejected aspects in an ADR.
9. **REGRESSION**: Add a mandatory regression test.
10. **UPDATE**: Log runtime evidence in `CAPABILITY_MATRIX.toml`.

---

## 2. 11 Upstream Study Targets
- **xv6**: Boot-to-shell reference path and minimal kernel clarity.
- **Linux Kernel**: VFS abstractions, scheduler, driver model, namespaces.
- **Redox OS**: Safe Rust OS architecture, scheme IPC, capability model.
- **Arch Linux**: Transparent package model, PKGINFO parser, PKGBUILD.
- **Linux Mint**: Desktop onboarding, update safety, Timeshift recovery.
- **FreeBSD**: Jail container isolation, release engineering, ZFS boot environments.
- **OpenBSD**: Least-privilege `pledge`/`unveil`, W^X memory, proactive auditing.
- **Debian**: DEB822 metadata stanzas, multi-channel release policy.
- **Fedora**: Automated Greenwave CI gating, staged atomic updates.
- **NixOS**: Declarative TOML/Nix configuration schema and generation rollbacks.
- **OSDev**: Bare-metal x86_64 UEFI boot trace, GDT/IDT, COM1 serial logging.

---

## 3. 5-Phase Study Curriculum
- **Phase 1 (2 wks)**: OS Fundamentals — QEMU x86_64 boot trace, paging, scheduler, shell.
- **Phase 2 (3 wks)**: Kernel Services — EEVDF scheduler, trait VFS, process fork/exec, IPC, syscalls.
- **Phase 3 (3 wks)**: Userspace & Distribution — Package metadata, SAT dependency solver, staged updates, rollback.
- **Phase 4 (3 wks)**: Security & Isolation — pledge/unveil, Landlock sandboxing, capability checks, signed updates.
- **Phase 5 (4 wks)**: Desktop & Hardware — VirtIO drivers, framebuffer graphics, Zenith Compositor, hardware matrix.

---

## 4. Research Artifacts Layout
- `research/upstream/`: Study notes per project (`xv6`, `linux`, `freebsd`, `openbsd`, `arch`, `debian`, `fedora`, `nixos`, `redox`, `mint`, `osdev`).
- `research/decisions/`: Architecture Decision Records (`ADR-001` through `ADR-005`).
- `research/comparisons/`: Domain comparison matrices (scheduler, VFS, security, packaging, hardware).
- `research/reports/`: Weekly "studied → implemented → validated" reports.

---

## 5. 12 Mandatory Implementation Rules
1. Study one concept at a time.
2. Extract the invariant, not the code.
3. License and attribution first (MIT / GPL-2.0 compatibility).
4. Implement the smallest vertical slice.
5. Runtime validation is mandatory.
6. CI gates merges (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`).
7. Security changes get separate review.
8. Reproducibility (log commit, toolchain, hashes).
9. Least privilege.
10. Document adoption boundaries.
11. Regression tests are mandatory.
12. Human approval for high-risk work.

---

## Verification
Executed via `tests/test_study_to_implementation_pipeline.rs` and `./run_sigma_tests.sh`.
