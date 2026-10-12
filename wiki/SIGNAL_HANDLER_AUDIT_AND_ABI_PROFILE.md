# SigmaOS Signal Subsystem Audit & ABI Profile

## 1. Audit Matrix Across Signal Modules

The SigmaOS signal handling ecosystem spans multiple kernel, IPC, POSIX compatibility, and userland modules:

| Subsystem Module | File Location | Functional Scope | Current Implementation Level |
|---|---|---|---|
| **Core Signal Engine** | `src/signal/mod.rs` | 64-bit `SignalSet` mask, `SignalActionFlags`, `SignalAltStack`, `SignalHandler`, `SignalState` (handlers, blocked, pending), `set_handler` (protecting `SIGKILL`/`SIGSTOP`), `deliver` with `sa_resethand` | Unit-tested model |
| **Sovereign Signal Development PR Suite** | `src/signal/sovereign_signal_development_and_pr_suite.rs` | 19-PR sequence suite, `SignalNumber` (1..64), `SigSet`, `SigAction`, `SigInfo`, `UserSignalFrame`, `SigAltStack`, per-process/per-thread state, permission checks (`EPERM`), signal queues | Standalone unit tests pass |
| **IPC Signals Control Plane** | `src/ipc/signals.rs` & `src/ipc/rt_signals.rs` | IPC-layer signal signaling, real-time signal queue simulation | Subsystem model |
| **POSIX Signal Compatibility** | `src/compatibility/posix_signals.rs` | POSIX signal conversion, `sigaction`, `sigprocmask`, `kill`, `raise` syscall stubs | Syscall stubs (Partial) |
| **Kernel Signal Manager** | `src/kernel/signal.rs` & `src/kernel/sigma_signal.rs` | Process signal table, pending signal queues, default disposition actions (terminate, dump, stop, cont, ignore) | Kernel unit-tested |
| **Kernel Proc Signals** | `src/kernel/proc/signals.rs` | Process process-directed and thread-directed signal queue routing | Kernel model |
| **Kernel SignalFD Extension** | `src/kernel/signalfd.rs` | Linux-inspired `signalfd` file descriptor interface, `signalfd_siginfo` representation | Unit-tested model |
| **Syscall Dispatch Shim** | `src/syscall/posix_compat.rs` | Syscall numbers for `sys_sigaction`, `sys_sigprocmask`, `sys_kill`, `sys_sigreturn` | Compile-only / Partial ABI |

---

## 2. ABI Profile & Standard Compliance

SigmaOS signal subsystem targets **POSIX.1-2024** standard compliance with select **Linux** and **FreeBSD** extension capabilities:

- **Primary Interface:** `sigaction()` with `sigset_t` masks and explicit action flags.
- **Legacy Interface:** `signal()` implemented strictly as a wrapper around `sigaction()`.
- **Uncatchable Signals:** `SIGKILL` (9) and `SIGSTOP` (19) cannot be caught, ignored, or blocked. Attempts to modify their dispositions via `sigaction()` return `-EINVAL`; attempts to block them via `sigprocmask()` are silently ignored by stripping their bits.
- **Process vs Thread Routing:** Signals generated for a process group or process PID are queued in `pending_process_signals`. Signals generated via `pthread_kill` or thread exceptions are queued in `pending_thread_signals`.

---

## 3. Signal Numbers & Default Dispositions

| Signal Number | Name | POSIX Default Action | Catchable / Maskable | Notes |
|---|---|---|---|---|
| 1 | `SIGHUP` | Terminate | Yes | Hangup on controlling terminal |
| 2 | `SIGINT` | Terminate | Yes | Terminal interrupt signal (Ctrl+C) |
| 3 | `SIGQUIT` | CoreDump | Yes | Terminal quit signal (Ctrl+\) |
| 4 | `SIGILL` | CoreDump | Yes | Illegal Instruction |
| 5 | `SIGTRAP` | CoreDump | Yes | Trace/breakpoint trap |
| 6 | `SIGABRT` | CoreDump | Yes | Process abort signal |
| 7 | `SIGBUS` | CoreDump | Yes | Bus error (bad memory access) |
| 8 | `SIGFPE` | CoreDump | Yes | Floating-point exception |
| 9 | **`SIGKILL`** | **Terminate** | **NO** | **Uncatchable, unmaskable kill** |
| 10 | `SIGUSR1` | Terminate | Yes | User-defined signal 1 |
| 11 | `SIGSEGV` | CoreDump | Yes | Invalid memory reference |
| 12 | `SIGUSR2` | Terminate | Yes | User-defined signal 2 |
| 13 | `SIGPIPE` | Terminate | Yes | Broken pipe: write to pipe with no readers |
| 14 | `SIGALRM` | Terminate | Yes | Timer signal from `alarm` |
| 15 | `SIGTERM` | Terminate | Yes | Termination signal |
| 16 | `SIGSTKFLT` | Terminate | Yes | Stack fault on coprocessor |
| 17 | `SIGCHLD` | Ignore | Yes | Child stopped or terminated |
| 18 | `SIGCONT` | Continue | Yes | Continue if stopped |
| 19 | **`SIGSTOP`** | **Stop** | **NO** | **Uncatchable, unmaskable stop** |
| 20 | `SIGTSTP` | Stop | Yes | Terminal stop signal (Ctrl+Z) |
| 21 | `SIGTTIN` | Stop | Yes | Background process read from tty |
| 22 | `SIGTTOU` | Stop | Yes | Background process write to tty |
| 23 | `SIGURG` | Ignore | Yes | Urgent condition on socket |
| 24 | `SIGXCPU` | CoreDump | Yes | CPU time limit exceeded |
| 25 | `SIGXFSZ` | CoreDump | Yes | File size limit exceeded |
| 26 | `SIGVTALRM` | Terminate | Yes | Virtual timer expired |
| 27 | `SIGPROF` | Terminate | Yes | Profiling timer expired |
| 28 | `SIGWINCH` | Ignore | Yes | Window resize signal |
| 29 | `SIGIO` | Terminate | Yes | I/O now possible |
| 30 | `SIGPWR` | Terminate | Yes | Power failure |
| 31 | `SIGSYS` | CoreDump | Yes | Bad system call |
| 32..64 | `SIGRTMIN`..`SIGRTMAX` | Terminate | Yes | Queued Real-Time Signals |

---

## 4. x86_64 User Signal Frame & Trampoline ABI

When the kernel delivers a user-handler signal to a Ring 3 thread, it constructs a frame on the user stack (respecting 16-byte System V AMD64 ABI alignment) and modifies the thread's instruction pointer to point to the handler.

### `UserSignalFrame` Layout

```rust
#[repr(C)]
pub struct UserSignalFrame {
    pub frame_version: u32,       // Version identifier for validation (e.g., 0x01)
    pub saved_mask: u64,          // Prior blocked signal set bitmask
    pub handler_ptr: u64,         // Ring 3 handler address
    pub trampoline_ptr: u64,      // Ring 3 restorer / trampoline address
    pub siginfo: SigInfo,         // Typed signal metadata structure
    pub saved_rax: u64,           // Interrupted register context
    pub saved_rbx: u64,
    pub saved_rcx: u64,
    pub saved_rdx: u64,
    pub saved_rsi: u64,
    pub saved_rdi: u64,
    pub saved_rbp: u64,
    pub saved_rsp: u64,
    pub saved_r8: u64,
    pub saved_r9: u64,
    pub saved_r10: u64,
    pub saved_r11: u64,
    pub saved_r12: u64,
    pub saved_r13: u64,
    pub saved_r14: u64,
    pub saved_r15: u64,
    pub saved_rip: u64,           // Interrupted instruction pointer
    pub saved_rflags: u64,        // Interrupted CPU flags
}
```

### Trampoline and `sigreturn` Validation
1. The kernel pushes `UserSignalFrame` to the user stack (or alternate stack if `SA_ONSTACK` is active).
2. The user register `%rdi` is loaded with `sig_number`, `%rsi` with `&siginfo` (if `SA_SIGINFO`), and `%rdx` with `&ucontext`.
3. The return address on top of the stack is set to `sa_restorer` (the user signal trampoline).
4. Upon handler completion, execution transfers to the trampoline, which executes the `sys_sigreturn` system call.
5. `sys_sigreturn` validates:
   - Frame alignment and bounds within canonical user virtual address space.
   - `frame_version` match.
   - Non-privileged `rflags` state (preventing privilege escalation).
   - Saved signal mask (ensuring `SIGKILL`/`SIGSTOP` remain unblocked).
6. Upon validation, saved registers, mask, and execution context are restored seamlessly.

---

## 5. Syscall Interruption and Restart Semantics

When a signal arrives for a thread blocked on a kernel syscall (e.g., `read`, `write`, `waitpid`, `nanosleep`):

- **Default / No `SA_RESTART`:** The blocking syscall is interrupted, kernel cleanup executes, and the syscall returns `-EINTR` to user space.
- **`SA_RESTART` Set:** For eligible slow system calls (`read`, `write`, `ioctl`, `wait4`), the kernel sets up the signal frame such that upon `sigreturn`, the instruction pointer points back to the `SYSCALL` instruction, automatically restarting the call.
- **Unrestartable Syscalls:** Timers (`nanosleep`, `clock_nanosleep`) and explicit poll calls (`epoll_wait`, `select`) always return `-EINTR` regardless of `SA_RESTART`.

---

## 6. Async-Signal-Safety Guidance

User-space signal handlers execute asynchronously and can interrupt normal program flow at any instruction boundary. Therefore, handlers **MUST ONLY** call async-signal-safe functions:

- **Unsafe in Handlers:** Dynamic memory allocation (`malloc`, `free`, Rust `Vec`/`String` allocation), stdio routines (`printf`, `cout`), mutex locking (susceptible to deadlocks if interrupted while holding lock).
- **Safe in Handlers:** Direct system calls (`write` to file descriptors, `_exit`), atomic variable writes, signal-safe lock-free ringbuffers.
