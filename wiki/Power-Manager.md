# Power Manager

## Overview

The Power Manager provides comprehensive power management inspired by Linux Mint's power settings and Omarchy's power utilities. It supports battery monitoring, power profiles, sleep/hibernate actions, and automatic power management.

## Features

- **Power Sources**: Battery, AC, UPS detection
- **Battery Status**: Charging, Discharging, Full, Not Charging, Unknown
- **Power Profiles**: Performance, Balanced, Power Saver
- **Sleep Actions**: Suspend, Hibernate, Hybrid Sleep, Shutdown
- **Battery Monitoring**: Percentage, capacity, energy, power, voltage
- **Time Estimation**: Time to empty and time to full calculations
- **Low Battery Detection**: Configurable low and critical thresholds
- **Auto-Sleep**: Configurable sleep timeouts and screen management
- **Auto-Suspend**: Automatic suspend on low battery and lid close

## Components

### PowerSource

```rust
pub enum PowerSource {
    Battery,  // Running on battery
    AC,       // Connected to AC power
    UPS,      // Connected to UPS
}
```

### PowerBatteryStatus

```rust
pub enum PowerBatteryStatus {
    Charging,      // Battery is charging
    Discharging,   // Battery is discharging
    Full,          // Battery is full
    NotCharging,   // Battery not charging
    Unknown,       // Unknown status
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

### SleepAction

```rust
pub enum SleepAction {
    Suspend,      // Suspend to RAM
    Hibernate,    // Hibernate to disk
    HybridSleep,  // Hybrid suspend/hibernate
    Shutdown,     // Power off
}
```

### BatteryInfo

Battery metrics including:
- Percentage (0-100)
- Status
- Capacity (Wh)
- Energy (Wh)
- Power draw (W)
- Voltage (V)
- Time to empty (minutes)
- Time to full (minutes)

### PowerConfig

Power management configuration including:
- Power source
- Current profile
- Auto-sleep settings
- Screen dim/off timeouts
- Low/critical battery thresholds
- Suspend on low battery
- Suspend on lid close

### PowerManager

Main management interface with:
- Battery monitoring
- Power profile management
- Low/critical battery detection
- Sleep action suggestions
- Time remaining estimation
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::system::PowerManager;

let mut manager = PowerManager::new();

// Get battery information
let battery = manager.get_battery();

// Get current power profile
let profile = manager.get_profile();

// Check if battery is low
let is_low = manager.is_battery_low();
```

### Battery Monitoring

```rust
// Update battery status
manager.update_battery(75.0, PowerBatteryStatus::Discharging);

// Check battery levels
if manager.is_battery_low() {
    println!("Battery is low!");
}

if manager.is_battery_critical() {
    println!("Battery is critical!");
}
```

### Power Profile Management

```rust
// Set power profile
manager.set_profile(PowerProfile::Performance);

// Switch to power saver
manager.set_profile(PowerProfile::PowerSaver);
```

### Time Estimation

```rust
// Get estimated time remaining
if let Some(time) = manager.get_time_remaining() {
    println!("Time remaining: {}", time);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Battery: {}%", stats.battery_percentage);
println!("Status: {}", stats.battery_status.as_str());
println!("Profile: {}", stats.power_profile.as_str());
```

## AI Agent Maintenance Instructions

When maintaining the Power Manager:

1. **Battery Accuracy**: Ensure battery percentage and status are accurately updated
2. **Time Estimation**: Maintain accurate time-to-empty and time-to-full calculations
3. **Threshold Detection**: Ensure low and critical battery thresholds work correctly
4. **Profile Switching**: Maintain proper power profile switching logic
5. **Sleep Suggestions**: Ensure sleep action suggestions are appropriate for battery levels
6. **Configuration**: Keep power configuration defaults sensible and secure

## Testing

Run the unit tests with:

```bash
cargo test --lib system::power_manager
```

## Future Enhancements

- Integration with actual battery APIs (/sys/class/power_supply)
- Adaptive power profiles based on workload
- Per-application power limits
- Thermal management integration
- Battery health monitoring
- Charge cycle tracking
- Power usage history and graphs
