# SigmaOS CPU Affinity Policy

## CPU Affinity Governance

1. **CPU Set Representation**:
   CPU affinity is represented as a 64-bit bitmask (`u64`), where bit $N$ corresponds to CPU core $N$.

2. **Affinity Set/Update Semantics**:
   Tasks inherit their parent's CPU mask upon `fork`/`clone`. System calls (`sched_setaffinity`) can update the affinity mask subject to capability checks (`CAP_SYS_NICE`).

3. **Migration Semantics & Cost**:
   Tasks are migrated across core run queues only if current CPU core is not in the process affinity mask or if multi-core load imbalance exceeds 25%. Migration updates NUMA accounting metrics.

4. **Affinity Exhaustion**:
   Setting an empty or invalid CPU affinity mask returns `SchedulerError::InvalidCpuAffinity`. If all allowed CPUs are offline, task remains paused until valid CPU topology is available.
