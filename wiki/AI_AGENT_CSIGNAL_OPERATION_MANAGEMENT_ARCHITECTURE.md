# AI Agent C-Signal Operation Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                       AI Process PCB Signal Controller                          |
|         (CSignalGovernor, SignalFrameManager, RealtimeSignalDispatcher)         |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                   Kernel Signal Vector Table & Mask Filter                      |
|      (sigaction Disposition Map, sigprocmask Block Set, sigpending Set)          |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
| Standard C-Signals    |   | Real-Time FIFO Queue  |   | Alternate Signal Stack|
| (SIGINT, SIGPIPE, etc)|   | (SIGRTMIN..SIGRTMAX)  |   | (sigaltstack for SEGV)|
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                       Userland Signal Frame Execution                           |
|         (User Stack / Alt Stack Frame, siginfo_t, sigreturn Trampoline)          |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **Kernel Signal Dispatcher & Disposition Map**:
   - Maintains PCB signal disposition tables mapping signal numbers (1..64) to `sigaction` structs (`sa_handler`, `sa_mask`, `sa_flags`).
   - Evaluates signal block masks (`sigprocmask`) before pushing signal frames to userland.

2. **Real-Time Signal Queue & Alternate Stack Engine**:
   - Real-time signals (`SIGRTMIN`..`SIGRTMAX`) are enqueued with `siginfo_t` payload values (`sigval_t`) and dispatched in strict priority order.
   - `sigaltstack` allocates isolated stack regions for handling fatal stack overflow faults (`SIGSEGV`).

3. **Async-Signal-Safety Verifier**:
   - `AsyncSignalSafetyAuditor` inspects signal handler assembly/bytecode to enforce reentrancy safety rules (disallowing heap allocation, non-reentrant locks, or un-buffered stdio).

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
