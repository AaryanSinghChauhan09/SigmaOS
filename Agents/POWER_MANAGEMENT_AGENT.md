# Power Management & Thermal Governance Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Maximize hardware battery longevity, optimize dynamic frequency scaling (DVFS), and enforce thermal throttling across heterogeneous cores.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
schedutil CPUfreq governor, Intel RAPL power limiting, ACPI thermal zones.

### FreeBSD
powerd dynamic CPU throttling daemon, ACPI sleep state state-machine.

### OpenBSD
apm(4) power management, sensor framework for thermal telemetry.

### NetBSD
envsys hardware monitor framework, x86 est (Enhanced SpeedStep) scaling.

### Arch Linux & Distros
TLP and auto-cpufreq battery heuristics, Omarchy low-power idle profiles.

### NixOS
Declarative power-profiles-daemon, hibernation thresholds and thermald policies.

## 4. SigmaOS Source Files
- `src/hardware/acpi.rs`
- `src/driver/thermal.rs`
- `src/kernel/power.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| ACPI DSDT/FADT Parsing | ✅ Implemented | Power button, S0-S5 states |
| Dynamic CPU Frequency Scaling | ✅ Implemented | EEVDF-coupled utilization tracking |
| Thermal Zone Throttling | 🚧 Partial | PID loop fan speed & P-state drops |
| Battery Health & Charge Thresholds | 🔲 Missing | Hardware EC battery write thresholds |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Modern Standby (S0ix / connected standby) transition state machine.
2. **[PRIORITY: MEDIUM]** Intel RAPL / AMD Energy counter hardware polling.
3. **[PRIORITY: LOW]** Embedded Controller (EC) direct battery preservation threshold writing.

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
- PID closed-loop controllers for thermal regulation
- Exponential Moving Average (EMA) load estimators for DVFS transition
- State machine for S0ix low-power residency entry/exit

## 9. Security & Hardening Directives
- Protect ACPI AML evaluation against stack overflow or malicious table injection
- Enforce strict privilege checking on voltage and frequency manipulation

## 10. Performance Targets vs Competitors
- Under 1.2W idle system draw on modern x86/ARM ultra-books
- Sleep wake latency < 250ms

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
- **Depends on:** ACPI Subsystem, CPU Scheduler, Interrupt Controller
- **Required by:** System Monitoring, UI Battery Applets, Thermal Daemon

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Thermal-Management`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Advanced Configuration and Power Interface (ACPI) Specification 6.5
- Intel 64 and IA-32 Architectures Software Developer's Manual Vol 3B
- Energy Efficient High Performance Computing Specifications
