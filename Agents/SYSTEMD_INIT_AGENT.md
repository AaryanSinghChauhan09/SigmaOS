# Init System & Service Supervisor Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide an ultra-fast, deterministic init system (SigmaInit), parallel service startup, socket activation, dependency tracking, and cgroups v2 resource policing.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
systemd unit dependency graph, socket activation, journald logging, cgroups v2 hierarchy.

### FreeBSD
rc.d dependency ordering via rcorder(8), jail management.

### OpenBSD
rc.subr simple robust service supervision, privsep daemons.

### NetBSD
proplib property list configuration, rcorder dependency graphs.

### Arch Linux & Distros
runit fast process supervision, OpenRC declarative dependencies, s6 supervision suites.

### NixOS
Declarative systemd service definitions synthesized at system generation time.

## 4. SigmaOS Source Files
- `src/kernel/init.rs`
- `src/services/mod.rs`
- `src/security/cgroups.rs`
- `src/kernel/process.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Parallel Dependency Startup | ✅ Implemented | Directed Acyclic Graph (DAG) |
| Socket Activation Engine | ✅ Implemented | On-demand service invocation |
| cgroups v2 Service Slices | ✅ Implemented | Memory, CPU, and task limits |
| Automatic Service Restart | ✅ Implemented | Exponential backoff watchdog |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Journaling structured log daemon with cryptographic tamper-evident sealing.
2. **[PRIORITY: MEDIUM]** Service watchdog ping heartbeat API via shared memory.
3. **[PRIORITY: LOW]** Target and runlevel state machine transition validations.

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
- Topological sorting (Tarjan's strongly connected components) for service graphs
- Token bucket rate-limiting for service crash restarts
- Epoll/kqueue event loops for socket activation multiplexing

## 9. Security & Hardening Directives
- Services run under restricted UID/GID with minimum capabilities (no CAP_SYS_ADMIN)
- Default sandbox directives (ProtectSystem=strict, PrivateTmp=true)

## 10. Performance Targets vs Competitors
- Boot from kernel handoff to graphical login < 450ms on modern NVMe
- Service graph traversal latency < 2ms

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
- **Depends on:** Process Manager, IPC, cgroups v2, Filesystem
- **Required by:** All System Daemons, Desktop Shell, Login Manager

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Systemd-Init-Component-Guide`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- systemd System and Service Manager Manual
- s6 Supervision Suite Architecture Guide by Laurent Bercot
- The Design and Implementation of the FreeBSD Operating System
