# PR Proposal: Periodic PR-Only Research-and-Implementation Pipeline

## Abstract

SigmaOS adds missing components through a **periodic, PR-only research-and-implementation pipeline**: each cycle studies one upstream OS mechanism, opens a tracking issue, implements the smallest connected component slice in a feature branch, validates it with the repository’s documented checks and QEMU/runtime tests, and merges only through a reviewable pull request. The README explicitly says roadmap proposals become completed work only after their code path and stated checks are verified, and that reference-project feature names do not establish SigmaOS parity.

## PR-Only Operating Model

Every study, design, implementation, test, documentation, and status change must be represented by a pull request.

```text
PERIODIC STUDY CYCLE
  → Open study issue
  → Open research/ADR PR
  → Open implementation PR
  → Open test/evidence PR
  → Open documentation/status PR
  → CI + review + merge
  → Close issue with linked PRs
```

For small changes, combine research, implementation, tests, and documentation into one PR. For kernel, security, installer, bootloader, update, or package-execution work, separate them into reviewable PRs.

## Periodic Cadence

| Cadence | Activity | PR type | Required evidence |
|---|---|---|---|
| Daily | Build/test/boot health | `chore/ci`, `fix/test`, `docs/status` | CI logs, test reports |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, tests |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Runtime regression report |
| Monthly | Add or complete one missing component | `feat/<component>` | QEMU/runtime validation |
| Quarterly | Roadmap and architecture review | `docs/roadmap`, `refactor/architecture` | Gap register and updated priorities |

## Missing-Component Roadmap

Implement missing components in this order. Each item must be delivered through one or more PRs and must produce runtime evidence.

| Priority | Missing component | Upstream study | First PR deliverable | Validation |
|---|---|---|---|---|
| P0 | Reproducible CI/build gate | Fedora, Debian | CI workflow and build manifest | Clean build and tests pass |
| P0 | QEMU boot validator | OSDev, xv6 | Boot script and boot matrix | QEMU boot-to-shell log |
| P0 | Bootloader integration | OSDev, GRUB/systemd-boot concepts | UEFI boot path | QEMU UEFI boot |
| P0 | Kernel entry and serial logging | xv6, Linux | Early console and kernel entry | Serial trace |
| P1 | Physical memory manager | xv6, Linux | Frame allocator | Allocation tests |
| P1 | Virtual memory/paging | xv6, Linux | Page tables and address spaces | Boot and memory tests |
| P1 | Interrupts and timer | xv6, OSDev | IDT, exception handlers, timer | Interrupt trace |
| P1 | Scheduler | Linux, xv6 | Minimal round-robin scheduler | Process-switch test |
| P1 | Syscall interface | Linux, FreeBSD | Stable syscall table and validation | Syscall tests |
| P1 | Init and shell | Linux Mint, Arch | PID 1, service startup, shell | Boot-to-shell |
| P2 | VFS | Linux, xv6 | Path resolution, FDs, file ops | Filesystem tests |
| P2 | RAM-disk filesystem | Linux tmpfs concepts | Simple in-memory FS | QEMU RAM-disk boot |
| P2 | Processes and signals | Linux, xv6 | Process lifecycle and signals | Interop tests |
| P2 | Pipes and IPC | Linux, FreeBSD | Pipes, event queues, shared memory | IPC tests |
| P2 | Installer | Debian, Mint | Clean QEMU install | Install log |
| P2 | Atomic update/rollback | Fedora, Mint | Update manifest and rollback | Failed-update recovery test |
| P2 | Security capabilities | OpenBSD, Linux | Capability checks and sandbox policy | Denial tests |
| P3 | Drivers | Linux, FreeBSD | virtio-blk, virtio-net, VGA | Device logs |
| P3 | Networking | Linux, FreeBSD | Socket API and basic TCP/IP path | Network integration test |
| P3 | Desktop | Redox, Mint, Omarchy | Framebuffer, input, window launch | Graphical smoke test |
| P3 | Package manager | Arch, Debian | Metadata parser and dependency resolver | Package corpus tests |
| P3 | Distro compatibility | Arch, Debian, Fedora, BSD | Metadata compatibility layer | Compatibility matrix |

## Branch and PR Conventions

### Branch Naming

```text
study/<component>-<topic>-<short-id>
feat/<component>-<capability>-<short-id>
fix/<component>-<issue>-<short-id>
test/<component>-<scenario>-<short-id>
docs/<component>-<topic>-<short-id>
chore/ci-<topic>-<short-id>
security/<component>-<topic>-<short-id>
```

### Commit Convention

Use Conventional Commits:

```text
feat(kernel): add QEMU x86_64 boot-to-shell validator
fix(vfs): normalize path-resolution error codes
test(memory): add frame allocator exhaustion tests
docs(research): study Linux VFS path resolution
chore(ci): require clean build and runtime smoke test
security(package): sandbox package-script execution
```

### PR Title Format

```text
<type>(<component>): <concise change>
```

## Required PR Types

1. **Research PR** (`study/<component>-<topic>-<id>`)
2. **Implementation PR** (`feat/<component>-<capability>-<id>`)
3. **Test and evidence PR** (`test/<component>-<scenario>-<id>`)
4. **Documentation and status PR** (`docs/<component>-<topic>-<id>`)

## 12 PR Review Rules

1. Every change is a PR
2. One PR, one outcome
3. Research before implementation
4. Evidence before status
5. Runtime path required
6. Security review is separate
7. Least privilege
8. Reproducibility
9. Rollback safety
10. License compliance
11. No parity claims
12. Human approval for high-risk merges

## First Six PR Deliverables

- **PR 1:** `chore(ci): add reproducible verification workflow`
- **PR 2:** `docs(research): add periodic OS study framework`
- **PR 3:** `feat(kernel): add QEMU x86_64 boot validator`
- **PR 4:** `feat(kernel): implement minimal boot-to-shell path`
- **PR 5:** `feat(vfs): add RAM-disk filesystem and path resolution`
- **PR 6:** `feat(installer): add clean-install and rollback test harness`

## First 30-Day PR Schedule

| Week | Study focus | PRs to open |
|---|---|---|
| 1 | xv6/OSDev boot, Linux kernel entry | CI PR, research PR, QEMU validator PR |
| 2 | xv6 memory, Linux paging | Memory research PR, memory implementation PR, test PR |
| 3 | xv6 processes, Linux scheduling | Scheduler research PR, scheduler implementation PR, test PR |
| 4 | Linux syscalls/VFS, xv6 filesystem | Syscall/VFS research PR, RAM-disk VFS PR, boot-to-shell PR |

## Definition of Merged and Complete

A missing component is considered added only when:

1. A study issue and ADR exist.
2. Implementation is merged through a PR.
3. Unit and integration tests pass.
4. The component is connected to a real runtime path.
5. QEMU or hardware evidence exists.
6. A regression test prevents reversal.
7. Documentation and capability status are updated.
8. Security review is complete where relevant.
9. Rollback or recovery behavior is defined where relevant.
10. CI remains green after merge.
