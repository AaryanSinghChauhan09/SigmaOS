# SigmaOS On-Screen Display Components

## Overview
SigmaOS on-screen display (OSD) manager providing volume, brightness, media, network, and battery overlays, competitive with Omarchy OSD.

## Features
- **OSD types**: Volume, Brightness, Media, Network, Battery, Custom
- **Show/hide** - Show and hide OSD entries
- **Hide all** - Hide all OSD entries
- **Configurable duration** - Per-display duration
- **Icons** - Emoji icons per type

## Structures

| Structure | Description |
|-----------|-------------|
| **OsdManager** | OSD manager |
| **OsdEntry** | Individual OSD entry |
| **OsdType** | Volume, Brightness, Media, Network, Battery, Custom |

## Usage Examples

```rust
// Create OSD manager
let mut mgr = OsdManager::new();

// Show volume
let id = mgr.show_volume(75.0, 100.0);

// Show brightness
let id = mgr.show_brightness(60.0, 100.0);

// Show media
let id = mgr.show_media("Song Title".to_string(), Some("Artist".to_string()));

// Show battery
let id = mgr.show_battery(85.0, true);

// Hide an entry
mgr.hide(id).unwrap();

// Hide all
mgr.hide_all();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/osd/manager.rs` - OSD manager implementation
- `wiki/OSD.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
DIRECT_DEVICE_CONTROL, USER_FEEDBACK, TRANSPARENCY