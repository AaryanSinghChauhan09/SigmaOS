# Input Subsystem & Event Pipeline Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide ultra-responsive, zero-latency input event processing for keyboards, multi-touch panels, gamepads, and drawing tablets.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
evdev (Event Device) kernel interface, libinput input event normalization.

### FreeBSD
evdev Linux-compatible subsystem, ums(4) and ukbd(4) drivers.

### OpenBSD
wsmouse and wskbd input multiplexers with minimal complexity.

### NetBSD
wsevent architecture and USB HID report descriptor parsers.

### Arch Linux & Distros
Omarchy Walker launcher low-latency input handling, SteamOS gamepad mapping.

### NixOS
Declarative touchpad gesture configuration and keycode remapping.

## 4. SigmaOS Source Files
- `src/driver/hid_input_device.rs`
- `src/desktop/keys.rs`
- `src/desktop/modal_keybindings.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| USB HID Report Descriptor Parser | ✅ Implemented | Mouse, Keyboard, Gamepad |
| Multi-touch Palm Rejection | ✅ Implemented | Geometric area heuristics |
| Modal Keybindings Engine | ✅ Implemented | Leader-key chords, vim modes |
| High-resolution Scrolling | 🚧 Partial | Pixel-precise scroll delta delivery |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** 1000Hz+ high polling rate gaming mouse event processing.
2. **[PRIORITY: MEDIUM]** Precision touchpad 3-finger and 4-finger gestures.
3. **[PRIORITY: LOW]** Hardware drawing tablet pressure and tilt sensitivity.

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
- Rolling window gesture recognizers (pinch-to-zoom, swipe)
- Kinetic inertia scrolling physics simulation
- Bounded lockless FIFO event queues

## 9. Security & Hardening Directives
- Input grab isolation ensuring unprivileged apps cannot inspect background keystrokes
- Cryptographic pairing for Bluetooth and wireless input peripherals

## 10. Performance Targets vs Competitors
- Input event kernel-to-compositor dispatch latency < 0.3ms
- Zero input event loss at 8000Hz mouse polling rates

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
- **Depends on:** USB Driver, Interrupt Handling, Desktop Compositor
- **Required by:** Compositor, Terminal, Window Manager, Applications

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Touchscreen-Support`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- USB Device Class Definition for Human Interface Devices (HID) v1.11
- Linux Input Subsystem Documentation
- FreeDesktop libinput Architecture Guide
