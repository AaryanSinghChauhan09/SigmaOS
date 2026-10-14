# SigmaOS Signal Handler Architecture & Development Plan

## Subsystem Overview

SigmaOS includes signal-related components across the kernel and IPC layers, including root-level `signal`, `signalfd`, POSIX compatibility adapters, and process manager dispatch. The signal subsystem syscall ABI provides numbers and libc wrapper structures, while requiring end-to-end kernel-side dispatch, signal frame construction, trampoline returns, and Ring 3 validation.

This document treats signal handling as **Prototype/Modeled** until a real user process receives a signal in Ring 3, enters a handler callback via an architecture-specific frame, returns through a validated kernel trampoline (`sigreturn`), and safely resumes execution.

### Key Conceptual Distinction
- **Signal Subsystem**: Generating, queuing, masking, selecting, filtering, and delivering signals across process and thread boundaries.
- **Signal Handler**: The user-space callback entered via an architecture-specific signal frame constructed by the kernel, returning through a trusted kernel trampoline (`sigreturn`).

POSIX.1-2024 specifies the `sigaction()` contract. While Linux and FreeBSD provide operational reference models, SigmaOS explicitly documents its supported POSIX subset and intentional differences rather than assuming full glibc/Linux compatibility.

---

## Required Semantics and Safety Rules

1. **Default to `sigaction` Semantics**
   - POSIX recommends `sigaction()` for explicit control over signal dispositions, flags, and masks.
   - `signal()` is provided solely as a libc compatibility wrapper defaulting to `sigaction()` semantics.

2. **Distinguish Process- and Thread-Directed Signals**
   - Store process-wide signal dispositions separately from each thread's blocked signal mask and thread-pending queue.
   - Select an eligible, unblocked thread for signal delivery only at safe delivery points (e.g., syscall return, interrupt/exception return, or scheduler transition to user mode).

3. **Protect Handler Execution State**
   - On signal delivery, save interrupted context (registers, stack pointer, instruction pointer), prior signal mask, and alternate stack state into a user-space frame.
   - Restore state strictly through a validated kernel `sigreturn` path.

4. **Enforce Uncatchable Signals in Kernel**
   - `SIGKILL` (9) and `SIGSTOP` (19) must never be caught, ignored, or blocked.
   - Kernel dispatch must reject `sigaction` or signal mask modifications targeting `SIGKILL` or `SIGSTOP`.

5. **Respect Masks, Nesting, and Flags**
   - Automatically block the delivered signal during handler execution unless `SA_NODEFER` is set.
   - Temporarily add `sa_mask` to the thread's blocked signal mask while the handler runs.
   - Restore the previous signal mask upon normal return via `sigreturn`.
   - Support `SA_RESETHAND` (reset disposition to `SIG_DFL` upon delivery).

6. **Define Interruption and Restart Behavior**
   - For every blocking system call (e.g., `read`, `write`, `waitpid`, `nanosleep`), specify whether interruption results in `EINTR`, partial completion, or automatic restart under `SA_RESTART`.

7. **Minimal and Auditable Handler Delivery Path**
   - Kernel signal frame delivery and return paths must allocate zero dynamic heap memory, avoid sleeping in invalid contexts, and strictly validate user-controlled frame parameters before restoring privileged context.
   - Document async-signal-safety guidance for user applications (avoid non-reentrant routines like `malloc` or `printf` inside signal handlers).

8. **Honest Evidence-Based Status Tracking**
   - Feature status claims must reflect runtime execution reality.
   - Signal frame data structure models and unit tests constitute Prototype evidence, not Integrated status.

---

## Proposed PR Sequence Roadmap

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(signal-handler): audit signal path and define ABI profile` | Audit signal constants, `sigaction`, masks, pending state, syscall dispatch, process/thread model, x86_64 return path, libc, and `signalfd`. Document supported POSIX subset and architecture limitations. | Each feature links to code, syscall dispatch, tests, and runtime evidence; compatibility claims are explicit; unsupported flags and signals are listed. |
| **2** | `test(signal-handler): add ABI and signal-frame layout checks` | Add compile-time/runtime checks for signal numbers, `sigset_t`, `sigaction`, `siginfo_t`, context layouts, stack alignment, frame offsets, and trampoline ABI. | Layout tests match selected architecture ABI; incompatible field/number changes fail CI; tests distinguish compile-time layout evidence from runtime delivery. |
| **3** | `feat(signal): connect signal syscalls to kernel dispatch` | Wire `sigaction`, signal mask, `kill`/`raise`, pending query, and signal-return syscall numbers to real handlers using safe user-copy APIs. | Ring 3 process can set/query disposition, block/unblock a signal, and send to authorized target; invalid pointers/signals fail safely; uncatchable restrictions enforced. |
| **4** | `feat(signal): add per-thread masks and pending signal queues` | Separate process dispositions and process-pending signals from thread masks and thread-directed pending signals; define standard-signal coalescing policy. | Tests cover pending/blocked/unblocked transitions, thread selection, process/thread targeting, exit cleanup, no duplicate delivery; semantics match documented subset. |
| **5** | `feat(signal-handler): deliver default actions at safe return points` | Deliver default ignore/terminate/stop/continue actions at syscall, exception, or scheduler return-to-user points. | Real user process terminates/stops/continues according to documented actions; delivery never occurs while kernel state is inconsistent; `SIGKILL`/`SIGSTOP` remain unmaskable. |
| **6** | `feat(signal-handler): construct validated x86_64 user signal frame` | Build frame with interrupted registers, instruction/stack pointers, prior mask, signal number, return trampoline; redirect user execution to handler. | QEMU Ring 3 test receives caught signal, observes expected argument, returns to interrupted instruction with state restored; malformed or unmapped stack fails safely. |
| **7** | `feat(signal-handler): implement x86_64 sigreturn validation` | Implement signal-return syscall/trampoline handling; validate user frame address, canonical user addresses, flags, stack pointer, instruction pointer, allowed register state. | Handler return restores user state and mask; forged frames cannot enter kernel mode or restore privileged flags; invalid frames terminate or signal process per policy. |
| **8** | `feat(signal-handler): implement action masks and restart semantics` | Apply `sa_mask`, automatic self-blocking, `SA_NODEFER`, `SA_RESETHAND`, and carefully scoped `SA_RESTART` subset. | Nested handler tests verify masks; actions remain installed/reset as specified; interrupted `read`/`wait` tests verify `EINTR` and restart behavior against published contract. |
| **9** | `feat(signal-handler): add siginfo and SA_SIGINFO subset` | Add typed signal information for supported kernel-generated events and three-argument handlers. | `siginfo_t` fields are initialized and bounded; source/code values documented; user payloads cannot smuggle pointers or privileged data. |
| **10** | `feat(signal-handler): add alternate stack support` | Add `sigaltstack` and `SA_ONSTACK` with stack mappings and guard pages. | Valid alternate stack receives eligible handlers; nested/disabled/too-small stack cases have defined behavior; return restores original stack settings. |
| **11** | `feat(signal): integrate SIGCHLD and wait/reap lifecycle` | Generate child status signals for supported exit/stop/continue transitions and integrate with wait/reap. | Tests cover child exit, multiple child events, handler races with `waitpid`, `SA_NOCLDSTOP` if implemented; child status is not lost. |
| **12** | `feat(signal): add synchronous signal-wait APIs` | Implement staged `sigwait`/`sigtimedwait` synchronous acceptance path, avoiding async handlers where callers prefer event-loop behavior. | Blocked signals can be synchronously accepted with correct metadata and timeout; interactions with `sigaction` on same signal documented. |
| **13** | `feat(signal): add queued real-time signal support with limits` | Add real-time signal range (`SIGRTMIN`..`SIGRTMAX`) and FIFO payload queue after standard signal delivery is stable. | Queued instances preserve order and payload; queue limits enforced; overflow and process exit release all queue memory. |
| **14** | `feat(signal): integrate signalfd as Linux-specific extension` | Connect `signalfd` module to real pending state, signal masks, poll/read, FD lifecycle, process permissions. | Ring 3 app blocks signal and reads through descriptor; close, readiness, mask changes work; feature labeled Linux-inspired. |
| **15** | `feat(libc): expose signal APIs and safe handler examples` | Implement `signal.h` constants/types and libc wrappers; provide examples using async-signal-safe behavior or synchronous waiting. | C and Rust userspace tests verify layouts, return values, `errno`, handler arguments, signal return; examples avoid unsafe calls like `printf()` in handlers. |
| **16** | `test(signal-handler): add fault-injection and concurrency suite` | Test nested signals, delivery during syscalls, task exit, mask changes, frame corruption, invalid handler addresses, repeated signals, queue exhaustion. | Reproducible QEMU tests pass; no kernel panic, privilege escape, lost signal, use-after-free, or unbounded allocation; failure artifacts include task/signal state. |
| **17** | `feat(signal-handler): validate AArch64 and RISC-V handler frames` | Add architecture-specific frame/trampoline/return paths as independent PRs once target architecture has real user-mode execution. | Each architecture separately passes handler, mask restoration, signal-return validation, invalid-frame tests; x86_64 results do not imply cross-arch support. |
| **18** | `docs(signal-handler): publish API, safety, and support matrix` | Document signals, actions, handler arguments, async-signal-safe expectations, interruption/restart, architecture support, known gaps. | Every claim links to a test and exact target; POSIX/Linux/BSD differences stated; handler safety guidance prominent. |

**Priority Focus:** Complete PRs **1–8** before attempting alternate stacks (`sigaltstack`), real-time signal queues, or `signalfd` integration.

---

## Periodic Maintenance PR Cadence

| Cadence | Proposed Recurring Task | Required Evidence |
|---|---|---|
| **Every PR** | `ci(signal): run ABI, frame, and handler-path checks` | Build clean, ABI-layout tests pass, signal unit tests pass, QEMU test for changed path passes. |
| **Weekly** | `test(signal): triage handler crashes and delivery regressions` | Minimized reproducer, affected signal/architecture, owner, severity, regression test or tracked issue. |
| **Weekly** | `docs(signal): reconcile signal support claims with tests` | `FEATURE_STATUS.toml` states whether behavior is model-only, syscall-integrated, QEMU-tested, or hardware-tested. |
| **Monthly** | `audit(signal): review frame construction and return validation` | Review handler address bounds, stack canonicality, flags, registers, masks, nested delivery, and assembly stubs. |
| **Monthly** | `test(signal): run syscall interruption and restart matrix` | Results for blocking I/O, wait, sleep/timers, `EINTR`, partial completion, and `SA_RESTART`. |
| **Monthly** | `security(signal): review target authorization and denial-of-service limits` | Permission checks, signal storms, queue caps, cross-process isolation, frame forgery protection. |
| **Quarterly** | `docs(signal): review POSIX and Linux/BSD differences` | Compare implemented subset against POSIX.1-2024, Linux, and FreeBSD; record deviations. |
| **Quarterly** | `test(signal): run architecture and process-lifecycle matrix` | Handler delivery/return, masks, child events, task exit, invalid frame rejection across target architectures. |
| **Each RC** | `release(signal): certify signal-handler support claims` | Target/revision, signal APIs certified, handler test results, known gaps, security findings. |
| **Post-Incident**| `fix(signal): prevent recurrence of bug` | Root cause analysis, reproducer, regression test added, backport decision recorded. |

---

## PR Review and Merge Gates

All PRs introducing or modifying signal handling code must include the following pull request review checklist:

```text
## Summary
What signal-handler behavior or correctness gap does this PR address?

## Scope
Disposition | mask/pending | delivery frame | sigreturn | libc | signalfd
State what is intentionally not implemented.

## Compatibility Profile
POSIX subset | Linux-inspired extension | BSD-inspired behavior | SigmaOS-specific
List exact semantics and deviations.

## Security and Failure Behavior
User-pointer handling, handler/frame validation, privilege state, authorization,
queue limits, syscall interruption, and cleanup.

## Runtime Evidence
Unit/model | Ring 3 QEMU | SMP QEMU | physical hardware
List architecture, boot configuration, signal, and observable result.

## Tests
Normal handler return, default action, masking, nested delivery, interruption,
invalid pointers/frame, task exit, signal flood, and anything not tested.

## Documentation and Status
Proposed | Prototype | Integrated | Supported
Link ABI docs, feature inventory, examples, and tests.

## Checklist
- [ ] `sigaction` semantics and ABI are explicit
- [ ] SIGKILL/SIGSTOP cannot be blocked or caught
- [ ] Frame/return state is validated
- [ ] Handler path does not trust user-controlled privileged state
- [ ] Async-signal-safe guidance is documented
- [ ] Required CI passes; review conversations resolved
```

### Reviewer Gate Requirements
- **1 Maintainer Approval**: Required for documentation updates, unit test additions, or libc wrapper cleanups.
- **2 Qualified Maintainer Approvals**: Required for signal frame assembly, `sigreturn` validation, syscall ABI dispatch, privilege boundary checks, or architecture return paths.

---

## Completion Criteria Tiers

- **Prototype Level**: Signal state structures, masks, and frame construction models are unit-tested in isolation (`rustc --test`), but no user-space process executes a handler in Ring 3.
- **Integrated Level**: On a named QEMU boot target (e.g., x86_64 QEMU virt), a real Ring 3 user process receives a signal, enters a registered handler callback via a valid kernel-constructed frame, returns through the validated `sigreturn` trampoline, and resumes normal execution or exits according to policy.
- **Supported Level**: Automated, reproducible test suites verify dispositions, masks, pending queues, process/thread targeting, syscall interruption (`EINTR`/`SA_RESTART`), malformed frame rejection, permission checks, queue caps, and multi-architecture targets without regressions or panics.
