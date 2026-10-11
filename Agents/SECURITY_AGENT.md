# Security Framework & Mandatory Access Control Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Enforce military-grade system hardening, OpenBSD pledge/unveil capabilities, FreeBSD Capsicum sandboxing, and Landlock v4 mandatory access controls.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
Landlock v4 LSM, seccomp-bpf syscall filtering, AppArmor profiles, IMA/EVM integrity.

### FreeBSD
Capsicum capability mode, MAC Framework (Multi-Level Security, Biba, MLS).

### OpenBSD
pledge(2) syscall restriction, unveil(2) filesystem hiding, KARL randomized link.

### NetBSD
kauth(9) kernel authorization framework, verified exec (veriexec).

### Arch Linux & Distros
Whonix anonymity compartmentalization, HardenedBSD ASLR improvements.

### NixOS
Declarative global security profiles, immutably mounted system trees.

## 4. SigmaOS Source Files
- `src/security/landlock_v4.rs`
- `src/security/bsd_pledge.rs`
- `src/security/capability_sandbox.rs`
- `src/security/namespaces.rs`
- `src/security/cgroups.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Landlock v4 Access Control | ✅ Implemented | Filesystem and TCP port scoping |
| OpenBSD pledge & unveil | ✅ Implemented | Fine-grained process restrictions |
| FreeBSD Capsicum Sandboxing | ✅ Implemented | Capability-mode file descriptors |
| seccomp-BPF Syscall Filter | ✅ Implemented | Allow/deny BPF programs |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** IMA (Integrity Measurement Architecture) executable hash verification.
2. **[PRIORITY: HIGH]** Hardware SMEP, SMAP, and CET shadow stack enforcement.
3. **[PRIORITY: MEDIUM]** Kernel Address Space Layout Randomization (KASLR) entropy enhancement.

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
- BPF bytecode verifier and JIT compiler
- Trie-based path lookup for unveil filesystem boundaries
- Bell-LaPadula lattice mathematical security policy solvers

## 9. Security & Hardening Directives
- Default-deny access policy on all newly created userland boundaries
- Memory safety in 100% of security evaluation logic (Rust safe subsets)

## 10. Performance Targets vs Competitors
- Syscall sandboxing overhead < 1.5% compared to raw kernel invocations
- Zero privilege escalation vulnerabilities across continuous fuzzing campaigns

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
- **Depends on:** Kernel Core, Virtual Memory, Process Management
- **Required by:** All Subsystems, User Applications, Network Daemons

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `07-Security`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- OpenBSD pledge(2) and unveil(2) Manual Pages
- FreeBSD Capsicum Architecture & Capability Mode Papers
- Linux Kernel Landlock LSM Documentation
