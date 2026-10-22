# SigmaOS Signal Development and Maintenance Plan

## Executive Summary

The SigmaOS Signal Development and Maintenance Plan defines the engineering framework for signal generation, delivery, queuing, masking, handler frame construction, and signal-return validation across the operating system.

Aligned with POSIX.1-2024 and leveraging Linux and FreeBSD reference architectures, this plan enforces strict process/thread separation, uncatchable signal rules, async-signal-safety guidelines, and multi-architecture signal frame contracts.

---

## 1. Architectural Principles and Standards

1. **POSIX Baseline:** POSIX.1-2024 forms the standard interface specification for `sigaction`, `sigprocmask`, `kill`, `raise`, `sigpending`, and `sigsuspend`.
2. **Linux & BSD Comparative References:** Linux `sigreturn`, `tgkill`, and `signalfd4`, as well as FreeBSD Capsicum capability-gated IPC signals, serve as optional extension benchmarks.
3. **Strict Memory Isolation:** Kernel frame allocation and signal-return validation must safely validate all user-space memory pointers (`copy_from_user` / `copy_to_user`).
4. **No Allocation on Delivery Path:** The kernel signal delivery path to a user process operates with zero dynamic heap memory allocations.
5. **Clear Subsystem Separation:** Signal state is maintained in `src/kernel/signal.rs`, `src/ipc/signals.rs`, and `src/signal/sovereign_signal_development_and_pr_suite.rs`.

---

## 2. Signal Development PR Sequence (19 PRs)

1. `PR 1`: Define signal model, constants, and POSIX supported profile.
2. `PR 2`: Implement `sigset_t` bitmask operations and layout verifications.
3. `PR 3`: Connect `sigaction` and `sigprocmask` system calls to kernel dispatch.
4. `PR 4`: Implement per-thread signal masks and thread-directed pending queues.
5. `PR 5`: Implement process-directed signal distribution and eligible thread selection.
6. `PR 6`: Implement default kernel signal actions (`Term`, `Ign`, `Core`, `Stop`, `Cont`).
7. `PR 7`: Implement x86_64 user signal frame construction on user stack.
8. `PR 8`: Implement x86_64 `sigreturn` system call and user frame validation.
9. `PR 9`: Implement `sa_mask` automatic blocking, `SA_NODEFER`, and `SA_RESETHAND`.
10. `PR 10`: Implement `SA_RESTART` system call interruption and restart semantics.
11. `PR 11`: Implement `siginfo_t` construction and `SA_SIGINFO` three-argument handlers.
12. `PR 12`: Implement `sigaltstack` alternate stack management and `SA_ONSTACK`.
13. `PR 13`: Integrate `SIGCHLD` notifications into child process exit and `waitpid`.
14. `PR 14`: Implement synchronous signal acceptance via `sigwait` and `sigtimedwait`.
15. `PR 15`: Implement Real-Time queued signals (range 32..64) with payload queues.
16. `PR 16`: Connect `signalfd4` Linux extension to kernel signal event queues.
17. `PR 17`: Expose libc signal wrappers and provide async-signal-safe examples.
18. `PR 18`: Add comprehensive fault-injection, concurrency, and frame forgery tests.
19. `PR 19`: Implement AArch64 and RISC-V 64-bit user signal frame trampolines.

---

## 3. Maintenance Cadence and Review Gates

- **CI Automation:** Every PR runs signal layout checks, bitmask tests, frame corruption validation, and standalone unit test suites.
- **Weekly Triage:** Monitor and resolve any signal handler entry/return crashes or process isolation regressions.
- **Monthly Security Audit:** Review `sigreturn` register masking, canonical stack verification, and permission checks.
- **Merge Gates:** Changes to assembly trampolines, user-frame construction, or `sigreturn` require 2 qualified reviewer approvals.

---

## 4. Maturity Matrix

- **Modeled / Prototype:** Structures and frame layouts unit-tested via `rustc --test src/signal/sovereign_signal_development_and_pr_suite.rs`.
- **Integrated:** Ring 3 process in QEMU executes a signal handler and returns safely via trampoline.
- **Supported:** Multi-arch automated hardware/QEMU testing passes all signal action, mask, restart, and frame integrity tests.
