# Desktop Screen Orientation Manager

## Overview

The Desktop Screen Orientation Manager provides comprehensive screen orientation management inspired by Linux Mint's display configuration and Omarchy's rotation utilities. It supports automatic and manual orientation control for multiple displays.

## Features

- **Screen Orientations**: Normal, Rotate 90°, Rotate 180°, Rotate 270°
- **Orientation Policies**: Auto (sensor-based), Manual
- **Per-Display Configuration**: Configure orientation for each display
- **Configuration Management**: Add, remove, and retrieve configurations
- **Display ID**: Track configuration by display identifier
- **Auto-Orientation**: Support automatic orientation via sensors
- **Default Display**: Pre-configured default display
- **Statistics**: Track total configurations and auto-orientation count

## Components

### ScreenOrientation

```rust
pub enum ScreenOrientation {
    Normal,      // 0 degrees
    Rotate90,    // 90 degrees clockwise
    Rotate180,   // 180 degrees
    Rotate270,   // 270 degrees clockwise
}
```

### OrientationPolicy

```rust
pub enum OrientationPolicy {
    Auto,   // Automatic orientation via sensors
    Manual, // Manual orientation control
}
```

### ScreenOrientationConfig

Screen orientation configuration with:
- Display ID
- Screen orientation
- Orientation policy

### DesktopScreenOrientationManager

Main management interface with:
- Per-display configuration management
- Add, remove, and retrieve configurations
- Orientation policy configuration
- Display tracking
- Default display configuration
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopScreenOrientationManager;

let mut manager = DesktopScreenOrientationManager::new();

// Get configuration
println!("Total configs: {}", manager.get_configs().len());
println!("Default display config: {:?}", manager.get_config(&"display_0".to_string()));
```

### Configuration Management

```rust
// Add configuration
let id = manager.add_config(
    "HDMI-1".to_string(),
    ScreenOrientation::Normal,
    OrientationPolicy::Auto,
);

// Get configuration
if let Some(config) = manager.get_config(&id) {
    println!("Display: {}, Orientation: {:?}", config.display_id, config.orientation);
}

// Remove configuration
manager.remove_config(&id);
```

### List Configurations

```rust
// Get all configurations
let configs = manager.get_configs();
for config in configs {
    println!("Display: {}, Orientation: {:?}", config.display_id, config.orientation);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total configs: {}", stats.total_configs);
println!("Auto orientation: {}", stats.auto_orientation);
```

## Default Configuration

The Screen Orientation Manager includes default configuration:

- **Default Display**: eDP-1 (typical laptop display)
- **Default Orientation**: Normal
- **Default Policy**: Manual

## AI Agent Maintenance Instructions

When maintaining the Screen Orientation Manager:

1. **Sensor Integration**: Integrate with actual orientation sensors (accelerometer, gyroscope)
2. **Display Detection**: Detect available displays from compositor
3. **Hotkey Rotation**: Implement hotkey-based screen rotation
4. **Per-Application Settings**: Add per-application orientation preferences
5. **Transition Effects**: Add smooth transition animations
6. **Lock Orientation**: Add orientation lock functionality
6. **Auto-Rotate**: Implement auto-rotate based on device mode (tablet/laptop)
7. **Multi-Monitor**: Add multi-monitor orientation management
8. **Profile Support**: Add orientation profiles for different use cases
9. **Configuration Import/Export**: Add configuration import/export
10. **XDG Desktop Portal**: Integrate with XDG desktop portal for orientation

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::screen_orientation_manager
```

## Future Enhancements

- Integration with actual orientation sensors
- Display detection from compositor
- Hotkey-based screen rotation
- Per-application orientation preferences
- Smooth transition animations
- Orientation lock functionality
- Auto-rotate based on device mode
- Multi-monitor orientation management
- Orientation profiles
- Configuration import/export
- XDG desktop portal integration
