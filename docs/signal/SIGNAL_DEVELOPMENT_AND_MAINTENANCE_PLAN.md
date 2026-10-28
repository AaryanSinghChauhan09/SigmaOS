# SigmaOS Signal Development and Maintenance Plan

## Executive Summary & Gap Analysis

SigmaOS already contains signal-related components in its repository—including `src/signal/sovereign_signal_development_and_pr_suite.rs`, `src/ipc/signals.rs`, `src/ipc/rt_signals.rs`, `src/compatibility/posix_signals.rs`, `src/kernel/signal.rs`, `src/kernel/proc/signals.rs`, `src/kernel/signalfd.rs`, `src/kernel/sigma_signal.rs`, `src/kernel/sigma_posix_signal_engine.rs`, `src/kernel/signal_dispatcher.rs`, and `src/kernel/sovereign_kernel_signal_interruption_suite.rs`—as well as entries in `FEATURE_STATUS.toml`. However, existing source code and unit tests do not prove end-to-end signal delivery in a booted user process. The project explicitly cautions that unit tests alone are insufficient to establish runtime integration.

This plan audits existing signal implementations and defines an evidence-gated, 19-PR sequence to close signal delivery, return-to-userspace, process, thread, and libc gaps.

---

## Baseline Semantics and Reference Behavior

The plan establishes POSIX.1-2024 as the portable API baseline and uses Linux and FreeBSD as behavior reference points (not as an assertion of complete ABI compatibility):

- **POSIX.1-2024 Preferred Interface:** `sigaction()` is the preferred interface for controlling signal dispositions. `SIGKILL` and `SIGSTOP` cannot be caught, ignored, or blocked in signal masks.
- **Linux/BSD Thread & Memory Reference:** Per-thread signal masks, process-directed vs. thread-directed pending signal sets, signal frame construction on user stacks, and return via sigreturn/trampoline mechanisms are modeled following standard OS conventions.

---

## Core Rules and Implementation Principles

1. **Asynchronous Control Flow Separation:** Keep signal generation, pending state management, selection, delivery, handler return, and default actions as distinct operations with clear ownership.
2. **Portable Semantics First:** Implement and test a small POSIX signal subset before adding Linux-specific real-time, signalfd, or BSD extensions. Mark extensions separately in documentation and capability matrices.
3. **Stable ABI Contract:** Signal numbers, `sigset_t`, `siginfo_t`, `sigaction`, signal-frame layout, syscall behavior, and libc wrapper semantics are userspace contracts. Never silently renumber or change a published layout.
4. **Safe User Memory Handling:** Copy action structures and masks from userspace exclusively via safe copy-in helpers; copy old actions or signal info back with safe copy-out helpers. Validate address range, alignment, and structure sizes.
5. **Permission Checking at Enqueue:** Verify target existence (`ESRCH`), process identity, credentials, and allowed signal semantics (`EPERM`, `EINVAL`) in kill-like APIs before modifying pending state.
6. **Uncatchable Signals Guard:** Enforce `SIGKILL` and `SIGSTOP` restrictions strictly in the kernel, regardless of libc or caller requests.
7. **Safe Handler Context Restoration:** Validate signal-frame location and return context upon `sigreturn`; never accept a forged frame that could restore privileged CPU registers, kernel addresses, or illegal flags.
8. **Documented Blocking Call Interruption:** Define when blocking syscalls return `EINTR`, restart under `SA_RESTART`, or complete partially.
9. **Evidence-Based Maturity:** Track each signal feature as Proposed, Prototype, Integrated, or Supported. "Integrated" requires real user-mode signal delivery and execution on a documented boot target.

---

## 19-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(signal): define signal model and supported profile` | Audit signal modules, syscalls, process/thread model, scheduler return paths, libc wrappers, shell/job-control use, and existing agent guidance. Publish POSIX baseline, Linux/BSD extensions policy, limitations, status, and ownership. | Inventory maps each signal feature to source, dispatch, tests, and runtime evidence; no "working" label implies end-to-end support without evidence. |
| 2 | `test(signal): add signal ABI and status inventory checks` | Add consistency checks for signal number definitions, names, action constants, masks, syscall IDs, libc wrappers, and docs. | CI catches duplicate/invalid signal numbers, incompatible structure layouts, missing wrappers, and stale documentation; reserved values are explicit. |
| 3 | `refactor(signal): separate generation pending selection and delivery` | Define typed signal state for per-process dispositions, per-thread masks/pending sets, process-directed pending set, and delivery result. Separate state transitions from syscall handling. | Unit tests cover enqueue, blocked signals, ignored dispositions, coalescing policy, and task exit; no delivery occurs from arbitrary interrupt context. |
| 4 | `feat(signal): implement signal sets and thread masks` | Implement set operations plus mask get/block/unblock operations and inheritance rules for process/thread creation. | Tests cover empty/full/add/delete/member operations, invalid signal bits, mask changes, pending visibility, and unmask-triggered delivery; SIGKILL/SIGSTOP cannot be blocked. |
| 5 | `feat(signal): implement sigaction with safe user-copy` | Implement query/install action, default/ignore/handler dispositions, action masks, and initial flags. | Invalid pointers and signal numbers fail safely; failed updates leave prior disposition unchanged; querying with null action works; uncatchable/unsupported signals follow documented rules. |
| 6 | `feat(signal): implement process-directed and thread-directed generation` | Add kill/raise and internal generation for faults, timers, and child state changes; distinguish process-directed from thread-directed signals. | Tests verify target selection only among eligible threads, permission failures, signal-zero checks if supported, invalid targets, and that failed sends do not enqueue. |
| 7 | `feat(signal): deliver default terminate stop and continue actions` | Implement baseline default actions, including termination, stop, continue, ignore, and core-dump placeholder behavior where core dumps are not yet implemented. | Tests cover default action and scheduler/process-state transitions; unimplemented core-dump behavior is documented and does not falsely claim a dump exists. |
| 8 | `feat(signal): deliver handlers through validated user signal frames` | At kernel-to-user return, construct a user signal frame containing interrupted context and saved mask; redirect execution to the handler and a controlled trampoline. | QEMU Ring 3 test runs a handler and resumes at interrupted instruction; corrupted, unmapped, or out-of-range frames terminate safely; architecture context restoration is tested. |
| 9 | `feat(signal): add sigreturn and context validation` | Implement architecture-specific signal-return path; validate frame version, user addresses, flags, privilege bits, and restored registers. | Handler return restores registers/mask/stack; forged or invalid context is rejected; no user-supplied state can restore kernel privilege or arbitrary privileged flags. |
| 10 | `feat(signal): add pending and synchronous wait interfaces` | Implement sigpending, sigsuspend, and a staged sigwaitinfo/sigtimedwait subset if required by the userspace target. | Tests cover pending-set accuracy, timeout, atomic mask-and-wait behavior, and wakeups without lost signals. |
| 11 | `feat(signal): define blocking syscall interruption and restart rules` | Integrate signals with read/write/wait/sleep/poll and other blocking paths; document partial I/O, EINTR, and restart semantics. | Tests inject signals while operations block; each call follows a documented interruption matrix; timed operations do not restart beyond specified timeout. |
| 12 | `feat(signal): implement SIGCHLD and child lifecycle integration` | Generate child-change notifications on exit, stop, and continue; add supported SA_NOCLDSTOP/SA_NOCLDWAIT behavior only if child reaping semantics exist. | Tests cover child exit/status/reap races and duplicate notifications; shell/init can reap children without polling forever. |
| 13 | `feat(signal): add process-group and terminal-generated signals` | Integrate process-group targeting and terminal Ctrl-C/Ctrl-Z/HUP behavior with TTY and shell only after those components are runtime-integrated. | PTY/QEMU tests prove foreground group delivery, background job behavior, stop/continue, terminal restoration, and permissions; unsupported TTY semantics are documented. |
| 14 | `feat(signal): add alternate signal stack support` | Add sigaltstack and SA_ONSTACK for fault/overflow handling if the process memory model supports protected stack mappings. | Tests cover valid/invalid stack ranges, nested delivery policy, stack exhaustion, and fallback behavior; alternate stack state is restored correctly. |
| 15 | `feat(signal): add queued real-time signals with resource limits` | Add a defined real-time signal range, queued siginfo, ordering, and per-user/process queue limits only after standard signal delivery is stable. | Tests prove FIFO ordering for same real-time signal, payload preservation, limit enforcement, and cleanup on process exit; real-time signals are never silently treated as standard coalescing signals. |
| 16 | `feat(signal): integrate signalfd as a pollable FD extension` | Integrate existing signalfd model with real FD table, read/poll, signal masks, and close semantics. Keep it Linux-specific. | Runtime test blocks a signal, receives it through FD, polls readiness, and verifies closure and mask changes; no POSIX portability claim is made. |
| 17 | `feat(libc): align signal wrappers and constants with kernel ABI` | Wire sigaction, mask operations, kill, raise, sigpending, wait APIs, and signal return stubs to actual kernel dispatch; define headers and error mapping. | C/Rust userspace tests verify constants, layouts, errno, wrapper calling convention, and handler round trip; unsupported calls fail predictably. |
| 18 | `test(signal): add QEMU stress and race regression suite` | Add tests for concurrent delivery, mask changes, process exit, thread exit, handler replacement, repeated signals, signal storms, and scheduler migration. | Tests are deterministic, bounded, and repeatable; failures retain logs and reproducer; no reliance on arbitrary sleeps for synchronization. |
| 19 | `docs(signal): publish signal reference and maturity matrix` | Add signal list/default actions, API references, behavior matrix, architecture status, security model, examples, and known gaps. | Every documented capability links to tests and target evidence; POSIX baseline and OS-specific extensions are clearly distinguished. |

---

## Periodic Maintenance Cadences

- **Every PR:** `ci(signal)` — Build, format, unit tests, ABI layout consistency, and applicable integration checks.
- **Weekly:** `test(signal)` — Triage signal regressions and fuzz findings; update status matrix in `FEATURE_STATUS.toml`.
- **Monthly:** `audit(signal)` — Review frame construction, user memory access helpers, restored registers/flags, stack range, and syscall return paths.
- **Monthly:** `test(signal)` — Execute process lifecycle and blocking-call test matrix on QEMU target.
- **Quarterly:** `docs(signal)` — Review POSIX and Linux/BSD divergence; update documentation and capability matrix.
- **Quarterly:** `security(signal)` — Review privilege boundaries, queue limits, signal storm DoS protection, and frame validation safety.
- **Each Release Candidate:** `release(signal)` — Certify signal support claims on booted target environment.

---

## Cross-Component Implementation Rules

1. **Scheduler and Context Switching:** Deliver pending signals exclusively at controlled kernel-to-user return points. Preserve masks and pending sets across preemption and thread migration.
2. **Process/Thread Model:** Maintain process-wide dispositions and process-directed pending signals separately from per-thread masks and thread-directed signals.
3. **Syscall Layer:** Integrate all signal syscalls through the unified dispatch and safe user-copy boundary.
4. **Architecture Ports:** Implement architecture-specific frame construction, register restoration, stack alignment, and return trampoline per supported architecture.
5. **VFS / File Descriptor Table:** `signalfd` is a Linux-specific extension implemented after standard signals and pollable FDs are integrated. Closing an FD unregisters subscriptions.
6. **Shell and Init:** Use process groups for job control and reliable `SIGCHLD`/wait semantics for child process reaping.
7. **Security Framework:** Enforce permission checks, capabilities, namespaces, and seccomp rules in the kernel at signal generation and delivery.
8. **Diagnostics:** Provide bounded counters and trace events without logging sensitive process memory or handler pointers.

---

## Status and Maturity Definitions

- **Proposed:** Architected and documented with PR issue/spec; no code or mock only.
- **Prototype:** Signal logic implemented with unit/mock tests passing; lacking full Ring 3 / QEMU dispatch.
- **Integrated:** Signal logic connected to kernel scheduler, syscall dispatch, and verified in real Ring 3 user process on a booted target.
- **Supported:** Comprehensive test coverage including signal masks, permission checks, blocking-call EINTR/restart, race conditions, and architecture frame validation.
