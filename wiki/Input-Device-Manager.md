# Input Device Manager

## Overview

The Input Device Manager provides comprehensive input device management inspired by Linux Mint's input settings and Omarchy's input utilities. It supports keyboard, mouse, touchpad, trackball, tablet, and gamepad configuration with device management and configurable settings.

## Features

- **Device Types**: Keyboard, Mouse, Touchpad, Trackball, Tablet, Gamepad
- **Device Management**: Add, enable, disable devices
- **Device Listing**: List all devices or filter by type
- **Keyboard Configuration**: Layout, repeat delay, repeat rate, caps lock behavior
- **Mouse Configuration**: Acceleration, sensitivity, handedness, scroll speed
- **Touchpad Configuration**: Tap to click, natural scrolling, two-finger scroll, edge scrolling, disable while typing, speed
- **Default Devices**: Pre-configured keyboard, mouse, and touchpad
- **Statistics**: Track device counts by type

## Components

### InputDeviceType

```rust
pub enum InputDeviceType {
    Keyboard,   // Keyboard device
    Mouse,      // Mouse device
    Touchpad,   // Touchpad device
    Trackball,  // Trackball device
    Tablet,     // Tablet device
    Gamepad,    // Gamepad device
}
```

### InputDevice

Represents an input device with:
- Unique device ID
- Device name
- Device type
- Enabled flag

### KeyboardConfig

Keyboard configuration with:
- Layout (e.g., "us", "uk", "de")
- Repeat delay (ms)
- Repeat rate (repeats per second)
- Caps lock behavior

### MouseConfig

Mouse configuration with:
- Acceleration factor
- Sensitivity level
- Left-handed mode
- Scroll speed

### TouchpadConfig

Touchpad configuration with:
- Tap to click
- Natural scrolling
- Two-finger scroll
- Edge scrolling
- Disable while typing
- Speed

### InputDeviceManager

Main management interface with:
- Device addition and listing
- Device enable/disable
- Type-based filtering
- Keyboard configuration
- Mouse configuration
- Touchpad configuration
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::input::InputDeviceManager;

let manager = InputDeviceManager::new();

// List all devices
let devices = manager.list_devices();
for device in devices {
    println!("{}: {} ({})", device.id, device.name, device.device_type.as_str());
}
```

### Device Management

```rust
// Add a device
let device = InputDevice::new(
    "mouse-1".to_string(),
    "Logitech Mouse".to_string(),
    InputDeviceType::Mouse,
);
manager.add_device(device);

// Get a device
if let Some(device) = manager.get_device("mouse-1") {
    println!("Device: {}", device.name);
}

// Enable/disable a device
manager.enable("mouse-1")?;
manager.disable("mouse-1")?;

// List devices by type
let keyboards = manager.list_by_type(InputDeviceType::Keyboard);
let mice = manager.list_by_type(InputDeviceType::Mouse);
```

### Keyboard Configuration

```rust
// Get current keyboard config
let config = manager.get_keyboard_config();
println!("Layout: {}", config.layout);
println!("Repeat delay: {}ms", config.repeat_delay);
println!("Repeat rate: {}", config.repeat_rate);

// Set keyboard config
let mut new_config = KeyboardConfig::new();
new_config.layout = "uk".to_string();
new_config.repeat_delay = 600;
new_config.repeat_rate = 25;
manager.set_keyboard_config(new_config);
```

### Mouse Configuration

```rust
// Get current mouse config
let config = manager.get_mouse_config();
println!("Acceleration: {}", config.acceleration);
println!("Sensitivity: {}", config.sensitivity);
println!("Left-handed: {}", config.left_handed);
println!("Scroll speed: {}", config.scroll_speed);

// Set mouse config
let mut new_config = MouseConfig::new();
new_config.acceleration = 1.5;
new_config.sensitivity = 7;
new_config.left_handed = true;
new_config.scroll_speed = 7;
manager.set_mouse_config(new_config);
```

### Touchpad Configuration

```rust
// Get current touchpad config
let config = manager.get_touchpad_config();
println!("Tap to click: {}", config.tap_to_click);
println!("Natural scrolling: {}", config.natural_scrolling);
println!("Two-finger scroll: {}", config.two_finger_scroll);
println!("Disable while typing: {}", config.disable_while_typing);

// Set touchpad config
let mut new_config = TouchpadConfig::new();
new_config.tap_to_click = false;
new_config.natural_scrolling = true;
new_config.speed = 7;
manager.set_touchpad_config(new_config);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total devices: {}", stats.total_devices);
println!("Enabled: {}", stats.enabled_count);
println!("Keyboards: {}", stats.keyboard_count);
println!("Mice: {}", stats.mouse_count);
println!("Touchpads: {}", stats.touchpad_count);
```

## Default Devices

The Input Device Manager includes three default devices:

- **keyboard-0**: System Keyboard
- **mouse-0**: System Mouse
- **touchpad-0**: System Touchpad

## Default Configurations

**Keyboard:**
- Layout: "us"
- Repeat delay: 500ms
- Repeat rate: 30/s
- Caps lock behavior: "default"

**Mouse:**
- Acceleration: 1.0
- Sensitivity: 5
- Left-handed: false
- Scroll speed: 5

**Touchpad:**
- Tap to click: true
- Natural scrolling: false
- Two-finger scroll: true
- Edge scrolling: false
- Disable while typing: true
- Speed: 5

## AI Agent Maintenance Instructions

When maintaining the Input Device Manager:

1. **Device Validation**: Validate device IDs and names before adding
2. **Config Validation**: Validate configuration values (e.g., acceleration between 0.1 and 10.0)
3. **Layout Support**: Ensure keyboard layouts are valid and available
4. **Device Detection**: Properly detect and classify input devices
5. **Config Persistence**: Save and restore configurations across reboots
6. **Hotplug Support**: Handle device hotplug events

## Testing

Run the unit tests with:

```bash
cargo test --lib input::input_device_manager
```

## Future Enhancements

- Integration with actual input device drivers (evdev, libinput)
- Per-device configuration profiles
- Macro support for keyboards
- DPI configuration for mice
- Gesture recognition for touchpads
- Multi-monitor pointer configuration
- Per-application input settings
- Input device pairing and grouping
- Custom key bindings
- Gamepad button mapping
