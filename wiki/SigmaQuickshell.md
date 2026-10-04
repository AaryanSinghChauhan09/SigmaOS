# SigmaQuickshell — Dynamic Desktop Shell Framework

> **Status:** In Progress
> **Language:** Rust
> **Source:** [`src/desktop/sigma_quickshell.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/tree/main/src/desktop/sigma_quickshell.rs)

## Overview

SigmaQuickshell is SigmaOS's dynamic, reactive desktop shell framework, directly inspired by [Quickshell](https://quickshell.outfoxxed.me/) — the QML-based shell used in Omarchy (DHH's opinionated Arch Linux + Hyprland distribution). Unlike Qt-based Quickshell, SigmaQuickshell is implemented entirely in pure safe Rust with zero external dependencies, making it `no_std`-friendly and fully auditable.

The system implements Quickshell's philosophy: declarative widget descriptions, reactive data binding, and layer-shell surface management, but expressed as Rust structs instead of QML.

## Architecture

```
SigmaQuickshell
├── ReactiveCell<T>          ← Observable state with generation tracking
├── ShellWidget              ← Layer surface descriptor (wlr-layer-shell)
│   ├── PanelContent         ← Top/bottom bars with applets
│   ├── NotificationContent  ← Notification popups
│   ├── LauncherContent      ← App launcher with fuzzy scoring
│   └── StatusBarContent     ← Waybar-compatible JSON output
├── HyprlandEvent            ← IPC event parser (workspace, fullscreen, submap)
└── SigmaQuickshell          ← Main shell manager
```

### Key Components

- **ReactiveCell**: Generational reactive state cell — notifies on change, tracks generation
- **ShellWidget**: Layer surface with Quickshell-style anchor system (top/bottom/left/right)
- **AppEntry + LauncherContent**: App launcher with frequency-weighted fuzzy scoring
- **StatusBarModule**: Waybar JSON-compatible module output
- **HyprlandEvent**: Parses Hyprland IPC socket events for workspace/window awareness

## Key Features

- **Layer-shell anchoring**: `Anchors::TOP_FILL`, `BOTTOM_FILL`, `FILL` constants matching wlr-layer-shell protocol
- **Reactive state**: `ReactiveCell<T>` with generation tracking for efficient re-rendering
- **Omakase defaults**: `SigmaQuickshell::default_top_panel()` provides an instant, opinionated panel layout
- **App scoring**: Frequency-weighted fuzzy matching for launcher results
- **Waybar compatibility**: `StatusBarModule::to_waybar_json()` for panel bar integration
- **Hyprland IPC**: Parses `workspace>>N`, `fullscreen>>1`, `submap>>name` events
- **Zero dependencies**: Pure Rust, no Qt, no GTK, no external crates

## Inspiration

Directly inspired by:
- **[Quickshell](https://quickshell.outfoxxed.me/)** — Omarchy's QML desktop shell for Hyprland
- **[Omarchy](https://github.com/basecamp/omarchy)** — DHH's opinionated Arch Linux + Hyprland developer workstation
- **[Waybar](https://github.com/Alexays/Waybar)** — Reference for JSON status module format
- **wlr-layer-shell** — Wayland protocol for shell surfaces (panels, notifications, launchers)

## Source Files

| File | Purpose |
|------|---------|
| `sigma_quickshell.rs` | Main module: reactive cells, widget system, panel/launcher/notif content, Hyprland IPC |

## Configuration

Widgets are configured programmatically:

```rust
let mut shell = SigmaQuickshell::new("/etc/sigmaos/shell.toml");
// Use default Omakase-style top panel
shell.add_widget(SigmaQuickshell::default_top_panel());
// Set active workspace
shell.set_active_workspace(2);
```

### Default Panel Layout

The default panel (inspired by Omarchy's Quickshell layout) includes:
- **Left**: App menu, workspace indicator
- **Center**: Date + time clock
- **Right**: CPU, Memory, Network, Volume, Battery, System Tray

## Usage

```bash
# Check compilation
cargo check --lib -- desktop::sigma_quickshell

# Run tests
cargo test --lib -- desktop::sigma_quickshell::tests
```

## Testing

```bash
cargo test --lib -- desktop::sigma_quickshell
```

**Test coverage:**
- `test_reactive_cell_change_detection` — generation tracking, no-change detection
- `test_launcher_scoring` — frequency-weighted fuzzy scoring validation
- `test_status_bar_json_output` — Waybar JSON format validation
- `test_quickshell_widget_management` — add/remove widget lifecycle
- `test_hyprland_event_parsing` — IPC event parser validation

## Roadmap

- [ ] IPC integration via Unix socket reader for live Hyprland events
- [ ] Config hot-reload via inotify watch
- [ ] CSS-in-Rust styling engine
- [ ] WGPU/Vulkan hardware-accelerated panel rendering
- [ ] Gesture recognition for scratchpad show/hide
- [ ] Multi-monitor per-widget monitor assignment

## See Also

- [Compositor](Compositor.md)
- [Theming-and-Customization](Theming-and-Customization.md)
- [Omarchy-Parity](Omarchy-Parity.md)
- [Shell-and-Userspace](Shell-and-Userspace.md)
