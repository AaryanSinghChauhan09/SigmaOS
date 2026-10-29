# SigmaOS Preemption Policy Specification

## Preemption Overview
Preemption allows the kernel to suspend a running process in favor of a higher-priority or more deserving process, ensuring system responsiveness and enforcing time-sharing invariants.

## Policy Rules

### 1. Time-Slice Expiry Preemption
- Each tick of the system timer decrements `remaining_quantum_ticks` for the currently running process on each CPU.
- When `remaining_quantum_ticks == 0`, the process's state transitions from `RUNNING` to `READY`, it is re-enqueued into its CPU's run queue, and `schedule_next` is invoked.

### 2. Interrupt-Driven Preemption
- When an I/O device or timer interrupt completes, the kernel checks if a higher-priority task became runnable.
- If preemption is required and `preempt_count == 0`, preemption occurs before returning to user mode.

### 3. Voluntary Yielding
- Processes may voluntarily yield CPU control via `sys_yield` or `sched_yield()`.
- Yielding resets quantum ticks and places the process at the end of its priority queue.

### 4. Preemption Restrictions & Interrupt Context Safety
- Preemption is strictly prohibited while holding spinlocks or executing inside raw interrupt handlers (`preempt_count > 0`).
- Allocation or blocking sleeps inside interrupt context or preemption-disabled sections are treated as fatal kernel bugs.
