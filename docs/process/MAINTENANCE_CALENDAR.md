# SigmaOS Maintenance Calendar and Operating Model

## Overview

SigmaOS operates on an **evidence-based maintenance operating model**. Every recurring task produces a reviewable artifact: a PR, issue, report, test result, or documented Architecture Decision Record (ADR). No task is considered "done" until its outcome is visible in the repository.

## Recurring Task Schedule

| Cadence | Task | Owner Role | Required Output / Artifact |
|---|---|---|---|
| **Daily** | CI Health Triage | Build/CI Maintainer | Fix PR or issue with assigned owner |
| **Weekly** | Dependency & Toolchain Review | Release Engineer | `chore(deps)` PR with `Cargo.lock` and toolchain evidence |
| **Weekly** | Security Advisory Review | Security Maintainer | `security(...)` PR or triage issue |
| **Weekly** | Open-Source Study | Component Maintainer | `docs/studies/YYYY-WXX-topic.md` PR |
| **Biweekly** | Component Gap Review | Component Maintainer | Gap issue (`gap(<comp>): ...`) + scoped feature PR |
| **Biweekly** | Documentation Accuracy Audit | Docs Maintainer | `docs(...)` PR matching code and tests |
| **Monthly** | Boot & Smoke-Test Validation | Platform Maintainer | Boot report in `docs/platform/BOOT_MATRIX.md` + fixes |
| **Monthly** | Fuzzing & Regression Review | Testing Maintainer | Fuzz findings PR + regression tests |
| **Monthly** | Installer & Recovery Rehearsal | Installer Maintainer | VM rehearsal report in `docs/operations/RECOVERY_RUNBOOK.md` + fixes |
| **Monthly** | Package & Update Rehearsal | Packaging Maintainer | Package transaction report + rollback tests |
| **Quarterly** | Release Readiness Review | Release Manager | Release checklist PR in `docs/release/` |
| **Quarterly** | Architecture & Roadmap Review | Tech Leads | ADR PR (`docs/adr/`) + roadmap updates |
| **Quarterly** | Hardware Support Audit | Driver Maintainer | Hardware matrix PR in `docs/platform/HARDWARE_SUPPORT.md` |
| **Semiannual** | License, Provenance & Supply-Chain Audit | Compliance Owner | SBOM & provenance PR |

## Daily Tasks Checklist

### CI Health Triage
- [ ] Monitor main branch nightly and PR CI status.
- [ ] Verify `cargo check --lib`, `cargo test --lib`, and `./run_sigma_tests.sh`.
- [ ] Triage any red builds immediately.
- [ ] Ensure QEMU boot-smoke workflow (`scripts/ci/qemu_boot_smoke.sh`) executes cleanly.

## Weekly Tasks Checklist

### Dependency & Toolchain Review
- [ ] Review Rust toolchain pins in `rust-toolchain.toml`.
- [ ] Check Cargo dependencies for updates and security patches.
- [ ] Verify GitHub Actions workflows and QEMU emulator dependencies.
- [ ] Prioritize security patches over non-essential updates.

### Security Advisory Review
- [ ] Check CVE feeds, RustSec advisories, and Linux/BSD security notes.
- [ ] Audit seccomp policies, capability handling, and cryptographic implementations.
- [ ] File `security(...)` PRs or issues for identified advisories.

### Open-Source Study
- [ ] Select a narrow mechanism from Linux, FreeBSD, OpenBSD, Redox, xv6, or Arch.
- [ ] Draft study notes under `docs/studies/YYYY-WXX-topic.md`.
- [ ] Evaluate current SigmaOS state against reference systems and document decision (Adopt/Adapt/Reject).

## Escalation Rules

1. **Broken Main Branch:** If CI fails on `main`, feature merges are frozen until a fix PR is merged.
2. **Boot Regressions:** If `scripts/ci/qemu_boot_smoke.sh` fails, platform maintainers block PR merges touching kernel boot or memory subsystems.
3. **Security Vulnerabilities:** Critical security advisories trigger immediate triage and hotfix PR creation within 24 hours.
