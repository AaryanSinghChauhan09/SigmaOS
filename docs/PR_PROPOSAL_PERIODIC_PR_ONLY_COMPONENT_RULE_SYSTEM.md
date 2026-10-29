# PR Proposal: SigmaOS Periodic PR-Only Component-Rule System

## Executive Summary

SigmaOS adopts a **periodic, PR-only component-rule system**: every cycle studies one upstream OS mechanism, converts it into enforceable rules for one SigmaOS component, implements the smallest connected improvement in a feature branch, and merges only after CI, tests, runtime evidence, documentation, and review pass.

The repository already has the right structure—`AGENTS.md`, component agent guides, capability/status inventories, tests, scripts, installer assets, and comparative gap analysis—but code presence and unit tests do not establish kernel integration, bootability, installation readiness, or hardware support. Every cycle must produce runtime evidence, not just documentation.

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

---

## Component Rule Lifecycle

Every SigmaOS component follows this 12-step loop:

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

## Component-Specific Mandatory Rules

### Kernel Rules

- All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
- Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
- Syscall handlers must validate every argument and return standardized errors.
- A kernel panic must produce a classified diagnostic and avoid silently continuing.
- Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

### Memory Rules

- Allocation and deallocation must be paired and measurable.
- Allocation failure must be handled explicitly.
- Paging changes require boot validation and memory-integrity tests.
- No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.

### VFS/Filesystem Rules

- All file access must pass through the VFS abstraction.
- Path resolution, mount boundaries, and error codes must be centralized.
- Filesystem state must remain coherent after failed operations.
- New filesystems require conformance tests before integration.

### IPC Rules

- Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
- IPC must enforce capability checks.
- Cross-process tests are mandatory for every IPC mechanism.

### Security Rules

- Every privileged operation requires an explicit capability.
- Denial tests are mandatory: protected operations must fail without the required capability.
- Package scripts, downloaded binaries, and external repositories are untrusted by default.
- Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.

### Driver Rules

- A driver name does not imply hardware support.
- Every driver must define probe, initialization, error, and shutdown behavior.
- Device support requires a named device, driver version, test date, and log.
- QEMU virtio devices should be validated before physical hardware claims.

### Packaging Rules

- Package metadata must be parsed before any installation action.
- Dependency resolution must support dry-run mode.
- Package scripts must run only in a sandbox with declared capabilities.
- Updates must be atomic or recoverable; failed updates must restore the prior known-good state.

### Desktop Rules

- Desktop features must not precede a stable boot and core-runtime path.
- Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
- Graphical claims require a QEMU graphical smoke test.

### Installer/Update Rules

- Installation must be testable in QEMU from a clean virtual disk.
- Updates must define pre-checks, staged activation, rollback, and recovery.
- A failed update must either complete atomically or restore the prior state.

---

## Periodic Study Plan

### 12-Week Study Schedule

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

### 10-Month Component Milestones

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

## PR-Only Workflow & Branch Conventions

### Branches

```text
study/<component>-<topic>-<id>
feat/<component>-<capability>-<id>
fix/<component>-<issue>-<id>
test/<component>-<scenario>-<id>
docs/<component>-<topic>-<id>
chore/ci-<topic>-<id>
security/<component>-<topic>-<id>
```

### Commits

```text
docs(research): study Linux VFS path resolution
feat(vfs): add centralized path resolver
test(vfs): add malformed-path regression tests
fix(kernel): classify boot failure at init stage
security(package): sandbox package-script execution
chore(ci): require QEMU boot evidence for kernel PRs
```

---

## CI Gates

Verification entry points:

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

Component-specific gates:

| Component | Additional required gate |
|---|---|
| Kernel/boot | QEMU boot smoke test |
| Memory | Allocation, exhaustion, and fuzz tests |
| VFS/filesystem | Filesystem conformance and QEMU RAM-disk test |
| IPC | Cross-process interoperability tests |
| Security | Denial tests and fuzzing |
| Drivers | QEMU device-specific test |
| Packaging | Metadata corpus and dry-run tests |
| Installer/update | Clean install and rollback test |
| Desktop | Graphical smoke test |

---

## 14 Industry-Standard Rules

1. **Everything is a pull request**: No direct commits to `main`.
2. **One component, one outcome per PR**: Focused scope per pull request.
3. **Study before implementation**: Require research note and ADR first.
4. **Rules must be testable**: Every rule must have a test or CI check.
5. **Runtime evidence controls status**: Evidence in `reports/` required for promotion.
6. **Reproducibility**: Recorded environment, commands, and artifact hashes.
7. **Least privilege**: Minimum necessary permissions for agents and scripts.
8. **Security review is separate**: Mandatory security review for capability changes.
9. **Fail safely**: Defined rollback or recovery behavior before merge.
10. **Regression tests are mandatory**: Every bug fix requires a regression test.
11. **License compliance first**: Dual MIT/GPL-2.0 compliance with attribution.
12. **No borrowed-status claims**: Upstream feature names do not imply SigmaOS parity.
13. **Human approval for high-risk merges**: Bootloader, security, installer, and release PRs require human approval.
14. **Archive stale research**: Research older than two quarters must be updated or archived.

---

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
