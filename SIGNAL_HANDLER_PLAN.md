# SigmaOS Signal Handler Master Architecture, Support Matrix, and PR Plan

## Executive Summary

SigmaOS implements a signal handling architecture designed for safe, deterministic, and POSIX.1-2024 compliant asynchronous event notification across user processes and threads. Signal handling in modern operating systems provides process notifications, error reporting (such as arithmetic exceptions and memory faults), job control, and inter-process communication.

A key distinction governs this architectural plan:
- **Signal Subsystem:** The kernel-side mechanisms responsible for generating, queuing, masking, selecting, routing, and delivering signals to target processes or threads.
- **Signal Handler:** The user-space callback entered via an architecture-specific user signal frame constructed by the kernel, and exited through a validated signal-return (`sigreturn`) trampoline.

Implementing `sigaction()` alone does not deliver a signal handler; complete handler execution requires end-to-end user frame construction, CPU context saved on the user stack, trampoline execution, and kernel frame validation during signal return.

---

## Required Semantics and Safety Rules

1. **Default to `sigaction` Semantics:** `signal()` is implemented strictly as a libc compatibility wrapper over `sigaction()`. POSIX.1-2024 recommends `sigaction()` for explicit control over signal masks, flags, and disposition parameters.
2. **Distinguish Process- and Thread-Directed Signals:** Process dispositions (`sigaction` tables) are stored per-process, while blocked signal masks (`sigprocmask`) and pending signal sets are maintained per-thread. Process-directed signals (e.g., `kill(pid, sig)`) are delivered to any eligible thread whose mask does not block the signal at a safe delivery point; thread-directed signals (e.g., `pthread_kill(tid, sig)` or synchronous CPU exceptions like `SIGSEGV`/`SIGFPE`) are queued in the target thread's pending set and delivered specifically to that thread.
3. **Protect Handler Execution State:** On signal delivery, the kernel constructs an x86_64 user signal frame containing interrupted CPU registers, instruction pointer (`rip`), stack pointer (`rsp`), saved signal mask, and alternate stack state. Control returns to the kernel strictly through a validated signal-return trampoline (`sigreturn`), which verifies canonical addresses and frame integrity before restoring execution context.
4. **Enforce Uncatchable Signals in Kernel:** `SIGKILL` (signal 9) and `SIGSTOP` (signal 19) cannot be caught, ignored, or blocked under any circumstances. The kernel rejects attempts to modify their dispositions or include them in blocked signal masks (`sigprocmask` / `pthread_sigmask`).
5. **Respect Masks and Nesting:** The delivered signal is automatically added to the thread's signal mask during handler execution unless `SA_NODEFER` is set. Additional signals specified in `sa_mask` are also blocked. The saved mask is restored upon return via `sigreturn`.
6. **Define Syscall Interruption Behavior:** Blocking system calls interrupted by a signal return `EINTR` or restart automatically if `SA_RESTART` is specified in `sa_flags`. The interruption contract is defined per syscall.
7. **Minimal and Auditable Handler Path:** The kernel signal delivery and return paths operate without dynamic memory allocations, never sleep in invalid contexts, and never trust handler-modified privileged CPU flags or addresses. User handlers must obey async-signal-safety requirements.
8. **Truth in Status Claims:** No feature is marked "Integrated" or "Supported" based solely on unit tests, struct definitions, or synthetic models. Integration requires real Ring 3 execution, handler entry, trampoline execution, and safe context restoration.

---

## Proposed 18-PR Implementation Sequence

| PR | Title | Scope | Acceptance Criteria |
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

**Implementation Priority:** Complete PRs **1–8** before attempting alternate stacks, real-time signals, or `signalfd`.

---

## Periodic Maintenance Cadence

| Cadence | Task | Required Evidence |
|---|---|---|
| Every PR | `ci(signal): run ABI, frame, and handler-path checks` | Build, ABI layout tests, signal unit tests, and QEMU tests for changed paths. |
| Weekly | `test(signal): triage handler crashes and delivery regressions` | Minimized reproducer, affected signal/arch, owner, severity, regression test or tracked issue. |
| Weekly | `docs(signal): reconcile signal support claims with tests` | `FEATURE_STATUS.toml` accurately states whether behavior is modeled, integrated, or supported. |
| Monthly | `audit(signal): review frame construction and return validation` | Audit user stack validation, instruction pointer sanity, register filtering, and mask restoration. |
| Monthly | `test(signal): run syscall interruption and restart matrix` | Automated test suite results for blocking I/O, wait, sleep/timers, `EINTR`, and `SA_RESTART`. |
| Monthly | `security(signal): review target authorization and DoS limits` | Review process permission checks, queue bounds, frame forgery rejection, and signal storms. |
| Quarterly | `docs(signal): review POSIX and Linux/BSD differences` | Update documented API against POSIX.1-2024 and reference Linux/BSD implementations. |
| Quarterly | `test(signal): run architecture and process-lifecycle matrix` | Validate handler delivery, return paths, child notifications, and task exit across supported targets. |
| Each RC | `release(signal): certify signal-handler support claims` | Certified target list, passing test logs, known limitations, and release documentation. |
| After Incident | `fix(signal): prevent recurrence of handler/return bug` | Root cause analysis, regression test case, affected platforms, and backport plan. |

---

## PR Review and Merge Gates

All signal-handler PRs must complete the following template checklist prior to maintainer review:

```text
## Summary
What signal-handler behavior or correctness gap does this PR address?

## Scope
Disposition | mask/pending | delivery frame | sigreturn | libc | signalfd
State what is intentionally not implemented.

## Compatibility Profile
POSIX subset | Linux-inspired extension | BSD-inspired behavior | SigmaOS-specific
List the exact semantics and deviations.

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
- [ ] sigaction semantics and ABI are explicit
- [ ] SIGKILL/SIGSTOP cannot be blocked or caught
- [ ] Frame/return state is validated
- [ ] Handler path does not trust user-controlled privileged state
- [ ] Async-signal-safe guidance is documented
- [ ] Required CI passes; review conversations resolved
```

### Review Requirements
- Standard tests and documentation changes require approval from **1 maintainer**.
- Assembly frame layout, `sigreturn` validation, privilege checks, or register context modifications require approval from **2 qualified reviewers**.

---

## Completion Criteria Tiers

- **Prototype:** Signal structures, dispositions, masks, and frame construction are modeled or unit-tested in isolation, but no real Ring 3 process executes a handler.
- **Integrated:** On a specified boot target (e.g. x86_64 QEMU), an actual Ring 3 process receives a signal, enters a user-space handler, executes a return trampoline, and safely resumes or terminates execution.
- **Supported:** Comprehensive, reproducible automated tests pass across all target architectures covering signal actions, masks, nested signals, interruption/restart, frame validation, permission checks, and queue bounds.
