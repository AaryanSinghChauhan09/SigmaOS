# SigmaOS Signal Handler Master Architecture & Development Plan

## Overview
SigmaOS provides a POSIX.1-2024 compliant signal handling subsystem designed for precise asynchronous event notification, execution state protection, and safe user-space handler dispatch. Signal handling in SigmaOS distinguishes between process-directed and thread-directed signals, protects execution state across architecture-specific user frames, enforces kernel-level uncatchable signal rules (`SIGKILL` and `SIGSTOP`), and supports restartable system calls via `SA_RESTART`.

## Required Semantics and Safety Rules

### Default to `sigaction` Semantics
SigmaOS adopts POSIX.1-2024 `sigaction()` as the standard for signal management. The `signal()` function is maintained purely as a libc compatibility wrapper that delegates to `sigaction()`. POSIX recommends `sigaction()` for explicit control of handler behavior, including flags (`SA_RESTART`, `SA_NODEFER`, `SA_RESETHAND`, `SA_SIGINFO`, `SA_ONSTACK`).

### Distinguish Process- and Thread-Directed Signals
Process dispositions and process-pending signal queues are maintained independently from thread-specific blocked signal masks (`sigset_t`) and thread-pending signal queues. Signal delivery selects an eligible thread (unblocked for the signal) at a safe delivery boundary (e.g., return from syscall, interrupt, or context switch).

### Protect Handler Execution State
On signal delivery to user-mode (Ring 3), the kernel saves the interrupted register context, current signal mask, and alternate stack state into an architecture-specific `UserSignalFrame` constructed on the user stack. Return from the handler is controlled through a validated signal-return trampoline (`sigreturn`), which restores the saved register context and signal mask after validating alignment and permissions.

### Enforce Uncatchable Signals in Kernel
`SIGKILL` (9) and `SIGSTOP` (19) must not be caught, ignored, or blocked. Any attempt to modify the disposition or mask for `SIGKILL` or `SIGSTOP` via `sigaction()` or `sigprocmask()` is rejected by the kernel with `EINVAL`.

### Respect Masks and Nesting
The delivered signal is automatically added to the thread's blocked signal mask during handler execution unless `SA_NODEFER` is set. Additional signals specified in `sa_mask` are also blocked while the handler runs. Upon normal return via `sigreturn`, the thread's previous signal mask is restored.

### Define Syscall Interruption Behavior
Blocking system calls (such as `read`, `write`, `waitpid`, `nanosleep`) return `EINTR` when interrupted by a caught signal, unless `SA_RESTART` is specified in `sa_flags` for supported restartable calls.

### Minimal and Auditable Handler-Path Behavior
Kernel signal frame construction and `sigreturn` validation execute without heap allocations, bounded execution time, and zero trust in user-modified frame state. User signal handlers must observe async-signal-safety guidelines; unsafe runtime routines (such as standard I/O) are prohibited inside signal handlers.

### No False Status Claims
Signal frame layout definitions, mock frame structures, or standalone unit tests represent Prototype evidence. A signal handler is classified as Integrated only when a real Ring 3 user process receives a signal, executes a user handler, and returns safely via `sigreturn` under QEMU or physical hardware.

## Proposed PR Sequence Roadmap

| PR | Title | Scope & Objectives | Acceptance Criteria |
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

## Periodic Maintenance PR Cadence

- **Every PR**: Run ABI, frame, and handler-path checks.
- **Weekly**: Triage handler crashes and delivery regressions; reconcile signal support claims in documentation.
- **Monthly**: Review frame construction and `sigreturn` validation rules; execute syscall interruption and restart test matrix; review signal permission boundaries and queue limits.
- **Quarterly**: Review POSIX.1-2024 and Linux/FreeBSD comparative deviations; execute architecture and process-lifecycle test matrix.
- **Each Release Candidate**: Certify signal-handler support claims against target revisions.
- **After Each Incident**: Post-mortem, regression test, and backport fixes for signal frame or delivery bugs.

## PR Review and Merge Gates

- All PRs affecting signal frame structures or `sigreturn` trampoline assembly require two maintainer approvals.
- Enforce uncatchable signal protection checks (`SIGKILL`/`SIGSTOP`).
- Verify zero raw memory corruption risks and validate canonical Ring 3 addresses.
- Require clean execution of `./run_sigma_tests.sh`.

## Completion Criteria

- **Prototype**: Signal structures and frame layouts unit-tested without Ring 3 execution.
- **Integrated**: A real Ring 3 process receives a signal, enters a handler, returns through `sigreturn`, and resumes execution.
- **Supported**: Full suite of automated tests for process/thread targeting, masks, signal queueing, `SA_RESTART`, and invalid frame rejections across all supported architectures.
