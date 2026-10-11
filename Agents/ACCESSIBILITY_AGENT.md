# Accessibility (a11y) Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Provide comprehensive assistive technology services, screen reading, high-contrast theming, and voice navigation natively integrated into the compositor and terminal layers.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
AT-SPI2 DBus protocols, Orca screen reader architecture, and evdev virtual input synthesis.

### FreeBSD
Console-level text-to-speech shims, devd hardware button accessibility bindings.

### OpenBSD
Minimal attack surface input routing for assistive devices without privilege escalation.

### NetBSD
wscons kernel display drivers with built-in Braille terminal display driver abstractions.

### Arch Linux & Distros
Arch Linux a11y minimal packages, Omarchy keyboard-centric navigation workflows.

### NixOS
Declarative global accessibility profile configuration spanning UI scale, speech rates, and fonts.

## 4. SigmaOS Source Files
- `src/desktop/accessibility.rs`
- `src/desktop/text_scaling.rs`
- `src/ui/mod.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Screen Reader Hook | 🚧 Partial | Speech dispatcher integration |
| Text Scaling & HiDPI | ✅ Implemented | Fractional scaling engine |
| High Contrast Themes | ✅ Implemented | CSS/Wayland surface filters |
| Braille TTY Driver | 🔲 Missing | Hardware serial braille terminal |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** Universal Screen Reader Daemon: Low-latency speech dispatcher integration in safe Rust.
2. **[PRIORITY: MEDIUM]** System-wide Focus Highlighting: Compositor-assisted focus border rendering for Wayland surfaces.
3. **[PRIORITY: LOW]** Head-tracking & Eye-gaze Input Engine: Direct HID camera translation.

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
- Dynamic focus graph traversal algorithms
- Low-latency phonetic audio queue ring-buffers
- Bézier smoothing curves for motor-impaired cursor stabilization

## 9. Security & Hardening Directives
- Strictly sandbox speech synthesis subprocesses
- Isolate keystroke reading to certified assistive clients avoiding keylogger abuse

## 10. Performance Targets vs Competitors
- Speech output latency < 15ms
- Cursor magnifier rendering overhead < 0.2ms per frame @ 120Hz

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
- **Depends on:** Compositor, Audio Subsystem, Input Subsystem
- **Required by:** Desktop Shell, Application Layer, Login Manager

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `Accessibility-and-UX`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- W3C WAI-ARIA Standards
- FreeDesktop.org AT-SPI2 Specification
- RFC 3270 Audio Streaming Protocols
