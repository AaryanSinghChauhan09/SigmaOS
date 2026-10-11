# Hardware Drivers & Abstraction Layer Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide robust, crash-resilient hardware drivers across x86_64 and ARM64 platforms with microkernel-inspired fault isolation.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
Linux driver core, sysfs device model, PCI MSI/MSI-X interrupts, USB xHCI subsystem.

### FreeBSD
Newbus device driver architecture, kernel device configuration autotuning.

### OpenBSD
config(8) declarative hardware driver autoconfiguration, strict DMA memory bounds.

### NetBSD
bus_space and bus_dma machine-independent driver abstractions.

### Arch Linux & Distros
Ubuntu automated driver detection, Arch Linux dkms driver compilation.

### NixOS
Declarative hardware configuration modules and firmware packaging.

## 4. SigmaOS Source Files
- `src/driver/ahci.rs`
- `src/driver/nvme_storage.rs`
- `src/driver/usb_xhci_host.rs`
- `src/driver/wifi.rs`
- `src/driver/bluez.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| PCI/PCIe Bus Enumeration | ✅ Implemented | Device discovery, BAR mapping |
| xHCI USB 3.x Host Controller | ✅ Implemented | Slot context, transfer rings |
| Wi-Fi 802.11 Framework | ✅ Implemented | Intel iwlwifi command execution |
| ACPI EC & Battery Driver | 🚧 Partial | Event notifications, status parsing |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Driver crash isolation: automated driver restart without kernel panic.
2. **[PRIORITY: MEDIUM]** Complete USB Type-C / USB-PD power delivery state machine.
3. **[PRIORITY: LOW]** Thunderbolt / USB4 PCIe tunneling controller.

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
- PCIe configuration space binary tree index
- Ring buffer producer-consumer queues with DMA coherence barriers
- Scatter-gather list coalescing for high-throughput DMA transfers

## 9. Security & Hardening Directives
- IOMMU / VT-d memory page table enforcement restricting peripheral DMA regions
- Firmware binary signature verification prior to loading

## 10. Performance Targets vs Competitors
- Driver initialization time < 200ms total during cold boot
- Zero kernel panics on malformed peripheral hotplug

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
- **Depends on:** Interrupt Controller, Memory Management (DMA), Bus Enumerators
- **Required by:** Storage, Networking, Graphics, Audio, Input

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Drivers`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- PCI Local Bus Specification Revision 3.0 / PCIe Base Spec 5.0
- Extensible Host Controller Interface (xHCI) for USB Spec 1.2
- Intel Virtualization Technology for Directed I/O (VT-d) Spec
