# AI Agent Consolidation Ratio Management Guidelines

## Purpose
These guidelines define operational protocols, mathematical formulas, and safety guardrails for AI coding agents tuning workload consolidation ratios in SigmaOS.

---

## Directives for AI Agents

1. **Overcommit Safety Boundaries**:
   - Maintain host memory overcommit ratio $C_{ratio} \le 3.0$ under normal operating conditions to avoid out-of-memory (OOM) thread thrashing.
   - Adjust balloon targets dynamically when host free RAM falls below 10%.

2. **Balloon Adjustment Pattern**:
```rust
// Inflate balloon to reclaim memory from VM
if host_free_ram_mb < minimum_threshold_mb {
    vm_manager.set_memory_balloon("vm-01", target_reduced_mb)?;
}
```

3. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm virtualization and ballooning unit tests pass cleanly.

---

## Related Files
- `src/virtualization/vm_manager.rs`
- `docs/AI_AGENT_CONSOLIDATION_RATIO_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_CONSOLIDATION_RATIO_MANAGEMENT.md`
