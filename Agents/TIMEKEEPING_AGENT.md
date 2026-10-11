# Timekeeping & High-Resolution Clocks Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide nanosecond-precision monotonic clocks, hardware TSC synchronization, PTP/NTP network time discipline, and zero-drift tickless timers.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
NO_HZ_FULL tickless kernel, clocksource/clockevents subsystem, POSIX timerfd, vDSO time.

### FreeBSD
timecounter hardware abstraction, callout timer wheel architecture.

### OpenBSD
OpenNTPD privilege-separated secure NTP daemon with TLS constraints.

### NetBSD
timecounter architecture, TSC calibration algorithms.

### Arch Linux & Distros
Chrony precision NTP client, Arch Linux RTC configuration.

### NixOS
Declarative NTP pool peers and hardware clock timezone configurations.

## 4. SigmaOS Source Files
- `src/kernel/time.rs`
- `src/klib/time.rs`
- `src/hardware/tsc.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Invariant TSC Hardware Clocksource | ✅ Implemented | Nanosecond monotonic clock |
| Tickless Kernel Operation (NO_HZ) | ✅ Implemented | Dynamic local APIC timer |
| POSIX high-res timers (timerfd) | ✅ Implemented | Nanosecond expiration queues |
| PTP / NTP Time Discipline Engine | 🚧 Partial | Phase-locked loop drift adjustment |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** User-space zero-syscall clock reading via vDSO shared memory page.
2. **[PRIORITY: MEDIUM]** PTP (Precision Time Protocol IEEE 1588) hardware timestamping.
3. **[PRIORITY: LOW]** Leap second smooth smearing algorithm.

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
- Hierarchical Timer Wheel (O(1) insertion and cascading expiration)
- Phase-Locked Loop (PLL) clock slew mathematical algorithms
- Multi-core TSC drift detection and correction calibration

## 9. Security & Hardening Directives
- Secure NTP time validation with cryptographic TLS constraint verification
- Prevent malicious backwards time jumps from bypassing token expirations

## 10. Performance Targets vs Competitors
- `clock_gettime(CLOCK_MONOTONIC)` latency < 12ns via vDSO
- Clock drift < 1 millisecond per 30 days without network sync

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
- **Depends on:** Hardware APIC, TSC, ACPI PM-Timer
- **Required by:** Scheduler, Audio Subsystem, Network Stack, Security TLS

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Timekeeping-Component-Guide`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- IEEE 1588 Standard for Precision Clock Synchronization
- Linux Timekeeping Architecture Documentation
- OpenNTPD Protocol and Design Papers
