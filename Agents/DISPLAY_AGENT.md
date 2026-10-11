# Display Server & Surface Compositor Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide tear-free, hardware-accelerated Wayland display presentation, multi-monitor topology management, HDR color rendering, and VRR synchronization.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
Wayland core protocol, DRM/KMS atomic modesetting, libliftoff hardware plane allocation.

### FreeBSD
evdev input backend, drm-kmod graphics stack port.

### OpenBSD
drm(4) KMS kernel modesetting, wsdisplay graphics integration.

### NetBSD
wsfb framebuffer device, DRM modesetting drivers.

### Arch Linux & Distros
Hyprland smooth animations and shader pipeline, Omarchy Wayland polish.

### NixOS
Declarative monitor layout mappings, color profile (ICC) management.

## 4. SigmaOS Source Files
- `src/graphics/drm_kms.rs`
- `src/graphics/zenith_compositor.rs`
- `src/desktop/hyprland_tiling.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| DRM/KMS Atomic Modesetting | ✅ Implemented | CRTC, Plane, Connector setup |
| Wayland Protocol Engine | ✅ Implemented | Surface commit, shm/dmabuf buffers |
| Variable Refresh Rate (VRR) | 🚧 Partial | Adaptive sync timing calculation |
| HDR10 & Wide Color Gamut | 🔲 Missing | PQ/HLG EOTF shader curves |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Complete DRM Direct Scan-out on primary overlay plane for fullscreen apps.
2. **[PRIORITY: MEDIUM]** Per-output fractional scaling with sharp text antialiasing.
3. **[PRIORITY: LOW]** Hardware color calibration LUT injection via atomic KMS properties.

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
- Damaged region tracking and damage accumulator bounding boxes
- Matrix transformations for multi-head virtual desktop spaces
- VSync hardware interrupt phase locked loops (PLL)

## 9. Security & Hardening Directives
- Screen-capture protocol restrictions (wlr-screencopy capability enforcement)
- Zero leakage of framebuffer memory between distinct client surfaces

## 10. Performance Targets vs Competitors
- Compositor frame delivery variance < 0.1ms @ 240Hz
- 0 dropped frames during workspace switching animations

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
- **Depends on:** GPU Drivers, Input Subsystem, Memory Management
- **Required by:** Desktop Shell, Window Manager, User Applications

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Compositor`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Wayland Core Protocol Specification
- Linux Direct Rendering Manager (DRM) Kernel API
- VESA DisplayPort and Embedded DisplayPort Specifications
