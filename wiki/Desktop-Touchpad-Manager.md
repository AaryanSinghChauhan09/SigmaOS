# Desktop Touchpad Manager

## Overview

The Desktop Touchpad Manager provides comprehensive touchpad management inspired by Linux Mint's mouse/touchpad settings and Omarchy's input utilities. It supports tap-to-click, scrolling modes, palm detection, sensitivity, and advanced touchpad configuration.

## Features

- **Tap-to-Click**: Disabled, Enabled modes
- **Natural Scrolling**: Disabled, Enabled modes
- **Two-Finger Scrolling**: Disabled, Enabled modes
- **Edge Scrolling**: Disabled, Enabled modes
- **Palm Detection**: Disabled, Enabled modes
- **Sensitivity**: Configurable sensitivity (0-100)
- **Acceleration**: Configurable acceleration multiplier
- **Speed**: Configurable cursor speed (0-100)
- **Disable While Typing**: Disable touchpad while typing
- **Tap and Drag**: Enable tap-and-drag gestures
- **Per-Device Configuration**: Separate configuration for each touchpad
- **Touchpad Device Tracking**: Track touchpad devices with size and finger count
- **Statistics**: Track device count, configuration count, and enabled features

## Components

### TapToClickMode

```rust
pub enum TapToClickMode {
    Disabled,  // Tap-to-click disabled
    Enabled,   // Tap-to-click enabled
}
```

### NaturalScrolling

```rust
pub enum NaturalScrolling {
    Disabled,  // Standard scrolling
    Enabled,   // Natural/reverse scrolling
}
```

### TwoFingerScrolling

```rust
pub enum TwoFingerScrolling {
    Disabled,  // Two-finger scrolling disabled
    Enabled,   // Two-finger scrolling enabled
}
```

### EdgeScrolling

```rust
pub enum EdgeScrolling {
    Disabled,  // Edge scrolling disabled
    Enabled,   // Edge scrolling enabled
}
```

### PalmDetection

```rust
pub enum PalmDetection {
    Disabled,  // Palm detection disabled
    Enabled,   // Palm detection enabled
}
```

### TouchpadDevice

Touchpad device with:
- Device ID and name
- Vendor and product
- Width and height (mm)
- Number of fingers supported

### TouchpadConfiguration

Touchpad configuration with:
- Device ID
- Tap-to-click mode
- Natural scrolling mode
- Two-finger scrolling mode
- Edge scrolling mode
- Palm detection mode
- Sensitivity (0-100)
- Acceleration multiplier
- Speed (0-100)
- Disable while typing flag
- Tap and drag flag

### DesktopTouchpadManager

Main management interface with:
- Device management (add, remove, retrieve)
- Configuration management
- Per-device settings
- Scrolling modes configuration
- Sensitivity and speed configuration
- Palm detection configuration
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopTouchpadManager;

let mut manager = DesktopTouchpadManager::new();

// Get configuration
println!("Total devices: {}", manager.get_devices().len());
println!("Total configurations: {}", manager.get_configurations().len());
```

### Device Management

```rust
// Add device
let device = TouchpadDevice::new(
    "custom".to_string(),
    "Custom Touchpad".to_string(),
)
.with_vendor("Vendor".to_string())
.with_product("Product".to_string())
.with_size(120, 80)
.with_num_fingers(5);

let id = manager.add_device(device);

// Remove device
manager.remove_device(&id);
```

### Configuration

```rust
// Set tap-to-click
manager.set_tap_to_click("touchpad_0", TapToClickMode::Enabled);

// Set natural scrolling
manager.set_natural_scrolling("touchpad_0", NaturalScrolling::Enabled);

// Set two-finger scrolling
manager.set_two_finger_scrolling("touchpad_0", TwoFingerScrolling::Enabled);

// Set edge scrolling
manager.set_edge_scrolling("touchpad_0", EdgeScrolling::Disabled);

// Set palm detection
manager.set_palm_detection("touchpad_0", PalmDetection::Enabled);
```

### Sensitivity and Speed

```rust
// Set sensitivity
manager.set_sensitivity("touchpad_0", 75);

// Set speed
manager.set_speed("touchpad_0", 60);
```

### Advanced Settings

```rust
// Disable while typing
manager.disable_while_typing("touchpad_0", true);

// Tap and drag
manager.tap_and_drag("touchpad_0", true);
```

### Full Configuration Update

```rust
let mut config = TouchpadConfiguration::new("touchpad_0".to_string());
config.set_tap_to_click(TapToClickMode::Enabled);
config.set_natural_scrolling(NaturalScrolling::Enabled);
config.set_sensitivity(80);
config.set_speed(70);

manager.update_configuration("touchpad_0", config);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total devices: {}", stats.total_devices);
println!("Total configurations: {}", stats.total_configurations);
println!("Tap-to-click enabled: {}", stats.tap_to_click_enabled);
println!("Natural scrolling enabled: {}", stats.natural_scrolling_enabled);
```

## Default Configuration

The Touchpad Manager includes default configuration:

- **Tap-to-Click**: Enabled
- **Natural Scrolling**: Disabled
- **Two-Finger Scrolling**: Enabled
- **Edge Scrolling**: Disabled
- **Palm Detection**: Enabled
- **Sensitivity**: 50
- **Acceleration**: 1.0
- **Speed**: 50
- **Disable While Typing**: true
- **Tap and Drag**: false
- **Default Device**: SynPS/2 Synaptics TouchPad

## AI Agent Maintenance Instructions

When maintaining the Touchpad Manager:

1. **Libinput Integration**: Integrate with libinput for actual touchpad control
2. **Device Detection**: Detect touchpad devices from /proc/bus/input/devices
3. **Gesture Support**: Add gesture support (pinch zoom, rotate, swipe)
4. **Multi-Touch**: Add multi-touch gesture recognition
5. **X11/Wayland**: Integrate with X11 and Wayland input systems
6. **Per-Application Settings**: Add per-application touchpad settings
7. **Profile Support**: Add touchpad profiles for different use cases
8. **Pressure Sensitivity**: Add pressure sensitivity configuration
9. **Button Mapping**: Add button mapping configuration
10. **Touchpad Calibration**: Add touchpad calibration tools

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::touchpad_manager
```

## Future Enhancements

- Libinput integration for actual touchpad control
- Device detection from /proc/bus/input/devices
- Gesture support (pinch zoom, rotate, swipe)
- Multi-touch gesture recognition
- X11 and Wayland input system integration
- Per-application touchpad settings
- Touchpad profiles for different use cases
- Pressure sensitivity configuration
- Button mapping configuration
- Touchpad calibration tools
