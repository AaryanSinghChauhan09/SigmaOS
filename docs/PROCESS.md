# SigmaOS Development & Governance Process

## Overview

SigmaOS strictly enforces a **periodic, PR-only component-rule system**. Code presence and unit tests alone do not establish kernel integration, bootability, installation readiness, or hardware support.

## Periodic Governance Model

| Cadence | Activity | PR type | Required output |
|---|---|---|---|
| Daily | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| Monthly | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| Quarterly | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

## Component Rule Loop

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

## 14 Industry-Standard Rules

1. **Everything is a pull request:** No direct commits to `main`.
2. **One component, one outcome per PR:** Keep scope narrow and atomic.
3. **Study before implementation:** An upstream study note and ADR are required prior to major implementation.
4. **Rules must be testable:** Automated unit, integration, or static check must detect rule violations.
5. **Runtime evidence controls status:** Status promotion requires QEMU/runtime logs under `reports/`.
6. **Reproducibility:** Recorded commands, toolchain versions, exit codes, and artifact hashes.
7. **Least privilege:** Sandboxed scripts and minimal capability access.
8. **Security review is separate:** Required for syscall, crypto, capability, and update changes.
9. **Fail safely:** Defined rollback and atomic error recovery.
10. **Regression tests are mandatory:** Bug fixes must include tests preventing regression.
11. **License compliance first:** Dual MIT/GPL-2.0 compliance and clean-room research.
12. **No borrowed-status claims:** Upstream reference feature names do not imply SigmaOS parity without evidence.
13. **Human approval for high-risk merges:** Bootloader, installer, security, and storage PRs require human review.
14. **Archive stale research:** Research older than two quarters must be updated or archived.
