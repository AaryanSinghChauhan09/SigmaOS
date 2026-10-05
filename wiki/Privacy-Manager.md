# Privacy Manager

## Overview

The Privacy Manager provides comprehensive privacy management inspired by Linux Mint's privacy settings and Omarchy's privacy utilities. It supports privacy controls, data collection management, and user privacy with configurable privacy levels.

## Features

- **Privacy Levels**: Minimal, Standard, High, Maximum
- **Privacy Settings**: Configurable privacy controls for various features
- **Categories**: Data Collection, Permissions, Notifications
- **Setting Management**: Add, enable, disable privacy settings
- **Category Filtering**: List settings by category
- **Privacy Level Control**: Set global privacy level with automatic setting adjustment
- **Default Settings**: Pre-configured telemetry, crash reports, location services, camera/microphone access, notifications, background activity, search history, analytics
- **Statistics**: Track setting counts by category and privacy level

## Components

### PrivacyLevel

```rust
pub enum PrivacyLevel {
    Minimal,   // Minimal privacy (most features enabled)
    Standard,  // Standard privacy (balanced)
    High,      // High privacy (more restrictions)
    Maximum,   // Maximum privacy (most restrictions)
}
```

### PrivacySetting

Represents a privacy setting with:
- Unique setting ID
- Setting name
- Description
- Enabled flag
- Category

### PrivacyManager

Main management interface with:
- Setting addition and listing
- Category-based filtering
- Enable/disable operations
- Privacy level control with automatic adjustment
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::security::PrivacyManager;

let mut manager = PrivacyManager::new();

// List all settings
let settings = manager.list_settings();
for setting in settings {
    println!("{}: {} ({})", setting.name, setting.description, if setting.is_enabled { "Enabled" } else { "Disabled" });
}
```

### Setting Management

```rust
// Add a setting
let setting = PrivacySetting::new(
    "custom-setting".to_string(),
    "Custom Setting".to_string(),
    "Custom privacy setting".to_string(),
    "Custom".to_string(),
);
manager.add_setting(setting);

// Get a setting
if let Some(setting) = manager.get_setting("telemetry") {
    println!("Setting: {}", setting.name);
}

// Enable/disable a setting
manager.disable("telemetry")?;
manager.enable("telemetry")?;
```

### Category Filtering

```rust
// List by category
let data_collection = manager.list_by_category("Data Collection");
let permissions = manager.list_by_category("Permissions");
let notifications = manager.list_by_category("Notifications");
```

### Privacy Level Control

```rust
// Get current privacy level
let level = manager.get_privacy_level();
println!("Privacy Level: {}", level.as_str());

// Set privacy level
manager.set_privacy_level(PrivacyLevel::Maximum);

// Setting Maximum privacy level automatically disables data collection
manager.set_privacy_level(PrivacyLevel::Minimal);

// Setting Minimal privacy level enables most features
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total settings: {}", stats.total_settings);
println!("Enabled: {}", stats.enabled_count);
println!("Data collection enabled: {}", stats.data_collection_count);
println!("Permissions count: {}", stats.permissions_count);
println!("Privacy level: {}", stats.privacy_level.as_str());
```

## Default Settings

The Privacy Manager includes pre-configured settings:

**Data Collection:**
- Telemetry: Send anonymous usage data
- Crash Reports: Send crash reports to developers
- Search History: Save search history
- Analytics: Send analytics data

**Permissions:**
- Location Services: Allow applications to access location
- Camera Access: Allow applications to access camera
- Microphone Access: Allow applications to access microphone
- Background Activity: Allow applications to run in background

**Notifications:**
- Desktop Notifications: Allow desktop notifications

## Privacy Level Behavior

- **Minimal**: Enables most features for maximum functionality
- **Standard**: Balanced privacy with reasonable restrictions
- **High**: More restrictive privacy settings
- **Maximum**: Disables all data collection features

## AI Agent Maintenance Instructions

When maintaining the Privacy Manager:

1. **Setting Validation**: Validate setting IDs and names before adding
2. **Category Consistency**: Ensure settings are properly categorized
3. **Level Transitions**: Ensure smooth transitions between privacy levels
4. **User Consent**: Implement user consent for privacy changes
5. **Persistence**: Save and restore privacy settings across reboots
6. **Default Settings**: Maintain sensible default privacy settings

## Testing

Run the unit tests with:

```bash
cargo test --lib security::privacy_manager
```

## Future Enhancements

- Per-application privacy settings
- Privacy dashboard
- Privacy audit logs
- Data collection reports
- Third-party application privacy controls
- Privacy wizard for initial setup
- Privacy import/export
- Location data clearing
- Browser privacy controls
- Analytics opt-out per application
