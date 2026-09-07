# AI Agent Zones Operation Management Guidelines

## Purpose
These guidelines define operational protocols, implementation patterns, and safety guardrails for AI coding agents instantiating or configuring container zones, VM memory zones, or thermal zones in SigmaOS.

---

## Directives for AI Agents

1. **Container Zone Proportional CPU Allocation**:
   - Assign non-zero `cpu_shares` when calling `create_zone` to ensure deterministic fair-share CPU percentage calculation.
   - Verify `vnic_ips` before spawning networked zone services.

2. **Zone Creation Pattern**:
```rust
let mut manager = SovereignZonesManager::new();
manager.create_zone("db_zone", 100, 1024 * 1024 * 1024)?;
manager.configure_vnic("db_zone", "10.0.0.5")?;
let cpu_pct = manager.calculate_cpu_percentage("db_zone")?;
assert_eq!(cpu_pct, 100.0);
```

3. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm Solaris zones and thermal zone unit tests pass cleanly.

---

## Related Files
- `src/kernel/linux_bsd_innovations.rs`
- `src/kernel/memory/sigma_buddy.rs`
- `docs/AI_AGENT_ZONES_OPERATION_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_ZONES_OPERATION_MANAGEMENT.md`
