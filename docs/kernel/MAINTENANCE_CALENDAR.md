# SigmaOS Kernel Maintenance Calendar and Operating Model

## Kernel Maintenance Objective

> **Maintain SigmaOS as a bootable, testable, secure, recoverable kernel codebase—not as a collection of independently passing subsystem models.**

The first recurring priority is the firmware-to-shell path, followed by installer/update recoverability, hardware evidence, and connecting subsystem models to runtime paths.

## Kernel Task Calendar

| Frequency | Kernel task | Primary output | Required PR |
|---|---|---|---|
| Every PR | Build, lint, unit, and standalone test gate | CI result | `fix`, `feat`, `test`, or `chore` PR |
| Daily | CI and nightly failure triage | Root-cause issue or fix | `fix(ci)` / `fix(kernel)` PR |
| Weekly | Kernel subsystem review | Status + gap list | `docs(kernel)` or scoped `feat` PR |
| Weekly | Upstream Linux/BSD kernel study | Study note | `docs(studies)` PR |
| Biweekly | Syscall ABI and compatibility review | Syscall contract update | `feat(syscalls)` / `test(syscalls)` PR |
| Biweekly | Memory and concurrency audit | Leak/race/OOM findings | `fix(memory)` / `test(memory)` PR |
| Monthly | Boot-to-shell validation | Boot log + matrix update | `test(boot)` PR |
| Monthly | Driver and hardware audit | Evidence-based hardware matrix | `docs(drivers)` PR |
| Monthly | Fuzzing triage | Crash reports and fixes | `fix(...)` + `test(...)` PR |
| Monthly | Kernel panic/diagnostic rehearsal | Failure report | `feat(diagnostics)` PR |
| Quarterly | Kernel release readiness review | Release checklist | `docs(release)` PR |
| Quarterly | Architecture and API stability review | ADR + roadmap | `docs(adr)` PR |
| Quarterly | Security architecture review | Threat model update | `security(...)` PR |
| Semiannual | Kernel code-health and dependency audit | Tech-debt backlog | `chore(kernel)` PR |

## Kernel Development Workflow

Every kernel change follows this 7-step sequence:

1. Open a scoped issue or select an existing gap.
2. Study the relevant Linux, FreeBSD, OpenBSD, Redox, or xv6 mechanism.
3. Define observable acceptance behavior.
4. Implement the smallest safe change.
5. Add unit, integration, and runtime evidence.
6. Update component status only after evidence exists.
7. Merge through a reviewed PR.

Proposals remain proposals until their completion criteria pass.

## Weekly Kernel Subsystem Review Schedule

| Week | Area | Review Questions |
|---|---|---|
| 1 | Boot, interrupts, traps | Does early boot reach the kernel? Are interrupt paths safe and testable? |
| 2 | Memory: paging, allocators, swap | Are allocation limits, OOM behavior, and page-table invariants defined? |
| 3 | Scheduler and processes | Are fairness, priorities, context switches, and process lifecycle tested? |
| 4 | VFS and filesystems | Are path validation, permissions, mounts, and crash behavior tested? |
| 5 | Syscalls and ELF | Are argument validation, error codes, and loader boundaries tested? |
| 6 | Drivers and devices | Is each driver probe/remove-safe, interrupt-safe, and evidence-backed? |
| 7 | Networking and sockets | Are packet parsing, socket lifecycle, and resource limits tested? |
| 8 | IPC, signals, timers | Are races, cleanup, masking, and timeout semantics tested? |

## Reference-Project Study Guidance

When studying upstream kernel mechanisms (e.g. CFS vs ULE, buddy/slab, Capsicum/unveil, xv6 traps, Redox schemes):
- Focus on one narrow mechanism, never an entire distro.
- Save study notes in `docs/studies/YYYY-Wxx-<topic>.md`.
- Include References, Mechanism analysis, SigmaOS state, Decision (Adopt / Adapt / Reject), and PR plan.

## First 12 Kernel PRs Sequence

| # | PR | Purpose |
|---:|---|---|
| 1 | `docs(kernel): add kernel maintenance calendar` | Establish recurring kernel work |
| 2 | `docs(kernel): add integration contracts and dependency map` | Make subsystem boundaries explicit |
| 3 | `feat(boot): add QEMU boot-to-shell smoke test` | Create the first runtime gate |
| 4 | `docs(platform): add boot matrix and failure policy` | Track boot evidence |
| 5 | `feat(syscalls): add syscall contract and validation tests` | Stabilize syscall boundary |
| 6 | `test(memory): add allocator OOM and leak stress tests` | Harden memory management |
| 7 | `feat(diagnostics): add structured boot and panic reports` | Make failures actionable |
| 8 | `docs(drivers): add evidence-based hardware matrix` | Prevent unverified driver claims |
| 9 | `feat(security): add kernel threat model and capability policy` | Establish security baseline |
| 10 | `test(fuzz): add syscall and ELF fuzz targets` | Protect external-input paths |
| 11 | `feat(installer): add recovery-path rehearsal` | Validate install/update recoverability |
| 12 | `docs(release): add kernel release gates and checklist` | Prevent premature release claims |

## Kernel Health Metrics

| Metric | Target |
|---|---|
| Main branch CI pass rate | 100% |
| QEMU boot-to-shell pass rate | 100% |
| Kernel PRs with tests or documented exemption | 100% |
| `unsafe` blocks with documented invariants | 100% |
| Syscalls with validation and error contract | 100% |
| Fuzz targets for external-input parsers | 100% |
| Driver claims backed by evidence | 100% |
| Boot-critical subsystems with diagnostic output | 100% |
| Security-sensitive PRs with security review | 100% |
| Status entries linked to evidence | 100% |
