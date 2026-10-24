# SigmaOS Development & Study Process

## Governing Principle

A component is not “implemented” until its runtime path is exercised, its failure modes are tested, its documentation states its limits, and its status is updated from evidence—not intention.

This rule addresses the reality that passing unit tests alone does not prove bootability, installation, or hardware support.

---

## Periodic Open-Source Study Program

Rather than ad-hoc feature additions, SigmaOS follows a structured, evidence-driven study cycle.

### Cadence Table

| Cycle | Duration | Focus | Output |
| :--- | :--- | :--- | :--- |
| **Weekly study** | 1–2 hours | One narrow mechanism from Linux, FreeBSD, OpenBSD, Redox, xv6, or Arch | `docs/studies/YYYY-WW-<topic>.md` |
| **Biweekly gap review** | 2–3 hours | Compare one SigmaOS component against reference designs | Gap table + ranked PR proposals |
| **Monthly integration review** | Half day | Run boot, installer, recovery, and regression checks | Status update + blockers |
| **Quarterly architecture review** | 1–2 days | Reassess roadmap, security posture, ABI stability, release criteria | Revised roadmap + ADRs |

---

## Study Note Template

Each study note must be created under:
`docs/studies/YYYY-WW-<component>-<topic>.md`

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

## Component Rules Summary

SigmaOS enforces strict repository-wide component guidelines (see `COMPONENT_RULES.toml`):

1. **Status Honesty**: A component may not be marked complete without runtime or integration evidence.
2. **Test Minimum**: Every new public function requires at least one positive and one failure-path test.
3. **No Silent Failure**: Errors must be typed, logged where appropriate, and propagated or explicitly handled.
4. **No Unbounded Allocation**: Kernel and `no_std` paths must document allocation limits and OOM behavior.
5. **API Stability**: Public kernel/userspace APIs require an ADR and compatibility note.
6. **Security Review**: Privilege, syscall, filesystem, network, crypto, and driver changes require security review.
7. **Documentation**: Every component must document its purpose, interface, limitations, tests, and status.
8. **No Claim Without Evidence**: Documentation and status matrices must cite tests, logs, or issues.

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
<type>(<scope>): <summary>

<body explaining motivation and behavior change>

Refs: #<issue>
```

### Required Verification
Before submitting a PR, run:
- `cargo fmt --check`
- `cargo check --lib`
- `./run_sigma_tests.sh`
