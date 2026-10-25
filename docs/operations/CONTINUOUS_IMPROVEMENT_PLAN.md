# SigmaOS Continuous OS Improvement Operational Plan

## 12-Week Periodic Study Schedule

| Week | Study Focus | Component Improved | PR Deliverable |
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

## 10-Month Component Milestones

| Month | Component Milestone | Promotion Target |
|---|---|---|
| 1 | QEMU x86_64 boot-to-shell | Kernel/boot: Integrated |
| 2 | Memory allocator and paging validation | Memory: Integrated |
| 3 | Scheduler and syscall foundation | Kernel: Integrated |
| 4 | VFS and RAM-disk filesystem | VFS: Integrated |
| 5 | Processes, signals, pipes | IPC: Integrated |
| 6 | Installer, update, rollback | Installer/update: Integrated |
| 7 | Capability model and sandbox | Security: Hardened |
| 8 | Virtio device support | Drivers: Integrated |
| 9 | Package metadata and dry-run | Packaging: Integrated |
| 10 | Minimal desktop session | Desktop: Prototype or Integrated |

## Definition of Done

A component rule or missing component is complete only when:

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
