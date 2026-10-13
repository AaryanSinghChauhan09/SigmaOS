# SigmaOS Signal Development and Maintenance Plan

## 1. Overview and Executive Summary
SigmaOS implements a robust, portable, and secure signal architecture built on **POSIX.1-2024** as the primary portable API baseline, using **Linux** and **FreeBSD** behavior as reference benchmarks. While SigmaOS maintains compatibility with standard signal interfaces, it enforces strict evidence-gated verification before declaring features as `Integrated` or `Supported`.

The plan establishes:
- Clear separation between signal generation, pending state management, mask evaluation, and userland delivery.
- Rigid ABI stability across signal numbers, structure layouts, syscalls, and userland frame definitions.
- Security-first handling: uncatchable signals (`SIGKILL`, `SIGSTOP`) remain strictly uncatchable; user-mode signal frames are validated to prevent privilege escalation or kernel state forgery.
- Interruption rules for blocking system calls (`EINTR`, `SA_RESTART`).
- A structured 19-PR sequence gateway and a recurring maintenance policy aligned with NIST SP 800-218 (SSDF) guidelines.

---

## 2. Rules and Implementation Principles

1. **Asynchronous Control Flow**: Keep signal generation, pending state, selection, delivery, handler return, and default actions as distinct operations with clear ownership.
2. **Portable Semantics First**: Implement and test a small POSIX signal subset before adding Linux-specific real-time, `signalfd`, or BSD extensions. Mark extensions separately in documentation and capability matrices.
3. **Stable ABI**: Signal numbers, `sigset_t`, `siginfo_t`, `sigaction`, signal-frame layout, syscall behavior, and libc wrapper semantics are userspace contracts. Never silently renumber or change a published layout.
4. **No User Pointer Dereference in Kernel**: Copy action structures and masks from userspace with safe copy-in helpers; copy old actions or signal info back with safe copy-out. Validate address arithmetic, alignment, and structure sizes.
5. **Permission Check Before Enqueueing**: For `kill()`-like APIs, verify target existence, process identity, credentials/capabilities, and allowed signal semantics before changing pending state. POSIX defines permission and `ESRCH`/`EPERM`/`EINVAL` outcomes for `kill()`.
6. **Uncatchable Signals Stay Uncatchable**: Enforce `SIGKILL` and `SIGSTOP` restrictions in the kernel, regardless of what libc or callers request.
7. **Handler Context Must Be Safely Restored**: Validate signal-frame location and return context; never accept a forged frame that can restore privileged CPU state or kernel addresses.
8. **Blocking Calls Have Documented Interruption Behavior**: Define when calls return `EINTR`, restart under `SA_RESTART`, or complete partially. Do not promise Linux-specific details under a POSIX profile unless implemented and tested.
9. **Maturity is Evidence-Based**: Track each feature as `Proposed`, `Prototype`, `Integrated`, or `Supported`. `Integrated` requires a real user-mode signal test on a documented boot target; unit tests alone are insufficient.

---

## 3. Proposed PR Sequence

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(signal): define signal model and supported profile` | Audit signal modules, syscalls, process/thread model, scheduler return paths, libc wrappers, shell/job-control use, and existing agent guidance. Publish POSIX baseline, Linux/BSD extensions policy, limitations, status, and ownership. | Inventory maps each signal feature to source, dispatch, tests, and runtime evidence; no "working" label implies end-to-end support without evidence. |
| **2** | `test(signal): add signal ABI and status inventory checks` | Add consistency checks for signal number definitions, names, action constants, masks, syscall IDs, libc wrappers, and docs. | CI catches duplicate/invalid signal numbers, incompatible structure layouts, missing wrappers, and stale documentation; reserved values are explicit. |
| **3** | `refactor(signal): separate generation pending selection and delivery` | Define typed signal state for per-process dispositions, per-thread masks/pending sets, process-directed pending set, and delivery result. Separate state transitions from syscall handling. | Unit tests cover enqueue, blocked signals, ignored dispositions, coalescing policy, and task exit; no delivery occurs from arbitrary interrupt context. |
| **4** | `feat(signal): implement signal sets and thread masks` | Implement set operations plus mask get/block/unblock operations and inheritance rules for process/thread creation. | Tests cover empty/full/add/delete/member operations, invalid signal bits, mask changes, pending visibility, and unmask-triggered delivery; `SIGKILL`/`SIGSTOP` cannot be blocked. |
| **5** | `feat(signal): implement sigaction with safe user-copy` | Implement query/install action, default/ignore/handler dispositions, action masks, and initial flags. | Invalid pointers and signal numbers fail safely; failed updates leave prior disposition unchanged; querying with null action works; uncatchable/unsupported signals follow documented rules. POSIX requires a failed `sigaction()` not to install a new handler. |
| **6** | `feat(signal): implement process-directed and thread-directed generation` | Add `kill`/`raise` and internal generation for faults, timers, and child state changes; distinguish process-directed from thread-directed signals. | Tests verify target selection only among eligible threads, permission failures, signal-zero checks if supported, invalid targets, and that failed sends do not enqueue. |
| **7** | `feat(signal): deliver default terminate stop and continue actions` | Implement baseline default actions, including termination, stop, continue, ignore, and core-dump placeholder behavior where core dumps are not yet implemented. | Tests cover default action and scheduler/process-state transitions; unimplemented core-dump behavior is documented and does not falsely claim a dump exists. |
| **8** | `feat(signal): deliver handlers through validated user signal frames` | At kernel-to-user return, construct a user signal frame containing interrupted context and saved mask; redirect execution to the handler and a controlled trampoline. | QEMU Ring 3 test runs a handler and resumes at interrupted instruction; corrupted, unmapped, or out-of-range frames terminate safely; architecture context restoration is tested. Linux similarly creates a user-space frame and uses a trampoline/sigreturn mechanism to restore context. |
| **9** | `feat(signal): add sigreturn and context validation` | Implement architecture-specific signal-return path; validate frame version, user addresses, flags, privilege bits, and restored registers. | Handler return restores registers/mask/stack; forged or invalid context is rejected; no user-supplied state can restore kernel privilege or arbitrary privileged flags. |
| **10** | `feat(signal): add pending and synchronous wait interfaces` | Implement `sigpending`, `sigsuspend`, and a staged `sigwaitinfo`/`sigtimedwait` subset if required by the userspace target. | Tests cover pending-set accuracy, timeout, atomic mask-and-wait behavior, and wakeups without lost signals. |
| **11** | `feat(signal): define blocking syscall interruption and restart rules` | Integrate signals with `read`/`write`/`wait`/`sleep`/`poll` and other blocking paths; document partial I/O, `EINTR`, and restart semantics. | Tests inject signals while operations block; each call follows a documented interruption matrix; timed operations do not restart beyond the specified timeout. POSIX `SA_RESTART` behavior is interface-dependent and must be designed, not assumed. |
| **12** | `feat(signal): implement SIGCHLD and child lifecycle integration` | Generate child-change notifications on exit, stop, and continue; add supported `SA_NOCLDSTOP`/`SA_NOCLDWAIT` behavior only if child reaping semantics exist. | Tests cover child exit/status/reap races and duplicate notifications; shell/init can reap children without polling forever. POSIX describes `SIGCHLD` usage with repeated nonblocking waits. |
| **13** | `feat(signal): add process-group and terminal-generated signals` | Integrate process-group targeting and terminal Ctrl-C/Ctrl-Z/HUP behavior with the TTY and shell only after those components are runtime-integrated. | PTY/QEMU tests prove foreground group delivery, background job behavior, stop/continue, terminal restoration, and permissions; unsupported TTY semantics are documented. |
| **14** | `feat(signal): add alternate signal stack support` | Add `sigaltstack` and `SA_ONSTACK` for fault/overflow handling if the process memory model supports protected stack mappings. | Tests cover valid/invalid stack ranges, nested delivery policy, stack exhaustion, and fallback behavior; alternate stack state is restored correctly. POSIX exposes `SA_ONSTACK` where supported. |
| **15** | `feat(signal): add queued real-time signals with resource limits` | Add a defined real-time signal range, queued `siginfo`, ordering, and per-user/process queue limits only after standard signal delivery is stable. | Tests prove FIFO ordering for same real-time signal, payload preservation, limit enforcement, and cleanup on process exit; real-time signals are never silently treated as standard coalescing signals. Linux distinguishes queued real-time signals from non-queued standard signals. |
| **16** | `feat(signal): integrate signalfd as a pollable FD extension` | Integrate the existing `signalfd` model with the real FD table, read/poll, signal masks, and close semantics. Keep it Linux-specific. | Runtime test blocks a signal, receives it through the FD, polls readiness, and verifies closure and mask changes; no POSIX portability claim is made. Linux documents `signalfd` as synchronous signal acceptance through an FD. |
| **17** | `feat(libc): align signal wrappers and constants with kernel ABI` | Wire `sigaction`, mask operations, `kill`, `raise`, `sigpending`, wait APIs, and signal return stubs to actual kernel dispatch; define headers and error mapping. | C/Rust userspace tests verify constants, layouts, `errno`, wrapper calling convention, and a handler round trip; unsupported calls fail predictably. |
| **18** | `test(signal): add QEMU stress and race regression suite` | Add tests for concurrent delivery, mask changes, process exit, thread exit, handler replacement, repeated signals, signal storms, and scheduler migration. | Tests are deterministic, bounded, and repeatable; failures retain logs and reproducer; no reliance on arbitrary sleeps for synchronization. |
| **19** | `docs(signal): publish signal reference and maturity matrix` | Add signal list/default actions, API references, behavior matrix, architecture status, security model, examples, and known gaps. | Every documented capability links to tests and target evidence; POSIX baseline and OS-specific extensions are clearly distinguished. |

---

## 4. Periodic Maintenance Cadence

| Cadence | Proposed Recurring PR/Task | Required Evidence |
|---|---|---|
| **Every PR** | `ci(signal): run signal checks for affected components` | Formatting/build, signal unit tests, ABI consistency, and relevant integration checks; skipped tests explained. |
| **Weekly** | `test(signal): triage new signal regressions and fuzz findings` | Each bug has a minimal reproducer, regression test, owner, and severity or a documented reason it is not actionable. |
| **Weekly** | `docs(signal): reconcile status with runtime evidence` | Update `FEATURE_STATUS.toml` and component docs only when test evidence changes; preserve `Prototype` where only models/unit tests exist. |
| **Monthly** | `audit(signal): review delivery and return-context safety` | Review frame construction, user memory access, restored registers/flags, stack range, syscall return path, and architecture-specific changes. |
| **Monthly** | `test(signal): run process lifecycle and blocking-call matrix` | Results for `kill`, mask, handler, default action, exit/wait, `EINTR`, restart, and cleanup on the supported QEMU target. |
| **Monthly** | `maint(signal): check signal constants and libc ABI drift` | Verify kernel, UAPI headers, libc, shell/job-control code, syscall map, docs, and reference profile remain consistent. |
| **Quarterly** | `docs(signal): review POSIX and Linux/BSD divergence` | Document changed standards/reference behavior, intentional deviations, and downstream compatibility implications. |
| **Quarterly** | `security(signal): review privilege and denial-of-service boundaries` | Review signal authorization, queue limits, process/thread identity lifetime, signal storms, handler frame validation, and privileged target protection. |
| **Each Release Candidate** | `release(signal): certify signal support claims` | Repeatable Ring 3 tests, supported architecture/configuration, known limitations, ABI headers, and release notes. |
| **After Each Signal Incident** | `fix(signal): prevent recurrence of <issue>` | Root cause, affected targets, regression test, ABI/security impact, and backport/release decision. |

---

## 5. Cross-Component Implementation Rules

- **Scheduler and context switching**: Deliver pending signals at controlled kernel-to-user return points, not from arbitrary interrupt handlers. Preserve signal masks and pending state correctly across preemption, thread migration, and blocked waits.
- **Process/thread model**: Store process-wide dispositions and pending signals separately from per-thread masks and thread-directed pending signals. Linux documents this distinction explicitly; SigmaOS does not collapse the two.
- **Syscall layer**: Integrate all signal syscalls through the stable dispatch and user-copy boundary. Ensure safe copy-in/copy-out for all pointers.
- **Architecture ports**: Define one implementation per architecture for frame construction, register restoration, stack alignment, and return trampoline. Do not mark AArch64/RISC-V signal support based solely on x86_64 behavior.
- **VFS/FD table**: `signalfd` is an optional Linux-style extension; implement only after signals and pollable FDs are real. Closing an FD must unregister subscriptions and release queued resources.
- **Shell and init**: Use process groups for job control and reliable `SIGCHLD`/`wait` semantics for child management.
- **Security framework**: Permission checks, seccomp rules, capabilities, namespaces, and MAC labels must be enforced at signal generation/delivery in the kernel.
- **Diagnostics**: Provide bounded counters or trace events for signal generation/delivery/denial without logging arbitrary handler pointers, user payloads, or sensitive process data.
- **Documentation**: Keep API semantics and actual test evidence in the owning signal component page and capability matrix. Reference Linux and BSD behavior as comparative evidence, not feature parity.

---

## 6. PR Template and Merge Gates

When submitting signal PRs, use the following structure:

```markdown
## Summary
What signal behavior or correctness gap does this PR address?

## Scope
Generation | pending state | masks | disposition | delivery | return | libc | docs

## Compatibility profile
POSIX baseline | Linux extension | BSD extension | SigmaOS-specific behavior

## Semantics
Targeting, default action, mask behavior, queue/coalescing behavior, blocking-call interruption, and error results.

## Security
Permission checks, user-pointer handling, frame validation, resource limits, process/thread lifetime, and cleanup.

## Tests and evidence
Unit/state-machine, invalid input, syscall, Ring 3, QEMU/PTY, stress, and architecture results.

## Status and documentation
Proposed | Prototype | Integrated | Supported
```

### Merge Gates:
1. Ordinary tests/docs require at least 1 maintainer review.
2. Signal ABI, architecture frames, `sigreturn`, process permissions, or scheduler interactions require at least 2 maintainer reviews.
3. CI checks must pass, all conversations resolved, and no force-push overrides permitted.

---

## 7. Recommended Rollout

1. **Phase 1**: Complete inventory audit, signal ABI tests, and a minimal x86_64 Ring 3 path for one terminating signal (`SIGKILL`/`SIGTERM`) and one caught signal (`SIGUSR1`).
2. **Phase 2**: Implement thread masks (`sigprocmask`), signal dispositions (`sigaction`), safe frame construction/delivery, and architecture `sigreturn`.
3. **Phase 3**: Enforce process/thread permissions, process group targeting, and blocking call interruption matrix (`EINTR`/`SA_RESTART`).
4. **Phase 4**: Implement `SIGCHLD` child lifecycle, alternate signal stacks (`sigaltstack`), real-time signal queues (`SIGRTMIN`..`SIGRTMAX`), and Linux `signalfd` integration.
5. **Phase 5**: Complete QEMU stress suite, publish full reference matrix, and mark features `Supported` as runtime evidence is verified.
