# Graphics Acceleration & GPU Computing Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Expose modern Vulkan API, GPU shader execution, hardware video decode/encode, and GPGPU computing with zero user-space overhead.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
Mesa 3D graphics drivers (RADV, ANV, NVK), DRM/KMS render nodes, DMA-BUF.

### FreeBSD
drm-kmod linuxkpi graphics porting infrastructure.

### OpenBSD
inteldrm and radeondrm kernel drivers with strict privsep.

### NetBSD
DRM3 kernel modesetting and gallium driver abstractions.

### Arch Linux & Distros
Clear Linux aggressive Mesa vector optimizations, Arch bleeding-edge Vulkan packages.

### NixOS
Declarative Vulkan ICD and VA-API driver selection.

## 4. SigmaOS Source Files
- `src/graphics/vulkan_api.rs`
- `src/graphics/gpu_driver.rs`
- `src/graphics/render3d.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Vulkan 1.3 Core API Implementation | ✅ Implemented | Command buffers, queues, pipelines |
| Hardware Video Decoding (VA-API) | 🚧 Partial | H.264/HEVC/AV1 bitstream unpackers |
| Raytracing Acceleration (VK_KHR) | 🚧 Partial | BVH tree construction |
| GPU Memory Allocator (VMA) | ✅ Implemented | Sub-allocation, buddy allocator |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Complete Vulkan conformance test suite (VK-CTS) validation.
2. **[PRIORITY: MEDIUM]** AV1 hardware decoding pipeline integration.
3. **[PRIORITY: LOW]** OpenCL / Compute shader unified runtime.

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
- Vulkan command buffer lockless recording
- Bounding Volume Hierarchy (BVH) GPU traversal
- Buddy allocation and slab caches for video memory management

## 9. Security & Hardening Directives
- GPU memory zeroing on reallocation to prevent cross-process data leakage
- Watchdog timer for GPU hangs with automated engine soft-reset

## 10. Performance Targets vs Competitors
- Vulkan draw call throughput > 15 million calls/sec
- 4K 120fps hardware video decoding with < 3% CPU utilization

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
- **Depends on:** PCIe Subsystem, Memory Management, Display Compositor
- **Required by:** Desktop Compositor, Media Players, Web Browsers, Games

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `GPU-and-Graphics`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Vulkan 1.3 Specification by Khronos Group
- SPIR-V Extended Instructions Specification
- VA-API (Video Acceleration API) Specification
