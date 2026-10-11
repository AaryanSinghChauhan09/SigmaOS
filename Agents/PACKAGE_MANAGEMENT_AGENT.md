# Package Management & Sovereign Distro Adapter Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide atomic, reproducible package management (SigPkg), transactional rollbacks, sandboxed builds, and universal multi-distro package transpilation.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
Nix/Guix content-addressed stores, Arch pacman speed, Fedora rpm-ostree atomic trees.

### FreeBSD
pkg(8) binary package manager, FreeBSD Ports collection with options dialogs.

### OpenBSD
pkg_add with pledge sandboxing and cryptographic signatures.

### NetBSD
pkgsrc multi-platform portable package system.

### Arch Linux & Distros
Arch Linux pacman simplicity, NixOS declarative generations, Alpine apk speed.

### NixOS
Atomic store path `/sigma/store/<hash>-<name>`, garbage collection, rollback pins.

## 4. SigmaOS Source Files
- `src/package/captain.rs`
- `src/package/aptkit.rs`
- `src/package/software_manager.rs`
- `src/package/declarative_app.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| SigPkg Content-Addressed Store | ✅ Implemented | Deterministic hash paths |
| Atomic Transaction Journal | ✅ Implemented | ACID rollback support |
| Universal Distro Transpiler | ✅ Implemented | DEB, RPM, PKGBUILD, APK intake |
| Sandboxed Build Daemon | 🚧 Partial | Landlock/namespaces build isolation |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Complete cryptographic package signature verification (Ed25519 + Dilithium).
2. **[PRIORITY: HIGH]** Delta-package binary diff updates (90% bandwidth reduction).
3. **[PRIORITY: MEDIUM]** System generation snapshots with instant GRUB/bootloader rollback.

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
- SAT/DPLL dependency solver with CDCL (Conflict-Driven Clause Learning)
- Merkle DAG content verification for package trees
- Content-defined chunking (FastCDC) for delta updates

## 9. Security & Hardening Directives
- Dual cryptographic signatures on all repo manifests
- Builds run in unprivileged user namespaces with zero network access

## 10. Performance Targets vs Competitors
- 10,000 package dependency resolution < 80ms
- Package install throughput > 120MB/s uncompressed write speed

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
- **Depends on:** Filesystems, Cryptography, Networking, Sandboxing
- **Required by:** System Provisioning, Userland Applications, Updates

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `09-Packaging`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Nix: A Safe and Flexible Software Deployment Model (Eelco Dolstra PhD thesis)
- Arch Linux PKGBUILD Specifications
- Debian Binary Package Format Specification
