# Control Groups (cgroups v2) & Resource Governor Component Guide

## Component Overview
Governs unified hierarchical resource control (CPU bandwidth, memory limits, block IO weight, process limits) inspired by Linux cgroups v2 and FreeBSD rctl.

## Linux Inspiration
Linux cgroups v2 unified hierarchy, cpu.weight/cpu.max controllers, memory.high/memory.max limits, io.weight latency protection, and psi pressure stall information.

## BSD Inspiration
FreeBSD rctl (resource limits) and hierarchical jail rules, DragonFly BSD per-core CPU resource partitions.

## Current SigmaOS Status
Hierarchical cgroup tree structure and basic memory quota accounting implemented. PSI pressure stall info and CPU bandwidth throttling require completion.

## Critical Missing Features
Complete CPU CFS-bandwidth quota throttling, PSI (Pressure Stall Information) calculation for CPU/memory/IO, and OOM killer targeting leaf cgroups first.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/governance/cgroups.rs, src/governance/cpu_controller.rs, src/governance/memory_controller.rs, src/governance/psi.rs

## Testing Strategy
Cgroup memory limit enforcement tests, CPU bandwidth capping verification under high contention, fork bomb containment.

## Dependencies
SCHEDULER_AGENT, MEMORY_MANAGEMENT_AGENT, PROCESS_MANAGEMENT_AGENT

## Success Criteria
Zero runaway process escapes, deterministic multi-tenant isolation, instantaneous throttle response under resource contention.

## Open Source Competitors Analysis
Avoids the convoluted legacy cgroups v1 / v2 duality in Linux by designing a clean, unified, zero-overhead hierarchy natively in safe Rust.

## Future Enhancements
Dynamic AI-driven resource rightsizing continuously tuning cgroup limits based on system activity.
