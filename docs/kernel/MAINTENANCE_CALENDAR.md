# SigmaOS Kernel Maintenance Calendar and Development Plan

## Overview
This document defines the formal recurring task calendar, subsystem review rotation, reference-project study framework, biweekly/monthly/quarterly audit cycles, and the first 12 PR roadmap for the **SigmaOS Kernel Lifecycle Program**.

---

## Recurring Task Calendar

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

---

## Weekly Subsystem Review Rotation (8 Weeks)

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

---

## First 12 Kernel PR Roadmap

| # | PR | Purpose | Status |
|---:|---|---|---|
| 1 | `docs(kernel): add kernel maintenance calendar` | Establish recurring kernel work | Completed |
| 2 | `docs(kernel): add integration contracts and dependency map` | Make subsystem boundaries explicit | Completed |
| 3 | `feat(boot): add QEMU boot-to-shell smoke test` | Create the first runtime gate | Completed |
| 4 | `docs(platform): add boot matrix and failure policy` | Track boot evidence | Completed |
| 5 | `feat(syscalls): add syscall contract and validation tests` | Stabilize syscall boundary | Completed |
| 6 | `test(memory): add allocator OOM and leak stress tests` | Harden memory management | Completed |
| 7 | `feat(diagnostics): add structured boot and panic reports` | Make failures actionable | Completed |
| 8 | `docs(drivers): add evidence-based hardware matrix` | Prevent unverified driver claims | Completed |
| 9 | `feat(security): add kernel threat model and capability policy` | Establish security baseline | Completed |
| 10 | `test(fuzz): add syscall and ELF fuzz targets` | Protect external-input paths | Completed |
| 11 | `feat(installer): add recovery-path rehearsal` | Validate install/update recoverability | Completed |
| 12 | `docs(release): add kernel release gates and checklist` | Prevent premature release claims | Completed |
