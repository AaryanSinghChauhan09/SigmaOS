# SigmaOS Signal Handler Audit and ABI Profile Specification

## 1. Subsystem Audit

An audit of signal-related code across the SigmaOS codebase reveals several implementation components:

1. **`src/signal/mod.rs` (`SignalDispatcher`):** Implements capability-gated asynchronous signal queuing (`SovereignSignal`: `Terminate`, `Interrupt`, `PageFault`, `PowerStateTransition`) with fixed 16-slot pending queues. Tests confirm capability gating, queue bounds, and FIFO polling.
2. **`src/signal/sovereign_signal_development_and_pr_suite.rs` (`SovereignSignalDevelopmentAndPrSuite`):** Implements zero-dependency POSIX.1-2024 signal primitives, `SignalNumber` (1..64), `SigSet` (64-bit bitmask), `SigAction`, `SigInfo`, `UserSignalFrame` for x86_64, `sigreturn` frame validator, and PR execution engine.
3. **`src/ipc/signals.rs` (`ProcessSignalState`, `SignalDeliverySystem`):** IPC-layer signal management supporting `SignalType`, `SignalDisposition` (`Default`, `Ignore`, `CustomHandler`), signal masking (`set_mask`), real-time signal payloads, and capability checks.
4. **`src/compatibility/posix_signals.rs`:** Defines C-compatible `sigset_t`, `sigaction_t`, `stack_t`, `siginfo_t`, and POSIX constants (`SIG_BLOCK`, `SIG_UNBLOCK`, `SIG_SETMASK`, `SA_NOCLDSTOP`, `SA_SIGINFO`, `SA_RESTART`).
5. **`src/kernel/signal.rs` & `src/kernel/sigma_signal.rs`:** Kernel-space signal pending sets, default actions (`Term`, `Ign`, `Core`, `Stop`, `Cont`), and process signal dispatch routines.
6. **`src/kernel/signalfd.rs`:** Linux-compatible `signalfd` implementation allowing signals to be read as file descriptor events (`signalfd_siginfo`).
7. **`tests/signal/signal_test.rs`:** Standalone contract test verifying that `SIGKILL` (signal 9) and `SIGSTOP` (signal 19) cannot be added to a blocked signal mask.

---

## 2. Signal ABI Profile Specifications

### 2.1 Signal Numbers and Dispositions

SigmaOS defines standard POSIX signals (1..31) and Real-Time signals (32..64):

| Number | Name | Default Action | Description |
|---|---|---|---|
| 1 | `SIGHUP` | Terminate | Hangup detected on controlling terminal |
| 2 | `SIGINT` | Terminate | Interrupt from keyboard (`Ctrl+C`) |
| 3 | `SIGQUIT` | Terminate with Core | Quit from keyboard (`Ctrl+\`) |
| 4 | `SIGILL` | Terminate with Core | Illegal instruction |
| 5 | `SIGTRAP` | Terminate with Core | Trace/breakpoint trap |
| 6 | `SIGABRT` | Terminate with Core | Abort signal from `abort()` |
| 7 | `SIGBUS` | Terminate with Core | Bus error (bad memory access) |
| 8 | `SIGFPE` | Terminate with Core | Floating-point exception |
| 9 | `SIGKILL` | Terminate (Uncatchable) | Kill signal (cannot be caught, ignored, or blocked) |
| 10 | `SIGUSR1` | Terminate | User-defined signal 1 |
| 11 | `SIGSEGV` | Terminate with Core | Segmentation fault (invalid memory reference) |
| 12 | `SIGUSR2` | Terminate | User-defined signal 2 |
| 13 | `SIGPIPE` | Terminate | Broken pipe: write to pipe with no readers |
| 14 | `SIGALRM` | Terminate | Timer signal from `alarm()` |
| 15 | `SIGTERM` | Terminate | Termination signal |
| 16 | `SIGSTKFLT` | Terminate | Stack fault on coprocessor |
| 17 | `SIGCHLD` | Ignore | Child status changed or terminated |
| 18 | `SIGCONT` | Continue | Continue executing if stopped |
| 19 | `SIGSTOP` | Stop (Uncatchable) | Stop process execution (cannot be caught, ignored, or blocked) |
| 20 | `SIGTSTP` | Stop | Stop typed at terminal (`Ctrl+Z`) |
| 21 | `SIGTTIN` | Stop | Background process attempting terminal read |
| 22 | `SIGTTOU` | Stop | Background process attempting terminal write |
| 23 | `SIGURG` | Ignore | Urgent condition on socket |
| 24 | `SIGXCPU` | Terminate with Core | CPU time limit exceeded |
| 25 | `SIGXFSZ` | Terminate with Core | File size limit exceeded |
| 26 | `SIGVTALRM` | Terminate | Virtual timer expired |
| 27 | `SIGPROF` | Terminate | Profiling timer expired |
| 28 | `SIGWINCH` | Ignore | Window size change |
| 29 | `SIGIO` | Terminate | I/O now possible |
| 30 | `SIGPWR` | Terminate | Power failure / restart |
| 31 | `SIGSYS` | Terminate with Core | Bad system call |
| 32..64 | `SIGRTMIN`..`SIGRTMAX` | Terminate | Real-Time queued signals |

### 2.2 Data Structure Layouts

#### `sigset_t` (64-bit Bitmask)
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SigSet {
    pub bits: u64, // Bit (N - 1) corresponds to signal N (1..64)
}
```

#### `sigaction_t` Layout
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SigAction {
    pub sa_handler: usize,    // Pointer to handler function (SIG_DFL=0, SIG_IGN=1)
    pub sa_flags: u32,        // SA_NOCLDSTOP, SA_NOCLDWAIT, SA_SIGINFO, SA_ONSTACK, SA_RESTART, SA_NODEFER, SA_RESETHAND
    pub sa_mask: SigSet,      // Additional signals blocked during handler execution
    pub sa_restorer: usize,   // Pointer to signal return trampoline (__restore_rt)
}
```

#### `siginfo_t` Layout
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SigInfo {
    pub si_signo: i32,        // Signal number
    pub si_errno: i32,        // Error number (0 if none)
    pub si_code: i32,         // Signal code (SI_USER, SI_KERNEL, SI_QUEUE, SI_TIMER, SI_TKILL)
    pub si_pid: u32,          // Sending process ID
    pub si_uid: u32,          // Real user ID of sender
    pub si_addr: usize,       // Faulting memory address (for SIGSEGV, SIGBUS, SIGILL, SIGFPE)
    pub si_status: i32,       // Exit value or signal status (for SIGCHLD)
    pub si_value: u64,        // Signal payload value (for sigqueue / RT signals)
}
```

### 2.3 x86_64 User Signal Frame Layout

When delivering a caught signal to a user-space handler, the kernel allocates and initializes a frame on the user stack aligned to a 16-byte boundary:

```text
Higher Addresses (User Stack)
+------------------------------------+
| Red Zone Guard (128 bytes)         |  (if applicable)
+------------------------------------+
| Magic Token (0x5349474652414D45)   |  "SIGFRAME" validation signature
+------------------------------------+
| Saved Instruction Pointer (RIP)    |  Interrupted user execution address
+------------------------------------+
| Saved Stack Pointer (RSP)          |  Interrupted user stack pointer
+------------------------------------+
| Saved CPU Flags (RFLAGS)           |  Masked to verify user-level flags (e.g. IF=1, IOPL=0)
+------------------------------------+
| Saved General Purpose Registers    |  RAX, RBX, RCX, RDX, RSI, RDI, RBP, R8..R15
+------------------------------------+
| Saved Signal Mask (SigSet)         |  Thread signal mask prior to signal delivery
+------------------------------------+
| SigInfo Struct (siginfo_t)         |  Populated if SA_SIGINFO flag is set
+------------------------------------+
| Trampoline Return Address          |  Points to sa_restorer or kernel VDSO sigreturn stub
+------------------------------------+  <- User RSP at Handler Entry
Lower Addresses
```

### 2.4 `sigreturn` Frame Validation Rules

When a signal handler completes and invokes `sys_sigreturn`, the kernel performs strict validation checks before restoring execution context:

1. **Stack Pointer Alignment:** The user stack pointer (`rsp`) must reside within canonical user-space memory (`0x0000_0000_0000_0000` to `0x0000_7FFF_FFFF_FFFF`) and align to a 16-byte boundary.
2. **Magic Token Verification:** The frame signature at the expected offset must equal `0x5349474652414D45` (`"SIGFRAME"`).
3. **Instruction Pointer Verification:** Saved `rip` must reside in executable user memory and cannot point into kernel address space (`0xFFFF_8000_0000_0000` and above).
4. **RFLAGS Filtering:** Saved `rflags` are masked with `0x0020_0202` (enforcing User Interrupt Enable `IF=1`, clearing `IOPL` and system control flags) to prevent privilege escalation.
5. **Signal Mask Restoration:** The saved signal mask is restored to the thread state after filtering out uncatchable signals (`SIGKILL` and `SIGSTOP`).

---

## 3. System Call Dispatch Integration

Signal system calls connect to kernel dispatch handlers through safety wrappers ensuring user-space memory access checks:

- `sys_sigaction(sig: i32, act: *const SigAction, oact: *mut SigAction) -> i32`
- `sys_sigprocmask(how: i32, set: *const SigSet, oset: *mut SigSet) -> i32`
- `sys_kill(pid: i32, sig: i32) -> i32`
- `sys_tgkill(tgid: i32, tid: i32, sig: i32) -> i32`
- `sys_sigpending(set: *mut SigSet) -> i32`
- `sys_sigreturn() -> !`
- `sys_signalfd4(fd: i32, mask: *const SigSet, sizemask: usize, flags: i32) -> i32`

---

## 4. POSIX Compatibility and Deviations

### Supported POSIX Subset
- `sigaction` disposition control (`SIG_DFL`, `SIG_IGN`, custom handler pointers).
- Signal masks (`SIG_BLOCK`, `SIG_UNBLOCK`, `SIG_SETMASK`).
- Automatic self-blocking of delivered signal during handler execution.
- Uncatchable signal restrictions for `SIGKILL` and `SIGSTOP`.
- `SA_NODEFER`, `SA_RESETHAND`, `SA_SIGINFO`, and `SA_RESTART` flag handling.

### Deviations and Non-Standard Extensions
- `signalfd4` is supported as a Linux-inspired extension; it is explicitly non-POSIX.
- Capability-gated signal routing (`IPCCapability`) provides sovereign sandboxing constraints beyond standard POSIX permission models.
