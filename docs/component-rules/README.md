# SigmaOS Periodic, PR-Only Component-Rule Governance System

SigmaOS adopts a **periodic, PR-only component-rule system**: every cycle studies one upstream OS mechanism, converts it into enforceable rules for one SigmaOS component, implements the smallest connected improvement in a feature branch, and merges only after CI, tests, runtime evidence, documentation, and review pass.

The repository already has the right structure (`AGENTS.md`, component agent guides, capability/status inventories, tests, scripts, installer assets, and comparative gap analysis), but code presence and unit tests alone do not establish kernel integration, bootability, installation readiness, or hardware support.

---

## Periodic Governance Model

Use a fixed cadence so upstream study always produces enforceable component rules and reviewable pull requests.

| Cadence | Activity | PR type | Required output |
|---|---|---|---|
| Daily | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| Monthly | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| Quarterly | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

Roadmap items remain proposals until their completion criteria pass; therefore, every cycle must end with evidence, not another planning document.

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

A component rule is enforceable only when it is connected to code, tests, CI, documentation, and a runtime validation path.

---

## Component Promotion Criteria

Each component rule file defines promotion requirements across 5 levels:

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |

---

## 12-Week Periodic Study Plan

Study one narrow mechanism per week, not an entire project:

| Week | Study focus | Component improved | PR deliverable |
|---|---|---|---|
| 1 | xv6/OSDev boot and interrupts | Kernel/boot | QEMU boot validator and boot trace |
| 2 | xv6 memory and Linux paging | Memory | Frame allocator and paging tests |
| 3 | xv6 processes and Linux scheduling | Scheduler | Minimal scheduler and process-switch test |
| 4 | Linux syscalls and VFS | Syscalls/VFS | Syscall table, path resolver, RAM-disk FS |
| 5 | Linux IPC and FreeBSD IPC | IPC | Pipes, signals, FD lifecycle tests |
| 6 | Arch/Debian packaging | Packaging | Metadata parser and dependency resolver |
| 7 | Fedora/Mint updates | Installer/update | Atomic update and rollback test |
| 8 | OpenBSD/FreeBSD security | Security | Capability denial tests and sandbox policy |
| 9 | Linux drivers and virtio | Drivers | virtio-blk, virtio-net, VGA tests |
| 10 | Redox/Rust OS design | Architecture | Unsafe-code policy and module boundaries |
| 11 | Linux networking | Network | Socket and basic network integration test |
| 12 | Mint/desktop onboarding | Zenith desktop | Graphical smoke test |

---

## 10-Month Component Milestones

Each month completes one component milestone:

| Month | Component milestone | Promotion target |
|---|---|---|
| 1 | QEMU x86_64 boot-to-shell | Kernel/boot: `Integrated` |
| 2 | Memory allocator and paging validation | Memory: `Integrated` |
| 3 | Scheduler and syscall foundation | Kernel: `Integrated` |
| 4 | VFS and RAM-disk filesystem | VFS: `Integrated` |
| 5 | Processes, signals, pipes | IPC: `Integrated` |
| 6 | Installer, update, rollback | Installer/update: `Integrated` |
| 7 | Capability model and sandbox | Security: `Hardened` |
| 8 | Virtio device support | Drivers: `Integrated` |
| 9 | Package metadata and dry-run | Packaging: `Integrated` |
| 10 | Minimal desktop session | Desktop: `Prototype` or `Integrated` |

---

## 14 Industry-Standard Rules

1. **Everything is a pull request:** No direct commits to `main`.
2. **One component, one outcome per PR:** Scope each PR to a single capability or rule.
3. **Study before implementation:** Major changes require an upstream study note and ADR first.
4. **Rules must be testable:** Every component rule must have automated verification.
5. **Runtime evidence controls status:** Passing unit tests do not establish OS integration.
6. **Reproducibility:** PRs must be reproducible from a clean checkout.
7. **Least privilege:** Components and scripts receive only minimum required capabilities.
8. **Security review is separate:** Privilege boundaries require dedicated security review.
9. **Fail safely:** System changes must define rollback and recovery behavior.
10. **Regression tests are mandatory:** Bug fixes require regression tests.
11. **License compliance first:** Dual MIT/GPL-2.0 compliance and clean-room implementation.
12. **No borrowed-status claims:** Upstream reference projects do not grant automatic parity.
13. **Human approval for high-risk merges:** Boot, installer, security, and update PRs require human review.
14. **Archive stale research:** Research older than two quarters must be updated or archived.

---

## Component Rules Index

- [VFS / Filesystem Rules](vfs.md)
- [Kernel Rules](kernel.md)
- [Memory Rules](memory.md)
- [IPC Rules](ipc.md)
- [Security Rules](security.md)
- [Driver Rules](drivers.md)
- [Packaging Rules](packaging.md)
- [Desktop Rules](desktop.md)
- [Installer & Update Rules](installer_update.md)
