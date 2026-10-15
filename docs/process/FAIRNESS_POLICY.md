# SigmaOS Fairness and Starvation Prevention Policy

## Metrics and Starvation Prevention

1. **Fairness Metric**: Measured as the ratio of scheduled CPU ticks vs waiting ticks in the run queue for tasks of equal priority.
2. **Starvation Threshold**: Default 50 ticks (500ms at 100Hz).
3. **Starvation Boosting**: During each `timer_tick()`, the scheduler inspects run queues. Any task that has waited in a queue for $\ge 50$ ticks receives a priority boost (+10 levels, capped at 62) and its `enqueue_tick` is updated to current tick.
4. **Starvation Regression Protection**: Automated unit and integration tests enforce that low-priority tasks cannot be starved indefinitely under high load.
