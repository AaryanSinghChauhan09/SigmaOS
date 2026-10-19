# SigmaOS Screensaver Components

## Overview
SigmaOS screensaver system providing screensaver with branding and animation, competitive with Omarchy screensaver (omancy-screensaver, branding, animation).

## Features
- **Screensaver types**: Blank, Logo, Animation, Custom
- **Idle timeout** - Configurable idle seconds
- **Activation/deactivation** - Activate and deactivate screensaver
- **Branding** - Custom branding text and logo
- **Themes** - Register and list screensaver themes
- **Animation speed** - Configurable animation speed
- **Fade duration** - Configurable fade duration

## Structures

| Structure | Description |
|-----------|-------------|
| **ScreensaverManager** | Screensaver manager |
| **ScreensaverConfig** | Screensaver configuration |
| **ScreensaverState** | Current screensaver state |
| **ScreensaverTheme** | Screensaver theme |
| **ScreensaverType** | Blank, Logo, Animation, Custom |
| **ScreensaverStatus** | Disabled, Enabled, Active, Idle |

## Usage Examples

```rust
// Create screensaver manager
let mut mgr = ScreensaverManager::new();

// Enable screensaver
mgr.enable();

// Set idle timeout
mgr.set_idle_seconds(300).unwrap();

// Set branding
mgr.set_branding_text(Some("SigmaOS".to_string()));

// Update idle time
mgr.update_idle_time(350); // Triggers idle state

// Activate screensaver
mgr.activate();

// Deactivate (wake)
mgr.deactivate();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/screensaver/manager.rs` - Screensaver manager implementation
- `wiki/Screensaver.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
IDLE_DETECTION, ENERGY_EFFICIENCY, USER_CONTROL