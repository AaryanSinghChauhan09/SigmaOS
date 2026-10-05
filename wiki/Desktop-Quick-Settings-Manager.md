# Desktop Quick Settings Manager

## Overview

The Desktop Quick Settings Manager provides comprehensive quick settings management inspired by Linux Mint's quick settings and Omarchy's control center utilities. It supports multiple setting types (toggles, sliders, actions, menus) with icons, descriptions, and value management.

## Features

- **Setting Types**: Toggle, Slider, Action, Menu
- **Setting Management**: Add, remove, enable, disable quick settings
- **Icons**: Custom icon support for each setting
- **Toggle Support**: On/off toggle functionality
- **Slider Support**: Value-based settings (0-100)
- **Descriptions**: Descriptive text for each setting
- **Setting Filtering**: List settings by type
- **Enabled Tracking**: Track enabled settings
- **Default Settings**: WiFi, Bluetooth, Airplane Mode, Brightness, Volume, Do Not Disturb, Night Light, Settings
- **Statistics**: Track total, toggle, slider, and enabled setting counts

## Components

### DesktopQuickSettingType

```rust
pub enum DesktopQuickSettingType {
    Toggle,  // On/off toggle
    Slider,  // Value slider (0-100)
    Action,  // Action button
    Menu,    // Menu entry
}
```

### DesktopQuickSetting

Quick setting structure with:
- Setting ID and name
- Setting type
- Custom icon (optional)
- Enabled status
- Value (for sliders, 0-100)
- Description (optional)

### DesktopQuickSettingsManager

Main management interface with:
- Setting registration and management
- Icon and description management
- Toggle functionality
- Slider value management
- Setting filtering by type
- Enabled setting tracking
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopQuickSettingsManager;

let mut manager = DesktopQuickSettingsManager::new();

// Get all settings
let settings = manager.get_settings();
println!("Total settings: {}", settings.len());
```

### Setting Management

```rust
// Add a new setting
let id = manager.add_setting("Location".to_string(), DesktopQuickSettingType::Toggle);

// Get setting by ID
if let Some(setting) = manager.get_setting(&id) {
    println!("Setting: {}", setting.name);
}

// Remove setting
manager.remove_setting(&id);
```

### Toggle Settings

```rust
// Set setting enabled state
manager.set_setting_enabled(&id, false);

// Toggle setting (flip enabled state)
manager.toggle_setting(&id);
```

### Slider Settings

```rust
// Set slider value (0-100)
manager.set_setting_value(&id, 75);

// Get slider value
if let Some(setting) = manager.get_setting(&id) {
    println!("Value: {:?}", setting.value);
}
```

### Icons and Descriptions

```rust
// Set setting icon
manager.set_setting_icon(&id, "location".to_string());

// Set setting description
manager.set_setting_description(&id, "Location services".to_string());
```

### Setting Filtering

```rust
// Get settings by type
let toggles = manager.get_settings_by_type(DesktopQuickSettingType::Toggle);
println!("Toggles: {}", toggles.len());

let sliders = manager.get_settings_by_type(DesktopQuickSettingType::Slider);
println!("Sliders: {}", sliders.len());

// Get only enabled settings
let enabled = manager.get_enabled_settings();
println!("Enabled: {}", enabled.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total settings: {}", stats.total_settings);
println!("Toggle settings: {}", stats.toggle_settings);
println!("Slider settings: {}", stats.slider_settings);
println!("Enabled settings: {}", stats.enabled_settings);
```

## Default Configuration

The Quick Settings Manager includes default settings:

- **WiFi**: Toggle type, icon "network-wireless", enabled, description "Wireless network"
- **Bluetooth**: Toggle type, icon "bluetooth", disabled, description "Bluetooth connectivity"
- **Airplane Mode**: Toggle type, icon "airplane-mode", disabled, description "Disable wireless connections"
- **Brightness**: Slider type, icon "display-brightness", value 75, description "Screen brightness"
- **Volume**: Slider type, icon "audio-volume-high", value 50, description "Output volume"
- **Do Not Disturb**: Toggle type, icon "notifications-disabled", disabled, description "Suppress notifications"
- **Night Light**: Toggle type, icon "night-light", disabled, description "Reduce blue light"
- **Settings**: Action type, icon "settings", description "System settings"

## AI Agent Maintenance Instructions

When maintaining the Quick Settings Manager:

1. **Icon Validation**: Ensure icon names exist in icon theme
2. **Value Validation**: Ensure slider values are within 0-100 range
3. **Toggle Logic**: Ensure toggle operations only work on toggle-type settings
4. **Slider Logic**: Ensure slider operations only work on slider-type settings
5. **State Sync**: Sync setting states with actual system settings
6. **Backend Integration**: Integrate with actual system settings (NetworkManager, PulseAudio, etc.)
7. **Quick Access**: Ensure quick settings are easily accessible from control center

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::quick_settings_manager
```

## Future Enhancements

- Integration with actual system settings (NetworkManager, PulseAudio, etc.)
- Real-time state updates
- Custom quick settings
- Quick settings grouping
- Per-user quick settings configuration
- Quick settings shortcuts
- Dynamic quick settings based on context
- Quick settings animation
- Multi-monitor support
