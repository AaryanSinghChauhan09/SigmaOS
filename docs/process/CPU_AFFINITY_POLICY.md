# CPU Affinity Policy Specification

## Overview
CPU affinity governs the assignment and legal migration of processes across CPU cores in SMP (Symmetric Multiprocessing) architectures.

## Rules & Requirements
1. **CPU Set Representation**:
   - Bitmask representation (`cpu_affinity: u64`) supporting up to 64 CPU cores. Bit `1 << N` set to 1 permits execution on CPU core `N`.

2. **Set / Update Semantics**:
   - `sys_sched_setaffinity(pid, mask)` validates bitmask against online CPU mask.
   - Requires valid process capabilities or owner UID match.

3. **Migration & Load Balancing**:
   - Scheduler load balancer evaluates per-CPU run queue lengths during timer ticks.
   - Tasks are migrated only when queue length disparity exceeds threshold and target core is enabled in task's `cpu_affinity` mask.

4. **Affinity Exhaustion & Fallback**:
   - If a task's affinity mask contains zero online CPUs, operation fails with `SchedulerError::InvalidCpuAffinity`.
