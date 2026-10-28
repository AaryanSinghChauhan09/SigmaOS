# SigmaOS Signal Subsystem Audit and ABI Profile

This document details the audit of SigmaOS signal handling components, standard POSIX.1-2024 ABI contracts, x86_64 user frame specifications, and Linux/FreeBSD comparative policy definitions.

## 1. Signal Audit and Codebase Inventory

| Subsystem Component | File Location | Architectural Purpose | Status & Verification Tier |
|---|---|---|---|
| Master Signal PR Suite | `src/signal/sovereign_signal_development_and_pr_suite.rs` | Safe Rust zero-dependency 19-PR signal lifecycle model and unit test suite | Unit-tested (19/19 PR stages pass) |
| Signal Dispatcher | `src/signal/mod.rs` | Async signal queue, capability checking, and target PID dispatch | Unit-tested (8 unit tests pass) |
| Kernel Signal Engine | `src/kernel/sigma_signal.rs` | Kernel signal state management, signal sets, and process signal actions | Unit-tested (6 unit tests pass) |
| POSIX Compatibility Signals | `src/compatibility/posix_signals.rs` | Standard POSIX signal numbers, default actions, and thread mask operations | Unit-tested (6 unit tests pass) |
| SignalFD Integration | `src/kernel/signalfd.rs` | Linux-inspired `signalfd` file descriptor creation and polling | Unit-tested |
| Kernel Interruption Suite | `src/kernel/sovereign_kernel_signal_interruption_suite.rs` | Kernel syscall interruption, restart (`SA_RESTART`), and `EINTR` policy | Unit-tested |
| Integrated Test Suite | `tests/signal/signal_test.rs` | Uncatchable signal verification (`SIGKILL`/`SIGSTOP`) | Unit-tested |

## 2. Supported POSIX Subset and Architecture Profile

SigmaOS targets POSIX.1-2024 signal compatibility with explicit Linux/FreeBSD comparative extensions:

- **Standard Signals (1..31)**:
  - `SIGHUP` (1), `SIGINT` (2), `SIGQUIT` (3), `SIGILL` (4), `SIGTRAP` (5), `SIGABRT` (6), `SIGBUS` (7), `SIGFPE` (8), `SIGKILL` (9, uncatchable/unblockable), `SIGUSR1` (10), `SIGSEGV` (11), `SIGUSR2` (12), `SIGPIPE` (13), `SIGALRM` (14), `SIGTERM` (15), `SIGCHLD` (17), `SIGCONT` (18), `SIGSTOP` (19, uncatchable/unblockable), `SIGTSTP` (20), `SIGSYS` (31).
- **Real-Time Signals (32..64)**:
  - `SIGRTMIN` (32) through `SIGRTMAX` (64) queued in FIFO order with per-process resource limits (`real_time_queue_limit`).
- **Signal Dispositions**:
  - `SIG_DFL` (Default action: Terminate, CoreDump, Stop, Continue, Ignore).
  - `SIG_IGN` (Ignore signal: discarded immediately on generation unless uncatchable).
  - `UserHandler(u64)` (Ring 3 user-space handler address).
- **Sigaction Flags**:
  - `SA_NOCLDSTOP`: Do not generate `SIGCHLD` when child processes stop.
  - `SA_NOCLDWAIT`: Do not create zombie processes on child termination.
  - `SA_SIGINFO`: Pass three-argument `siginfo_t` handler payload.
  - `SA_ONSTACK`: Execute handler on alternate signal stack (`sigaltstack`).
  - `SA_RESTART`: Automatically restart interrupted blocking system calls.
  - `SA_NODEFER`: Do not block the delivered signal during handler execution.
  - `SA_RESETHAND`: Reset disposition to `SIG_DFL` upon entering the handler.

## 3. x86_64 User Signal Frame Construction & Layout

When the kernel selects a thread for signal delivery, it allocates and constructs a `UserSignalFrame` on the target thread's user stack (or alternate stack if `SA_ONSTACK` is configured):

```text
+-------------------------------------------------------+  High Address
| Interrupted User Registers (RAX, RBX, RCX, RDX, RDI, |
| RSI, RBP, R8-R15, RSP, RIP, RFLAGS)                   |
+-------------------------------------------------------+
| Saved User Signal Mask (SigSet - 64 bits)            |
+-------------------------------------------------------+
| Signal Information Payload (siginfo_t)               |
+-------------------------------------------------------+
| Trampoline Return Address (sa_restorer / sigreturn)   |  <- RSP on Handler Entry
+-------------------------------------------------------+  Low Address
```

### Safety and Validation Rules for Frame Construction

1. **Stack Alignment**: RSP on handler entry must satisfy x86_64 System V ABI alignment rules (16-byte aligned before `CALL` / 8-byte off 16-byte boundary on entry).
2. **Canonical Address Check**: The frame user address must reside strictly in canonical user address space (`< 0x0000_7FFF_FFFF_FFFF`).
3. **Guard Page & Permissions**: Stack writing uses safe user-copy routines (`copy_to_user`). Unmapped or write-protected stack pointers trigger `SIGSEGV`.
4. **Privilege Boundary Protection**: Saved register state flags (`RFLAGS`) must never allow restoring kernel mode, enabling hardware breakpoints, or modifying privilege level bits (CPL 0).

## 4. Signal Return (`sigreturn`) Validation Rules

Upon completion of user handler execution, the user trampoline issues the `sigreturn` system call. The kernel performs strict validation prior to context restoration:

1. **Frame Signature & Version**: The frame magic and version tag (`frame_version == 1`) must match expected constants.
2. **Canonical RIP / RSP**: Restored instruction pointer (`RIP`) and stack pointer (`RSP`) must point to canonical user memory.
3. **RFLAGS Sanitization**: Reserved and privileged RFLAGS bits are masked to preserve user-mode execution flags (e.g. IF=1, IOPL=0).
4. **Mask Restoration**: The saved signal mask is restored to the thread state and sanitized via `sanitize_unblockable()` to ensure `SIGKILL` and `SIGSTOP` remain unblocked.

Failed frame validation causes immediate process termination with `SIGSEGV` or `SIGSYS` and generates an auditable security event.

## 5. System Call Interruption and Restart Rules (`EINTR` vs `SA_RESTART`)

When a thread blocked in a sleeping kernel call receives a signal:

- If the signal action sets `SA_RESTART` and the system call belongs to the restartable subset (e.g. `read`, `write`, `waitpid`, `select`, `epoll_wait`):
  - The kernel rewinds the user instruction pointer to re-execute the syscall instruction after handler completion (`RESTART_SYSCALL`).
- If `SA_RESTART` is not set or the system call is non-restartable:
  - The system call completes immediately, returning `-EINTR` to user space.

## 6. Linux/FreeBSD Comparative Extension Policy

- **Linux-inspired Extensions**:
  - `signalfd(2)`: Exposes pending signals as a pollable file descriptor.
  - `pidfd_send_signal(2)`: Target processes via stable file descriptors rather than PIDs.
- **FreeBSD-inspired Extensions**:
  - `pdfork(2)` / Process Descriptors: Signal dispatch scoped within capability sandboxes.
- **SigmaOS Policy**:
  - Standard POSIX APIs (`sigaction`, `sigprocmask`, `kill`, `raise`, `sigpending`, `sigsuspend`, `sigwait`) take precedence.
  - Extensions are labeled as non-POSIX extensions and isolated behind feature gates.
