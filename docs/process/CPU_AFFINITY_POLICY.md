# SigmaOS CPU Affinity Policy (`CPU_AFFINITY_POLICY.md`)

## 1. Overview
CPU affinity controls task-to-CPU assignment in multi-core (SMP) architectures, optimizing cache locality, interconnect traffic, and NUMA memory latency.

## 2. Core Representation & Masks
- **64-bit Bitmask**: `CpuAffinityMask` uses `u64` bitmask where bit $k = 1$ indicates CPU core $k$ is legal for process execution.
- **Default Mask**: New processes inherit full system mask (`CpuAffinityMask::all_cores()`).

## 3. Operations & NUMA Migration
- **Set Affinity**: `assign_process_affinity(pid, mask)` enforces non-empty mask constraints.
- **Optimal Core Selection**: `select_optimal_core_for_task` evaluates current run queue loads across affinity-permitted cores and selects the core with minimal queue load.
- **NUMA Migration**: `migrate_process_numa_node` updates affinity mask to target NUMA node's CPU set.
- **Thread Pinning**: `pin_thread_to_core` binds a thread to a single CPU core.
