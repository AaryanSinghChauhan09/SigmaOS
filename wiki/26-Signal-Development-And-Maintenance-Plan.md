# SigmaOS Signals Development and Maintenance Plan

## Executive Summary & Subsystem Overview

SigmaOS already has signal-related components in its repository—including `signal`, `signalfd`, process groups, `SIGCHLD`/process handling references, and a `FEATURE_STATUS.toml` inventory—but current source code and unit tests alone do not establish that signal delivery works end to end in a running user process. The project explicitly cautions that source code and unit tests do not prove runtime integration, so this plan first audits existing code and then closes the delivery, return-to-user, process, and libc gaps with evidence-gated PRs.

The plan uses **POSIX.1-2024** as the portable API baseline and Linux/FreeBSD as behavior references, not as an assertion that SigmaOS is ABI-compatible with either. POSIX defines `sigaction()` as the preferred interface for controlling signal disposition and specifies that `SIGKILL` and `SIGSTOP` cannot be blocked through the signal mask. Linux documents per-thread masks, process- and thread-directed pending signals, signal-frame delivery, and return through a signal trampoline.

---

## Rules and Implementation Principles

1. **Signals are asynchronous control flow.**
   Keep signal generation, pending state, selection, delivery, handler return, and default actions as distinct operations with clear ownership.

2. **Portable semantics first.**
   Implement and test a small POSIX signal subset before adding Linux-specific real-time, `signalfd`, or BSD extensions. Mark extensions separately in documentation and the capability matrix.

3. **Stable ABI.**
   Signal numbers, `sigset_t`, `siginfo_t`, `sigaction`, signal-frame layout, syscall behavior, and libc wrapper semantics are userspace contracts. Never silently renumber or change a published layout.

4. **No user pointer dereference in kernel.**
   Copy action structures and masks from userspace with safe copy-in helpers; copy old actions or signal info back with safe copy-out. Validate address arithmetic, alignment where required, and structure sizes.

5. **Permission-check before enqueueing.**
   For `kill()`-like APIs, verify target existence, process identity, credentials/capabilities, and allowed signal semantics before changing pending state. POSIX defines permission and `ESRCH`/`EPERM`/`EINVAL` outcomes for `kill()`.

6. **Uncatchable signals stay uncatchable.**
   Enforce `SIGKILL` and `SIGSTOP` restrictions in the kernel, regardless of what libc or callers request.

7. **Handler context must be safely restored.**
   Validate signal-frame location and return context; never accept a forged frame that can restore privileged CPU state or kernel addresses.

8. **Blocking calls have documented interruption behavior.**
   Define when calls return `EINTR`, restart under `SA_RESTART`, or complete partially. Do not promise Linux-specific details under a POSIX profile unless implemented and tested.

9. **Maturity is evidence-based.**
   Track each feature as **Proposed**, **Prototype**, **Integrated**, or **Supported**. “Integrated” requires a real user-mode signal test on a documented boot target; unit tests alone are insufficient.

---

## Proposed PR Sequence

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(signal): define signal model and supported profile` | Audit signal modules, syscalls, process/thread model, scheduler return paths, libc wrappers, shell/job-control use, and existing agent guidance. Publish POSIX baseline, Linux/BSD extensions policy, limitations, status, and ownership. | Inventory maps each signal feature to source, dispatch, tests, and runtime evidence; no “working” label implies end-to-end support without evidence. |
| **2** | `test(signal): add signal ABI and status inventory checks` | Add consistency checks for signal number definitions, names, action constants, masks, syscall IDs, libc wrappers, and docs. | CI catches duplicate/invalid signal numbers, incompatible structure layouts, missing wrappers, and stale documentation; reserved values are explicit. |
| **3** | `refactor(signal): separate generation pending selection and delivery` | Define typed signal state for per-process dispositions, per-thread masks/pending sets, process-directed pending set, and delivery result. Separate state transitions from syscall handling. | Unit tests cover enqueue, blocked signals, ignored dispositions, coalescing policy, and task exit; no delivery occurs from arbitrary interrupt context. |
| **4** | `feat(signal): implement signal sets and thread masks` | Implement set operations plus mask get/block/unblock operations and inheritance rules for process/thread creation. | Tests cover empty/full/add/delete/member operations, invalid signal bits, mask changes, pending visibility, and unmask-triggered delivery; `SIGKILL`/`SIGSTOP` cannot be blocked. |
| **5** | `feat(signal): implement sigaction with safe user-copy` | Implement query/install action, default/ignore/handler dispositions, action masks, and initial flags. | Invalid pointers and signal numbers fail safely; failed updates leave prior disposition unchanged; querying with null action works; uncatchable/unsupported signals follow documented rules. POSIX requires a failed `sigaction()` not to install a new handler. |
| **6** | `feat(signal): implement process-directed and thread-directed generation` | Add `kill`/`raise` and internal generation for faults, timers, and child state changes; distinguish process-directed from thread-directed signals. | Tests verify target selection only among eligible threads, permission failures, signal-zero checks if supported, invalid targets, and that failed sends do not enqueue. |
| **7** | `feat(signal): deliver default terminate stop and continue actions` | Implement baseline default actions, including termination, stop, continue, ignore, and core-dump placeholder behavior where core dumps are not yet implemented. | Tests cover default action and scheduler/process-state transitions; unimplemented core-dump behavior is documented and does not falsely claim a dump exists. |
| **8** | `feat(signal): deliver handlers through validated user signal frames` | At kernel-to-user return, construct a user signal frame containing interrupted context and saved mask; redirect execution to the handler and a controlled trampoline. | QEMU Ring 3 test runs a handler and resumes at interrupted instruction; corrupted, unmapped, or out-of-range frames terminate safely; architecture context restoration is tested. Linux similarly creates a user-space frame and uses a trampoline/`sigreturn` mechanism to restore context. |
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

## Periodic Maintenance PR Cadence

| Cadence | Proposed Recurring PR/Task | Required Evidence |
|---|---|---|
| **Every PR** | `ci(signal): run signal checks for affected components` | Formatting/build, signal unit tests, ABI consistency, and relevant integration checks; skipped tests explained. |
| **Weekly** | `test(signal): triage new signal regressions and fuzz findings` | Each bug has a minimal reproducer, regression test, owner, and severity or a documented reason it is not actionable. |
| **Weekly** | `docs(signal): reconcile status with runtime evidence` | Update `FEATURE_STATUS.toml` and component docs only when test evidence changes; preserve **Prototype** where only models/unit tests exist. |
| **Monthly** | `audit(signal): review delivery and return-context safety` | Review frame construction, user memory access, restored registers/flags, stack range, syscall return path, and architecture-specific changes. |
| **Monthly** | `test(signal): run process lifecycle and blocking-call matrix` | Results for `kill`, mask, handler, default action, exit/wait, `EINTR`, restart, and cleanup on the supported QEMU target. |
| **Monthly** | `maint(signal): check signal constants and libc ABI drift` | Verify kernel, UAPI headers, libc, shell/job-control code, syscall map, docs, and reference profile remain consistent. |
| **Quarterly** | `docs(signal): review POSIX and Linux/BSD divergence` | Document changed standards/reference behavior, intentional deviations, and downstream compatibility implications. |
| **Quarterly** | `security(signal): review privilege and denial-of-service boundaries` | Review signal authorization, queue limits, process/thread identity lifetime, signal storms, handler frame validation, and privileged target protection. |
| **Each RC** | `release(signal): certify signal support claims` | Repeatable Ring 3 tests, supported architecture/configuration, known limitations, ABI headers, and release notes. |
| **Post-Incident**| `fix(signal): prevent recurrence of <issue>` | Root cause, affected targets, regression test, ABI/security impact, and backport/release decision. |

Periodic checks should create reviewed issues or PRs; they should not auto-merge changes to signal numbering, ABI layouts, handler-frame semantics, or privilege checks. NIST SSDF recommends embedding security practices through development and release workflows, while Linux kernel selftests are designed to exercise actual code paths after building and booting a kernel—both support keeping runtime evidence separate from model-only tests.

---

## Cross-Component Implementation Rules

- **Scheduler and context switching:** Deliver pending signals at controlled kernel-to-user return points, not from arbitrary interrupt handlers. Preserve signal masks and pending state correctly across preemption, thread migration, and blocked waits.
- **Process/thread model:** Store process-wide dispositions and pending signals separately from per-thread masks and thread-directed pending signals. Linux documents this distinction explicitly; SigmaOS should not collapse the two if it intends POSIX-style threading.
- **Syscall layer:** Integrate all signal syscalls through the stable dispatch and user-copy boundary. The public SigmaOS inventory says syscall dispatch and Ring 3 validation are still gaps, so signal work must be gated on those paths.
- **Architecture ports:** Define one implementation per architecture for frame construction, register restoration, stack alignment, and return trampoline. Do not mark AArch64/RISC-V signal support based solely on x86_64 behavior; the feature inventory lists all three architecture boot paths as incomplete.
- **VFS/FD table:** `signalfd` is an optional Linux-style extension; implement only after signals and pollable FDs are real. Closing an FD must unregister subscriptions and release queued resources.
- **Shell and init:** Use process groups for job control and reliable `SIGCHLD`/`wait` semantics for child management. Do not claim interactive job control until TTY signal generation and foreground process groups work together.
- **Security framework:** Permission checks, seccomp rules, capabilities, namespaces, and MAC labels must be enforced at signal generation/delivery in the kernel. A policy model or UI permission is not enforcement.
- **Diagnostics:** Provide bounded counters or trace events for signal generation/delivery/denial without logging arbitrary handler pointers, user payloads, or sensitive process data.
- **Documentation:** Keep API semantics and actual test evidence in the owning signal component page and capability matrix. Reference Linux and BSD behavior as comparative evidence, not feature parity.

---

## PR Template and Merge Gates

Use this template for signal PRs:

```text
## Summary
What signal behavior or correctness gap does this PR address?

## Scope
Generation | pending state | masks | disposition | delivery | return | libc | docs
State what is explicitly not implemented.

## Compatibility profile
POSIX baseline | Linux extension | BSD extension | SigmaOS-specific behavior
List intentional deviations.

## Semantics
Targeting, default action, mask behavior, queue/coalescing behavior,
blocking-call interruption, and error results.

## Security
Permission checks, user-pointer handling, frame validation, resource limits,
process/thread lifetime, and cleanup.

## Tests and evidence
Unit/state-machine, invalid input, syscall, Ring 3, QEMU/PTY, stress, and
architecture results. List commands run and untested targets.

## Status and documentation
Proposed | Prototype | Integrated | Supported
Link updated API docs, signal inventory, and feature status.

## Checklist
- [ ] Signal constants and ABI layout unchanged or explicitly reviewed
- [ ] SIGKILL/SIGSTOP constraints enforced
- [ ] User memory copied through validated helpers
- [ ] Signal delivery/return tested on the real user-mode path
- [ ] Permissions, queue bounds, and cleanup tested
- [ ] Required CI checks pass; review comments resolved
```

### Gate Review Requirements
Require at least one maintainer review for ordinary tests/docs and two relevant reviewers for signal ABI, architecture frames, `sigreturn`, process permissions, or scheduler interactions. Protect the default branch with PR reviews, passing CI, resolved conversations, and no force pushes.

---

## Recommended Rollout

Start with the inventory audit, signal ABI tests, and a minimal x86_64 ring-3 path for one terminating signal and one caught signal. Then implement masks/dispositions, safe frame delivery/return, process permissions, and blocking-call semantics. Defer real-time signals, `signalfd`, alternate stacks, and Linux/BSD compatibility extensions until the basic path is stable.

A signal feature is **Integrated** only when a real userspace process can receive and handle it and safely resume or terminate on a documented boot target. It is **Supported** only when repeatable tests cover signal masks, default and caught actions, permission failures, process lifecycle, return-context validation, interruption semantics, and every architecture advertised as supported.
