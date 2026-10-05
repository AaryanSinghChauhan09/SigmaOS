# Desktop Keyboard Manager

## Overview

The Desktop Keyboard Manager provides comprehensive keyboard management inspired by Linux Mint's keyboard settings and Omarchy's input utilities. It supports keyboard layouts, repeat settings, key repeat configuration, and per-device keyboard configuration.

## Features

- **Keyboard Layouts**: Multiple keyboard layouts with language and variant support
- **Layout Variants**: Support for layout variants (e.g., Dvorak, Colemak)
- **Repeat Modes**: Off, Delayed, Immediate
- **Repeat Configuration**: Configurable repeat delay and rate
- **NumLock Control**: Enable/disable NumLock
- **CapsLock Warning**: CapsLock warning indicator
- **Per-Device Configuration**: Separate configuration for each keyboard
- **Keyboard Device Tracking**: Track keyboard devices with vendor, product, and key count
- **Current Layout**: Track and switch current keyboard layout
- **Layout Search**: Search layouts by name and language
- **Statistics**: Track device count, layout count, and current layout status

## Components

### KeyboardLayout

Keyboard layout with:
- Layout ID and name
- Language code
- Variant (optional)

### RepeatMode

```rust
pub enum RepeatMode {
    Off,        // Key repeat disabled
    Delayed,    // Delayed key repeat
    Immediate,  // Immediate key repeat
}
```

### KeyboardDevice

Keyboard device with:
- Device ID and name
- Vendor and product
- Number of keys

### KeyboardConfiguration

Keyboard configuration with:
- Device ID
- Layout
- Variant
- Repeat mode
- Repeat delay (milliseconds)
- Repeat rate (repeats per second)
- NumLock on flag
- CapsLock warning flag

### DesktopKeyboardManager

Main management interface with:
- Device management (add, remove, retrieve)
- Configuration management
- Layout management (add, remove, retrieve)
- Per-device layout configuration
- Repeat configuration
- NumLock and CapsLock configuration
- Current layout tracking
- Layout search
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopKeyboardManager;

let mut manager = DesktopKeyboardManager::new();

// Get configuration
println!("Total devices: {}", manager.get_devices().len());
println!("Total layouts: {}", manager.get_layouts().len());
println!("Current layout: {:?}", manager.get_current_layout());
```

### Device Management

```rust
// Add device
let device = KeyboardDevice::new(
    "custom".to_string(),
    "Custom Keyboard".to_string(),
)
.with_vendor("Vendor".to_string())
.with_product("Product".to_string())
.with_num_keys(104);

let id = manager.add_device(device);

// Remove device
manager.remove_device(&id);
```

### Layout Management

```rust
// Add layout
let layout = KeyboardLayout::new(
    "custom".to_string(),
    "Custom Layout".to_string(),
    "custom".to_string(),
)
.with_variant("dvorak".to_string());

let id = manager.add_layout(layout);

// Remove layout
manager.remove_layout(&id);

// Set current layout
manager.set_current_layout(&id);
```

### Per-Device Layout

```rust
// Set layout for specific device
manager.set_layout_for_device("keyboard_0", "gb".to_string());
manager.set_layout_for_device("keyboard_0", "de".to_string());
```

### Repeat Configuration

```rust
// Set repeat mode
manager.set_repeat_mode("keyboard_0", RepeatMode::Immediate);

// Set repeat delay (milliseconds)
manager.set_repeat_delay("keyboard_0", 300);

// Set repeat rate (repeats per second)
manager.set_repeat_rate("keyboard_0", 50);
```

### NumLock and CapsLock

```rust
// Set NumLock
manager.set_numlock("keyboard_0", true);

// Set CapsLock warning
manager.set_capslock_warning("keyboard_0", false);
```

### Layout Search

```rust
// Search layouts
let results = manager.search_layouts("English");
for layout in results {
    println!("Found: {} ({})", layout.name, layout.language);
}
```

### Full Configuration Update

```rust
let mut config = KeyboardConfiguration::new("keyboard_0".to_string());
config.set_layout("gb".to_string());
config.set_variant(Some("dvorak".to_string()));
config.set_repeat_mode(RepeatMode::Immediate);
config.set_repeat_delay(300);
config.set_repeat_rate(50);

manager.update_configuration("keyboard_0", config);
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total devices: {}", stats.total_devices);
println!("Total layouts: {}", stats.total_layouts);
println!("Current layout set: {}", stats.current_layout_set);
```

## Default Layouts

The manager includes default layouts:

- **English (US)**: us layout
- **English (UK)**: gb layout
- **German**: de layout
- **French**: fr layout
- **Spanish**: es layout
- **Japanese**: jp layout with jp106 variant

## Default Configuration

The Keyboard Manager includes default configuration:

- **Current Layout**: English (US)
- **Default Layout for Device**: us
- **Repeat Mode**: Delayed
- **Repeat Delay**: 500ms
- **Repeat Rate**: 30 repeats/second
- **NumLock**: false
- **CapsLock Warning**: true
- **Default Device**: AT Translated Set 2 keyboard

## AI Agent Maintenance Instructions

When maintaining the Keyboard Manager:

1. **X11/Wayland Integration**: Integrate with X11 and Wayland for actual keyboard control
2. **XKB Integration**: Integrate with X Keyboard Extension (XKB) for layout switching
3. **Device Detection**: Detect keyboard devices from /proc/bus/input/devices
4. **Key Mapping**: Add key mapping and remapping support
5. **Compose Keys**: Add compose key configuration
6. **Keyboard Shortcuts**: Add keyboard shortcut management
7. **Layout Indicator**: Add layout indicator integration
8. **Auto-Switch**: Add automatic layout switching based on application
9. **IME Integration**: Integrate with input method engines
10. **Macro Support**: Add macro recording and playback

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::keyboard_manager
```

## Future Enhancements

- X11 and Wayland integration for actual keyboard control
- XKB integration for layout switching
- Device detection from /proc/bus/input/devices
- Key mapping and remapping support
- Compose key configuration
- Keyboard shortcut management
- Layout indicator integration
- Automatic layout switching based on application
- Input method engine integration
- Macro recording and playback
