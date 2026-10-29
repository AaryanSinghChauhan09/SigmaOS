# SigmaOS Kernel Maintenance Calendar & Recurring Operations

This document establishes the recurring development, validation, hardening, and maintenance program for the SigmaOS kernel codebase.

## Objective

Maintain SigmaOS as a bootable, testable, secure, recoverable kernel codebase—not as a collection of independently passing subsystem models.

Primary priority order:
1. Firmware-to-shell boot path.
2. Installer and update recoverability.
3. Evidence-backed driver/hardware support matrix.
4. Subsystem model connection to runtime execution paths.

---

## Recurring Task Schedule

| Frequency | Kernel Task | Primary Output | Required PR Type |
|---|---|---|---|
| Every PR | Build, lint, unit, and standalone test gate | CI result | `fix`, `feat`, `test`, `chore` |
| Daily | CI and nightly failure triage | Root-cause issue or fix | `fix(ci)` / `fix(kernel)` |
| Weekly | Kernel subsystem review | Status + gap list | `docs(kernel)` / `feat` |
| Weekly | Upstream Linux/BSD kernel study | Study note | `docs(studies)` |
| Biweekly | Syscall ABI and compatibility review | Syscall contract update | `feat(syscalls)` / `test(syscalls)` |
| Biweekly | Memory and concurrency audit | Leak/race/OOM findings | `fix(memory)` / `test(memory)` |
| Monthly | Boot-to-shell validation | Boot log + matrix update | `test(boot)` |
| Monthly | Driver and hardware audit | Evidence-backed hardware matrix | `docs(drivers)` |
| Monthly | Fuzzing triage | Crash reports and fixes | `fix(...)` + `test(...)` |
| Monthly | Kernel panic/diagnostic rehearsal | Failure report | `feat(diagnostics)` |
| Quarterly | Kernel release readiness review | Release checklist | `docs(release)` |
| Quarterly | Architecture and API stability review | ADR + roadmap | `docs(adr)` |
| Quarterly | Security architecture review | Threat model update | `security(...)` |
| Semiannual | Kernel code-health and dependency audit | Tech-debt backlog | `chore(kernel)` |

---

## Weekly Subsystem Review Rotation

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

---

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
