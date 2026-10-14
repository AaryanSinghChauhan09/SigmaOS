# SigmaOS Evidence-Based Open-Source Study & Gap-Closing Process

## Governing Principle
A component is not “implemented” until its runtime path is exercised, its failure modes are tested, its documentation states its limits, and its status is updated from evidence—not intention.

Code presence and unit test execution alone do not guarantee kernel integration, bootability, or hardware support.

---

## Recurring Cadence

| Cycle | Duration | Focus | Output |
| :--- | :--- | :--- | :--- |
| **Weekly study** | 1–2 hours | One narrow mechanism from Linux, FreeBSD, OpenBSD, Redox, xv6, or Arch | `docs/studies/YYYY-WW-<topic>.md` |
| **Biweekly gap review** | 2–3 hours | Compare one SigmaOS component against reference designs | Gap table + ranked PR proposals |
| **Monthly integration review** | Half day | Run boot, installer, recovery, and regression checks | Status update + blockers |
| **Quarterly architecture review** | 1–2 days | Reassess roadmap, security posture, ABI stability, release criteria | Revised roadmap + ADRs |

---

## Open-Source Study Template
Each study note must live at `docs/studies/YYYY-WW-<component>-<topic>.md` and follow the standardized section template:

```markdown
# Study: <Topic>

## Reference systems
- Linux:
- FreeBSD/OpenBSD:
- Redox/xv6/other:

## Mechanism under study
- Problem:
- Interface:
- Data structures:
- Concurrency model:
- Failure behavior:

## SigmaOS current state
- Code location:
- Existing tests:
- Verified runtime behavior:
- Known gaps:

## Adaptation decision
- Adopt:
- Adapt:
- Reject:
- Reason:

## Proposed PRs
- [ ] PR 1:
- [ ] PR 2:

## Acceptance evidence
- [ ] `cargo check --lib`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] QEMU/boot or integration evidence, if applicable
```

---

## Branch & Commit Workflow

### Branch Naming Conventions
- `feat/<component>-<short-description>`
- `fix/<component>-<short-description>`
- `docs/<component>-<short-description>`
- `study/<component>-<short-description>`
- `test/<component>-<short-description>`
- `chore/<component>-<short-description>`
- `security/<component>-<short-description>`

### Commit Message Format
```text
type(scope): summary

body explaining motivation and behavior change

Refs: #issue
BREAKING CHANGE: description, if applicable
```
