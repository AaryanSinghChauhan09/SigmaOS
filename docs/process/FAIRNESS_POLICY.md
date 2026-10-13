# SigmaOS Fairness & Starvation Prevention Policy (`FAIRNESS_POLICY.md`)

## 1. Overview
The SigmaOS scheduler enforces CPU time allocation fairness among runnable tasks while guaranteeing zero starvation for low-priority background threads.

## 2. Fairness Metrics
- **Virtual Runtime (`vruntime`)**: Monitored per task to track CPU time consumed relative to task priority.
- **Runnable-vs-Waiting Ratio**: Measures time a task spends in `READY` state waiting for dispatch versus time spent `RUNNING`.

## 3. Starvation Detection & Boosting
- **Threshold**: Default starvation threshold is 50 timer ticks.
- **Boosting Mechanism**: During `timer_tick()`, tasks that have waited $\ge 50$ ticks without execution receive a priority boost of +10 levels (capped at level 62).
- **Regression Verification**: Tested via `tests/scheduler/fairness/fairness_test.rs`.
