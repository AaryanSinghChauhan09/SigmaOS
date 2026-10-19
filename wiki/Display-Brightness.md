# SigmaOS Display Brightness Components

## Overview
SigmaOS display brightness manager providing display brightness control, competitive with Omarchy display brightness (omancy-brightness-*, omancy-display-text-size).

## Features
- **Display management** - Add, list, remove displays
- **Brightness control** - Set brightness with range validation
- **Brightness increase/decrease** - Step-based adjustment
- **Brightness percentage** - Get brightness as percentage (0-100)
- **Active display** - Set and get active display
- **Built-in/external** - Track built-in vs external displays

## Structures

| Structure | Description |
|-----------|-------------|
| **DisplayBrightnessManager** | Display brightness manager |
| **DisplayDevice** | Individual display device |

## Usage Examples

```rust
// Create display brightness manager
let mut mgr = DisplayBrightnessManager::new();

// Add a display
mgr.add_display(DisplayDevice {
    id: "eDP-1".to_string(),
    name: "Built-in Display".to_string(),
    current_brightness: 50,
    min_brightness: 1,
    max_brightness: 100,
    is_built_in: true,
});

// Set brightness
mgr.set_brightness("eDP-1", 80).unwrap();

// Increase/decrease brightness
mgr.increase_brightness("eDP-1", 20).unwrap();
mgr.decrease_brightness("eDP-1", 10).unwrap();

// Get brightness percentage
let pct = mgr.get_brightness_percent("eDP-1").unwrap();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Hardware (P1)

## Related Files
- `src/display/brightness.rs` - Display brightness implementation
- `wiki/Display-Brightness.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL, USER_CONTROL, ENERGY_EFFICIENCY,
ACCESSIBILITY