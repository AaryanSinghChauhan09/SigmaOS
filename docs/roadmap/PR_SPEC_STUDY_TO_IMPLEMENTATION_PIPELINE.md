# PR Specification: Study-to-Implementation Pipeline for Upstream OS Concepts

**Specification ID:** `PR-SPEC-STUDY-TO-IMPLEMENTATION-PIPELINE`
**Target Subsystems:** `Architecture`, `Research`, `Kernel`, `Memory`, `VFS`, `Scheduler`, `Drivers`, `Security`, `Packaging`, `Desktop`, `Governance`

---

## 1. Specification Overview

This specification establishes the mandatory governance, research workflow, design decision process, and runtime evidence requirements for adopting concepts from upstream operating systems (Linux, Arch, Mint, Redox, xv6, FreeBSD, OpenBSD, Debian, Fedora, NixOS, OSDev) into SigmaOS.

---

## 2. 10-Step Repeatable Study Cycle

Every borrowed concept MUST follow:

1. **SELECT:** Choose one narrow, well-bounded OS mechanism.
2. **STUDY:** Analyze upstream implementation, data structures, control flow, and limitations.
3. **DESIGN NOTE:** Author a clean-room design note for SigmaOS.
4. **DEFINE SLICE:** Define the smallest testable vertical slice.
5. **IMPLEMENT:** Implement behind a strict module boundary in Rust (`#![no_std]` compliant where applicable).
6. **ADD TESTS:** Write unit, integration, and runtime test cases.
7. **VALIDATE:** Verify in QEMU x86_64 or on named hardware.
8. **DOCUMENT:** Explicitly state what was adopted, changed, simplified, or rejected.
9. **REGRESSION PROTECTION:** Add negative tests and regression checks.
10. **UPDATE CAPABILITY:** Update `CAPABILITY_MATRIX.toml` with runtime test evidence.

---

## 3. Reference System Deliverables & Curriculum

```text
Project          Study Deliverable                     Curriculum Target
-------------------------------------------------------------------------------
xv6              Annotated boot-to-shell map           Phase 1 (OS Fundamentals)
Linux Kernel     Interface comparison matrix           Phase 2 (Kernel Services)
Redox OS         Rust architecture note                Phase 2 & 5 (Rust OS / Window Server)
Arch Linux       Package & config design note          Phase 3 (Userspace & Distro)
Linux Mint       Recovery workflow                     Phase 3 (Update & Recovery)
FreeBSD          Release checklist                     Phase 4 & 5 (Isolation & Release)
OpenBSD          Threat model                          Phase 4 (Security & Confinement)
Debian           Release & packaging policy            Phase 3 (Packaging & Repositories)
Fedora           CI & release gate design              Phase 3 & 4 (Atomic Updates & CI)
NixOS            Configuration schema                  Phase 3 & 12-Week Roadmap
OSDev            QEMU boot trace                       Phase 1 (Early Kernel Milestones)
```

---

## 4. Mandatory Rules & Definition of Done

- **Rule 1 (Single Concept Scope):** Study mechanisms independently, not broad OS distributions.
- **Rule 2 (Extract Invariants):** Focus on underlying design principles, safety guarantees, and state transitions.
- **Rule 3 (License Safety):** Ensure dual MIT/GPL-2.0 compatibility and clean-room implementation.
- **Rule 4 (Smallest Vertical Path):** Connect boot → memory → scheduler → syscall → VFS → shell.
- **Rule 5 (Runtime Proof):** Require QEMU execution logs or serial traces; unit tests do not prove bootability.
- **Rule 6 (CI Gatekeeper):** Code must pass `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`.
- **Rule 7 (Independent Security Gate):** Capability, sandbox, syscall, and update changes require security review.
- **Rule 8 (Reproducibility):** Record commit hash, toolchain, command logs, exit codes, and artifact hashes.
- **Rule 9 (Least Privilege):** Confinement and permission checks must be default-deny.
- **Rule 10 (Adoption Boundaries):** State explicit rationale for adopted, modified, or rejected mechanisms.
- **Rule 11 (Mandatory Regression Tests):** Every fix or feature slice requires a regression test.
- **Rule 12 (Human Gatekeeper):** High-risk bootloader, installer, security, or storage changes require human signoff.

---

## 5. Evidence & Capability Matrix Integration

Capability promotion in `CAPABILITY_MATRIX.toml` occurs only when a studied concept satisfies all 10 criteria in the Definition of Done and provides verifiable QEMU serial output or test execution logs.
