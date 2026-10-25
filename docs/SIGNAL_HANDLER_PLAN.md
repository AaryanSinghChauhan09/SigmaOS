# SigmaOS Signal Handler Master Architecture & Development Plan

## Overview & Context

SigmaOS signal handling provides user-space asynchronous signal notification, context preservation, stack frame construction, and signal trampoline return mechanisms based on POSIX.1-2024 standards with explicit Linux and FreeBSD reference semantics.

### Signal Subsystem vs. Signal Handler
- **Signal Subsystem**: Responsible for signal generation, queuing, masking, selecting, target thread/process routing, and default kernel-level actions (e.g., terminate, core dump, stop, continue, ignore).
- **Signal Handler**: The user-space callback routine (`sa_handler` or `sa_sigaction`) entered via an architecture-specific user stack frame constructed by the kernel and returned from through a verified trampoline (`sigreturn`).

## Required Semantics and Safety Rules

1. **Default to `sigaction` Semantics**: `sigaction()` provides explicit control over handler flags, masks, and arguments. The legacy `signal()` API is exposed strictly as a libc compatibility wrapper.
2. **Distinguish Process- and Thread-Directed Signals**: Process dispositions are shared across thread groups, while thread masks and thread-pending signals are stored per-thread. Signals directed to a process are delivered to an eligible unblocked thread at a safe delivery point.
3. **Protect Handler Execution State**: On delivery, the kernel constructs an x86_64 user stack frame containing saved CPU registers, instruction/stack pointers, signal number, `siginfo_t`, and saved signal mask (`sigset_t`). State restoration occurs strictly through validated `sigreturn` syscall dispatch.
4. **Enforce Uncatchable Signals**: `SIGKILL` (9) and `SIGSTOP` (19) cannot be caught, ignored, blocked, or intercepted by user code.
5. **Respect Masks and Nesting**: Delivered signals are automatically masked during handler execution unless `SA_NODEFER` is specified. Signals in `sa_mask` are blocked while the handler runs and restored upon `sigreturn`.
6. **Define Interruption Behavior**: Interrupted blocking syscalls return `-EINTR` unless `SA_RESTART` is specified for eligible restarted syscalls (e.g., `read`, `write`, `waitpid`).
7. **Minimal and Auditable Kernel Handler Path**: The kernel delivery and return paths operate without heap allocation, do not sleep in atomic contexts, and do not trust user-modified privileged registers (such as CS/SS or kernel flags).
8. **Honest Runtime Status Claims**: Features are labeled Prototype until real Ring 3 execution, user frame entry, and `sigreturn` trampoline completion are demonstrated.

## Proposed 18-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(signal-handler): audit signal path and define ABI profile` | Audit signal constants, `sigaction`, masks, pending state, syscall dispatch, process/thread model, x86_64 return path, libc, and `signalfd`. Document supported POSIX subset and architecture limitations. | Each feature links to code, syscall dispatch, tests, and runtime evidence; compatibility claims are explicit; unsupported flags and signals are listed. |
| 2 | `test(signal-handler): add ABI and signal-frame layout checks` | Add compile-time/runtime checks for signal numbers, `sigset_t`, `sigaction`, `siginfo_t`, context layouts, stack alignment, frame offsets, and trampoline ABI. | Layout tests match the selected architecture ABI; incompatible field/number changes fail CI; tests distinguish compile-time layout evidence from runtime delivery. |
| 3 | `feat(signal): connect signal syscalls to kernel dispatch` | Wire `sigaction`, signal mask, `kill`/`raise`, pending query, and signal-return syscall numbers to real handlers using safe user-copy APIs. | Ring 3 process can set/query a disposition, block/unblock a signal, and send to an authorized target; invalid pointers and signals fail safely; uncatchable restrictions are enforced. |
| 4 | `feat(signal): add per-thread masks and pending signal queues` | Separate process dispositions and process-pending signals from thread masks and thread-directed pending signals; define standard-signal coalescing policy. | Tests cover pending/blocked/unblocked transitions, thread selection, process/thread targeting, exit cleanup, and no duplicate delivery; semantics match the documented subset. |
| 5 | `feat(signal-handler): deliver default actions at safe return points` | Deliver default ignore/terminate/stop/continue actions at syscall, exception, or scheduler return-to-user points. | Real user process terminates/stops/continues according to documented actions; delivery never occurs while kernel state is inconsistent; `SIGKILL`/`SIGSTOP` remain unmaskable. |
| 6 | `feat(signal-handler): construct validated x86_64 user signal frame` | Build a frame with interrupted registers, instruction/stack pointers, prior mask, signal number, and return trampoline; redirect user execution to the handler. | QEMU Ring 3 test receives a caught signal, observes the expected argument, and returns to the interrupted instruction with state restored; malformed or unmapped stack fails safely. |
| 7 | `feat(signal-handler): implement x86_64 sigreturn validation` | Implement signal-return syscall/trampoline handling; validate user frame address, canonical user addresses, flags, stack pointer, instruction pointer, and allowed register state. | Handler return restores user state and mask; forged frames cannot enter kernel mode or restore privileged flags; invalid frames terminate or signal the process according to documented policy. |
| 8 | `feat(signal-handler): implement action masks and restart semantics` | Apply `sa_mask`, automatic self-blocking, `SA_NODEFER`, `SA_RESETHAND`, and a carefully scoped `SA_RESTART` subset. | Nested handler tests verify masks; actions remain installed/reset as specified; interrupted `read`/`wait` tests verify `EINTR` and restart behavior against the published contract. |
| 9 | `feat(signal-handler): add siginfo and SA_SIGINFO subset` | Add typed signal information for supported kernel-generated events and three-argument handlers. | `siginfo_t` fields are initialized and bounded; source/code values are documented; user-provided signal payloads cannot smuggle pointers or privileged data. |
| 10 | `feat(signal-handler): add alternate stack support` | Add `sigaltstack` and `SA_ONSTACK` if stack mappings and guard pages are available. | Valid alternate stack receives eligible handlers; nested/disabled/too-small stack cases have defined behavior; return restores original stack settings. |
| 11 | `feat(signal): integrate SIGCHLD and wait/reap lifecycle` | Generate child status signals for supported exit/stop/continue transitions and integrate with wait/reap. | Tests cover child exit, multiple child events, handler races with `waitpid`, and `SA_NOCLDSTOP` only if implemented; child status is not lost. |
| 12 | `feat(signal): add synchronous signal-wait APIs` | Implement a staged `sigwait`/`sigtimedwait` or equivalent synchronous acceptance path, avoiding async handlers where callers prefer event-loop behavior. | Blocked signals can be synchronously accepted with correct metadata and timeout; interactions with `sigaction` on the same signal are documented. |
| 13 | `feat(signal): add queued real-time signal support with limits` | Add real-time signal range and FIFO payload queue only after standard signal delivery is stable. | Queued instances preserve order and payload; queue limits are enforced; overflow and process exit release all queue memory. |
| 14 | `feat(signal): integrate signalfd as Linux-specific extension` | Connect the existing `signalfd` module to real pending state, signal masks, poll/read, FD lifecycle, and process permissions. | Ring 3 app blocks a signal and reads it through the descriptor; close, readiness, and mask changes work; feature is labeled Linux-inspired, not POSIX. |
| 15 | `feat(libc): expose signal APIs and safe handler examples` | Implement `signal.h` constants/types and libc wrappers; provide examples that use async-signal-safe behavior or synchronous waiting. | C and Rust userspace tests verify layouts, return values, errno, handler arguments, and signal return; examples do not call unsafe functions such as `printf()` from handlers. |
| 16 | `test(signal-handler): add fault-injection and concurrency suite` | Test nested signals, delivery during syscalls, task exit, mask changes, frame corruption, invalid handler addresses, repeated signals, and queue exhaustion. | Reproducible QEMU tests pass; no kernel panic, privilege escape, lost pending signal, use-after-free, or unbounded allocation; failure artifacts include task and signal state. |
| 17 | `feat(signal-handler): validate AArch64 and RISC-V handler frames` | Add architecture-specific frame/trampoline/return paths as independent PRs once each architecture has real user-mode execution. | Each architecture separately passes handler, mask restoration, signal-return validation, and invalid-frame tests; x86_64 results do not imply cross-architecture support. |
| 18 | `docs(signal-handler): publish API, safety, and support matrix` | Document signals, actions, handler arguments, async-signal-safe expectations, interruption/restart, architecture support, and known gaps. | Every claim links to a test and exact target; POSIX/Linux/BSD differences are stated; handler safety guidance is prominent. |

> **Priority Focus**: Complete PRs **1–8** before progressing to alternate stacks, real-time queues, or `signalfd` extensions.

## Periodic Maintenance Cadence

| Cadence | Recurring Task / PR | Required Evidence |
|---|---|---|
| Every PR | `ci(signal): run ABI, frame, and handler-path checks` | Build, ABI layout tests, signal unit tests, and QEMU tests for changed paths. |
| Weekly | `test(signal): triage handler crashes and delivery regressions` | Minimized reproducer, target signal/arch, owner, and regression test. |
| Weekly | `docs(signal): reconcile signal support claims with tests` | Feature inventory update reflecting exact level (Prototype, Integrated, Supported). |
| Monthly | `audit(signal): review frame construction and return validation` | Audit user stack frame boundaries, register safety, canonical addresses, and `sigreturn`. |
| Monthly | `test(signal): run syscall interruption and restart matrix` | Results for blocking I/O, wait, sleep, `EINTR`, and `SA_RESTART`. |
| Monthly | `security(signal): review target authorization and DoS limits` | Review process permission checks, signal flood limits, queue caps, and forgery guards. |
| Quarterly | `docs(signal): review POSIX and Linux/BSD differences` | Update compatibility matrices against POSIX.1-2024 and Linux/FreeBSD reference behavior. |
| Quarterly | `test(signal): run architecture and process-lifecycle matrix` | Validate delivery, return, child events, and exit cleanup across active architectures. |
| Each RC | `release(signal): certify signal-handler support claims` | Test suite certification report and known limitations update. |
| After Incident | `fix(signal): prevent recurrence of signal handling bug` | Root cause post-mortem, regression test addition, and security advisory. |

## PR Review and Merge Gates

All signal-handler PRs must complete the following standard checklist:
- [ ] `sigaction` semantics and ABI are explicit.
- [ ] `SIGKILL` and `SIGSTOP` cannot be blocked, caught, or ignored.
- [ ] Frame construction and return state are validated for user space boundaries.
- [ ] Handler path does not trust user-controlled privileged register state.
- [ ] Async-signal-safe guidelines are documented for example handlers.
- [ ] All required CI unit tests and integration tests pass.

## Completion Criteria Tiers

- **Prototype**: Signal state or handler frames are modeled/unit-tested in host tests, but no real user process handler runs.
- **Integrated**: On a named boot target (e.g. x86_64 QEMU), a Ring 3 user process receives a signal, enters a handler, returns through a validated trampoline, and resumes execution cleanly.
- **Supported**: Repeatable tests cover actions, masks, pending signals, thread routing, interruption/restart, frame validation, permission gates, queue caps, and all advertised hardware targets.
