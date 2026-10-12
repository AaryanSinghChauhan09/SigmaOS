# PR Specification: Periodic, PR-Only Component-Rule System

## 1. Specification Overview

This document specifies the 12-week study plan, 10-month component milestones, PR workflow guidelines, and CI validation gates for the SigmaOS periodic, PR-only component-rule system.

---

## 2. 12-Week Upstream Study Plan

Every weekly cycle studies one narrow upstream OS mechanism and yields a PR deliverable:

| Week | Study focus | Component improved | PR deliverable |
|---|---|---|---|
| **1** | xv6/OSDev boot and interrupts | Kernel/boot | QEMU boot validator and boot trace |
| **2** | xv6 memory and Linux paging | Memory | Frame allocator and paging tests |
| **3** | xv6 processes and Linux scheduling | Scheduler | Minimal scheduler and process-switch test |
| **4** | Linux syscalls and VFS | Syscalls/VFS | Syscall table, path resolver, RAM-disk FS |
| **5** | Linux IPC and FreeBSD IPC | IPC | Pipes, signals, FD lifecycle tests |
| **6** | Arch/Debian packaging | Packaging | Metadata parser and dependency resolver |
| **7** | Fedora/Mint updates | Installer/update | Atomic update and rollback test |
| **8** | OpenBSD/FreeBSD security | Security | Capability denial tests and sandbox policy |
| **9** | Linux drivers and virtio | Drivers | virtio-blk, virtio-net, VGA tests |
| **10** | Redox/Rust OS design | Architecture | Unsafe-code policy and module boundaries |
| **11** | Linux networking | Network | Socket and basic network integration test |
| **12** | Mint/desktop onboarding | Zenith desktop | Graphical smoke test |

---

## 3. 10-Month Component Milestones

Each month completes one major component milestone with evidence-gated promotion:

| Month | Component milestone | Promotion target |
|---|---|---|
| **1** | QEMU x86_64 boot-to-shell | Kernel/boot: `Integrated` |
| **2** | Memory allocator and paging validation | Memory: `Integrated` |
| **3** | Scheduler and syscall foundation | Kernel: `Integrated` |
| **4** | VFS and RAM-disk filesystem | VFS: `Integrated` |
| **5** | Processes, signals, pipes | IPC: `Integrated` |
| **6** | Installer, update, rollback | Installer/update: `Integrated` |
| **7** | Capability model and sandbox | Security: `Hardened` |
| **8** | Virtio device support | Drivers: `Integrated` |
| **9** | Package metadata and dry-run | Packaging: `Integrated` |
| **10** | Minimal desktop session | Desktop: `Prototype` or `Integrated` |

---

## 4. Branch, Commit, and PR Naming Conventions

### Branch Naming
```text
study/<component>-<topic>-<id>
feat/<component>-<capability>-<id>
fix/<component>-<issue>-<id>
test/<component>-<scenario>-<id>
docs/<component>-<topic>-<id>
chore/ci-<topic>-<id>
security/<component>-<topic>-<id>
```

### Commit Messages
```text
docs(research): study Linux VFS path resolution
feat(vfs): add centralized path resolver
test(vfs): add malformed-path regression tests
fix(kernel): classify boot failure at init stage
security(package): sandbox package-script execution
chore(ci): require QEMU boot evidence for kernel PRs
```

### PR Titles
```text
<type>(<component>): <concise change>

feat(vfs): add centralized path resolver
security(package): sandbox package-script execution
```

---

## 5. Required CI Verification Commands

Every PR must pass the following root verification suite:

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```
