# POSIX Signal Handling Agent

| Field | Value |
|---|---|
| **Portal** | Kernel Core |
| **Maintainer Agent** | POSIX_SIGNAL_HANDLING_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_posix_signal_engine.rs` |
| **Language** | Rust (primary), Zig (accelerator) |

---

## Table of Contents
1. [Overview](#overview)
2. [Technical Specifications](#technical-specifications)
3. [Implementation](#implementation)
4. [Standard Library Authorization](#standard-library-authorization)
5. [Configuration](#configuration)
6. [Diagnostics](#diagnostics)
7. [See Also](#see-also)

---

## 1. Overview
Manages POSIX signal delivery, `sigaction` disposition tables, real-time signal queuing (POSIX.1b), `SA_RESTART` syscall semantics, and per-thread signal masks (`pthread_sigmask`).

## 2. Technical Specifications
- Signal range: SIGMIN(1) – SIGRTMAX(64)
- RT signal queue depth: configurable via `/proc/sys/kernel/rtsig-max`
- Disposition: SIG_DFL, SIG_IGN, user handler
- Mask operations: sigprocmask, pthread_sigmask, sigpending

## 3. Implementation
Engine: [`sigma_posix_signal_engine.rs`](../src/kernel/sigma_posix_signal_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized. Use `libc::sigaction`, `libc::sigprocmask` freely.

## 5. Configuration
```toml
[signal_handling]
rt_signal_queue_depth = 8192
sa_restart_default = true
coredump_signals = ["SIGSEGV", "SIGBUS", "SIGABRT"]
```

## 6. Diagnostics
```bash
kill -l                          # list all signals
/proc/self/status | grep Sig     # SigBlk, SigPnd, SigCgt masks
sigma-diag signal --pid <PID>
```

## 7. See Also
- [CRASH_DUMP_CORE_AGENT](./CRASH_DUMP_CORE_AGENT.md)
- [KERNEL_MODULE_LOADING_AGENT](./KERNEL_MODULE_LOADING_AGENT.md)
