# SigmaOS Signal Development and Maintenance Plan

## Executive Summary

The **SigmaOS Signals Development and Maintenance Plan** establishes the engineering standards, POSIX.1-2024 baseline, Linux/BSD comparative extension policies, proposed PR sequence, maintenance cadences, and review gates for the signal handling subsystem across kernel and userspace layers.

---

## 1. Baseline Rules & Operating Principles

1. **POSIX.1-2024 Compliance Baseline:**
   - All standard signals (1..31) follow POSIX.1-2024 semantics by default.
   - `sigaction()` is the canonical API for setting dispositions. `signal()` is provided solely as a libc wrapper.

2. **Uncatchable Signal Invariants:**
   - `SIGKILL` (9) and `SIGSTOP` (19) cannot be caught, blocked, or ignored.
   - Any attempt to set a handler or modify masks for `SIGKILL` or `SIGSTOP` is silently ignored or rejected with `EINVAL`.

3. **Process vs. Thread Decoupling:**
   - Process signal state holds global dispositions and process-pending queue.
   - Thread signal state holds thread signal mask and thread-pending queue.
   - A thread receives a signal only if unblocked in its local mask.

4. **Safe User Stack Frame & Trampoline:**
   - Upon signal delivery, the kernel constructs a user stack frame (`UserSignalFrame`) on the process stack or alternate stack (`sigaltstack`).
   - Execution jumps to the signal handler with the return address pointing to the libc/kernel trampoline (`__restore_rt`).
   - `sys_sigreturn` restores saved state, previous mask, and resumes interrupted execution.

5. **Interruption & Restart Semantics:**
   - Interrupted blocking system calls return `-EINTR` unless `SA_RESTART` flag is set and the syscall supports automatic restart.

---

## 2. Proposed PR Sequence Roadmap (19 PRs)

1. `PR 1`: Define signal model and supported POSIX.1-2024 profile.
2. `PR 2`: Add ABI layout and struct layout unit tests.
3. `PR 3`: Connect signal syscall dispatch (`sigaction`, `sigprocmask`, `kill`, `raise`).
4. `PR 4`: Separate per-process and per-thread signal state and queues.
5. `PR 5`: Deliver default actions at kernel-to-user return points.
6. `PR 6`: Implement x86_64 user signal frame construction.
7. `PR 7`: Implement x86_64 `sigreturn` frame validation and context restoration.
8. `PR 8`: Implement `sa_mask`, self-blocking, `SA_NODEFER`, and `SA_RESTART`.
9. `PR 9`: Implement `siginfo_t` and `SA_SIGINFO` three-argument handlers.
10. `PR 10`: Add `sigaltstack` and `SA_ONSTACK` alternate signal stack support.
11. `PR 11`: Integrate `SIGCHLD` with process exit, stop, continue, and `waitpid`.
12. `PR 12`: Implement `sigwait`, `sigwaitinfo`, and `sigtimedwait` synchronous APIs.
13. `PR 13`: Add real-time signal range (32..64) and FIFO queue with process caps.
14. `PR 14`: Integrate Linux-inspired `signalfd` module into kernel VFS and epoll.
15. `PR 15`: Expose `signal.h` and libc wrapper interfaces.
16. `PR 16`: Add signal fault injection, signal storm, and concurrency stress tests.
17. `PR 17`: Validate AArch64 and RISC-V signal frame construction and `sigreturn`.
18. `PR 18`: Publish API documentation, safety guidelines, and support matrix.
19. `PR 19`: Complete end-to-end Ring 3 signal handler verification suite.

---

## 3. Maintenance Cadences & Merge Gates

- **Every PR:** ABI layout check, signal unit tests, static code analysis.
- **Weekly:** Triage signal handler crashes, reconcile status claims in `FEATURE_STATUS.toml`.
- **Monthly:** Security audit of frame construction, `sigreturn` validation, and syscall restart behavior.
- **Quarterly:** Review POSIX/Linux/BSD deviations and update architecture matrix.
- **Each Release Candidate:** Full certification across QEMU targets.
