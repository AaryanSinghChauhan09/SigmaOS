# SigmaOS CPU Affinity Policy Specification

## CPU Affinity Overview
CPU affinity binds processes or threads to specific CPU cores, optimizing cache locality, reducing cross-socket NUMA interconnect traffic, and providing core isolation for real-time tasks.

## Policy Rules

### 1. CPU Set Representation
- CPU affinity is represented as a 64-bit bitmask (`cpu_affinity`).
- Bit $k$ set to 1 indicates execution is permitted on CPU core $k$.

### 2. Migration & Placement Semantics
- When a process transitions to `READY`, `select_best_cpu` evaluates legal cores matching `cpu_affinity`.
- The core with the minimum current run-queue depth (`len()`) is selected to balance load across cores while respecting affinity masks.

### 3. Capability & Permission Requirements
- Setting cross-process CPU affinity via `sched_setaffinity` requires appropriate process ownership or `CAP_SYS_NICE` capability tokens.

### 4. CPU Hotplug & Affinity Exhaustion
- If all CPUs in a process's affinity mask become offline or unavailable, the affinity mask automatically falls back to the default online CPU mask (e.g. `1u64 << default_cpu`).
