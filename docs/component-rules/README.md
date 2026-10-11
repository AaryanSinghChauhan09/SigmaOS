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

## Industry-Standard PR Rules

1. **Everything is a pull request**: No direct commits to `main`. Research, rules, implementation, tests, documentation, CI, and status changes must arrive through PRs.
2. **One component, one outcome per PR**: A PR may add one component capability, enforce one rule, fix one regression, or update one documentation area.
3. **Study before implementation**: A component rule or major component change requires an upstream study note and ADR first.
4. **Rules must be testable**: Every component rule must have at least one unit test, integration test, static check, CI gate, or runtime validation that detects violation.
5. **Runtime evidence controls status**: Unit tests prove logic; QEMU boot, installer, update, package, and hardware tests prove the OS. Passing unit tests alone do not establish kernel integration or hardware support.
6. **Reproducibility**: Every PR must be reproducible from a clean checkout with recorded commands, toolchain versions, exit codes, and artifact hashes.
7. **Least privilege**: Agents, CI jobs, packages, and services receive only minimum required permissions.
8. **Security review is separate**: Capability enforcement, sandboxing, syscalls, memory safety, cryptography, package execution, networking, and update mechanisms require dedicated security review.
9. **Fail safely**: Boot, installer, update, filesystem, and storage PRs must define rollback or recovery behavior before merge.
10. **Regression tests are mandatory**: Every bug fix and newly enforced rule must include a test that would fail if the behavior regressed.
11. **License compliance first**: SigmaOS is dual-licensed MIT/GPL-2.0. Copied upstream code requires license compatibility and attribution; clean-room research notes and original implementations are preferred.
12. **No borrowed-status claims**: A feature in Linux, FreeBSD, OpenBSD, Arch, Fedora, or Redox does not imply the same feature exists in SigmaOS.
13. **Human approval for high-risk merges**: Humans approve bootloader, installer, update, security-boundary, package-execution, release, and destructive-storage PRs.
14. **Archive stale research**: Research older than two quarters must be reaffirmed, updated, or archived.

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
