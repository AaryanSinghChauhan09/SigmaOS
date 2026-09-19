# AI Agent Omarchy Navigation Management Guide

## Overview
This wiki guide details Omarchy-inspired keyboard-driven tiling navigation protocols for AI coding agents operating on SigmaOS. It covers Hyprland-style dwindle vs scrolling tiling layouts (`WindowLayoutMode`), window grouping (`Super+G`), window popping/floating overlays (`Super+O`), drop-down scratchpad workspaces (`Super+Grave` / `Super+S`), and directional focus traversal (`Super+Arrows`).

## Key Navigation Protocols
1. **Dwindle vs Scrolling Layouts**: Dwindle splits active tiles recursively; scrolling aligns windows side-by-side beyond screen edges (`Super+L`).
2. **Window Grouping**: `Super+G` groups windows into tabbed containers navigable via `Super+Ctrl+Arrows`.
3. **Scratchpad Workspace**: `Super+Grave` drops down a Quake-style overlay terminal without altering active tiling layouts.

## Layout Configuration Example (`src/desktop/zenith_advanced_features.rs`)
```rust
let mut zenith = ZenithAdvancedFeatures::new();
zenith.apply_desktop_inspiration(DesktopInspirationPreset::CosmicRust);
```

## Related Documents
- `docs/AI_AGENT_OMARCHY_NAVIGATION_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_OMARCHY_NAVIGATION_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENTS_UX_MANAGEMENT_GUIDE.md`
