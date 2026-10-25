# SigmaOS Fairness and Starvation Prevention Policy

## Fairness Metrics & Guarantees

1. **Virtual Runtime (`vruntime`)**:
   - Accrues during task execution on CPU.
   - Used to select candidates when picking among tasks of identical priority.

2. **Starvation Bound**:
   - Any task queued in `READY` state longer than `starvation_threshold_ticks` (default 50 ticks) is automatically boosted.

3. **Priority Boosting**:
   - `boost_starved_tasks` moves starving lower-priority tasks into higher priority queues to guarantee bounded execution latency.
