# SigmaOS Periodic Component-Rule System

## Overview

SigmaOS adopts a **periodic, PR-only component-rule system**: every cycle studies one upstream OS mechanism, converts it into enforceable rules for one SigmaOS component, implements the smallest connected improvement in a feature branch, and merges only after CI, tests, runtime evidence, documentation, and review pass.

The repository's README is explicit that code presence and unit tests do not establish kernel integration, bootability, installation readiness, or hardware support. Roadmap items remain proposals until their completion criteria pass; therefore, every cycle must end with evidence, not another planning document.

## Periodic Governance Model

A fixed cadence ensures upstream study always produces enforceable component rules and reviewable pull requests.

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

## Periodic Study Plan (12-Week Cycle)

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

## PR-Only Workflow Steps

1. **Open a study issue**: Define component, upstream study, gap, expected PRs, validation requirement.
2. **Open a research/ADR PR**: Branch `study/<component>-<topic>-<id>`, adds upstream notes, ADR, and comparison.
3. **Open an implementation PR**: Branch `feat/<component>-<capability>-<id>`, minimal safe changes with unit/integration tests.
4. **Open a test/evidence PR**: Branch `test/<component>-<scenario>-<id>`, runtime/QEMU logs under `reports/`.
5. **Open documentation/status PR**: Branch `docs/<component>-<topic>-<id>`, rules, known limits, matrix updates.

## CI Gates

Every PR must pass:
```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

Component-specific additional gates:
- Kernel/boot: QEMU boot smoke test
- Memory: Allocation, exhaustion, and fuzz tests
- VFS/filesystem: Filesystem conformance and QEMU RAM-disk test
- IPC: Cross-process interoperability tests
- Security: Denial tests and fuzzing
- Drivers: QEMU device-specific test
- Packaging: Metadata corpus and dry-run tests
- Installer/update: Clean install and rollback test
- Desktop: Graphical smoke test

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
