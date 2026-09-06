# AI Agent Guidelines: Concurrent Process Management in SigmaOS

## Overview
This document defines operational guidelines and architectural directives for AI agents working on **Concurrent Process Management** in SigmaOS. It specifies Process Control Block (PCB) allocations, parent-child process hierarchies (`fork`, `exec`, `exit`, `waitpid`), multicore EEVDF/BORE thread scheduling, POSIX process groups and sessions, cgroups v2 resource quotas, signal dispatching, and zero-copy IPC synchronization across `#![no_std]` runtime environments in SigmaOS.

---

## 1. Concurrent Process Subsystems & Modules

AI agents interacting with process creation, task scheduling, or process lifecycle management in SigmaOS must interface with the following core subsystems:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Process Control & Lifecycle** | `src/kernel/process.rs`, `src/kernel/proc/process_lifecycle.rs` | Concurrent `ProcessManager`, PCB structures, PID assignment, parent/child relationships, and lifecycle state machines. |
| **Multicore Task Scheduler** | `src/kernel/scheduler.rs`, `src/kernel/roundrobin.rs` | Concurrent thread scheduling (EEVDF, BORE, Multi-Queue Round-Robin) with CPU core affinity and NUMA load balancing. |
| **Advanced Process Control** | `src/process/advanced_process_control.rs` | Process cancellation manager (`ProcessCancellationAndTerminationManager`), POSIX process groups (PGID), sessions (SID), cgroups v2 quotas, and signal handling (`SIGKILL`, `SIGTERM`). |
| **Process Task Structures** | `src/kernel/sched/task.rs` | Low-level `Task` descriptors, credentials (`Cred`), scheduling policies (`SchedPolicy`), and process states (`ProcessState`). |
| **Concurrent IPC Namespaces** | `src/ipc/ipc_namespace.rs`, `src/performance/zero_copy_ipc.rs` | Isolated IPC namespaces, semaphores (`SemaphoreObject`), shared memory mappings, and lock-free atomic ring buffers. |

---

## 2. Architectural Rules & Concurrency Invariants

AI agents must enforce the following 4 core invariants when implementing or auditing concurrent process management:

```
+-------------------------------------------------------------------------+
|             SIGMAOS CONCURRENT PROCESS ARCHITECTURE                     |
+-------------------------------------------------------------------------+
                                     |
         +---------------------------+---------------------------+
         |                           |                           |
         v                           v                           v
  [Process Lifecycle & PCB]   [Multicore Schedulers]     [Advanced Process Control]
  • New -> Ready -> Running   • EEVDF / BORE / RoundRobin • cgroups v2 Quota Limits
  • Blocked -> Zombie         • NUMA CPU Core Affinity    • Process Groups (PGID)
  • Parent/Child waitpid()    • Sub-µs Preemption Latency • Signal Dispatch (SIGKILL)
```

### 1. Atomic PCB State Machine Transitions
- **Invariant:** Process state transitions (`src/kernel/process.rs`, `src/kernel/sched/task.rs`) MUST be executed atomically using `AtomicUsize` stores or protected by inner spinlocks.
- **Rule:** Concurrent state mutations across multiple CPU cores must follow valid progression paths (`New` $\to$ `Ready` $\to$ `Running` $\to$ `BlockedWaiting` $\to$ `Zombie` $\to$ `Terminated`) without data races.

### 2. Zombie Child Reaping Guarantee
- **Invariant:** When a child process terminates, it enters `ProcessState::Zombie`. Parent processes executing `waitpid()` MUST reclaim the child's exit status code and deallocate its PCB structures.
- **Rule:** Un-reaped zombie processes must be re-parented to `INIT_PID` (PID 1) on parent termination to prevent PID descriptor leaks.

### 3. Signal Handling & Spinlock Safety
- **Invariant:** Asynchronous signal dispatch routines (`SIGKILL`, `SIGTERM`, `SIGSTOP`) MUST NOT hold inner process spinlocks while waking up or terminating target tasks.
- **Rule:** Forceful process cancellation must automatically release all held spinlocks, file locks (`flock`), and IPC semaphores to prevent system-wide deadlocks.

### 4. Zero Ring 0 Panic Rule
- Process creation, scheduling, and signal operations must return explicit `Result<T, &'static str>` or `Result<T, SchedulerError>` status codes instead of triggering unhandled kernel panics.

---

## 3. Verification & Testing Protocols

Every concurrent process management change must be verified via standalone unit tests and integrated test execution:

```bash
# Run standalone unit test for process lifecycle and scheduling
rustc --test --edition 2021 src/kernel/process.rs -o build/test_kernel_process && ./build/test_kernel_process

# Run full test suite
./run_sigma_tests.sh
```

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing changes touching process creation, thread scheduling, or process signals:

- [ ] Are PCB state transitions updated atomically without lock contention races?
- [ ] Does child termination properly transition to `Zombie` and support `waitpid()` status reclamation?
- [ ] Do signal dispatch and cancellation routines release held locks prior to task teardown?
- [ ] Have all unit tests passed with 0 failures in `./run_sigma_tests.sh`?
