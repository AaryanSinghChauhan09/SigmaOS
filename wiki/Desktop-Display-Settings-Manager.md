# Desktop Display Settings Manager

## Overview

The Desktop Display Settings Manager provides comprehensive display management inspired by Linux Mint's display settings and Omarchy's display utilities. It supports multiple displays, display modes, resolution configuration, refresh rate control, and primary display selection.

## Features

- **Display Modes**: Extend, Mirror, Single, Disable
- **Refresh Rates**: Configurable refresh rates in Hz
- **Resolutions**: Configurable width and height
- **Display Management**: Add, remove, and manage displays
- **Primary Display**: Track and set primary display
- **Display Enable/Disable**: Enable or disable individual displays
- **Display Mode Control**: Set global display mode (extend, mirror, single, disable)
- **Resolution Configuration**: Set resolution per display
- **Refresh Rate Configuration**: Set refresh rate per display
- **Connected Displays**: Filter displays by connection status
- **Enabled Displays**: Filter displays by enabled status
- **Statistics**: Track display counts, primary display status, and mode

## Components

### DesktopDisplayMode

```rust
pub enum DesktopDisplayMode {
    Extend,   // Extend desktop across displays
    Mirror,   // Mirror displays
    Single,   // Single display mode
    Disable,  // Disable display
}
```

### DesktopRefreshRate

Refresh rate with:
- Value in Hz

### DesktopResolution

Resolution with:
- Width in pixels
- Height in pixels

### DesktopDisplay

Display with:
- Display ID and name
- Connection status
- Enable status
- Primary flag
- Resolution
- Refresh rate
- Display mode

### DesktopDisplaySettingsManager

Main management interface with:
- Display management (add, remove, retrieve)
- Primary display tracking
- Display enable/disable
- Display mode control
- Resolution configuration
- Refresh rate configuration
- Connected/enabled display filtering
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopDisplaySettingsManager;

let mut manager = DesktopDisplaySettingsManager::new();

// Get configuration
println!("Total displays: {}", manager.get_displays().len());
println!("Primary display: {:?}", manager.get_primary_display());
println!("Display mode: {}", manager.get_display_mode().as_str());
```

### Display Management

```rust
// Add display
let display = DesktopDisplay::new("HDMI-1".to_string(), "HDMI-1".to_string());

let id = manager.add_display(display);

// Remove display
manager.remove_display(&id);
```

### Primary Display

```rust
// Set primary display
manager.set_primary_display(&id);

// Get primary display
if let Some(display) = manager.get_primary_display() {
    println!("Primary display: {}", display.name);
}
```

### Display Enable/Disable

```rust
// Enable display
manager.enable_display(&id);

// Disable display
manager.disable_display(&id);
```

### Display Mode

```rust
// Set display mode
manager.set_display_mode(DesktopDisplayMode::Mirror);
println!("Display mode: {}", manager.get_display_mode().as_str());
```

### Resolution Configuration

```rust
// Set display resolution
manager.set_display_resolution(
    &id,
    DesktopResolution::new(2560, 1440),
);
```

### Refresh Rate Configuration

```rust
// Set display refresh rate
manager.set_display_refresh_rate(
    &id,
    DesktopRefreshRate::new(144),
);
```

### Connected/Enabled Displays

```rust
// Get connected displays
let connected = manager.get_connected_displays();
for display in connected {
    println!("Connected: {}", display.name);
}

// Get enabled displays
let enabled = manager.get_enabled_displays();
for display in enabled {
    println!("Enabled: {}", display.name);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total displays: {}", stats.total_displays);
println!("Connected displays: {}", stats.connected_displays);
println!("Enabled displays: {}", stats.enabled_displays);
println!("Primary display set: {}", stats.primary_display_set);
println!("Mode: {}", stats.mode);
```

## Default Display

The manager includes a default display:

- **Name**: eDP-1
- **Connected**: true
- **Enabled**: true
- **Primary**: true
- **Resolution**: 1920x1080
- **Refresh Rate**: 60Hz

## Default Configuration

The Display Settings Manager includes default configuration:

- **Primary Display**: eDP-1
- **Display Mode**: Extend
- **Default Display**: 1920x1080 @ 60Hz

## AI Agent Maintenance Instructions

When maintaining the Display Settings Manager:

1. **DRM/KMS Integration**: Integrate with DRM/KMS for display detection
2. **EDID Parsing**: Add EDID parsing for display capabilities
3. **Color Profile**: Add color profile management
3. **HDR Support**: Add HDR display support
4. **Scaling**: Add display scaling configuration
5. **Rotation**: Add display rotation support
6. **Multi-Monitor Layout**: Add layout configuration for multiple monitors
7. **Display Detection**: Add automatic display detection
8. **Display Hotplug**: Add display hotplug support
9. **Virtual Displays**: Add virtual display support
10. **Display Config Profiles**: Add display configuration profiles

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::display_settings_manager
```

## Future Enhancements

- DRM/KMS integration for display detection
- EDID parsing for display capabilities
- Color profile management
- HDR display support
- Display scaling configuration
- Display rotation support
- Multi-monitor layout configuration
- Automatic display detection
- Display hotplug support
- Virtual display support
- Display configuration profiles
