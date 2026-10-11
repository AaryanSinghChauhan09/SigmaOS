# SigmaOS Component Rules & Governance Architecture

## Periodic Governance Model

SigmaOS operates under a fixed cadence to ensure upstream open-source OS mechanism studies systematically yield enforceable component rules, testable changes, and reviewable pull requests.

| Cadence | Activity | PR type | Required output |
|---|---|---|---|
| Daily | Build, test, boot, and status health | `chore/ci`, `fix/test`, `docs/status` | CI result and failure triage |
| Weekly | Study one upstream mechanism | `docs/research`, `feat/<component>` | Research note, ADR, small implementation PR |
| Biweekly | Revalidate a component | `test/<component>`, `fix/<component>` | Regression/runtime report |
| Monthly | Complete one component milestone | `feat/<component>` | Runtime-validated capability |
| Quarterly | Review architecture, security, roadmap | `docs/roadmap`, `refactor/architecture` | Updated gap register and priorities |

Roadmap items remain proposals until completion criteria pass with runtime evidence.

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

A component rule is enforceable only when connected to code, unit/integration tests, CI checks, documentation, and a runtime validation path.

---

## Component Rule Template

Create one rules file per component under `docs/component-rules/`:

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

---

## Component-Specific Rules

### Kernel rules
- All kernel changes must preserve a defined boot contract: firmware → bootloader → kernel entry → memory → interrupts → scheduler → init → shell.
- Kernel code must minimize `unsafe`; every `unsafe` block requires a safety justification and test or review link.
- Syscall handlers must validate every argument and return standardized errors.
- A kernel panic must produce a classified diagnostic and avoid silently continuing.
- Every kernel PR must run `cargo check --lib`, `cargo test --lib`, `./run_sigma_tests.sh`, and the relevant QEMU boot test.

### Memory rules
- Allocation and deallocation must be paired and measurable.
- Allocation failure must be handled explicitly.
- Paging changes require boot validation and memory-integrity tests.
- No memory subsystem may expose raw pointers across module boundaries without a documented safety invariant.

### VFS/filesystem rules
- All file access must pass through the VFS abstraction.
- Path resolution, mount boundaries, and error codes must be centralized.
- Filesystem state must remain coherent after failed operations.
- New filesystems require conformance tests before integration.

### IPC rules
- Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
- IPC must enforce capability checks.
- Cross-process tests are mandatory for every IPC mechanism.

### Security rules
- Every privileged operation requires an explicit capability.
- Denial tests are mandatory: protected operations must fail without the required capability.
- Package scripts, downloaded binaries, and external repositories are untrusted by default.
- Cryptographic, sandboxing, syscall, memory-safety, networking, and update changes require separate security review.

### Driver rules
- A driver name does not imply hardware support.
- Every driver must define probe, initialization, error, and shutdown behavior.
- Device support requires a named device, driver version, test date, and log.
- QEMU virtio devices should be validated before physical hardware claims.

### Packaging rules
- Package metadata must be parsed before any installation action.
- Dependency resolution must support dry-run mode.
- Package scripts must run only in a sandbox with declared capabilities.
- Updates must be atomic or recoverable; failed updates must restore the prior known-good state.

### Desktop rules
- Desktop features must not precede a stable boot and core-runtime path.
- Every desktop change must define behavior for input loss, application crash, display failure, and shutdown.
- Graphical claims require a QEMU graphical smoke test.

### Installer/update rules
- Installation must be testable in QEMU from a clean virtual disk.
- Updates must define pre-checks, staged activation, rollback, and recovery.
- A failed update must either complete atomically or restore the prior state.

---

## PR-Only Workflow

### 1. Open a study issue
Every cycle begins with an issue specifying component, upstream study, gap, expected PRs, and validation requirement.

### 2. Open a research/ADR PR
**Branch:** `study/<component>-<topic>-<id>`
**Adds:** Research notes, ADRs, and comparative analysis under `research/`.

### 3. Open an implementation PR
**Branch:** `feat/<component>-<capability>-<id>`
**Adds:** Source changes, public interface documentation, unit and integration tests, and runtime hooks.

### 4. Open a test/evidence PR
**Branch:** `test/<component>-<scenario>-<id>`
**Adds:** Regression tests, fuzz targets, QEMU test logs, and structured reports under `reports/`.

### 5. Open a documentation/status PR
**Branch:** `docs/<component>-<topic>-<id>`
**Adds:** Component rules, known limitations, matrix updates, and evidence links.

---

## Pull-Request Template

Mandatory template under `.github/PULL_REQUEST_TEMPLATE.md`:

```markdown
## Summary
<!-- What component does this PR improve, add, or regulate? -->

## Related issue
Closes #<issue-id>

## Component
<!-- kernel, memory, VFS, IPC, security, drivers, packaging, desktop, installer, etc. -->

## Upstream study
- Project:
- Version/commit:
- Research note:
- ADR:

## Component rules added or enforced
- [ ] New rule
- [ ] Existing rule enforced
- [ ] Rule test added
- [ ] CI gate added
- [ ] Documentation updated

## What SigmaOS adopts
<!-- State the upstream principle or mechanism adopted. -->

## What SigmaOS changes or rejects
<!-- State simplifications, deviations, and rejected ideas. -->

## Changes
- [ ] Source implementation
- [ ] Unit tests
- [ ] Integration tests
- [ ] QEMU/runtime test
- [ ] Documentation
- [ ] Capability/status update
- [ ] Regression protection
- [ ] Security review, if applicable

## Verification
```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
make check
make test
make format
```

## Runtime evidence
- [ ] QEMU boot log
- [ ] Installer/update test log
- [ ] Package dry-run report
- [ ] Hardware test log, if applicable
- [ ] Artifact hash

## Safety and rollback
- [ ] No broad credentials or host access introduced
- [ ] Untrusted input validated
- [ ] Package scripts sandboxed, if applicable
- [ ] Rollback/recovery behavior defined
- [ ] Security review completed, if applicable

## Reviewer checklist
- [ ] Scope is narrow
- [ ] Component rules are enforceable
- [ ] Tests cover the rules
- [ ] Runtime evidence is linked
- [ ] Documentation matches implementation
- [ ] Capability status is evidence-based
- [ ] No unsupported Linux/BSD parity claims
```

---

## CI Gates

Verification entrypoints required before merge:

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

## Industry-Standard PR Rules

1. **Everything is a pull request**: No direct commits to `main`. Research, rules, implementation, tests, documentation, CI, and status changes must arrive through PRs.
2. **One component, one outcome per PR**: A PR may add one component capability, enforce one rule, fix one regression, or update one documentation area.
3. **Study before implementation**: A component rule or major component change requires an upstream study note and ADR first.
4. **Rules must be testable**: Every component rule must have at least one unit test, integration test, static check, CI gate, or runtime validation that detects violation.
5. **Runtime evidence controls status**: Unit tests prove logic; QEMU boot, installer, update, package, and hardware tests prove the OS.
6. **Reproducibility**: Every PR must be reproducible from a clean checkout with recorded commands, toolchain versions, exit codes, and artifact hashes.
7. **Least privilege**: Agents, CI jobs, packages, and services receive only minimum required permissions.
8. **Security review is separate**: Capability enforcement, sandboxing, syscalls, memory safety, cryptography, package execution, networking, and update mechanisms require dedicated security review.
9. **Fail safely**: Boot, installer, update, filesystem, and storage PRs must define rollback or recovery behavior before merge.
10. **Regression tests are mandatory**: Every bug fix and newly enforced rule must include a test that would fail if the behavior regressed.
11. **License compliance first**: SigmaOS is dual-licensed MIT/GPL-2.0. Clean-room research notes and original implementations are preferred.
12. **No borrowed-status claims**: A feature in Linux, FreeBSD, OpenBSD, Arch, Fedora, or Redox does not imply the same feature exists in SigmaOS.
13. **Human approval for high-risk merges**: Humans approve bootloader, installer, update, security-boundary, package-execution, release, and destructive-storage PRs.
14. **Archive stale research**: Research older than two quarters must be reaffirmed, updated, or archived.

---

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

---

## First 30-Day PR Schedule

| Week | Study focus | PRs to open |
|---|---|---|
| 1 | xv6/OSDev boot, Linux kernel entry | `chore/ci` verification workflow; `docs/research` boot framework; `feat/kernel` QEMU validator |
| 2 | xv6 memory, Linux paging | `docs/research` memory ADR; `feat/memory` allocator/paging; `test/memory` evidence |
| 3 | xv6 processes, Linux scheduling | `docs/research` scheduler ADR; `feat/kernel` scheduler; `test/kernel` process-switch tests |
| 4 | Linux syscalls/VFS, xv6 filesystem | `docs/research` VFS ADR; `feat/vfs` RAM-disk/path resolver; `feat/kernel` boot-to-shell |

---

## Monthly Component Milestones

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

## Branch & Commit Naming Conventions

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
