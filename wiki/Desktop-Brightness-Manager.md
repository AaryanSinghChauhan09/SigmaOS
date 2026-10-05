# Desktop Brightness Manager

## Overview

The Desktop Brightness Manager provides comprehensive brightness control inspired by Linux Mint's brightness settings and Omarchy's display utilities. It supports screen backlight, keyboard backlight, and indicator LED management with adaptive brightness capabilities.

## Features

- **Brightness Types**: Backlight (screen), Keyboard, Indicator LED
- **Device Management**: Add, remove, enable, disable brightness devices
- **Brightness Control**: Per-device brightness (0-100) with min/max limits
- **Brightness Adjustment**: Increase and decrease brightness by amount
- **Adaptive Brightness**: Disabled, Auto, Manual modes
- **Ambient Light Sensor**: Simulated ambient light level (0-100)
- **Auto-Adjustment**: Automatic brightness adjustment based on ambient light
- **Device Filtering**: List devices by type
- **Backlight Detection**: Get primary backlight device
- **Default Configuration**: Built-in display and keyboard backlight
- **Statistics**: Track device count, adaptive mode, and ambient light level

## Components

### BrightnessType

```rust
pub enum BrightnessType {
    Backlight,     // Screen backlight
    Keyboard,      // Keyboard backlight
    Indicator,     // Indicator LED
}
```

### AdaptiveBrightnessMode

```rust
pub enum AdaptiveBrightnessMode {
    Disabled,  // Adaptive brightness disabled
    Auto,      // Automatic adjustment based on ambient light
    Manual,    // Manual control only
}
```

### BrightnessDevice

Device structure with:
- Device ID and name
- Device type
- Current brightness (0-100)
- Maximum and minimum brightness limits
- Availability status

### BrightnessManager

Main management interface with:
- Device registration and management
- Brightness control with clamping
- Brightness increase/decrease operations
- Adaptive brightness mode selection
- Ambient light level tracking
- Auto-adjustment based on ambient light
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::BrightnessManager;

let mut manager = BrightnessManager::new();

// Get all devices
let devices = manager.get_devices();
println!("Total devices: {}", devices.len());

// Get adaptive mode
let mode = manager.get_adaptive_mode();
println!("Adaptive mode: {}", mode.as_str());
```

### Device Management

```rust
// Add a new device
let id = manager.add_device(
    "External Monitor".to_string(),
    BrightnessType::Backlight,
    100, // max brightness
    0,   // min brightness
);

// Get device by ID
if let Some(device) = manager.get_device(&id) {
    println!("Device: {}", device.name);
    println!("Brightness: {}", device.brightness);
}

// Remove device
manager.remove_device(&id);
```

### Brightness Control

```rust
// Set brightness (clamped to min/max)
manager.set_brightness(&id, 75);

// Get brightness
if let Some(brightness) = manager.get_brightness(&id) {
    println!("Brightness: {}", brightness);
}

// Increase brightness
manager.increase_brightness(&id, 10);

// Decrease brightness
manager.decrease_brightness(&id, 10);
```

### Device Availability

```rust
// Set device availability
manager.set_device_available(&id, false);

// Check if device is available
if let Some(device) = manager.get_device(&id) {
    println!("Available: {}", device.is_available);
}
```

### Adaptive Brightness

```rust
// Set adaptive mode
manager.set_adaptive_mode(AdaptiveBrightnessMode::Auto);

// Get ambient light level
let ambient = manager.get_ambient_light_level();
println!("Ambient light: {}", ambient);

// Set ambient light level (auto-adjusts backlight in auto mode)
manager.set_ambient_light_level(80);

// Manually trigger adjustment
manager.adjust_for_ambient_light();
```

### Device Filtering

```rust
// Get backlight devices
let backlights = manager.get_devices_by_type(BrightnessType::Backlight);
println!("Backlights: {}", backlights.len());

// Get keyboard devices
let keyboards = manager.get_devices_by_type(BrightnessType::Keyboard);
println!("Keyboards: {}", keyboards.len());

// Get primary backlight device
if let Some(backlight) = manager.get_backlight_device() {
    println!("Primary backlight: {}", backlight.name);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Devices: {}", stats.device_count);
println!("Adaptive mode: {}", stats.adaptive_mode.as_str());
println!("Ambient light: {}", stats.ambient_light_level);
```

## Default Configuration

The Brightness Manager includes default devices:

- **Built-in Display**: Backlight type, brightness 50%, available
- **Keyboard Backlight**: Keyboard type, brightness 50%, available

Default settings:
- **Adaptive Mode**: Disabled
- **Ambient Light Level**: 50%

## AI Agent Maintenance Instructions

When maintaining the Brightness Manager:

1. **Brightness Validation**: Ensure brightness values are within min/max range
2. **Device Validation**: Ensure device types are valid before adding
3. **Adaptive Logic**: Ensure auto-adjustment respects user manual overrides
4. **Ambient Sensor**: Integrate with actual ambient light sensor hardware
5. **Hotplug Detection**: Detect and handle display hotplug events
6. **Power Management**: Respect power state when adjusting brightness
7. **Backend Integration**: Integrate with actual brightness backend (systemd-backlight, kernel backlight)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::brightness_manager
```

## Future Enhancements

- Integration with actual backlight subsystem (Linux /sys/class/backlight)
- Real ambient light sensor integration
- Per-display brightness for multi-monitor setups
- Night light integration (blue light reduction)
- Gamma correction support
- Brightness transition animations
- Per-user brightness profiles
- Keyboard backlight auto-off on inactivity
- Brightness schedules (time-based)
- Smooth brightness transitions
