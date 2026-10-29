# SigmaOS Role-Based Governance & Operating Model

## Overview
SigmaOS uses a structured role-based governance model operating across 12 maintenance roles. Each role can be filled by a human maintainer, an AI agent, or a human-supervised AI agent.

---

## 1. 12 Maintenance Roles

| Role | Mission & Primary Scope | Cadence | Leadership Profile |
|---|---|---|---|
| **Maintainer** | Owns subsystem health, code review, backlog grooming, and merge approval | Daily / Weekly | Human-Led, Agent-Assisted |
| **Updater** | Scans and updates dependencies, Rust toolchain, and upstream advisories | Daily / Weekly | Mostly Agent |
| **Reviewer** | Audits all incoming changes for correctness, safety, tests, and style | Per PR | Agent First Pass, Human Approval |
| **Consolidator** | Fights sprawl by removing dead code, duplicate docs, and stale flags | Weekly / Monthly | Mostly Agent |
| **Researcher** | Studies Linux/BSD upstream designs, writes one-page design notes | Weekly / Monthly | Agent Drafts, Human Decides |
| **Triager** | Sorts issues, reproduces bugs, deduplicates, and assigns severity | Daily | Mostly Agent |
| **Tester / Fuzzer** | Executes test suites, continuous fuzzing, and hardware boot checks | Daily / Nightly | Agent Automated, Human Hardware |
| **Security Officer** | Audits `unsafe` Rust blocks, syscall surfaces, key management, and CVEs | Daily / Monthly | Human-Led |
| **Release Manager** | Builds ISOs, validates reproducible builds, and publishes releases | Per Release | Human-Led, Agent-Assisted |
| **Docs Steward** | Keeps documentation and status reports synchronized with CI evidence | Weekly | Agent Drafts, Human Decides |
| **Benchmarker** | Runs scoreboard benchmarks against Debian, Arch, Alpine, FreeBSD, OpenBSD | Weekly / Monthly | Mostly Agent |
| **Community Manager**| Manages contributor onboarding, issues, and release communications | Daily / Monthly | Human-Led |

---

## 2. Workflows Between Roles
1. **Researcher** studies reference implementations and writes a 1-page design note with acceptance criteria.
2. **Triager** labels and prioritizes resulting issues.
3. **Maintainer** assigns tasks to contributors or AI agents to create PRs.
4. **Reviewer** executes Stage 1 (automated checks) and Stage 2 (code review checklist).
5. **Tester/Fuzzer** and **Benchmarker** confirm test passes and benchmark stability.
6. **Maintainer** (Human or Human-approved) merges the PR.
7. **Docs Steward** updates documentation and **Consolidator** removes redundant artifacts.
8. **Release Manager** tags releases and **Security Officer** verifies attestations.

---

## 3. Weekly Operational Calendar
- **Monday:** Triage sweep, issue labeling, sprint planning, and upstream advisory digest (*Triager, Updater, Maintainer*).
- **Tuesday–Wednesday:** Subsystem development, PR code reviews, design note publication (*Maintainer, Reviewer, Researcher*).
- **Thursday:** Consolidation sweep, dead code cleanup, doc sync, benchmark execution (*Consolidator, Docs Steward, Benchmarker*).
- **Friday:** Release candidate verification, weekly health report generation, retrospective (*Release Manager, Tester/Fuzzer, Community Manager*).

---

## 4. 7 Governance Guardrails
1. **Human Approval Required:** Human maintainers must approve any changes touching kernel code, cryptography, bootloaders, disk installers, or signing keys.
2. **Evidence-Gated Success:** No feature or task may be marked complete without a linked CI artifact (QEMU boot log, test run output).
3. **Sandboxing & Least Privilege:** AI agents operate under scoped access tokens with no direct push privileges to `main` and zero access to release keys.
4. **Small, Reviewable PRs:** All changes must be delivered in atomic, test-backed pull requests with daily rate caps.
5. **Regression Ratchet:** Every bug fix must include an automated regression test; code coverage and `unsafe` counts cannot degrade without human override.
6. **Audit Log & Rollback:** Every agent action is logged in machine-readable format, supporting single-command automated rollback.
7. **Honest Public Claims:** Public READMEs and documentation must strictly reflect features verified on bare-metal or QEMU today.
