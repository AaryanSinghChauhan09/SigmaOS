# Storage Subsystem & Block Devices Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide lightning-fast, crash-resilient block storage, NVMe/AHCI hardware abstraction, asynchronous I/O pipelines, and snapshotting capabilities.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
NVMe-over-Fabrics, blk-mq multi-queue block layer, io_uring asynchronous block engine.

### FreeBSD
CAM (Common Access Method) storage subsystem, GEOM modular disk framework.

### OpenBSD
softraid crypto-volumes, sd(4) SCSI/NVMe storage architecture.

### NetBSD
RAIDframe software RAID architecture, wedge partition abstraction.

### Arch Linux & Distros
Arch Linux blk-mq tuning, DragonFly HAMMER2 transactional storage.

### NixOS
Declarative ZFS/Btrfs mount generation, atomic rollback boot configurations.

## 4. SigmaOS Source Files
- `src/driver/nvme_storage.rs`
- `src/driver/ahci.rs`
- `src/storage/block.rs`
- `src/io/io_uring_v2.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| NVMe PCIe Controller | ✅ Implemented | Multi-queue submission/completion |
| AHCI SATA Controller | ✅ Implemented | FIS parsing, NCQ support |
| io_uring v2 Asynchronous Engine | ✅ Implemented | SQPOLL kernel thread submission |
| ZFS/Btrfs Hybrid Engine | 🚧 Partial | Snapshot COW tree management |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** NVMe namespace dynamic hotplug and multipath routing.
2. **[PRIORITY: HIGH]** End-to-end data integrity checksum verification (xxHash64 block tagging).
3. **[PRIORITY: MEDIUM]** Write-amplification reduction wear-leveling cache for flash devices.

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
- Multi-queue lockless scheduler (blk-mq)
- B-link trees for transactional index lookups
- Asynchronous Ring Buffer submission with memory barrier synchronization

## 9. Security & Hardening Directives
- Hardware AES-XTS full disk encryption integration with TPM2 root-of-trust
- Secure erase and sanitize protocol execution

## 10. Performance Targets vs Competitors
- 1.5M IOPS single-socket NVMe random 4KB read performance
- < 12μs median submission-to-completion hardware latency

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
- **Depends on:** PCIe Bus Driver, Memory Management, Interrupt Handling (MSI-X)
- **Required by:** Filesystems, Swapping / Virtual Memory, Database Engines

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Storage-AHCI-and-NVMe-Engine`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- NVM Express Base Specification Revision 2.0
- Serial ATA Advanced Host Controller Interface (AHCI) 1.3.1
- IEEE 1667 Standard for Authentication in Host Attachments
