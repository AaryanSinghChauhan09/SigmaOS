# SigmaOS Recurring Task Maintenance Calendar

This document defines the recurring task schedules, owner roles, required review artifacts, evidence rules, and escalation pathways for the SigmaOS operating system project.

## Core Operating Principle

> **Every recurring task must produce a reviewable artifact: a PR, issue, report, test result, or documented decision. No task is “done” until its result is visible in the repository.**

In accordance with SigmaOS governance, passing unit tests does not establish bootability or installability, and driver presence does not imply tested hardware support. Periodic work must explicitly distinguish:
1. **Code Exists** (`0`)
2. **Tests Pass** (`1`)
3. **System Behavior Verified** (`2`)

---

## Recurring Schedule Matrix

| Frequency | Task | Owner Role | Required Output / Artifact | Verification Gate |
|---|---|---|---|---|
| **Daily** | CI Health Triage | Build / CI Maintainer | Fix PR or GitHub Issue | System Behavior Verified |
| **Weekly** | Dependency & Toolchain Review | Release Engineer | `chore(deps)` PR | Tests Pass |
| **Weekly** | Security Advisory Review | Security Maintainer | `security(...)` PR or Triage Issue | System Behavior Verified |
| **Weekly** | Open-Source Mechanism Study | Component Maintainer | `docs/studies/...` PR | Code Exists |
| **Biweekly** | Component Gap Review | Component Maintainer | Gap Issue + Scoped PR | System Behavior Verified |
| **Biweekly** | Documentation Accuracy Audit | Docs Maintainer | `docs(...)` PR | Code Exists |
| **Monthly** | Boot & Platform Validation | Platform Maintainer | Boot Report (`docs/platform/BOOT_MATRIX.md`) | System Behavior Verified |
| **Monthly** | Fuzzing & Regression Review | Testing Maintainer | Fuzz Findings + Fix PR | Tests Pass |
| **Monthly** | Installer & Recovery Rehearsal | Installer Maintainer | VM Install Report (`docs/operations/RECOVERY_RUNBOOK.md`) | System Behavior Verified |
| **Monthly** | Package & Update Rehearsal | Packaging Maintainer | Transaction Report | System Behavior Verified |
| **Quarterly** | Release Readiness Review | Release Manager | Release Checklist PR | System Behavior Verified |
| **Quarterly** | Architecture & Roadmap Review | Tech Leads | ADR / Roadmap PR | Code Exists |
| **Quarterly** | Hardware Support Audit | Driver Maintainer | Hardware Matrix PR (`docs/platform/HARDWARE_SUPPORT.md`) | System Behavior Verified |
| **Semiannual** | Supply-Chain & License Audit | Compliance Owner | SBOM / Provenance PR | Code Exists |

---

## Task Execution Guidelines

### Daily Tasks
- **CI Health Triage**: Executed on every PR and nightly workflows. A broken main branch blocks all non-essential PR merges. Red builds must be triaged within 24 hours.

### Weekly Tasks
- **Dependency Review**: Audit `Cargo.lock`, pinned Rust toolchain in `rust-toolchain.toml`, and GitHub Actions versions. Security patches take precedence over feature additions.
- **Security Advisory Review**: Monitor CVE feeds, RustSec advisories, and kernel security bulletins. Mitigations feed directly into hardening PRs.
- **Open-Source Mechanism Study**: Publish structured studies under `docs/studies/YYYY-Wxx-<topic>.md` comparing Linux, FreeBSD, OpenBSD, Redox, or xv6 against SigmaOS.

### Biweekly Tasks
- **Component Gap Review**: Systematically analyze core subsystems (boot, memory, scheduler, VFS, networking, security, installer) using `.github/ISSUE_TEMPLATE/component_gap.md`.
- **Documentation Accuracy Audit**: Ensure `FEATURE_STATUS.toml` and `CAPABILITY_MATRIX.toml` accurately reflect runtime evidence rather than theoretical modeling.

### Monthly Tasks
- **Boot & Platform Validation**: Execute `scripts/ci/qemu_boot_smoke.sh` and log exact boot-to-shell timing and diagnostic output in `docs/platform/BOOT_MATRIX.md`.
- **Fuzzing & Regression Review**: Run fuzz targets in `fuzz/` against syscalls, VFS path parsing, and package metadata parsers.
- **Installer & Recovery Rehearsal**: Execute dry-run and failure injection tests (`installer/tests/dry_run.rs`, `installer/tests/failure_injection.rs`) and update `docs/operations/RECOVERY_RUNBOOK.md`.
- **Package Rehearsal**: Verify atomic package transactions, rollback safety, and signature validation.

### Quarterly & Semiannual Tasks
- **Release Readiness**: Evaluate release tiers (Experimental -> Developer Preview -> Test Distribution -> Release Candidate -> Stable).
- **Hardware Audit**: Update driver evidence ledgers in `docs/platform/HARDWARE_SUPPORT.md`.
- **Supply-Chain & Licensing**: Re-generate SBOMs, verify SPDX license compliance, and audit third-party dependencies.

---

## Escalation and Unblocking Rules
1. **Broken Main Branch**: Immediately alerts Build Maintainer; merges frozen until fixed or reverted.
2. **Critical Security Advisory**: Bypasses routine sprint scheduling; immediate fix PR or mitigation issue within 48 hours.
3. **Unverified Claims**: Any status matrix claim lacking linked test evidence or boot logs is downgraded during quarterly reviews.
