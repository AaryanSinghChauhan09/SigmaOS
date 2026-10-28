# Run Queue Policy Specification

## Overview
The run queue is the core data structure managing `READY` tasks eligible for CPU execution in SigmaOS.

## Specifications
1. **Bounded Capacity**:
   - Each per-CPU run queue enforces an explicit maximum capacity (`MAX_RUNQUEUE_CAPACITY = 1024`).
   - Run queue exhaustion returns `RunQueueError::QueueFull` and invokes backpressure or OOM state handling.

2. **Race-Free Enqueue and Dequeue**:
   - Operations are protected by per-CPU lockless/atomic primitives or fine-grained spinlocks.
   - 64-bit priority bitmaps enable O(1) identification of highest-priority runnable tasks.

3. **Single Scheduling Invariant**:
   - No task can reside in multiple run queues simultaneously.
   - Enqueue operations verify task state and reject tasks already present in a queue.

4. **Lost-Wakeup Prevention**:
   - When a process prepares to sleep, the scheduler inspects `wake_event_pending` and `pending_signals`.
   - If an event arrived concurrently before sleeping completes, the process is re-enqueued to `READY` immediately.
