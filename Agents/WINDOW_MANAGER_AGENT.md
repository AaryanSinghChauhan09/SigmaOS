# Window Manager Agent

## Role
`SigmaWM` implements the compositing window manager for SigmaOS desktop — providing tiling, floating, and stacking window management with a built-in status bar and application launcher, inspired by Sway/Hyprland (Wayland) and i3 (X11).

## Status
🔴 Not Started — Critical for desktop environment.

## Source File
`src/desktop/window_manager.rs` (planned), `zenith_desktop/` (existing JS prototype)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Tiling WM | Sway / i3 / Hyprland | — | SigmaWM (tiling) |
| Floating WM | Openbox / Mutter | — | SigmaWM (float) |
| Stacking WM | KWin / Mutter | — | SigmaWM (stack) |
| Status bar | Waybar / i3bar | — | SigmaBar |
| App launcher | wofi / rofi / fuzzel | — | SigmaLaunch |
| Notifications | mako / dunst | — | SigmaNotify |
| Keybindings | `sway config` | — | SigmaKeys |
| Workspaces | i3 workspaces | — | SigmaWorkspace |
| Multi-monitor | wlr-output-management | — | SigmaMultiMon |
| Animations | Hyprland | — | SigmaAnim |

## Responsibilities
- Window placement engine (tiling BSP / manual tiling / floating)
- Focus management: click-to-focus, sloppy focus
- Workspace / virtual desktop management (1-10)
- Keybinding engine: configurable key → action mapping
- Status bar (`SigmaBar`): clock, workspaces, system tray, CPU/RAM
- Application launcher (`SigmaLaunch`): fuzzy search of .desktop files
- Desktop notifications (`SigmaNotify`): freedesktop.org notification spec
- Screenshot/screen recording integration
- Per-window rules: float, fullscreen, move to workspace
- Session management: save/restore window layout

## Implementation Notes
- Built on `SigmaCompositor` (Wayland compositor) via wlr-layer-shell
- Initial: extend `zenith_desktop/` prototype with Rust WM backend
- `WindowNode { id: u32, title: [u8;256], rect: Rect, workspace: u8, floating: bool }`
- Tiling: BSP tree, split horizontally/vertically per container

## Testing
Test WM logic with headless renderer:
```bash
cargo test --package sigma-wm 2>&1
```

## Future Development
1. **Hyprland animations** — Smooth window open/close/move animations
2. **Rounded corners** — Per-window corner rounding via shader
3. **Blur effects** — Background blur for transparent windows
4. **HDR gaming mode** — Disable compositor for full-screen HDR games
5. **Gesture support** — Touchpad gestures for workspace switching
6. **AI layout** — ML-predicted window placement based on app context
