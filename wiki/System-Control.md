# SigmaOS System Control Components

## Overview
SigmaOS system control manager providing system state management, competitive with Omarchy system (omancy-system-factory-reset, omancy-system-lock, omancy-system-reboot, omancy-system-shutdown, omancy-system-sleep, omancy-system-wake, etc.).

## Features
- **System actions**: Reboot, Shutdown, Lock, Logout, Sleep, Wake, LidClose, FactoryReset, Stats
- **State management** - Running, Locked, Sleeping, Rebooting, ShuttingDown
- **Stats tracking** - Uptime, CPU, memory, disk, temperature, battery
- **Factory reset** - With confirmation requirement
- **Lock/unlock** - System lock and unlock

## Structures

| Structure | Description |
|-----------|-------------|
| **SystemControlManager** | System control manager |
| **SystemStats** | System statistics |
| **SystemAction** | Reboot, Shutdown, Lock, etc. |
| **SystemState** | Running, Locked, Sleeping, etc. |

## Usage Examples

```rust
// Create system control manager
let mut mgr = SystemControlManager::new();

// Lock system
mgr.lock().unwrap();

// Unlock system
mgr.unlock().unwrap();

// Reboot system
mgr.reboot().unwrap();

// Sleep system
mgr.sleep().unwrap();

// Wake system
mgr.wake().unwrap();

// Factory reset (requires confirmation)
mgr.factory_reset(true).unwrap();

// Update stats
let stats = SystemStats { ... };
mgr.update_stats(stats);
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: System Administration (P1)

## Related Files
- `src/system/control.rs` - System control implementation
- `wiki/System-Control.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
FAIL_FAST_SYSTEM, LOW_LEVEL_CONTROL, DIRECT_DEVICE_CONTROL, MINIMAL_KERNEL_PHILOSOPHY,
POWER_MANAGEMENT, USER_CONTROL