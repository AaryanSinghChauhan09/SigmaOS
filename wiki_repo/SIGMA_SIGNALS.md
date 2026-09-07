# SigmaOS Signal Subsystem

## Overview

SigmaOS implements a POSIX-compatible signal subsystem for inter-process communication and kernel→process notification. Supports all 31 standard POSIX signals with proper disposition, masking, and delivery semantics.

**Location:** `src/kernel/sigma_signal.rs`

---

## Signal Table

| Signal | # | Default Action | Description |
|--------|---|---------------|-------------|
| SIGHUP | 1 | Terminate | Hangup |
| SIGINT | 2 | Terminate | Interrupt (Ctrl+C) |
| SIGQUIT | 3 | Core dump | Quit |
| SIGKILL | 9 | Terminate | Kill (uncatchable) |
| SIGSEGV | 11 | Core dump | Segmentation fault |
| SIGTERM | 15 | Terminate | Termination request |
| SIGSTOP | 19 | Stop | Stop (uncatchable) |
| SIGCONT | 18 | Continue | Resume stopped process |
| SIGCHLD | 17 | Ignore | Child status change |
| SIGUSR1 | 10 | Terminate | User-defined 1 |
| SIGUSR2 | 12 | Terminate | User-defined 2 |
| SIGWINCH | 28 | Ignore | Terminal resize |

---

## API Reference

```rust
// Per-process signal state
let mut state = SigmaSignalState::new(pid);

// Install signal handler (sigaction)
state.sigaction(Signal::SIGUSR1, SigAction {
    disposition: SignalDisposition::Handler,
    handler_addr: 0x401000, // handler function address
    mask: SigSet::empty(),
    flags: SigActionFlags::empty(),
}).unwrap();

// Block signals (sigprocmask)
let mut mask = SigSet::empty();
mask.add(Signal::SIGTERM);
state.block(mask);

// Send signal
state.send_signal(Signal::SIGUSR1, sender_pid);

// Deliver pending signals (called on syscall return)
let actions = state.process_pending();

// System-wide signal manager
let mut mgr = SigmaSignalManager::new();
mgr.register_process(1000);
mgr.kill(1000, Signal::SIGTERM, 0).unwrap();
mgr.deliver(1000);
```

---

## Signal Dispositions

| Disposition | Effect |
|-------------|--------|
| `Terminate` | Exit process |
| `CoreDump` | Exit + write core file |
| `Ignore` | Discard signal |
| `Stop` | Pause process (SIGSTOP) |
| `Continue` | Resume process (SIGCONT) |
| `Handler` | Call user-installed function |

---

## Comparison

| Feature | Linux | FreeBSD | OpenBSD | SigmaOS |
|---------|-------|---------|---------|---------|
| All 31 POSIX signals | Yes | Yes | Yes | Yes |
| sigaction() | Yes | Yes | Yes | Yes |
| sigprocmask() | Yes | Yes | Yes | Yes |
| SA_RESTART | Yes | Yes | Yes | Yes |
| SA_RESETHAND | Yes | Yes | Yes | Yes |
| Real-time signals | 32-64 | 32-64 | No | Planned |
| pledge integration | No | No | Yes | Planned |
| no_std | No | No | No | **Yes** |
