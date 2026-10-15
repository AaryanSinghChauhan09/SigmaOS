# Process & Thread Scheduler Component Guide

## Component Overview
The Scheduler manages multi-core thread scheduling, real-time priorities, preemptive multi-tasking, energy-aware placement, and CPU affinity mask enforcement.

## Linux Inspiration
Linux CFS (Completely Fair Scheduler), EEVDF (Earliest Eligible Virtual Deadline First), SCHED_DEADLINE, SCHED_FIFO, and topology-aware core placement.

## BSD Inspiration
FreeBSD ULE scheduler (interactive heuristics and NUMA affinity), OpenBSD multi-queue scheduler, DragonFly BSD per-core LWKT queues.

## Current SigmaOS Status
Priority-based round-robin and basic fair queuing implemented. Real-time deadline guarantees and EEVDF lag compensation require full verification.

## Critical Missing Features
Full EEVDF scheduling algorithm with positive/negative lag compensation, core-affinity topology caches, NUMA node-aware work stealing, and energy-aware DVFS frequency hooks.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/kernel/scheduler.rs, src/sched/eevdf.rs, src/sched/topology.rs, src/sched/rt.rs

## Testing Strategy
Synthetic thread starvation benchmarks, hackbench latency regressions, and cyclictest real-time jitter verification in QEMU.

## Dependencies
KERNEL_CORE_AGENT, TIMEKEEPING_AGENT, INTERRUPT_APIC_AGENT

## Success Criteria
Sub-2-microsecond scheduling jitter, zero thread starvation under 10,000 active threads, deterministic sub-millisecond real-time deadline adherence.

## Open Source Competitors Analysis
Surpasses Linux EEVDF by employing zero-allocation intrusive B-trees and deterministic lockless per-CPU work-stealing rings.

## Future Enhancements
Neural-guided predictive time-slice sizing based on historical process CPU bursts and speculative execution branch footprints.
