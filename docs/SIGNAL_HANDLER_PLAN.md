# SigmaOS Signal Handler Master Architecture and Plan

This document establishes the master architecture, POSIX.1-2024 compliance baseline, x86_64 signal frame and `sigreturn` safety rules, 18-PR implementation roadmap, periodic maintenance cadences, and review gates for the SigmaOS Signal Handling Subsystem.

## 1. System Overview and Core Distinction

In SigmaOS, a clear architectural distinction is maintained between the **signal subsystem** and **signal handlers**:
- **Signal Subsystem:** The kernel-side infrastructure responsible for generating, queuing, masking, selecting, and delivering signals to target processes or threads.
- **Signal Handler:** The user-space callback executed by an architecture-specific signal frame constructed on the user stack and returned from via a validated signal trampoline (`sigreturn`).

`sigaction()` is the primary POSIX interface for configuring signal dispositions. Implementing `sigaction()` alone does not constitute handler delivery; real delivery requires context saving, user stack frame construction, instruction pointer redirection, and trampoline return.

## 2. Required Semantics and Safety Rules

1. **Default to `sigaction` Semantics:** `signal()` is provided solely as a libc compatibility wrapper. All kernel signal disposition handling follows POSIX `sigaction()` semantics.
2. **Process vs. Thread Signal Targeting:**
   - Process-directed signals (e.g., `kill(pid, sig)`) are queued in the process-level pending signal set and delivered to any eligible thread whose mask does not block the signal.
   - Thread-directed signals (e.g., `pthread_kill(tid, sig)` or synchronous CPU exceptions like `SIGSEGV`, `SIGFPE`) are queued in the thread's pending signal set and delivered specifically to that thread.
3. **Protected Handler Execution State:**
   - On delivery, the kernel saves the interrupted register context (`rip`, `rsp`, `rflags`, general registers), signal mask, and alternate stack state into a user-space frame (`ucontext_t` / `sigframe`).
   - Execution redirects to the handler with `rdi` = signal number, `rsi` = `siginfo_t*`, and `rdx` = `ucontext_t*`.
   - The handler returns via a kernel-provided trampoline that issues a `sigreturn` system call.
4. **Uncatchable Signals:** `SIGKILL` (9) and `SIGSTOP` (19) cannot be caught, ignored, or blocked. The kernel explicitly rejects attempts to modify their dispositions or add them to signal masks.
5. **Nesting and Action Masks:**
   - The delivered signal is automatically added to the thread's signal mask during handler execution unless `SA_NODEFER` is set.
   - Any signals specified in `sa_mask` are also blocked while the handler runs.
   - The original signal mask is restored upon safe return via `sigreturn`.
6. **Syscall Interruption and Restart Semantics:**
   - Blocking system calls interrupted by a caught signal return `-EINTR` by default.
   - If the handler was installed with `SA_RESTART` and the system call is restartable (e.g., `read`, `write`, `waitpid`), the kernel rewinds the instruction pointer to re-execute the system call upon handler return.
7. **Async-Signal Safety and Privilege Isolation:**
   - Kernel frame construction and return validation must execute in bounded $O(1)$ time without dynamic memory allocations or unhandled fault conditions.
   - The kernel never trusts user-modified privileged register state (e.g., CS/SS descriptor selectors, CPL level, or RFLAGS interrupt flags) restored via `sigreturn`.

## 3. 18-PR Sequence Roadmap

| PR | Title | Scope & Acceptance Criteria |
|---|---|---|
| 1 | `docs(signal-handler): audit signal path and define ABI profile` | Audit signal constants, `sigaction`, masks, pending state, syscall dispatch, process/thread model, x86_64 return path, libc, and `signalfd`. |
| 2 | `test(signal-handler): add ABI and signal-frame layout checks` | Add compile-time/runtime layout checks for signal numbers, `sigset_t`, `sigaction`, `siginfo_t`, context layouts, and frame offsets. |
| 3 | `feat(signal): connect signal syscalls to kernel dispatch` | Wire `sigaction`, signal mask, `kill`/`raise`, pending query, and `sigreturn` to kernel dispatch using safe user-copy APIs. |
| 4 | `feat(signal): add per-thread masks and pending signal queues` | Separate process dispositions and process pending signals from thread masks and thread-directed pending queues. |
| 5 | `feat(signal-handler): deliver default actions at safe return points` | Deliver default ignore/terminate/stop/continue actions at syscall, exception, or scheduler return-to-user points. |
| 6 | `feat(signal-handler): construct validated x86_64 user signal frame` | Build frame with interrupted registers, `rip`, `rsp`, prior mask, signal number, and return trampoline on user stack. |
| 7 | `feat(signal-handler): implement x86_64 sigreturn validation` | Implement `sigreturn` validation: check frame address, canonical user pointers, flags, stack pointer, and privilege state. |
| 8 | `feat(signal-handler): implement action masks and restart semantics` | Apply `sa_mask`, automatic self-blocking, `SA_NODEFER`, `SA_RESETHAND`, and `SA_RESTART` syscall rewinding. |
| 9 | `feat(signal-handler): add siginfo and SA_SIGINFO subset` | Add typed signal information (`siginfo_t`) for kernel events (faults, child status, timer expiration). |
| 10 | `feat(signal-handler): add alternate stack support` | Implement `sigaltstack` and `SA_ONSTACK` for handling stack overflow conditions safely. |
| 11 | `feat(signal): integrate SIGCHLD and wait/reap lifecycle` | Generate child status signals (`SIGCHLD`) on process exit/stop/continue and integrate with `waitpid`. |
| 12 | `feat(signal): add synchronous signal-wait APIs` | Implement `sigwait`, `sigwaitinfo`, and `sigtimedwait` for synchronous event-loop signal acceptance. |
| 13 | `feat(signal): add queued real-time signal support with limits` | Add real-time signals (SIGRTMIN..SIGRTMAX) with FIFO payload queues and bounded queue resource caps. |
| 14 | `feat(signal): integrate signalfd as Linux-specific extension` | Connect `signalfd` module to real pending state, signal masks, poll/read, FD lifecycle, and permissions. |
| 15 | `feat(libc): expose signal APIs and safe handler examples` | Provide C/Rust userland wrappers in `signal.h` and documented async-signal-safe handler guidelines. |
| 16 | `test(signal-handler): add fault-injection and concurrency suite` | Test nested signals, syscall delivery, process exit races, frame corruption, and signal floods in QEMU. |
| 17 | `feat(signal-handler): validate AArch64 and RISC-V handler frames` | Add architecture-specific frame, trampoline, and `sigreturn` paths for AArch64 and RISC-V targets. |
| 18 | `docs(signal-handler): publish API, safety, and support matrix` | Finalize API reference, async-signal-safety guidance, architecture support matrix, and feature status entries. |

## 4. Periodic Maintenance Cadences

- **Every PR:** Run ABI layout checks, frame validation tests, and unit tests via `rustc --test src/signal/mod.rs`.
- **Weekly:** Triage signal delivery regressions and reconcile feature inventory status in `FEATURE_STATUS.toml`.
- **Monthly:** Security audit of `sigreturn` frame validation, canonical user pointer checks, and privilege isolation.
- **Quarterly:** Review POSIX.1-2024 compliance and document any Linux/FreeBSD behavioral differences.
- **Per Release Candidate:** Certify end-to-end Ring 3 handler execution round-trips in QEMU.

## 5. Completion Criteria

- **Prototype:** Signal structures, frame layouts, or dispatch stubs exist in code and unit tests, but no Ring 3 handler has executed.
- **Integrated:** On a target QEMU build, a real Ring 3 user process receives a signal, enters its handler via a kernel-constructed frame, returns via `sigreturn`, and safely resumes execution.
- **Supported:** Comprehensive tests pass across process/thread targeting, masks, nested signals, `SA_RESTART`, `siginfo_t`, `sigaltstack`, and all claimed architectures without memory leaks or privilege escapes.
