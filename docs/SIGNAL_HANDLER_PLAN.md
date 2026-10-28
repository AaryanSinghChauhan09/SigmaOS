# SigmaOS Signal Handler Master Architecture and Plan

SigmaOS signal handling provides standard POSIX.1-2024 signal semantics, thread-directed delivery, validated user signal frames, and kernel-enforced safety rules.

## Core Terminology & Boundary Definitions

- **Signal Subsystem**: Responsible for signal generation, queuing, masking, target selection (process vs thread), and signal delivery state transitions.
- **Signal Handler**: The Ring 3 user-space callback executed via an architecture-specific signal frame and returned from using a kernel-validated signal return trampoline (`sigreturn`).

Implementing `sigaction()` alone establishes dispositions but does not deliver handlers; delivery requires kernel-side stack frame construction, context switches, register state saving, and trampoline verification.

## Required Semantics and Safety Rules

1. **Default to `sigaction` Semantics**: `signal()` is a libc compatibility wrapper over `sigaction()`. POSIX recommends `sigaction()` for explicit control of handler behavior.
2. **Distinguish Process- and Thread-directed Signals**: Store process dispositions separately from each thread's blocked mask and pending state. Select eligible threads at safe delivery points (syscall, interrupt, or scheduler return-to-user).
3. **Protect Handler Execution State**: On delivery, construct a stack frame with interrupted registers, instruction pointer, stack pointer, signal mask, and siginfo. Restore context only through a validated `sigreturn` trampoline.
4. **Enforce Uncatchable Signals in Kernel**: `SIGKILL` (9) and `SIGSTOP` (19) must never be caught, ignored, or blocked. The kernel strips these signals from user-supplied masks.
5. **Respect Masks and Nesting**: Block the delivered signal during handler execution unless `SA_NODEFER` is set; add `sa_mask` during handler execution; restore the original mask on normal return.
6. **Define Interruption Behavior**: Specify whether blocking system calls return `EINTR` or restart automatically under `SA_RESTART`.
7. **Minimal and Auditable Handler-Path Behavior**: Kernel delivery and return paths must not allocate without bounds, sleep in invalid contexts, or trust user-modified privileged state. User handlers must obey async-signal-safety rules.
8. **No False Status Claims**: Model-only or prototype frame builders represent Prototype evidence, not Integrated signal delivery.

## Proposed PR Sequence

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(signal-handler): audit signal path and define ABI profile` | Audit signal constants, `sigaction`, masks, pending state, syscall dispatch, process/thread model, x86_64 return path, libc, and `signalfd`. | Each feature links to code, syscall dispatch, tests, and runtime evidence; compatibility claims are explicit. |
| 2 | `test(signal-handler): add ABI and signal-frame layout checks` | Add compile-time and runtime checks for signal numbers, `sigset_t`, `sigaction`, `siginfo_t`, context layouts, and stack alignment. | Layout tests match target ABI; incompatible field/number changes fail CI; tests distinguish compile-time layout from runtime delivery. |
| 3 | `feat(signal): connect signal syscalls to kernel dispatch` | Wire `sigaction`, signal mask, `kill`/`raise`, pending query, and `sigreturn` syscall numbers to real handlers using safe user-copy APIs. | Ring 3 process can set/query disposition, block/unblock signals, and send to authorized target; invalid pointers and signals fail safely. |
| 4 | `feat(signal): add per-thread masks and pending signal queues` | Separate process dispositions and process-pending signals from thread masks and thread-directed pending signals. | Tests cover pending/blocked/unblocked transitions, thread selection, process/thread targeting, and exit cleanup. |
| 5 | `feat(signal-handler): deliver default actions at safe return points` | Deliver default ignore/terminate/stop/continue actions at syscall, exception, or scheduler return-to-user points. | Real user process terminates/stops/continues according to documented actions; delivery never occurs while kernel state is inconsistent. |
| 6 | `feat(signal-handler): construct validated x86_64 user signal frame` | Build a frame with interrupted registers, instruction/stack pointers, prior mask, signal number, and return trampoline. | QEMU Ring 3 test receives caught signal, observes expected arguments, and returns to interrupted instruction; malformed stack fails safely. |
| 7 | `feat(signal-handler): implement x86_64 sigreturn validation` | Implement signal-return syscall/trampoline handling; validate user frame address, canonical addresses, flags, stack pointer, and register state. | Handler return restores user state and mask; forged frames cannot enter kernel mode or restore privileged flags. |
| 8 | `feat(signal-handler): implement action masks and restart semantics` | Apply `sa_mask`, automatic self-blocking, `SA_NODEFER`, `SA_RESETHAND`, and scoped `SA_RESTART` subset. | Nested handler tests verify masks; actions remain installed/reset as specified; interrupted `read`/`wait` tests verify `EINTR` and restart. |
| 9 | `feat(signal-handler): add siginfo and SA_SIGINFO subset` | Add typed signal information for supported kernel-generated events and three-argument handlers. | `siginfo_t` fields are initialized and bounded; source/code values are documented; user payloads cannot smuggle pointers or privileged data. |
| 10 | `feat(signal-handler): add alternate stack support` | Add `sigaltstack` and `SA_ONSTACK` support when stack mappings and guard pages are available. | Valid alternate stack receives eligible handlers; nested/disabled/too-small stack cases have defined behavior; return restores stack settings. |
| 11 | `feat(signal): integrate SIGCHLD and wait/reap lifecycle` | Generate child status signals for exit/stop/continue transitions and integrate with wait/reap. | Tests cover child exit, multiple child events, handler races with `waitpid`, and `SA_NOCLDSTOP`; child status is preserved. |
| 12 | `feat(signal): add synchronous signal-wait APIs` | Implement `sigwait`/`sigtimedwait` synchronous acceptance path, avoiding async handlers where callers prefer event-loop behavior. | Blocked signals can be synchronously accepted with correct metadata and timeout; interactions with `sigaction` are documented. |
| 13 | `feat(signal): add queued real-time signal support with limits` | Add real-time signal range (32..64) and FIFO payload queue only after standard signal delivery is stable. | Queued instances preserve order and payload; queue limits are enforced; overflow and process exit release queue memory. |
| 14 | `feat(signal): integrate signalfd as Linux-specific extension` | Connect `signalfd` module to real pending state, signal masks, poll/read, FD lifecycle, and process permissions. | Ring 3 app blocks signal and reads it through descriptor; close, readiness, and mask changes work; labeled Linux-inspired extension. |
| 15 | `feat(libc): expose signal APIs and safe handler examples` | Implement `signal.h` constants/types and libc wrappers; provide examples using async-signal-safe behavior or synchronous waiting. | C/Rust userspace tests verify layouts, return values, errno, handler arguments, and signal return; examples avoid unsafe functions like `printf()`. |
| 16 | `test(signal-handler): add fault-injection and concurrency suite` | Test nested signals, delivery during syscalls, task exit, mask changes, frame corruption, invalid handler addresses, and queue exhaustion. | Reproducible QEMU tests pass; no kernel panic, privilege escape, lost pending signal, use-after-free, or unbounded allocation. |
| 17 | `feat(signal-handler): validate AArch64 and RISC-V handler frames` | Add architecture-specific frame/trampoline/return paths as independent PRs once real user-mode execution is validated. | Each architecture separately passes handler, mask restoration, signal-return validation, and invalid-frame tests. |
| 18 | `docs(signal-handler): publish API, safety, and support matrix` | Document signals, actions, handler arguments, async-signal-safe expectations, interruption/restart, and support matrix. | Every claim links to a test and target; POSIX/Linux/BSD differences are stated; handler safety guidance is prominent. |

## Periodic Maintenance Cadence

- **Every PR**: Run ABI layout, frame, and handler-path unit tests.
- **Weekly**: Triage handler crashes, frame corruption, and delivery regressions.
- **Weekly**: Reconcile signal support claims in `FEATURE_STATUS.toml` with passing tests.
- **Monthly**: Review frame construction, trampoline addresses, stack boundaries, register restoration, and architecture assembly.
- **Monthly**: Run system call interruption (`EINTR`) and restart (`SA_RESTART`) matrix tests.
- **Monthly**: Audit permission checks, queue limits, signal floods, and frame forgery scenarios.
- **Quarterly**: Review POSIX.1-2024 compliance and document Linux/FreeBSD deviations.
- **Quarterly**: Run architecture and process-lifecycle matrix tests across all supported architectures.
- **Release Candidate**: Certify signal-handler support claims against hardware/QEMU boot results.
- **Incident Response**: Implement regression test and root-cause fix for any handler or return-path defect.

## Review and Merge Gates

All signal-handler pull requests must complete the PR checklist:
- [ ] `sigaction` semantics and ABI are explicit.
- [ ] `SIGKILL` and `SIGSTOP` cannot be blocked, caught, or ignored.
- [ ] User frame and `sigreturn` trampoline state are strictly validated.
- [ ] Handler delivery path does not trust user-controlled privileged state.
- [ ] Async-signal-safe execution guidance is documented.
- [ ] Mandatory CI checks pass and maintainer reviews are approved.

## Subsystem Maturity Model

- **Prototype**: Signal state or handler frames are modeled/unit-tested but no real user handler executes.
- **Integrated**: On a named boot target, a Ring 3 user process receives a signal, enters a handler, returns via the validated trampoline, and resumes execution.
- **Supported**: Repeatable test suite covers dispositions, masks, pending queues, process/thread routing, interruption/restart, malformed frames, permissions, resource limits, and all target architectures.
