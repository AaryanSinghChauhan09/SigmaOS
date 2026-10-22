# ADR-004: EEVDF Preemptive Scheduler and Starvation Protection

## Component
Kernel core / Scheduler

## Current gap
Task scheduler lacks starvation detection and preemption fairness tracking.

## Upstream study
Linux EEVDF (Earliest Eligible Virtual Deadline First) scheduler and xv6 process table.

## Improvement
Implement bounded priority run-queue with vruntime tracking and starvation boosting.

## Scope
- Run-queue scheduling
- Starvation detection
- Preemption time slices
