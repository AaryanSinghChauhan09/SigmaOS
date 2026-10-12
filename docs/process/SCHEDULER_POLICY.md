# SigmaOS Scheduler Policy Specification

## 1. Scheduling Class
SigmaOS adopts a **hybrid EEVDF/CFS and Priority-Boosted Round-Robin** scheduling engine.
- Interactive and normal tasks are governed by virtual runtime (`vruntime`) progression.
- Real-time / high-priority tasks (priority 0..15) use deterministic Round-Robin within their priority band.
- Low-priority tasks (priority 16..63) utilize CFS virtual runtime tracking with starvation boosting.

## 2. Run-Queue Structure
- Multi-core SMP run queue architecture with per-CPU `BoundedRunQueue`.
- Bounded capacity (default `MAX_RUNQUEUE_CAPACITY = 2048` tasks per queue).
- Bitmapped 64-priority-level array for $O(1)$ priority selection.
- Sub-queues maintain FIFO ordering per priority level.

## 3. Time-Slice Policy
- Default quantum: 10 timer ticks (100ms at 100Hz clock interrupt rate).
- Real-time priority tasks receive fixed quantum.
- Interactive tasks dynamically scale time-slices based on latency targets and priority nice levels (-20 to +19).

## 4. Priority and Nice Semantics
- 64 priority levels: 0 (highest priority / real-time) to 63 (lowest priority / background).
- Standard nice values map into kernel priority range: nice -20 maps to priority 0; nice +19 maps to priority 63.
- Interactive tasks receive dynamic priority boosts based on wait time vs CPU compute time.

## 5. Preemption Points
- Timer interrupt ticks (time-slice expiration).
- Syscall return path (if higher-priority task became runnable).
- Device interrupt handler completion (unblocking sleeping higher-priority thread).
- Explicit thread yield (`sched_yield`).

## 6. Fairness Metric
- Measured as the ratio of allocated CPU time vs waiting time across tasks at equal priority levels.
- `vruntime` normalization ensures fair execution across different weight classes.

## 7. Starvation Prevention
- Periodic starvation auditing during `timer_tick()`.
- Starvation threshold: 50 ticks (500ms).
- Starved tasks exceeding threshold receive a temporary priority boost to priority 0 to execute immediately.

## 8. Priority Inversion Handling
- Priority Inheritance Protocol (PIP) and Priority Ceiling Protocol (PCP) integrated into mutex and wait-queue primitives.
- High-priority task blocked on a lock held by a low-priority task temporarily elevates the holder's priority.

## 9. CPU Affinity Behavior
- 64-bit affinity bitmask per process descriptor.
- Load balancer migrates runnable tasks to least-loaded CPU core satisfying affinity mask.
- Strict enforcement prevents scheduling a process on non-permitted CPU cores.

## 10. Behavior Under Zero Runnable Tasks
- Core transitions to idle loop (`cpu_idle`).
- Executes platform-specific low-power instruction (`hlt` on x86_64, `wfi` on ARM64).
- Maintains timer interrupt ticks for prompt wakeup on external events.
