# SigmaOS Engineering Process & Operating Model

## Governing Principle
A component is not “implemented” until its runtime path is exercised, its failure modes are tested, its documentation states its limits, and its status is updated from evidence—not intention.

---

## Periodic Open-Source Study Program

SigmaOS utilizes a recurring, evidence-driven study cycle rather than ad-hoc feature additions. Design references include Linux, Arch, Mint, Redox, xv6, FreeBSD, and OpenBSD.

### Study Cadence
| Cycle | Duration | Focus | Output |
| :--- | :--- | :--- | :--- |
| **Weekly Study** | 1–2 hours | One narrow mechanism from Linux, FreeBSD, OpenBSD, Redox, xv6, or Arch | `docs/studies/YYYY-WW-<topic>.md` |
| **Biweekly Gap Review** | 2–3 hours | Compare one SigmaOS component against reference designs | Gap table + ranked PR proposals |
| **Monthly Integration Review** | Half day | Run boot, installer, recovery, and regression checks | Status update + blockers |
| **Quarterly Architecture Review** | 1–2 days | Reassess roadmap, security posture, ABI stability, release criteria | Revised roadmap + ADRs |

---

## Study Template

Each study note must live at `docs/studies/YYYY-WW-<component>-<topic>.md` and include:

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

## Pull Request Workflow

### Branch Naming
- `feat/<component>-<short-description>`
- `fix/<component>-<short-description>`
- `docs/<component>-<short-description>`
- `study/<component>-<short-description>`
- `test/<component>-<short-description>`
- `chore/<component>-<short-description>`
- `security/<component>-<short-description>`

### Commit Format (Conventional Commits)
```text
type(scope): summary

body explaining motivation and behavior change

Refs: #issue
BREAKING CHANGE: description, if applicable
```

---

## Repository Engineering Rules

1. **Code Quality**:
   - Mandatory formatting (`cargo fmt --check`).
   - Static analysis check (`cargo check --lib`).
   - All `unsafe` blocks require justification comments and invariant lists.
   - Small modules with explicit boundaries.

2. **Testing**:
   - Every PR includes tests or explicit written exemption.
   - Unit tests cover normal, boundary, invalid-input, and failure paths.
   - CI distinguishes unit-test success from boot/runtime success.

3. **Security**:
   - Privileged operations declare required capabilities.
   - Input validation on all external boundaries.
   - Fail closed for security-sensitive operations.

4. **Reliability**:
   - Long-running operations support timeout, cancellation, and retry.
   - Destructive operations require confirmation, dry-run, logging, and rollback.
