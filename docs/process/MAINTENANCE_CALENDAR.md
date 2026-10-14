# SigmaOS Maintenance Calendar & Governance Model

## Overview

SigmaOS operates on a recurring maintenance cadence. Every task produces a reviewable artifact (PR, issue, report, test result, or documented decision). No task is considered "done" until its outcome is visible in the repository.

## Task Cadence Summary

| Frequency | Task | Owner Role | Required PR/Output |
|---|---|---|---|
| Daily | CI health triage | Build/CI maintainer | Fix PR or triage issue |
| Weekly | Dependency and toolchain review | Release engineer | `chore(deps)` PR |
| Weekly | Security advisory review | Security maintainer | `security(...)` PR or triage issue |
| Weekly | Open-source study | Component maintainer | `docs/studies/...` PR |
| Biweekly | Component gap review | Component maintainer | Gap issue + scoped feature PR |
| Biweekly | Documentation accuracy audit | Docs maintainer | `docs(...)` PR |
| Monthly | Boot and smoke-test validation | Platform maintainer | Boot report + fixes |
| Monthly | Fuzzing and regression review | Testing maintainer | Fuzz findings + fixes |
| Monthly | Installer/recovery rehearsal | Installer maintainer | VM install report + fixes |
| Monthly | Package/update rehearsal | Packaging maintainer | Package transaction report |
| Quarterly | Release readiness review | Release manager | Release checklist PR |
| Quarterly | Architecture and roadmap review | Tech leads | ADR/roadmap PR |
| Quarterly | Hardware support audit | Driver maintainer | Hardware matrix PR |
| Semiannual | License, provenance, & supply chain audit | Compliance owner | SBOM/provenance PR |

## Roles and Responsibilities

- **Build/CI Maintainer**: Monitors CI status on PRs and nightly builds. Ensures main branch remains green.
- **Release Engineer**: Manages Rust toolchain versions, lockfile freshness, dependency safety, and release prep scripts.
- **Security Maintainer**: Triage CVE feeds, RustSec advisories, and upstream kernel security reports.
- **Component Maintainers**: Lead subsystem gap reviews, open-source studies, and targeted capability PRs.
- **Platform Maintainer**: Executes monthly QEMU boot-to-shell validation and updates `docs/platform/BOOT_MATRIX.md`.
- **Installer Maintainer**: Rehearses VM installation, failure injection, dry-run mode, and recovery runbooks.
- **Packaging Maintainer**: Validates atomic update transactions, package manager solver, and rollback flows.
- **Tech Leads**: Author ADRs in `docs/adr/`, align roadmap items with acceptance criteria, and conduct release readiness audits.

## Escalation Rules & Release Blockers

1. **Red Main Branch**: Any failing job on `main` is a P0 release blocker. The Build/CI maintainer opens a triage issue immediately and assigns an owner.
2. **Boot Failure**: If monthly QEMU boot validation fails, all feature merges for affected subsystems are frozen until fixed.
3. **Installer Mutation Failure**: Non-atomic or unrecoverable installer failures block ISO image release candidates.
4. **Unreviewed Security Vulnerability**: Unaddressed critical CVE/RustSec advisories in dependencies halt release tagging.
