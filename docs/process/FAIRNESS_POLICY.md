# SigmaOS Fairness and Starvation Policy Specification

## Fairness and Starvation Metrics
1. **Fairness Metric**:
   - Runnable-vs-waiting ratio: tasks with equal priority share CPU time proportionally.
   - Virtual runtime (`vruntime`) scales inversly with task weight (priority nice value).

2. **Starvation Threshold**:
   - `starvation_threshold_ticks` = 50 ticks (500ms).

3. **Starvation Prevention & Priority Boost**:
   - Periodic scan in `timer_tick()` checks `current_tick - enqueue_tick`.
   - Any task exceeding threshold has its effective priority boosted to 0 (highest level) until scheduled.

4. **Verification**:
   - Tested via `test_starvation_boosting` regression test suite.
