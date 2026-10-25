# SigmaOS Signal Handler Audit and ABI Profile

## Executive Audit Summary

An audit of signal-related components across `src/signal/mod.rs`, `src/signal/sovereign_signal_development_and_pr_suite.rs`, `src/ipc/signals.rs`, `src/compatibility/posix_signals.rs`, `src/kernel/signal.rs`, `src/kernel/sigma_signal.rs`, `src/kernel/signalfd.rs`, and `tests/signal/signal_test.rs` indicates that signal data structures, constants, dispositions, masks, and PR management logic are fully modeled and unit-tested in safe Rust.

However, kernel-level dispatch to Ring 3 user handlers and real process signal delivery via CPU stack frame insertion require integration testing on active boot targets (QEMU x86_64).

## Architecture & ABI Profile Specification

### x86_64 User Stack Signal Frame Layout
When delivering a signal to a user-mode process, the kernel constructs a frame on the user stack (aligned to a 16-byte boundary per SysV AMD64 ABI):

```
+------------------------------------+  High Memory
| Saved Signal Mask (sigset_t)       |
+------------------------------------+
| ucontext_t / sigcontext            |
|   - r8..r15, rdi, rsi, rbp, rbx    |
|   - rdx, rcx, rax, rip, rsp        |
|   - eflags, cs, ss, gs, fs         |
+------------------------------------+
| siginfo_t (si_signo, si_code, etc) |
+------------------------------------+
| Restorer Trampoline / Return Address|
+------------------------------------+  Low Memory (RSP)
```

### Trampoline and `sigreturn` Dispatch
1. The kernel pushes the frame onto the user stack.
2. The user register `%rdi` is loaded with `si_signo`, `%rsi` with `&siginfo_t`, and `%rdx` with `&ucontext_t`.
3. Execution jumps to the user handler address `sa_handler` or `sa_sigaction`.
4. Upon handler return, execution jumps to `sa_restorer` trampoline which invokes syscall `SYS_rt_sigreturn`.
5. Kernel `sys_rt_sigreturn` validates canonical frame addresses, privilege bits (ensuring CS/SS remain Ring 3), restores saved registers, restores the prior signal mask, and resumes interrupted thread execution.

## Comparative Compatibility Profile

| Feature | POSIX.1-2024 Baseline | Linux Reference | FreeBSD Reference | SigmaOS Architecture |
|---|---|---|---|---|
| `sigaction` API | Required | Supported | Supported | Primary interface |
| `signal()` API | Legacy / Deprecated | Libc Wrapper | Libc Wrapper | Libc compatibility wrapper |
| `sigset_t` Size | Implementation-defined | 64-bit (8 bytes) | 128-bit | 64-bit mask (1..64) |
| `SIGKILL`/`SIGSTOP` | Uncatchable / Unblockable | Uncatchable | Uncatchable | Enforced kernel guard |
| `signalfd` | Not specified | Linux extension | kqueue extension | Supported as Linux-inspired extension |
| `sigaltstack` | Optional | Supported | Supported | Target PR 10 |
| `SA_RESTART` | System call restart | Supported | Supported | Scoped subset in PR 8 |

## Signal Numbers and Default Dispositions

| Signal Number | Name | POSIX Standard Default Action | Catchable / Maskable |
|---|---|---|---|
| 1 | SIGHUP | Terminate | Yes |
| 2 | SIGINT | Terminate | Yes |
| 3 | SIGQUIT | Core Dump | Yes |
| 4 | SIGILL | Core Dump | Yes |
| 5 | SIGTRAP | Core Dump | Yes |
| 6 | SIGABRT | Core Dump | Yes |
| 7 | SIGBUS | Core Dump | Yes |
| 8 | SIGFPE | Core Dump | Yes |
| 9 | SIGKILL | Terminate Immediately | **No** (Uncatchable / Unblockable) |
| 10 | SIGUSR1 | Terminate | Yes |
| 11 | SIGSEGV | Core Dump | Yes |
| 12 | SIGUSR2 | Terminate | Yes |
| 13 | SIGPIPE | Terminate | Yes |
| 14 | SIGALRM | Terminate | Yes |
| 15 | SIGTERM | Terminate | Yes |
| 17 | SIGCHLD | Ignore | Yes |
| 18 | SIGCONT | Continue execution | Yes |
| 19 | SIGSTOP | Stop process | **No** (Uncatchable / Unblockable) |
| 20 | SIGTSTP | Stop process | Yes |
| 32..64 | Real-Time | Terminate | Yes |

## Async-Signal-Safety Guidelines for User-Space
Handlers executing in user space must adhere to async-signal-safety rules:
- Must NOT invoke non-reentrant routines (`malloc`, `free`, `printf`, `pthread_mutex_lock`).
- Should only set atomic flags (e.g. `volatile sig_atomic_t`) or write to non-blocking self-pipes / `signalfd`.
