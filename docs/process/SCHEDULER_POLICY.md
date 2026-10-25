# SigmaOS Scheduler Policy & Contract

## Architecture Specifications

1. **Run-Queue Structure**:
   - Priority-indexed run queues (e.g. 64 priority levels) or CFS Red-Black tree tracking entity `vruntime`.
   - Fast $O(1)$ or $O(\log N)$ task selection.

2. **Time-Slice Policy**:
   - Default quantum time slice (e.g., 10ms to 100ms depending on nice level and dynamic latency target).
   - Preemption triggered on timer tick interrupt if task expires quantum or higher-priority task wakes up.

3. **Priority & Nice Semantics**:
   - Priority range mapping (e.g., nice values `-20` to `+19` mapped to scheduler weights).
   - Interactive tasks receive dynamic responsiveness boosts.

4. **Preemption Points**:
   - Kernel preemption on return from interrupt / syscall.
   - Immediate preemption when a higher-priority process transitions to `READY`.

5. **Fairness Metric & Starvation Prevention**:
   - Fair distribution of CPU execution time proportional to task weights.
   - Priority boosting / age-based starvation prevention for low-priority runnable tasks.

6. **Priority Inversion Handling**:
   - Priority Inheritance Protocol (PIP) active when low-priority thread holds a mutex requested by a high-priority thread.

7. **CPU Affinity & Zero Runnable Task Behavior**:
   - Per-CPU run queues with task affinity mask support.
   - Transitions CPU to low-power `idle` loop when zero runnable tasks remain in run queues.
