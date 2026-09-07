# AI Agent Omarchy Navigation Management Guidelines

## Purpose
These guidelines define operational protocols, keybinding maps, and safety guardrails for AI coding agents managing or configuring Omarchy-style tiling navigation in SigmaOS Zenith DE.

---

## Directives for AI Agents

1. **Hotkey Binding Mapping**:
   - Maintain standard Omarchy keybindings: `Super+Return` (Terminal), `Super+Space` (Menu), `Super+Q`/`Super+W` (Close window), `Super+F` (Fullscreen), `Super+G` (Group), `Super+O` (Pop/Float pin), `Super+Grave` (Scratchpad).

2. **Layout Mode Switching**:
   - Toggle dwindle vs scrolling layouts per workspace without resetting existing window geometry.

3. **Code Pattern: Switching Layouts and Scratchpads**:
```rust
let mut zenith = ZenithAdvancedFeatures::new();
zenith.apply_desktop_inspiration(DesktopInspirationPreset::CosmicRust);
zenith.set_layout_mode(WindowLayoutMode::Dwindle);
```

4. **Testing and Verification**:
   - Run `./run_sigma_tests.sh` to confirm UI/UX benchmarks and desktop compositor tests pass cleanly.

---

## Related Files
- `src/desktop/zenith_advanced_features.rs`
- `docs/AI_AGENT_OMARCHY_NAVIGATION_MANAGEMENT_ARCHITECTURE.md`
- `wiki/AI_AGENT_OMARCHY_NAVIGATION_MANAGEMENT.md`
