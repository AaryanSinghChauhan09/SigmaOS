# SigmaOS Kernel Maintenance Calendar

## Objective
Maintain SigmaOS as a bootable, testable, secure, recoverable kernel codebase—not as a collection of independently passing subsystem models.

## Recurring Kernel Task Cadence

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

## Weekly Subsystem Review Rotation

| Week | Area | Review questions |
|---|---|---|
| 1 | Boot, interrupts, traps | Does early boot reach the kernel? Are interrupt paths safe and testable? |
| 2 | Memory: paging, allocators, swap | Are allocation limits, OOM behavior, and page-table invariants defined? |
| 3 | Scheduler and processes | Are fairness, priorities, context switches, and process lifecycle tested? |
| 4 | VFS and filesystems | Are path validation, permissions, mounts, and crash behavior tested? |
| 5 | Syscalls and ELF | Are argument validation, error codes, and loader boundaries tested? |
| 6 | Drivers and devices | Is each driver probe/remove-safe, interrupt-safe, and evidence-backed? |
| 7 | Networking and sockets | Are packet parsing, socket lifecycle, and resource limits tested? |
| 8 | IPC, signals, timers | Are races, cleanup, masking, and timeout semantics tested? |
