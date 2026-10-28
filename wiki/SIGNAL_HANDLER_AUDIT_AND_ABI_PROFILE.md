# SigmaOS Signal Subsystem Audit & ABI Profile

## Executive Summary

This document provides the signal subsystem audit and architecture ABI profile for SigmaOS, establishing POSIX.1-2024 compliance baselines alongside explicit Linux and FreeBSD extension policies.

---

## 1. Subsystem Audit Matrix

| Component | Audit Status | Current Architecture | Next Steps / Milestone |
|---|---|---|---|
| **Constants & Numbers** | Standard (1..31) & RT (32..64) | `SignalNumber` enum in `src/signal/sovereign_signal_development_and_pr_suite.rs` | Verify bitfield conversions in `sigset_t` |
| **SigAction & Dispositions** | Modeled & Unit-Tested | `SigAction`, `SigHandler`, `SigActionFlags` | Connect syscall ABI to process table |
| **Process vs Thread State** | Modeled | `ProcessSignalState` & `ThreadSignalState` | Wire per-thread mask check in scheduler return path |
| **Uncatchable Signals** | Enforced | `SIGKILL` (9) & `SIGSTOP` (19) guard | Enforce across all dispatch and mask functions |
| **Signal Frame Construction** | Modeled / x86_64 ABI | `UserSignalFrame` struct with saved registers & mask | Implement Ring 3 stack frame writer in kernel |
| **SigReturn Validation** | Modeled / Validated | Frame address, stack canonicality, privilege flags check | Connect `sys_sigreturn` handler to kernel dispatch |
| **Syscall Interruption & Restart** | Modeled | `EINTR` & `SA_RESTART` flag handling | Integrate with blocking VFS/IPC wait queues |
| **Real-time Signals Queue** | Bounded & Tested | `MAX_RT_QUEUE_PER_PROCESS` limit (32 items) | Expose `sigqueue` syscall |
| **Signalfd Extension** | Modeled | `signalfd` file handle & poll interface | Wire event notifications to FD readiness |

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
