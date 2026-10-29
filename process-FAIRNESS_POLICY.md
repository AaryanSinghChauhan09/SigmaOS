# SigmaOS Fairness and Starvation Prevention Policy Specification

## Fairness Objectives
SigmaOS guarantees that all runnable processes receive proportional access to CPU execution time relative to their assigned priority and nice values, preventing starvation under high load or nested priority hierarchies.

## Policy Rules

### 1. Fairness Metric
- Fairness is measured as the ratio of actual CPU time executed (`total_scheduled_ticks`) to expected CPU share calculated via virtual runtime (`vruntime`).
- Processes with lower `vruntime` are prioritized during dequeue operations.

### 2. Starvation Detection & Boosting
- Starvation Threshold: Defined by `DEFAULT_STARVATION_THRESHOLD_TICKS` (50 ticks / 500ms).
- On every timer tick (`timer_tick`), the scheduler inspects run-queue tasks. Any task waiting in `READY` state longer than the starvation threshold has its effective priority boosted.
- Boosted tasks return to their baseline priority once scheduled for a full quantum.

### 3. Priority Inversion Mitigation
- Priority Inheritance Protocol (PIP) boosts lock holders to the highest priority among waiting threads, ensuring high-priority threads are not blocked behind lower-priority threads.
