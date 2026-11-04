# PR Proposal: SigmaOS Periodic PR-Only Component-Rule System

## Summary

SigmaOS adopts a **periodic, PR-only component-rule system**: every cycle studies one upstream OS mechanism, converts it into enforceable rules for one SigmaOS component, implements the smallest connected improvement in a feature branch, and merges only after CI, tests, runtime evidence, documentation, and review pass.

While the SigmaOS codebase contains structures such as `AGENTS.md`, component agent guides, capability/status inventories, tests, scripts, installer assets, and comparative gap analysis, code presence and unit tests alone do not establish kernel integration, bootability, installation readiness, or hardware support. This proposal establishes the periodic governance framework ensuring roadmap items transition to completed work only through runtime evidence.

## Periodic Governance Model

Use a fixed cadence so upstream study always produces enforceable component rules and reviewable pull requests.

| Cadence | Activity | PR type | Required output |
|---|---|---|---|
| Daily | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| Monthly | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| Quarterly | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

Roadmap proposals remain proposals until their completion criteria pass; every cycle ends with evidence, not another planning document.

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

## Component Rule Template

Rules files reside under `docs/component-rules/`:

```markdown
# Component Rules: VFS

## Component owner
filesystem-agent

## Upstream references
- Linux VFS
- xv6 filesystem
- FreeBSD VFS

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS VFS components.

## Mandatory rules

1. All path resolution must use the central resolver.
2. Every filesystem operation must return a standardized error code.
3. A failed operation must leave filesystem state coherent.
4. New filesystem backends must implement the VFS trait contract.
5. Path traversal, invalid components, and mount boundaries require tests.
6. No VFS change may bypass capability checks.
7. Every VFS change must include unit, integration, and QEMU runtime evidence.

## Prohibited

- Direct filesystem access outside the VFS abstraction
- Silent error swallowing
- Unsupported parity claims
- Status promotion without runtime evidence

## Required evidence

- Unit test report
- Integration test report
- QEMU RAM-disk boot log
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
```

## Component-Specific Rules

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

## Periodic Study Plan

### Weekly Cycle
Study one narrow mechanism, not an entire project.

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

### Monthly Milestone Targets

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

## PR-Only Workflow

### 1. Open a Study Issue
Every cycle begins with an issue defining the component, upstream reference, observed gap, expected PRs, and validation requirements.

### 2. Open a Research/ADR PR
Branch: `study/<component>-<topic>-<id>`
Adds research notes in `research/upstream/`, ADR in `research/decisions/`, and comparison matrices in `research/comparisons/`.

### 3. Open an Implementation PR
Branch: `feat/<component>-<capability>-<id>`
Adds source code changes, public interface documentation, unit tests, and runtime integration hooks.

### 4. Open a Test/Evidence PR
Branch: `test/<component>-<scenario>-<id>`
Adds regression tests, negative/denial tests, fuzzing targets, and runtime evidence in `reports/`.

### 5. Open Documentation/Status PR
Branch: `docs/<component>-<topic>-<id>`
Updates component rules, limitations, and `CAPABILITY_MATRIX.toml` / `FEATURE_STATUS.toml`.

## Branch, Commit, and PR Title Conventions

- Branches: `study/...`, `feat/...`, `fix/...`, `test/...`, `docs/...`, `chore/ci-...`, `security/...`
- Commits: Conventional Commits (`docs(research): ...`, `feat(vfs): ...`, `test(vfs): ...`, `fix(kernel): ...`, `security(package): ...`, `chore(ci): ...`)
- PR Titles: `<type>(<component>): <concise change>`

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
