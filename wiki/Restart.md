# SigmaOS Restart Components

## Overview
SigmaOS restart manager providing component restart management, competitive with Omarchy restart (omancy-restart-app, omancy-restart-audio, omancy-restart-bluetooth, omancy-restart-shell, omancy-restart-terminal, omancy-restart-wifi, etc.).

## Features
- **Components**: App, Audio, Bluetooth, Btop, Gum, Helix, Herdr, Hyprctl, Hyprsunset, Shell, Terminal, Tmux, Trackpad, Wifi, Xcompose, Custom
- **Restart tracking** - Track restart count per component
- **Status tracking** - Idle, Restarting, Restarted, Failed
- **Restart detection** - Check if component is restarting

## Structures

| Structure | Description |
|-----------|-------------|
| **RestartManager** | Restart manager |
| **RestartRecord** | Individual restart record |
| **RestartComponent** | App, Audio, Bluetooth, etc. |
| **RestartStatus** | Idle, Restarting, Restarted, Failed |

## Usage Examples

```rust
// Create restart manager
let mut mgr = RestartManager::new();

// Restart a component
mgr.restart(RestartComponent::Audio).unwrap();

// Check restart count
let count = mgr.restart_count(&RestartComponent::Audio);

// Check if restarting
let restarting = mgr.is_restarting(&RestartComponent::Audio);

// List all records
let records = mgr.list_records();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: System Administration (P1)

## Related Files
- `src/restart/manager.rs` - Restart manager implementation
- `wiki/Restart.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM,
SAFE_CONCURRENCY, RACE_FREE DESIGN, DEADLOCK_AVOIDANCE, EFFICIENT_SCHEDULING,
COMPOSITION_OVER_INHERITANCE