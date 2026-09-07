# AI Agent Omarchy Navigation & Window Management Architecture

## Executive Overview

SigmaOS Zenith Desktop Environment incorporates keyboard-centric tiling navigation protocols inspired by Omarchy Linux and Hyprland Wayland compositors. Implemented across `src/desktop/zenith_advanced_features.rs` and `src/desktop/mod.rs`, Zenith DE provides Hyprland-style dwindle vs. scrolling window layout modes (`WindowLayoutMode`), keyboard window grouping (`Super+G`), window popping/floating overlay (`Super+O`), scratchpad drop-down workspaces (`Super+Grave` / `Super+S`), and directional focus traversal (`Super+Arrows`) built with zero-dependency Rust primitives (`#![no_std]`).

This document serves as the architectural reference for AI coding agents inspecting, configuring, or managing Omarchy-style navigation and window management in SigmaOS.

---

## Subsystem Architecture & Navigation Pipeline

```
                                +-----------------------------------+
                                |    Keyboard Input / Hotkey Event  |
                                |       Super + [Key Combo]         |
                                +-----------------------------------+
                                                  |
                                                  v
                                +-----------------------------------+
                                |    Zenith Window Manager          |
                                |  (src/desktop/zenith_advanced)    |
                                +-----------------------------------+
                                 /          |           |          \
                                /           |           |           \
            +-----------------------+ +---------------+ +---------------+ +-----------------------+
            | Tiling Layout Engine  | | Window Group  | | Popped Window | | Scratchpad Workspace  |
            | Dwindle vs Scrolling  | | Super+G Group | | Super+O Pin | | Super+Grave Dropdown|
            | DynamicBSP / Master   | | Super+Ctrl+Nav| | Floating    | | Quake Terminal Overlay|
            +-----------------------+ +---------------+ +---------------+ +-----------------------+
                                \           |           |           /
                                 \          |           |          /
                                  v         v           v         v
                                +-----------------------------------+
                                |   Zenith Compositor Rendering     |
                                +-----------------------------------+
```

### Core Navigation Components

1. **Hyprland-Style Window Layout Modes (`WindowLayoutMode`)**:
   - `Dwindle`: Recursively splits active workspace tiles vertically and horizontally as new windows open.
   - `Scrolling`: Lines up open windows side-by-side beyond display edges with horizontal pan navigation (`Super+L`).
   - `MasterStack` & `DynamicBSP`: BSD Lumina / Pop!_OS COSMIC tiling modes.

2. **Window Grouping (`Super+G`)**:
   - Groups active windows into a tabbed/stacked container. Cycle grouped windows via `Super+Ctrl+Left/Right` or `Super+Alt+1/2/3/4`.

3. **Window Popping & Pinning (`Super+O`)**:
   - Pops the focused window out of the tiling layout into a floating overlay pinned across workspace switching.

4. **Scratchpad Workspace (`Super+Grave` / `Super+S`)**:
   - Drop-down terminal/TUI workspace that slides over active windows (Quake console style) without interrupting the underlying workspace layout.

---

## Zero-Allocation Guardrails

AI agents tuning navigation routines must observe these zero-allocation constraints:
- Window focus traversal calculates active bounding box intersections in $O(1)$ stack registers.
- Scratchpad slide transitions compute offset percentages without vector allocations.

---

## Related Architectural References
- `src/desktop/zenith_advanced_features.rs` - Master Zenith DE layout engine.
- `src/desktop/mod.rs` - Desktop subsystem module root.
- `docs/AI_AGENTS_DESKTOP_ENVIRONMENTS_MANAGEMENT.md` - Desktop environment architecture.
