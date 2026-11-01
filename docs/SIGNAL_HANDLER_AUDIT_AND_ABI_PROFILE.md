# SigmaOS Signal Handler Audit and ABI Profile

## Executive Summary
This document establishes the ABI layout, signal constants, stack alignment rules, and frame structures for the SigmaOS signal handling subsystem. It defines the interface contract between the SigmaOS kernel and Ring 3 user applications across x86_64, AArch64, and RISC-V architectures.

## Signal Constants and Default Actions

SigmaOS standardizes signal numbers 1 through 31 based on POSIX.1-2024 / Linux x86_64 ABI specifications, with real-time signals mapped from 32 (`SIGRTMIN`) to 64 (`SIGRTMAX`).

| Signal | Number | Default Action | Uncatchable? | Description |
|---|---|---|---|---|
| `SIGHUP` | 1 | Terminate | No | Hangup detected on controlling terminal |
| `SIGINT` | 2 | Terminate | No | Terminal interrupt signal (Ctrl+C) |
| `SIGQUIT` | 3 | Core Dump | No | Terminal quit signal (Ctrl+\) |
| `SIGILL` | 4 | Core Dump | No | Illegal instruction |
| `SIGTRAP` | 5 | Core Dump | No | Trace/breakpoint trap |
| `SIGABRT` | 6 | Core Dump | No | Process abort signal |
| `SIGBUS` | 7 | Core Dump | No | Bus error (bad memory access) |
| `SIGFPE` | 8 | Core Dump | No | Floating-point exception |
| `SIGKILL` | 9 | Terminate | **Yes** | Uncatchable termination signal |
| `SIGUSR1` | 10 | Terminate | No | User-defined signal 1 |
| `SIGSEGV` | 11 | Core Dump | No | Invalid memory reference (Segmentation fault) |
| `SIGUSR2` | 12 | Terminate | No | User-defined signal 2 |
| `SIGPIPE` | 13 | Terminate | No | Broken pipe: write to pipe with no readers |
| `SIGALRM` | 14 | Terminate | No | Timer signal from `alarm` or `settimer` |
| `SIGTERM` | 15 | Terminate | No | Termination signal |
| `SIGSTKFLT` | 16 | Terminate | No | Coprocessor stack fault |
| `SIGCHLD` | 17 | Ignore | No | Child process stopped, continued, or terminated |
| `SIGCONT` | 18 | Continue | No | Continue executing if stopped |
| `SIGSTOP` | 19 | Stop | **Yes** | Uncatchable stop process signal |
| `SIGTSTP` | 20 | Stop | No | Terminal stop signal (Ctrl+Z) |
| `SIGTTIN` | 21 | Stop | No | Background process read from tty |
| `SIGTTOU` | 22 | Stop | No | Background process write to tty |
| `SIGURG` | 23 | Ignore | No | Urgent condition on socket |
| `SIGXCPU` | 24 | Core Dump | No | CPU time limit exceeded |
| `SIGFSZ` | 25 | Core Dump | No | File size limit exceeded |
| `SIGVTALRM` | 26 | Terminate | No | Virtual timer expired |
| `SIGPROF` | 27 | Terminate | No | Profiling timer expired |
| `SIGWINCH` | 28 | Ignore | No | Window resize signal |
| `SIGIO` | 29 | Terminate | No | I/O now possible |
| `SIGPWR` | 30 | Terminate | No | Power failure / reboot |
| `SIGSYS` | 31 | Core Dump | No | Bad system call |
| `SIGRTMIN`..`SIGRTMAX` | 32..64 | Terminate | No | Real-time queued signals |

## Signal Set Representation (`sigset_t`)

`sigset_t` is represented as a 64-bit unsigned bitmask (`u64`). Bit `N-1` corresponds to signal number `N`.
- Bit 8 (0x100) -> `SIGKILL` (Signal 9)
- Bit 18 (0x40000) -> `SIGSTOP` (Signal 19)

### Uncatchable Signal Enforcements
Kernel functions `sigaction()` and `sigprocmask()` execute `sanitize_unblockable()` on input masks to guarantee that bits for `SIGKILL` and `SIGSTOP` remain cleared.

## User Signal Frame Layout (x86_64 ABI)

When delivering a signal to a Ring 3 thread, the kernel aligns the user stack to a 16-byte boundary (accounting for red-zone preservation if `SA_ONSTACK` is not active) and constructs a `UserSignalFrame`:

```text
+------------------------------------+  <- High Address (16-byte aligned)
| Interrupted RSP                    |
| Interrupted RFLAGS                 |
| Interrupted RIP                    |
| Interrupted General Registers      |  (RAX, RBX, RCX, RDX, RSI, RDI, RBP, R8-R15)
| Saved Signal Mask (sigset_t u64)   |
| SigInfo Structure (siginfo_t)      |
| Trampoline Restorer Pointer        |
| Frame Version & Magic Header       |
+------------------------------------+  <- Low Address (User RSP on handler entry)
```

## Signal Return Trampoline (`sigreturn`)

The trampoline code resides in the user process address space (provided by C runtime or VDSO):
```assembly
mov $15, %rax    # __NR_rt_sigreturn on x86_64
syscall
```

When `sys_rt_sigreturn` is invoked:
1. The kernel validates that `%rsp` points to a valid, canonical `UserSignalFrame` in user space.
2. The magic header, frame version, and alignment are verified.
3. The saved register context and signal mask are restored to the CPU execution state.
4. Execution resumes at the interrupted instruction (`interrupted_rip`).

## Architectural Comparison Matrix

| Property | POSIX.1-2024 Baseline | Linux Reference | FreeBSD Reference | SigmaOS Architecture |
|---|---|---|---|---|
| `sigaction` | Mandatory standard | Primary API | Primary API | Primary API |
| `signal()` | Legacy interface | Libc wrapper | Libc wrapper | Libc wrapper |
| Uncatchable Signals | `SIGKILL`, `SIGSTOP` | Enforced in kernel | Enforced in kernel | Enforced in kernel |
| Signal Mask Size | Standardized type | 64-bit bitmask | 128-bit bitmask | 64-bit bitmask (`u64`) |
| Real-time Signals | `SIGRTMIN`..`SIGRTMAX` | Supported (queued) | Supported | Supported (queued, capped at 32 per process) |
| Frame Validation | Implementation-defined | Kernel verification | Kernel verification | Strict frame version & magic verification |
