# SigmaOS Engineering & Open-Source Study Process

## Governing Principle

A component is not "implemented" until its runtime path is exercised, its failure modes are tested, its documentation states its limits, and its status is updated from evidence—not intention.

Code presence and unit test pass rates alone are not equivalent to kernel integration or verified hardware/platform support.

---

## Periodic Open-Source Study Program

Rather than ad-hoc feature copying, SigmaOS maintains a structured, recurring study-and-gap-closing process.

### Cadence

| Cycle | Duration | Focus | Output |
| :--- | :--- | :--- | :--- |
| **Weekly study** | 1–2 hours | One narrow mechanism from Linux, FreeBSD, OpenBSD, Redox, xv6, or Arch | `docs/studies/YYYY-WW-<topic>.md` |
| **Biweekly gap review** | 2–3 hours | Compare one SigmaOS component against reference designs | Gap table + ranked PR proposals |
| **Monthly integration review** | Half day | Run boot, installer, recovery, and regression checks | Status update + blockers |
| **Quarterly architecture review** | 1–2 days | Reassess roadmap, security posture, ABI stability, release criteria | Revised roadmap + ADRs |

---

## Study Template

Every study note must be stored at:
`docs/studies/YYYY-WW-<component>-<topic>.md`

### Required Sections

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

Note: Study outputs must explicitly categorize decisions into **Adopt**, **Adapt**, or **Reject**. Reference OS designs serve as design references rather than absolute parity targets.

---

## Repository-Wide Engineering Rules

1. **Code Quality**:
   - Rust formatting is mandatory (`cargo fmt --check`).
   - Static analysis must pass (`cargo check --lib` and Clippy).
   - Unsafe blocks require justification comments, safety invariant lists, and explicitly reviewed boundaries.
   - Public APIs require documentation comments and usage examples.

2. **Testing**:
   - Every PR includes tests or a written justification for why tests are not applicable.
   - Unit tests cover normal, boundary, invalid-input, and failure paths.
   - Fuzz targets exist for parsers, network packets, filesystem metadata, and syscall decoding.
   - CI must distinguish unit-test pass rates from verified runtime/boot execution.

3. **Security**:
   - All external inputs (disk metadata, network packets, ELF files, USB descriptors, package specs) must be strictly validated.
   - Principle of least privilege applied by default across all syscalls and capability boundaries.
   - Fail closed on security-sensitive operations.

4. **Reliability**:
   - Operations with external dependencies specify timeout, retry, and cancellation semantics.
   - Destructive disk operations enforce confirmation, dry-run modes, and recovery paths.
   - Unbounded memory allocations in kernel/no_std paths are prohibited.

---

## Pull Request Workflow

### Branch Naming Conventions
- `feat/<component>-<short-description>`
- `fix/<component>-<short-description>`
- `docs/<component>-<short-description>`
- `study/<component>-<short-description>`
- `test/<component>-<short-description>`
- `chore/<component>-<short-description>`
- `security/<component>-<short-description>`

### Commit Message Format
Use Conventional Commits format:
```text
<type>(<scope>): <summary>

<body>

Refs: #<issue-id>
```

### PR Title Format
`<type>(<scope>): <imperative summary>`

Example: `feat(boot): add reproducible QEMU boot smoke test`
