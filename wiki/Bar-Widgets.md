# SigmaOS Bar Widgets Components

## Overview
SigmaOS bar widgets system providing modular bar with interactive widgets, competitive with Omarchy's modular bar (workspaces, clock, weather, media, battery, etc.).

## Features
- **Modular bar** with draggable widgets
- **Widget kinds**: Workspaces, Active Window, Clock, Weather, Media, System Tray, Battery, Keyboard Layout, Network, Audio, DND, Night Light, Stay Awake, Screen Recording, Dictation, Reminders, Custom
- **Bar positions**: Top, Bottom, Left, Right
- **Transparency control** (double-click to toggle like Omarchy)
- **Widget visibility toggle**
- **Default widgets** pre-configured like Omarchy

## Default Widgets

| Widget | Position |
|--------|----------|
| Workspaces | Left |
| Active Window | Left |
| Clock | Center |
| Weather | Right |
| Media | Right |
| System Tray | Right |
| Battery | Right |
| Audio | Right |

## Structures

| Structure | Description |
|-----------|-------------|
| **BarManager** | Bar manager with config and widgets |
| **BarConfig** | Bar configuration (position, transparency, height, widgets) |
| **BarWidget** | Individual bar widget |
| **WidgetConfig** | Widget configuration |
| **WidgetKind** | Workspaces, ActiveWindow, Clock, Weather, Media, etc. |
| **WidgetPosition** | Left, Center, Right |
| **BarPosition** | Top, Bottom, Left, Right |

## Usage Examples

```rust
// Create bar manager
let mut bar = BarManager::new();

// Add custom widget
bar.add_widget("reminders".to_string(), WidgetKind::Reminders, WidgetPosition::Right);

// Set bar transparency
bar.set_transparency(0.8).unwrap();

// Set bar position
bar.set_position(BarPosition::Bottom);

// Toggle widget visibility
bar.toggle_widget("clock").unwrap();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/bar/widgets.rs` - Bar widgets implementation
- `wiki/Bar-Widgets.md` - Wiki documentation

## Principles Followed
PLUG_IN_DRIVER_MODEL, COMPOSITION_OVER_INHERITANCE, MODULARITY, COMPONENT_ISOLATION,
SINGLE_RESPONSIBILITY, LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY,
CLEAR_ABSTRACTION, ENCAPSULATION, CODE_REUSABILITY