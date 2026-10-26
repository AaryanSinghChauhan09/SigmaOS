# SigmaOS Fairness & Starvation Prevention Policy

## Purpose
This document specifies virtual runtime metrics, starvation bounds, and priority boosting strategies for SigmaOS.

---

## 1. Virtual Runtime Metric (`vruntime`)
- Each process control block tracks execution time in `vruntime`.
- Tasks with smaller `vruntime` values are prioritized when selecting among equal-priority candidates.

## 2. Starvation Prevention Strategy
- **Starvation Metric**: Measured by the elapsed timer ticks since a task in `ProcessState::Ready` last executed.
- **Starvation Threshold**: Set to `DEFAULT_STARVATION_THRESHOLD_TICKS = 50` ticks.
- **Boosting Policy**: When `timer_tick()` identifies a task waiting $\ge 50$ ticks, its effective priority is temporarily boosted to level 15, ensuring it is scheduled promptly.

## 3. Regression Testing Requirements
- Every fairness and starvation change must be verified via `tests/scheduler/fairness/fairness_test.rs`.
- Starvation test must verify that low-priority tasks receive CPU execution under heavy load.
