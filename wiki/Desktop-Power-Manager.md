# Desktop Power Manager

## Overview

The Desktop Power Manager provides comprehensive power management inspired by Linux Mint's power settings and Omarchy's power utilities. It supports battery monitoring, power profiles, power action configuration, and automatic power management.

## Features

- **Power Actions**: Nothing, Suspend, Hibernate, Shutdown, Reboot, Screen Off
- **Power Profiles**: Performance, Balanced, Power Saver
- **Battery Monitoring**: Capacity, status, health, voltage, current, time remaining
- **Battery Status**: Charging, Discharging, Full, Unknown
- **Power Action Configuration**: Lid close, power button, battery critical actions
- **Timeout Configuration**: Auto-suspend and screen-off timeouts
- **Battery Detection**: Detect if on battery or charging
- **Battery Critical Detection**: Detect when battery falls below threshold
- **Average Capacity**: Calculate average capacity across all batteries
- **Statistics**: Track battery count, average capacity, power profile, and status

## Components

### PowerAction

```rust
pub enum PowerAction {
    Nothing,      // Do nothing
    Suspend,      // Suspend to RAM
    Hibernate,    // Hibernate to disk
    Shutdown,     // Power off
    Reboot,       // Reboot system
    ScreenOff,    // Turn off screen only
}
```

### PowerProfile

```rust
pub enum PowerProfile {
    Performance,  // Maximum performance
    Balanced,     // Balanced performance/power
    PowerSaver,   // Maximum power savings
}
```

### BatteryStatus

```rust
pub enum BatteryStatus {
    Charging,     // Battery is charging
    Discharging,  // Battery is discharging
    Full,         // Battery is full
    Unknown,      // Status unknown
}
```

### BatteryDevice

Battery device with:
- Device ID and name
- Capacity (0-100%)
- Battery status
- Health (0-100%)
- Voltage (Volts)
- Current (Amperes)
- Time remaining (minutes)

### DesktopPowerManager

Main management interface with:
- Battery management (add, remove, retrieve)
- Battery capacity and status updates
- Power profile selection
- Power action configuration
- Timeout configuration
- Battery detection (on battery, charging, critical)
- Average capacity calculation
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopPowerManager;

let mut manager = DesktopPowerManager::new();

// Get configuration
println!("Total batteries: {}", manager.get_batteries().len());
println!("Current profile: {}", manager.get_power_profile().as_str());
println!("Average capacity: {:.1}%", manager.get_average_capacity());
```

### Battery Management

```rust
// Add battery
let battery = BatteryDevice::new(
    "custom".to_string(),
    "Custom Battery".to_string(),
);

let id = manager.add_battery(battery);

// Remove battery
manager.remove_battery(&id);
```

### Battery Updates

```rust
// Update battery capacity
manager.update_battery_capacity("battery_0", 75);

// Update battery status
manager.update_battery_status("battery_0", BatteryStatus::Charging);

// Update time remaining
manager.update_battery_time_remaining("battery_0", 120);
```

### Power Profile

```rust
// Set power profile
manager.set_power_profile(PowerProfile::Performance);
manager.set_power_profile(PowerProfile::Balanced);
manager.set_power_profile(PowerProfile::PowerSaver);

// Get current profile
let profile = manager.get_power_profile();
```

### Power Actions

```rust
// Set lid close action
manager.set_lid_close_action(PowerAction::Suspend);

// Set power button action
manager.set_power_button_action(PowerAction::ScreenOff);

// Set battery critical action
manager.set_battery_critical_action(PowerAction::Hibernate);

// Get actions
let lid_action = manager.get_lid_close_action();
let power_action = manager.get_power_button_action();
```

### Timeouts

```rust
// Set auto-suspend timeout (minutes)
manager.set_auto_suspend_timeout(30);

// Set screen-off timeout (minutes)
manager.set_screen_off_timeout(10);
```

### Battery Detection

```rust
// Check if on battery
if manager.is_on_battery() {
    println!("Running on battery power");
}

// Check if charging
if manager.is_charging() {
    println!("Battery is charging");
}

// Check battery critical (below threshold)
if manager.get_battery_critical(20) {
    println!("Battery is critical!");
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total batteries: {}", stats.total_batteries);
println!("Average capacity: {:.1}%", stats.average_capacity);
println!("On battery: {}", stats.on_battery);
println!("Charging: {}", stats.charging);
println!("Current profile: {}", stats.current_profile.as_str());
```

## Default Configuration

The Power Manager includes default configuration:

- **Power Profile**: Balanced
- **Lid Close Action**: Suspend
- **Power Button Action**: Screen Off
- **Battery Critical Action**: Suspend
- **Auto-Suspend Timeout**: 30 minutes
- **Screen-Off Timeout**: 10 minutes
- **Default Battery**: 1 battery device

## AI Agent Maintenance Instructions

When maintaining the Power Manager:

1. **Battery Detection**: Integrate with actual battery monitoring from /sys/class/power_supply
2. **UPower Integration**: Integrate with UPower daemon for accurate battery data
3. **Profile Switching**: Implement automatic profile switching based on battery level
4. **TLP Integration**: Integrate with TLP for advanced power saving
5. **PM-Services**: Integrate with systemd-powered or equivalent
6. **Power Events**: Handle power events (AC connect/disconnect, lid close/open)
7. **Thermal Throttling**: Add thermal throttling support
8. **CPU Frequency Scaling**: Add CPU frequency scaling based on power profile
9. **Suspend/Hibernate**: Implement actual suspend/hibernate functionality
10. **Battery Calibration**: Add battery calibration and health monitoring

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::power_manager
```

## Future Enhancements

- Integration with /sys/class/power_supply
- UPower daemon integration
- Automatic profile switching based on battery level
- TLP integration for advanced power saving
- systemd-powered integration
- Power event handling (AC connect/disconnect, lid events)
- Thermal throttling support
- CPU frequency scaling based on power profile
- Actual suspend/hibernate implementation
- Battery calibration and health monitoring
