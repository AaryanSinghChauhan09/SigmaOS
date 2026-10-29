# SigmaOS Development Process & Workflow

## Overview
SigmaOS enforces an evidence-driven study-and-gap-closing cycle. Passing unit tests alone does not prove bootability, installation, or hardware support. Every component is evaluated based on runtime evidence, failure mode testing, and explicit documentation of limitations.

## Recurring Study & Review Cadence
- **Weekly study (1–2 hours):** Focus on one narrow mechanism from Linux, FreeBSD, OpenBSD, Redox, xv6, or Arch. Output: `docs/studies/YYYY-WW-<topic>.md`.
- **Biweekly gap review (2–3 hours):** Compare one SigmaOS component against reference designs. Output: Gap table + ranked PR proposals.
- **Monthly integration review (Half day):** Run boot, installer, recovery, and regression checks. Output: Status update + blockers.
- **Quarterly architecture review (1–2 days):** Reassess roadmap, security posture, ABI stability, release criteria. Output: Revised roadmap + ADRs.

## Study Note Template
Each study note must be stored at `docs/studies/YYYY-WW-<component>-<topic>.md` and contain:
- Reference systems (Linux, FreeBSD/OpenBSD, Redox/xv6/other)
- Mechanism under study (Problem, Interface, Data structures, Concurrency model, Failure behavior)
- SigmaOS current state (Code location, Existing tests, Verified runtime behavior, Known gaps)
- Adaptation decision (Adopt, Adapt, Reject with justification)
- Proposed PRs & Acceptance evidence

## Branch & PR Conventions
- Branch naming: `feat/`, `fix/`, `docs/`, `study/`, `test/`, `chore/`, `security/`
- Commit message format: Conventional Commits (`type(scope): summary`)
- PR template: Enforces evidence checklist (`cargo check`, `cargo test`, `./run_sigma_tests.sh`, QEMU boot logs, security review, status impact).
