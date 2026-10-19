# SigmaOS Battery Manager Components

## Overview
SigmaOS battery manager providing battery charge monitoring and status tracking, competitive with Omarchy battery controls (omancy-battery-low, omancy-battery-present, omancy-battery-status).

## Features
- **Battery device management** - Add, list, get battery devices
- **Charge monitoring** - Track charge percentage
- **Status tracking** - Charging, Discharging, Full, Empty, Unknown
- **Low threshold** - Configurable low battery threshold
- **Critical threshold** - Configurable critical battery threshold
- **Total charge** - Average charge across all batteries
- **Charging detection** - Check if any battery is charging

## Structures

| Structure | Description |
|-----------|-------------|
| **BatteryManager** | Battery manager with device registry |
| **BatteryDevice** | Individual battery device |
| **BatteryStatus** | Charging, Discharging, Full, Empty, Unknown |

## Usage Examples

```rust
// Create battery manager
let mut mgr = BatteryManager::new();

// Add a battery
mgr.add_battery(BatteryDevice {
    id: "bat0".to_string(),
    name: "Battery 0".to_string(),
    present: true,
    charge_percent: 85.0,
    status: BatteryStatus::Discharging,
    capacity_mah: Some(5000),
    current_mah: None,
    voltage_mv: None,
    temperature_c: None,
    cycle_count: None,
});

// Update charge
mgr.update_charge("bat0", 75.0).unwrap();

// Update status
mgr.update_status("bat0", BatteryStatus::Charging).unwrap();

// Check thresholds
let is_low = mgr.is_any_low();
let is_critical = mgr.is_any_critical();
let is_charging = mgr.is_any_charging();

// Get total charge
let total = mgr.total_charge_percent();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Hardware (P1)

## Related Files
- `src/battery/manager.rs` - Battery manager implementation
- `wiki/Battery.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
POWER_MANAGEMENT, LOW_LEVEL_CONTROL, DIRECT_DEVICE_CONTROL, USER_NOTIFICATION,
MEMORY_SAFETY