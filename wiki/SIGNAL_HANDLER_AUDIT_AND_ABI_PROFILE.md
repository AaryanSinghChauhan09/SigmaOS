# SigmaOS Signal Subsystem Audit & ABI Profile

## Executive Summary

This document provides the signal subsystem audit and architecture ABI profile for SigmaOS, establishing POSIX.1-2024 compliance baselines alongside explicit Linux and FreeBSD extension policies.

---

## 1. Subsystem Audit Matrix

| Component | Audit Status | Current Architecture | Next Steps / Milestone |
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

When a signal handler completes and invokes `sys_sigreturn`, the kernel restores the interrupted execution context from the System V x86_64 ABI `ucontext_t` structure (saved registers, `sigset_t`, and `stack_t`) that was written onto the user stack at delivery time. Before restoring, it performs strict validation checks:

1. **Stack Pointer Alignment:** The user stack pointer (`rsp`) must reside within canonical user-space memory (`0x0000_0000_0000_0000` to `0x0000_7FFF_FFFF_FFFF`) and align to a 16-byte boundary.
2. **Magic Token Verification:** The frame signature at the expected offset must equal `0x5349474652414D45` (`"SIGFRAME"`).
3. **Instruction Pointer Verification:** Saved `rip` must reside in executable user memory and cannot point into kernel address space (`0xFFFF_8000_0000_0000` and above).
4. **RFLAGS Filtering:** Saved `rflags` are masked with `0x0020_0202` (enforcing User Interrupt Enable `IF=1`, clearing `IOPL` and system control flags) to prevent privilege escalation.
5. **Signal Mask Restoration:** The saved signal mask is restored to the thread state after filtering out uncatchable signals (`SIGKILL` and `SIGSTOP`).
6. **Alternate Stack Validation:** If `SA_ONSTACK` was configured, the kernel verifies that the alternate stack registered via `sigaltstack` has valid guard pages and adequate headroom before allowing a frame to be placed on it.

---

## 2. ABI Profile & Layout Specification (x86_64)

### Signal Number Mapping
- `1`: `SIGHUP` (Hangup) - Default: Terminate
- `2`: `SIGINT` (Interrupt) - Default: Terminate
- `3`: `SIGQUIT` (Quit) - Default: Core/Terminate
- `4`: `SIGILL` (Illegal Instruction) - Default: Core/Terminate
- `5`: `SIGTRAP` (Trace/Breakpoint) - Default: Core/Terminate
- `6`: `SIGABRT` (Abort) - Default: Core/Terminate
- `7`: `SIGBUS` (Bus Error) - Default: Core/Terminate
- `8`: `SIGFPE` (Floating Point Exception) - Default: Core/Terminate
- `9`: `SIGKILL` (Kill, Uncatchable) - Default: Terminate (Forced)
- `10`: `SIGUSR1` (User Signal 1) - Default: Terminate
- `11`: `SIGSEGV` (Segmentation Fault) - Default: Core/Terminate
- `12`: `SIGUSR2` (User Signal 2) - Default: Terminate
- `13`: `SIGPIPE` (Broken Pipe) - Default: Terminate
- `14`: `SIGALRM` (Alarm Timer) - Default: Terminate
- `15`: `SIGTERM` (Termination Signal) - Default: Terminate
- `17`: `SIGCHLD` (Child Status Changed) - Default: Ignore
- `18`: `SIGCONT` (Continue Stopped Process) - Default: Continue
- `19`: `SIGSTOP` (Stop, Uncatchable) - Default: Stop (Forced)
- `20`: `SIGTSTP` (Terminal Stop) - Default: Stop
- `32..64`: Real-Time Signals (`SIGRTMIN` .. `SIGRTMAX`) - Default: Terminate

### `sigset_t` Layout
- 64-bit integer bitfield (`u64`). Bit $N-1$ represents signal number $N$.

### `UserSignalFrame` Layout (x86_64)
- **Offset `0x00`**: `rax` (Saved Return Value / Syscall Number)
- **Offset `0x08`**: `rbx`
- **Offset `0x10`**: `rcx`
- **Offset `0x18`**: `rdx`
- **Offset `0x20`**: `rsi`
- **Offset `0x28`**: `rdi` (First argument to handler: signal number)
- **Offset `0x30`**: `rbp`
- **Offset `0x38`**: `rsp` (Saved User Stack Pointer)
- **Offset `0x40`**: `r8` .. `r15`
- **Offset `0x80`**: `rip` (Interrupted Instruction Pointer)
- **Offset `0x88`**: `rflags` (Interrupted RFLAGS)
- **Offset `0x90`**: `saved_mask` (Previous Thread Signal Mask `SigSet`)
- **Offset `0x98`**: `siginfo` (`SigInfo` payload if `SA_SIGINFO`)
- **Offset `0xC0`**: `trampoline_addr` (Address of Ring 3 `sigreturn` stub)

---

## 3. Security & Validation Rules

1. **User Pointer & Memory Bounds:**
   - Saved user stack pointer (`rsp`) must be aligned to 16 bytes per System V x86_64 ABI.
   - Stack region must lie strictly within user canonical address space (`0x0000_0000_0000_0000` .. `0x0000_7FFF_FFFF_FFFF`).

2. **RFLAGS Security:**
   - `sys_sigreturn` MUST NOT restore privileged hardware rflags bits (e.g. `IOPL`, `NT`, or kernel privilege levels). Ring 3 privilege flags (`0x202` - IF set, user mode) are forcibly masked during frame validation.

3. **Frame Magic & Tamper Protection:**
   - Frames contain a kernel magic cookie (`0x5349_4746_5241_4d45` / `"SIGFRAME"`). Forged or corrupted frames trigger immediate process termination (`SIGSEGV` / force kill).

---

## 4. Architecture Limitations & Future Work

- **x86_64:** Fully modeled frame construction & sigreturn validation.
- **AArch64 & RISC-V:** Signal numbers and POSIX dispositions defined; hardware frame layout deferred until Ring 3 execution for those architectures is integrated (PR 17).
