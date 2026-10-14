# SigmaOS Fairness and Starvation Prevention Policy Specification

## Fairness Metrics
- **Virtual Runtime (`vruntime`)**: Measures cumulative CPU time consumed by process adjusted by priority/nice weight.
- **Fair Allocation**: Tasks with lower `vruntime` are prioritized during dequeue within the same priority level.

## Starvation Prevention Strategy
- **Threshold**: `DEFAULT_STARVATION_THRESHOLD_TICKS = 50` ticks (500 ms).
- **Starvation Boosting**: If a process remains in `READY` queue without execution for $\ge 50$ ticks, the scheduler temporarily boosts its priority level and sets `starvation_boosted` flag to prevent starvation by high-frequency tasks.
