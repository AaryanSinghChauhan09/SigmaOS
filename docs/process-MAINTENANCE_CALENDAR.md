# SigmaOS Maintenance Calendar and Task Ownership

## Core Operating Principle

> **Every recurring task must produce a reviewable artifact: a PR, issue, report, test result, or documented decision. No task is “done” until its result is visible in the repository.**

SigmaOS explicitly distinguishes between:
1. **Code Exists**: Implementation files are committed in the repository.
2. **Tests Pass**: Logic unit tests or integration tests pass in CI.
3. **System Behavior Verified**: Real or simulated QEMU/VM boot, installation, hardware, or update behavior is confirmed with diagnostic logs.

---

## Recurring Task Schedule

| Frequency | Task ID | Task Name | Owner Role | Required Output / Deliverable |
|---|---|---|---|---|
| Daily | `daily_ci` | CI health triage | Build/CI Maintainer | Fix PR or triage issue with designated owner |
| Weekly | `weekly_deps` | Dependency and toolchain review | Release Engineer | `chore(deps)` PR with audit evidence |
| Weekly | `weekly_security` | Security advisory review | Security Maintainer | `security(...)` PR or security triage issue |
| Weekly | `weekly_study` | Open-source study | Component Maintainer | `docs/studies/...` study PR |
| Biweekly | `biweekly_gap` | Component gap review | Component Maintainer | Component gap issue + scoped feature PR |
| Biweekly | `biweekly_docs` | Documentation accuracy audit | Docs Maintainer | `docs(...)` accuracy revision PR |
| Monthly | `monthly_boot` | Boot and smoke-test validation | Platform Maintainer | QEMU boot report + platform matrix PR |
| Monthly | `monthly_fuzz` | Fuzzing and regression review | Testing Maintainer | Fuzz findings report + patch PR |
| Monthly | `monthly_installer` | Installer/recovery rehearsal | Installer Maintainer | VM install rehearsal report + runbook PR |
| Monthly | `monthly_package` | Package/update rehearsal | Packaging Maintainer | Package update transaction report |
| Quarterly | `quarterly_release` | Release readiness review | Release Manager | Release readiness checklist PR |
| Quarterly | `quarterly_architecture` | Architecture and roadmap review | Tech Lead | Architecture Decision Record (ADR) PR |
| Quarterly | `quarterly_hardware` | Hardware support audit | Driver Maintainer | Hardware evidence matrix PR |
| Semiannual | `semiannual_compliance` | License & supply-chain audit | Compliance Owner | Software Bill of Materials (SBOM) PR |

---

## Escalation and Enforcement Rules

1. **Red Main Branch Blocker**: A broken CI build on `main` blocks all non-essential feature merges until resolved.
2. **Evidence-Gated Status**: No component claim in `FEATURE_STATUS.toml` or `CAPABILITY_MATRIX.toml` may be changed to `Complete` without linking to a passing test or QEMU log.
3. **PR Standard**: Every recurring task output must use Conventional Commits and follow `.github/PULL_REQUEST_TEMPLATE.md`.
