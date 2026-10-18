# SigmaOS Signal Path Audit and ABI Profile

## Executive Summary & Subsystem Inventory

SigmaOS contains signal handling components spread across kernel modules, IPC layers, and POSIX compatibility interfaces. This document audits the existing signal implementation, maps system call ABI definitions, defines signal frame and context layouts, details the trampoline return path, and establishes architecture-specific limits.

---

## 1. Audit Matrix of Signal Subsystem Components

| Subsystem Module | Path | Responsibilities & Implemented Features | Current Implementation Tier |
|---|---|---|---|
| **Core Signal Types & Sets** | `src/signal/mod.rs` | Standard POSIX signal definitions (1–64), 64-bit bitmask `SignalSet`, `SignalActionFlags`, `SignalAltStack`, `SignalHandler` (Default, Ignore, Custom), `SignalState` (dispositions, blocked set, pending set, alt stack), `SIGKILL`/`SIGSTOP` guard rules. | Prototype (`rustc --test` passed) |
| **Development & PR Suite** | `src/signal/sovereign_signal_development_and_pr_suite.rs` | Development test harness simulating PRs 1–19, verifying `sigaction` uncatchable guards, permission checks, frame corruption rejection, and real-time signal queue limits. | Prototype (Simulation harness) |
| **IPC Real-Time Signals** | `src/ipc/rt_signals.rs` | Real-time signals (signals 32..64 / `SIGRTMIN`..`SIGRTMAX`), queued signal instances with payloads (`sigval_t`), FIFO ordering per signal number, queue limits per process (default 64). | Prototype |
| **IPC Signals Control** | `src/ipc/signals.rs` | Control plane signal manager (`SovereignSignalManager`), queuing signals by PID, mask operations, process-level delivery filtering. | Prototype |
| **POSIX Signal Compatibility** | `src/compatibility/posix_signals.rs` | POSIX compatibility mapping (`sigaction_t`, `siginfo_t`, `sigset_t`, `sigaltstack_t`), translation between POSIX numbers and internal kernel signal enums, signal disposition management. | Prototype |
| **Kernel Process Signals** | `src/kernel/signal.rs`, `src/kernel/proc/signals.rs` | Process-directed signal dispatch, target PID authorization checks (same UID / root check), thread selection for delivery. | Prototype / Unintegrated |
| **Linux `signalfd` Integration** | `src/kernel/signalfd.rs` | `signalfd` file descriptor interface, `signalfd_siginfo` serialization, polling/reading blocked pending signals via event loop. | Prototype |
| **Kernel Signal Dispatcher** | `src/kernel/sigma_signal.rs`, `src/kernel/signal_dispatcher.rs` | Signal queue dispatching, kernel-space signal delivery notification loops. | Prototype |
| **Kernel Interruption Suite** | `src/kernel/sovereign_kernel_signal_interruption_suite.rs` | Interruption testing framework for blocking system calls (`read`, `write`, `waitpid`, `sleep`), evaluating `EINTR` vs `SA_RESTART`. | Prototype |

---

## 2. Signal Numbers, Actions, and POSIX Subset

### Standard Signal Number Mapping
SigmaOS maps standard POSIX signal numbers (1..31) and real-time signals (32..64):

| Signal Number | Name | Default Action | Uncatchable / Unblockable | Description |
|---|---|---|---|---|
| 1 | `SIGHUP` | Terminate | No | Hangup detected on controlling terminal |
| 2 | `SIGINT` | Terminate | No | Terminal interrupt signal (Ctrl+C) |
| 3 | `SIGQUIT` | Core / Terminate | No | Terminal quit signal (Ctrl+\) |
| 4 | `SIGILL` | Core / Terminate | No | Illegal Instruction |
| 5 | `SIGTRAP` | Core / Terminate | No | Trace/breakpoint trap |
| 6 | `SIGABRT` | Core / Terminate | No | Process abort signal |
| 7 | `SIGBUS` | Core / Terminate | No | Bus error (bad memory access) |
| 8 | `SIGFPE` | Core / Terminate | No | Floating-point exception |
| 9 | `SIGKILL` | Terminate | **YES** | Kill signal (cannot be caught, ignored, or blocked) |
| 10 | `SIGUSR1` | Terminate | No | User-defined signal 1 |
| 11 | `SIGSEGV` | Core / Terminate | No | Invalid memory reference (Segmentation Fault) |
| 12 | `SIGUSR2` | Terminate | No | User-defined signal 2 |
| 13 | `SIGPIPE` | Terminate | No | Broken pipe: write to pipe with no readers |
| 14 | `SIGALRM` | Terminate | No | Timer signal from `alarm` or `settimer` |
| 15 | `SIGTERM` | Terminate | No | Termination signal |
| 16 | `SIGSTKFLT` | Terminate | No | Stack fault on coprocessor |
| 17 | `SIGCHLD` | Ignore | No | Child process stopped or terminated |
| 18 | `SIGCONT` | Continue | No | Continue executing if stopped |
| 19 | `SIGSTOP` | Stop | **YES** | Stop execution (cannot be caught, ignored, or blocked) |
| 20 | `SIGTSTP` | Stop | No | Terminal stop signal (Ctrl+Z) |
| 21 | `SIGTTIN` | Stop | No | Background process read from tty |
| 22 | `SIGTTOU` | Stop | No | Background process write to tty |
| 23 | `SIGURG` | Ignore | No | Urgent condition on socket |
| 24 | `SIGXCPU` | Core / Terminate | No | CPU time limit exceeded |
| 25 | `SIGXFSZ` | Core / Terminate | No | File size limit exceeded |
| 26 | `SIGVTALRM` | Terminate | No | Virtual timer expired |
| 27 | `SIGPROF` | Terminate | No | Profiling timer expired |
| 28 | `SIGWINCH` | Ignore | No | Window size change |
| 29 | `SIGIO` / `SIGPOLL` | Terminate | No | I/O now possible |
| 30 | `SIGPWR` | Terminate | No | Power failure |
| 31 | `SIGSYS` | Core / Terminate | No | Bad system call |
| 32..64 | `SIGRTMIN`..`SIGRTMAX` | Terminate | No | Real-time queued signals |

---

## 3. ABI Layouts and Data Structures

### `sigset_t` Bitmask Representation
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct sigset_t {
    pub val: [u64; 1], // 64-bit bitmask supporting signals 1..64
}
```

### `sigaction_t` Layout
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sigaction_t {
    pub sa_handler: u64,      // Handler address: SIG_DFL (0), SIG_IGN (1), or user function
    pub sa_flags: u32,        // SA_NOCLDSTOP, SA_NOCLDWAIT, SA_SIGINFO, SA_ONSTACK, SA_RESTART, SA_NODEFER, SA_RESETHAND
    pub _pad: u32,            // 32-bit alignment padding
    pub sa_mask: sigset_t,    // Additional mask blocked during handler execution
    pub sa_restorer: u64,     // User-space trampoline restorer pointer
}
```

### `siginfo_t` Layout (56 Bytes)
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct siginfo_t {
    pub si_signo: i32,        // Signal number
    pub si_errno: i32,        // An errno value (if applicable)
    pub si_code: i32,         // Signal code (SI_USER, SI_KERNEL, SI_QUEUE, etc.)
    pub si_pid: i32,          // Sending process ID
    pub si_uid: u32,          // Sending user ID
    pub si_status: i32,       // Exit value or signal status
    pub si_addr: u64,         // Faulting memory address (SIGSEGV/SIGBUS)
    pub si_value: u64,        // Signal payload value (sigval_t for sigqueue)
    pub _pad: [u8; 16],       // Future expansion padding
}
```

---

## 4. x86_64 User Signal Frame & `sigreturn` Trampoline ABI

When delivering a signal to a Ring 3 process on x86_64, the kernel constructs a frame on the user stack (aligned to 16 bytes):

### x86_64 User Signal Frame Layout
```text
+-------------------------------------------------------+
|  Padding (16-byte stack alignment)                    |
+-------------------------------------------------------+
|  Restorer / Trampoline Code (or pointer to sa_restorer)|
|  e.g., mov $15, %rax ; syscall (sys_rt_sigreturn)     |
+-------------------------------------------------------+
|  siginfo_t structure (56 bytes)                       |
+-------------------------------------------------------+
|  ucontext_t / Saved Context:                          |
|    - Saved Instruction Pointer (RIP)                 |
|    - Saved Stack Pointer (RSP)                       |
|    - Saved Flags (RFLAGS - Ring 3 flags only)        |
|    - General Registers (RAX, RBX, RCX, RDX, RSI,     |
|      RDI, RBP, R8-R15)                                |
|    - Saved Signal Mask (sigset_t - 8 bytes)          |
+-------------------------------------------------------+  <- User RSP upon entering handler
```

### Handler Entry Arguments (x86_64 Calling Convention)
- `RDI`: Signal Number (`si_signo`)
- `RSI`: Pointer to `siginfo_t` (if `SA_SIGINFO` set) or 0
- `RDX`: Pointer to `ucontext_t` (if `SA_SIGINFO` set) or 0
- `RSP`: Points to top of user signal frame, aligned to 16 bytes.

### `sigreturn` Validation Safety Rules
1. **Canonical Address Enforcement**: User stack pointer (`RSP`) and instruction pointer (`RIP`) restored from frame must be canonical user addresses (`< 0x0000_7FFF_FFFF_FFFF`).
2. **Privilege Boundary Guard**: Saved `RFLAGS` restored via `sigreturn` must strip privileged processor flags (e.g., IOPL, IF override, system flags) and enforce Ring 3 execution status (`CS` = 0x2B / `SS` = 0x33).
3. **Stack Bounds Guard**: The signal frame must reside entirely within mapped, writeable user stack pages.
4. **Frame Corruption Action**: If `sigreturn` validates a corrupted or forged frame, the kernel immediately terminates the process with `SIGSEGV` or `SIGKILL`.

---

## 5. Supported POSIX Subset & Architecture Limits

### Supported Features
- POSIX standard signal dispositions (`SIG_DFL`, `SIG_IGN`, custom handler).
- `sigaction` flags: `SA_NODEFER`, `SA_RESETHAND`, `SA_SIGINFO`, `SA_RESTART` (subset for `read`/`write`).
- Process-wide dispositions with thread-specific masks (`pthread_sigmask` / `rt_sigprocmask`).
- Real-time signals (`SIGRTMIN`..`SIGRTMAX`) with FIFO payload queuing.
- `signalfd` event-loop monitoring.

### Current Limitations & Unsupported Features
- `sigaltstack` (`SA_ONSTACK`) requires VFS guard page allocation (targeted in PR 10).
- Multi-architecture signal frames (AArch64 and RISC-V 64) are specified but unintegrated until user-mode execution is validated on those architectures (targeted in PR 17).
- Full glibc signal trampoline compatibility is intentionally excluded in favor of clean kernel-provided restorer code and direct `sys_rt_sigreturn` dispatch.
