# SigmaOS Fairness and Starvation Prevention Policy

## Fairness and Starvation Metrics

1. **Fairness Metric**:
   Fairness is evaluated using normalized virtual runtime (`vruntime`) progression and relative wait times across processes within the same priority class. Variance in wait time among equal-priority processes must stay under 15%.

2. **Starvation Metric**:
   Starvation is defined as any runnable process remaining in `READY` state without CPU allocation for longer than `DEFAULT_STARVATION_THRESHOLD_TICKS` (50 ticks / 500ms).

3. **Starvation Prevention Strategy**:
   The scheduler performs periodic starvation audits. Tasks exceeding the starvation threshold receive dynamic priority boosting to priority level 62, forcing immediate execution in the next scheduling cycle.

4. **Starvation Regression Tests**:
   Continuous integration suites execute starvation benchmarks under heavily skewed workloads (e.g., 100 high-priority CPU burners vs. 1 low-priority worker) to verify zero starvation incidents.
