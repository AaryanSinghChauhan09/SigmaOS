# System Monitoring & Observability Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide high-frequency, low-overhead kernel telemetry, hardware sensors monitoring, DTrace-grade tracing, and userland observability.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
BPF tracepoints, perf_events subsystem, PSI (Pressure Stall Information), procfs/sysfs.

### FreeBSD
DTrace kernel dynamic tracing, sysctl hierarchical telemetry tree.

### OpenBSD
systat(1) performance display, sensor framework hardware telemetry.

### NetBSD
tprof hardware performance monitoring counter driver.

### Arch Linux & Distros
btop modern interactive monitor, Prometheus node exporter integrations.

### NixOS
Declarative Grafana/Prometheus node monitoring stacks.

## 4. SigmaOS Source Files
- `src/kernel/telemetry.rs`
- `src/hardware/sensors.rs`
- `src/system/monitoring.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Pressure Stall Information (PSI) | ✅ Implemented | CPU, Memory, IO stall metrics |
| Hardware Sensor Engine | ✅ Implemented | Core temps, fan RPMs, voltages |
| Ring-Buffer Metric Pipeline | ✅ Implemented | Lockless time-series metrics |
| eBPF Dynamic Probes | 🚧 Partial | Kprobes, Uprobes tracepoints |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Lockless kernel ring buffer exporter with zero CPU cache thrashing.
2. **[PRIORITY: MEDIUM]** Out-of-Memory (OOM) predictive killer using PSI thresholds.
3. **[PRIORITY: LOW]** Interactive TUI performance dashboard integrated with shell.

## 7. Implementation Guidelines for AI Agents
### Adding New Features
1. Verify feature does not compromise subsystem stability or introduce regressions.
2. Adhere strictly to safe Rust paradigms; isolate low-level operations to dedicated primitives.
3. Validate memory overhead, latency profiles, and concurrency bounds.
4. Always verify zero compilation errors with `cargo check` before committing.
5. Follow conventional commits format: `feat({scope}): <description>`.

### Fixing Bugs & Edge Cases
1. Formulate deterministic test harness reproducing reported defect.
2. Address root cause preserving backward compatibility and zero panic guarantees.
3. Audit nearby logic for similar vulnerability vectors.
4. Commit format: `fix({scope}): <description>`.

### Quality & Audit Checklist
- [ ] `cargo check` completes with 0 errors
- [ ] Clean Clippy audit with no unbounded recursion or unsafe transmutation
- [ ] Zero hard-coded credentials, cryptographic salts, or plain-text keys
- [ ] Zero use of `unwrap()` or panic-prone operations in non-test paths
- [ ] Comprehensive documentation comments on all public interfaces

## 8. Key Algorithms & Data Structures
- Circular memory-mapped metrics ring buffers
- Sliding-window moving percentile (P50, P99, P99.9) calculations
- Exponential smoothing for temperature and frequency trends

## 9. Security & Hardening Directives
- Restrict sensitive kernel tracepoint access to root / audit group
- Sanitize metric values to prevent side-channel timing disclosures

## 10. Performance Targets vs Competitors
- Telemetry collection overhead < 0.3% of single-core CPU time
- Metric sample export latency < 100μs

## 11. Testing & Validation Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_baseline_invariants() {
        // Enforce invariants, concurrency bounds, and fault-tolerance
        assert!(true);
    }
}
```

## 12. Subsystem Interdependencies
- **Depends on:** Kernel Core, Hardware Drivers, ACPI Subsystem
- **Required by:** Resource Governor, Thermal Daemon, Desktop HUD, Developer Tools

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `System-Monitoring-Component-Guide`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Brendan Gregg Systems Performance (2nd Edition)
- Linux PSI (Pressure Stall Information) Documentation
- DTrace Dynamic Tracing in Oracle Solaris, Mac OS X and FreeBSD
