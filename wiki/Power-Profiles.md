# SigmaOS Power Profiles Components

## Overview
SigmaOS power profiles manager providing power profile management, competitive with Omarchy power profiles (omancy-powerprofiles-init, omancy-powerprofiles-list, omancy-powerprofiles-set).

## Features
- **Power profiles**: Performance, Balanced, PowerSave, ExtremePowerSave, Custom
- **Auto-switching** - Auto-switch based on power source (AC/Battery)
- **Battery integration** - Battery percentage monitoring
- **CPU governor** - Per-profile CPU governor
- **GPU power limit** - Per-profile GPU limit
- **Disk spin-down** - Per-profile disk settings
- **Network powersave** - Per-profile network settings
- **USB autosuspend** - Per-profile USB settings
- **Brightness reduce** - Per-profile brightness reduction

## Structures

| Structure | Description |
|-----------|-------------|
| **PowerProfilesManager** | Power profiles manager |
| **PowerProfileConfig** | Individual profile configuration |
| **PowerProfile** | Performance, Balanced, PowerSave, ExtremePowerSave, Custom |
| **PowerSource** | AcAdapter, Battery, Unknown |

## Usage Examples

```rust
// Create power profiles manager
let mut mgr = PowerProfilesManager::new();

// Set profile
mgr.set_profile(PowerProfile::PowerSave).unwrap();

// Auto-switch based on power source
mgr.set_power_source(PowerSource::Battery);
mgr.set_battery_percent(15);
mgr.auto_switch(); // Switches to ExtremePowerSave

// Add custom profile
let config = PowerProfileConfig { ... };
mgr.add_custom_profile("eco".to_string(), config);
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Power Management (P1)

## Related Files
- `src/power/profiles.rs` - Power profiles implementation
- `wiki/Power-Profiles.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
ENERGY_EFFICIENCY, POWER_MANAGEMENT, USER_CONTROL