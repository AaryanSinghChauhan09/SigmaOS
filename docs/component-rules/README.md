# SigmaOS Component Rules

## Overview

SigmaOS components operate under a periodic PR-only component-rule system. Every component is assigned an agent owner, upstream references, mandatory operational rules, explicit prohibitions, required evidence, and promotion criteria.

## Periodic Governance Model

Use a fixed cadence so upstream study always produces enforceable component rules and reviewable pull requests.

| Cadence | Activity | PR type | Required output |
|---|---|---|---|
| Daily | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| Monthly | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| Quarterly | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

## Component Rule Lifecycle

Every component follows this 12-step lifecycle:

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

## 14 Industry-Standard PR Rules

1. **Everything is a pull request:** No direct commits to `main`.
2. **One component, one outcome per PR:** Single focused objective.
3. **Study before implementation:** Major changes require upstream study and ADR.
4. **Rules must be testable:** Every rule has tests or checks.
5. **Runtime evidence controls status:** Unit tests prove logic; boot/runtime tests prove the OS.
6. **Reproducibility:** Clean checkout reproducible builds with recorded commands and hashes.
7. **Least privilege:** Minimal permissions for CI, agents, and packages.
8. **Security review is separate:** Cryptography, sandboxing, and capabilities require dedicated review.
9. **Fail safely:** Rollback and recovery behaviors defined before merge.
10. **Regression tests are mandatory:** Bug fixes must include regression coverage.
11. **License compliance first:** Dual-license MIT/GPL-2.0 clean-room compliance.
12. **No borrowed-status claims:** Upstream capability does not imply SigmaOS capability.
13. **Human approval for high-risk merges:** Bootloader, installer, security, and release changes require human sign-off.
14. **Archive stale research:** Research older than two quarters must be reaffirmed or archived.

## Periodic Study Plan

### Weekly Study Schedule
- **Week 1:** xv6/OSDev boot and interrupts -> Kernel/boot (`chore/ci`, `docs/research`, `feat/kernel`)
- **Week 2:** xv6 memory and Linux paging -> Memory (`docs/research`, `feat/memory`, `test/memory`)
- **Week 3:** xv6 processes and Linux scheduling -> Scheduler (`docs/research`, `feat/kernel`, `test/kernel`)
- **Week 4:** Linux syscalls and VFS -> Syscalls/VFS (`docs/research`, `feat/vfs`, `feat/kernel`)
- **Week 5:** Linux IPC and FreeBSD IPC -> IPC (`docs/research`, `feat/ipc`, `test/ipc`)
- **Week 6:** Arch/Debian packaging -> Packaging (`docs/research`, `feat/pkg`, `test/pkg`)
- **Week 7:** Fedora/Mint updates -> Installer/update (`docs/research`, `feat/installer`, `test/update`)
- **Week 8:** OpenBSD/FreeBSD security -> Security (`docs/research`, `security/policy`, `test/security`)
- **Week 9:** Linux drivers and virtio -> Drivers (`docs/research`, `feat/drivers`, `test/virtio`)
- **Week 10:** Redox/Rust OS design -> Architecture (`docs/research`, `refactor/core`, `chore/safety`)
- **Week 11:** Linux networking -> Network (`docs/research`, `feat/net`, `test/net`)
- **Week 12:** Mint/desktop onboarding -> Zenith desktop (`docs/research`, `feat/desktop`, `test/ui`)

## Monthly Milestones

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

## Definition of Done

1. A study issue and ADR exist.
2. Component rules are written in `docs/component-rules/`.
3. Implementation is merged through a PR.
4. Unit and integration tests pass.
5. The component is connected to a real runtime path.
6. QEMU or hardware evidence exists.
7. A regression test prevents reversal.
8. Documentation and capability status are updated.
9. Security review is complete where relevant.
10. CI remains green after merge.

## Component Rule Index

- [Kernel Rules](kernel.md)
- [Memory Rules](memory.md)
- [VFS / Filesystem Rules](vfs.md)
- [IPC Rules](ipc.md)
- [Security Rules](security.md)
- [Driver Rules](drivers.md)
- [Packaging Rules](packaging.md)
- [Desktop Rules](desktop.md)
- [Installer and Update Rules](installer_update.md)
