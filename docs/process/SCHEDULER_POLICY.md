# SigmaOS Scheduler Policy & Fairness Contract

## Run Queue & Priority Policy

SigmaOS scheduling is governed by `BoundedRunQueue` and priority-aware time-slicing:

1. **Priority Multilevel Structure**:
   - Supports 64 priority levels (0 to 63, where 0 is highest priority).
   - Priority selection uses 64-bit bitmap index scan for $O(1)$ highest-priority task lookup.
2. **Time-Slice Allocation**:
   - Standard time slice per task: 10ms to 50ms depending on nice level (-20 to +19).
   - Higher priority tasks receive larger time slices and preemption precedence.
3. **Preemption Points**:
   - Timer interrupts.
   - Task state change from `SLEEPING` to `READY` when task has higher priority than currently running task.
   - Yield syscall (`sched_yield`).
4. **Starvation Prevention & Fairness**:
   - Bounded run queue tracks task wait duration.
   - Starvation boosting promotes tasks waiting longer than threshold duration (e.g. 500ms) to higher priority bands.
5. **CPU Affinity & Multiprocessing**:
   - Each run queue is bound to a specific CPU core (`cpu_id`).
   - Task migration balances load across available core run queues.
6. **Zero Runnable Task Handling**:
   - Idle task (`PID 0`) executes `hlt` / low-power wait instruction when run queue is empty.
