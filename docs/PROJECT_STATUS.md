# SigmaOS Project Status & Metrics

## Operational Metrics
This document tracks the operational metrics for SigmaOS runtime stability, test coverage, and documentation honesty.

| Metric | Target | Current Status |
| :--- | :--- | :--- |
| QEMU x86_64 boot-to-shell success | 100% on CI | Verified (`scripts/ci/qemu_boot_smoke.sh`) |
| Boot smoke-test duration | Under 5 minutes | Verified (< 1 min in QEMU smoke) |
| PRs with tests or documented exemption | 100% | Enforced via `.github/PULL_REQUEST_TEMPLATE.md` |
| Components with documented limitations | 100% | Enforced via `COMPONENT_RULES.toml` |
| Security-sensitive PRs with security review | 100% | Enforced via `SECURITY_ARCHITECTURE.md` |
| Installer dry-run pass rate | 100% | Verified (`installer/tests/dry_run.rs`) |
| Driver claims backed by evidence | 100% | Verified (`docs/platform/HARDWARE_SUPPORT.md`) |
| Open gaps with an owner and acceptance criteria | 100% | Tracked via `.github/ISSUE_TEMPLATE/gap.md` |

## Governing Principle
A component is not "implemented" until its runtime path is exercised, its failure modes are tested, its documentation states its limits, and its status is updated from evidence—not intention.
