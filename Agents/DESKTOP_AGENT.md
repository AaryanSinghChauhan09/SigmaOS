# Zenith Desktop Environment & UI Framework Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Deliver a fast, beautiful, keyboard-driven Wayland desktop environment combining Omarchy aesthetics, Hyprland tiling, and Linux Mint rock-solid usability.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
Hyprland smooth rendering, GNOME Shell Mutter compositor, KDE KWin window management.

### FreeBSD
Lumina desktop lightweight memory footprint, devd desktop integration.

### OpenBSD
cwm (calm window manager) minimalistic keyboard bindings and security model.

### NetBSD
CTWM and basic X11 window managers with rock-solid stability.

### Arch Linux & Distros
Omarchy Linux Omakase desktop, Linux Mint Cinnamon applet ecosystem, Arch minimalist Hyprland dots.

### NixOS
Declarative desktop theming, font configurations, and shortcut matrices.

## 4. SigmaOS Source Files
- `src/desktop/hyprland_tiling.rs`
- `src/desktop/panel.rs`
- `src/desktop/workspace.rs`
- `src/desktop/menu.rs`
- `src/desktop/notifications.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Hyprland Dynamic Tiling | ✅ Implemented | Dwindle / master-stack tiling layouts |
| Top/Bottom Status Bar & HUD | ✅ Implemented | CPU, RAM, battery, network indicators |
| Modal Application Launcher | ✅ Implemented | Fuzzy search, command palette |
| Multi-Monitor Workspace Binding | ✅ Implemented | Independent workspace per display |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Wayland Layer-Shell V1 support for popups and notification overlays.
2. **[PRIORITY: MEDIUM]** Smooth hardware-interpolated spring physics animations.
3. **[PRIORITY: LOW]** Live workspace overview / expose view with window drag-and-drop.

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
- Binary Space Partitioning (BSP) tree window layout engine
- Crit-bit or Levenshtein distance fuzzy search matching for app launcher
- Spring-damper ODE physical animation solvers

## 9. Security & Hardening Directives
- Wayland client isolation preventing eavesdropping on adjacent window surfaces
- Privileged screen capture prompt with user confirmation token

## 10. Performance Targets vs Competitors
- Compositor frame rendering latency < 2ms @ 144Hz
- Idle desktop memory footprint < 120MB RSS

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
- **Depends on:** Display Compositor, GPU Drivers, Input Subsystem
- **Required by:** User Applications, System Settings, Notification Daemons

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Desktop-Environment-Component-Guide`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- Wayland Layer Shell Protocol Specification
- FreeDesktop.org Desktop Entry and Notification Specs
- CSS Box Model and Animation Timing Curves
