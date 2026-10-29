# PR Spec: SigmaOS Periodic PR-Only Component-Rule System

## Specification Overview

This specification formalizes the periodic, PR-only component-rule system for SigmaOS. It establishes the mandatory governance cadences, the 12-step component rule lifecycle, component-specific rules, the 12-week study plan, 10-month milestones, PR workflow rules, CI gates, and 14 industry-standard rules.

---

## 1. Governance Cadence Matrix

| Cadence | Activity | PR Type | Required Output |
|---|---|---|---|
| Daily | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| Monthly | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| Quarterly | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

---

## 2. 12-Step Component Lifecycle

1. **SELECT COMPONENT**: Identify the target component domain.
2. **MEASURE CURRENT CAPABILITY**: Audit existing implementation against `CAPABILITY_MATRIX.toml`.
3. **SELECT UPSTREAM REFERENCE**: Choose Linux, xv6, FreeBSD, OpenBSD, Arch, or Redox reference mechanism.
4. **STUDY ONE MECHANISM**: Narrow study to a single mechanism or interface contract.
5. **WRITE COMPONENT RULES**: Codify rules in `docs/component-rules/<component>.md`.
6. **OPEN RESEARCH/ADR PR**: Submit research note and Architecture Decision Record.
7. **IMPLEMENT SMALLEST SAFE CHANGE**: Build minimal connected implementation slice in feature branch.
8. **ADD TESTS AND RUNTIME VALIDATION**: Write unit, integration, and QEMU boot/runtime evidence.
9. **UPDATE DOCS AND CAPABILITY STATUS**: Update documentation, `CAPABILITY_MATRIX.toml`, and `FEATURE_STATUS.toml`.
10. **OPEN REVIEW PR**: Open review PR with complete evidence link.
11. **MERGE ONLY AFTER GATES PASS**: Require CI pass and CODEOWNERS review.
12. **MONITOR FOR REGRESSIONS**: Add regression checks to continuous test suites.

---

## 3. Component Domain Rules & Gates

| Domain | Rules File | Required Additional Gate |
|---|---|---|
| Kernel | `docs/component-rules/kernel.md` | QEMU boot smoke test |
| Memory | `docs/component-rules/memory.md` | Allocation, exhaustion, and fuzz tests |
| VFS/Filesystems | `docs/component-rules/vfs.md` | Filesystem conformance & QEMU RAM-disk test |
| IPC | `docs/component-rules/ipc.md` | Cross-process interoperability tests |
| Security | `docs/component-rules/security.md` | Denial tests and fuzzing |
| Drivers | `docs/component-rules/drivers.md` | QEMU device-specific test |
| Packaging | `docs/component-rules/packaging.md` | Metadata corpus and dry-run tests |
| Desktop | `docs/component-rules/desktop.md` | Graphical smoke test |
| Installer/Update | `docs/component-rules/installer_update.md` | Clean install and rollback test |

---

## 4. Verification Commands

All PRs must execute and pass:

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

---

## 5. 14 Industry-Standard Operating Rules

1. **Everything is a pull request**: Direct commits to `main` are prohibited.
2. **One component, one outcome per PR**: Single focused goal per PR.
3. **Study before implementation**: Require research note & ADR before major code changes.
4. **Rules must be testable**: Automated checks or test cases for every rule.
5. **Runtime evidence controls status**: Code presence alone does not establish OS capability.
6. **Reproducibility**: Environment, commands, and hashes recorded in reports.
7. **Least privilege**: Sandboxed execution and minimal credentials for jobs/agents.
8. **Security review is separate**: Dedicated security review for privilege boundaries.
9. **Fail safely**: Rollback/recovery mechanisms required for installer/update/kernel PRs.
10. **Regression tests are mandatory**: Bug fixes require regression test coverage.
11. **License compliance first**: Dual MIT/GPL-2.0 compliance with clean-room notes.
12. **No borrowed-status claims**: Upstream feature names do not imply SigmaOS parity.
13. **Human approval for high-risk merges**: Human review for kernel/installer/security merges.
14. **Archive stale research**: Research older than two quarters must be updated or archived.
