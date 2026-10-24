# SigmaOS Signal Handler Subsystem Audit & ABI Profile

This document details the architectural audit, System V x86_64 ABI layout profile, context saving requirements, and `sigreturn` frame validation rules for the SigmaOS signal handling subsystem.

## 1. Architectural Audit of Signal Subsystem Components

A comprehensive audit of existing signal-related code across the codebase reveals:

| Subsystem Component | Location | Current Scope & Status | Target Integration Path |
|---|---|---|---|
| Core Signal Dispatcher | `src/signal/mod.rs` | Model of capability-gated async signal dispatch and basic PID polling. | Connect capability checks to kernel task credentials and signal permissions. |
| Master PR & Parity Engine | `src/signal/sovereign_signal_development_and_pr_suite.rs` | Zero-dependency implementation of POSIX.1-2024 signal sets, `sigaction`, signal delivery, `siginfo_t`, `sigaltstack`, `sigreturn` validation, and real-time signal queues. | Provide canonical kernel-side algorithms for frame creation, mask manipulation, and frame verification. |
| POSIX Signal Compatibility | `src/compatibility/posix_signals.rs` | POSIX signal number constants, signal mask utilities, and disposition tables. | Align constants with System V ABI and expose to userland libc headers. |
| Kernel Signal Dispatch | `src/kernel/signal.rs` | Kernel signal queue and task signal state structures. | Bind to scheduler context return path and user-copy APIs. |
| Linux `signalfd` Interface | `src/kernel/signalfd.rs` | File descriptor wrapper for reading pending signals. | Integrate with kernel event polling (`epoll`) and process signal queue. |
| Userland Signal Test | `tests/signal/signal_test.rs` | Assertion tests verifying unblockable `SIGKILL` contract. | Expand with Ring 3 handler execution, frame corruption, and interruption tests. |

## 2. ABI Layout & Context Frame Specifications (System V x86_64)

### 2.1 Signal Set (`sigset_t`)
- Size: 64 bits (`u64`), represented as a bitmask where bit $N-1$ corresponds to signal number $N$.
- Standard Signals: Signals 1 (`SIGHUP`) through 31 (`SIGSYS`).
- Real-Time Signals: Signals 32 (`SIGRTMIN`) through 64 (`SIGRTMAX`).

### 2.2 Signal Frame (`sigframe`) Layout on User Stack
When a signal handler is delivered, the kernel constructs a `sigframe` on the user stack aligned to a 16-byte boundary:

```text
+-------------------------------------------------------+
|  Padding for 16-byte alignment                         |
+-------------------------------------------------------+
|  ucontext_t (saved registers, sigset_t, stack_t)      |
+-------------------------------------------------------+
|  siginfo_t (signal number, errno, code, value/fault)  |
+-------------------------------------------------------+
|  Return Trampoline Address / Restorer                 |  <-- rsp at handler entry
+-------------------------------------------------------+
```

### 2.3 Handler Register Calling Convention
- `rdi`: Signal number (`i32`)
- `rsi`: Pointer to `siginfo_t` on user stack (`*const siginfo_t`)
- `rdx`: Pointer to `ucontext_t` on user stack (`*const ucontext_t`)
- `rsp`: Must be 16-byte aligned before calling the handler.

## 3. `sigreturn` Frame Validation Rules

To prevent privilege escalation, arbitrary kernel code execution, or stack corruption via forged signal frames, the kernel strictly validates the `ucontext_t` structure restored during a `sigreturn` system call:

1. **Address Bounds & Alignment:**
   - The user frame address must reside entirely within canonical user space memory (`0x0000_0000_0000_0000` to `0x007F_FFFF_FFFF_FFFF`).
   - The user stack pointer (`rsp`) must be 8-byte aligned.
2. **Instruction Pointer (`rip`):**
   - Must be a valid canonical user-space memory address. Restoring a kernel-space `rip` is prohibited and triggers immediate process termination with `SIGSEGV`.
3. **Privilege & Selector Enforcements:**
   - Saved Code Segment (`cs`) and Stack Segment (`ss`) register selectors must equal user-mode selectors (`USER_CS` = 0x23, `USER_DS` = 0x1B).
   - Saved RFLAGS register must not contain restricted CPU flags (e.g., IOPL bits set to 0, IF / Interrupt Flag forced enabled, NT flag cleared).
4. **Stack Guard Page Verification:**
   - If `SA_ONSTACK` is configured, the kernel checks that the alternate stack allocated via `sigaltstack` has valid guard pages and adequate headroom.

## 4. Syscall Interruption and Restart Rules

- **Interrupted Syscalls:** When a thread blocked in a system call (e.g., `read`, `write`, `select`, `nanosleep`) receives a caught signal:
  - If `SA_RESTART` is NOT set in `sa_flags`: The system call returns `-EINTR`.
  - If `SA_RESTART` IS set in `sa_flags`: The kernel rewinds the saved instruction pointer (`rip -= 2` for 2-byte syscall instructions) so the system call automatically re-executes after `sigreturn`.
- **Non-Restartable Syscalls:** Timeouts (`sigsuspend`, `pause`, `nanosleep`) always return `-EINTR` upon signal delivery regardless of `SA_RESTART`.
