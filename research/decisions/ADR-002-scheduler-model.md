# ADR-002: EEVDF Starvation-Free Scheduler Model

## Status
Accepted

## Context
SigmaOS needs a scalable CPU scheduler providing starvation-free latency bounds and fair CPU time allocation.

## Upstream Study
Linux EEVDF (Earliest Eligible Virtual Deadline First) replaces CFS by calculating virtual deadlines per task.

## Decision
Implement a hybrid EEVDF scheduler wrapped in safe Rust concurrency primitives with 10ms slice preemption.

## Consequences
- Guarantees upper bound on scheduling latency per task.
- Zero task starvation under heavy multi-threaded workloads.
- Priority nice levels dynamically adjust virtual deadline calculations.

## Validation
- Priority latency integration test across 100 concurrent threads.
