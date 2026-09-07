# SigmaOS AI Agent C-Signal Operation Management Guidelines

## 1. Overview
SigmaOS implements full POSIX and BSD C-signal handling frameworks managed autonomously or interactively by AI system agents (such as `CSignalGovernor`, `SignalFrameManager`, `AsyncSignalSafetyAuditor`, and `RealtimeSignalDispatcher`). These guidelines define POSIX/BSD C-signal primitives (`signal`, `sigaction`, `sigprocmask`, `sigpending`, `sigsuspend`, `sigwait`), signal disposition tables, alternate signal stacks (`sigaltstack`), POSIX real-time signals (`SIGRTMIN`..`SIGRTMAX`), and async-signal-safe constraints for AI agents in SigmaOS.

## 2. Core C-Signal Management Principles

### 2.1 C-Signal Disposition & `sigaction` Tables
- **Signal Dispositions**: Every agent process PCB tracks signal disposition handlers (`SIG_DFL`, `SIG_IGN`, or custom handler function pointer).
- **`sigaction` Registration**: AI agents register signal handlers with extended flags (`SA_SIGINFO`, `SA_RESTART`, `SA_NODEFER`, `SA_ONSTACK`).
- **Uncatchable Signals**: `SIGKILL` and `SIGSTOP` dispositions remain immutable and cannot be caught, blocked, or ignored.

### 2.2 Signal Masking & Atomicity (`sigprocmask` & `sigsuspend`)
- **Signal Block Masks**: AI agents block non-reentrant signals during critical thread sections using `sigprocmask` (`SIG_BLOCK`, `SIG_UNBLOCK`, `SIG_SETMASK`).
- **Atomic Wait (`sigsuspend` & `sigwait`)**: Agents atomically replace the process signal mask and suspend thread execution until a pending signal is delivered, preventing race conditions between signal checks and sleep states.

### 2.3 Alternate Signal Stacks (`sigaltstack`)
- **Stack Overflow Recovery**: AI agents handling stack overflow signals (`SIGSEGV` / `SIGBUS`) allocate dedicated alternate signal stacks (`sigaltstack`) to allow recovery and stack backtrace logging without recursive fault loops.

### 2.4 POSIX Real-Time Signals (`SIGRTMIN`..`SIGRTMAX`)
- **Queued Payload Delivery**: Real-time signals carry 32-bit/64-bit value payloads (`sigval_t`) and are queued in FIFO order rather than coalesced.
- **Priority Delivery**: Real-time signals are delivered in increasing signal number order (`SIGRTMIN` before `SIGRTMAX`).

### 2.5 Async-Signal-Safety Constraints
- **Reentrancy Rules**: C-signal handlers executed in AI agent contexts must invoke **only** async-signal-safe functions (e.g. `write`, `_exit`, `sigaction`, lock-free atomic CAS ops). Non-reentrant heap allocations (`malloc`/`free`), standard I/O streams (`printf`), or mutex acquisitions are strictly forbidden inside signal handlers.

---
*Maintained by the SigmaOS POSIX Signals & Process Steering Committee.*
