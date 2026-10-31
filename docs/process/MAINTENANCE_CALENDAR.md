# SigmaOS Maintenance Operating Model: Recurring Task Calendar

## Overview
SigmaOS enforces a strict, scheduled, evidence-based maintenance operating model. The project rejects unbounded feature roadmaps without operational hygiene. Every recurring task must produce a reviewable artifact: a pull request, issue, test report, or documented architecture decision. No task is considered "done" until its result is committed and visible in the repository.

Periodic work distinguishes three levels of truth:
1. **Code exists**: The source implementation has been written.
2. **Tests pass**: Automated unit/model tests pass.
3. **System behavior verified**: The behavior has been demonstrated on physical hardware or a full-system emulator (e.g., QEMU boot-to-shell, dry-run VM installer).

---

## Recurring Task Schedule

| Frequency | Task | Owner Role | Required PR/Output |
|---|---|---|---|
| Daily | CI health triage | Build/CI Maintainer | Fix PR or issue with owner |
| Weekly | Dependency and toolchain review | Release Engineer | `chore(deps)` PR |
| Weekly | Security advisory review | Security Maintainer | `security(...)` PR or triage issue |
| Weekly | Open-source study | Component Maintainer | `docs/studies/...` PR |
| Biweekly | Component gap review | Component Maintainer | Gap issue + scoped feature PR |
| Biweekly | Documentation accuracy audit | Docs Maintainer | `docs(...)` PR |
| Monthly | Boot and smoke-test validation | Platform Maintainer | Boot report + fixes |
| Monthly | Fuzzing and regression review | Testing Maintainer | Fuzz findings + fixes |
| Monthly | Installer/recovery rehearsal | Installer Maintainer | VM install report + fixes |
| Monthly | Package/update rehearsal | Packaging Maintainer | Package transaction report |
| Quarterly | Release readiness review | Release Manager | Release checklist PR |
| Quarterly | Architecture and roadmap review | Tech Leads | ADR/roadmap PR |
| Quarterly | Hardware support audit | Driver Maintainer | Hardware matrix PR |
| Semiannual | License, provenance, and supply-chain audit | Compliance Owner | SBOM/provenance PR |

---

## Escalation and Operational Rules

1. **Red Main Branch**: A broken build or failing check on the `main` branch is an immediate release blocker. The Build/CI Maintainer must revert or patch within 4 hours.
2. **Untested Claims**: Any claim of feature completeness without a linked test result or runtime log must be flagged during the biweekly documentation audit and downgraded.
3. **Hardware Honesty**: Driver entries in `docs/platform/HARDWARE_SUPPORT.md` cannot claim "Verified" without attached runtime execution evidence from `scripts/hardware/collect_diagnostics.sh` or a QEMU log.
4. **Safety Verification**: Destructive operations (partitioning, formatting, kernel upgrades) must always maintain a tested rollback path documented in `docs/operations/RECOVERY_RUNBOOK.md`.
