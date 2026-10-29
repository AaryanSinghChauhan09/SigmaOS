# SigmaOS Maintenance Calendar and Operational Cadence

## Core Operating Principle

> **Every recurring task must produce a reviewable artifact: a PR, issue, report, test result, or documented decision. No task is "done" until its result is visible in the repository.**

SigmaOS establishes that passing unit tests alone does not prove bootability or installability, nor do driver names imply tested hardware support. Periodic work explicitly distinguishes **code exists**, **tests pass**, and **system behavior verified**.

---

## Task Calendar Overview

| Frequency | Task | Owner Role | Required Output |
|---|---|---|---|
| **Daily** | CI Health Triage | Build/CI Maintainer | Fix PR or triage issue |
| **Weekly** | Dependency & Toolchain Review | Release Engineer | `chore(deps)` PR |
| **Weekly** | Security Advisory Review | Security Maintainer | `security(...)` PR or issue |
| **Weekly** | Open-Source Mechanism Study | Component Maintainer | `docs/studies/...` PR |
| **Biweekly** | Component Gap Review | Component Maintainer | Gap issue + scoped feature PR |
| **Biweekly** | Documentation Accuracy Audit | Docs Maintainer | `docs(...)` PR |
| **Monthly** | Boot & Platform Smoke-Test Validation | Platform Maintainer | Boot report + matrix update |
| **Monthly** | Fuzzing & Regression Review | Testing Maintainer | Fuzz findings + fix PR |
| **Monthly** | Installer & Recovery Rehearsal | Installer Maintainer | VM install report + fixes |
| **Monthly** | Package & Update Rehearsal | Packaging Maintainer | Package transaction report |
| **Quarterly** | Release Readiness Review | Release Manager | Release checklist PR |
| **Quarterly** | Architecture & Roadmap Review | Tech Leads | ADR / roadmap update PR |
| **Quarterly** | Hardware Support Audit | Driver Maintainer | Hardware matrix PR |
| **Semiannual** | License, Provenance & Supply Chain Audit | Compliance Owner | SBOM / provenance PR |

---

## Detailed Operational Procedures

### Daily Tasks
- **CI Health Triage:** Executed on every pull request and nightly. The main branch must remain green. Required gates include `cargo fmt --check`, `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, Clippy, fuzz smoke targets, and QEMU boot smoke tests. Red main builds immediately block PR merges and trigger owner assignment.

### Weekly Tasks
- **Dependency and Toolchain Review:** Audits Rust toolchain, Cargo dependencies, GitHub Actions versions, QEMU version, and build scripts. Security patches are prioritized over feature upgrades. Lockfile modifications require test evidence.
- **Security Advisory Review:** Audits CVE feeds, RustSec advisories, distro security notices, and upstream kernel advisories. Feeds concrete security hardening PRs rather than passive proposals.
- **Open-Source Mechanism Study:** Investigates a focused mechanism from Linux, FreeBSD, OpenBSD, Redox, xv6, or Arch. Published as `docs/studies/YYYY-Wxx-<topic>.md`.

### Biweekly Tasks
- **Component Gap Review:** Evaluates a target subsystem against reference implementations, cataloging tested vs. modeled capabilities, and filing scoped gap issues via `.github/ISSUE_TEMPLATE/component_gap.md`.
- **Documentation Accuracy Audit:** Audits subsystem documentation against source code and test suites. Updates `FEATURE_STATUS.toml` and `CAPABILITY_MATRIX.toml` to reflect reality.

### Monthly Tasks
- **Boot and Platform Validation:** Executes QEMU x86_64 boot smoke tests (`scripts/ci/qemu_boot_smoke.sh` & `tests/boot/boot_to_shell_test.py`), records boot logs, and updates `docs/platform/BOOT_MATRIX.md`.
- **Fuzzing & Regression Review:** Executes `fuzz/` targets across syscall parsers, ELF loader, filesystem metadata, package metadata, installer configs, and network packets.
- **Installer & Recovery Rehearsal:** Executes VM installation dry-runs (`installer/tests/dry_run.rs`), tests failure injection (`installer/tests/failure_injection.rs`), and validates recovery procedures (`docs/operations/RECOVERY_RUNBOOK.md`).
- **Package & Update Rehearsal:** Validates package build reproducibility, dependency resolution, signature verification, interrupted update recovery, and atomic rollback paths.

### Quarterly Tasks
- **Release Readiness Review:** Evaluates release readiness tiers against `docs/release/RELEASE_CHECKLIST.md` and `docs/release/RELEASE_POLICY.md`.
- **Architecture & Roadmap Review:** Reviews cross-component sequencing, API stability, security architecture, and files Architectural Decision Records (ADRs).
- **Hardware Support Audit:** Audits driver classes against diagnostic collections (`scripts/hardware/collect_diagnostics.sh`) and updates `docs/platform/HARDWARE_SUPPORT.md`.

### Semiannual Tasks
- **License, Provenance, and Supply-Chain Audit:** Audits open-source licenses, generates SBOM artifacts, and verifies supply-chain provenance.

---

## Escalation and Resolution Rules
1. **Red Main Branch:** Any failure on `main` halts non-critical feature merges until resolved or reverted.
2. **Unverified Claims:** Any documentation claim lacking a backing unit test, boot log, or diagnostic evidence must be flagged and corrected during the biweekly documentation audit.
3. **Security Vulnerabilities:** Critical security advisories bypass standard queue priority and require immediate triage within 24 hours.
