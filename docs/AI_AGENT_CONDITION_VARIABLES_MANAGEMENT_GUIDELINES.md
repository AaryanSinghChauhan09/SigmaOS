# SigmaOS AI Agent Condition Variables Management Guidelines

## 1. Executive Summary & Overview

Condition variables (`Condvar` / `SimpleCondVar`) in SigmaOS enable threads to efficiently block and wait for arbitrary state changes without spinning or burning CPU time. Managed in conjunction with mutexes (`SimpleMutex`) and futex wait queues (`FutexWaitQueue`), condition variables provide non-blocking signaling, thread parking, and event notification across kernel tasks and userland processes.

This document establishes the official guidelines and architectural specifications for AI agents managing condition variables, wait queues, and thread notification patterns in SigmaOS.

---

## 2. Core Architectural Components for Condition Variables

AI agents using condition variables interface with the following core modules:

| Component / Primitive | Module Location | Description & Operational Contract |
| :--- | :--- | :--- |
| `SimpleCondVar` / `Condvar` | Kernel & Userland IPC | Condition variable synchronization primitive associated with a mutex |
| `SimpleMutex` | `src/kernel/` | Mutex guarding the boolean predicate associated with the condition variable |
| `FutexWaitQueue` | `src/kernel/linux_bsd_innovations.rs` | Futex-backed wait queue managing thread parking and waking |
| `ProcessWaitQueue` | `src/kernel/ipc.rs` | Process and thread PCB wait state transitions (`TASK_INTERRUPTIBLE`) |

---

## 3. Mandatory Design Rules & Implementation Patterns

### 3.1 Spurious Wakeup Guard Loop

AI agents must **never** assume that a thread waking from `condvar.wait()` implies the condition predicate is satisfied. POSIX and SigmaOS kernel scheduling permit spurious wakeups.

**Correct Condition Variable Wait Pattern**:
```rust
// Mutex guards the shared predicate state
let mut guard = mutex.lock();

// ALWAYS evaluate the predicate in a while loop
while !shared_state.is_ready() {
    // Atomically releases mutex guard and blocks current thread on condvar
    condvar.wait(&mut guard);
}

// Mutex guard is re-acquired; state predicate is guaranteed true
shared_state.consume_data();
```

---

### 3.2 Signal (`notify_one`) vs. Broadcast (`notify_all`)

AI agents notifying condition variables must select the appropriate notification scope:

1. **`notify_one()` / `signal()`**:
   - Wakes exactly **one** thread from the condition variable wait queue.
   - Used when tasks are homogeneous and any single waking thread can consume the work item (e.g. producer-consumer thread pools, job queues).
   - Minimizes thundering herd scheduling overhead.

2. **`notify_all()` / `broadcast()`**:
   - Wakes **all** threads blocked on the condition variable wait queue.
   - Mandatory when state transitions affect all waiters (e.g. barrier synchronization, global system shutdown, event completion, multi-reader cache invalidation).

---

### 3.3 Lost Wakeup Prevention & Mutex Association

To prevent "lost wakeup" race conditions:

1. **Predicate Lock Association**: The condition predicate **must** be mutated while holding the mutex associated with the condition variable.
2. **Notification Under Lock vs. After Unlock**:
   - Modifying predicate state and signaling while holding or after releasing the lock is permitted, but state mutation itself must occur under the lock prior to signaling.
   - Signaled threads attempting to re-acquire the lock will block on the mutex if it is still held (`FUTEX_REQUEUE` optimization re-queues waiters directly to the mutex lock queue).

---

## 4. Timed Wait & Timeout Handling

When blocking on event queues or network buffers, AI agents must use timed waits (`wait_timeout`) to prevent permanent thread deadlock:

- If `wait_timeout` returns due to deadline expiration, the agent must check if the condition predicate was satisfied before taking fallback action.
- Timed waits use monotonic timer clocks (`CMOS RTC` / `HPET` / `APIC Timer`) immune to system time jumps.

---

## 5. Verification & Concurrency Testing Protocol

AI agents modifying condition variables or wait queue implementations must verify changes:

1. **Unit & Integration Suite**: Run `./run_sigma_tests.sh` to confirm thread wait queue correctness.
2. **Fuzzing & Stress Tests**: Execute `tests/stress_and_fuzz_tests.rs` to validate zero-deadlock execution under multi-threaded contention.

---

*Approved by the SigmaOS Core Kernel Concurrency & Scheduling Committee.*
