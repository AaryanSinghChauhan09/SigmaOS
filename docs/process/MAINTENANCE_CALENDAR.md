# SigmaOS Maintenance Calendar & Operational Model

This document establishes the recurring operational cadence for maintaining SigmaOS as a high-integrity, evidence-based open-source operating system project.

## Operational Principle

> **Every recurring task must produce a reviewable artifact: a PR, issue, report, test result, or documented decision. No task is "done" until its result is visible in the repository.**

Passing unit tests alone does not establish bootability or installability, and driver names do not imply verified hardware support. Periodic maintenance explicitly distinguishes **code exists**, **tests pass**, and **system behavior verified**.

---

## Recurring Task Schedule Matrix

| Frequency | Task | Owner Role | Required PR / Output Artifact |
|---|---|---|---|
| **Daily** | CI health triage | Build / CI Maintainer | Fix PR or triage issue with owner |
| **Weekly** | Dependency and toolchain review | Release Engineer | `chore(deps)` PR |
| **Weekly** | Security advisory review | Security Maintainer | `security(...)` PR or triage issue |
| **Weekly** | Open-source study | Component Maintainer | `docs/studies/...` PR |
| **Biweekly** | Component gap review | Component Maintainer | Gap issue + scoped feature PR |
| **Biweekly** | Documentation accuracy audit | Docs Maintainer | `docs(...)` PR |
| **Monthly** | Boot and smoke-test validation | Platform Maintainer | Boot report + matrix update |
| **Monthly** | Fuzzing and regression review | Testing Maintainer | Fuzz findings + fix PR |
| **Monthly** | Installer/recovery rehearsal | Installer Maintainer | VM install report + runbook fix |
| **Monthly** | Package/update rehearsal | Packaging Maintainer | Package transaction report |
| **Quarterly** | Release readiness review | Release Manager | Release checklist PR |
| **Quarterly** | Architecture and roadmap review | Tech Leads | ADR / roadmap PR |
| **Quarterly** | Hardware support audit | Driver Maintainer | Hardware support matrix PR |
| **Semiannual** | License, provenance, and supply-chain audit | Compliance Owner | SBOM / provenance PR |

---

## Detailed Task Runbooks

### 1. Daily: CI Health Triage
- **Trigger**: Automated on every PR and nightly at 02:00 UTC.
- **Goal**: Maintain 100% green main branch. A red main branch blocks releases and feature merges.
- **Execution Checklist**:
  1. Check CI status across `.github/workflows/`.
  2. If any job fails, isolate cause (flaky test, toolchain break, environment missing).
  3. Create fix PR using title format: `fix(ci): <imperative summary>`.
  4. If resolution requires upstream fix or multi-step work, open a blocking issue and assign owner.

### 2. Weekly: Dependency & Toolchain Review
- **Goal**: Refresh toolchain and Cargo dependencies safely.
- **Execution Checklist**:
  1. Audit toolchain versions (`rust-toolchain.toml`) and GitHub Actions pinned SHAs.
  2. Run `cargo audit` to detect dependency CVEs.
  3. Security patches take precedence over feature upgrades.
  4. Run full test suite (`cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`).
  5. Submit `chore(deps)` PR with test evidence and risk assessment.

### 3. Weekly: Security Advisory Review
- **Goal**: Monitor CVE feeds, RustSec advisories, and upstream Linux/BSD security bulletins.
- **Execution Checklist**:
  1. Review security feeds for affected crates or subsystem patterns.
  2. Evaluate SigmaOS exposure against capability model and syscall boundary.
  3. Submit `security(<component>)` PR with regression test or triage issue.

### 4. Weekly: Open-Source Study
- **Goal**: Deep-dive into mature mechanisms in Linux, FreeBSD, OpenBSD, Redox, xv6, or Arch.
- **Execution Checklist**:
  1. Create study document under `docs/studies/YYYY-Wxx-<topic>.md`.
  2. Document reference mechanisms, SigmaOS current state, decisions (Adopt/Adapt/Reject), and follow-up tasks.

### 5. Biweekly: Component Gap Review
- **Goal**: systematically track missing capabilities across 15 component domains.
- **Execution Checklist**:
  1. Select a component domain (Boot, Kernel, Memory, Scheduler, VFS, Syscalls, Drivers, Net, IPC, Security, Desktop, Installer, Packaging, Docs, CI).
  2. Create gap issue using `.github/ISSUE_TEMPLATE/component_gap.md`.
  3. Submit scoped feature PR addressing acceptance criteria.

### 6. Biweekly: Documentation Accuracy Audit
- **Goal**: Enforce status honesty across docs and TOML matrices.
- **Execution Checklist**:
  1. Audit documentation against actual code and unit/integration tests.
  2. Revise claims without runtime/boot evidence.
  3. Synchronize `FEATURE_STATUS.toml` and `CAPABILITY_MATRIX.toml`.

### 7. Monthly: Boot & Platform Validation
- **Goal**: Execute QEMU boot smoke test and record boot-to-shell state.
- **Execution Checklist**:
  1. Run `scripts/ci/qemu_boot_smoke.sh` and `python3 tests/boot/boot_to_shell_test.py`.
  2. Update `docs/platform/BOOT_MATRIX.md` with boot log links, duration, and status.

### 8. Monthly: Fuzzing & Regression Review
- **Goal**: Exercise fuzz targets and fix memory safety / panic bugs.
- **Execution Checklist**:
  1. Run fuzzers in `fuzz/` (syscalls, ELF loader, VFS parser, network stack).
  2. File crash reports and submit `fix(<component>)` PRs with regression tests.

### 9. Monthly: Installer & Recovery Rehearsal
- **Goal**: Prove installer reliability and failure recovery paths in VM environment.
- **Execution Checklist**:
  1. Execute `cargo test --test dry_run` and `cargo test --test failure_injection`.
  2. Update `docs/operations/RECOVERY_RUNBOOK.md` with findings.

### 10. Monthly: Package & Update Rehearsal
- **Goal**: Validate package manager transpilations, atomic update transactional integrity, and rollbacks.
- **Execution Checklist**:
  1. Run package update rehearsal suite.
  2. Verify sigpkg signature verification and state rollback invariants.

### 11. Quarterly: Release Readiness Review
- **Goal**: Audit SigmaOS against release tiers (Experimental, Developer Preview, Test Distribution, Release Candidate, Stable).
- **Execution Checklist**:
  1. Evaluate release criteria checklist in `docs/release/RELEASE_CHECKLIST.md`.
  2. Submit quarterly release readiness PR detailing status and blockers.

### 12. Quarterly: Architecture & Roadmap Review
- **Goal**: Evaluate cross-component architecture, API stability, and technical debt.
- **Execution Checklist**:
  1. Draft ADRs in `docs/adr/` for major architectural decisions.
  2. Update `FUTURE-DEVELOPMENT-ROADMAP.md` and prune unverified narrative claims.

### 13. Quarterly: Hardware Support Audit
- **Goal**: Audit driver claims against real or emulated execution logs.
- **Execution Checklist**:
  1. Execute `scripts/hardware/collect_diagnostics.sh`.
  2. Update `docs/platform/HARDWARE_SUPPORT.md` driver matrix.

### 14. Semiannual: License, Provenance & Supply-Chain Audit
- **Goal**: Verify dependency license compliance, SBOM generation, and cosign signatures.
- **Execution Checklist**:
  1. Generate SPDX/CycloneDX SBOM.
  2. Audit third-party license compatibility and submit provenance report PR.

---

## Escalation Rules & Release Blockers

1. **Broken Build / Main Red**: Immediate stop-the-line escalation. All feature merges paused until main build is green.
2. **Boot Regression**: Failure of `qemu_boot_smoke.sh` blocks any PR touching bootloader, kernel initialization, or MMU.
3. **Unverified Driver Claims**: Driver code without hardware or VM test evidence must be tagged `Status: Untested / Experimental` in `docs/platform/HARDWARE_SUPPORT.md`.
