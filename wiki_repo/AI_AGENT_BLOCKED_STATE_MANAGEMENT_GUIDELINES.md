# SigmaOS AI Agent Blocked State Management Guidelines

## 1. Overview
SigmaOS implements rigorous thread/process blocked state management frameworks governing autonomous AI agents (such as `BlockedStateGovernor`, `FutexWaitQueueManager`, `DeadlockDetector`, and `AntiLockupWatchdog`). These guidelines define process PCB state transitions (`TASK_INTERRUPTIBLE`, `TASK_UNINTERRUPTIBLE`, `TASK_KILLABLE`, `TASK_STOPPED`, `TASK_TRACED`), futex wait queues, I/O wait handling, deadlock detection, and anti-lockup watchdog recovery for AI agents in SigmaOS.

## 2. Core Blocked State Management Principles

### 2.1 Process State Transitions & Wait Queues
- **Interruptible Waiting (`TASK_INTERRUPTIBLE`)**: AI agents waiting on network sockets, IPC pipes, or timers enter `TASK_INTERRUPTIBLE` state, allowing signal handling (`SIGINT`, `SIGTERM`, `SIGKILL`) to wake the thread immediately.
- **Uninterruptible Waiting (`TASK_UNINTERRUPTIBLE` & `TASK_KILLABLE`)**: Critical I/O disk waits or kernel page-fault locks use `TASK_KILLABLE` state, ensuring fatal signals (`SIGKILL`) can unblock and terminate stalled tasks.

### 2.2 Fast Userspace Mutexes (Futex) & Event Demultiplexing
- **Futex Hash Buckets**: Futex operations (`FUTEX_WAIT`, `FUTEX_WAKE`, `FUTEX_REQUEUE`) map to hashed spinlock wait buckets (`FutexHashBucket`) to minimize lock contention.
- **Epoll & Kqueue Integration**: Agents handling async I/O multiplex events via Linux `epoll` or BSD `kqueue` without entering blocking kernel polling loops.

### 2.3 I/O Wait & Asynchronous Ring Operations
- **`io_uring` Async Completion**: Long-running file and storage I/O operations are submitted to `io_uring` ring buffers (`IoUringEngine`), preventing threads from blocking in kernel I/O wait queues.

### 2.4 Deadlock Detection & Anti-Lockup Watchdog
- **Lock Dependency Graph Analysis**: `DeadlockDetector` continuously analyzes lock acquisition graphs across AI agent threads to detect cyclic lock dependencies.
- **NMI Hard/Soft Lockup Watchdog**: `AntiLockupWatchdog` monitors CPU core timers. If an AI agent thread blocks a CPU core for > 20 seconds, a Non-Maskable Interrupt (NMI) triggers stack backtrace logging and thread termination.

---
*Maintained by the SigmaOS Process Scheduling & Kernel Steering Committee.*
