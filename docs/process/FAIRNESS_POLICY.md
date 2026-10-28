# Fairness and Starvation Prevention Policy

## Overview
SigmaOS scheduler guarantees CPU time distribution equity among runnable processes while strictly preventing starvation for low-priority tasks.

## Metrics & Rules
1. **Fairness Metric**:
   - Runnable-vs-waiting time ratio and normalized `vruntime` tracking.
   - Tasks with lower `vruntime` are given scheduling preference within the same priority band.

2. **Starvation Metric & Threshold**:
   - `starvation_threshold_ticks`: Maximum ticks a `READY` task can wait without being scheduled (Default: 50 ticks = 500ms at 100Hz).

3. **Starvation Prevention Strategy**:
   - Periodic timer tick inspects wait durations in run queues.
   - Tasks exceeding `starvation_threshold_ticks` receive priority boosting to priority 0 (highest level) until scheduled for one quantum.

4. **Fairness & Regression Testing**:
   - Continuous benchmark suites verify runtime balance across cpu-bound and i/o-bound workload mixes under heavy load.
