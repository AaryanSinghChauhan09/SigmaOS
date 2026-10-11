# Filesystems & Virtual File System (VFS) Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Deliver an ultra-high performance VFS, transactional snapshotting, transparent zstd compression, and seamless POSIX compliance.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
VFS inode/dentry caching layer, Btrfs subvolumes, ext4 journaling, OverlayFS.

### FreeBSD
ZFS integration, UFS2 with soft updates and snapshots, GEOM storage layers.

### OpenBSD
FFS2 with soft updates, mount options enforcing nodev, nosuid, and noexec.

### NetBSD
LFS (Log-structured File System), puffs (Pass-to-Userspace Filesystem Framework).

### Arch Linux & Distros
DragonFly HAMMER2 master-master clustering and snapshots, Arch btrfs subvolume layout.

### NixOS
Erase-your-darlings ephemeral rootfs on Btrfs/ZFS, content-addressed Nix store.

## 4. SigmaOS Source Files
- `src/fs/mod.rs`
- `src/fs/vfs.rs`
- `src/fs/btrfs.rs`
- `src/fs/ext4.rs`
- `src/fs/hammer2.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| VFS Inode & Dentry Cache | ✅ Implemented | Path resolution, mount points |
| ext4 Read/Write Engine | ✅ Implemented | Extent trees, inode tables |
| Btrfs COW Subvolume Engine | ✅ Implemented | B-tree indexing, snapshots |
| HAMMER2 Filesystem | 🚧 Partial | Transactional snapshots, PFS support |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Transparent in-line zstd / lz4 compression on block writes.
2. **[PRIORITY: HIGH]** Atomic filesystem snapshots with sub-millisecond creation time.
3. **[PRIORITY: MEDIUM]** Asynchronous metadata fsync via io_uring submission queues.

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
- Radix-tree and RCU-protected hash tables for dentry lookup
- Copy-on-Write (COW) B-tree transactional algorithms
- Two-phase commit journaling for ext4/UFS consistency

## 9. Security & Hardening Directives
- Posix ACLs and extended attributes (xattr) verification
- Cryptographic checksumming (BLAKE3/xxHash64) of data and metadata blocks

## 10. Performance Targets vs Competitors
- 500,000 file metadata lookups per second per core
- Zero corruption across ungraceful power loss simulations

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
- **Depends on:** Block Storage Layer, Memory Cache / Page Cache, Locks
- **Required by:** Process Management, User Applications, Package Manager

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `05-Filesystems`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- POSIX.1-2017 File System Specifications
- Btrfs On-Disk Format Documentation
- DragonFly BSD HAMMER2 On-Disk Specification
