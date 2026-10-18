# SigmaOS

SigmaOS is a sovereign operating system built in Rust. Missing components are added through a **periodic, PR-only research-and-implementation pipeline**: each cycle studies one upstream OS mechanism, opens a tracking issue, implements the smallest connected component slice in a feature branch, validates it with the repository's documented checks and QEMU/runtime tests, and merges only through a reviewable pull request.

> **IMPORTANT DISCLAIMER**
> Roadmap proposals and design documents become completed work ONLY after their code path and stated checks are verified in runtime testing. Reference-project feature names (from Linux, FreeBSD, OpenBSD, Arch, Debian, Fedora, etc.) serve as design references and do NOT establish SigmaOS feature parity without explicit runtime evidence.

---

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

---

## Periodic Cadence

| Cadence | Activity | PR type | Required evidence |
|---|---|---|---|
| **Daily** | Build/test/boot health | `chore/ci`, `fix/test`, `docs/status` | CI logs, test reports |
| **Weekly** | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, tests |
| **Biweekly** | Revalidate a component | `test/<component>`, `fix/<component>` | Runtime regression report |
| **Monthly** | Add or complete one missing component | `feat/<component>` | QEMU/runtime validation |
| **Quarterly** | Roadmap and architecture review | `docs/roadmap`, `refactor/architecture` | Gap register and updated priorities |

---

## Missing-Component Roadmap

Implement missing components in this order. Each item must be delivered through one or more PRs and must produce runtime evidence.

| Priority | Missing component | Upstream study | First PR deliverable | Validation |
|---|---|---|---|---|
| **P0** | Reproducible CI/build gate | Fedora, Debian | CI workflow and build manifest | Clean build and tests pass |
| **P0** | QEMU boot validator | OSDev, xv6 | Boot script and boot matrix | QEMU boot-to-shell log |
| **P0** | Bootloader integration | OSDev, GRUB/systemd-boot concepts | UEFI boot path | QEMU UEFI boot |
| **P0** | Kernel entry and serial logging | xv6, Linux | Early console and kernel entry | Serial trace |
| **P1** | [Physical memory manager](./PHYSICAL_MEMORY_MANAGER_PLAN.md) | xv6, Linux | Frame allocator | Allocation tests |
| **P1** | Virtual memory/paging | xv6, Linux | Page tables and address spaces | Boot and memory tests |
| **P1** | Interrupts and timer | xv6, OSDev | IDT, exception handlers, timer | Interrupt trace |
| **P1** | [Scheduler](./SCHEDULER_DEVELOPMENT_PLAN.md) / [Dispatcher](./DISPATCHER_PLAN.md) | Linux, xv6 | Minimal round-robin scheduler & task context handoff | Process-switch test |
| **P1** | Syscall interface | Linux, FreeBSD | Stable syscall table and validation | Syscall tests |
| **P1** | Init and shell | Linux Mint, Arch | PID 1, service startup, shell | Boot-to-shell |
| **P2** | VFS | Linux, xv6 | Path resolution, FDs, file ops | Filesystem tests |
| **P2** | RAM-disk filesystem | Linux tmpfs concepts | Simple in-memory FS | QEMU RAM-disk boot |
| **P2** | Processes and signals | Linux, xv6 | Process lifecycle and signals | Interop tests |
| **P2** | [Pipes and IPC](./ipc-handler.md) | Linux, FreeBSD | Pipes, event queues, shared memory | IPC tests |
| **P2** | Installer | Debian, Mint | Clean QEMU install | Install log |
| **P2** | Atomic update/rollback | Fedora, Mint | Update manifest and rollback | Failed-update recovery test |
| **P2** | Security capabilities | OpenBSD, Linux | Capability checks and sandbox policy | Denial tests |
| **P3** | Drivers | Linux, FreeBSD | virtio-blk, virtio-net, VGA | Device logs |
| **P3** | Networking | Linux, FreeBSD | Socket API and basic TCP/IP path | Network integration test |
| **P3** | Desktop | Redox, Mint, Omarchy | Framebuffer, input, window launch | Graphical smoke test |
| **P3** | Package manager | Arch, Debian | Metadata parser and dependency resolver | Package corpus tests |
| **P3** | Distro compatibility | Arch, Debian, Fedora, BSD | Metadata compatibility layer | Compatibility matrix |
| **P3** | [Kernel & Hardening Parity](./37-Tech-Media-And-Linux-BSD-Distro-Parity-PR-Roadmap.md) | Phoronix, HW Busters, CachyOS, OpenBSD | `sched_ext` BPF BORE v2, ATX 3.1 PSU governor, Landlock V6 | Unit & integration tests |
| **P3** | [Desktop & Userland Innovations](./38-Universal-Desktop-And-Userland-Innovations-PR-Roadmap.md) | It's FOSS, How-To Geek, Omarchy, Mint | Wayland 1.24 HDR, Sovereign FOSS apps, Omakase themes | Unit & integration tests |
| **P3** | [Enterprise Security, AI & Network](./39-Enterprise-Security-AI-And-Networking-PR-Roadmap.md) | KDnuggets, The New Stack, Fedora, NixOS | Active Directory SSSD, eBPF PQC WireGuard VPN, SIMD AI | Unit & integration tests |
| **P3** | [Omarchy Desktop Parity AI Roadmap](./40-Omarchy-Linux-And-Sovereign-Desktop-Parity-AI-Agent-Roadmap.md) | Omarchy Linux, Arch Linux, Hyprland, Walker | Ghostty PTY, Master-stack layout, Walker launcher, Wallust sync | Unit & integration tests |
| **P3** | [Linux/BSD Kernel & Security AI Roadmap](./41-Linux-BSD-Kernel-Security-And-Virtualization-AI-Agent-Roadmap.md) | Linux 6.12+, FreeBSD 14.1, OpenBSD 7.6, CachyOS | SchedExt BPF, Landlock V6, FreeBSD VNET, MicroVMs, PREEMPT_RT | Unit & integration tests |

---

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

Examples:
- `study/vfs-path-resolution-042`
- `feat/kernel-qemu-boot-043`
- `test/vfs-ramdisk-044`
- `docs/capability-matrix-audit-045`
- `security/package-sandbox-046`

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

feat(kernel): add QEMU x86_64 boot-to-shell validator
```

---

## Required PR Types

### 1. Research PR
- **Branch:** `study/<component>-<topic>-<id>`
- **Adds:** `research/upstream/<project>/<topic>.md`, `research/decisions/ADR-<id>-<topic>.md`, `research/comparisons/<component>.md`
- **Must contain:** Upstream project/commit, problem solved, core design/data structures, SigmaOS adaptation decision, security/failure assumptions, licensing analysis, implementation scope, and required validation.

### 2. Implementation PR
- **Branch:** `feat/<component>-<capability>-<id>`
- **Adds/changes:** Component source under module boundary, public interface docs, minimal implementation, unit/integration tests, runtime hook/QEMU test, evidence-based status update.

### 3. Test and Evidence PR
- **Branch:** `test/<component>-<scenario>-<id>`
- **Adds:** Regression tests, fuzz targets, QEMU boot/runtime test, report generator, evidence artifacts under `reports/`.

### 4. Documentation and Status PR
- **Branch:** `docs/<component>-<topic>-<id>`
- **Adds:** Component docs, known limitations, troubleshooting guidance, `CAPABILITY_MATRIX.toml` & `FEATURE_STATUS.toml` updates with evidence links.

---

## PR Review Rules

1. **Rule 1: Every change is a PR** — No direct commits to `main`.
2. **Rule 2: One PR, one outcome** — A PR adds one capability, fixes one regression, adds one test suite, or updates one doc area.
3. **Rule 3: Research before implementation** — A missing component must have a study issue and ADR before a large implementation PR is opened.
4. **Rule 4: Evidence before status** — Status matrices updated only with passing tests and runtime evidence.
5. **Rule 5: Runtime path required** — New components must connect to the real boot/runtime path or remain labeled `Prototype`.
6. **Rule 6: Security review is separate** — Core security mechanisms require a dedicated security review.
7. **Rule 7: Least privilege** — No broad credentials, unrestricted network access, or unsandboxed execution.
8. **Rule 8: Reproducibility** — PRs must be reproducible from a clean checkout using recorded commands and artifact hashes.
9. **Rule 9: Rollback safety** — Storage, update, boot, and installer PRs must define rollback or recovery behavior.
10. **Rule 10: License compliance** — Dual MIT/GPL-2.0. Copied upstream code requires license compatibility and attribution; otherwise use clean-room implementation.
11. **Rule 11: No parity claims** — Reference projects are design references, not proof of SigmaOS feature parity.
12. **Rule 12: Human approval for high-risk merges** — Bootloader, installer, update, security-boundary, package-execution, release, and destructive storage PRs require human approval.

---

## Required Verification Commands

Every PR must pass:

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

Component-specific gates:

| PR type | Additional required CI |
|---|---|
| Kernel/boot | QEMU boot smoke test |
| VFS/filesystem | Filesystem conformance and QEMU RAM-disk test |
| Memory | Allocation/fuzz tests |
| Security | Denial tests and fuzzing |
| Installer/update | Clean install and rollback test |
| Packaging | Metadata corpus and dry-run tests |
| Drivers | QEMU device-specific test |
| Desktop | Graphical smoke test |

---

## First Six PRs

- **PR 1:** `chore(ci): add reproducible verification workflow`
- **PR 2:** `docs(research): add periodic OS study framework`
- **PR 3:** `feat(kernel): add QEMU x86_64 boot validator`
- **PR 4:** `feat(kernel): implement minimal boot-to-shell path`
- **PR 5:** `feat(vfs): add RAM-disk filesystem and path resolution`
- **PR 6:** `feat(installer): add clean-install and rollback test harness`

---

## First 30-Day PR Schedule

| Week | Study focus | PRs to open |
|---|---|---|
| 1 | xv6/OSDev boot, Linux kernel entry | CI PR, research PR, QEMU validator PR |
| 2 | xv6 memory, Linux paging | Memory research PR, memory implementation PR, test PR |
| 3 | xv6 processes, Linux scheduling | Scheduler research PR, scheduler implementation PR, test PR |
| 4 | Linux syscalls/VFS, xv6 filesystem | Syscall/VFS research PR, RAM-disk VFS PR, boot-to-shell PR |

---

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
