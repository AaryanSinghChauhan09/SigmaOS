# AGENTS_BLOCKED_PROCESS_MANAGEMENT.md — AI Agent Blocked Process Management Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, architectural models, deadlock prevention guardrails, and verification protocols for managing, developing, and extending **Blocked Process Management, Wait Queues, Deadlock Resolution, and Interrupted System Calls** in **SigmaOS**.

---

## 1. SigmaOS Blocked Process Subsystem Overview

In SigmaOS, processes transition to blocked or waiting states when waiting for I/O completion, page faults, IPC pipe data, lock acquisition, or explicit POSIX `waitpid` / `eventfd` signals.

### Core Blocked Process & Synchronization Modules
* **PCB State & Scheduler Wait Queues (`src/process/`, `src/scheduler/`)**:
  - `SovereignProcessState::Blocked`: Process state indicating suspension pending event notifications.
  - Wait queue waking via `EventFd`, `ZeroCopyIpcChannel`, and `JobControlLifecycleEngine`.
* **Process Termination & Cancellation Manager (`src/process/`)**:
  - `ProcessCancellationAndTerminationManager`: Safely unwinding blocked processes, releasing held mutexes/locks, closing open file descriptors, and reaping zombie children.
  - Signal-based waking (`SIGKILL`, `SIGTERM`, `SIGINT`) returning `EINTR` to interrupted syscalls.
* **IPC Pipe & Socket Blocking (`src/kernel/ipc.rs`, `src/ipc/std_streams.rs`)**:
  - `SovereignPipe`: Reader/writer wait queues blocking on empty pipes or full `PIPE_BUF` buffers, handling broken pipe (`EPIPE` / `SIGPIPE`) states.
  - Deadlock detection and channel congestion monitoring (`AdvancedIpcHub`).

---

## 2. Blocked Process Management Guidelines for AI Agents

When modifying or extending blocked process, wait queue, or lock management logic:

### 1. Atomic State Transitions & Wait Queue Waking
* **State Atomicity**: Always update `Pcb.state` atomically before adding a process to a wait queue to prevent lost-wakeup race conditions.
* **Non-Deadlocking Wakeups**: Ensure wait queue notification routines (e.g. `eventfd_write()` or `pipe_write()`) do not hold inner spinlocks while waking up blocked tasks.

### 2. Syscall Interruption & `EINTR` Safety
* **Interrupted Syscalls**: Blocked system calls (e.g., `read()`, `write()`, `waitpid()`, `select()`) must detect signal arrival and return `Err("EINTR")` allowing userland libc or runtime retry handlers to safely handle the signal.
* **Orphan & Zombie Cleanup**: Ensure parent processes waiting on blocked children clean up resource allocations upon process termination.

### 3. Deadlock Detection & Cancellation Unwinding
* **Resource Unwinding**: When terminating a blocked process via `ProcessCancellationAndTerminationManager`, guarantee that all held spinlocks, file locks (`flock`), and IPC semaphores are forcefully released to prevent system-wide deadlock.

---

## 3. Verification & Testing Protocols

1. **REPL Process CLI Commands**: Inspect and manage blocked processes via interactive Shell REPL commands:
   - `ps`: View process status (`R` = Running, `B` = Blocked, `Z` = Zombie).
   - `top`: Monitor CPU usage and waiting process counts.
   - `kill` / `killall`: Send waking or termination signals (`SIGKILL`, `SIGTERM`, `SIGCONT`) to blocked tasks.
2. **Core Test Runner Execution**:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for Blocked Process Changes

Before submitting blocked process or wait queue changes:
- [ ] Confirmed atomic PCB state transitions prior to wait queue insertion.
- [ ] Verified `EINTR` signal interruption returns for blocked syscalls.
- [ ] Verified lock/semaphore unwinding on blocked process cancellation.
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded blocked process learnings using `initiate_memory_recording`.
