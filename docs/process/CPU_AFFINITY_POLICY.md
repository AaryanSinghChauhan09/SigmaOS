# SigmaOS CPU Affinity Policy

## Bitmask Representation and Migration Semantics

1. **CPU Set Mask**: Represented as a 64-bit integer (`u64`) mask where bit `C` enables CPU core `C`.
2. **Default Mask**: 1 (`0x1`, CPU core 0) if 0 is passed.
3. **Core Selection**: `select_best_cpu` evaluates all CPUs permitted by `cpu_affinity` mask and selects the core with the minimal run queue load.
4. **Migration Semantics**: When a task's affinity mask is updated, if it is currently in `READY` state, it may be migrated to an eligible core with lower run queue depth.
5. **Affinity Exhaustion**: If no enabled CPU in the affinity mask is valid (`mask & ((1 << num_cpus) - 1) == 0`), enqueue fails with `SchedulerError::InvalidCpuAffinity`.
