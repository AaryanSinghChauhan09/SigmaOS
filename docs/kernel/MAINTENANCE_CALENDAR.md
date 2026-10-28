# SigmaOS Kernel Maintenance Calendar

This document establishes the recurring kernel development, validation, hardening, and maintenance task cadences for SigmaOS.

## Kernel Maintenance Objective

> **Maintain SigmaOS as a bootable, testable, secure, recoverable kernel codebase—not as a collection of independently passing subsystem models.**

## Recurring Task Schedule

| Frequency | Kernel Task | Primary Output | Required PR |
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

## Weekly Kernel Subsystem Rotation Schedule

| Week | Area | Review Scope & Focus |
|---|---|---|
| 1 | Boot, interrupts, traps | Firmware-to-kernel handoff, GDT/IDT setups, interrupt latency & safety |
| 2 | Memory: paging, allocators, swap | Frame allocator limits, 4-level page tables, swap slot bounds, OOM recovery |
| 3 | Scheduler and processes | Bounded run queues, priority levels, context switches, POSIX lifecycle |
| 4 | VFS and filesystems | Path resolution, permission checks, mount hierarchy, crash consistency |
| 5 | Syscalls and ELF | Argument bounds validation, copy-in/copy-out safety, ELF header checks |
| 6 | Drivers and devices | Probing, IRQ handlers, DMA safety, emulator/hardware evidence |
| 7 | Networking and sockets | Packet buffer limits, TCP state machine, socket locks, resource bounds |
| 8 | IPC, signals, timers | Queue capacity, signal masking, timer handler race safety, cleanup |
