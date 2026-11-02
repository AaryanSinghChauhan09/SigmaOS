# SigmaOS-window-manager

Tiling and stacking window manager for SigmaOS. Manages workspaces, window focus, layouts, and keybindings. Inspired by the best ideas in Hyprland, sway, i3, and niri.

## Upstream Inspiration

- [Hyprland](https://hyprland.org/) — dynamic tiling with animations and titlebars
- [sway](https://github.com/swaywm/sway) — i3-compatible Wayland WM
- [i3](https://i3wm.org/) — tiling WM with excellent workspace model
- [niri](https://github.com/YaLTeR/niri) — scrollable-tiling Wayland compositor

## Purpose

- **Workspace management**: create, switch, name workspaces; move windows between them
- **Tiling layout**: horizontal, vertical, and spiral tiling algorithms
- **Focus management**: focus-follows-mouse, explicit focus, focus ring
- **Keybinding dispatch**: xkbcommon-based keybinding table with action dispatch
- **Floating/scratchpad**: toggle windows between tiled and floating mode
- **Multi-monitor**: workspaces pinned to outputs or shared

## Build Instructions

```bash
cargo build --release
cargo test
```

## API Overview

### Core Types

- `WorkspaceManager` — owns all workspaces and routing
- `Workspace` — named set of windows with a layout mode
- `FocusTracker` — tracks the focused window across workspaces
- `KeybindingTable` — maps (modifiers, key) → `Action`
- `Action` — enum of all WM actions (focus, move, resize, etc.)

### Example

```rust
use sigma_wm::{WorkspaceManager, FocusTracker, KeybindingTable};

let mut wm = WorkspaceManager::new();
let ws = wm.create_workspace("main");
wm.focus_workspace(ws);
```

## Upstream Reference

| Upstream | Concept absorbed |
|----------|-----------------|
| [Hyprland dwindle layout](https://wiki.hyprland.org/Configuring/Dwindle-Layout/) | `DwindleLayout` — recursive spiral split with alternating axis |
| [dwm master+stack](https://dwm.suckless.org/) | `MasterStackLayout` — primary master pane + vertical slave stack |
| [Hyprland](https://github.com/hyprwm/Hyprland) | `LayoutKind` enum (Dwindle/Master/Stacking), `LayoutConfig` (gaps, border_px) |

`src/layout.rs` adds `DwindleLayout`, `MasterStackLayout`, `LayoutConfig`, `LayoutKind`, `Rect`, and `layout_of()`. Wire `pub mod layout;` + `pub use layout::{LayoutConfig, LayoutKind};` are in `src/lib.rs`.
