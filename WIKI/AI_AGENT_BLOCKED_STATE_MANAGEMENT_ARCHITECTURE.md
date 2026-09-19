# AI Agent Blocked State Management Architecture in SigmaOS

## Architecture Blueprint

```
+---------------------------------------------------------------------------------+
|                         AI Agent Thread / Task Process                          |
|             (TASK_RUNNING -> TASK_INTERRUPTIBLE / TASK_KILLABLE)                 |
+---------------------------------------------------------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                       Wait Queue & Futex Hash Bucket Router                     |
|           (FutexHashBucket, Epoll/Kqueue Demux, io_uring Completion)            |
+---------------------------------------------------------------------------------+
                                        |
       +--------------------------------+--------------------------------+
       |                                |                                |
       v                                v                                v
+-----------------------+   +-----------------------+   +-----------------------+
| Futex Wait Queue      |   | Async I/O Ring Wait   |   | Deadlock Detector     |
| (FUTEX_WAIT/WAKE)     |   | (io_uring SQ/CQ)      |   | (Lock Dependency Graph|
+-----------------------+   +-----------------------+   +-----------------------+
       |                                |                                |
       +--------------------------------+--------------------------------+
                                        |
                                        v
+---------------------------------------------------------------------------------+
|                     Anti-Lockup NMI Watchdog & Timer Supervisor                 |
|             (Soft/Hard Lockup Detection, NMI Stack Backtrace, Waker)           |
+---------------------------------------------------------------------------------+
```

## Architectural Components

1. **PCB State Transition Engine**:
   - Manages task state flags (`TASK_RUNNING`, `TASK_INTERRUPTIBLE`, `TASK_UNINTERRUPTIBLE`, `TASK_KILLABLE`, `TASK_STOPPED`, `TASK_TRACED`).
   - Ensures `TASK_KILLABLE` tasks unblock immediately when receiving `SIGKILL` or `SIGTERM`.

2. **Futex & Event Multiplexing**:
   - Hashed spinlock futex buckets reduce lock contention on concurrent thread synchronization.
   - Non-blocking `epoll` and `kqueue` event loops wake blocked agent threads upon I/O readiness.

3. **Lock Dependency & Watchdog Subsystem**:
   - `DeadlockDetector` maintains directed acyclic lock graphs to identify circular wait states.
   - `AntiLockupWatchdog` uses NMI hardware timers to detect soft/hard core lockups (> 20s) and force-recover stalled CPUs.

4. **Wiki Syncing**:
   This document is mirrored in `./wiki/` and `./wiki_repo/` for GitHub Wiki access.
