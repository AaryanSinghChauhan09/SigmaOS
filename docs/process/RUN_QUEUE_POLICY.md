# SigmaOS Run Queue Policy Specification

## Purpose
This document defines the structural integrity, capacity bounds, and safety invariants for the SigmaOS run-queue subsystem.

---

## 1. Capacity Limits
- **Bounded Capacity**: Each per-CPU run queue has a maximum capacity of `MAX_RUNQUEUE_CAPACITY` (1024 tasks).
- **Exhaustion Handling**: Enqueueing into a full run queue returns `Err(RunQueueError::CapacityExceeded)`. OOM or allocation failure is handled gracefully without panicking.

## 2. Queue Structure & Complexity
- **Priority Levels**: 64 priority queues (levels 0 to 63).
- **Selection Algorithm**: A 64-bit priority bitmap allows $O(1)$ selection of the non-empty queue with the highest priority level.
- **FIFO Execution**: Tasks within the same priority level execute in FIFO order.

## 3. Concurrency and Safety Guarantees
- **Race-Free Operations**: Enqueue and dequeue operations are synchronized via per-run-queue spinlocks or interrupt guards.
- **Single-Scheduling Invariant**: No task reference (`TaskRunRef`) may reside in more than one run queue simultaneously, nor be scheduled on multiple CPUs at once.
- **State Consistency**: A task in a run queue must always have state `ProcessState::Ready`.

## 4. Lost-Wakeup Prevention
- When a task transitions from `RUNNING` to `SLEEPING`, the scheduler checks `wake_event_pending`.
- If an interrupt delivered a wakeup before the task completed entering `SLEEPING`, `wake_event_pending` is reset to `false` and the task remains `ProcessState::Ready` in the run queue.
