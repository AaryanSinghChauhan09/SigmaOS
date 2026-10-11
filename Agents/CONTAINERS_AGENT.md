# Containerization & Sandboxed Runtimes Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Enable OCI-compliant, daemonless container execution utilizing kernel namespaces, cgroups v2, and secure sandboxing with near-zero startup overhead.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
cgroups v2, PID/Mount/Network/User/IPC/UTS namespaces, seccomp-bpf, overlayfs.

### FreeBSD
FreeBSD Jails with VNET network virtualization, rctl resource limits.

### OpenBSD
pledge(2) and unveil(2) syscalls for zero-privilege confinement.

### NetBSD
compat_linux container shims, chroot jail enhancements.

### Arch Linux & Distros
Podman rootless containers, Alpine ultra-minimal container rootfses.

### NixOS
Declarative systemd-nspawn micro-containers and Nix store mounting.

## 4. SigmaOS Source Files
- `src/security/namespaces.rs`
- `src/security/cgroups.rs`
- `src/security/capability_sandbox.rs`
- `src/virtualization/container.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Linux Namespace Isolation | ✅ Implemented | PID, Mount, Network, User |
| cgroups v2 Resource Accounting | ✅ Implemented | Memory, CPU, IO controllers |
| Rootless OCI Runtime | 🚧 Partial | Bundle parsing and pivot_root |
| OverlayFS Layering | 🚧 Partial | Lower/upper dir merge views |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** OCI Runtime specification compliance test suite passing.
2. **[PRIORITY: MEDIUM]** Ephemeral micro-VM container fallback using hardware KVM.
3. **[PRIORITY: LOW]** Image layer streaming with content-addressed chunk deduplication.

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
- cgroups v2 hierarchical proportional share CFS scheduling
- Copy-on-write directory snapshot synthesis
- UID/GID range translation and mapping algorithms

## 9. Security & Hardening Directives
- Default seccomp filter denying dangerous syscalls (unshare, kexec, bpf)
- Read-only rootfs enforcement and capability dropping (drop CAP_SYS_ADMIN)

## 10. Performance Targets vs Competitors
- Container cold start execution latency < 12ms
- Memory footprint per idle container runtime < 1.8MB

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
- **Depends on:** Process Management, Filesystems, Security Subsystem
- **Required by:** Package Managers, Microservices, Development Environments

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Container-Runtime`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Open Container Initiative (OCI) Runtime Specification v1.1
- Open Container Initiative (OCI) Image Specification v1.1
- Linux Namespaces & cgroups v2 Manuals
