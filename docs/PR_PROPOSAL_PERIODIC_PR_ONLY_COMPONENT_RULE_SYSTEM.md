# PR Proposal: Periodic, PR-Only Component-Rule System for SigmaOS

## Summary

SigmaOS adopts a **periodic, PR-only component-rule system** where every cycle studies one upstream OS mechanism (from Linux, FreeBSD, OpenBSD, Arch, Debian, Fedora, Mint, xv6, or Redox), converts it into enforceable rules for one SigmaOS component, implements the smallest connected improvement in a feature branch, and merges only after CI, unit/integration tests, QEMU runtime evidence, documentation, and security review pass.

While SigmaOS contains significant code and test suites, code presence and passing unit tests alone do not establish kernel integration, bootability, installation readiness, or hardware support. This proposal establishes the governance framework, component rules, periodic cadence, and PR workflow to ensure every roadmap claim is evidence-gated.

---

## Periodic Governance Model

SigmaOS operates under a fixed cadence so upstream study consistently yields enforceable component rules and reviewable pull requests:

| Cadence | Activity | PR type | Required output |
|---|---|---|---|
| **Daily** | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| **Weekly** | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| **Biweekly** | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| **Monthly** | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| **Quarterly** | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

---

## Component Rule Lifecycle

Every SigmaOS component must follow this 12-step loop:

```text
1. SELECT COMPONENT
2. MEASURE CURRENT CAPABILITY
3. SELECT UPSTREAM REFERENCE
4. STUDY ONE MECHANISM
5. WRITE COMPONENT RULES
6. OPEN RESEARCH/ADR PR
7. IMPLEMENT SMALLEST SAFE CHANGE
8. ADD TESTS AND RUNTIME VALIDATION
9. UPDATE DOCS AND CAPABILITY STATUS
10. OPEN REVIEW PR
11. MERGE ONLY AFTER GATES PASS
12. MONITOR FOR REGRESSIONS
```

A component rule is enforceable only when connected to code, tests, CI, documentation, and a runtime validation path.

---

## 14 Industry-Standard Rules

1. **Everything is a pull request**: No direct commits to `main`.
2. **One component, one outcome per PR**: Keep changes atomic and reviewable.
3. **Study before implementation**: Research note and ADR required prior to major component work.
4. **Rules must be testable**: Every rule must have a detecting check or test.
5. **Runtime evidence controls status**: Evidence in QEMU/hardware determines capability levels (`Proposed` -> `Prototype` -> `Integrated` -> `Hardened` -> `Supported`).
6. **Reproducibility**: Every PR must record build environment, commands, exit codes, and artifact hashes.
7. **Least privilege**: Agents, CI jobs, and processes receive minimum required permissions.
8. **Security review is separate**: Privileged operations, syscalls, sandboxing, and updates require explicit security review.
9. **Fail safely**: Rollback/recovery mechanisms must be defined for stateful operations.
10. **Regression tests are mandatory**: Every bug fix includes a regression test.
11. **License compliance first**: Dual-licensed MIT/GPL-2.0 compliance with clean-room attribution.
12. **No borrowed-status claims**: Upstream feature names do not imply SigmaOS parity without proof.
13. **Human approval for high-risk merges**: Bootloader, kernel, security, and installer PRs require human review.
14. **Archive stale research**: Research older than two quarters must be reaffirmed or archived.

---

## Definition of Done

A component rule or missing component is complete only when:
1. A study issue and ADR exist.
2. Component rules are written under `docs/component-rules/`.
3. Implementation is merged through a PR.
4. Unit and integration tests pass.
5. Connected to a real runtime path.
6. QEMU or hardware evidence exists.
7. A regression test prevents reversal.
8. Documentation and `CAPABILITY_MATRIX.toml` / `FEATURE_STATUS.toml` are updated.
9. Security review is completed where applicable.
10. CI remains green after merge.
