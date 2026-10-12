# SigmaOS CPU Affinity Policy Specification

## CPU Affinity Semantics
1. **CPU Bitmask Representation**:
   - 64-bit mask (`cpu_affinity`) mapping CPU cores 0..63.
   - Value `0` is automatically normalized to `1` (CPU 0).

2. **Core Selection Algorithm**:
   - Scheduler evaluates run-queue loads across all CPUs set in process affinity mask.
   - Assigns task to CPU core with minimal run-queue length (`select_best_cpu`).

3. **Affinity Migration**:
   - Updates to `cpu_affinity` trigger re-evaluation of current CPU assignment.
   - Task enqueued on non-matching CPU is migrated to permitted CPU core before state transitions to `RUNNING`.

4. **Hotplug and Exhaustion Handling**:
   - If CPU affinity mask specifies no online CPUs, process creation or `mark_ready` fails with `SchedulerError::InvalidCpuAffinity`.
