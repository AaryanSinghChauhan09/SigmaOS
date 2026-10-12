# SigmaOS Kernel Maintenance Calendar

## Overview

SigmaOS maintains a recurring kernel lifecycle program modeled on Linux and BSD practices, adapted for SigmaOS's Rust codebase. The primary objective is to maintain SigmaOS as a bootable, testable, secure, recoverable kernel codebase—not as a collection of independently passing subsystem models.

---

## Task Cadence

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

---

## First 12 Kernel PR Roadmap

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
