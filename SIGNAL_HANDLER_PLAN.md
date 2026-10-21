# SigmaOS Signal Handler Architecture & Development Plan

## Subsystem Overview

SigmaOS has signal-related code, including root-level `signal` and `signalfd` components, and its feature inventory describes the syscall ABI as **partial**: it has syscall numbers and libc wrappers, but still needs kernel-side dispatch and real Ring 3 testing. The inventory's process manager also still needs real process spawning. This plan treats signal handling as unintegrated until a real user process receives a signal and returns safely.

The key distinction: **signal subsystem** means generating, queuing, masking, selecting, and delivering signals; **signal handler** means the user-space callback entered by the architecture-specific signal frame and returned from through a trusted trampoline. Implementing `sigaction()` alone does not deliver a handler. POSIX.1-2024 specifies the `sigaction()` contract, while Linux and FreeBSD provide useful implementation references; SigmaOS documents its supported POSIX subset and intentional differences rather than assuming full compatibility.

---

## Required Semantics and Safety Rules

- **Default to `sigaction` semantics.** Make `signal()` a libc compatibility wrapper; POSIX recommends `sigaction()` for explicit control of handler behavior.
- **Distinguish process- and thread-directed signals.** Store process dispositions separately from each thread's blocked mask and pending state; select an eligible thread only at a safe delivery point.
- **Protect handler execution state.** On delivery, save the interrupted context, signal mask, and any supported alternate-stack state in a user frame; restore them only through a validated signal-return path.
- **Enforce uncatchable signals in the kernel.** `SIGKILL` and `SIGSTOP` must not be caught, ignored, or blocked.
- **Respect masks and nesting.** Block the delivered signal during its handler unless `SA_NODEFER` applies; add `sa_mask` while the handler runs; restore the previous mask on normal return.
- **Define interruption behavior.** For each blocking syscall, specify whether a signal causes partial success, `EINTR`, or restart under `SA_RESTART`.
- **Make handler-path behavior minimal and auditable.** The kernel's delivery and return path must not allocate without bounds, sleep in invalid contexts, or trust handler-modified privileged state. User handlers must follow async-signal-safety rules; many library functions, including stdio routines, are unsafe in handlers.
- **No false status claims.** A signal frame model, unit test, or libc stub is Prototype-level evidence, not an Integrated signal handler.

---

## Proposed PR Sequence Roadmap

| PR | Title | Scope | Acceptance criteria |
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

**Priority:** Complete PRs **1–8** before adding alternate stacks, real-time signals, or Linux-specific `signalfd`.

---

## Periodic Maintenance PR Cadence

| Cadence | Proposed recurring PR/task | Required evidence |
|---|---|---|
| Every PR | `ci(signal): run ABI, frame, and handler-path checks` | Build, ABI-layout tests, signal unit tests, and QEMU tests for changed paths; skipped tests explained. |
| Weekly | `test(signal): triage handler crashes and delivery regressions` | Minimized reproducer, affected signal/architecture, owner, severity, and regression test or tracked issue. |
| Weekly | `docs(signal): reconcile signal support claims with tests` | Feature inventory states whether behavior is model-only, syscall-integrated, QEMU-tested, or hardware-tested. |
| Monthly | `audit(signal): review frame construction and return validation` | Review handler address, frame bounds, stack, flags, registers, masks, nested delivery, and architecture-specific assembly. |
| Monthly | `test(signal): run syscall interruption and restart matrix` | Results for blocking I/O, wait, sleep/timers, `EINTR`, partial completion, and `SA_RESTART`. |
| Monthly | `security(signal): review target authorization and denial-of-service limits` | Review permission checks, signal storms, queue caps, cross-process information leakage, and frame-forgery cases. |
| Quarterly | `docs(signal): review POSIX and Linux/BSD differences` | Check the documented API against the chosen POSIX edition and selected Linux/FreeBSD references; record deviations. |
| Quarterly | `test(signal): run architecture and process-lifecycle matrix` | Handler delivery/return, masks, child events, task exit, and invalid frames on each architecture claimed. |
| Each release candidate | `release(signal): certify signal-handler support claims` | Exact target/revision, signal APIs, handler tests, known gaps, security findings, and release notes. |
| After each incident | `fix(signal): prevent recurrence of <handler or return-path bug>` | Root cause, reproducer, regression test, affected versions/architectures, and backport decision. |

---

## PR Review and Merge Gates

Use this template for signal-handler PRs:

```text
## Summary
What signal-handler behavior or correctness gap does this PR address?

## Scope
Disposition | mask/pending | delivery frame | sigreturn | libc | signalfd
State what is intentionally not implemented.

## Compatibility profile
POSIX subset | Linux-inspired extension | BSD-inspired behavior | SigmaOS-specific
List the exact semantics and deviations.

## Security and failure behavior
User-pointer handling, handler/frame validation, privilege state, authorization,
queue limits, syscall interruption, and cleanup.

## Runtime evidence
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

---

## Completion Criteria Tiers

- **Prototype:** signal state or handler frames are modeled/unit-tested but no real user handler runs.
- **Integrated:** on a named boot target, an actual user process receives a signal, enters a handler, returns through the validated trampoline, and resumes or exits as documented.
- **Supported:** repeatable tests cover actions, masks, pending signals, process/thread targeting, interrupt/restart behavior, malformed frames, permission checks, queue limits, and every architecture advertised.
