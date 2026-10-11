# SigmaOS Signal Handling Subsystem Audit & Architecture ABI Profile

**Document Title:** `SIGNAL_HANDLER_AUDIT_AND_ABI_PROFILE.md`
**Version:** 1.0.0-PROD-AUDIT
**Status:** Prototype Level / Unintegrated Runtime
**Target Architecture:** x86_64 System V AMD64 ABI (AArch64 & RISC-V 64 planned)
**Specification References:** POSIX.1-2024 / POSIX.1-2017 `sigaction(2)`, Linux x86_64 Syscall ABI, FreeBSD `sys/kern/kern_sig.c`

---

## 1. EXECUTIVE SUMMARY & SYSTEM INTEGRATION STATUS

SigmaOS contains multiple signal handling modules and POSIX compatibility shims across its kernel, IPC, and compatibility layers. A comprehensive repository audit confirms that while data structures, signal numbers, bitmasks, and unit-tested managers exist in pure Rust, **signal handling is currently at Prototype level and remains unintegrated into the kernel execution lifecycle**.

### Evidence Matrix & Status Classification

| Signal Component | Location in Codebase | Unit Test Evidence | Syscall Dispatch | Kernel Entry/Exit Integration | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Capability Async Dispatcher** | `src/signal/mod.rs` | `test_raise_and_poll_signal` (`rustc --test`) | Unwired | N/A | Prototype |
| **BSD/Linux IPC Signals** | `src/ipc/signals.rs` | `SignalQueue` internal tests | Unwired | N/A | Prototype |
| **POSIX Signal Manager** | `src/compatibility/posix_signals.rs` | `test_signal_delivery_engine` (`rustc --test`) | `sys_sigaction` stub | Unwired | Prototype |
| **Kernel POSIX Signals** | `src/kernel/signal.rs` | `SignalQueue` internal tests | Unwired | Unwired | Prototype |
| **Process Signal Manager** | `src/kernel/proc/signals.rs` | `ProcessSignalManager` internal tests | Unwired | Scheduler context switch unwired | Prototype |
| **SigmaOS Core Signal Engine** | `src/kernel/sigma_signal.rs` | `test_signal_send_and_receive` (`rustc --test`) | Unwired | SignalFrame defined, trap unwired | Prototype |
| **Linux `signalfd` Engine** | `src/kernel/signalfd.rs` | `SignalFd` internal tests | Unwired | VFS/Poll unwired | Prototype |
| **Syscall Dispatch Shims** | `src/syscall/posix_compat.rs`, `dispatch.rs` | `test_sigaction_invalid_signal` | Returns mock `0` | Return-to-user path unwired | Stub |

### Key Architectural Findings
1. **Unintegrated Ring 3 Delivery:** No user-mode process in SigmaOS currently receives an asynchronous signal or enters a user-mode handler. The kernel assembly entry/exit paths in `src/arch/x86_64/` do not construct user stack frames (`SignalFrame`) or redirect `RIP` to user handlers.
2. **Missing `sigreturn` Trampoline:** System call 15 (`sys_rt_sigreturn`) is defined in the ABI table (`src/syscall/abi.rs`), but no kernel handler exists to validate forged user frames, restore saved register state, or unmask signals upon handler completion.
3. **Mock Syscall Stubs:** `sys_sigaction` and `sys_sigprocmask` in `src/syscall/posix_compat.rs` validate input bounds (rejecting invalid signal numbers and attempts to catch `SIGKILL`/`SIGSTOP`), but return mock success (`0`) without registering handlers in process/thread signal state.

---

## 2. SIGNAL CONSTANTS & NUMBER MAPPING MATRIX

SigmaOS maintains several internal representations of signal numbers. The canonical mapping aligns with Linux x86_64 / POSIX standard numbers (1–31 for standard signals, 32–64 for real-time signals).

### Multi-Module Signal Constant Alignment

| Signal Name | Value | POSIX Standard | `src/kernel/sigma_signal.rs` | `src/compatibility/posix_signals.rs` | `src/kernel/signal.rs` | `src/ipc/signals.rs` | Default Action |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `SIGHUP` | 1 | Yes | `Signal::SIGHUP` | `PosixSignal::SIGHUP` | `Signal::SIGHUP` | N/A | Terminate |
| `SIGINT` | 2 | Yes | `Signal::SIGINT` | `PosixSignal::SIGINT` | `Signal::SIGINT` | `SignalType::SigInt` | Terminate |
| `SIGQUIT` | 3 | Yes | `Signal::SIGQUIT` | `PosixSignal::SIGQUIT` | `Signal::SIGQUIT` | N/A | Core Dump |
| `SIGILL` | 4 | Yes | `Signal::SIGILL` | `PosixSignal::SIGILL` | `Signal::SIGILL` | N/A | Core Dump |
| `SIGTRAP` | 5 | Yes | `Signal::SIGTRAP` | `PosixSignal::SIGTRAP` | `Signal::SIGTRAP` | N/A | Core Dump |
| `SIGABRT` | 6 | Yes | `Signal::SIGABRT` | `PosixSignal::SIGABRT` | `Signal::SIGABRT` | N/A | Core Dump |
| `SIGBUS` | 7 | Yes | `Signal::SIGBUS` | `PosixSignal::SIGBUS` | `Signal::SIGBUS` | N/A | Core Dump |
| `SIGFPE` | 8 | Yes | `Signal::SIGFPE` | `PosixSignal::SIGFPE` | `Signal::SIGFPE` | N/A | Core Dump |
| `SIGKILL` | 9 | Yes (Uncatchable) | `Signal::SIGKILL` | `PosixSignal::SIGKILL` | `Signal::SIGKILL` | `SignalType::SigKill` | Terminate (Force) |
| `SIGUSR1` | 10 | Yes | `Signal::SIGUSR1` | `PosixSignal::SIGUSR1` | `Signal::SIGUSR1` | `SignalType::SigUsr1` | Terminate |
| `SIGSEGV` | 11 | Yes | `Signal::SIGSEGV` | `PosixSignal::SIGSEGV` | `Signal::SIGSEGV` | `SignalType::SigSegv` | Core Dump |
| `SIGUSR2` | 12 | Yes | `Signal::SIGUSR2` | `PosixSignal::SIGUSR2` | `Signal::SIGUSR2` | `SignalType::SigUsr2` | Terminate |
| `SIGPIPE` | 13 | Yes | `Signal::SIGPIPE` | `PosixSignal::SIGPIPE` | `Signal::SIGPIPE` | N/A | Terminate |
| `SIGALRM` | 14 | Yes | `Signal::SIGALRM` | `PosixSignal::SIGALRM` | `Signal::SIGALRM` | N/A | Terminate |
| `SIGTERM` | 15 | Yes | `Signal::SIGTERM` | `PosixSignal::SIGTERM` | `Signal::SIGTERM` | `SignalType::SigTerm` | Terminate |
| `SIGSTKFLT`| 16 | Linux | `Signal::SIGSTKFLT`| `PosixSignal::SIGSTKFLT`| `Signal::SIGSTKFLT`| N/A | Terminate |
| `SIGCHLD` | 17 | Yes | `Signal::SIGCHLD` | `PosixSignal::SIGCHLD` | `Signal::SIGCHLD` | N/A | Ignore |
| `SIGCONT` | 18 | Yes | `Signal::SIGCONT` | `PosixSignal::SIGCONT` | `Signal::SIGCONT` | N/A | Continue |
| `SIGSTOP` | 19 | Yes (Uncatchable) | `Signal::SIGSTOP` | `PosixSignal::SIGSTOP` | `Signal::SIGSTOP` | N/A | Stop (Force) |
| `SIGTSTP` | 20 | Yes | `Signal::SIGTSTP` | `PosixSignal::SIGTSTP` | `Signal::SIGTSTP` | N/A | Stop |
| `SIGTTIN` | 21 | Yes | `Signal::SIGTTIN` | `PosixSignal::SIGTTIN` | `Signal::SIGTTIN` | N/A | Stop |
| `SIGTTOU` | 22 | Yes | `Signal::SIGTTOU` | `PosixSignal::SIGTTOU` | `Signal::SIGTTOU` | N/A | Stop |
| `SIGURG` | 23 | Yes | `Signal::SIGURG` | `PosixSignal::SIGURG` | `Signal::SIGURG` | N/A | Ignore |
| `SIGXCPU` | 24 | Yes | `Signal::SIGXCPU` | `PosixSignal::SIGXCPU` | `Signal::SIGXCPU` | N/A | Core Dump |
| `SIGXFSZ` | 25 | Yes | `Signal::SIGXFSZ` | `PosixSignal::SIGXFSZ` | `Signal::SIGXFSZ` | N/A | Core Dump |
| `SIGVTALRM`| 26 | Yes | `Signal::SIGVTALRM`| `PosixSignal::SIGVTALRM`| `Signal::SIGVTALRM`| N/A | Terminate |
| `SIGPROF` | 27 | Yes | `Signal::SIGPROF` | `PosixSignal::SIGPROF` | `Signal::SIGPROF` | N/A | Terminate |
| `SIGWINCH` | 28 | Yes | `Signal::SIGWINCH` | `PosixSignal::SIGWINCH` | `Signal::SIGWINCH` | N/A | Ignore |
| `SIGIO` | 29 | Linux/BSD | `Signal::SIGIO` | N/A | `Signal::SIGIO` | N/A | Terminate |
| `SIGPWR` | 30 | Linux | `Signal::SIGPWR` | N/A | `Signal::SIGPWR` | N/A | Terminate |
| `SIGSYS` | 31 | Yes | `Signal::SIGSYS` | N/A | `Signal::SIGSYS` | N/A | Core Dump |
| `SIGRTMIN`..`SIGRTMAX` | 32..64 | POSIX Real-Time | Supported in `src/kernel/signal.rs` & `src/kernel/sigma_signal.rs` | Terminate |

---

## 3. `sigaction` SEMANTICS & KERNEL SAFETY RULES

POSIX.1-2024 mandates explicit handler dispositions via `sigaction`. In SigmaOS, `sigaction` semantics are implemented in `src/kernel/sigma_signal.rs` (`SigAction`) and `src/compatibility/posix_signals.rs` (`SignalAction`).

### Kernel Safety Rules & Invariants
1. **Uncatchable Signals Protection:** `SIGKILL` (9) and `SIGSTOP` (19) cannot be caught, ignored, or blocked. Attempts to set handlers or mask `SIGKILL` or `SIGSTOP` must return `-EINVAL` (`-22`).
   - Implemented in `src/syscall/posix_compat.rs`:
     ```rust
     if signum == 9 || signum == 19 { return -22; } // Can't catch SIGKILL/SIGSTOP
     ```
   - Implemented in `src/kernel/sigma_signal.rs`:
     ```rust
     if sig == Signal::SIGKILL || sig == Signal::SIGSTOP {
         return Err("SIGKILL and SIGSTOP cannot be caught or ignored");
     }
     ```
2. **Signal Masking during Handler Execution:**
   - Unless `SA_NODEFER` is set, the signal being delivered is automatically added to the thread's blocked signal mask during handler execution.
   - Any additional signals specified in `sa_mask` are blocked for the duration of the handler.
   - The thread's original mask is restored upon return via `sys_rt_sigreturn`.
3. **Supported `sa_flags` Bitmask Matrix:**

| Flag | Value | Description | Implementation Status |
| :--- | :--- | :--- | :--- |
| `SA_NOCLDSTOP` | `0x00000001` | Do not generate `SIGCHLD` when child processes stop. | Implemented in `PosixSignalManager` |
| `SA_NOCLDWAIT` | `0x00000002` | Do not create zombie processes upon child termination. | Implemented in `PosixSignalManager` |
| `SA_SIGINFO` | `0x00000004` | Pass `siginfo_t` and `ucontext_t` to 3-argument handler. | Structs defined in `sigma_signal.rs` |
| `SA_ONSTACK` | `0x08000000` | Execute handler on alternate stack registered via `sigaltstack`. | Struct defined in `posix_signals.rs` |
| `SA_RESTART` | `0x10000000` | Automatically restart interrupted system calls. | Flag parsed, syscall restart pending |
| `SA_NODEFER` | `0x40000000` | Do not automatically block the signal while its handler runs. | Flag parsed |
| `SA_RESETHAND` | `0x80000000` | Reset handler disposition to `SIG_DFL` upon signal entry. | Implemented in `sigma_signal.rs` |

---

## 4. PROCESS VS. THREAD MASK & PENDING SIGNAL MODEL

Linux and POSIX distinguish between **process-directed** and **thread-directed** signals. SigmaOS implements this dual-layer queuing architecture in `src/compatibility/posix_signals.rs` and `src/kernel/sigma_signal.rs`.

```
                    +-------------------------------------+
                    |     Process Signal Disposition      |
                    | (Shared sigaction handlers across TIDs)|
                    +-------------------------------------+
                                       |
                   +-------------------+-------------------+
                   |                                       |
      +-------------------------+             +-------------------------+
      |  Process Pending Queue  |             |  Thread 1 Pending Queue |
      |  (e.g., kill(pid, sig)) |             | (e.g., pthread_kill)    |
      +-------------------------+             +-------------------------+
                   |                                       |
                   +-------------------+-------------------+
                                       |
                                       v
                     +-----------------------------------+
                     |      Eligible Thread Selection    |
                     |  (Target thread mask !is_blocked) |
                     +-----------------------------------+
                                       |
                                       v
                     +-----------------------------------+
                     | Return to User Mode Delivery Point|
                     | (Kernel trap exit / context return)|
                     +-----------------------------------+
```

### Signal Queue & Coalescing Rules
- **Standard Signals (1–31):** Standard signals are non-queued (coalesced). Multiple pending instances of the same standard signal collapse into a single bit setting in `pending_set: SigSet`.
- **Real-Time Signals (32–64):** Real-time signals are queued FIFO up to `MAX_QUEUED_SIGNALS` (64 per process).
- **Delivery Order:** Standard signals are delivered in numerical order (1 to 31), followed by real-time signals in numerical and FIFO order.

---

## 5. x86_64 USER SIGNAL FRAME & `sigreturn` ABI SPECIFICATION

When the kernel delivers a signal to a Ring 3 user thread with a custom handler, it must construct a `SignalFrame` on the user thread's stack and modify the interrupted context before returning to user mode.

### x86_64 User Stack Frame Layout (`SignalFrame`)

Defined in `src/kernel/sigma_signal.rs`:

```rust
#[repr(C)]
pub struct SignalFrame {
    pub rdi: u64,     // Arg 1: Signal number
    pub rsi: u64,     // Arg 2: Pointer to siginfo_t (if SA_SIGINFO)
    pub rdx: u64,     // Arg 3: Pointer to ucontext_t (if SA_SIGINFO)
    pub rcx: u64,
    pub r8:  u64,
    pub r9:  u64,
    pub rax: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,     // Interrupted instruction pointer
    pub rflags: u64,  // Interrupted processor flags
    pub rsp: u64,     // Interrupted stack pointer
    pub mask: SigSet, // Saved signal mask (to restore on sigreturn)
    pub sig: u8,      // Signal number
    pub handler: usize, // Userland handler function address
}
```

### Trampoline Execution & `sigreturn` Sequence
1. **Frame Setup:** The kernel copies `SignalFrame` onto the user stack, maintaining strict 16-byte stack alignment (System V AMD64 ABI requirement).
2. **Registers on Entry to Handler:**
   - `RDI` = Signal number (`sig`)
   - `RSI` = Pointer to `siginfo_t` on user stack (or 0)
   - `RDX` = Pointer to `ucontext_t` on user stack (or 0)
   - `RIP` = Address of user signal handler
   - `RSP` = Address of `SignalFrame` on user stack
   - `RAX` = User return address pointing to the userland trampoline (`__restore_rt`).
3. **Handler Return:** The handler finishes and executes `ret`, transferring execution to the trampoline (`__restore_rt`), which executes system call 15 (`sys_rt_sigreturn`).
4. **`sigreturn` Kernel Validation Profile:**
   - The kernel validates that the `SignalFrame` address is located entirely within canonical user virtual memory (`0x0000_0000_0000_0000 .. 0x0000_7FFF_FFFF_FFFF`).
   - The saved `rflags` register is sanitized: privileged flags (IOPL, IF override, reserved bits) are stripped.
   - Saved `RIP` and `RSP` are verified to be canonical user addresses.
   - Saved signal mask (`mask`) is restored to the thread's signal mask.
   - Interrupted register state (`RAX`, `RBX`, `RCX`, `RDX`, `RSI`, `RDI`, `RBP`, `R8`–`R15`) is restored, and user execution resumes at the interrupted `RIP`.

---

## 6. SYSCALL DISPATCH & API MAPPING

| Syscall Number (x86_64) | Syscall Name | File Location | Subsystem Dispatch Function | Current Status |
| :--- | :--- | :--- | :--- | :--- |
| **13** | `sys_rt_sigaction` | `src/syscall/posix_compat.rs` | `sys_sigaction(signum, act, oldact)` | Stub (validates signum, returns 0) |
| **14** | `sys_rt_sigprocmask` | `src/syscall/posix_compat.rs` | `sys_sigprocmask(how, set, oldset)` | Stub (returns 0) |
| **15** | `sys_rt_sigreturn` | `src/syscall/abi.rs` | Unwired | Unwired (ABI constant declared) |
| **62** | `sys_kill` | `src/syscall/abi.rs` | Unwired | Unwired (ABI constant declared) |
| **289** | `sys_signalfd4` | `src/kernel/signalfd.rs` | `SignalFd::new(mask, flags)` | Independent struct, unwired to VFS |
| **FreeBSD** | `sys_pdkill` | `src/syscall/capsicum_syscalls.rs` | `sys_pdkill(state, fd, signal)` | Capability-gated via `CAP_PDKILL` |

---

## 7. UNSUPPORTED FLAGS & KNOWN LIMITATIONS

1. **Unsupported Flags:**
   - Non-standard Linux kernel internal flags (`SA_INTERRUPT`, `SA_TRAP`) are not supported.
   - Floating-point / AVX / AMX extended register state (`fpstate` / `xstate`) saving in `ucontext_t` is currently stubbed to standard GPRs.
2. **Architecture Limitations:**
   - **x86_64:** Core struct definitions and signal mask bitwise operations are fully verified; assembly frame generation on trap exit is unwired.
   - **AArch64 & RISC-V 64:** Signal frame layouts and sigreturn trampolines are currently undefined and scheduled for PR 17.

---

## 8. PROPOSED PR SEQUENCE ROADMAP (PR 2 THROUGH PR 18)

| PR # | PR Title | Primary Deliverable Scope | Status |
| :--- | :--- | :--- | :--- |
| **PR 1** | `docs(signal-handler): audit signal path and define ABI profile` | Comprehensive audit, ABI specification, and status matrix. | **Completed** |
| **PR 2** | `test(signal-handler): add ABI and signal-frame layout checks` | Compile-time and runtime size/offset/alignment tests for `SignalFrame` and `sigaction`. | Planned |
| **PR 3** | `feat(signal): connect signal syscalls to kernel dispatch` | Wire `sys_rt_sigaction`, `sys_rt_sigprocmask`, and `sys_kill` to real kernel state. | Planned |
| **PR 4** | `feat(signal): add per-thread masks and pending signal queues` | Separate process dispositions from thread masks and thread pending queues. | Planned |
| **PR 5** | `feat(signal-handler): deliver default actions at safe return points` | Deliver `SIG_DFL` default actions (terminate, ignore, stop, continue) at return-to-user points. | Planned |
| **PR 6** | `feat(signal-handler): construct validated x86_64 user signal frame` | Build `SignalFrame` on Ring 3 user stack and redirect `RIP` to user handler. | Planned |
| **PR 7** | `feat(signal-handler): implement x86_64 sigreturn validation` | Implement `sys_rt_sigreturn` frame validation, register restoration, and mask unblocking. | Planned |
| **PR 8** | `feat(signal-handler): implement action masks and restart semantics` | Implement `sa_mask`, `SA_NODEFER`, `SA_RESETHAND`, and `SA_RESTART` system call restart logic. | Planned |
| **PR 9** | `feat(signal-handler): add siginfo and SA_SIGINFO subset` | Support 3-argument handlers with populated `siginfo_t` payloads. | Planned |
| **PR 10**| `feat(signal-handler): add alternate stack support` | Implement `sigaltstack` syscall and `SA_ONSTACK` frame allocation. | Planned |
| **PR 11**| `feat(signal): integrate SIGCHLD and wait/reap lifecycle` | Generate `SIGCHLD` on child exit/stop/continue and integrate with `waitpid`. | Planned |
| **PR 12**| `feat(signal): add synchronous signal-wait APIs` | Implement `sigwait`, `sigwaitinfo`, and `sigtimedwait` synchronous acceptance shims. | Planned |
| **PR 13**| `feat(signal): add queued real-time signal support with limits` | Implement POSIX real-time signals (32–64) with FIFO queues and resource caps. | Planned |
| **PR 14**| `feat(signal): integrate signalfd as Linux-specific extension` | Connect `signalfd` file descriptors to process pending queues and poll/epoll. | Planned |
| **PR 15**| `feat(libc): expose signal APIs and safe handler examples` | Provide C `signal.h` shims, standard macros, and async-signal-safe examples. | Planned |
| **PR 16**| `test(signal-handler): add fault-injection and concurrency suite` | Stress tests for nested signals, stack exhaustion, frame forgery, and concurrent signals. | Planned |
| **PR 17**| `feat(signal-handler): validate AArch64 and RISC-V handler frames` | Architecture-specific frame construction and `sigreturn` for AArch64 and RISC-V 64. | Planned |
| **PR 18**| `docs(signal-handler): publish API, safety, and support matrix` | Publish final userland API documentation, async-signal-safety guidance, and support matrix. | Planned |
