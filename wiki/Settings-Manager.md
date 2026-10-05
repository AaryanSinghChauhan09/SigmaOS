# Settings Manager

## Overview

The Settings Manager provides comprehensive system settings management inspired by Linux Mint's settings and Omarchy's configuration utilities. It supports system-wide and user settings across multiple categories.

## Features

- **Setting Value Types**: String, Integer, Boolean, Float, String List
- **Setting Categories**: Appearance, Display, Sound, Network, Power, Privacy, Accessibility, Input, System
- **Default Settings**: Pre-configured essential system settings
- **Read-Only Protection**: Critical settings marked as read-only
- **Setting Lookup**: Find settings by key or category
- **Search**: Search settings by key
- **Reset**: Reset individual settings or entire categories to defaults
- **Export/Import**: Export and import settings for backup/restore
- **Statistics**: Track setting counts by category

## Components

### SettingValue

```rust
pub enum SettingValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Float(f64),
    StringList(Vec<String>),
}
```

### SettingCategory

```rust
pub enum SettingCategory {
    Appearance,   // Theme, fonts, colors
    Display,      // Brightness, night light
    Sound,        // Volume, mute
    Network,      // Auto-connect, metered
    Power,        // Power profile, sleep
    Privacy,      // Analytics, location
    Accessibility, // Accessibility features
    Input,        // Keyboard, mouse
    System,       // Timezone, language
}
```

### SettingEntry

Represents a setting with:
- Key and value
- Category
- Description
- Read-only flag

### SettingsManager

Main management interface with:
- Default system settings registration
- Setting get/set operations
- Category-based filtering
- Search functionality
- Reset operations
- Export/import
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::system::SettingsManager;

let mut manager = SettingsManager::new();

// Get a setting
if let Some(theme) = manager.get("theme") {
    println!("Current theme: {:?}", theme.value);
}

// Set a setting
manager.set("theme", SettingValue::String("Sigma Light".to_string()))?;
```

### Managing Settings

```rust
// Get setting value
if let Some(value) = manager.get_value("volume") {
    if let Some(vol) = value.as_integer() {
        println!("Volume: {}", vol);
    }
}

// Set boolean setting
manager.set("mute", SettingValue::Boolean(true))?;

// Set integer setting
manager.set("volume", SettingValue::Integer(80))?;
```

### Filtering Settings

```rust
// List all settings
let all = manager.list_all();

// List by category
let appearance = manager.list_by_category(SettingCategory::Appearance);

// Search settings
let results = manager.search("theme");
```

### Reset Operations

```rust
// Reset a single setting
manager.reset("theme")?;

// Reset entire category
let count = manager.reset_category(SettingCategory::Appearance);
println!("Reset {} settings", count);
```

### Export/Import

```rust
// Export all settings
let exported = manager.export();

// Import settings
let mut manager2 = SettingsManager::new();
manager2.import(exported)?;
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total settings: {}", stats.total_settings);
println!("Read-only: {}", stats.readonly_count);
println!("By category: {:?}", stats.by_category);
```

## Default Settings

The Settings Manager includes these default settings:

**Appearance:**
- theme: "Sigma Dark"
- accent_color: "Blue"
- font_size: 14

**Display:**
- brightness: 100
- night_light: false

**Sound:**
- volume: 75
- mute: false

**Network:**
- auto_connect: true
- metered_warning: true

**Power:**
- power_profile: "Balanced"
- auto_sleep: true
- sleep_timeout: 30

**Privacy:**
- collect_analytics: false
- location_services: false

**Input:**
- keyboard_layout: "us"
- mouse_acceleration: true

**System:**
- timezone: "UTC" (read-only)
- language: "en_US" (read-only)

## AI Agent Maintenance Instructions

When maintaining the Settings Manager:

1. **Read-Only Protection**: Ensure critical settings (timezone, language) remain read-only
2. **Type Safety**: Validate setting values before assignment
3. **Default Values**: Maintain sensible default values for all settings
4. **Category Organization**: Keep settings properly categorized
5. **Export/Import**: Ensure exported settings can be safely imported
6. **Reset Logic**: Maintain accurate reset logic for each setting

## Testing

Run the unit tests with:

```bash
cargo test --lib system::settings_manager
```

## Future Enhancements

- Integration with actual configuration backends (GSettings, dconf)
- User-specific settings
- Setting validation
- Setting change notifications
- Setting profiles
- Remote configuration management
