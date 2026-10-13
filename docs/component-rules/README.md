# SigmaOS Periodic Component-Rule System

## Overview

SigmaOS adopts a **periodic, PR-only component-rule system**. Every cycle studies one upstream OS mechanism, converts it into enforceable rules for one SigmaOS component, implements the smallest connected improvement in a feature branch, and merges only after CI, tests, runtime evidence, documentation, and review pass.

While the codebase contains extensive architecture models and standalone unit tests, code presence alone does not establish kernel integration, bootability, installation readiness, or hardware support. Claims must be established by connected runtime evidence.

---

## Periodic Governance Model

A fixed cadence ensures that upstream study always produces enforceable component rules and reviewable pull requests.

| Cadence | Activity | PR type | Required output |
|---|---|---|---|
| Daily | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| Monthly | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| Quarterly | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

Roadmap items remain proposals until their completion criteria pass; every cycle ends with evidence, not another planning document.

---

## Component Rule Lifecycle

Every SigmaOS component follows this 12-step lifecycle:

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

## Industry-Standard Rules

1. **Everything is a pull request:** No direct commits to `main`.
2. **One component, one outcome per PR:** Add one capability, enforce one rule, fix one regression, or update one doc area per PR.
3. **Study before implementation:** Major component changes require an upstream research note and ADR first.
4. **Rules must be testable:** Every rule must have a test, static check, CI gate, or runtime validation.
5. **Runtime evidence controls status:** Passing unit tests prove logic; QEMU boot, installer, update, package, and hardware tests prove the OS.
6. **Reproducibility:** Recorded commands, toolchain versions, exit codes, and artifact hashes.
7. **Least privilege:** Minimum required permissions for CI, agents, packages, and services.
8. **Security review is separate:** Privilege, syscall, memory, sandbox, crypto, network, and update changes require separate security review.
9. **Fail safely:** Rollback or recovery behavior required before merging core path changes.
10. **Regression tests are mandatory:** Bug fixes and newly enforced rules must include regression tests.
11. **License compliance first:** Dual-licensed MIT/GPL-2.0. Clean-room research or proper attribution required.
12. **No borrowed-status claims:** Reference projects do not imply SigmaOS parity without runtime evidence.
13. **Human approval for high-risk merges:** Bootloader, installer, update, security, and storage PRs require human review.
14. **Archive stale research:** Research older than two quarters must be reaffirmed or archived.

---

## Periodic Study Plan & Milestones

### 12-Week Weekly Study Focus

| Week | Study Focus | Component Improved | PR Deliverable |
|---|---|---|---|
| 1 | xv6/OSDev boot & interrupts | Kernel/boot | QEMU boot validator & boot trace |
| 2 | xv6 memory & Linux paging | Memory | Frame allocator & paging tests |
| 3 | xv6 processes & Linux scheduling | Scheduler | Minimal scheduler & process-switch test |
| 4 | Linux syscalls & VFS | Syscalls/VFS | Syscall table, path resolver, RAM-disk FS |
| 5 | Linux IPC & FreeBSD IPC | IPC | Pipes, signals, FD lifecycle tests |
| 6 | Arch/Debian packaging | Packaging | Metadata parser & dependency resolver |
| 7 | Fedora/Mint updates | Installer/update | Atomic update & rollback test |
| 8 | OpenBSD/FreeBSD security | Security | Capability denial tests & sandbox policy |
| 9 | Linux drivers & virtio | Drivers | virtio-blk, virtio-net, VGA tests |
| 10 | Redox/Rust OS design | Architecture | Unsafe-code policy & module boundaries |
| 11 | Linux networking | Network | Socket & basic network integration test |
| 12 | Mint/desktop onboarding | Zenith desktop | Graphical smoke test |

### 10-Month Component Milestones

| Month | Component Milestone | Promotion Target |
|---|---|---|
| 1 | QEMU x86_64 boot-to-shell | Kernel/boot: `Integrated` |
| 2 | Memory allocator & paging validation | Memory: `Integrated` |
| 3 | Scheduler & syscall foundation | Kernel: `Integrated` |
| 4 | VFS & RAM-disk filesystem | VFS: `Integrated` |
| 5 | Processes, signals, pipes | IPC: `Integrated` |
| 6 | Installer, update, rollback | Installer/update: `Integrated` |
| 7 | Capability model & sandbox | Security: `Hardened` |
| 8 | Virtio device support | Drivers: `Integrated` |
| 9 | Package metadata & dry-run | Packaging: `Integrated` |
| 10 | Minimal desktop session | Desktop: `Prototype` or `Integrated` |

---

## Definition of Done

A component rule or missing component is complete only when:

1. A study issue and ADR exist.
2. Component rules are written in `docs/component-rules/`.
3. Implementation is merged through a PR.
4. Unit and integration tests pass.
5. Connected to a real runtime path.
6. QEMU or hardware evidence exists.
7. A regression test prevents reversal.
8. Documentation and capability status are updated.
9. Security review is complete where relevant.
10. CI remains green after merge.
