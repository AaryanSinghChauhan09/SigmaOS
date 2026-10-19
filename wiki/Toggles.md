# SigmaOS Toggles Components

## Overview
SigmaOS toggles manager providing system toggle management, competitive with Omarchy toggles (omancy-toggle-bar, omancy-toggle-idle, omancy-toggle-nightlight, omancy-toggle-notification-silencing, omancy-toggle-screensaver, omancy-toggle-suspend, omancy-toggle-touchpad, omancy-toggle-touchscreen).

## Features
- **Toggle types**: Bar, Idle, NightLight, NotificationSilencing, Screensaver, Suspend, Touchpad, Touchscreen, HybridGpu, CrashCapture, InputDevice
- **Toggle on/off** - Toggle individual settings
- **Set state** - Explicitly set toggle state
- **List all** - List all toggle states
- **Toggle count** - Get toggle count

## Structures

| Structure | Description |
|-----------|-------------|
| **TogglesManager** | Toggles manager |
| **ToggleType** | Bar, Idle, NightLight, etc. |
| **ToggleState** | On, Off |

## Usage Examples

```rust
// Create toggles manager
let mut mgr = TogglesManager::new();

// Toggle a setting
let new_state = mgr.toggle(ToggleType::NightLight);

// Set state explicitly
mgr.set(ToggleType::Touchpad, ToggleState::Off);

// Check state
let enabled = mgr.is_enabled(&ToggleType::Bar);

// List all toggles
let all = mgr.list_all();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/toggles/manager.rs` - Toggles manager implementation
- `wiki/Toggles.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
USER_FREEDOM_AND_CONTROL, SIMPLE_USER_INTERFACE, TOGGLE_STATE