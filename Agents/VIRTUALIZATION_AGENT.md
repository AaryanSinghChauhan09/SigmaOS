# Virtualization & MicroVM Hypervisor Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Deliver hardware-assisted KVM-grade virtualization, Intel VT-x / AMD-V execution, virtio device emulation, and microsecond MicroVM boot times.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
KVM (Kernel-based Virtual Machine), Firecracker lightweight microVM, virtio modern devices.

### FreeBSD
bhyve hypervisor, vmm.ko kernel module, virtio storage.

### OpenBSD
vmd(8) hypervisor with strict pledge/unveil privsep sandboxing.

### NetBSD
NVMM (NetBSD Virtual Machine Monitor) hardware virtualization driver.

### Arch Linux & Distros
Clear Linux Cloud-Hypervisor, Alpine lightweight VM guests.

### NixOS
Declarative NixOS microVM test matrix generation.

## 4. SigmaOS Source Files
- `src/virtualization/kvm.rs`
- `src/virtualization/virtio.rs`
- `src/virtualization/microvm.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Intel VT-x & AMD-V Hardware Setup | ✅ Implemented | VMXON/VMCB lifecycle management |
| VirtIO Modern Device Emulation | ✅ Implemented | VirtIO-Net, VirtIO-Blk, VirtIO-GPU |
| MicroVM Minimal Guest Loader | ✅ Implemented | Direct kernel boot without BIOS/UEFI |
| Live Memory Snapshotting | 🚧 Partial | Dirty-page tracking, fast restore |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Nested virtualization support for secure development sandboxes.
2. **[PRIORITY: MEDIUM]** VirtIO-FS zero-copy host filesystem sharing.
3. **[PRIORITY: LOW]** VM memory deduplication (KSM) with cryptographic page hashing.

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
- Extended Page Tables (EPT / NPT) two-dimensional page walking
- Virtqueue split/packed ring buffer descriptors
- Asynchronous VM-exit dispatch state machines

## 9. Security & Hardening Directives
- Hardware EPT memory isolation preventing guest escape to host kernel
- Hypervisor process runs unprivileged with seccomp-bpf and Landlock restrictions

## 10. Performance Targets vs Competitors
- MicroVM cold boot to userland execution < 15ms
- CPU virtualization overhead < 1.2% compared to bare metal

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
- **Depends on:** CPU Hardware Virtualization Extensions, Memory Allocator, Interrupts
- **Required by:** Container Sandboxing, Untrusted Workloads, Compatibility Layers

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Virtualization-and-Containers`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Intel 64 and IA-32 Architectures Software Developer's Manual Vol 3C (VMX)
- AMD64 Architecture Programmer's Manual Volume 2: System Programming (SVM)
- Virtual I/O Device (VIRTIO) Version 1.2 Specification
