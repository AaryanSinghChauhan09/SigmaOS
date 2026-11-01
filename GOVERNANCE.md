# SigmaOS Governance Model & Operating Rules

## Overview
SigmaOS employs a Role-Based Governance model designed for seamless collaboration between human maintainers and autonomous AI maintenance agents.

## Maintenance Roles
1. **Maintainers:** Subsystem authority, architecture integrity, and code merging.
2. **Updater:** Dependency updates, CVE tracking, and upstream digests.
3. **Reviewer:** Code inspection, correctness verification, and lint validation.
4. **Consolidator:** Codebase pruning, deduplication, and dead-code removal.
5. **Researcher:** Reference OS analysis (Linux, FreeBSD, OpenBSD, Redox) and design notes.
6. **Triager:** Issue categorization, reproduction, and priority assignment.
7. **Tester/Fuzzer:** Automated test suites, fuzz campaigns, and hardware regression tracking.
8. **Security Officer:** Unsafe block audits, disclosure processes, and signing key custody.
9. **Release Manager:** Release branch stabilization, ISO builds, and changelog verification.
10. **Docs Steward:** Truth-in-documentation maintenance and README accuracy.
11. **Benchmarker:** Comparative OS performance benchmarking and regression alerts.
12. **Community Manager:** Contributor onboarding, RFC coordination, and community communication.

## Bootstrap Phase
The initial bootstrap phase activates four core roles:
- **Triager**
- **Reviewer**
- **Updater**
- **Tester/Fuzzer**

## Weekly Operational Calendar
- **Monday:** Triage sweep, planning, and upstream advisory digest.
- **Tuesday–Wednesday:** Active subsystem development and code review.
- **Thursday:** Consolidation sweep, doc sync, and benchmark performance run.
- **Friday:** Release candidate verification, weekly health report, and retrospective.

## Mandatory Operating Guardrails
1. **Human Approval Gate:** Kernel, crypto, bootloader, installer, and keys require human maintainer signoff.
2. **Evidence-Gated Claims:** Capabilities require linked CI execution artifacts to be marked complete.
3. **Least Privilege:** Agents run with scoped tokens, no signing keys, and no direct pushes to `main`.
4. **Small Scoped PRs:** Incremental PRs with tests; daily PR limits enforced.
5. **Regression Ratchet:** Bug fixes require tests; coverage and unsafe counts cannot regress.
6. **Audit & Rollback:** Complete audit log of automated actions with one-command rollback.
7. **Public Honesty:** Public documentation states only what is proven to run on hardware today.
