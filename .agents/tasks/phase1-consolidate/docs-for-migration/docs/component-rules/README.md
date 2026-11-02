# SigmaOS Periodic Component-Rule System

## Overview

SigmaOS adopts a **periodic, PR-only component-rule system**: every cycle studies one upstream OS mechanism, converts it into enforceable rules for one SigmaOS component, implements the smallest connected improvement in a feature branch, and merges only after CI, tests, runtime evidence, documentation, and review pass.

The repository's README is explicit that code presence and unit tests do not establish kernel integration, bootability, installation readiness, or hardware support. Roadmap items remain proposals until their completion criteria pass; therefore, every cycle must end with evidence, not another planning document.

## Periodic Governance Model

A fixed cadence ensures upstream study produces enforceable component rules and reviewable pull requests.

| Cadence | Activity | PR type | Required output |
|---|---|---|---|
| Daily | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| Monthly | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| Quarterly | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

## Component Rule Lifecycle

Every SigmaOS component follows this 12-step improvement loop:

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

A component rule is enforceable only when it is connected to code, tests, CI, documentation, and a runtime validation path.

## 14 Industry-Standard PR Rules

1. **Rule 1: Everything is a pull request** - No direct commits to `main`. Research, rules, implementation, tests, documentation, CI, and status changes must arrive through PRs.
2. **Rule 2: One component, one outcome per PR** - A PR may add one component capability, enforce one rule, fix one regression, or update one documentation area.
3. **Rule 3: Study before implementation** - A component rule or major component change requires an upstream study note and ADR first.
4. **Rule 4: Rules must be testable** - Every component rule must have at least one unit test, integration test, static check, CI gate, or runtime validation that detects violation.
5. **Rule 5: Runtime evidence controls status** - Unit tests prove logic; QEMU boot, installer, update, package, and hardware tests prove the OS. Unit tests and code presence do not establish kernel integration or hardware support.
6. **Rule 6: Reproducibility** - Every PR must be reproducible from a clean checkout with recorded commands, toolchain versions, container image, timestamps, exit codes, and artifact hashes.
7. **Rule 7: Least privilege** - Agents, CI jobs, packages, and services receive only minimum required permissions. No broad credentials, unrestricted network access, host filesystem access, or unsandboxed package-script execution.
8. **Rule 8: Security review is separate** - Capability enforcement, sandboxing, syscalls, memory safety, cryptography, package execution, networking, and update mechanisms require dedicated security review.
9. **Rule 9: Fail safely** - Boot, installer, update, filesystem, and storage PRs must define rollback or recovery behavior before merge.
10. **Rule 10: Regression tests are mandatory** - Every bug fix and newly enforced rule must include a test that would fail if the behavior regressed.
11. **Rule 11: License compliance first** - SigmaOS is dual-licensed MIT/GPL-2.0. Copied upstream code requires license compatibility and attribution; otherwise use clean-room research notes and original implementation.
12. **Rule 12: No borrowed-status claims** - A feature in Linux, FreeBSD, OpenBSD, Arch, Fedora, or Redox does not imply the same feature exists in SigmaOS.
13. **Rule 13: Human approval for high-risk merges** - Humans approve bootloader, installer, update, security-boundary, package-execution, release, and destructive-storage PRs.
14. **Rule 14: Archive stale research** - Research older than two quarters must be reaffirmed, updated, or archived. It cannot override current component status.

## Component Rules Index

- [VFS / Filesystems](vfs.md)
- [Kernel](kernel.md)
- [Memory](memory.md)
- [IPC](ipc.md)
- [Security](security.md)
- [Drivers](drivers.md)
- [Packaging](packaging.md)
- [Desktop](desktop.md)
- [Installer / Update](installer_update.md)
