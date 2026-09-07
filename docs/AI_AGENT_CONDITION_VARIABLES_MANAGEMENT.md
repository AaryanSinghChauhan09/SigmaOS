# ⚙️ AI Agent Condition Variables Management in SigmaOS

## Executive Summary
Condition variables (`CondVar`) in SigmaOS provide deterministic thread synchronization primitives allowing threads to block execution until specific predicate conditions become true. Autonomous AI Agents (**Bolt ⚡**, **Palette 🎨**, and **Sentinel 🛡️**) managing task queues, process scheduling, IPC channels, and driver event notifications must adhere to strict condition variable handling specifications to prevent missed wakeups, lost signals, and priority inversion deadlocks.

---

## 1. Core Synchronization Architecture & Primitives

SigmaOS condition variables operate in conjunction with Mutexes or Spinlocks to protect shared state predicates:

```rust
pub struct ConditionVariable {
    wait_queue: LockFreeSpscQueue<ThreadId>,
    sequence_counter: AtomicU64,
}
```

### Key Operations
- **`wait(&self, mutex: &Mutex<T>)`**: Atomically releases the associated mutex, enqueues the calling thread onto the `CondVar` wait queue, and suspends thread execution (`TASK_BLOCKED`).
- **`signal(&self)`**: Unblocks the highest-priority thread waiting on the condition variable.
- **`broadcast(&self)`**: Unblocks all threads currently waiting on the condition variable.
- **`wait_timeout(&self, mutex: &Mutex<T>, timeout_ticks: u64)`**: Suspends thread execution with an explicit deadline to prevent indefinite blocking.

---

## 2. Spurious Wakeup Guard Pattern

AI Agents MUST always wrap condition variable waits inside a predicate check loop (`while` loop) rather than an `if` statement to handle spurious wakeups safely:

```rust
// ✅ CORRECT: Protected against spurious wakeups
let mut state = lock.lock();
while !state.predicate_satisfied() {
    condvar.wait(&mut state);
}
```

```rust
// ❌ INCORRECT: Vulnerable to spurious wakeups
let mut state = lock.lock();
if !state.predicate_satisfied() {
    condvar.wait(&mut state); // DANGER: Unchecked re-entry
}
```

---

## 3. Priority Inheritance & Inversion Prevention

When high-priority real-time threads wait on condition variables held by lower-priority threads:
1. **Priority Propagation**: The thread holding the associated mutex temporary inherits the highest priority among all threads blocked on the `CondVar`.
2. **Deterministic Queue Ordering**: The wait queue prioritizes threads according to real-time priority class (`SCHED_FIFO` / `SCHED_RR`).

---

## 4. AI Agent Operational Rules

1. **Bolt ⚡ (Performance)**: Use `signal()` over `broadcast()` when only a single worker thread can process the available item to avoid thundering herd contention on the mutex.
2. **Palette 🎨 (UX & Interactivity)**: Ensure UI rendering and desktop event loops use `wait_timeout()` with non-blocking UI state updates to prevent frame drops.
3. **Sentinel 🛡️ (Security & Integrity)**: Verify that condition variable waiters never hold locks out of order and that all wait calls enforce IRQL safety limits (`IRQL <= DISPATCH_LEVEL`).
