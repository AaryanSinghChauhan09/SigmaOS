# SigmaOS Component Rules & Periodic PR-Only Governance Model

SigmaOS adopts a **periodic, PR-only component-rule system**: every cycle studies one upstream OS mechanism, converts it into enforceable rules for one SigmaOS component, implements the smallest connected improvement in a feature branch, and merges only after CI, tests, runtime evidence, documentation, and review pass.

The repository structure—`AGENTS.md`, component agent guides, capability/status inventories, tests, scripts, installer assets, and comparative gap analysis—governs subsystem development, but code presence and unit tests alone do not establish kernel integration, bootability, installation readiness, or hardware support without runtime evidence.

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

Roadmap items remain proposals until their completion criteria pass; therefore, every cycle must end with evidence, not another planning document.

---

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

---

## Component Rule Template

Each component maintains a rules document in `docs/component-rules/<component>.md` following this structure:

```markdown
# Component Rules: <Component Name>

## Component Owner
<agent-owner>

## Upstream References
- <Upstream Ref 1>
- <Upstream Ref 2>

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS <Component> components.

## Mandatory Rules
1. <Rule 1>
2. <Rule 2>

## Prohibited
- <Prohibited 1>
- <Prohibited 2>

## Required Evidence
- Unit test report
- Integration test report
- QEMU boot/runtime log
- Regression test for bug fixes

## Promotion Criteria
| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
```

---

## 14 Industry-Standard Governance Rules

1. **Rule 1: Everything is a pull request** — No direct commits to `main`. Research, rules, implementation, tests, documentation, CI, and status changes arrive through PRs.
2. **Rule 2: One component, one outcome per PR** — A PR adds one capability, enforces one rule, fixes one regression, or updates one documentation area.
3. **Rule 3: Study before implementation** — A component rule or major component change requires an upstream study note and ADR first.
4. **Rule 4: Rules must be testable** — Every component rule must have at least one unit test, integration test, static check, CI gate, or runtime validation that detects violation.
5. **Rule 5: Runtime evidence controls status** — Unit tests prove logic; QEMU boot, installer, update, package, and hardware tests prove the OS.
6. **Rule 6: Reproducibility** — Every PR must be reproducible from a clean checkout with recorded commands, toolchain versions, exit codes, and artifact hashes.
7. **Rule 7: Least privilege** — Agents, CI jobs, packages, and services receive only minimum required permissions.
8. **Rule 8: Security review is separate** — Core security mechanisms require dedicated security review.
9. **Rule 9: Fail safely** — Boot, installer, update, filesystem, and storage PRs must define rollback or recovery behavior before merge.
10. **Rule 10: Regression tests are mandatory** — Every bug fix and newly enforced rule must include a test preventing regression.
11. **Rule 11: License compliance first** — Dual-licensed MIT/GPL-2.0. Copied upstream code requires license compatibility and attribution; otherwise use clean-room implementations.
12. **Rule 12: No borrowed-status claims** — Upstream features do not establish SigmaOS parity without explicit evidence.
13. **Rule 13: Human approval for high-risk merges** — Bootloader, installer, update, security-boundary, package-execution, release, and destructive storage PRs require human review.
14. **Rule 14: Archive stale research** — Research older than two quarters must be reaffirmed or archived.

---

## Periodic Study Plan

### Weekly Cadence

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

### Monthly Milestones

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
